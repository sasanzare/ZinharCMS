use axum::extract::{Extension, Path, State};
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
use crate::services::{jwt, rbac};
use crate::state::AppState;

const INVITATION_TTL_DAYS: i64 = 7;
const PLAN_MEMBER_LIMIT: i64 = 3;
const PLAN_CONTENT_LIMIT: i64 = 50;
const PLAN_MEDIA_LIMIT_MB: i64 = 1024;

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
    pub members_limit: i64,
    pub content_limit: i64,
    pub media_limit_mb: i64,
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

    tx.commit().await?;

    let membership = load_organization_membership(&state, claims.sub, organization.id).await?;
    Ok(Json(OrganizationDetailResponse {
        organization,
        membership,
        plan_limits: default_plan_limits(),
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
    Ok(Json(OrganizationDetailResponse {
        organization,
        membership,
        plan_limits: default_plan_limits(),
    }))
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

    let membership =
        load_organization_membership(&state, tenant.user_id, tenant.organization_id).await?;
    Ok(Json(OrganizationDetailResponse {
        organization,
        membership,
        plan_limits: default_plan_limits(),
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
    .bind(invitation.role)
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

    Ok(Json(
        load_organization_member(&state, tenant.organization_id, payload.user_id).await?,
    ))
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

fn normalize_email(email: &str) -> Result<String, AppError> {
    let email = email.trim().to_ascii_lowercase();
    if !email.contains('@') || email.starts_with('@') || email.ends_with('@') {
        return Err(AppError::Validation("email is invalid".to_owned()));
    }
    Ok(email)
}

fn default_plan_limits() -> PlanLimitResponse {
    PlanLimitResponse {
        plan: "Free".to_owned(),
        members_limit: PLAN_MEMBER_LIMIT,
        content_limit: PLAN_CONTENT_LIMIT,
        media_limit_mb: PLAN_MEDIA_LIMIT_MB,
    }
}
