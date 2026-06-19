use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::services::jwt;
use crate::state::AppState;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .or_else(|| preview_query_token(&req))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims =
        jwt::verify_access_token(token, &state.config).map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

fn preview_query_token(req: &Request) -> Option<&str> {
    req.uri()
        .path()
        .starts_with("/api/preview/")
        .then(|| req.uri().query().and_then(token_from_query))?
}

fn token_from_query(query: &str) -> Option<&str> {
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        if matches!(key, "access_token" | "token") && !value.is_empty() {
            Some(value)
        } else {
            None
        }
    })
}
