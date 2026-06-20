use axum::extract::{Extension, Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::{rbac, webhooks};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/webhooks", get(list_webhooks).post(create_webhook))
        .route(
            "/api/webhooks/{id}",
            get(get_webhook).put(update_webhook).delete(delete_webhook),
        )
        .route("/api/webhooks/{id}/deliveries", get(list_deliveries))
        .route("/api/webhooks/{id}/test", post(test_webhook))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct WebhookRequest {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct WebhookResponse {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct WebhookDeliveryResponse {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event: String,
    pub payload: Value,
    pub status: String,
    pub status_code: Option<i32>,
    pub response_body: Option<String>,
    pub error: Option<String>,
    pub attempted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryListQuery {
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteConfirm {
    pub confirm: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WebhookTestResponse {
    pub sent: bool,
    pub event: String,
}

#[utoipa::path(
    get,
    path = "/api/webhooks",
    tag = "webhooks",
    responses((status = 200, description = "Webhook subscriptions", body = [WebhookResponse]))
)]
pub async fn list_webhooks(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<WebhookResponse>>, AppError> {
    rbac::require_webhook_manager(&claims)?;
    let rows = sqlx::query_as::<_, WebhookResponse>(
        r#"
        SELECT id, name, url, events, secret, is_active, created_at, updated_at
        FROM webhooks
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows))
}

#[utoipa::path(
    post,
    path = "/api/webhooks",
    tag = "webhooks",
    request_body = WebhookRequest,
    responses((status = 200, description = "Created webhook", body = WebhookResponse))
)]
pub async fn create_webhook(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<WebhookRequest>,
) -> Result<Json<WebhookResponse>, AppError> {
    rbac::require_webhook_manager(&claims)?;
    let events = normalize_events(payload.events)?;
    let secret = normalize_secret(payload.secret)?.unwrap_or_else(webhooks::generate_secret);
    validate_webhook_request(&payload.name, &payload.url, &events, Some(&secret))?;

    let row = sqlx::query_as::<_, WebhookResponse>(
        r#"
        INSERT INTO webhooks (name, url, events, secret, is_active)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, url, events, secret, is_active, created_at, updated_at
        "#,
    )
    .bind(payload.name.trim())
    .bind(payload.url.trim())
    .bind(events)
    .bind(secret)
    .bind(payload.is_active.unwrap_or(true))
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/webhooks/{id}",
    tag = "webhooks",
    params(("id" = Uuid, Path, description = "Webhook id")),
    responses((status = 200, description = "Webhook", body = WebhookResponse))
)]
pub async fn get_webhook(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<WebhookResponse>, AppError> {
    rbac::require_webhook_manager(&claims)?;
    load_webhook_response(&state, id).await.map(Json)
}

#[utoipa::path(
    put,
    path = "/api/webhooks/{id}",
    tag = "webhooks",
    params(("id" = Uuid, Path, description = "Webhook id")),
    request_body = WebhookRequest,
    responses((status = 200, description = "Updated webhook", body = WebhookResponse))
)]
pub async fn update_webhook(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<WebhookRequest>,
) -> Result<Json<WebhookResponse>, AppError> {
    rbac::require_webhook_manager(&claims)?;
    let events = normalize_events(payload.events)?;
    let secret = normalize_secret(payload.secret)?;
    validate_webhook_request(&payload.name, &payload.url, &events, secret.as_deref())?;

    let row = sqlx::query_as::<_, WebhookResponse>(
        r#"
        UPDATE webhooks
        SET name = $2,
            url = $3,
            events = $4,
            secret = COALESCE($5, secret),
            is_active = $6,
            updated_at = now()
        WHERE id = $1
        RETURNING id, name, url, events, secret, is_active, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(payload.name.trim())
    .bind(payload.url.trim())
    .bind(events)
    .bind(secret)
    .bind(payload.is_active.unwrap_or(true))
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/api/webhooks/{id}",
    tag = "webhooks",
    params(("id" = Uuid, Path, description = "Webhook id")),
    responses((status = 200, description = "Deleted webhook", body = WebhookResponse))
)]
pub async fn delete_webhook(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Query(query): Query<DeleteConfirm>,
) -> Result<Json<WebhookResponse>, AppError> {
    rbac::require_webhook_manager(&claims)?;
    if query.confirm != Some(true) {
        return Err(AppError::Validation(
            "pass ?confirm=true to delete a webhook".to_owned(),
        ));
    }

    let row = sqlx::query_as::<_, WebhookResponse>(
        r#"
        DELETE FROM webhooks
        WHERE id = $1
        RETURNING id, name, url, events, secret, is_active, created_at, updated_at
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/webhooks/{id}/deliveries",
    tag = "webhooks",
    params(("id" = Uuid, Path, description = "Webhook id")),
    responses((status = 200, description = "Recent webhook deliveries", body = [WebhookDeliveryResponse]))
)]
pub async fn list_deliveries(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Query(query): Query<DeliveryListQuery>,
) -> Result<Json<Vec<WebhookDeliveryResponse>>, AppError> {
    rbac::require_webhook_manager(&claims)?;
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let rows = sqlx::query_as::<_, WebhookDeliveryResponse>(
        r#"
        SELECT id,
               webhook_id,
               event,
               payload,
               status,
               status_code,
               response_body,
               error,
               attempted_at
        FROM webhook_deliveries
        WHERE webhook_id = $1
        ORDER BY attempted_at DESC
        LIMIT $2
        "#,
    )
    .bind(id)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows))
}

#[utoipa::path(
    post,
    path = "/api/webhooks/{id}/test",
    tag = "webhooks",
    params(("id" = Uuid, Path, description = "Webhook id")),
    responses((status = 200, description = "Webhook test result", body = WebhookTestResponse))
)]
pub async fn test_webhook(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<WebhookTestResponse>, AppError> {
    rbac::require_webhook_manager(&claims)?;
    let webhook = load_webhook(&state, id).await?;
    let event = webhook
        .events
        .first()
        .cloned()
        .unwrap_or_else(|| webhooks::PAGE_PUBLISH.to_owned());
    let payload = serde_json::json!({
        "event": event,
        "test": true,
        "webhook_id": webhook.id,
        "sent_at": Utc::now(),
    });
    webhooks::dispatch_webhook(&state, &webhook, &event, &payload).await?;

    Ok(Json(WebhookTestResponse { sent: true, event }))
}

async fn load_webhook_response(state: &AppState, id: Uuid) -> Result<WebhookResponse, AppError> {
    sqlx::query_as::<_, WebhookResponse>(
        r#"
        SELECT id, name, url, events, secret, is_active, created_at, updated_at
        FROM webhooks
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

async fn load_webhook(state: &AppState, id: Uuid) -> Result<webhooks::Webhook, AppError> {
    sqlx::query_as::<_, webhooks::Webhook>(
        r#"
        SELECT id, name, url, events, secret, is_active, created_at, updated_at
        FROM webhooks
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

fn validate_webhook_request(
    name: &str,
    url: &str,
    events: &[String],
    secret: Option<&str>,
) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::Validation("name is required".to_owned()));
    }
    webhooks::validate_url(url.trim())?;
    webhooks::validate_events(events)?;
    if let Some(secret) = secret
        && secret.len() < 16
    {
        return Err(AppError::Validation(
            "secret must be at least 16 characters".to_owned(),
        ));
    }
    Ok(())
}

fn normalize_events(events: Vec<String>) -> Result<Vec<String>, AppError> {
    let mut events = events
        .into_iter()
        .map(|event| event.trim().to_owned())
        .filter(|event| !event.is_empty())
        .collect::<Vec<_>>();
    events.sort();
    events.dedup();
    webhooks::validate_events(&events)?;
    Ok(events)
}

fn normalize_secret(secret: Option<String>) -> Result<Option<String>, AppError> {
    let secret = secret.map(|secret| secret.trim().to_owned());
    match secret {
        Some(secret) if secret.is_empty() => Ok(None),
        Some(secret) if secret.len() < 16 => Err(AppError::Validation(
            "secret must be at least 16 characters".to_owned(),
        )),
        other => Ok(other),
    }
}
