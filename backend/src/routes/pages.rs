use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(module_status))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PagesModuleStatus {
    pub module: String,
    pub planned_features: Vec<String>,
}

#[utoipa::path(
    get,
    path = "/api/pages",
    tag = "pages",
    responses((status = 200, description = "Pages module status", body = PagesModuleStatus))
)]
pub async fn module_status() -> Json<PagesModuleStatus> {
    Json(PagesModuleStatus {
        module: "pages".to_owned(),
        planned_features: [
            "page_json",
            "component_registry",
            "page_versions",
            "live_preview",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
    })
}
