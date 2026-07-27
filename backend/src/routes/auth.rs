use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Extension, State};
use axum::http::header::{COOKIE, ORIGIN, SET_COOKIE};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::{jwt, password, preview_tickets, rbac, security, sessions};
use crate::state::AppState;

const REFRESH_COOKIE_NAME: &str = "zinhar_refresh_token";

type AuthResult = (HeaderMap, Json<AuthResponse>);

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth", get(module_status))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh))
        .route("/api/auth/logout", post(logout))
}

pub fn protected_router() -> Router<AppState> {
    Router::new().route("/api/auth/me", get(me))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthModuleStatus {
    pub module: String,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub revoked: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: AuthUser,
    pub organizations: Vec<OrganizationMembershipResponse>,
    pub default_organization_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
pub struct OrganizationMembershipResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub role: String,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MeResponse {
    pub user: AuthUser,
    pub organizations: Vec<OrganizationMembershipResponse>,
    pub default_organization_id: Option<Uuid>,
}

#[derive(Debug, FromRow)]
struct LoginUser {
    id: Uuid,
    email: String,
    name: String,
    avatar_url: Option<String>,
    password_hash: String,
    role: String,
}

struct IssuedAuthResponse {
    body: AuthResponse,
    refresh_token: String,
}

#[utoipa::path(
    get,
    path = "/api/auth",
    tag = "auth",
    responses((status = 200, description = "Auth module status", body = AuthModuleStatus))
)]
pub async fn module_status() -> Json<AuthModuleStatus> {
    Json(AuthModuleStatus {
        module: "auth".to_owned(),
        endpoints: [
            "POST /api/auth/register",
            "POST /api/auth/login",
            "POST /api/auth/refresh",
            "POST /api/auth/logout",
            "GET /api/auth/me",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
    })
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses((status = 200, description = "Registered user and token pair", body = AuthResponse))
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<AuthResult, AppError> {
    validate_register(&payload)?;

    let email = payload.email.trim().to_ascii_lowercase();
    let password_hash = password::hash_password(&payload.password)?;
    let role = rbac::default_registration_role();

    let mut tx = state.db.begin().await?;
    let user = sqlx::query_as::<_, AuthUser>(
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, $2, $3)
        RETURNING id, email::text as email, name, avatar_url, $4::text as role
        "#,
    )
    .bind(&email)
    .bind(&password_hash)
    .bind(payload.name.trim())
    .bind(role)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_roles (user_id, role_id)
        SELECT $1, id FROM roles WHERE name = $2
        "#,
    )
    .bind(user.id)
    .bind(role)
    .execute(&mut *tx)
    .await?;

    attach_default_organization_membership(&mut tx, user.id, role).await?;
    tx.commit().await?;

    let issued = issue_auth_response(&state, user).await?;
    auth_response_with_cookie(&state, issued)
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses((status = 200, description = "Token pair", body = AuthResponse))
)]
pub async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<AuthResult, AppError> {
    let email = payload.email.trim().to_ascii_lowercase();
    let ip_address = security::client_ip(&headers, addr.ip(), &state.config.trusted_proxy_cidrs);
    security::require_login_allowed(
        &state.db,
        &ip_address,
        state.config.login_rate_limit_max_failures,
        state.config.login_rate_limit_window_seconds,
    )
    .await?;

    let user = sqlx::query_as::<_, LoginUser>(
        r#"
        SELECT u.id,
               u.email::text as email,
               u.name,
               u.avatar_url,
               u.password_hash,
               r.name as role
        FROM users u
        JOIN user_roles ur ON ur.user_id = u.id
        JOIN roles r ON r.id = ur.role_id
        WHERE u.email = $1 AND u.is_active = true
        ORDER BY CASE r.name
            WHEN 'super_admin' THEN 1
            WHEN 'admin' THEN 2
            WHEN 'editor' THEN 3
            WHEN 'author' THEN 4
            WHEN 'viewer' THEN 5
            ELSE 99
        END
        LIMIT 1
        "#,
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await?;

    let Some(user) = user else {
        security::record_login_attempt(&state.db, &email, &ip_address, false).await?;
        return Err(AppError::Unauthorized(
            "invalid email or password".to_owned(),
        ));
    };

    if !password::verify_password(&payload.password, &user.password_hash)? {
        security::record_login_attempt(&state.db, &email, &ip_address, false).await?;
        return Err(AppError::Unauthorized(
            "invalid email or password".to_owned(),
        ));
    }

    security::record_login_attempt(&state.db, &email, &ip_address, true).await?;
    let issued = issue_auth_response(
        &state,
        AuthUser {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
            role: user.role,
        },
    )
    .await?;
    auth_response_with_cookie(&state, issued)
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    responses((status = 200, description = "Rotated token pair", body = AuthResponse))
)]
pub async fn refresh(State(state): State<AppState>, headers: HeaderMap) -> Response {
    if let Err(error) = validate_cookie_request_origin(&headers, &state) {
        return error.into_response();
    }
    match refresh_session(&state, &headers).await {
        Ok(response) => response.into_response(),
        Err(error) => {
            let clear_cookie = matches!(&error, AppError::Unauthorized(_));
            let mut response = error.into_response();
            if clear_cookie && let Ok(cookie) = clear_refresh_cookie(&state) {
                response.headers_mut().insert(SET_COOKIE, cookie);
            }
            response
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    responses((status = 200, description = "Logout result", body = LogoutResponse))
)]
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<LogoutResponse>), AppError> {
    validate_cookie_request_origin(&headers, &state)?;
    let refresh_token = refresh_token_from_request(&headers);
    let revoked = if let Some(refresh_token) = refresh_token {
        sessions::revoke_refresh_family(&state.db, &refresh_token).await?
    } else {
        false
    };

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, clear_refresh_cookie(&state)?);
    Ok((headers, Json(LogoutResponse { revoked })))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "auth",
    responses((status = 200, description = "Current user", body = MeResponse))
)]
pub async fn me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MeResponse>, AppError> {
    let user = load_auth_user(&state, claims.sub).await?;
    let organizations = load_organization_memberships(&state, user.id).await?;
    let default_organization_id = default_organization_id(&organizations);
    Ok(Json(MeResponse {
        user,
        organizations,
        default_organization_id,
    }))
}

async fn attach_default_organization_membership(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    global_role: &str,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role, status, joined_at)
        SELECT id,
               $1,
               CASE $2
                 WHEN 'super_admin' THEN 'owner'::organization_member_role
                 WHEN 'admin' THEN 'admin'::organization_member_role
                 WHEN 'editor' THEN 'editor'::organization_member_role
                 WHEN 'viewer' THEN 'viewer'::organization_member_role
                 ELSE 'author'::organization_member_role
               END,
               'active'::organization_member_status,
               now()
        FROM organizations
        WHERE slug = 'default'
        ON CONFLICT (organization_id, user_id) DO UPDATE
        SET role = EXCLUDED.role,
            status = 'active'::organization_member_status,
            updated_at = now()
        "#,
    )
    .bind(user_id)
    .bind(global_role)
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE organizations
        SET owner_id = $1,
            updated_at = now()
        WHERE slug = 'default'
          AND owner_id IS NULL
          AND $2 IN ('super_admin', 'admin')
        "#,
    )
    .bind(user_id)
    .bind(global_role)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
async fn issue_auth_response(
    state: &AppState,
    user: AuthUser,
) -> Result<IssuedAuthResponse, AppError> {
    let identity = sessions::load_current_auth_identity(&state.db, user.id)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid credentials".to_owned()))?;
    let refresh_token =
        sessions::issue_refresh_family(&state.db, user.id, refresh_ttl_seconds(state)?).await?;
    build_auth_response(state, user, identity, refresh_token).await
}

async fn build_auth_response(
    state: &AppState,
    mut user: AuthUser,
    identity: sessions::CurrentAuthIdentity,
    refresh_token: sessions::IssuedRefreshToken,
) -> Result<IssuedAuthResponse, AppError> {
    user.role = identity.role;
    let access_token =
        jwt::sign_access_token(user.id, &user.role, identity.auth_version, &state.config)?;
    let organizations = load_organization_memberships(state, user.id).await?;
    let default_organization_id = default_organization_id(&organizations);

    Ok(IssuedAuthResponse {
        body: AuthResponse {
            access_token,
            token_type: "Bearer".to_owned(),
            expires_in: state.config.jwt_access_expiry,
            user,
            organizations,
            default_organization_id,
        },
        refresh_token: refresh_token.raw_token,
    })
}

async fn refresh_session(state: &AppState, headers: &HeaderMap) -> Result<AuthResult, AppError> {
    let refresh_token = refresh_token_from_request(headers)
        .ok_or_else(|| AppError::Unauthorized("invalid refresh token".to_owned()))?;
    let rotation =
        sessions::rotate_refresh_token(&state.db, &refresh_token, refresh_ttl_seconds(state)?)
            .await?;
    let sessions::RefreshRotation::Rotated { issued, identity } = rotation else {
        return Err(AppError::Unauthorized("invalid refresh token".to_owned()));
    };
    let user = load_auth_user(state, identity.user_id).await?;
    let issued = build_auth_response(state, user, identity, issued).await?;
    auth_response_with_cookie(state, issued)
}

fn refresh_ttl_seconds(state: &AppState) -> Result<i64, AppError> {
    i64::try_from(state.config.jwt_refresh_expiry)
        .map_err(|_| AppError::Internal("refresh token lifetime is too large".to_owned()))
}

fn auth_response_with_cookie(
    state: &AppState,
    issued: IssuedAuthResponse,
) -> Result<AuthResult, AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        refresh_cookie(
            &issued.refresh_token,
            state.config.jwt_refresh_expiry,
            state.config.cookie_secure,
        )?,
    );
    Ok((headers, Json(issued.body)))
}

async fn load_auth_user(state: &AppState, user_id: Uuid) -> Result<AuthUser, AppError> {
    sqlx::query_as::<_, AuthUser>(
        r#"
        SELECT u.id,
               u.email::text as email,
               u.name,
               u.avatar_url,
               r.name as role
        FROM users u
        JOIN user_roles ur ON ur.user_id = u.id
        JOIN roles r ON r.id = ur.role_id
        WHERE u.id = $1 AND u.is_active = true
        ORDER BY CASE r.name
            WHEN 'super_admin' THEN 1
            WHEN 'admin' THEN 2
            WHEN 'editor' THEN 3
            WHEN 'author' THEN 4
            WHEN 'viewer' THEN 5
            ELSE 99
        END
        LIMIT 1
        "#,
    )
    .bind(user_id)
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

fn default_organization_id(organizations: &[OrganizationMembershipResponse]) -> Option<Uuid> {
    organizations
        .iter()
        .find(|organization| organization.slug == "default")
        .or_else(|| organizations.first())
        .map(|organization| organization.id)
}

fn refresh_token_from_request(headers: &HeaderMap) -> Option<String> {
    cookie_value(headers, REFRESH_COOKIE_NAME)
}

fn validate_cookie_request_origin(headers: &HeaderMap, state: &AppState) -> Result<(), AppError> {
    let mut origins = headers.get_all(ORIGIN).iter();
    let Some(origin) = origins.next() else {
        return Ok(());
    };
    if origins.next().is_some() {
        return Err(AppError::Forbidden(
            "request origin is not allowed".to_owned(),
        ));
    }
    let origin = origin
        .to_str()
        .map_err(|_| AppError::Forbidden("request origin is not allowed".to_owned()))?;
    let allowed = preview_tickets::canonical_origin(&state.config.cors_origin)
        .ok_or_else(|| AppError::Internal("configured CORS origin is invalid".to_owned()))?;
    if preview_tickets::canonical_origin(origin).as_deref() != Some(allowed.as_str()) {
        return Err(AppError::Forbidden(
            "request origin is not allowed".to_owned(),
        ));
    }
    Ok(())
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let (cookie_name, cookie_value) = cookie.trim().split_once('=')?;
                (cookie_name == name).then(|| cookie_value.to_owned())
            })
        })
}

fn refresh_cookie(token: &str, max_age: u64, secure: bool) -> Result<HeaderValue, AppError> {
    let secure = if secure { "; Secure" } else { "" };
    HeaderValue::from_str(&format!(
        "{REFRESH_COOKIE_NAME}={token}; HttpOnly; SameSite=Lax; Path=/api/auth; Max-Age={max_age}{secure}"
    ))
    .map_err(|error| AppError::Internal(error.to_string()))
}

fn clear_refresh_cookie(state: &AppState) -> Result<HeaderValue, AppError> {
    let secure = if state.config.cookie_secure {
        "; Secure"
    } else {
        ""
    };
    HeaderValue::from_str(&format!(
        "{REFRESH_COOKIE_NAME}=; HttpOnly; SameSite=Lax; Path=/api/auth; Max-Age=0{secure}"
    ))
    .map_err(|error| AppError::Internal(error.to_string()))
}

fn validate_register(payload: &RegisterRequest) -> Result<(), AppError> {
    if !payload.email.contains('@') || payload.email.trim().len() < 5 {
        return Err(AppError::Validation("email is invalid".to_owned()));
    }
    if payload.password.len() < 8 {
        return Err(AppError::Validation(
            "password must be at least 8 characters".to_owned(),
        ));
    }
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("name is required".to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderMap;
    use serde_json::json;

    use super::*;
    use crate::config::Config;

    #[test]
    fn auth_json_never_contains_a_refresh_token() {
        let response = AuthResponse {
            access_token: "access-token".to_owned(),
            token_type: "Bearer".to_owned(),
            expires_in: 3600,
            user: AuthUser {
                id: Uuid::now_v7(),
                email: "user@example.invalid".to_owned(),
                name: "Test User".to_owned(),
                avatar_url: None,
                role: "author".to_owned(),
            },
            organizations: Vec::new(),
            default_organization_id: None,
        };

        let value = serde_json::to_value(response).unwrap();
        assert_eq!(value.get("access_token"), Some(&json!("access-token")));
        assert!(value.get("refresh_token").is_none());
    }

    #[test]
    fn refresh_token_is_accepted_only_from_the_http_only_cookie_contract() {
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_static("other=value; zinhar_refresh_token=cookie-token"),
        );
        assert_eq!(
            refresh_token_from_request(&headers).as_deref(),
            Some("cookie-token")
        );
        assert!(refresh_token_from_request(&HeaderMap::new()).is_none());
    }

    #[tokio::test]
    async fn cookie_authenticated_endpoints_enforce_browser_origin() {
        let config = Config::test_with_stripe_secret("test-webhook-secret");
        let db = crate::db::connect_lazy(&config.database_url).unwrap();
        let redis = redis::Client::open(config.redis_url.clone()).unwrap();
        let state = AppState::new(config, db, redis).unwrap();

        assert!(validate_cookie_request_origin(&HeaderMap::new(), &state).is_ok());
        let mut allowed = HeaderMap::new();
        allowed.insert(ORIGIN, HeaderValue::from_static("http://localhost:5173"));
        assert!(validate_cookie_request_origin(&allowed, &state).is_ok());

        let mut duplicate = HeaderMap::new();
        duplicate.append(ORIGIN, HeaderValue::from_static("http://localhost:5173"));
        duplicate.append(ORIGIN, HeaderValue::from_static("http://localhost:5173"));
        assert!(validate_cookie_request_origin(&duplicate, &state).is_err());

        for value in [
            "null",
            "https://evil.example.invalid",
            "http://localhost:5173/path",
        ] {
            let mut rejected = HeaderMap::new();
            rejected.insert(ORIGIN, HeaderValue::from_str(value).unwrap());
            assert!(validate_cookie_request_origin(&rejected, &state).is_err());
        }
    }

    #[tokio::test]
    async fn logout_cookie_clears_the_browser_session() {
        let config = Config::test_with_stripe_secret("test-webhook-secret");
        let db = crate::db::connect_lazy(&config.database_url).unwrap();
        let redis = redis::Client::open(config.redis_url.clone()).unwrap();
        let state = AppState::new(config, db, redis).unwrap();
        let cookie = clear_refresh_cookie(&state)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/api/auth"));
        assert!(cookie.contains("Max-Age=0"));
    }
}
