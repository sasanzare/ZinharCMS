use axum::extract::{Extension, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::{jwt, password, rbac};
use crate::state::AppState;

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth", get(module_status))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh))
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(me))
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub revoked: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: AuthUser,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
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

#[derive(Debug, FromRow)]
struct RefreshRecord {
    user_id: Uuid,
    role: String,
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
) -> Result<Json<AuthResponse>, AppError> {
    validate_register(&payload)?;

    let email = payload.email.trim().to_ascii_lowercase();
    let password_hash = password::hash_password(&payload.password)?;
    let existing_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await?;
    let role = rbac::default_registration_role(existing_users);

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
    tx.commit().await?;

    issue_auth_response(&state, user).await.map(Json)
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses((status = 200, description = "Token pair", body = AuthResponse))
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let email = payload.email.trim().to_ascii_lowercase();
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
    .await?
    .ok_or_else(|| AppError::Unauthorized("invalid email or password".to_owned()))?;

    if !password::verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized(
            "invalid email or password".to_owned(),
        ));
    }

    issue_auth_response(
        &state,
        AuthUser {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
            role: user.role,
        },
    )
    .await
    .map(Json)
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses((status = 200, description = "Rotated token pair", body = AuthResponse))
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let token_hash = jwt::hash_refresh_token(&payload.refresh_token);
    let record = sqlx::query_as::<_, RefreshRecord>(
        r#"
        SELECT rt.user_id, r.name as role
        FROM refresh_tokens rt
        JOIN users u ON u.id = rt.user_id
        JOIN user_roles ur ON ur.user_id = u.id
        JOIN roles r ON r.id = ur.role_id
        WHERE rt.token_hash = $1
          AND rt.revoked_at IS NULL
          AND rt.expires_at > now()
          AND u.is_active = true
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
    .bind(&token_hash)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("invalid refresh token".to_owned()))?;

    sqlx::query("UPDATE refresh_tokens SET revoked_at = now() WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(&state.db)
        .await?;

    let mut user = load_auth_user(&state, record.user_id).await?;
    user.role = record.role;
    issue_auth_response(&state, user).await.map(Json)
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    request_body = LogoutRequest,
    responses((status = 200, description = "Logout result", body = LogoutResponse))
)]
pub async fn logout(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, AppError> {
    let token_hash = jwt::hash_refresh_token(&payload.refresh_token);
    let result = sqlx::query(
        "UPDATE refresh_tokens SET revoked_at = now() WHERE token_hash = $1 AND revoked_at IS NULL",
    )
    .bind(token_hash)
    .execute(&state.db)
    .await?;

    Ok(Json(LogoutResponse {
        revoked: result.rows_affected() > 0,
    }))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "auth",
    responses((status = 200, description = "Current user", body = AuthUser))
)]
pub async fn me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<AuthUser>, AppError> {
    load_auth_user(&state, claims.sub).await.map(Json)
}

async fn issue_auth_response(state: &AppState, user: AuthUser) -> Result<AuthResponse, AppError> {
    let access_token = jwt::sign_access_token(user.id, &user.role, &state.config)?;
    let refresh_token = jwt::generate_refresh_token();
    let token_hash = jwt::hash_refresh_token(&refresh_token);
    let expires_at = Utc::now() + Duration::seconds(state.config.jwt_refresh_expiry as i64);

    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.id)
    .bind(token_hash)
    .bind(expires_at)
    .execute(&state.db)
    .await?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_owned(),
        expires_in: state.config.jwt_access_expiry,
        user,
    })
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
