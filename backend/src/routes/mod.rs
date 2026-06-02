pub mod auth;
pub mod content;
pub mod media;
pub mod pages;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use redis::AsyncCommands;
use serde::Serialize;
use sqlx::Executor;
use utoipa::{OpenApi, ToSchema};

use crate::error::AppError;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/ready", get(readiness))
        .route("/openapi.json", get(openapi))
        .nest("/api/auth", auth::router())
        .nest("/api/content", content::router())
        .nest("/api/media", media::router())
        .nest("/api/pages", pages::router())
        .with_state(state)
}

#[derive(OpenApi)]
#[openapi(
    paths(root, health, readiness, auth::module_status, content::module_status, media::module_status, pages::module_status),
    components(schemas(
        ApiInfo,
        HealthResponse,
        ReadyResponse,
        DependencyCheck,
        auth::AuthModuleStatus,
        content::ContentModuleStatus,
        media::MediaModuleStatus,
        pages::PagesModuleStatus
    )),
    tags(
        (name = "system", description = "Phase-zero system endpoints"),
        (name = "auth", description = "Authentication module placeholder"),
        (name = "content", description = "Content module placeholder"),
        (name = "media", description = "Media module placeholder"),
        (name = "pages", description = "Page builder module placeholder")
    )
)]
struct ApiDoc;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub docs: String,
    pub health: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReadyResponse {
    pub status: String,
    pub checks: Vec<DependencyCheck>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DependencyCheck {
    pub name: String,
    pub ok: bool,
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "system",
    responses((status = 200, description = "API metadata", body = ApiInfo))
)]
async fn root() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "ZangarCMS API".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        docs: "/openapi.json".to_owned(),
        health: "/health".to_owned(),
    })
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses((status = 200, description = "Liveness check", body = HealthResponse))
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}

#[utoipa::path(
    get,
    path = "/ready",
    tag = "system",
    responses((status = 200, description = "Readiness check", body = ReadyResponse))
)]
async fn readiness(State(state): State<AppState>) -> Result<Json<ReadyResponse>, AppError> {
    let mut checks = Vec::with_capacity(2);

    let db_ok = match state.db.execute("SELECT 1").await {
        Ok(_) => DependencyCheck {
            name: "postgres".to_owned(),
            ok: true,
            message: "reachable".to_owned(),
        },
        Err(error) => DependencyCheck {
            name: "postgres".to_owned(),
            ok: false,
            message: error.to_string(),
        },
    };
    checks.push(db_ok);

    let redis_ok = match state.redis.get_multiplexed_async_connection().await {
        Ok(mut connection) => match connection.ping::<String>().await {
            Ok(_) => DependencyCheck {
                name: "redis".to_owned(),
                ok: true,
                message: "reachable".to_owned(),
            },
            Err(error) => DependencyCheck {
                name: "redis".to_owned(),
                ok: false,
                message: error.to_string(),
            },
        },
        Err(error) => DependencyCheck {
            name: "redis".to_owned(),
            ok: false,
            message: error.to_string(),
        },
    };
    checks.push(redis_ok);

    let all_ok = checks.iter().all(|check| check.ok);
    let response = ReadyResponse {
        status: (if all_ok { "ready" } else { "degraded" }).to_owned(),
        checks,
    };

    if all_ok {
        Ok(Json(response))
    } else {
        Err(AppError::ServiceUnavailable(
            serde_json::to_string(&response)
                .unwrap_or_else(|_| "dependency check failed".to_owned()),
        ))
    }
}

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
