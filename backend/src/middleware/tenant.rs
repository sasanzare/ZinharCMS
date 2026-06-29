use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::{jwt, quota, rate_limit};
use crate::state::AppState;

pub const ORGANIZATION_HEADER: &str = "X-Organization-Id";

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct TenantContext {
    pub organization_id: Uuid,
    pub organization_slug: String,
    pub organization_name: String,
    pub role: String,
    pub user_id: Uuid,
}

#[derive(Debug, FromRow)]
struct TenantMembershipRow {
    organization_id: Uuid,
    organization_slug: String,
    organization_name: String,
    role: String,
}

pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .map(Ok)
        .unwrap_or_else(|| verify_claims(&state, &req))?;
    let organization_id = organization_id_from_request(&req)?;
    let tenant = load_tenant_context(&state, claims.sub, organization_id).await?;
    rate_limit::check_and_record_request(&state, &tenant).await?;
    if !is_quota_exempt_path(req.uri().path()) {
        quota::check_and_record_api_request(&state.db, &tenant).await?;
    }

    req.extensions_mut().insert(claims);
    req.extensions_mut().insert(tenant);

    Ok(next.run(req).await)
}

fn is_quota_exempt_path(path: &str) -> bool {
    path.starts_with("/api/billing")
}

fn verify_claims(state: &AppState, req: &Request) -> Result<Claims, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .or_else(|| preview_query_value(req, "access_token"))
        .or_else(|| preview_query_value(req, "token"))
        .ok_or_else(|| AppError::Unauthorized("missing bearer token".to_owned()))?;

    jwt::verify_access_token(token, &state.config)
        .map_err(|_| AppError::Unauthorized("invalid bearer token".to_owned()))
}

fn organization_id_from_request(req: &Request) -> Result<Uuid, AppError> {
    let value = req
        .headers()
        .get(ORGANIZATION_HEADER)
        .and_then(|header| header.to_str().ok())
        .or_else(|| preview_query_value(req, "organization_id"))
        .ok_or_else(|| AppError::BadRequest(format!("missing {ORGANIZATION_HEADER} header")))?;

    Uuid::parse_str(value)
        .map_err(|_| AppError::BadRequest("organization id is invalid".to_owned()))
}

fn preview_query_value<'a>(req: &'a Request, key: &str) -> Option<&'a str> {
    req.uri()
        .path()
        .starts_with("/api/preview/")
        .then(|| req.uri().query().and_then(|query| query_value(query, key)))?
}

fn query_value<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|pair| {
        let (candidate, value) = pair.split_once('=').unwrap_or((pair, ""));
        (candidate == key && !value.is_empty()).then_some(value)
    })
}

async fn load_tenant_context(
    state: &AppState,
    user_id: Uuid,
    organization_id: Uuid,
) -> Result<TenantContext, AppError> {
    let row = sqlx::query_as::<_, TenantMembershipRow>(
        r#"
        SELECT o.id as organization_id,
               o.slug as organization_slug,
               o.name as organization_name,
               om.role::text as role
        FROM organizations o
        JOIN organization_members om ON om.organization_id = o.id
        WHERE o.id = $1
          AND om.user_id = $2
          AND o.status = 'active'::organization_status
          AND om.status = 'active'::organization_member_status
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    let Some(row) = row else {
        return Err(AppError::Forbidden(
            "user is not an active member of this organization".to_owned(),
        ));
    };

    Ok(TenantContext {
        organization_id: row.organization_id,
        organization_slug: row.organization_slug,
        organization_name: row.organization_name,
        role: row.role,
        user_id,
    })
}
