use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(module_status))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaModuleStatus {
    pub module: String,
    pub planned_features: Vec<String>,
}

#[utoipa::path(
    get,
    path = "/api/media",
    tag = "media",
    responses((status = 200, description = "Media module status", body = MediaModuleStatus))
)]
pub async fn module_status() -> Json<MediaModuleStatus> {
    Json(MediaModuleStatus {
        module: "media".to_owned(),
        planned_features: ["upload", "metadata", "variants", "secure storage"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
    })
}
