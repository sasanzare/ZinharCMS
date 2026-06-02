use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(module_status))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthModuleStatus {
    pub module: String,
    pub planned_endpoints: Vec<String>,
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
        planned_endpoints: [
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
