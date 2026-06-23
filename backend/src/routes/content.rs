use axum::extract::{Extension, Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::plugins;
use crate::routes::delivery;
use crate::services::entry_validation::{is_valid_slug, parse_fields, validate_entry_data};
use crate::services::{rbac, rls, security, webhooks, workflow};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/content-types",
            get(list_content_types).post(create_content_type),
        )
        .route(
            "/api/content-types/{id}",
            get(get_content_type)
                .put(update_content_type)
                .delete(delete_content_type),
        )
        .route(
            "/api/entries/{type_slug}",
            get(list_entries).post(create_entry),
        )
        .route(
            "/api/entries/{type_slug}/{id}",
            get(get_entry).put(update_entry).delete(delete_entry),
        )
        .route(
            "/api/entries/{type_slug}/{id}/submit-review",
            post(submit_entry_for_review),
        )
        .route("/api/entries/{type_slug}/{id}/publish", post(publish_entry))
        .route("/api/entries/{type_slug}/{id}/reject", post(reject_entry))
        .route("/api/entries/{type_slug}/{id}/archive", post(archive_entry))
        .route("/api/entries/{type_slug}/{id}/restore", post(restore_entry))
        .route(
            "/api/entries/{type_slug}/{id}/unpublish",
            post(unpublish_entry),
        )
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ContentTypeRequest {
    pub name: String,
    pub slug: String,
    pub fields: Value,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ContentTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub fields: Value,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteConfirm {
    pub confirm: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct EntryListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EntryRequest {
    pub data: Value,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ContentEntryResponse {
    pub id: Uuid,
    pub type_id: Uuid,
    pub data: Value,
    pub status: String,
    pub version: i32,
    pub author_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EntryListResponse {
    pub data: Vec<ContentEntryResponse>,
    pub page: i64,
    pub per_page: i64,
}

#[utoipa::path(
    get,
    path = "/api/content-types",
    tag = "content",
    responses((status = 200, description = "Content types", body = [ContentTypeResponse]))
)]
pub async fn list_content_types(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<ContentTypeResponse>>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, ContentTypeResponse>(
        r#"
        SELECT id, name, slug, fields, created_by, created_at, updated_at
        FROM content_types
        WHERE organization_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_all(db.as_mut())
    .await?;

    Ok(Json(rows))
}

#[utoipa::path(
    post,
    path = "/api/content-types",
    tag = "content",
    request_body = ContentTypeRequest,
    responses((status = 200, description = "Created content type", body = ContentTypeResponse))
)]
pub async fn create_content_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<ContentTypeRequest>,
) -> Result<Json<ContentTypeResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_content_type_manager(&tenant.role)?;
    validate_content_type_request(&payload)?;

    let row = sqlx::query_as::<_, ContentTypeResponse>(
        r#"
        INSERT INTO content_types (organization_id, name, slug, fields, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, slug, fields, created_by, created_at, updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(payload.name.trim())
    .bind(payload.slug.trim())
    .bind(payload.fields)
    .bind(claims.sub)
    .fetch_one(db.as_mut())
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/content-types/{id}",
    tag = "content",
    params(("id" = Uuid, Path, description = "Content type id")),
    responses((status = 200, description = "Content type", body = ContentTypeResponse))
)]
pub async fn get_content_type(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<ContentTypeResponse>, AppError> {
    load_content_type_by_id(&state, &tenant, id).await.map(Json)
}

#[utoipa::path(
    put,
    path = "/api/content-types/{id}",
    tag = "content",
    params(("id" = Uuid, Path, description = "Content type id")),
    request_body = ContentTypeRequest,
    responses((status = 200, description = "Updated content type", body = ContentTypeResponse))
)]
pub async fn update_content_type(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ContentTypeRequest>,
) -> Result<Json<ContentTypeResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_content_type_manager(&tenant.role)?;
    validate_content_type_request(&payload)?;

    let row = sqlx::query_as::<_, ContentTypeResponse>(
        r#"
        UPDATE content_types
        SET name = $2,
            slug = $3,
            fields = $4,
            updated_at = now()
        WHERE id = $1 AND organization_id = $5
        RETURNING id, name, slug, fields, created_by, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(payload.name.trim())
    .bind(payload.slug.trim())
    .bind(payload.fields)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/api/content-types/{id}",
    tag = "content",
    params(("id" = Uuid, Path, description = "Content type id")),
    responses((status = 200, description = "Deleted content type", body = ContentTypeResponse))
)]
pub async fn delete_content_type(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
    Query(query): Query<DeleteConfirm>,
) -> Result<Json<ContentTypeResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_content_type_manager(&tenant.role)?;
    if query.confirm != Some(true) {
        return Err(AppError::Validation(
            "pass ?confirm=true to delete a content type".to_owned(),
        ));
    }

    let row = sqlx::query_as::<_, ContentTypeResponse>(
        r#"
        DELETE FROM content_types
        WHERE id = $1 AND organization_id = $2
        RETURNING id, name, slug, fields, created_by, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/entries/{type_slug}",
    tag = "entries",
    params(("type_slug" = String, Path, description = "Content type slug")),
    responses((status = 200, description = "Entries", body = EntryListResponse))
)]
pub async fn list_entries(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(type_slug): Path<String>,
    Query(query): Query<EntryListQuery>,
) -> Result<Json<EntryListResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;
    let (sort_column, sort_direction) = parse_sort(query.sort.as_deref())?;

    if let Some(status) = query.status.as_deref() {
        validate_status(status)?;
        let sql = format!(
            r#"
            SELECT e.id,
                   e.type_id,
                   e.data,
                   e.status::text as status,
                   e.version,
                   e.author_id,
                   e.published_at,
                   e.created_at,
                   e.updated_at
            FROM content_entries e
            JOIN content_types ct ON ct.id = e.type_id
            WHERE ct.organization_id = $1
              AND e.organization_id = $1
              AND ct.slug = $2
              AND e.status::text = $3
            ORDER BY e.{sort_column} {sort_direction}
            LIMIT $4 OFFSET $5
            "#
        );
        let data = sqlx::query_as::<_, ContentEntryResponse>(&sql)
            .bind(tenant.organization_id)
            .bind(&type_slug)
            .bind(status)
            .bind(per_page)
            .bind(offset)
            .fetch_all(db.as_mut())
            .await?;

        return Ok(Json(EntryListResponse {
            data,
            page,
            per_page,
        }));
    }

    let sql = format!(
        r#"
        SELECT e.id,
               e.type_id,
               e.data,
               e.status::text as status,
               e.version,
               e.author_id,
               e.published_at,
               e.created_at,
               e.updated_at
        FROM content_entries e
        JOIN content_types ct ON ct.id = e.type_id
        WHERE ct.organization_id = $1
          AND e.organization_id = $1
          AND ct.slug = $2
        ORDER BY e.{sort_column} {sort_direction}
        LIMIT $3 OFFSET $4
        "#
    );
    let data = sqlx::query_as::<_, ContentEntryResponse>(&sql)
        .bind(tenant.organization_id)
        .bind(&type_slug)
        .bind(per_page)
        .bind(offset)
        .fetch_all(db.as_mut())
        .await?;

    Ok(Json(EntryListResponse {
        data,
        page,
        per_page,
    }))
}

#[utoipa::path(
    post,
    path = "/api/entries/{type_slug}",
    tag = "entries",
    params(("type_slug" = String, Path, description = "Content type slug")),
    request_body = EntryRequest,
    responses((status = 200, description = "Created entry", body = ContentEntryResponse))
)]
pub async fn create_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(type_slug): Path<String>,
    Json(payload): Json<EntryRequest>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_entry_writer(&tenant.role)?;
    let content_type = load_content_type_by_slug(&state, &tenant, &type_slug).await?;
    let fields = parse_fields(&content_type.fields)?;
    let data = plugins::run_entry_before_save(
        &state,
        tenant.organization_id,
        &type_slug,
        payload.data,
        claims.sub,
    )
    .await?;
    let data = security::sanitize_entry_data(&fields, data);
    validate_entry_data(&fields, &data)?;

    let row = sqlx::query_as::<_, ContentEntryResponse>(
        r#"
        INSERT INTO content_entries (organization_id, type_id, data, author_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id,
                  type_id,
                  data,
                  status::text as status,
                  version,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(content_type.id)
    .bind(data)
    .bind(claims.sub)
    .fetch_one(db.as_mut())
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/entries/{type_slug}/{id}",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Entry", body = ContentEntryResponse))
)]
pub async fn get_entry(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    load_entry(&state, &tenant, &type_slug, id).await.map(Json)
}

#[utoipa::path(
    put,
    path = "/api/entries/{type_slug}/{id}",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    request_body = EntryRequest,
    responses((status = 200, description = "Updated entry", body = ContentEntryResponse))
)]
pub async fn update_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
    Json(payload): Json<EntryRequest>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_entry_writer(&tenant.role)?;
    let content_type = load_content_type_by_slug(&state, &tenant, &type_slug).await?;
    let fields = parse_fields(&content_type.fields)?;
    let data = plugins::run_entry_before_save(
        &state,
        tenant.organization_id,
        &type_slug,
        payload.data,
        claims.sub,
    )
    .await?;
    let data = security::sanitize_entry_data(&fields, data);
    validate_entry_data(&fields, &data)?;

    let row = sqlx::query_as::<_, ContentEntryResponse>(
        r#"
        UPDATE content_entries
        SET data = $3,
            version = version + 1,
            updated_at = now()
        WHERE id = $1 AND type_id = $2 AND organization_id = $4
        RETURNING id,
                  type_id,
                  data,
                  status::text as status,
                  version,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .bind(content_type.id)
    .bind(data)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;

    if row.status == "published" {
        delivery::invalidate_content_cache(&state, tenant.organization_id, &type_slug).await;
    }

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/api/entries/{type_slug}/{id}",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Deleted entry", body = ContentEntryResponse))
)]
pub async fn delete_entry(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_EDITOR])?;
    let content_type = load_content_type_by_slug(&state, &tenant, &type_slug).await?;

    let row = sqlx::query_as::<_, ContentEntryResponse>(
        r#"
        DELETE FROM content_entries
        WHERE id = $1 AND type_id = $2 AND organization_id = $3
        RETURNING id,
                  type_id,
                  data,
                  status::text as status,
                  version,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .bind(content_type.id)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;

    if row.status == "published" {
        delivery::invalidate_content_cache(&state, tenant.organization_id, &type_slug).await;
    }

    Ok(Json(row))
}

#[utoipa::path(
    post,
    path = "/api/entries/{type_slug}/{id}/submit-review",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Submitted entry for review", body = ContentEntryResponse))
)]
pub async fn submit_entry_for_review(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    rbac::require_org_entry_writer(&tenant.role)?;
    let entry = transition_entry(
        &state,
        &tenant,
        &type_slug,
        id,
        workflow::WorkflowStatus::PendingReview,
        false,
    )
    .await?;
    Ok(Json(entry))
}

#[utoipa::path(
    post,
    path = "/api/entries/{type_slug}/{id}/publish",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Published entry", body = ContentEntryResponse))
)]
pub async fn publish_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    rbac::require_org_entry_publisher(&tenant.role)?;
    let entry = transition_entry(
        &state,
        &tenant,
        &type_slug,
        id,
        workflow::WorkflowStatus::Published,
        true,
    )
    .await?;
    delivery::invalidate_content_cache(&state, tenant.organization_id, &type_slug).await;
    plugins::run_entry_after_publish(
        &state,
        tenant.organization_id,
        &type_slug,
        entry.data.clone(),
        claims.sub,
    )
    .await?;
    webhooks::trigger_event(
        &state,
        tenant.organization_id,
        webhooks::ENTRY_PUBLISH,
        entry_webhook_payload(webhooks::ENTRY_PUBLISH, &tenant, &type_slug, &entry),
    )
    .await;
    Ok(Json(entry))
}

#[utoipa::path(
    post,
    path = "/api/entries/{type_slug}/{id}/unpublish",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Unpublished entry", body = ContentEntryResponse))
)]
pub async fn unpublish_entry(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    rbac::require_org_entry_publisher(&tenant.role)?;
    let entry = transition_entry(
        &state,
        &tenant,
        &type_slug,
        id,
        workflow::WorkflowStatus::Draft,
        true,
    )
    .await?;
    delivery::invalidate_content_cache(&state, tenant.organization_id, &type_slug).await;
    webhooks::trigger_event(
        &state,
        tenant.organization_id,
        webhooks::ENTRY_UNPUBLISH,
        entry_webhook_payload(webhooks::ENTRY_UNPUBLISH, &tenant, &type_slug, &entry),
    )
    .await;
    Ok(Json(entry))
}

#[utoipa::path(
    post,
    path = "/api/entries/{type_slug}/{id}/reject",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Rejected entry", body = ContentEntryResponse))
)]
pub async fn reject_entry(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    rbac::require_org_workflow_reviewer(&tenant.role)?;
    let entry = transition_entry(
        &state,
        &tenant,
        &type_slug,
        id,
        workflow::WorkflowStatus::Draft,
        true,
    )
    .await?;
    Ok(Json(entry))
}

#[utoipa::path(
    post,
    path = "/api/entries/{type_slug}/{id}/archive",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Archived entry", body = ContentEntryResponse))
)]
pub async fn archive_entry(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    rbac::require_org_workflow_reviewer(&tenant.role)?;
    let entry = transition_entry(
        &state,
        &tenant,
        &type_slug,
        id,
        workflow::WorkflowStatus::Archived,
        true,
    )
    .await?;
    delivery::invalidate_content_cache(&state, tenant.organization_id, &type_slug).await;
    Ok(Json(entry))
}

#[utoipa::path(
    post,
    path = "/api/entries/{type_slug}/{id}/restore",
    tag = "entries",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id" = Uuid, Path, description = "Entry id")
    ),
    responses((status = 200, description = "Restored entry", body = ContentEntryResponse))
)]
pub async fn restore_entry(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path((type_slug, id)): Path<(String, Uuid)>,
) -> Result<Json<ContentEntryResponse>, AppError> {
    rbac::require_org_workflow_reviewer(&tenant.role)?;
    let entry = transition_entry(
        &state,
        &tenant,
        &type_slug,
        id,
        workflow::WorkflowStatus::Draft,
        true,
    )
    .await?;
    Ok(Json(entry))
}

fn entry_webhook_payload(
    event: &str,
    tenant: &TenantContext,
    type_slug: &str,
    entry: &ContentEntryResponse,
) -> Value {
    serde_json::json!({
        "event": event,
        "entity": "entry",
        "organization_id": tenant.organization_id,
        "organization_slug": tenant.organization_slug,
        "type_slug": type_slug,
        "entry": entry,
    })
}

async fn transition_entry(
    state: &AppState,
    tenant: &TenantContext,
    type_slug: &str,
    id: Uuid,
    status: workflow::WorkflowStatus,
    can_bypass_review: bool,
) -> Result<ContentEntryResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let content_type = load_content_type_by_slug(state, tenant, type_slug).await?;
    let current = load_entry(state, tenant, type_slug, id).await?;
    workflow::require_transition(&current.status, status, can_bypass_review)?;
    let next_status = status.as_str();
    let published_at_sql = if status == workflow::WorkflowStatus::Published {
        "now()"
    } else {
        "NULL"
    };
    let sql = format!(
        r#"
        UPDATE content_entries
        SET status = $3::content_status,
            published_at = {published_at_sql},
            version = version + 1,
            updated_at = now()
        WHERE id = $1 AND type_id = $2 AND organization_id = $4
        RETURNING id,
                  type_id,
                  data,
                  status::text as status,
                  version,
                  author_id,
                  published_at,
                  created_at,
                  updated_at
        "#
    );

    sqlx::query_as::<_, ContentEntryResponse>(&sql)
        .bind(id)
        .bind(content_type.id)
        .bind(next_status)
        .bind(tenant.organization_id)
        .fetch_one(db.as_mut())
        .await
        .map_err(AppError::from)
}

async fn load_content_type_by_id(
    state: &AppState,
    tenant: &TenantContext,
    id: Uuid,
) -> Result<ContentTypeResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    sqlx::query_as::<_, ContentTypeResponse>(
        r#"
        SELECT id, name, slug, fields, created_by, created_at, updated_at
        FROM content_types
        WHERE id = $1 AND organization_id = $2
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_content_type_by_slug(
    state: &AppState,
    tenant: &TenantContext,
    slug: &str,
) -> Result<ContentTypeResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    sqlx::query_as::<_, ContentTypeResponse>(
        r#"
        SELECT id, name, slug, fields, created_by, created_at, updated_at
        FROM content_types
        WHERE slug = $1 AND organization_id = $2
        "#,
    )
    .bind(slug)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_entry(
    state: &AppState,
    tenant: &TenantContext,
    type_slug: &str,
    id: Uuid,
) -> Result<ContentEntryResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    sqlx::query_as::<_, ContentEntryResponse>(
        r#"
        SELECT e.id,
               e.type_id,
               e.data,
               e.status::text as status,
               e.version,
               e.author_id,
               e.published_at,
               e.created_at,
               e.updated_at
        FROM content_entries e
        JOIN content_types ct ON ct.id = e.type_id
        WHERE ct.organization_id = $1
          AND e.organization_id = $1
          AND ct.slug = $2
          AND e.id = $3
        "#,
    )
    .bind(tenant.organization_id)
    .bind(type_slug)
    .bind(id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

fn validate_content_type_request(payload: &ContentTypeRequest) -> Result<(), AppError> {
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("name is required".to_owned()));
    }
    if !is_valid_slug(payload.slug.trim()) {
        return Err(AppError::Validation("slug is invalid".to_owned()));
    }
    parse_fields(&payload.fields)?;
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

fn validate_status(status: &str) -> Result<(), AppError> {
    workflow::WorkflowStatus::parse(status)?;
    Ok(())
}
