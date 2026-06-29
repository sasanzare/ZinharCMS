use axum::extract::{Extension, Path, Query, State};
use axum::routing::{delete, get, patch, post, put};
use axum::{Json, Router};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::routes::auth::OrganizationMembershipResponse;
use crate::services::{audit, email, jwt, quota, rbac, rls};
use crate::state::AppState;

const INVITATION_TTL_DAYS: i64 = 7;

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/api/organizations", get(list_organizations))
        .route("/api/organizations", post(create_organization))
        .route(
            "/api/organization-invitations/accept",
            post(accept_invitation),
        )
}

pub fn tenant_router() -> Router<AppState> {
    Router::new()
        .route("/api/organizations/current", get(get_current_organization))
        .route(
            "/api/organizations/current",
            put(update_current_organization),
        )
        .route(
            "/api/organizations/current/members",
            get(list_organization_members),
        )
        .route(
            "/api/organizations/current/members/{user_id}",
            patch(update_organization_member),
        )
        .route(
            "/api/organizations/current/members/{user_id}",
            delete(remove_organization_member),
        )
        .route(
            "/api/organizations/current/invitations",
            get(list_organization_invitations),
        )
        .route(
            "/api/organizations/current/invitations",
            post(create_organization_invitation),
        )
        .route(
            "/api/organizations/current/invitations/{invitation_id}",
            delete(revoke_organization_invitation),
        )
        .route(
            "/api/organizations/current/workspace",
            get(get_workspace_access),
        )
        .route(
            "/api/organizations/current/domains",
            get(list_organization_domains).post(create_organization_domain),
        )
        .route(
            "/api/organizations/current/domains/{domain_id}",
            delete(delete_organization_domain),
        )
        .route(
            "/api/organizations/current/rate-limit",
            get(get_rate_limit).put(update_rate_limit),
        )
        .route(
            "/api/organizations/current/audit-logs",
            get(list_audit_logs),
        )
        .route(
            "/api/organizations/current/email-deliveries",
            get(list_email_deliveries),
        )
        .route(
            "/api/organizations/current/alerts",
            get(list_saas_alert_rules),
        )
        .route("/api/organizations/current/leave", post(leave_organization))
        .route(
            "/api/organizations/current/transfer-ownership",
            post(transfer_organization_ownership),
        )
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateOrganizationRequest {
    pub name: String,
    pub slug: String,
    pub settings: Option<Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InviteMemberRequest {
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransferOwnershipRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AcceptInvitationRequest {
    pub token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct OrganizationDomainRequest {
    pub domain: String,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRateLimitRequest {
    pub requests_per_minute: i32,
    pub user_requests_per_minute: i32,
    pub burst: i32,
}

#[derive(Debug, Deserialize)]
pub struct LimitQuery {
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub status: String,
    pub owner_id: Option<Uuid>,
    pub settings: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanLimitResponse {
    pub plan: String,
    pub plan_slug: String,
    pub members_limit: i64,
    pub content_limit: i64,
    pub media_limit_mb: i64,
    pub api_requests_limit: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationDetailResponse {
    pub organization: OrganizationResponse,
    pub membership: OrganizationMembershipResponse,
    pub plan_limits: PlanLimitResponse,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct OrganizationMemberResponse {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub status: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct OrganizationInvitationResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub status: String,
    pub invited_by: Option<Uuid>,
    pub invited_by_name: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrganizationWorkspaceResponse {
    pub slug: String,
    pub workspace_url: String,
    pub domains: Vec<OrganizationDomainResponse>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct OrganizationDomainResponse {
    pub id: Uuid,
    pub domain: String,
    pub status: String,
    pub is_primary: bool,
    pub verification_token: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct RateLimitResponse {
    pub requests_per_minute: i32,
    pub user_requests_per_minute: i32,
    pub burst: i32,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct AuditLogResponse {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub actor_email: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct EmailDeliveryResponse {
    pub id: Uuid,
    pub recipient_email: String,
    pub template: String,
    pub subject: String,
    pub provider: String,
    pub status: String,
    pub provider_message_id: Option<String>,
    pub error: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct SaasAlertRuleResponse {
    pub id: Uuid,
    pub alert_key: String,
    pub severity: String,
    pub is_enabled: bool,
    pub config: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedInvitationResponse {
    #[serde(flatten)]
    pub invitation: OrganizationInvitationResponse,
    pub token: String,
    pub accept_path: String,
}

#[derive(Debug, FromRow)]
struct MemberRoleRow {
    role: String,
}

#[derive(Debug, FromRow)]
struct AcceptedInvitationRow {
    id: Uuid,
    organization_id: Uuid,
    role: String,
}

#[utoipa::path(
    get,
    path = "/api/organizations",
    tag = "organizations",
    responses((status = 200, description = "Active organizations for current user", body = [OrganizationMembershipResponse]))
)]
pub async fn list_organizations(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<OrganizationMembershipResponse>>, AppError> {
    Ok(Json(
        load_organization_memberships(&state, claims.sub).await?,
    ))
}

#[utoipa::path(
    post,
    path = "/api/organizations",
    tag = "organizations",
    request_body = CreateOrganizationRequest,
    responses((status = 200, description = "Created organization", body = OrganizationDetailResponse))
)]
pub async fn create_organization(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateOrganizationRequest>,
) -> Result<Json<OrganizationDetailResponse>, AppError> {
    let name = normalize_name(&payload.name)?;
    let slug = normalize_slug(&payload.slug)?;

    let mut tx = state.db.begin().await?;
    let organization = sqlx::query_as::<_, OrganizationResponse>(
        r#"
        INSERT INTO organizations (name, slug, owner_id)
        VALUES ($1, $2, $3)
        RETURNING id,
                  name,
                  slug,
                  status::text as status,
                  owner_id,
                  settings,
                  created_at,
                  updated_at
        "#,
    )
    .bind(name)
    .bind(slug)
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role, status, joined_at)
        VALUES ($1, $2, 'owner'::organization_member_role, 'active'::organization_member_status, now())
        "#,
    )
    .bind(organization.id)
    .bind(claims.sub)
    .execute(&mut *tx)
    .await?;
    quota::ensure_default_subscription_in_transaction(&mut tx, organization.id, claims.sub).await?;

    tx.commit().await?;
    audit::record_for_organization(
        &state.db,
        organization.id,
        Some(claims.sub),
        "organization.create",
        "organization",
        Some(organization.id),
        serde_json::json!({ "slug": &organization.slug, "name": &organization.name }),
    )
    .await?;

    let tenant = organization_tenant_context(&organization, claims.sub, rbac::ORG_OWNER);
    let plan_limits = load_plan_limit_response(&state, &tenant).await?;
    let membership = load_organization_membership(&state, claims.sub, organization.id).await?;
    Ok(Json(OrganizationDetailResponse {
        organization,
        membership,
        plan_limits,
    }))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current",
    tag = "organizations",
    responses((status = 200, description = "Current organization detail", body = OrganizationDetailResponse))
)]
pub async fn get_current_organization(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<OrganizationDetailResponse>, AppError> {
    let organization = load_organization(&state, tenant.organization_id).await?;
    let membership =
        load_organization_membership(&state, tenant.user_id, tenant.organization_id).await?;
    let plan_limits = load_plan_limit_response(&state, &tenant).await?;
    Ok(Json(OrganizationDetailResponse {
        organization,
        membership,
        plan_limits,
    }))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/workspace",
    tag = "organizations",
    responses((status = 200, description = "Current organization workspace access", body = OrganizationWorkspaceResponse))
)]
pub async fn get_workspace_access(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<OrganizationWorkspaceResponse>, AppError> {
    Ok(Json(OrganizationWorkspaceResponse {
        slug: tenant.organization_slug.clone(),
        workspace_url: workspace_url(&state.config.app_base_url, &tenant.organization_slug),
        domains: load_domains(&state, tenant.organization_id).await?,
    }))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/domains",
    tag = "organizations",
    responses((status = 200, description = "Current organization domains", body = [OrganizationDomainResponse]))
)]
pub async fn list_organization_domains(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<OrganizationDomainResponse>>, AppError> {
    require_org_admin(&tenant.role)?;
    Ok(Json(load_domains(&state, tenant.organization_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/organizations/current/domains",
    tag = "organizations",
    request_body = OrganizationDomainRequest,
    responses((status = 200, description = "Created organization domain", body = OrganizationDomainResponse))
)]
pub async fn create_organization_domain(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<OrganizationDomainRequest>,
) -> Result<Json<OrganizationDomainResponse>, AppError> {
    require_org_admin(&tenant.role)?;
    let domain = normalize_domain(&payload.domain)?;
    let is_primary = payload.is_primary.unwrap_or(false);
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    if is_primary {
        sqlx::query("UPDATE organization_domains SET is_primary = false, updated_at = now() WHERE organization_id = $1")
            .bind(tenant.organization_id)
            .execute(&mut *tx)
            .await?;
    }

    let row = sqlx::query_as::<_, OrganizationDomainResponse>(
        r#"
        INSERT INTO organization_domains (organization_id, domain, is_primary, created_by)
        VALUES ($1, $2, $3, $4)
        RETURNING id,
                  domain,
                  status,
                  is_primary,
                  verification_token,
                  verified_at,
                  created_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(domain)
    .bind(is_primary)
    .bind(tenant.user_id)
    .fetch_one(&mut *tx)
    .await?;

    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "organization.domain.create",
        "organization_domain",
        Some(row.id),
        serde_json::json!({ "domain": &row.domain, "is_primary": row.is_primary }),
    )
    .await?;
    tx.commit().await?;

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/api/organizations/current/domains/{domain_id}",
    tag = "organizations",
    params(("domain_id" = Uuid, Path, description = "Domain id")),
    responses((status = 200, description = "Deleted organization domain", body = OrganizationDomainResponse))
)]
pub async fn delete_organization_domain(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(domain_id): Path<Uuid>,
) -> Result<Json<OrganizationDomainResponse>, AppError> {
    require_org_admin(&tenant.role)?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let row = sqlx::query_as::<_, OrganizationDomainResponse>(
        r#"
        DELETE FROM organization_domains
        WHERE id = $1 AND organization_id = $2
        RETURNING id,
                  domain,
                  status,
                  is_primary,
                  verification_token,
                  verified_at,
                  created_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(domain_id)
    .bind(tenant.organization_id)
    .fetch_one(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "organization.domain.delete",
        "organization_domain",
        Some(row.id),
        serde_json::json!({ "domain": &row.domain }),
    )
    .await?;
    tx.commit().await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/rate-limit",
    tag = "organizations",
    responses((status = 200, description = "Current organization rate limit", body = RateLimitResponse))
)]
pub async fn get_rate_limit(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<RateLimitResponse>, AppError> {
    require_org_admin(&tenant.role)?;
    Ok(Json(load_or_create_rate_limit(&state, &tenant).await?))
}

#[utoipa::path(
    put,
    path = "/api/organizations/current/rate-limit",
    tag = "organizations",
    request_body = UpdateRateLimitRequest,
    responses((status = 200, description = "Updated current organization rate limit", body = RateLimitResponse))
)]
pub async fn update_rate_limit(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<UpdateRateLimitRequest>,
) -> Result<Json<RateLimitResponse>, AppError> {
    require_org_admin(&tenant.role)?;
    validate_rate_limit(&payload)?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let row = sqlx::query_as::<_, RateLimitResponse>(
        r#"
        INSERT INTO organization_rate_limits (
          organization_id,
          requests_per_minute,
          user_requests_per_minute,
          burst,
          updated_by
        )
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (organization_id) DO UPDATE
        SET requests_per_minute = EXCLUDED.requests_per_minute,
            user_requests_per_minute = EXCLUDED.user_requests_per_minute,
            burst = EXCLUDED.burst,
            updated_by = EXCLUDED.updated_by,
            updated_at = now()
        RETURNING requests_per_minute,
                  user_requests_per_minute,
                  burst,
                  updated_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(payload.requests_per_minute)
    .bind(payload.user_requests_per_minute)
    .bind(payload.burst)
    .bind(tenant.user_id)
    .fetch_one(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "organization.rate_limit.update",
        "organization_rate_limit",
        None,
        serde_json::json!({
            "requests_per_minute": row.requests_per_minute,
            "user_requests_per_minute": row.user_requests_per_minute,
            "burst": row.burst,
        }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/audit-logs",
    tag = "organizations",
    responses((status = 200, description = "Current organization audit log", body = [AuditLogResponse]))
)]
pub async fn list_audit_logs(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<Vec<AuditLogResponse>>, AppError> {
    require_org_admin(&tenant.role)?;
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, AuditLogResponse>(
        r#"
        SELECT audit.id,
               audit.actor_id,
               users.email::text as actor_email,
               audit.action,
               audit.entity_type,
               audit.entity_id,
               audit.metadata,
               audit.created_at
        FROM audit_logs audit
        LEFT JOIN users ON users.id = audit.actor_id
        WHERE audit.organization_id = $1
        ORDER BY audit.created_at DESC
        LIMIT $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(limit)
    .fetch_all(db.as_mut())
    .await?;
    Ok(Json(rows))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/email-deliveries",
    tag = "organizations",
    responses((status = 200, description = "Current organization email deliveries", body = [EmailDeliveryResponse]))
)]
pub async fn list_email_deliveries(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<LimitQuery>,
) -> Result<Json<Vec<EmailDeliveryResponse>>, AppError> {
    require_org_admin(&tenant.role)?;
    let limit = query.limit.unwrap_or(25).clamp(1, 100);
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, EmailDeliveryResponse>(
        r#"
        SELECT id,
               recipient_email,
               template,
               subject,
               provider,
               status,
               provider_message_id,
               error,
               sent_at,
               created_at,
               updated_at
        FROM email_deliveries
        WHERE organization_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(limit)
    .fetch_all(db.as_mut())
    .await?;
    Ok(Json(rows))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/alerts",
    tag = "organizations",
    responses((status = 200, description = "Current organization SaaS alert rules", body = [SaasAlertRuleResponse]))
)]
pub async fn list_saas_alert_rules(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<SaasAlertRuleResponse>>, AppError> {
    require_org_admin(&tenant.role)?;
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, SaasAlertRuleResponse>(
        r#"
        SELECT id,
               alert_key,
               severity,
               is_enabled,
               config,
               created_at,
               updated_at
        FROM saas_alert_rules
        WHERE organization_id = $1
        ORDER BY CASE severity WHEN 'critical' THEN 1 WHEN 'warning' THEN 2 ELSE 3 END,
                 alert_key ASC
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_all(db.as_mut())
    .await?;
    Ok(Json(rows))
}
#[utoipa::path(
    put,
    path = "/api/organizations/current",
    tag = "organizations",
    request_body = UpdateOrganizationRequest,
    responses((status = 200, description = "Updated current organization", body = OrganizationDetailResponse))
)]
pub async fn update_current_organization(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<UpdateOrganizationRequest>,
) -> Result<Json<OrganizationDetailResponse>, AppError> {
    require_org_admin(&tenant.role)?;

    let name = normalize_name(&payload.name)?;
    let slug = normalize_slug(&payload.slug)?;
    let settings = payload.settings.unwrap_or_else(|| serde_json::json!({}));
    if !settings.is_object() {
        return Err(AppError::Validation(
            "organization settings must be a JSON object".to_owned(),
        ));
    }

    let organization = sqlx::query_as::<_, OrganizationResponse>(
        r#"
        UPDATE organizations
        SET name = $2,
            slug = $3,
            settings = $4,
            updated_at = now()
        WHERE id = $1
          AND status = 'active'::organization_status
        RETURNING id,
                  name,
                  slug,
                  status::text as status,
                  owner_id,
                  settings,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(name)
    .bind(slug)
    .bind(settings)
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "organization.settings.update",
        "organization",
        Some(organization.id),
        serde_json::json!({ "slug": &organization.slug, "name": &organization.name }),
    )
    .await?;

    let membership =
        load_organization_membership(&state, tenant.user_id, tenant.organization_id).await?;
    let plan_limits = load_plan_limit_response(&state, &tenant).await?;
    Ok(Json(OrganizationDetailResponse {
        organization,
        membership,
        plan_limits,
    }))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/members",
    tag = "organizations",
    responses((status = 200, description = "Current organization members", body = [OrganizationMemberResponse]))
)]
pub async fn list_organization_members(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<OrganizationMemberResponse>>, AppError> {
    require_org_admin(&tenant.role)?;

    let members = sqlx::query_as::<_, OrganizationMemberResponse>(
        r#"
        SELECT u.id as user_id,
               u.email::text as email,
               u.name,
               u.avatar_url,
               om.role::text as role,
               om.status::text as status,
               om.joined_at,
               om.created_at,
               om.updated_at
        FROM organization_members om
        JOIN users u ON u.id = om.user_id
        WHERE om.organization_id = $1
        ORDER BY CASE om.role
            WHEN 'owner' THEN 1
            WHEN 'admin' THEN 2
            WHEN 'editor' THEN 3
            WHEN 'author' THEN 4
            WHEN 'viewer' THEN 5
            WHEN 'billing_manager' THEN 6
            ELSE 99
        END, lower(u.email::text)
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(members))
}

#[utoipa::path(
    patch,
    path = "/api/organizations/current/members/{user_id}",
    tag = "organizations",
    params(("user_id" = Uuid, Path, description = "Member user id")),
    request_body = UpdateMemberRoleRequest,
    responses((status = 200, description = "Updated organization member", body = OrganizationMemberResponse))
)]
pub async fn update_organization_member(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateMemberRoleRequest>,
) -> Result<Json<OrganizationMemberResponse>, AppError> {
    let new_role = validate_member_role(&payload.role)?;
    let current = load_member_role(&state, tenant.organization_id, user_id).await?;

    if current.role == rbac::ORG_OWNER || new_role == rbac::ORG_OWNER {
        require_org_owner(&tenant.role)?;
    } else {
        require_org_admin(&tenant.role)?;
    }

    if current.role == rbac::ORG_OWNER && new_role != rbac::ORG_OWNER {
        ensure_not_last_owner(&state, tenant.organization_id).await?;
    }

    let member = sqlx::query_as::<_, OrganizationMemberResponse>(
        r#"
        UPDATE organization_members om
        SET role = $3::organization_member_role,
            updated_at = now()
        FROM users u
        WHERE om.organization_id = $1
          AND om.user_id = $2
          AND u.id = om.user_id
        RETURNING u.id as user_id,
                  u.email::text as email,
                  u.name,
                  u.avatar_url,
                  om.role::text as role,
                  om.status::text as status,
                  om.joined_at,
                  om.created_at,
                  om.updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(user_id)
    .bind(new_role)
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "organization.member.role_update",
        "organization_member",
        Some(member.user_id),
        serde_json::json!({ "email": &member.email, "old_role": &current.role, "new_role": &member.role }),
    )
    .await?;

    Ok(Json(member))
}

#[utoipa::path(
    delete,
    path = "/api/organizations/current/members/{user_id}",
    tag = "organizations",
    params(("user_id" = Uuid, Path, description = "Member user id")),
    responses((status = 200, description = "Removed organization member", body = OrganizationMemberResponse))
)]
pub async fn remove_organization_member(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<OrganizationMemberResponse>, AppError> {
    let current = load_member_role(&state, tenant.organization_id, user_id).await?;
    if current.role == rbac::ORG_OWNER {
        require_org_owner(&tenant.role)?;
        ensure_not_last_owner(&state, tenant.organization_id).await?;
    } else {
        require_org_admin(&tenant.role)?;
    }

    let member = sqlx::query_as::<_, OrganizationMemberResponse>(
        r#"
        DELETE FROM organization_members om
        USING users u
        WHERE om.organization_id = $1
          AND om.user_id = $2
          AND u.id = om.user_id
        RETURNING u.id as user_id,
                  u.email::text as email,
                  u.name,
                  u.avatar_url,
                  om.role::text as role,
                  om.status::text as status,
                  om.joined_at,
                  om.created_at,
                  om.updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "organization.member.remove",
        "organization_member",
        Some(member.user_id),
        serde_json::json!({ "email": &member.email, "role": &member.role }),
    )
    .await?;

    Ok(Json(member))
}

#[utoipa::path(
    get,
    path = "/api/organizations/current/invitations",
    tag = "organizations",
    responses((status = 200, description = "Current organization invitations", body = [OrganizationInvitationResponse]))
)]
pub async fn list_organization_invitations(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<OrganizationInvitationResponse>>, AppError> {
    require_org_admin(&tenant.role)?;
    expire_pending_invitations(&state, tenant.organization_id).await?;

    let invitations = load_invitations(&state, tenant.organization_id).await?;
    Ok(Json(invitations))
}

#[utoipa::path(
    post,
    path = "/api/organizations/current/invitations",
    tag = "organizations",
    request_body = InviteMemberRequest,
    responses((status = 200, description = "Created organization invitation", body = CreatedInvitationResponse))
)]
pub async fn create_organization_invitation(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<InviteMemberRequest>,
) -> Result<Json<CreatedInvitationResponse>, AppError> {
    let role = validate_member_role(&payload.role)?;
    if role == rbac::ORG_OWNER {
        require_org_owner(&tenant.role)?;
    } else {
        require_org_admin(&tenant.role)?;
    }

    quota::ensure_member_capacity(&state.db, &tenant, true).await?;

    let email = normalize_email(&payload.email)?;
    let token = jwt::generate_refresh_token();
    let token_hash = jwt::hash_refresh_token(&token);
    let expires_at = Utc::now() + Duration::days(INVITATION_TTL_DAYS);

    let invitation = sqlx::query_as::<_, OrganizationInvitationResponse>(
        r#"
        INSERT INTO organization_invitations (
          organization_id,
          email,
          role,
          token_hash,
          invited_by,
          status,
          expires_at
        )
        VALUES (
          $1,
          $2,
          $3::organization_member_role,
          $4,
          $5,
          'pending'::organization_invitation_status,
          $6
        )
        ON CONFLICT (organization_id, email) WHERE status = 'pending'::organization_invitation_status
        DO UPDATE
        SET role = EXCLUDED.role,
            token_hash = EXCLUDED.token_hash,
            invited_by = EXCLUDED.invited_by,
            expires_at = EXCLUDED.expires_at,
            updated_at = now()
        RETURNING id,
                  email::text as email,
                  role::text as role,
                  status::text as status,
                  invited_by,
                  (
                    SELECT name
                    FROM users
                    WHERE users.id = organization_invitations.invited_by
                  ) as invited_by_name,
                  expires_at,
                  accepted_at,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(email)
    .bind(role)
    .bind(token_hash)
    .bind(tenant.user_id)
    .bind(expires_at)
    .fetch_one(&state.db)
    .await?;

    let accept_path = format!("/organization?invite={token}");
    email::send_invitation_email(
        &state.db,
        &state.config,
        &tenant,
        &invitation.email,
        &accept_path,
    )
    .await?;
    audit::record(
        &state.db,
        &tenant,
        "organization.invitation.create",
        "organization_invitation",
        Some(invitation.id),
        serde_json::json!({ "email": &invitation.email, "role": &invitation.role }),
    )
    .await?;
    Ok(Json(CreatedInvitationResponse {
        invitation,
        token,
        accept_path,
    }))
}

#[utoipa::path(
    delete,
    path = "/api/organizations/current/invitations/{invitation_id}",
    tag = "organizations",
    params(("invitation_id" = Uuid, Path, description = "Invitation id")),
    responses((status = 200, description = "Revoked organization invitation", body = OrganizationInvitationResponse))
)]
pub async fn revoke_organization_invitation(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(invitation_id): Path<Uuid>,
) -> Result<Json<OrganizationInvitationResponse>, AppError> {
    require_org_admin(&tenant.role)?;

    let invitation = sqlx::query_as::<_, OrganizationInvitationResponse>(
        r#"
        UPDATE organization_invitations invitation
        SET status = 'revoked'::organization_invitation_status,
            updated_at = now()
        WHERE invitation.id = $1
          AND invitation.organization_id = $2
          AND invitation.status = 'pending'::organization_invitation_status
        RETURNING invitation.id,
                  invitation.email::text as email,
                  invitation.role::text as role,
                  invitation.status::text as status,
                  invitation.invited_by,
                  (
                    SELECT name
                    FROM users
                    WHERE users.id = invitation.invited_by
                  ) as invited_by_name,
                  invitation.expires_at,
                  invitation.accepted_at,
                  invitation.created_at,
                  invitation.updated_at
        "#,
    )
    .bind(invitation_id)
    .bind(tenant.organization_id)
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "organization.invitation.revoke",
        "organization_invitation",
        Some(invitation.id),
        serde_json::json!({ "email": &invitation.email, "role": &invitation.role }),
    )
    .await?;

    Ok(Json(invitation))
}

#[utoipa::path(
    post,
    path = "/api/organization-invitations/accept",
    tag = "organizations",
    request_body = AcceptInvitationRequest,
    responses((status = 200, description = "Accepted organization invitation", body = OrganizationMembershipResponse))
)]
pub async fn accept_invitation(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<AcceptInvitationRequest>,
) -> Result<Json<OrganizationMembershipResponse>, AppError> {
    let token = payload.token.trim();
    if token.is_empty() {
        return Err(AppError::Validation(
            "invitation token is required".to_owned(),
        ));
    }

    let token_hash = jwt::hash_refresh_token(token);
    let mut tx = state.db.begin().await?;

    let invitation = sqlx::query_as::<_, AcceptedInvitationRow>(
        r#"
        SELECT invitation.id,
               invitation.organization_id,
               invitation.role::text as role
        FROM organization_invitations invitation
        JOIN users u ON lower(u.email::text) = lower(invitation.email::text)
        WHERE invitation.token_hash = $1
          AND invitation.status = 'pending'::organization_invitation_status
          AND invitation.expires_at > now()
          AND u.id = $2
        "#,
    )
    .bind(token_hash)
    .bind(claims.sub)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("valid invitation not found".to_owned()))?;
    quota::ensure_member_capacity_for_org(&state.db, invitation.organization_id, claims.sub, false)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role, status, joined_at)
        VALUES ($1, $2, $3::organization_member_role, 'active'::organization_member_status, now())
        ON CONFLICT (organization_id, user_id) DO UPDATE
        SET role = EXCLUDED.role,
            status = 'active'::organization_member_status,
            joined_at = COALESCE(organization_members.joined_at, now()),
            updated_at = now()
        "#,
    )
    .bind(invitation.organization_id)
    .bind(claims.sub)
    .bind(&invitation.role)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE organization_invitations
        SET status = 'accepted'::organization_invitation_status,
            accepted_at = now(),
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(invitation.id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    audit::record_for_organization(
        &state.db,
        invitation.organization_id,
        Some(claims.sub),
        "organization.invitation.accept",
        "organization_invitation",
        Some(invitation.id),
        serde_json::json!({ "role": &invitation.role }),
    )
    .await?;

    Ok(Json(
        load_organization_membership(&state, claims.sub, invitation.organization_id).await?,
    ))
}

#[utoipa::path(
    post,
    path = "/api/organizations/current/leave",
    tag = "organizations",
    responses((status = 200, description = "Left current organization", body = OrganizationMemberResponse))
)]
pub async fn leave_organization(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<OrganizationMemberResponse>, AppError> {
    let current = load_member_role(&state, tenant.organization_id, tenant.user_id).await?;
    if current.role == rbac::ORG_OWNER {
        ensure_not_last_owner(&state, tenant.organization_id).await?;
    }

    let member = sqlx::query_as::<_, OrganizationMemberResponse>(
        r#"
        DELETE FROM organization_members om
        USING users u
        WHERE om.organization_id = $1
          AND om.user_id = $2
          AND u.id = om.user_id
        RETURNING u.id as user_id,
                  u.email::text as email,
                  u.name,
                  u.avatar_url,
                  om.role::text as role,
                  om.status::text as status,
                  om.joined_at,
                  om.created_at,
                  om.updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(tenant.user_id)
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "organization.member.leave",
        "organization_member",
        Some(member.user_id),
        serde_json::json!({ "email": &member.email, "role": &member.role }),
    )
    .await?;

    Ok(Json(member))
}

#[utoipa::path(
    post,
    path = "/api/organizations/current/transfer-ownership",
    tag = "organizations",
    request_body = TransferOwnershipRequest,
    responses((status = 200, description = "Transferred organization ownership", body = OrganizationMemberResponse))
)]
pub async fn transfer_organization_ownership(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<TransferOwnershipRequest>,
) -> Result<Json<OrganizationMemberResponse>, AppError> {
    require_org_owner(&tenant.role)?;
    if payload.user_id == tenant.user_id {
        return Err(AppError::Validation(
            "ownership is already assigned to this member".to_owned(),
        ));
    }
    load_member_role(&state, tenant.organization_id, payload.user_id).await?;

    let mut tx = state.db.begin().await?;
    sqlx::query(
        r#"
        UPDATE organization_members
        SET role = 'admin'::organization_member_role,
            updated_at = now()
        WHERE organization_id = $1
          AND user_id = $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(tenant.user_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE organization_members
        SET role = 'owner'::organization_member_role,
            status = 'active'::organization_member_status,
            joined_at = COALESCE(joined_at, now()),
            updated_at = now()
        WHERE organization_id = $1
          AND user_id = $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(payload.user_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE organizations
        SET owner_id = $2,
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(tenant.organization_id)
    .bind(payload.user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    audit::record(
        &state.db,
        &tenant,
        "organization.ownership.transfer",
        "organization_member",
        Some(payload.user_id),
        serde_json::json!({ "previous_owner_id": tenant.user_id, "new_owner_id": payload.user_id }),
    )
    .await?;

    Ok(Json(
        load_organization_member(&state, tenant.organization_id, payload.user_id).await?,
    ))
}

async fn load_domains(
    state: &AppState,
    organization_id: Uuid,
) -> Result<Vec<OrganizationDomainResponse>, AppError> {
    let mut db = rls::organization_connection(&state.db, organization_id, None).await?;
    sqlx::query_as::<_, OrganizationDomainResponse>(
        r#"
        SELECT id,
               domain,
               status,
               is_primary,
               verification_token,
               verified_at,
               created_by,
               created_at,
               updated_at
        FROM organization_domains
        WHERE organization_id = $1
        ORDER BY is_primary DESC, created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_or_create_rate_limit(
    state: &AppState,
    tenant: &TenantContext,
) -> Result<RateLimitResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, tenant).await?;
    sqlx::query_as::<_, RateLimitResponse>(
        r#"
        INSERT INTO organization_rate_limits (organization_id)
        VALUES ($1)
        ON CONFLICT (organization_id) DO UPDATE
        SET updated_at = organization_rate_limits.updated_at
        RETURNING requests_per_minute,
                  user_requests_per_minute,
                  burst,
                  updated_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

fn validate_rate_limit(payload: &UpdateRateLimitRequest) -> Result<(), AppError> {
    if payload.requests_per_minute <= 0 || payload.user_requests_per_minute <= 0 {
        return Err(AppError::Validation(
            "rate limits must be positive integers".to_owned(),
        ));
    }
    if payload.burst < 0 {
        return Err(AppError::Validation("burst cannot be negative".to_owned()));
    }
    Ok(())
}

fn workspace_url(base_url: &str, slug: &str) -> String {
    format!("{}/workspace/{}", base_url.trim_end_matches('/'), slug)
}
async fn load_organization(
    state: &AppState,
    organization_id: Uuid,
) -> Result<OrganizationResponse, AppError> {
    sqlx::query_as::<_, OrganizationResponse>(
        r#"
        SELECT id,
               name,
               slug,
               status::text as status,
               owner_id,
               settings,
               created_at,
               updated_at
        FROM organizations
        WHERE id = $1
          AND status = 'active'::organization_status
        "#,
    )
    .bind(organization_id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

async fn load_organization_memberships(
    state: &AppState,
    user_id: Uuid,
) -> Result<Vec<OrganizationMembershipResponse>, AppError> {
    sqlx::query_as::<_, OrganizationMembershipResponse>(
        r#"
        SELECT o.id,
               o.name,
               o.slug,
               om.role::text as role,
               om.status::text as status
        FROM organizations o
        JOIN organization_members om ON om.organization_id = o.id
        WHERE om.user_id = $1
          AND om.status = 'active'::organization_member_status
          AND o.status = 'active'::organization_status
        ORDER BY CASE om.role
            WHEN 'owner' THEN 1
            WHEN 'admin' THEN 2
            WHEN 'editor' THEN 3
            WHEN 'author' THEN 4
            WHEN 'viewer' THEN 5
            WHEN 'billing_manager' THEN 6
            ELSE 99
        END, o.created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
    .map_err(AppError::from)
}

async fn load_organization_membership(
    state: &AppState,
    user_id: Uuid,
    organization_id: Uuid,
) -> Result<OrganizationMembershipResponse, AppError> {
    sqlx::query_as::<_, OrganizationMembershipResponse>(
        r#"
        SELECT o.id,
               o.name,
               o.slug,
               om.role::text as role,
               om.status::text as status
        FROM organizations o
        JOIN organization_members om ON om.organization_id = o.id
        WHERE o.id = $1
          AND om.user_id = $2
          AND om.status = 'active'::organization_member_status
          AND o.status = 'active'::organization_status
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

async fn load_organization_member(
    state: &AppState,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<OrganizationMemberResponse, AppError> {
    sqlx::query_as::<_, OrganizationMemberResponse>(
        r#"
        SELECT u.id as user_id,
               u.email::text as email,
               u.name,
               u.avatar_url,
               om.role::text as role,
               om.status::text as status,
               om.joined_at,
               om.created_at,
               om.updated_at
        FROM organization_members om
        JOIN users u ON u.id = om.user_id
        WHERE om.organization_id = $1
          AND om.user_id = $2
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

async fn load_member_role(
    state: &AppState,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<MemberRoleRow, AppError> {
    sqlx::query_as::<_, MemberRoleRow>(
        r#"
        SELECT role::text as role
        FROM organization_members
        WHERE organization_id = $1
          AND user_id = $2
          AND status = 'active'::organization_member_status
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("organization member not found".to_owned()))
}

async fn load_invitations(
    state: &AppState,
    organization_id: Uuid,
) -> Result<Vec<OrganizationInvitationResponse>, AppError> {
    sqlx::query_as::<_, OrganizationInvitationResponse>(
        r#"
        SELECT invitation.id,
               invitation.email::text as email,
               invitation.role::text as role,
               invitation.status::text as status,
               invitation.invited_by,
               inviter.name as invited_by_name,
               invitation.expires_at,
               invitation.accepted_at,
               invitation.created_at,
               invitation.updated_at
        FROM organization_invitations invitation
        LEFT JOIN users inviter ON inviter.id = invitation.invited_by
        WHERE invitation.organization_id = $1
        ORDER BY invitation.created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(&state.db)
    .await
    .map_err(AppError::from)
}

async fn expire_pending_invitations(
    state: &AppState,
    organization_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE organization_invitations
        SET status = 'expired'::organization_invitation_status,
            updated_at = now()
        WHERE organization_id = $1
          AND status = 'pending'::organization_invitation_status
          AND expires_at <= now()
        "#,
    )
    .bind(organization_id)
    .execute(&state.db)
    .await?;
    Ok(())
}

async fn ensure_not_last_owner(state: &AppState, organization_id: Uuid) -> Result<(), AppError> {
    let owner_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM organization_members
        WHERE organization_id = $1
          AND role = 'owner'::organization_member_role
          AND status = 'active'::organization_member_status
        "#,
    )
    .bind(organization_id)
    .fetch_one(&state.db)
    .await?;

    if owner_count <= 1 {
        return Err(AppError::Validation(
            "the last organization owner cannot be removed or downgraded".to_owned(),
        ));
    }

    Ok(())
}

fn require_org_owner(role: &str) -> Result<(), AppError> {
    if role == rbac::ORG_OWNER {
        Ok(())
    } else {
        Err(AppError::Forbidden(
            "only organization owners can perform this action".to_owned(),
        ))
    }
}

fn require_org_admin(role: &str) -> Result<(), AppError> {
    rbac::require_org_any(role, &[rbac::ORG_ADMIN])
}

fn validate_member_role(role: &str) -> Result<&'static str, AppError> {
    match role.trim() {
        rbac::ORG_OWNER => Ok(rbac::ORG_OWNER),
        rbac::ORG_ADMIN => Ok(rbac::ORG_ADMIN),
        rbac::ORG_EDITOR => Ok(rbac::ORG_EDITOR),
        rbac::ORG_AUTHOR => Ok(rbac::ORG_AUTHOR),
        rbac::ORG_VIEWER => Ok(rbac::ORG_VIEWER),
        rbac::ORG_BILLING_MANAGER => Ok(rbac::ORG_BILLING_MANAGER),
        _ => Err(AppError::Validation(
            "organization member role is invalid".to_owned(),
        )),
    }
}

fn normalize_name(name: &str) -> Result<String, AppError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::Validation(
            "organization name is required".to_owned(),
        ));
    }
    Ok(name.to_owned())
}

fn normalize_slug(slug: &str) -> Result<String, AppError> {
    let slug = slug.trim().to_ascii_lowercase();
    let valid = !slug.is_empty()
        && slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && !slug.contains("--");
    if !valid {
        return Err(AppError::Validation(
            "organization slug must use lowercase letters, numbers, and single hyphens".to_owned(),
        ));
    }
    Ok(slug)
}

fn normalize_domain(domain: &str) -> Result<String, AppError> {
    let domain = domain
        .trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_ascii_lowercase();
    let valid = !domain.is_empty()
        && domain.contains('.')
        && domain.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'.'
        })
        && !domain.starts_with('-')
        && !domain.ends_with('-')
        && !domain.contains("..")
        && domain
            .rsplit('.')
            .next()
            .is_some_and(|part| part.len() >= 2);
    if !valid {
        return Err(AppError::Validation("domain is invalid".to_owned()));
    }
    Ok(domain)
}
fn normalize_email(email: &str) -> Result<String, AppError> {
    let email = email.trim().to_ascii_lowercase();
    if !email.contains('@') || email.starts_with('@') || email.ends_with('@') {
        return Err(AppError::Validation("email is invalid".to_owned()));
    }
    Ok(email)
}

fn organization_tenant_context(
    organization: &OrganizationResponse,
    user_id: Uuid,
    role: &str,
) -> TenantContext {
    TenantContext {
        organization_id: organization.id,
        organization_slug: organization.slug.clone(),
        organization_name: organization.name.clone(),
        role: role.to_owned(),
        user_id,
    }
}

async fn load_plan_limit_response(
    state: &AppState,
    tenant: &TenantContext,
) -> Result<PlanLimitResponse, AppError> {
    Ok(plan_limit_response(
        quota::load_current_plan(&state.db, tenant).await?,
    ))
}

fn plan_limit_response(plan: quota::PlanLimits) -> PlanLimitResponse {
    PlanLimitResponse {
        plan: plan.name,
        plan_slug: plan.slug,
        members_limit: i64::from(plan.member_limit),
        content_limit: i64::from(plan.content_limit),
        media_limit_mb: i64::from(plan.media_limit_mb),
        api_requests_limit: i64::from(plan.api_requests_limit),
    }
}
