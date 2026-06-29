use reqwest::Client;
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::rls;

pub async fn send_invitation_email(
    pool: &PgPool,
    config: &Config,
    tenant: &TenantContext,
    recipient_email: &str,
    accept_path: &str,
) -> Result<(), AppError> {
    let accept_url = absolute_url(&config.app_base_url, accept_path);
    let subject = format!("Invitation to {}", tenant.organization_name);
    let body = format!(
        "You have been invited to join {} in ZinharCMS. Open this link to accept: {}",
        tenant.organization_name, accept_url
    );
    let payload = json!({
        "accept_url": accept_url,
        "organization_id": tenant.organization_id,
        "organization_name": tenant.organization_name,
    });

    send_transactional_email(
        pool,
        config,
        tenant.organization_id,
        recipient_email,
        "organization_invitation",
        &subject,
        &body,
        payload,
    )
    .await
}

pub async fn send_billing_notification(
    pool: &PgPool,
    config: &Config,
    tenant: &TenantContext,
    recipient_email: &str,
    plan_name: &str,
    status: &str,
) -> Result<(), AppError> {
    let subject = format!("Billing update for {}", tenant.organization_name);
    let body = format!(
        "{} billing changed to plan {} with status {}.",
        tenant.organization_name, plan_name, status
    );
    let payload = json!({
        "organization_id": tenant.organization_id,
        "organization_name": tenant.organization_name,
        "plan": plan_name,
        "status": status,
    });

    send_transactional_email(
        pool,
        config,
        tenant.organization_id,
        recipient_email,
        "billing_notification",
        &subject,
        &body,
        payload,
    )
    .await
}

async fn send_transactional_email(
    pool: &PgPool,
    config: &Config,
    organization_id: Uuid,
    recipient_email: &str,
    template: &str,
    subject: &str,
    body: &str,
    payload: Value,
) -> Result<(), AppError> {
    let provider = config.email_provider.trim().to_ascii_lowercase();
    let mut db = rls::organization_connection(pool, organization_id, None).await?;
    let delivery_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO email_deliveries (
          organization_id,
          recipient_email,
          template,
          subject,
          provider,
          payload
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(recipient_email)
    .bind(template)
    .bind(subject)
    .bind(&provider)
    .bind(json!({
        "from": &config.email_from,
        "body": body,
        "data": payload,
    }))
    .fetch_one(db.as_mut())
    .await?;

    let result = match provider.as_str() {
        "disabled" => DeliveryResult::skipped("email provider disabled"),
        "webhook" => send_webhook(config, recipient_email, template, subject, body, payload).await,
        _ => DeliveryResult::sent(format!("log:{delivery_id}")),
    };

    update_delivery(pool, organization_id, delivery_id, &result).await?;
    if result.is_failure() && config.email_failure_mode == "strict" {
        return Err(AppError::ServiceUnavailable(format!(
            "email provider failed: {}",
            result.error.unwrap_or_else(|| "unknown error".to_owned())
        )));
    }

    Ok(())
}

async fn send_webhook(
    config: &Config,
    recipient_email: &str,
    template: &str,
    subject: &str,
    body: &str,
    payload: Value,
) -> DeliveryResult {
    let Some(url) = config.email_webhook_url.as_ref() else {
        return DeliveryResult::failed("EMAIL_WEBHOOK_URL is not configured");
    };
    let response = Client::new()
        .post(url)
        .json(&json!({
            "from": &config.email_from,
            "to": recipient_email,
            "template": template,
            "subject": subject,
            "body": body,
            "payload": payload,
        }))
        .send()
        .await;

    match response {
        Ok(response) if response.status().is_success() => {
            let provider_message_id = response
                .headers()
                .get("x-message-id")
                .and_then(|value| value.to_str().ok())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("webhook:{}", uuid::Uuid::now_v7()));
            DeliveryResult::sent(provider_message_id)
        }
        Ok(response) => DeliveryResult::failed(format!("webhook returned {}", response.status())),
        Err(error) => DeliveryResult::failed(error.to_string()),
    }
}

async fn update_delivery(
    pool: &PgPool,
    organization_id: Uuid,
    delivery_id: Uuid,
    result: &DeliveryResult,
) -> Result<(), AppError> {
    let mut db = rls::organization_connection(pool, organization_id, None).await?;
    sqlx::query(
        r#"
        UPDATE email_deliveries
        SET status = $3,
            provider_message_id = $4,
            error = $5,
            sent_at = CASE WHEN $3 = 'sent' THEN now() ELSE sent_at END,
            updated_at = now()
        WHERE id = $1
          AND organization_id = $2
        "#,
    )
    .bind(delivery_id)
    .bind(organization_id)
    .bind(&result.status)
    .bind(&result.provider_message_id)
    .bind(&result.error)
    .execute(db.as_mut())
    .await?;
    Ok(())
}

fn absolute_url(base_url: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

struct DeliveryResult {
    status: String,
    provider_message_id: Option<String>,
    error: Option<String>,
}

impl DeliveryResult {
    fn sent(provider_message_id: String) -> Self {
        Self {
            status: "sent".to_owned(),
            provider_message_id: Some(provider_message_id),
            error: None,
        }
    }

    fn skipped(reason: &str) -> Self {
        Self {
            status: "skipped".to_owned(),
            provider_message_id: None,
            error: Some(reason.to_owned()),
        }
    }

    fn failed(reason: impl ToString) -> Self {
        Self {
            status: "failed".to_owned(),
            provider_message_id: None,
            error: Some(reason.to_string()),
        }
    }

    fn is_failure(&self) -> bool {
        self.status == "failed"
    }
}

#[cfg(test)]
mod tests {
    use super::absolute_url;

    #[test]
    fn absolute_url_joins_base_and_path() {
        assert_eq!(
            absolute_url("http://localhost:5173/", "/organization?invite=abc"),
            "http://localhost:5173/organization?invite=abc"
        );
    }
}
