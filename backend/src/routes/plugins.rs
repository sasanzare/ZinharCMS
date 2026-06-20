use axum::extract::{Extension, Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::rbac;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/plugins", get(list_plugins))
        .route(
            "/api/plugins/{plugin_key}",
            get(get_plugin).put(update_plugin),
        )
        .route("/api/plugins/{plugin_key}/enable", post(enable_plugin))
        .route("/api/plugins/{plugin_key}/disable", post(disable_plugin))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PluginUpdateRequest {
    pub is_enabled: bool,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PluginResponse {
    pub id: Uuid,
    pub plugin_key: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub hooks: Vec<String>,
    pub is_enabled: bool,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/api/plugins",
    tag = "plugins",
    responses((status = 200, description = "CMS plugins", body = [PluginResponse]))
)]
pub async fn list_plugins(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<PluginResponse>>, AppError> {
    rbac::require_plugin_reader(&claims)?;
    sync_builtin_plugins(&state).await?;
    let rows = sqlx::query_as::<_, PluginResponse>(
        r#"
        SELECT id,
               plugin_key,
               name,
               version,
               description,
               hooks,
               is_enabled,
               is_system,
               created_at,
               updated_at
        FROM cms_plugins
        ORDER BY is_system DESC, name ASC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows))
}

#[utoipa::path(
    get,
    path = "/api/plugins/{plugin_key}",
    tag = "plugins",
    params(("plugin_key" = String, Path, description = "Plugin key")),
    responses((status = 200, description = "CMS plugin", body = PluginResponse))
)]
pub async fn get_plugin(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(plugin_key): Path<String>,
) -> Result<Json<PluginResponse>, AppError> {
    rbac::require_plugin_reader(&claims)?;
    load_plugin(&state, &plugin_key).await.map(Json)
}

#[utoipa::path(
    put,
    path = "/api/plugins/{plugin_key}",
    tag = "plugins",
    params(("plugin_key" = String, Path, description = "Plugin key")),
    request_body = PluginUpdateRequest,
    responses((status = 200, description = "Updated CMS plugin", body = PluginResponse))
)]
pub async fn update_plugin(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(plugin_key): Path<String>,
    Json(payload): Json<PluginUpdateRequest>,
) -> Result<Json<PluginResponse>, AppError> {
    rbac::require_plugin_manager(&claims)?;
    set_plugin_enabled(&state, &plugin_key, payload.is_enabled)
        .await
        .map(Json)
}

#[utoipa::path(
    post,
    path = "/api/plugins/{plugin_key}/enable",
    tag = "plugins",
    params(("plugin_key" = String, Path, description = "Plugin key")),
    responses((status = 200, description = "Enabled CMS plugin", body = PluginResponse))
)]
pub async fn enable_plugin(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(plugin_key): Path<String>,
) -> Result<Json<PluginResponse>, AppError> {
    rbac::require_plugin_manager(&claims)?;
    set_plugin_enabled(&state, &plugin_key, true)
        .await
        .map(Json)
}

#[utoipa::path(
    post,
    path = "/api/plugins/{plugin_key}/disable",
    tag = "plugins",
    params(("plugin_key" = String, Path, description = "Plugin key")),
    responses((status = 200, description = "Disabled CMS plugin", body = PluginResponse))
)]
pub async fn disable_plugin(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(plugin_key): Path<String>,
) -> Result<Json<PluginResponse>, AppError> {
    rbac::require_plugin_manager(&claims)?;
    set_plugin_enabled(&state, &plugin_key, false)
        .await
        .map(Json)
}

async fn set_plugin_enabled(
    state: &AppState,
    plugin_key: &str,
    is_enabled: bool,
) -> Result<PluginResponse, AppError> {
    validate_plugin_key(plugin_key)?;
    sqlx::query_as::<_, PluginResponse>(
        r#"
        UPDATE cms_plugins
        SET is_enabled = $2,
            updated_at = now()
        WHERE plugin_key = $1
        RETURNING id,
                  plugin_key,
                  name,
                  version,
                  description,
                  hooks,
                  is_enabled,
                  is_system,
                  created_at,
                  updated_at
        "#,
    )
    .bind(plugin_key)
    .bind(is_enabled)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

async fn load_plugin(state: &AppState, plugin_key: &str) -> Result<PluginResponse, AppError> {
    validate_plugin_key(plugin_key)?;
    sqlx::query_as::<_, PluginResponse>(
        r#"
        SELECT id,
               plugin_key,
               name,
               version,
               description,
               hooks,
               is_enabled,
               is_system,
               created_at,
               updated_at
        FROM cms_plugins
        WHERE plugin_key = $1
        "#,
    )
    .bind(plugin_key)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

async fn sync_builtin_plugins(state: &AppState) -> Result<(), AppError> {
    for plugin in crate::plugins::builtin_plugins() {
        let hooks = plugin
            .hooks()
            .iter()
            .map(|hook| (*hook).to_owned())
            .collect::<Vec<_>>();

        sqlx::query(
            r#"
            INSERT INTO cms_plugins (plugin_key, name, version, description, hooks, is_enabled, is_system)
            VALUES ($1, $2, $3, $4, $5, TRUE, TRUE)
            ON CONFLICT (plugin_key) DO UPDATE
            SET name = EXCLUDED.name,
                version = EXCLUDED.version,
                description = EXCLUDED.description,
                hooks = EXCLUDED.hooks,
                is_system = TRUE,
                updated_at = now()
            "#,
        )
        .bind(plugin.key())
        .bind(plugin.name())
        .bind(plugin.version())
        .bind(plugin.description())
        .bind(hooks)
        .execute(&state.db)
        .await?;
    }

    Ok(())
}

fn validate_plugin_key(value: &str) -> Result<(), AppError> {
    let valid = !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--");
    if valid {
        Ok(())
    } else {
        Err(AppError::Validation("plugin_key is invalid".to_owned()))
    }
}
