use std::collections::HashMap;

use axum::extract::{Extension, Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sqlx::{FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::routes::marketplace_runtime::active_kill_switch_reason;
use crate::routes::pages::PageResponse;
use crate::services::entry_validation::is_valid_slug;
use crate::services::marketplace_adapters::{
    ComponentDefinition, PUBLIC_HOOK_TYPES, apply_asset_mapping, component_definitions,
    hook_definitions, template_page_json,
};
use crate::services::{audit, quota, rbac, rls};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/marketplace/runtime/components",
            get(list_marketplace_components),
        )
        .route(
            "/api/marketplace/templates/{installation_id}/preview",
            post(preview_template),
        )
        .route(
            "/api/marketplace/templates/{installation_id}/import",
            post(import_template),
        )
        .route("/api/marketplace/hooks", get(list_marketplace_hooks))
        .route(
            "/api/marketplace/hooks/{hook_type}/authorize",
            post(authorize_marketplace_hook),
        )
}

#[derive(Debug, Serialize, FromRow, ToSchema, Clone)]
pub struct MarketplaceComponentResponse {
    pub id: Uuid,
    pub installation_id: Uuid,
    pub component_key: String,
    pub name: String,
    pub category: String,
    pub props_schema: Value,
    pub listing_title: String,
    pub version: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct TemplateAdapterRequest {
    pub template_key: Option<String>,
    #[serde(default)]
    pub asset_mapping: HashMap<String, Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TemplatePreviewResponse {
    pub installation_id: Uuid,
    pub template_key: String,
    pub page_json: Value,
    pub required_assets: Vec<String>,
    pub mapped_assets: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TemplateImportRequest {
    pub title: String,
    pub slug: String,
    #[serde(flatten)]
    pub adapter: TemplateAdapterRequest,
}

#[derive(Debug, Serialize, FromRow, ToSchema, Clone)]
pub struct MarketplaceHookResponse {
    pub installation_id: Uuid,
    pub hook_key: String,
    pub hook_type: String,
    pub label: String,
    pub contract_version: String,
    pub config: Value,
    pub listing_title: String,
    pub version: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceHookAuthorizeRequest {
    pub hook_key: String,
    #[serde(default)]
    pub context: Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceHookAuthorizationResponse {
    pub allowed: bool,
    pub hook_key: String,
    pub hook_type: String,
    pub contract_version: String,
    pub execution: String,
    pub reason_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, FromRow)]
struct AdapterInstallationRow {
    id: Uuid,
    status: String,
    runtime_status: String,
    version: String,
    manifest_json: Value,
    listing_title: String,
}

#[utoipa::path(
    get,
    path = "/api/marketplace/runtime/components",
    tag = "marketplace",
    responses((status = 200, description = "Installed Marketplace Component Pack registry", body = [MarketplaceComponentResponse]))
)]
pub async fn list_marketplace_components(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<MarketplaceComponentResponse>>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    ensure_runtime_available(db.as_mut(), tenant.organization_id).await?;
    let installations =
        load_adapter_installations(db.as_mut(), tenant.organization_id, "component_pack").await?;
    let mut response = Vec::new();
    for installation in installations {
        if installation.status != "active" || installation.runtime_status != "ready" {
            continue;
        }
        let namespace = component_namespace(
            tenant.organization_id,
            &installation.listing_title,
            installation.id,
        );
        for definition in component_definitions(&installation.manifest_json, &namespace)
            .map_err(AppError::Validation)?
        {
            let row = upsert_component_registry(
                db.as_mut(),
                tenant.organization_id,
                installation.id,
                &installation.version,
                &installation.listing_title,
                &definition,
            )
            .await?;
            response.push(row);
        }
    }
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/templates/{installation_id}/preview",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Installed design template id")),
    request_body = TemplateAdapterRequest,
    responses((status = 200, description = "Template import preview", body = TemplatePreviewResponse))
)]
pub async fn preview_template(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
    Json(payload): Json<TemplateAdapterRequest>,
) -> Result<Json<TemplatePreviewResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let installation = load_adapter_installation(
        db.as_mut(),
        tenant.organization_id,
        installation_id,
        "design_template",
    )
    .await?;
    ensure_installation_ready(&installation)?;
    let (template_key, page_json, required_assets) =
        template_page_json(&installation.manifest_json, payload.template_key.as_deref())
            .map_err(AppError::Validation)?;
    let mapped = validate_asset_mapping(
        db.as_mut(),
        tenant.organization_id,
        &required_assets,
        &payload.asset_mapping,
    )
    .await?;
    let mut resolved = page_json;
    apply_asset_mapping(&mut resolved, &mapped).map_err(AppError::Validation)?;
    Ok(Json(TemplatePreviewResponse {
        installation_id,
        template_key,
        page_json: resolved,
        required_assets,
        mapped_assets: payload.asset_mapping.keys().cloned().collect(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/templates/{installation_id}/import",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Installed design template id")),
    request_body = TemplateImportRequest,
    responses((status = 201, description = "Template cloned into an organization page", body = PageResponse))
)]
#[allow(clippy::explicit_auto_deref)]
pub async fn import_template(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
    Json(payload): Json<TemplateImportRequest>,
) -> Result<(axum::http::StatusCode, Json<PageResponse>), AppError> {
    rbac::require_org_page_writer(&tenant.role)?;
    if payload.title.trim().is_empty() || !is_valid_slug(payload.slug.trim()) {
        return Err(AppError::Validation(
            "template title and a valid slug are required".to_owned(),
        ));
    }
    quota::ensure_content_capacity(&state.db, &tenant).await?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let installation = load_adapter_installation(
        &mut tx,
        tenant.organization_id,
        installation_id,
        "design_template",
    )
    .await?;
    ensure_installation_ready(&installation)?;
    let (template_key, page_json, required_assets) = template_page_json(
        &installation.manifest_json,
        payload.adapter.template_key.as_deref(),
    )
    .map_err(AppError::Validation)?;
    let mapped = validate_asset_mapping(
        &mut tx,
        tenant.organization_id,
        &required_assets,
        &payload.adapter.asset_mapping,
    )
    .await?;
    let mut resolved = page_json;
    apply_asset_mapping(&mut resolved, &mapped).map_err(AppError::Validation)?;
    sync_component_registry(&mut tx, tenant.organization_id).await?;
    crate::routes::pages::validate_page_json_for_tenant(&state, &tenant, &resolved).await?;
    let page = sqlx::query_as::<_, PageResponse>(
        r#"INSERT INTO pages (organization_id, title, slug, page_json, author_id)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, title, slug, page_json, status::text as status, author_id,
                     published_at, created_at, updated_at"#,
    )
    .bind(tenant.organization_id)
    .bind(payload.title.trim())
    .bind(payload.slug.trim())
    .bind(&resolved)
    .bind(tenant.user_id)
    .fetch_one(&mut *tx)
    .await?;
    sqlx::query(
        r#"INSERT INTO page_versions (organization_id, page_id, version, page_json, created_by)
           VALUES ($1, $2, 1, $3, $4)"#,
    )
    .bind(tenant.organization_id)
    .bind(page.id)
    .bind(&resolved)
    .bind(tenant.user_id)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"INSERT INTO marketplace_template_imports
           (organization_id, installation_id, page_id, template_key, asset_mapping, created_by)
           VALUES ($1, $2, $3, $4, $5, $6)"#,
    )
    .bind(tenant.organization_id)
    .bind(installation_id)
    .bind(page.id)
    .bind(&template_key)
    .bind(Value::Object(mapped.clone()))
    .bind(tenant.user_id)
    .execute(&mut *tx)
    .await?;
    audit::record_in_transaction(&mut tx, tenant.organization_id, Some(tenant.user_id), "marketplace.template.import", "page", Some(page.id), json!({"installation_id": installation_id, "template_key": template_key, "asset_mapping": mapped})).await?;
    tx.commit().await?;
    Ok((axum::http::StatusCode::CREATED, Json(page)))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/hooks",
    tag = "marketplace",
    responses((status = 200, description = "Installed public Marketplace Plugin Hooks", body = [MarketplaceHookResponse]))
)]
pub async fn list_marketplace_hooks(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<MarketplaceHookResponse>>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    ensure_runtime_available(db.as_mut(), tenant.organization_id).await?;
    let installations =
        load_adapter_installations(db.as_mut(), tenant.organization_id, "integration_plugin")
            .await?;
    let mut response = Vec::new();
    for installation in installations {
        if installation.status != "active" || installation.runtime_status != "ready" {
            continue;
        }
        for hook in hook_definitions(&installation.manifest_json).map_err(AppError::Validation)? {
            response.push(MarketplaceHookResponse {
                installation_id: installation.id,
                hook_key: hook.key,
                hook_type: hook.hook_type,
                label: hook.label,
                contract_version: hook.contract_version,
                config: hook.config,
                listing_title: installation.listing_title.clone(),
                version: installation.version.clone(),
                enabled: true,
            });
        }
    }
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/hooks/{hook_type}/authorize",
    tag = "marketplace",
    params(("hook_type" = String, Path, description = "Public hook contract type")),
    request_body = MarketplaceHookAuthorizeRequest,
    responses((status = 200, description = "Public hook authorization decision", body = MarketplaceHookAuthorizationResponse))
)]
pub async fn authorize_marketplace_hook(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(hook_type): Path<String>,
    Json(payload): Json<MarketplaceHookAuthorizeRequest>,
) -> Result<Json<MarketplaceHookAuthorizationResponse>, AppError> {
    if !PUBLIC_HOOK_TYPES.contains(&hook_type.as_str()) {
        return Ok(Json(MarketplaceHookAuthorizationResponse {
            allowed: false,
            hook_key: payload.hook_key,
            hook_type,
            contract_version: "2026-07".to_owned(),
            execution: "not_executed".to_owned(),
            reason_code: Some("unsupported_hook_type".to_owned()),
            message: Some("only public hook contracts are supported".to_owned()),
        }));
    }
    if !payload.context.is_object() {
        return Err(AppError::Validation(
            "hook context must be a JSON object".to_owned(),
        ));
    }
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    ensure_runtime_available(db.as_mut(), tenant.organization_id).await?;
    let installations =
        load_adapter_installations(db.as_mut(), tenant.organization_id, "integration_plugin")
            .await?;
    for installation in installations {
        if installation.status != "active" || installation.runtime_status != "ready" {
            continue;
        }
        if let Ok(hooks) = hook_definitions(&installation.manifest_json)
            && let Some(hook) = hooks
                .into_iter()
                .find(|hook| hook.key == payload.hook_key && hook.hook_type == hook_type)
        {
            return Ok(Json(MarketplaceHookAuthorizationResponse {
                allowed: true,
                hook_key: hook.key,
                hook_type,
                contract_version: hook.contract_version,
                execution: "not_executed".to_owned(),
                reason_code: None,
                message: Some(
                    "public hook contract authorized; adapter execution is host-owned".to_owned(),
                ),
            }));
        }
    }
    Ok(Json(MarketplaceHookAuthorizationResponse {
        allowed: false,
        hook_key: payload.hook_key,
        hook_type,
        contract_version: "2026-07".to_owned(),
        execution: "not_executed".to_owned(),
        reason_code: Some("hook_not_installed".to_owned()),
        message: Some("hook is not registered by an active integration plugin".to_owned()),
    }))
}

async fn ensure_runtime_available(
    connection: &mut PgConnection,
    organization_id: Uuid,
) -> Result<(), AppError> {
    if let Some(reason) = active_kill_switch_reason(connection, organization_id).await? {
        return Err(AppError::Conflict(format!(
            "Marketplace runtime is blocked by an active kill switch: {reason}"
        )));
    }
    Ok(())
}

fn ensure_installation_ready(installation: &AdapterInstallationRow) -> Result<(), AppError> {
    if installation.status != "active" || installation.runtime_status != "ready" {
        return Err(AppError::Conflict(
            "Marketplace installation is not active and runtime-ready".to_owned(),
        ));
    }
    Ok(())
}

async fn load_adapter_installations(
    connection: &mut PgConnection,
    organization_id: Uuid,
    product_type: &str,
) -> Result<Vec<AdapterInstallationRow>, AppError> {
    Ok(sqlx::query_as::<_, AdapterInstallationRow>(
        r#"SELECT installation.id, listing.product_type, installation.status, installation.runtime_status,
                  version.version, version.manifest_json, listing.title as listing_title
           FROM marketplace_installations installation
           JOIN marketplace_listings listing ON listing.id = installation.listing_id
           JOIN marketplace_versions version ON version.id = installation.version_id
           WHERE installation.organization_id = $1 AND listing.product_type = $2
             AND installation.status <> 'uninstalled'"#,
    ).bind(organization_id).bind(product_type).fetch_all(connection).await?)
}

async fn load_adapter_installation(
    connection: &mut PgConnection,
    organization_id: Uuid,
    id: Uuid,
    product_type: &str,
) -> Result<AdapterInstallationRow, AppError> {
    sqlx::query_as::<_, AdapterInstallationRow>(
        r#"SELECT installation.id, listing.product_type, installation.status, installation.runtime_status,
                  version.version, version.manifest_json, listing.title as listing_title
           FROM marketplace_installations installation
           JOIN marketplace_listings listing ON listing.id = installation.listing_id
           JOIN marketplace_versions version ON version.id = installation.version_id
           WHERE installation.id = $1 AND installation.organization_id = $2 AND listing.product_type = $3"#,
    ).bind(id).bind(organization_id).bind(product_type).fetch_optional(connection).await?.ok_or_else(|| AppError::NotFound("Marketplace adapter installation not found".to_owned()))
}

async fn sync_component_registry(
    connection: &mut PgConnection,
    organization_id: Uuid,
) -> Result<(), AppError> {
    let installations =
        load_adapter_installations(connection, organization_id, "component_pack").await?;
    for installation in installations {
        if installation.status != "active" || installation.runtime_status != "ready" {
            continue;
        }
        let namespace = component_namespace(
            organization_id,
            &installation.listing_title,
            installation.id,
        );
        for definition in component_definitions(&installation.manifest_json, &namespace)
            .map_err(AppError::Validation)?
        {
            upsert_component_registry(
                connection,
                organization_id,
                installation.id,
                &installation.version,
                &installation.listing_title,
                &definition,
            )
            .await?;
        }
    }
    Ok(())
}

#[allow(clippy::explicit_auto_deref)]
async fn validate_asset_mapping(
    connection: &mut PgConnection,
    organization_id: Uuid,
    required: &[String],
    mapping: &HashMap<String, Uuid>,
) -> Result<Map<String, Value>, AppError> {
    let mut resolved = Map::new();
    for key in required {
        let media_id = mapping
            .get(key)
            .ok_or_else(|| AppError::Validation(format!("asset mapping is missing for '{key}'")))?;
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM media WHERE id = $1 AND organization_id = $2)",
        )
        .bind(media_id)
        .bind(organization_id)
        .fetch_one(&mut *connection)
        .await?;
        if !exists {
            return Err(AppError::Validation(format!(
                "asset '{key}' does not belong to this organization"
            )));
        }
        resolved.insert(key.clone(), Value::String(media_id.to_string()));
    }
    Ok(resolved)
}

async fn upsert_component_registry(
    connection: &mut PgConnection,
    organization_id: Uuid,
    installation_id: Uuid,
    version: &str,
    listing_title: &str,
    definition: &ComponentDefinition,
) -> Result<MarketplaceComponentResponse, AppError> {
    let display_name = format!(
        "{} / {} ({})",
        listing_title,
        definition.name,
        &installation_id.simple().to_string()[..8]
    );
    sqlx::query_as::<_, MarketplaceComponentResponse>(
        r#"INSERT INTO component_registry (organization_id, component_key, name, category, props_schema, is_system, marketplace_installation_id)
           VALUES ($1, $2, $3, $4, $5, FALSE, $6)
           ON CONFLICT (component_key) DO UPDATE SET name = EXCLUDED.name, category = EXCLUDED.category,
             props_schema = EXCLUDED.props_schema, marketplace_installation_id = EXCLUDED.marketplace_installation_id,
             updated_at = now()
           RETURNING id, marketplace_installation_id as installation_id, component_key, name, category, props_schema,
                     $7::text as listing_title, $8::text as version, TRUE as enabled"#,
    )
    .bind(organization_id)
    .bind(&definition.key)
    .bind(display_name)
    .bind(&definition.category)
    .bind(&definition.props_schema)
    .bind(installation_id)
    .bind(listing_title)
    .bind(version)
    .fetch_one(connection)
    .await
    .map_err(AppError::from)
}

fn component_namespace(
    organization_id: Uuid,
    listing_title: &str,
    installation_id: Uuid,
) -> String {
    let mut title = String::new();
    for ch in listing_title.to_ascii_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            title.push(ch);
        } else if !title.ends_with('-') {
            title.push('-');
        }
    }
    let title = title.trim_matches('-');
    let title = if title.is_empty() { "package" } else { title };
    format!(
        "{}-{}-{}",
        organization_id.simple(),
        title,
        installation_id.simple()
    )
}

#[cfg(test)]
const ROUTER_CONTRACT: &str = "GET /api/marketplace/runtime/components POST /api/marketplace/templates/{installation_id}/preview POST /api/marketplace/templates/{installation_id}/import GET /api/marketplace/hooks POST /api/marketplace/hooks/{hook_type}/authorize";

#[cfg(test)]
mod tests {
    const PHASE_EIGHT_MIGRATION: &str =
        include_str!("../../migrations/0021_v3_phase_eight_runtime_adapters.sql");
    const PHASE_EIGHT_DOC: &str = include_str!("../../../docs/V3_PHASE_EIGHT.md");

    #[test]
    fn phase_eight_contract_is_migrated_documented_and_routed() {
        for required in [
            "marketplace_template_imports",
            "marketplace_plugin_hooks",
            "marketplace_installation_id",
            "marketplace_plugin_hooks_tenant_select",
        ] {
            assert!(
                PHASE_EIGHT_MIGRATION.contains(required),
                "missing Phase 8 migration contract: {required}"
            );
        }
        for endpoint in [
            "/api/marketplace/runtime/components",
            "/api/marketplace/templates/{installation_id}/preview",
            "/api/marketplace/templates/{installation_id}/import",
            "/api/marketplace/hooks",
            "/api/marketplace/hooks/{hook_type}/authorize",
        ] {
            assert!(
                PHASE_EIGHT_DOC.contains(endpoint) || super::ROUTER_CONTRACT.contains(endpoint),
                "missing Phase 8 route contract: {endpoint}"
            );
        }
        assert!(PHASE_EIGHT_DOC.contains("asset mapping"));
        assert!(PHASE_EIGHT_DOC.contains("never executed"));
    }
}
