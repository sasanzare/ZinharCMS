use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::Value;
use sha2::Sha256;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::{quota, rls};

type HmacSha256 = Hmac<Sha256>;

const STRIPE_API_BASE: &str = "https://api.stripe.com/v1";
const STRIPE_SIGNATURE_TOLERANCE_SECONDS: i64 = 300;

#[derive(Debug, Clone)]
pub struct CheckoutSession {
    pub session_id: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct CustomerPortalSession {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct WebhookResult {
    pub event_id: String,
    pub event_type: String,
    pub status: String,
    pub already_processed: bool,
}

#[derive(Debug, FromRow)]
struct OrganizationBillingProfile {
    organization_id: Uuid,
    organization_name: String,
    owner_email: Option<String>,
}

pub fn price_id_for_plan(config: &Config, plan: &quota::PlanLimits) -> Option<String> {
    plan.stripe_price_id
        .clone()
        .or_else(|| match plan.slug.as_str() {
            "pro" => config.stripe_pro_price_id.clone(),
            "enterprise" => config.stripe_enterprise_price_id.clone(),
            _ => None,
        })
}

pub async fn create_checkout_session(
    pool: &PgPool,
    config: &Config,
    tenant: &TenantContext,
    plan_slug: &str,
) -> Result<CheckoutSession, AppError> {
    let plan = load_plan_by_slug(pool, plan_slug).await?;
    let price_id = price_id_for_plan(config, &plan).ok_or_else(|| {
        AppError::Validation(format!(
            "Stripe price is not configured for the {} plan",
            plan.name
        ))
    })?;
    let customer_id = ensure_customer(pool, config, tenant).await?;

    let params = vec![
        ("mode".to_owned(), "subscription".to_owned()),
        ("customer".to_owned(), customer_id),
        ("success_url".to_owned(), config.stripe_success_url.clone()),
        ("cancel_url".to_owned(), config.stripe_cancel_url.clone()),
        ("line_items[0][price]".to_owned(), price_id),
        ("line_items[0][quantity]".to_owned(), "1".to_owned()),
        (
            "client_reference_id".to_owned(),
            tenant.organization_id.to_string(),
        ),
        (
            "metadata[organization_id]".to_owned(),
            tenant.organization_id.to_string(),
        ),
        ("metadata[plan_slug]".to_owned(), plan.slug.clone()),
        (
            "subscription_data[metadata][organization_id]".to_owned(),
            tenant.organization_id.to_string(),
        ),
        (
            "subscription_data[metadata][plan_slug]".to_owned(),
            plan.slug,
        ),
    ];
    let payload = stripe_post(config, "checkout/sessions", params).await?;
    Ok(CheckoutSession {
        session_id: required_string(&payload, "id")?,
        url: required_string(&payload, "url")?,
    })
}

pub async fn create_customer_portal_session(
    pool: &PgPool,
    config: &Config,
    tenant: &TenantContext,
) -> Result<CustomerPortalSession, AppError> {
    let customer_id = load_customer_id(pool, tenant).await?.ok_or_else(|| {
        AppError::Validation("Stripe customer is not available for this organization".to_owned())
    })?;
    let params = vec![
        ("customer".to_owned(), customer_id),
        (
            "return_url".to_owned(),
            config.stripe_portal_return_url.clone(),
        ),
    ];
    let payload = stripe_post(config, "billing_portal/sessions", params).await?;
    Ok(CustomerPortalSession {
        url: required_string(&payload, "url")?,
    })
}

pub async fn handle_webhook(
    pool: &PgPool,
    config: &Config,
    signature: &str,
    body: &[u8],
) -> Result<WebhookResult, AppError> {
    verify_signature(config, signature, body)?;
    let payload: Value = serde_json::from_slice(body)
        .map_err(|error| AppError::BadRequest(format!("invalid Stripe payload: {error}")))?;
    let event_id = required_string(&payload, "id")?;
    let event_type = required_string(&payload, "type")?;

    let mut tx = rls::begin_bypass_transaction(pool).await?;
    let inserted = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO billing_events (provider, provider_event_id, event_type, payload, status)
        VALUES ('stripe', $1, $2, $3, 'processing')
        ON CONFLICT (provider, provider_event_id) DO NOTHING
        RETURNING id
        "#,
    )
    .bind(&event_id)
    .bind(&event_type)
    .bind(&payload)
    .fetch_optional(&mut *tx)
    .await?;

    if inserted.is_none() {
        tx.commit().await?;
        return Ok(WebhookResult {
            event_id,
            event_type,
            status: "already_processed".to_owned(),
            already_processed: true,
        });
    }

    let processing_result = process_event(&mut tx, config, &event_type, &payload).await;
    match processing_result {
        Ok(EventOutcome::Processed(organization_id)) => {
            update_billing_event(&mut tx, &event_id, organization_id, "processed", None).await?;
            tx.commit().await?;
            Ok(WebhookResult {
                event_id,
                event_type,
                status: "processed".to_owned(),
                already_processed: false,
            })
        }
        Ok(EventOutcome::Ignored(organization_id)) => {
            update_billing_event(&mut tx, &event_id, organization_id, "ignored", None).await?;
            tx.commit().await?;
            Ok(WebhookResult {
                event_id,
                event_type,
                status: "ignored".to_owned(),
                already_processed: false,
            })
        }
        Err(error) => {
            let message = error.to_string();
            update_billing_event(&mut tx, &event_id, None, "failed", Some(&message)).await?;
            tx.commit().await?;
            Err(error)
        }
    }
}

enum EventOutcome {
    Processed(Option<Uuid>),
    Ignored(Option<Uuid>),
}

async fn process_event(
    tx: &mut Transaction<'_, Postgres>,
    config: &Config,
    event_type: &str,
    payload: &Value,
) -> Result<EventOutcome, AppError> {
    match event_type {
        "checkout.session.completed" => {
            let organization_id = apply_checkout_completed(tx, payload).await?;
            Ok(EventOutcome::Processed(Some(organization_id)))
        }
        "customer.subscription.updated" => {
            let organization_id = apply_subscription_event(tx, config, payload, false).await?;
            Ok(EventOutcome::Processed(Some(organization_id)))
        }
        "customer.subscription.deleted" => {
            let organization_id = apply_subscription_event(tx, config, payload, true).await?;
            Ok(EventOutcome::Processed(Some(organization_id)))
        }
        _ => Ok(EventOutcome::Ignored(None)),
    }
}

async fn apply_checkout_completed(
    tx: &mut Transaction<'_, Postgres>,
    payload: &Value,
) -> Result<Uuid, AppError> {
    let object = event_object(payload)?;
    let organization_id = organization_id_from_object(object)?;
    let plan_slug = metadata_string(object, "plan_slug").ok_or_else(|| {
        AppError::Validation("Stripe checkout metadata.plan_slug is missing".to_owned())
    })?;
    let plan_id = plan_id_by_slug(tx, &plan_slug).await?.ok_or_else(|| {
        AppError::Validation(format!(
            "plan {plan_slug} from Stripe metadata was not found"
        ))
    })?;
    let customer_id = object_string(object, "customer");
    let subscription_id = object_string(object, "subscription");

    upsert_subscription(
        tx,
        organization_id,
        plan_id,
        "active",
        customer_id.as_deref(),
        subscription_id.as_deref(),
        None,
        None,
        false,
    )
    .await?;
    Ok(organization_id)
}

async fn apply_subscription_event(
    tx: &mut Transaction<'_, Postgres>,
    config: &Config,
    payload: &Value,
    deleted: bool,
) -> Result<Uuid, AppError> {
    let object = event_object(payload)?;
    let subscription_id = object_string(object, "id");
    let customer_id = object_string(object, "customer");
    let organization_id = match organization_id_from_object(object) {
        Ok(id) => id,
        Err(_) => {
            organization_id_by_provider_ids(tx, subscription_id.as_deref(), customer_id.as_deref())
                .await?
                .ok_or_else(|| {
                    AppError::Validation(
                        "organization could not be resolved from Stripe subscription event"
                            .to_owned(),
                    )
                })?
        }
    };

    let metadata_plan_slug = metadata_string(object, "plan_slug");
    let price_id = subscription_price_id(object);
    let plan_id = plan_id_for_subscription(
        tx,
        config,
        price_id.as_deref(),
        metadata_plan_slug.as_deref(),
    )
    .await?;
    let plan_id = match plan_id {
        Some(id) => id,
        None => current_subscription_plan_id(tx, organization_id).await?,
    };
    let status = if deleted {
        "canceled".to_owned()
    } else {
        stripe_status_to_internal(object_string(object, "status").as_deref())
    };
    let period_start = unix_time_field(object, "current_period_start");
    let period_end = unix_time_field(object, "current_period_end");
    let cancel_at_period_end = object
        .get("cancel_at_period_end")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    upsert_subscription(
        tx,
        organization_id,
        plan_id,
        &status,
        customer_id.as_deref(),
        subscription_id.as_deref(),
        period_start,
        period_end,
        cancel_at_period_end,
    )
    .await?;
    Ok(organization_id)
}

async fn update_billing_event(
    tx: &mut Transaction<'_, Postgres>,
    event_id: &str,
    organization_id: Option<Uuid>,
    status: &str,
    error: Option<&str>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE billing_events
        SET organization_id = COALESCE($2, organization_id),
            status = $3,
            error = $4,
            processed_at = CASE WHEN $3 IN ('processed', 'ignored') THEN now() ELSE processed_at END,
            updated_at = now()
        WHERE provider = 'stripe'
          AND provider_event_id = $1
        "#,
    )
    .bind(event_id)
    .bind(organization_id)
    .bind(status)
    .bind(error)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn ensure_customer(
    pool: &PgPool,
    config: &Config,
    tenant: &TenantContext,
) -> Result<String, AppError> {
    quota::ensure_default_subscription(pool, tenant).await?;
    if let Some(customer_id) = load_customer_id(pool, tenant).await? {
        return Ok(customer_id);
    }

    let profile = load_billing_profile(pool, tenant).await?;
    let mut params = vec![
        ("name".to_owned(), profile.organization_name),
        (
            "metadata[organization_id]".to_owned(),
            profile.organization_id.to_string(),
        ),
    ];
    if let Some(email) = profile.owner_email {
        params.push(("email".to_owned(), email));
    }
    let payload = stripe_post(config, "customers", params).await?;
    let customer_id = required_string(&payload, "id")?;

    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query(
        r#"
        UPDATE organization_subscriptions
        SET provider = 'stripe',
            provider_customer_id = $2,
            updated_at = now()
        WHERE organization_id = $1
        "#,
    )
    .bind(tenant.organization_id)
    .bind(&customer_id)
    .execute(db.as_mut())
    .await?;

    Ok(customer_id)
}

async fn load_customer_id(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<Option<String>, AppError> {
    quota::ensure_default_subscription(pool, tenant).await?;
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query_scalar::<_, Option<String>>(
        r#"
        SELECT provider_customer_id
        FROM organization_subscriptions
        WHERE organization_id = $1
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_billing_profile(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<OrganizationBillingProfile, AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query_as::<_, OrganizationBillingProfile>(
        r#"
        SELECT organization.id as organization_id,
               organization.name as organization_name,
               owner.email::text as owner_email
        FROM organizations organization
        LEFT JOIN users owner ON owner.id = organization.owner_id
        WHERE organization.id = $1
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_plan_by_slug(pool: &PgPool, slug: &str) -> Result<quota::PlanLimits, AppError> {
    sqlx::query_as::<_, quota::PlanLimits>(
        r#"
        SELECT id,
               slug,
               name,
               description,
               price_monthly_cents,
               member_limit,
               content_limit,
               media_limit_mb,
               api_requests_limit,
               features,
               stripe_price_id
        FROM plans
        WHERE slug = $1
          AND is_active = true
        "#,
    )
    .bind(slug.trim().to_ascii_lowercase())
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

async fn stripe_post(
    config: &Config,
    endpoint: &str,
    params: Vec<(String, String)>,
) -> Result<Value, AppError> {
    let secret_key = config.stripe_secret_key.as_ref().ok_or_else(|| {
        AppError::ServiceUnavailable("Stripe secret key is not configured".to_owned())
    })?;
    let response = Client::new()
        .post(format!("{STRIPE_API_BASE}/{endpoint}"))
        .bearer_auth(secret_key)
        .form(&params)
        .send()
        .await
        .map_err(|error| AppError::ServiceUnavailable(format!("Stripe request failed: {error}")))?;
    let status = response.status();
    let payload = response.json::<Value>().await.map_err(|error| {
        AppError::ServiceUnavailable(format!("Stripe response was invalid: {error}"))
    })?;

    if !status.is_success() {
        let message = payload
            .get("error")
            .and_then(|error| error.get("message"))
            .and_then(Value::as_str)
            .unwrap_or("Stripe request failed");
        return Err(AppError::ServiceUnavailable(format!(
            "Stripe error: {message}"
        )));
    }

    Ok(payload)
}

fn verify_signature(config: &Config, signature: &str, body: &[u8]) -> Result<(), AppError> {
    let webhook_secret = config.stripe_webhook_secret.as_ref().ok_or_else(|| {
        AppError::ServiceUnavailable("Stripe webhook secret is not configured".to_owned())
    })?;
    let timestamp = stripe_signature_part(signature, "t").ok_or_else(|| {
        AppError::Unauthorized("Stripe signature timestamp is missing".to_owned())
    })?;
    let timestamp_seconds = timestamp
        .parse::<i64>()
        .map_err(|_| AppError::Unauthorized("Stripe signature timestamp is invalid".to_owned()))?;
    let age = Utc::now()
        .timestamp()
        .saturating_sub(timestamp_seconds)
        .abs();
    if age > STRIPE_SIGNATURE_TOLERANCE_SECONDS {
        return Err(AppError::Unauthorized(
            "Stripe signature timestamp is outside the tolerance window".to_owned(),
        ));
    }

    let mut signed_payload = Vec::with_capacity(timestamp.len() + 1 + body.len());
    signed_payload.extend_from_slice(timestamp.as_bytes());
    signed_payload.push(b'.');
    signed_payload.extend_from_slice(body);

    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
        .map_err(|_| AppError::Internal("invalid Stripe webhook secret".to_owned()))?;
    mac.update(&signed_payload);
    let expected = hex_encode(&mac.finalize().into_bytes());

    let valid = signature
        .split(',')
        .filter_map(|part| part.trim().strip_prefix("v1="))
        .any(|provided| secure_eq(provided.as_bytes(), expected.as_bytes()));
    if !valid {
        return Err(AppError::Unauthorized(
            "Stripe signature is invalid".to_owned(),
        ));
    }
    Ok(())
}

fn stripe_signature_part<'a>(signature: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    signature
        .split(',')
        .find_map(|part| part.trim().strip_prefix(&prefix))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn secure_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right.iter())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}

fn required_string(payload: &Value, field: &str) -> Result<String, AppError> {
    payload
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| AppError::ServiceUnavailable(format!("Stripe response missing {field}")))
}

fn event_object(payload: &Value) -> Result<&Value, AppError> {
    payload
        .get("data")
        .and_then(|data| data.get("object"))
        .ok_or_else(|| AppError::Validation("Stripe event data.object is missing".to_owned()))
}

fn object_string(object: &Value, field: &str) -> Option<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn metadata_string(object: &Value, key: &str) -> Option<String> {
    object
        .get("metadata")
        .and_then(|metadata| metadata.get(key))
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
}

fn organization_id_from_object(object: &Value) -> Result<Uuid, AppError> {
    let raw = object_string(object, "client_reference_id")
        .or_else(|| metadata_string(object, "organization_id"))
        .ok_or_else(|| {
            AppError::Validation("Stripe organization metadata is missing".to_owned())
        })?;
    Uuid::parse_str(&raw)
        .map_err(|_| AppError::Validation("Stripe organization metadata is invalid".to_owned()))
}

fn subscription_price_id(object: &Value) -> Option<String> {
    object
        .get("items")
        .and_then(|items| items.get("data"))
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .and_then(|item| item.get("price"))
        .and_then(|price| price.get("id"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn unix_time_field(object: &Value, field: &str) -> Option<DateTime<Utc>> {
    object
        .get(field)
        .and_then(Value::as_i64)
        .and_then(|seconds| DateTime::from_timestamp(seconds, 0))
}

fn stripe_status_to_internal(status: Option<&str>) -> String {
    match status.unwrap_or("incomplete") {
        "trialing" => "trialing",
        "active" => "active",
        "past_due" | "unpaid" => "past_due",
        "canceled" => "canceled",
        _ => "incomplete",
    }
    .to_owned()
}

async fn plan_id_by_slug(
    tx: &mut Transaction<'_, Postgres>,
    slug: &str,
) -> Result<Option<Uuid>, AppError> {
    sqlx::query_scalar::<_, Uuid>("SELECT id FROM plans WHERE slug = $1 AND is_active = true")
        .bind(slug)
        .fetch_optional(&mut **tx)
        .await
        .map_err(AppError::from)
}

async fn plan_id_for_subscription(
    tx: &mut Transaction<'_, Postgres>,
    config: &Config,
    price_id: Option<&str>,
    plan_slug: Option<&str>,
) -> Result<Option<Uuid>, AppError> {
    if let Some(slug) = plan_slug {
        if let Some(plan_id) = plan_id_by_slug(tx, slug).await? {
            return Ok(Some(plan_id));
        }
    }
    if let Some(price_id) = price_id {
        if let Some(plan_id) = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM plans WHERE stripe_price_id = $1 AND is_active = true",
        )
        .bind(price_id)
        .fetch_optional(&mut **tx)
        .await?
        {
            return Ok(Some(plan_id));
        }
        let slug = if config.stripe_pro_price_id.as_deref() == Some(price_id) {
            Some("pro")
        } else if config.stripe_enterprise_price_id.as_deref() == Some(price_id) {
            Some("enterprise")
        } else {
            None
        };
        if let Some(slug) = slug {
            return plan_id_by_slug(tx, slug).await;
        }
    }
    Ok(None)
}

async fn current_subscription_plan_id(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
) -> Result<Uuid, AppError> {
    sqlx::query_scalar::<_, Uuid>(
        "SELECT plan_id FROM organization_subscriptions WHERE organization_id = $1",
    )
    .bind(organization_id)
    .fetch_one(&mut **tx)
    .await
    .map_err(AppError::from)
}

async fn organization_id_by_provider_ids(
    tx: &mut Transaction<'_, Postgres>,
    subscription_id: Option<&str>,
    customer_id: Option<&str>,
) -> Result<Option<Uuid>, AppError> {
    sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT organization_id
        FROM organization_subscriptions
        WHERE ($1::text IS NOT NULL AND provider_subscription_id = $1)
           OR ($2::text IS NOT NULL AND provider_customer_id = $2)
        ORDER BY updated_at DESC
        LIMIT 1
        "#,
    )
    .bind(subscription_id)
    .bind(customer_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(AppError::from)
}

async fn upsert_subscription(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    plan_id: Uuid,
    status: &str,
    customer_id: Option<&str>,
    subscription_id: Option<&str>,
    period_start: Option<DateTime<Utc>>,
    period_end: Option<DateTime<Utc>>,
    cancel_at_period_end: bool,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO organization_subscriptions (
          organization_id,
          plan_id,
          status,
          provider,
          provider_customer_id,
          provider_subscription_id,
          current_period_start,
          current_period_end,
          cancel_at_period_end
        )
        VALUES (
          $1,
          $2,
          $3::organization_subscription_status,
          'stripe',
          $4,
          $5,
          COALESCE($6, date_trunc('month', now())),
          COALESCE($7, date_trunc('month', now()) + interval '1 month'),
          $8
        )
        ON CONFLICT (organization_id) DO UPDATE
        SET plan_id = EXCLUDED.plan_id,
            status = EXCLUDED.status,
            provider = 'stripe',
            provider_customer_id = COALESCE(EXCLUDED.provider_customer_id, organization_subscriptions.provider_customer_id),
            provider_subscription_id = COALESCE(EXCLUDED.provider_subscription_id, organization_subscriptions.provider_subscription_id),
            current_period_start = COALESCE(EXCLUDED.current_period_start, organization_subscriptions.current_period_start),
            current_period_end = COALESCE(EXCLUDED.current_period_end, organization_subscriptions.current_period_end),
            cancel_at_period_end = EXCLUDED.cancel_at_period_end,
            updated_at = now()
        "#,
    )
    .bind(organization_id)
    .bind(plan_id)
    .bind(status)
    .bind(customer_id)
    .bind(subscription_id)
    .bind(period_start)
    .bind(period_end)
    .bind(cancel_at_period_end)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_status_maps_supported_values() {
        assert_eq!(stripe_status_to_internal(Some("active")), "active");
        assert_eq!(stripe_status_to_internal(Some("trialing")), "trialing");
        assert_eq!(stripe_status_to_internal(Some("past_due")), "past_due");
        assert_eq!(stripe_status_to_internal(Some("unpaid")), "past_due");
        assert_eq!(stripe_status_to_internal(Some("canceled")), "canceled");
        assert_eq!(
            stripe_status_to_internal(Some("incomplete_expired")),
            "incomplete"
        );
    }

    #[test]
    fn signature_verification_accepts_valid_payload() {
        let payload = br#"{"id":"evt_test","type":"checkout.session.completed"}"#;
        let timestamp = Utc::now().timestamp().to_string();
        let secret = "whsec_test_secret";
        let mut signed = Vec::new();
        signed.extend_from_slice(timestamp.as_bytes());
        signed.push(b'.');
        signed.extend_from_slice(payload);
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(&signed);
        let signature = format!(
            "t={},v1={}",
            timestamp,
            hex_encode(&mac.finalize().into_bytes())
        );
        let config = Config::test_with_stripe_secret(secret);

        assert!(verify_signature(&config, &signature, payload).is_ok());
    }
}
