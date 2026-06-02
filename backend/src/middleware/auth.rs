use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub async fn auth_middleware(
    State(_state): State<AppState>,
    req: Request,
    _next: Next,
) -> Result<Response, StatusCode> {
    let _token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Phase zero only wires the auth boundary. JWT verification is implemented
    // with the full auth service in phase one before any protected route uses it.
    Err(StatusCode::UNAUTHORIZED)
}
