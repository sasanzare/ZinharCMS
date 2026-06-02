use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(module_status))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ContentModuleStatus {
    pub module: String,
    pub planned_resources: Vec<String>,
}

#[utoipa::path(
    get,
    path = "/api/content",
    tag = "content",
    responses((status = 200, description = "Content module status", body = ContentModuleStatus))
)]
pub async fn module_status() -> Json<ContentModuleStatus> {
    Json(ContentModuleStatus {
        module: "content".to_owned(),
        planned_resources: ["content-types", "entries", "versions", "publishing"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
    })
}
