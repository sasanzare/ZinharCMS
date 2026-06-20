use std::time::Duration;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::jwt;
use crate::state::AppState;

type HmacSha256 = Hmac<Sha256>;

pub const ENTRY_PUBLISH: &str = "entry.publish";
pub const ENTRY_UNPUBLISH: &str = "entry.unpublish";
pub const PAGE_PUBLISH: &str = "page.publish";
pub const PAGE_UNPUBLISH: &str = "page.unpublish";

pub const SUPPORTED_EVENTS: &[&str] =
    &[ENTRY_PUBLISH, ENTRY_UNPUBLISH, PAGE_PUBLISH, PAGE_UNPUBLISH];

#[derive(Debug, Clone, FromRow)]
pub struct Webhook {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn generate_secret() -> String {
    jwt::generate_refresh_token()
}

pub fn validate_url(value: &str) -> Result<(), AppError> {
    let url = reqwest::Url::parse(value)
        .map_err(|_| AppError::Validation("webhook url must be a valid URL".to_owned()))?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(AppError::Validation(
            "webhook url must use http or https with a host".to_owned(),
        ));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(AppError::Validation(
            "webhook url cannot include credentials".to_owned(),
        ));
    }
    Ok(())
}

pub fn validate_events(events: &[String]) -> Result<(), AppError> {
    if events.is_empty() {
        return Err(AppError::Validation(
            "webhook events cannot be empty".to_owned(),
        ));
    }
    for event in events {
        if !SUPPORTED_EVENTS.contains(&event.as_str()) {
            return Err(AppError::Validation(format!(
                "webhook event '{event}' is not supported"
            )));
        }
    }
    Ok(())
}

pub fn sign_payload(secret: &str, body: &str) -> Result<String, AppError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|error| AppError::Internal(error.to_string()))?;
    mac.update(body.as_bytes());
    Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
}

pub async fn trigger_event(state: &AppState, event: &'static str, payload: Value) {
    let webhooks = match sqlx::query_as::<_, Webhook>(
        r#"
        SELECT id, name, url, events, secret, is_active, created_at, updated_at
        FROM webhooks
        WHERE is_active = TRUE AND $1 = ANY(events)
        "#,
    )
    .bind(event)
    .fetch_all(&state.db)
    .await
    {
        Ok(webhooks) => webhooks,
        Err(error) => {
            tracing::warn!(%event, %error, "failed to load webhooks for event");
            return;
        }
    };

    for webhook in webhooks {
        let state = state.clone();
        let payload = payload.clone();
        tokio::spawn(async move {
            if let Err(error) = dispatch_webhook(&state, &webhook, event, &payload).await {
                tracing::warn!(
                    webhook_id = %webhook.id,
                    webhook_name = %webhook.name,
                    %event,
                    %error,
                    "webhook dispatch failed"
                );
            }
        });
    }
}

pub async fn dispatch_webhook(
    state: &AppState,
    webhook: &Webhook,
    event: &str,
    payload: &Value,
) -> Result<(), AppError> {
    let body =
        serde_json::to_string(payload).map_err(|error| AppError::Internal(error.to_string()))?;
    let signature = sign_payload(&webhook.secret, &body)?;
    let response = reqwest::Client::new()
        .post(&webhook.url)
        .header("X-CMS-Event", event)
        .header("X-CMS-Signature", signature)
        .header("Content-Type", "application/json")
        .body(body)
        .timeout(Duration::from_secs(10))
        .send()
        .await;

    let delivery = match response {
        Ok(response) => {
            let status_code = response.status().as_u16() as i32;
            let delivered = response.status().is_success();
            let response_body = response.text().await.unwrap_or_default();
            DeliveryAttempt {
                webhook_id: webhook.id,
                event,
                payload,
                status: if delivered { "delivered" } else { "failed" },
                status_code: Some(status_code),
                response_body: Some(truncate(&response_body, 2_000)),
                error: None,
            }
        }
        Err(error) => DeliveryAttempt {
            webhook_id: webhook.id,
            event,
            payload,
            status: "failed",
            status_code: None,
            response_body: None,
            error: Some(truncate(&error.to_string(), 2_000)),
        },
    };

    record_delivery(state, delivery).await?;

    Ok(())
}

struct DeliveryAttempt<'a> {
    webhook_id: Uuid,
    event: &'a str,
    payload: &'a Value,
    status: &'static str,
    status_code: Option<i32>,
    response_body: Option<String>,
    error: Option<String>,
}

async fn record_delivery(state: &AppState, attempt: DeliveryAttempt<'_>) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO webhook_deliveries (
          webhook_id,
          event,
          payload,
          status,
          status_code,
          response_body,
          error
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(attempt.webhook_id)
    .bind(attempt.event)
    .bind(attempt.payload)
    .bind(attempt.status)
    .bind(attempt.status_code)
    .bind(attempt.response_body)
    .bind(attempt.error)
    .execute(&state.db)
    .await?;

    Ok(())
}

fn truncate(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_payload_is_stable_for_same_secret_and_body() {
        let first = sign_payload("secret", r#"{"event":"page.publish"}"#).unwrap();
        let second = sign_payload("secret", r#"{"event":"page.publish"}"#).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn validate_events_rejects_unknown_events() {
        let events = vec!["page.publish".to_owned(), "unknown".to_owned()];
        assert!(validate_events(&events).is_err());
    }
}
