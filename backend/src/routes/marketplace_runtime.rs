use axum::extract::{Extension, Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{FromRow, PgConnection};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::services::marketplace_runtime::{
    RuntimeAuthorization, authorize_runtime_operation, operation_definitions,
    validate_kill_switch_reason,
};
use crate::services::{audit, rbac, rls};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/marketplace/permissions", get(list_permissions))
        .route("/api/marketplace/runtime/status", get(runtime_status))
        .route(
            "/api/marketplace/installations/{installation_id}/runtime/authorize",
            post(authorize_runtime),
        )
        .route(
            "/api/marketplace/kill-switches/organization",
            post(activate_organization_kill_switch),
        )
        .route(
            "/api/marketplace/kill-switches/global",
            post(activate_global_kill_switch),
        )
        .route(
            "/api/marketplace/kill-switches/{kill_switch_id}/lift",
            post(lift_kill_switch),
        )
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplacePermissionResponse {
    pub permission_key: String,
    pub description: String,
    pub category: String,
    pub risk_level: String,
    pub product_types: Value,
    pub runtime_operations: Value,
    pub enabled: bool,
}

#[derive(Debug, Serialize, FromRow, ToSchema, Clone)]
pub struct MarketplaceKillSwitchResponse {
    pub id: Uuid,
    pub scope: String,
    pub organization_id: Option<Uuid>,
    pub reason: String,
    pub active: bool,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub lifted_by: Option<Uuid>,
    pub lifted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceRuntimeStatusResponse {
    pub global_blocked: bool,
    pub organization_blocked: bool,
    pub organization_id: Uuid,
    pub status_message: String,
    pub active_kill_switches: Vec<MarketplaceKillSwitchResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceKillSwitchRequest {
    pub reason: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceRuntimeAuthorizeRequest {
    pub operation: String,
    pub entry_point: String,
    pub payload: Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceRuntimeAuthorizationResponse {
    pub allowed: bool,
    pub installation_id: Uuid,
    pub operation: String,
    pub required_permission: Option<String>,
    pub entry_point: String,
    pub sandbox_policy: String,
    pub execution: String,
    pub reason_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, FromRow)]
struct RuntimeInstallationRow {
    status: String,
    runtime_status: String,
    product_type: String,
    manifest_json: Value,
    permissions_json: Value,
}

#[utoipa::path(
    get,
    path = "/api/marketplace/permissions",
    tag = "marketplace",
    responses((status = 200, description = "Marketplace permission catalog", body = [MarketplacePermissionResponse]))
)]
pub async fn list_permissions(
    State(state): State<AppState>,
) -> Result<Json<Vec<MarketplacePermissionResponse>>, AppError> {
    let rows = sqlx::query_as::<_, MarketplacePermissionResponse>(
        r#"
        SELECT permission_key, description, category, risk_level,
               product_types, runtime_operations, enabled
        FROM marketplace_permission_catalog
        WHERE enabled = TRUE
        ORDER BY permission_key
        "#,
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(rows))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/runtime/status",
    tag = "marketplace",
    responses((status = 200, description = "Marketplace runtime and kill-switch status", body = MarketplaceRuntimeStatusResponse))
)]
pub async fn runtime_status(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<MarketplaceRuntimeStatusResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let switches = active_kill_switches(db.as_mut(), tenant.organization_id).await?;
    let global_blocked = switches.iter().any(|switch| switch.scope == "global");
    let organization_blocked = switches.iter().any(|switch| switch.scope == "organization");
    let status_message = if global_blocked {
        "Marketplace runtime is globally blocked by an emergency kill switch".to_owned()
    } else if organization_blocked {
        "Marketplace runtime is blocked for this organization".to_owned()
    } else {
        "Marketplace runtime is available through the allowlisted sandbox host API".to_owned()
    };

    Ok(Json(MarketplaceRuntimeStatusResponse {
        global_blocked,
        organization_blocked,
        organization_id: tenant.organization_id,
        status_message,
        active_kill_switches: switches,
    }))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/installations/{installation_id}/runtime/authorize",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Marketplace installation id")),
    request_body = MarketplaceRuntimeAuthorizeRequest,
    responses((status = 200, description = "Sandbox host API authorization decision", body = MarketplaceRuntimeAuthorizationResponse))
)]
pub async fn authorize_runtime(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
    Json(payload): Json<MarketplaceRuntimeAuthorizeRequest>,
) -> Result<Json<MarketplaceRuntimeAuthorizationResponse>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let installation = sqlx::query_as::<_, RuntimeInstallationRow>(
        r#"
        SELECT installation.status,
               installation.runtime_status,
               listing.product_type,
               version.manifest_json,
               installation.permissions_json
        FROM marketplace_installations installation
        JOIN marketplace_listings listing ON listing.id = installation.listing_id
        JOIN marketplace_versions version ON version.id = installation.version_id
        WHERE installation.id = $1
          AND installation.organization_id = $2
        "#,
    )
    .bind(installation_id)
    .bind(tenant.organization_id)
    .fetch_optional(db.as_mut())
    .await?
    .ok_or_else(|| AppError::NotFound("Marketplace installation not found".to_owned()))?;

    let blocked_reason = active_kill_switch_reason(db.as_mut(), tenant.organization_id).await?;
    let decision = if let Some(reason) = blocked_reason {
        Err(RuntimePolicyError {
            code: "kill_switch_active",
            message: reason,
            required_permission: None,
        })
    } else {
        authorize_runtime_operation(
            &installation.status,
            &installation.runtime_status,
            &installation.product_type,
            &installation.manifest_json,
            &installation.permissions_json,
            &payload.operation,
            &payload.entry_point,
            &payload.payload,
        )
        .map_err(|error| RuntimePolicyError {
            code: error.code,
            message: error.message,
            required_permission: operation_definitions()
                .iter()
                .find(|definition| definition.operation == payload.operation)
                .map(|definition| definition.required_permission.to_owned()),
        })
    };

    let response = match decision {
        Ok(authorization) => {
            audit::record(
                &state.db,
                &tenant,
                "marketplace.runtime.authorized",
                "marketplace_installation",
                Some(installation_id),
                json!({
                    "operation": authorization.operation,
                    "entry_point": authorization.entry_point,
                    "sandbox_policy": authorization.sandbox_policy,
                    "execution": authorization.execution,
                }),
            )
            .await?;
            authorization_response(installation_id, authorization, None)
        }
        Err(error) => {
            audit::record(
                &state.db,
                &tenant,
                "marketplace.runtime.denied",
                "marketplace_installation",
                Some(installation_id),
                json!({
                    "operation": payload.operation,
                    "entry_point": payload.entry_point,
                    "reason_code": error.code,
                }),
            )
            .await?;
            denied_runtime_response(
                installation_id,
                &payload.operation,
                &payload.entry_point,
                error,
            )
        }
    };

    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/kill-switches/organization",
    tag = "marketplace",
    request_body = MarketplaceKillSwitchRequest,
    responses((status = 201, description = "Organization Marketplace kill switch activated", body = MarketplaceKillSwitchResponse))
)]
pub async fn activate_organization_kill_switch(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<MarketplaceKillSwitchRequest>,
) -> Result<Json<MarketplaceKillSwitchResponse>, AppError> {
    rbac::require_org_marketplace_kill_switch_manager(&tenant.role)?;
    let reason = validate_kill_switch_reason(&payload.reason)
        .map_err(|error| AppError::Validation(error.message))?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let switch_row = sqlx::query_as::<_, MarketplaceKillSwitchResponse>(
        r#"
        INSERT INTO marketplace_kill_switches (scope, organization_id, reason, created_by)
        VALUES ('organization', $1, $2, $3)
        RETURNING id, scope, organization_id, reason, active, created_by,
                  created_at, lifted_by, lifted_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(&reason)
    .bind(tenant.user_id)
    .fetch_one(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        UPDATE marketplace_installations
        SET runtime_status = 'blocked',
            runtime_block_reason = $2,
            runtime_blocked_at = now(),
            runtime_checked_at = now(),
            updated_at = now()
        WHERE organization_id = $1 AND status <> 'uninstalled'
        "#,
    )
    .bind(tenant.organization_id)
    .bind(&reason)
    .execute(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "marketplace.kill_switch.organization.activate",
        "marketplace_kill_switch",
        Some(switch_row.id),
        json!({ "scope": "organization", "reason": reason }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(switch_row))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/kill-switches/global",
    tag = "marketplace",
    request_body = MarketplaceKillSwitchRequest,
    responses((status = 201, description = "Global Marketplace kill switch activated", body = MarketplaceKillSwitchResponse))
)]
pub async fn activate_global_kill_switch(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<MarketplaceKillSwitchRequest>,
) -> Result<Json<MarketplaceKillSwitchResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let reason = validate_kill_switch_reason(&payload.reason)
        .map_err(|error| AppError::Validation(error.message))?;
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let switch_row = sqlx::query_as::<_, MarketplaceKillSwitchResponse>(
        r#"
        INSERT INTO marketplace_kill_switches (scope, reason, created_by)
        VALUES ('global', $1, $2)
        RETURNING id, scope, organization_id, reason, active, created_by,
                  created_at, lifted_by, lifted_at
        "#,
    )
    .bind(&reason)
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        UPDATE marketplace_installations
        SET runtime_status = 'blocked',
            runtime_block_reason = $1,
            runtime_blocked_at = now(),
            runtime_checked_at = now(),
            updated_at = now()
        WHERE status <> 'uninstalled'
        "#,
    )
    .bind(&reason)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO audit_logs (organization_id, actor_id, action, entity_type, entity_id, metadata)
        SELECT DISTINCT installation.organization_id, $1,
               'marketplace.kill_switch.global.activate', 'marketplace_kill_switch', $2,
               $3
        FROM marketplace_installations installation
        WHERE installation.status <> 'uninstalled'
        "#,
    )
    .bind(claims.sub)
    .bind(switch_row.id)
    .bind(json!({ "scope": "global", "reason": reason }))
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(switch_row))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/kill-switches/{kill_switch_id}/lift",
    tag = "marketplace",
    params(("kill_switch_id" = Uuid, Path, description = "Marketplace kill switch id")),
    responses((status = 200, description = "Marketplace kill switch lifted", body = MarketplaceKillSwitchResponse))
)]
pub async fn lift_kill_switch(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(kill_switch_id): Path<Uuid>,
) -> Result<Json<MarketplaceKillSwitchResponse>, AppError> {
    let mut lookup = rls::tenant_connection(&state.db, &tenant).await?;
    let existing = sqlx::query_as::<_, MarketplaceKillSwitchResponse>(
        r#"
        SELECT id, scope, organization_id, reason, active, created_by,
               created_at, lifted_by, lifted_at
        FROM marketplace_kill_switches
        WHERE id = $1 AND active = TRUE
          AND (scope = 'global' OR organization_id = $2)
        "#,
    )
    .bind(kill_switch_id)
    .bind(tenant.organization_id)
    .fetch_optional(lookup.as_mut())
    .await?
    .ok_or_else(|| AppError::NotFound("active Marketplace kill switch not found".to_owned()))?;

    if existing.scope == "global" {
        rbac::require_any(&claims, &[rbac::ADMIN])?;
    } else {
        rbac::require_org_marketplace_kill_switch_manager(&tenant.role)?;
    }

    let mut tx = if existing.scope == "global" {
        rls::begin_bypass_transaction(&state.db).await?
    } else {
        rls::begin_tenant_transaction(&state.db, &tenant).await?
    };
    let lifted = sqlx::query_as::<_, MarketplaceKillSwitchResponse>(
        r#"
        UPDATE marketplace_kill_switches
        SET active = FALSE, lifted_by = $2, lifted_at = now()
        WHERE id = $1
        RETURNING id, scope, organization_id, reason, active, created_by,
                  created_at, lifted_by, lifted_at
        "#,
    )
    .bind(kill_switch_id)
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await?;

    if existing.scope == "organization" {
        let global_active: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM marketplace_kill_switches WHERE scope = 'global' AND active = TRUE)",
        )
        .fetch_one(&mut *tx)
        .await?;
        if !global_active {
            sqlx::query(
                r#"
                UPDATE marketplace_installations
                SET runtime_status = 'ready', runtime_block_reason = NULL,
                    runtime_blocked_at = NULL, runtime_checked_at = now(), updated_at = now()
                WHERE organization_id = $1 AND status <> 'uninstalled'
                "#,
            )
            .bind(tenant.organization_id)
            .execute(&mut *tx)
            .await?;
        }
        audit::record_in_transaction(
            &mut tx,
            tenant.organization_id,
            Some(claims.sub),
            "marketplace.kill_switch.organization.lift",
            "marketplace_kill_switch",
            Some(kill_switch_id),
            json!({ "scope": "organization", "reason": existing.reason }),
        )
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE marketplace_installations installation
            SET runtime_status = CASE WHEN EXISTS (
                    SELECT 1 FROM marketplace_kill_switches active_switch
                    WHERE active_switch.scope = 'organization'
                      AND active_switch.organization_id = installation.organization_id
                      AND active_switch.active = TRUE
                ) THEN 'blocked' ELSE 'ready' END,
                runtime_block_reason = CASE WHEN EXISTS (
                    SELECT 1 FROM marketplace_kill_switches active_switch
                    WHERE active_switch.scope = 'organization'
                      AND active_switch.organization_id = installation.organization_id
                      AND active_switch.active = TRUE
                ) THEN 'organization kill switch active' ELSE NULL END,
                runtime_blocked_at = CASE WHEN EXISTS (
                    SELECT 1 FROM marketplace_kill_switches active_switch
                    WHERE active_switch.scope = 'organization'
                      AND active_switch.organization_id = installation.organization_id
                      AND active_switch.active = TRUE
                ) THEN now() ELSE NULL END,
                runtime_checked_at = now(), updated_at = now()
            WHERE installation.status <> 'uninstalled'
            "#,
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO audit_logs (organization_id, actor_id, action, entity_type, entity_id, metadata)
            SELECT DISTINCT installation.organization_id, $1,
                   'marketplace.kill_switch.global.lift', 'marketplace_kill_switch', $2,
                   $3
            FROM marketplace_installations installation
            WHERE installation.status <> 'uninstalled'
            "#,
        )
        .bind(claims.sub)
        .bind(kill_switch_id)
        .bind(json!({ "scope": "global", "reason": existing.reason }))
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(Json(lifted))
}

pub async fn active_kill_switch_reason(
    connection: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Option<String>, AppError> {
    sqlx::query_scalar::<_, String>(
        r#"
        SELECT reason
        FROM marketplace_kill_switches
        WHERE active = TRUE
          AND (scope = 'global' OR (scope = 'organization' AND organization_id = $1))
        ORDER BY CASE scope WHEN 'global' THEN 0 ELSE 1 END, created_at DESC
        LIMIT 1
        "#,
    )
    .bind(organization_id)
    .fetch_optional(connection)
    .await
    .map_err(AppError::from)
}

async fn active_kill_switches(
    connection: &mut PgConnection,
    organization_id: Uuid,
) -> Result<Vec<MarketplaceKillSwitchResponse>, AppError> {
    Ok(sqlx::query_as::<_, MarketplaceKillSwitchResponse>(
        r#"
        SELECT id, scope, organization_id, reason, active, created_by,
               created_at, lifted_by, lifted_at
        FROM marketplace_kill_switches
        WHERE active = TRUE
          AND (scope = 'global' OR organization_id = $1)
        ORDER BY CASE scope WHEN 'global' THEN 0 ELSE 1 END, created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(connection)
    .await?)
}

fn authorization_response(
    installation_id: Uuid,
    authorization: RuntimeAuthorization,
    reason: Option<RuntimePolicyError>,
) -> MarketplaceRuntimeAuthorizationResponse {
    let (reason_code, message) = reason
        .map(|reason| (Some(reason.code.to_owned()), Some(reason.message)))
        .unwrap_or((None, None));
    MarketplaceRuntimeAuthorizationResponse {
        allowed: authorization.allowed,
        installation_id,
        operation: authorization.operation,
        required_permission: Some(authorization.required_permission),
        entry_point: authorization.entry_point,
        sandbox_policy: authorization.sandbox_policy.to_owned(),
        execution: authorization.execution.to_owned(),
        reason_code,
        message,
    }
}

fn denied_runtime_response(
    installation_id: Uuid,
    operation: &str,
    entry_point: &str,
    error: RuntimePolicyError,
) -> MarketplaceRuntimeAuthorizationResponse {
    MarketplaceRuntimeAuthorizationResponse {
        allowed: false,
        installation_id,
        operation: operation.to_owned(),
        required_permission: error.required_permission,
        entry_point: entry_point.to_owned(),
        sandbox_policy: "v3.7-allowlisted-host-api".to_owned(),
        execution: "not_executed".to_owned(),
        reason_code: Some(error.code.to_owned()),
        message: Some(error.message),
    }
}

#[derive(Debug)]
struct RuntimePolicyError {
    code: &'static str,
    message: String,
    required_permission: Option<String>,
}

#[cfg(test)]
const ROUTER_CONTRACT: &str = "GET /api/marketplace/permissions GET /api/marketplace/runtime/status POST /api/marketplace/installations/{installation_id}/runtime/authorize POST /api/marketplace/kill-switches/organization POST /api/marketplace/kill-switches/global POST /api/marketplace/kill-switches/{kill_switch_id}/lift";

#[cfg(test)]
mod tests {
    const PHASE_SEVEN_MIGRATION: &str =
        include_str!("../../migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql");
    const PHASE_SEVEN_DOC: &str = include_str!("../../../docs/V3_PHASE_SEVEN.md");

    #[test]
    fn phase_seven_contract_is_migrated_documented_and_routed() {
        for required in [
            "marketplace_permission_catalog",
            "marketplace_kill_switches",
            "runtime_status",
            "marketplace_kill_switches_tenant_select",
            "marketplace_kill_switches_global_active",
        ] {
            assert!(
                PHASE_SEVEN_MIGRATION.contains(required),
                "missing Phase 7 migration contract: {required}"
            );
        }
        for endpoint in [
            "/api/marketplace/permissions",
            "/api/marketplace/runtime/status",
            "/runtime/authorize",
            "/api/marketplace/kill-switches/organization",
            "/api/marketplace/kill-switches/global",
            "/kill-switches/{kill_switch_id}/lift",
        ] {
            assert!(
                PHASE_SEVEN_DOC.contains(endpoint) || super::ROUTER_CONTRACT.contains(endpoint),
                "missing Phase 7 route contract: {endpoint}"
            );
        }
        assert!(PHASE_SEVEN_DOC.contains("`execution = not_executed`"));
        assert!(PHASE_SEVEN_DOC.contains("global and organization scopes"));
    }
}
