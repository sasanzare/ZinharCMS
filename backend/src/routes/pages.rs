use std::collections::HashSet;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::routes::delivery;
use crate::services::entry_validation::is_valid_slug;
use crate::services::{audit, quota, rbac, rls, webhooks, workflow};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/pages", get(list_pages).post(create_page))
        .route("/api/pages/slug/{slug}", get(get_page_by_slug))
        .route(
            "/api/pages/{id}",
            get(get_page).put(update_page).delete(delete_page),
        )
        .route(
            "/api/pages/{id}/submit-review",
            post(submit_page_for_review),
        )
        .route("/api/pages/{id}/publish", post(publish_page))
        .route("/api/pages/{id}/reject", post(reject_page))
        .route("/api/pages/{id}/archive", post(archive_page))
        .route("/api/pages/{id}/restore", post(restore_page))
        .route("/api/pages/{id}/unpublish", post(unpublish_page))
        .route("/api/pages/{id}/versions", get(list_page_versions))
        .route(
            "/api/pages/{id}/versions/{version}/restore",
            post(restore_page_version),
        )
        .route(
            "/api/component-registry",
            get(list_components).post(create_component),
        )
        .route(
            "/api/component-registry/{component_key}",
            get(get_component)
                .put(update_component)
                .delete(delete_component),
        )
        .route("/api/preview/{page_id}", get(preview_page))
}

#[derive(Debug, Deserialize)]
pub struct PageListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PageRequest {
    pub title: String,
    pub slug: String,
    #[serde(default = "default_page_json")]
    pub page_json: Value,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PageResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub page_json: Value,
    pub status: String,
    pub author_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PageListResponse {
    pub data: Vec<PageResponse>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PageVersionResponse {
    pub id: Uuid,
    pub page_id: Uuid,
    pub version: i32,
    pub page_json: Value,
    pub snapshot_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ComponentRegistryRequest {
    pub component_key: String,
    pub name: String,
    pub category: String,
    #[serde(default = "empty_object")]
    pub props_schema: Value,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ComponentRegistryResponse {
    pub id: Uuid,
    pub component_key: String,
    pub name: String,
    pub category: String,
    pub props_schema: Value,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ComponentListQuery {
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteConfirm {
    pub confirm: Option<bool>,
}

#[utoipa::path(
    get,
    path = "/api/pages",
    tag = "pages",
    responses((status = 200, description = "Pages", body = PageListResponse))
)]
pub async fn list_pages(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<PageListQuery>,
) -> Result<Json<PageListResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;
    let (sort_column, sort_direction) = parse_sort(query.sort.as_deref())?;

    let data = if let Some(status) = query.status.as_deref() {
        validate_page_status(status)?;
        let sql = format!(
            r#"
            SELECT id,
                   title,
                   slug,
                   page_json,
                   status::text as status,
                   author_id,
                   published_at,
                   created_at,
                   updated_at
            FROM pages
            WHERE organization_id = $1 AND status::text = $2
            ORDER BY {sort_column} {sort_direction}
            LIMIT $3 OFFSET $4
            "#
        );
        sqlx::query_as::<_, PageResponse>(&sql)
            .bind(tenant.organization_id)
            .bind(status)
            .bind(per_page)
            .bind(offset)
            .fetch_all(db.as_mut())
            .await?
    } else {
        let sql = format!(
            r#"
            SELECT id,
                   title,
                   slug,
                   page_json,
                   status::text as status,
                   author_id,
                   published_at,
                   created_at,
                   updated_at
            FROM pages
            WHERE organization_id = $1
            ORDER BY {sort_column} {sort_direction}
            LIMIT $2 OFFSET $3
            "#
        );
        sqlx::query_as::<_, PageResponse>(&sql)
            .bind(tenant.organization_id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(db.as_mut())
            .await?
    };

    Ok(Json(PageListResponse {
        data,
        page,
        per_page,
    }))
}

#[utoipa::path(
    post,
    path = "/api/pages",
    tag = "pages",
    request_body = PageRequest,
    responses((status = 200, description = "Created page", body = PageResponse))
)]
pub async fn create_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<PageRequest>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_page_writer(&tenant.role)?;
    quota::ensure_content_capacity(&state.db, &tenant).await?;
    validate_page_request(&state, &tenant, &payload).await?;

    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let page = sqlx::query_as::<_, PageResponse>(
        r#"
        INSERT INTO pages (organization_id, title, slug, page_json, author_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id,
                  title,
                  slug,
                  page_json,
                  status::text as status,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(payload.title.trim())
    .bind(payload.slug.trim())
    .bind(&payload.page_json)
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await?;

    create_version_snapshot(
        &mut tx,
        tenant.organization_id,
        page.id,
        &page.page_json,
        claims.sub,
    )
    .await?;
    tx.commit().await?;
    broadcast_page_json(&state, page.id, &page.page_json).await;

    Ok(Json(page))
}

#[utoipa::path(
    get,
    path = "/api/pages/{id}",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Page", body = PageResponse))
)]
pub async fn get_page(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<PageResponse>, AppError> {
    load_page_by_id(&state, &tenant, id).await.map(Json)
}

#[utoipa::path(
    get,
    path = "/api/pages/slug/{slug}",
    tag = "pages",
    params(("slug" = String, Path, description = "Page slug")),
    responses((status = 200, description = "Page", body = PageResponse))
)]
pub async fn get_page_by_slug(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(slug): Path<String>,
) -> Result<Json<PageResponse>, AppError> {
    if !is_valid_slug(&slug) {
        return Err(AppError::Validation("slug is invalid".to_owned()));
    }
    load_page_by_slug(&state, &tenant, &slug).await.map(Json)
}
#[utoipa::path(
    put,
    path = "/api/pages/{id}",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    request_body = PageRequest,
    responses((status = 200, description = "Updated page", body = PageResponse))
)]
pub async fn update_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<PageRequest>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_page_writer(&tenant.role)?;
    validate_page_request(&state, &tenant, &payload).await?;

    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let page = sqlx::query_as::<_, PageResponse>(
        r#"
        UPDATE pages
        SET title = $2,
            slug = $3,
            page_json = $4,
            updated_at = now()
        WHERE id = $1 AND organization_id = $5
        RETURNING id,
                  title,
                  slug,
                  page_json,
                  status::text as status,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .bind(payload.title.trim())
    .bind(payload.slug.trim())
    .bind(&payload.page_json)
    .bind(tenant.organization_id)
    .fetch_one(&mut *tx)
    .await?;

    create_version_snapshot(
        &mut tx,
        tenant.organization_id,
        page.id,
        &page.page_json,
        claims.sub,
    )
    .await?;
    tx.commit().await?;
    broadcast_page_json(&state, page.id, &page.page_json).await;
    if page.status == "published" {
        delivery::invalidate_page_cache(&state, tenant.organization_id).await;
    }

    Ok(Json(page))
}

#[utoipa::path(
    delete,
    path = "/api/pages/{id}",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Deleted page", body = PageResponse))
)]
pub async fn delete_page(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
    Query(query): Query<DeleteConfirm>,
) -> Result<Json<PageResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_page_manager(&tenant.role)?;
    if query.confirm != Some(true) {
        return Err(AppError::Validation(
            "pass ?confirm=true to delete a page".to_owned(),
        ));
    }

    let page = sqlx::query_as::<_, PageResponse>(
        r#"
        DELETE FROM pages
        WHERE id = $1 AND organization_id = $2
        RETURNING id,
                  title,
                  slug,
                  page_json,
                  status::text as status,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;

    if page.status == "published" {
        delivery::invalidate_page_cache(&state, tenant.organization_id).await;
    }

    audit::record(
        &state.db,
        &tenant,
        "page.delete",
        "page",
        Some(page.id),
        serde_json::json!({ "title": &page.title, "slug": &page.slug, "status": &page.status }),
    )
    .await?;

    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/api/pages/{id}/publish",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Published page", body = PageResponse))
)]
pub async fn publish_page(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_page_publisher(&tenant.role)?;
    let page = transition_page(
        &state,
        &tenant,
        id,
        workflow::WorkflowStatus::Published,
        true,
    )
    .await?;
    delivery::invalidate_page_cache(&state, tenant.organization_id).await;
    webhooks::trigger_event(
        &state,
        tenant.organization_id,
        webhooks::PAGE_PUBLISH,
        page_webhook_payload(webhooks::PAGE_PUBLISH, &tenant, &page),
    )
    .await;
    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/api/pages/{id}/unpublish",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Unpublished page", body = PageResponse))
)]
pub async fn unpublish_page(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_page_publisher(&tenant.role)?;
    let page = transition_page(&state, &tenant, id, workflow::WorkflowStatus::Draft, true).await?;
    delivery::invalidate_page_cache(&state, tenant.organization_id).await;
    webhooks::trigger_event(
        &state,
        tenant.organization_id,
        webhooks::PAGE_UNPUBLISH,
        page_webhook_payload(webhooks::PAGE_UNPUBLISH, &tenant, &page),
    )
    .await;
    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/api/pages/{id}/submit-review",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Submitted page for review", body = PageResponse))
)]
pub async fn submit_page_for_review(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_page_writer(&tenant.role)?;
    let page = transition_page(
        &state,
        &tenant,
        id,
        workflow::WorkflowStatus::PendingReview,
        false,
    )
    .await?;
    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/api/pages/{id}/reject",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Rejected page", body = PageResponse))
)]
pub async fn reject_page(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_workflow_reviewer(&tenant.role)?;
    let page = transition_page(&state, &tenant, id, workflow::WorkflowStatus::Draft, true).await?;
    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/api/pages/{id}/archive",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Archived page", body = PageResponse))
)]
pub async fn archive_page(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_workflow_reviewer(&tenant.role)?;
    let page = transition_page(
        &state,
        &tenant,
        id,
        workflow::WorkflowStatus::Archived,
        true,
    )
    .await?;
    delivery::invalidate_page_cache(&state, tenant.organization_id).await;
    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/api/pages/{id}/restore",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Restored page", body = PageResponse))
)]
pub async fn restore_page(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<PageResponse>, AppError> {
    rbac::require_org_workflow_reviewer(&tenant.role)?;
    let page = transition_page(&state, &tenant, id, workflow::WorkflowStatus::Draft, true).await?;
    Ok(Json(page))
}
#[utoipa::path(
    get,
    path = "/api/pages/{id}/versions",
    tag = "pages",
    params(("id" = Uuid, Path, description = "Page id")),
    responses((status = 200, description = "Page versions", body = [PageVersionResponse]))
)]
pub async fn list_page_versions(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<PageVersionResponse>>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let versions = sqlx::query_as::<_, PageVersionResponse>(
        r#"
        SELECT id, page_id, version, page_json, snapshot_at, created_by
        FROM page_versions
        WHERE page_id = $1 AND organization_id = $2
        ORDER BY version DESC
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_all(db.as_mut())
    .await?;

    Ok(Json(versions))
}

#[utoipa::path(
    post,
    path = "/api/pages/{id}/versions/{version}/restore",
    tag = "pages",
    params(
        ("id" = Uuid, Path, description = "Page id"),
        ("version" = i32, Path, description = "Page version")
    ),
    responses((status = 200, description = "Restored page", body = PageResponse))
)]
pub async fn restore_page_version(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path((id, version)): Path<(Uuid, i32)>,
) -> Result<Json<PageResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_page_manager(&tenant.role)?;
    if version < 1 {
        return Err(AppError::Validation("version must be positive".to_owned()));
    }

    let version_row = sqlx::query_as::<_, PageVersionResponse>(
        r#"
        SELECT id, page_id, version, page_json, snapshot_at, created_by
        FROM page_versions
        WHERE page_id = $1 AND version = $2 AND organization_id = $3
        "#,
    )
    .bind(id)
    .bind(version)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;

    let component_keys = load_component_keys(&state, &tenant).await?;
    validate_page_json(&version_row.page_json, &component_keys)?;

    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let page = sqlx::query_as::<_, PageResponse>(
        r#"
        UPDATE pages
        SET page_json = $2,
            status = 'draft'::page_status,
            published_at = NULL,
            updated_at = now()
        WHERE id = $1 AND organization_id = $3
        RETURNING id,
                  title,
                  slug,
                  page_json,
                  status::text as status,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .bind(&version_row.page_json)
    .bind(tenant.organization_id)
    .fetch_one(&mut *tx)
    .await?;

    create_version_snapshot(
        &mut tx,
        tenant.organization_id,
        page.id,
        &page.page_json,
        claims.sub,
    )
    .await?;
    tx.commit().await?;
    broadcast_page_json(&state, page.id, &page.page_json).await;

    Ok(Json(page))
}

#[utoipa::path(
    get,
    path = "/api/component-registry",
    tag = "components",
    responses((status = 200, description = "Component registry", body = [ComponentRegistryResponse]))
)]
pub async fn list_components(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<ComponentListQuery>,
) -> Result<Json<Vec<ComponentRegistryResponse>>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = if let Some(category) = query.category.as_deref() {
        sqlx::query_as::<_, ComponentRegistryResponse>(
            r#"
            SELECT id,
                   component_key,
                   name,
                   category,
                   props_schema,
                   is_system,
                   created_at,
                   updated_at
            FROM component_registry
            WHERE (is_system = TRUE OR organization_id = $1) AND category = $2
            ORDER BY category ASC, name ASC
            "#,
        )
        .bind(tenant.organization_id)
        .bind(category)
        .fetch_all(db.as_mut())
        .await?
    } else {
        sqlx::query_as::<_, ComponentRegistryResponse>(
            r#"
            SELECT id,
                   component_key,
                   name,
                   category,
                   props_schema,
                   is_system,
                   created_at,
                   updated_at
            FROM component_registry
            WHERE is_system = TRUE OR organization_id = $1
            ORDER BY category ASC, name ASC
            "#,
        )
        .bind(tenant.organization_id)
        .fetch_all(db.as_mut())
        .await?
    };

    Ok(Json(rows))
}
#[utoipa::path(
    post,
    path = "/api/component-registry",
    tag = "components",
    request_body = ComponentRegistryRequest,
    responses((status = 200, description = "Created component", body = ComponentRegistryResponse))
)]
pub async fn create_component(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<ComponentRegistryRequest>,
) -> Result<Json<ComponentRegistryResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_component_registry_manager(&tenant.role)?;
    validate_component_request(&payload)?;

    let row = sqlx::query_as::<_, ComponentRegistryResponse>(
        r#"
        INSERT INTO component_registry (organization_id, component_key, name, category, props_schema, is_system)
        VALUES ($1, $2, $3, $4, $5, false)
        RETURNING id,
                  component_key,
                  name,
                  category,
                  props_schema,
                  is_system,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(payload.component_key.trim())
    .bind(payload.name.trim())
    .bind(payload.category.trim())
    .bind(payload.props_schema)
    .fetch_one(db.as_mut())
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/component-registry/{component_key}",
    tag = "components",
    params(("component_key" = String, Path, description = "Component key")),
    responses((status = 200, description = "Component", body = ComponentRegistryResponse))
)]
pub async fn get_component(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(component_key): Path<String>,
) -> Result<Json<ComponentRegistryResponse>, AppError> {
    if !is_valid_slug(&component_key) {
        return Err(AppError::Validation("component_key is invalid".to_owned()));
    }
    load_component(&state, &tenant, &component_key)
        .await
        .map(Json)
}

#[utoipa::path(
    put,
    path = "/api/component-registry/{component_key}",
    tag = "components",
    params(("component_key" = String, Path, description = "Component key")),
    request_body = ComponentRegistryRequest,
    responses((status = 200, description = "Updated component", body = ComponentRegistryResponse))
)]
pub async fn update_component(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(component_key): Path<String>,
    Json(payload): Json<ComponentRegistryRequest>,
) -> Result<Json<ComponentRegistryResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_component_registry_manager(&tenant.role)?;
    validate_component_request(&payload)?;
    if payload.component_key.trim() != component_key {
        return Err(AppError::Validation(
            "component_key cannot differ from the path".to_owned(),
        ));
    }

    let row = sqlx::query_as::<_, ComponentRegistryResponse>(
        r#"
        UPDATE component_registry
        SET name = $3,
            category = $4,
            props_schema = $5,
            updated_at = now()
        WHERE component_key = $1 AND organization_id = $2 AND is_system = FALSE
        RETURNING id,
                  component_key,
                  name,
                  category,
                  props_schema,
                  is_system,
                  created_at,
                  updated_at
        "#,
    )
    .bind(component_key)
    .bind(tenant.organization_id)
    .bind(payload.name.trim())
    .bind(payload.category.trim())
    .bind(payload.props_schema)
    .fetch_one(db.as_mut())
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/api/component-registry/{component_key}",
    tag = "components",
    params(("component_key" = String, Path, description = "Component key")),
    responses((status = 200, description = "Deleted component", body = ComponentRegistryResponse))
)]
pub async fn delete_component(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(component_key): Path<String>,
    Query(query): Query<DeleteConfirm>,
) -> Result<Json<ComponentRegistryResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_component_registry_manager(&tenant.role)?;
    if query.confirm != Some(true) {
        return Err(AppError::Validation(
            "pass ?confirm=true to delete a component".to_owned(),
        ));
    }

    let existing = load_component(&state, &tenant, &component_key).await?;
    if existing.is_system {
        return Err(AppError::Validation(
            "system components cannot be deleted".to_owned(),
        ));
    }

    let row = sqlx::query_as::<_, ComponentRegistryResponse>(
        r#"
        DELETE FROM component_registry
        WHERE component_key = $1 AND organization_id = $2 AND is_system = FALSE
        RETURNING id,
                  component_key,
                  name,
                  category,
                  props_schema,
                  is_system,
                  created_at,
                  updated_at
        "#,
    )
    .bind(component_key)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "component.delete",
        "component_registry",
        Some(row.id),
        serde_json::json!({ "component_key": &row.component_key, "name": &row.name }),
    )
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/preview/{page_id}",
    tag = "preview",
    params(("page_id" = Uuid, Path, description = "Page id")),
    responses((status = 101, description = "WebSocket page JSON preview stream"))
)]
pub async fn preview_page(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(page_id): Path<Uuid>,
) -> Response {
    ws.on_upgrade(move |socket| handle_preview_socket(socket, page_id, tenant, state))
}

async fn handle_preview_socket(
    mut socket: WebSocket,
    page_id: Uuid,
    tenant: TenantContext,
    state: AppState,
) {
    let sender = preview_sender(&state, page_id).await;
    let mut rx = sender.subscribe();

    if let Ok(page) = load_page_by_id(&state, &tenant, page_id).await {
        let Ok(message) = serde_json::to_string(&page.page_json) else {
            return;
        };
        if socket.send(Message::Text(message.into())).await.is_err() {
            return;
        }
    }

    while let Ok(update) = rx.recv().await {
        if socket.send(Message::Text(update.into())).await.is_err() {
            break;
        }
    }
}

fn page_webhook_payload(event: &str, tenant: &TenantContext, page: &PageResponse) -> Value {
    serde_json::json!({
        "event": event,
        "entity": "page",
        "organization_id": tenant.organization_id,
        "organization_slug": tenant.organization_slug,
        "page": page,
    })
}
async fn transition_page(
    state: &AppState,
    tenant: &TenantContext,
    id: Uuid,
    status: workflow::WorkflowStatus,
    can_bypass_review: bool,
) -> Result<PageResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let current = load_page_by_id(state, tenant, id).await?;
    workflow::require_transition(&current.status, status, can_bypass_review)?;
    let next_status = status.as_str();
    let published_at_sql = if status == workflow::WorkflowStatus::Published {
        "now()"
    } else {
        "NULL"
    };
    let sql = format!(
        r#"
        UPDATE pages
        SET status = $2::page_status,
            published_at = {published_at_sql},
            updated_at = now()
        WHERE id = $1 AND organization_id = $3
        RETURNING id,
                  title,
                  slug,
                  page_json,
                  status::text as status,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#
    );

    let page = sqlx::query_as::<_, PageResponse>(&sql)
        .bind(id)
        .bind(next_status)
        .bind(tenant.organization_id)
        .fetch_one(db.as_mut())
        .await?;
    broadcast_page_json(state, page.id, &page.page_json).await;

    Ok(page)
}

async fn load_page_by_id(
    state: &AppState,
    tenant: &TenantContext,
    id: Uuid,
) -> Result<PageResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    sqlx::query_as::<_, PageResponse>(
        r#"
        SELECT id,
               title,
               slug,
               page_json,
               status::text as status,
               author_id,
               published_at,
               created_at,
               updated_at
        FROM pages
        WHERE id = $1 AND organization_id = $2
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_page_by_slug(
    state: &AppState,
    tenant: &TenantContext,
    slug: &str,
) -> Result<PageResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    sqlx::query_as::<_, PageResponse>(
        r#"
        SELECT id,
               title,
               slug,
               page_json,
               status::text as status,
               author_id,
               published_at,
               created_at,
               updated_at
        FROM pages
        WHERE slug = $1 AND organization_id = $2
        "#,
    )
    .bind(slug)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_component(
    state: &AppState,
    tenant: &TenantContext,
    component_key: &str,
) -> Result<ComponentRegistryResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    sqlx::query_as::<_, ComponentRegistryResponse>(
        r#"
        SELECT id,
               component_key,
               name,
               category,
               props_schema,
               is_system,
               created_at,
               updated_at
        FROM component_registry
        WHERE component_key = $1 AND (is_system = TRUE OR organization_id = $2)
        ORDER BY is_system ASC
        "#,
    )
    .bind(component_key)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}
async fn create_version_snapshot(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    organization_id: Uuid,
    page_id: Uuid,
    page_json: &Value,
    user_id: Uuid,
) -> Result<PageVersionResponse, AppError> {
    sqlx::query_as::<_, PageVersionResponse>(
        r#"
        INSERT INTO page_versions (organization_id, page_id, version, page_json, created_by)
        VALUES (
          $1,
          $2,
          COALESCE((SELECT MAX(version) + 1 FROM page_versions WHERE page_id = $2 AND organization_id = $1), 1),
          $3,
          $4
        )
        RETURNING id, page_id, version, page_json, snapshot_at, created_by
        "#,
    )
    .bind(organization_id)
    .bind(page_id)
    .bind(page_json)
    .bind(user_id)
    .fetch_one(&mut **tx)
    .await
    .map_err(AppError::from)
}

async fn load_component_keys(
    state: &AppState,
    tenant: &TenantContext,
) -> Result<HashSet<String>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_scalar::<_, String>(
        "SELECT component_key FROM component_registry WHERE is_system = TRUE OR organization_id = $1",
    )
    .bind(tenant.organization_id)
    .fetch_all(db.as_mut())
    .await?;
    Ok(rows.into_iter().collect())
}

async fn validate_page_request(
    state: &AppState,
    tenant: &TenantContext,
    payload: &PageRequest,
) -> Result<(), AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::Validation("title is required".to_owned()));
    }
    if !is_valid_slug(payload.slug.trim()) {
        return Err(AppError::Validation("slug is invalid".to_owned()));
    }

    let component_keys = load_component_keys(state, tenant).await?;
    validate_page_json(&payload.page_json, &component_keys)
}

pub(crate) async fn validate_page_json_for_tenant(
    state: &AppState,
    tenant: &TenantContext,
    page_json: &Value,
) -> Result<(), AppError> {
    let component_keys = load_component_keys(state, tenant).await?;
    validate_page_json(page_json, &component_keys)
}

fn validate_component_request(payload: &ComponentRegistryRequest) -> Result<(), AppError> {
    if !is_valid_slug(payload.component_key.trim()) {
        return Err(AppError::Validation("component_key is invalid".to_owned()));
    }
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("name is required".to_owned()));
    }
    if !is_valid_slug(payload.category.trim()) {
        return Err(AppError::Validation("category is invalid".to_owned()));
    }

    let object = payload
        .props_schema
        .as_object()
        .ok_or_else(|| AppError::Validation("props_schema must be a JSON object".to_owned()))?;
    for (prop_name, prop_schema) in object {
        if !is_valid_prop_name(prop_name) {
            return Err(AppError::Validation(format!(
                "prop '{prop_name}' must use snake_case letters, numbers, or underscores"
            )));
        }
        validate_prop_schema(prop_name, prop_schema)?;
    }

    Ok(())
}

fn validate_prop_schema(prop_name: &str, prop_schema: &Value) -> Result<(), AppError> {
    let object = prop_schema.as_object().ok_or_else(|| {
        AppError::Validation(format!("prop '{prop_name}' schema must be a JSON object"))
    })?;

    let Some(prop_type) = object.get("type").and_then(Value::as_str) else {
        return Err(AppError::Validation(format!(
            "prop '{prop_name}' schema requires a string type"
        )));
    };

    match prop_type {
        "array" | "boolean" | "email" | "json" | "media" | "number" | "richtext" | "select"
        | "text" | "url" => Ok(()),
        other => Err(AppError::Validation(format!(
            "prop type '{other}' is not supported"
        ))),
    }
}

fn validate_page_json(page_json: &Value, component_keys: &HashSet<String>) -> Result<(), AppError> {
    let object = page_json
        .as_object()
        .ok_or_else(|| AppError::Validation("page_json must be a JSON object".to_owned()))?;

    if object
        .get("version")
        .is_some_and(|version| !version.is_string())
    {
        return Err(AppError::Validation(
            "page_json.version must be a string".to_owned(),
        ));
    }

    if let Some(metadata) = object.get("metadata") {
        validate_metadata(metadata)?;
    }

    let layout = object
        .get("layout")
        .ok_or_else(|| AppError::Validation("page_json.layout is required".to_owned()))?;
    let mut node_count = 0;
    validate_page_node(layout, component_keys, 0, &mut node_count)
}

fn validate_metadata(metadata: &Value) -> Result<(), AppError> {
    let object = metadata.as_object().ok_or_else(|| {
        AppError::Validation("page_json.metadata must be a JSON object".to_owned())
    })?;

    for field in ["title", "description", "og_image"] {
        if object.get(field).is_some_and(|value| !value.is_string()) {
            return Err(AppError::Validation(format!(
                "page_json.metadata.{field} must be a string"
            )));
        }
    }

    Ok(())
}

fn validate_page_node(
    node: &Value,
    component_keys: &HashSet<String>,
    depth: usize,
    node_count: &mut usize,
) -> Result<(), AppError> {
    if depth > 12 {
        return Err(AppError::Validation(
            "page_json.layout is nested too deeply".to_owned(),
        ));
    }
    *node_count += 1;
    if *node_count > 500 {
        return Err(AppError::Validation(
            "page_json.layout has too many components".to_owned(),
        ));
    }

    let object = node
        .as_object()
        .ok_or_else(|| AppError::Validation("layout node must be a JSON object".to_owned()))?;
    let id = required_string(object, "id")?;
    if id.trim().is_empty() || id.chars().count() > 128 {
        return Err(AppError::Validation(
            "layout node id must be 1 to 128 characters".to_owned(),
        ));
    }

    let component_type = required_string(object, "type")?;
    if depth == 0 {
        if component_type != "root" {
            return Err(AppError::Validation(
                "page_json.layout root type must be 'root'".to_owned(),
            ));
        }
    } else if component_type == "root" {
        return Err(AppError::Validation(
            "only the layout root can use type 'root'".to_owned(),
        ));
    } else if !component_keys.contains(component_type) {
        return Err(AppError::Validation(format!(
            "component type '{component_type}' is not registered"
        )));
    }

    validate_optional_object(object, "props")?;
    validate_optional_object(object, "styles")?;

    if let Some(children) = object.get("children") {
        let children = children.as_array().ok_or_else(|| {
            AppError::Validation("layout node children must be an array".to_owned())
        })?;
        for child in children {
            validate_page_node(child, component_keys, depth + 1, node_count)?;
        }
    }

    Ok(())
}

fn required_string<'a>(object: &'a Map<String, Value>, field: &str) -> Result<&'a str, AppError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| AppError::Validation(format!("layout node {field} must be a string")))
}

fn validate_optional_object(object: &Map<String, Value>, field: &str) -> Result<(), AppError> {
    if object.get(field).is_some_and(|value| !value.is_object()) {
        return Err(AppError::Validation(format!(
            "layout node {field} must be a JSON object"
        )));
    }
    Ok(())
}

fn parse_sort(sort: Option<&str>) -> Result<(&'static str, &'static str), AppError> {
    let Some(sort) = sort else {
        return Ok(("created_at", "DESC"));
    };

    let (field, direction) = sort.split_once(':').unwrap_or((sort, "desc"));
    let field = match field {
        "created_at" => "created_at",
        "updated_at" => "updated_at",
        "published_at" => "published_at",
        "title" => "title",
        other => {
            return Err(AppError::Validation(format!(
                "sort field '{other}' is not supported"
            )));
        }
    };
    let direction = match direction.to_ascii_lowercase().as_str() {
        "asc" => "ASC",
        "desc" => "DESC",
        other => {
            return Err(AppError::Validation(format!(
                "sort direction '{other}' is not supported"
            )));
        }
    };

    Ok((field, direction))
}

fn validate_page_status(status: &str) -> Result<(), AppError> {
    match status {
        "draft" | "pending_review" | "published" | "archived" => Ok(()),
        other => Err(AppError::Validation(format!(
            "status '{other}' is not supported"
        ))),
    }
}

fn is_valid_prop_name(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() || first == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

async fn preview_sender(state: &AppState, page_id: Uuid) -> tokio::sync::broadcast::Sender<String> {
    if let Some(sender) = state
        .page_preview_channels
        .read()
        .await
        .get(&page_id)
        .cloned()
    {
        return sender;
    }

    let mut channels = state.page_preview_channels.write().await;
    channels
        .entry(page_id)
        .or_insert_with(|| tokio::sync::broadcast::channel(32).0)
        .clone()
}

async fn broadcast_page_json(state: &AppState, page_id: Uuid, page_json: &Value) {
    let Ok(message) = serde_json::to_string(page_json) else {
        return;
    };
    let Some(sender) = state
        .page_preview_channels
        .read()
        .await
        .get(&page_id)
        .cloned()
    else {
        return;
    };
    let _ = sender.send(message);
}

fn default_page_json() -> Value {
    serde_json::json!({
        "version": "1.0",
        "metadata": {},
        "layout": {
            "id": "root",
            "type": "root",
            "children": []
        }
    })
}

fn empty_object() -> Value {
    Value::Object(Map::new())
}
