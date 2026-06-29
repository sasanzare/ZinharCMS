use axum::body::Bytes;
use axum::extract::{Extension, State};
use axum::http::HeaderMap;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::{audit, email, quota, rbac, stripe_billing};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/billing/plans", get(list_plans))
        .route("/api/billing/subscription", get(get_subscription))
        .route("/api/billing/subscription", put(change_subscription_plan))
        .route("/api/billing/checkout", post(create_checkout_session))
        .route("/api/billing/portal", post(create_customer_portal_session))
        .route("/api/billing/usage", get(get_usage))
        .route("/api/billing/usage/rebuild", post(rebuild_usage))
}

pub fn public_router() -> Router<AppState> {
    Router::new().route("/api/billing/stripe/webhook", post(stripe_webhook))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePlanRequest {
    pub plan_slug: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckoutSessionRequest {
    pub plan_slug: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckoutSessionResponse {
    pub session_id: String,
    pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CustomerPortalResponse {
    pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BillingWebhookResponse {
    pub event_id: String,
    pub event_type: String,
    pub status: String,
    pub already_processed: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlanResponse {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub price_monthly_cents: i32,
    pub member_limit: i32,
    pub content_limit: i32,
    pub media_limit_mb: i32,
    pub api_requests_limit: i32,
    pub stripe_checkout_available: bool,
    pub features: Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionResponse {
    pub organization_id: Uuid,
    pub plan_id: Uuid,
    pub plan_slug: String,
    pub plan_name: String,
    pub status: String,
    pub provider: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at_period_end: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UsageMetricResponse {
    pub metric: String,
    pub used: i64,
    pub limit: i64,
    pub remaining: Option<i64>,
    pub percent: Option<f64>,
    pub near_limit: bool,
    pub exceeded: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BillingUsageResponse {
    pub period_start: NaiveDate,
    pub plan: PlanResponse,
    pub subscription: SubscriptionResponse,
    pub members: UsageMetricResponse,
    pub content_records: UsageMetricResponse,
    pub media_bytes: UsageMetricResponse,
    pub api_requests: UsageMetricResponse,
}

#[utoipa::path(
    get,
    path = "/api/billing/plans",
    tag = "billing",
    responses((status = 200, description = "Available billing plans", body = [PlanResponse]))
)]
pub async fn list_plans(
    State(state): State<AppState>,
) -> Result<Json<Vec<PlanResponse>>, AppError> {
    let plans = quota::list_plans(&state.db)
        .await?
        .into_iter()
        .map(|plan| PlanResponse::from_plan(plan, &state.config))
        .collect();
    Ok(Json(plans))
}

#[utoipa::path(
    get,
    path = "/api/billing/subscription",
    tag = "billing",
    responses((status = 200, description = "Current organization subscription", body = SubscriptionResponse))
)]
pub async fn get_subscription(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    Ok(Json(
        quota::load_subscription(&state.db, &tenant).await?.into(),
    ))
}

#[utoipa::path(
    put,
    path = "/api/billing/subscription",
    tag = "billing",
    request_body = ChangePlanRequest,
    responses((status = 200, description = "Changed organization plan", body = SubscriptionResponse))
)]
pub async fn change_subscription_plan(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<ChangePlanRequest>,
) -> Result<Json<SubscriptionResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_BILLING_MANAGER])?;
    let subscription: SubscriptionResponse =
        quota::change_plan(&state.db, &tenant, &payload.plan_slug)
            .await?
            .into();
    audit::record(
        &state.db,
        &tenant,
        "billing.subscription.change",
        "organization_subscription",
        None,
        serde_json::json!({
            "plan_slug": &subscription.plan_slug,
            "plan_name": &subscription.plan_name,
            "status": &subscription.status,
            "provider": &subscription.provider,
        }),
    )
    .await?;
    if let Some(recipient) = load_actor_email(&state, &tenant).await? {
        email::send_billing_notification(
            &state.db,
            &state.config,
            &tenant,
            &recipient,
            &subscription.plan_name,
            &subscription.status,
        )
        .await?;
    }
    Ok(Json(subscription))
}

#[utoipa::path(
    post,
    path = "/api/billing/checkout",
    tag = "billing",
    request_body = CheckoutSessionRequest,
    responses((status = 200, description = "Created Stripe checkout session", body = CheckoutSessionResponse))
)]
pub async fn create_checkout_session(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<CheckoutSessionRequest>,
) -> Result<Json<CheckoutSessionResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_BILLING_MANAGER])?;
    let session: CheckoutSessionResponse = stripe_billing::create_checkout_session(
        &state.db,
        &state.config,
        &tenant,
        &payload.plan_slug,
    )
    .await?
    .into();
    audit::record(
        &state.db,
        &tenant,
        "billing.checkout.create",
        "stripe_checkout_session",
        None,
        serde_json::json!({ "plan_slug": payload.plan_slug, "session_id": &session.session_id }),
    )
    .await?;
    Ok(Json(session))
}

#[utoipa::path(
    post,
    path = "/api/billing/portal",
    tag = "billing",
    responses((status = 200, description = "Created Stripe customer portal session", body = CustomerPortalResponse))
)]
pub async fn create_customer_portal_session(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<CustomerPortalResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_BILLING_MANAGER])?;
    let session: CustomerPortalResponse =
        stripe_billing::create_customer_portal_session(&state.db, &state.config, &tenant)
            .await?
            .into();
    audit::record(
        &state.db,
        &tenant,
        "billing.portal.create",
        "stripe_customer_portal_session",
        None,
        serde_json::json!({ "url": &session.url }),
    )
    .await?;
    Ok(Json(session))
}

#[utoipa::path(
    post,
    path = "/api/billing/stripe/webhook",
    tag = "billing",
    request_body(content = String, content_type = "application/json"),
    responses((status = 200, description = "Processed Stripe webhook", body = BillingWebhookResponse))
)]
pub async fn stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<BillingWebhookResponse>, AppError> {
    let signature = headers
        .get("stripe-signature")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Stripe signature header is missing".to_owned()))?;
    Ok(Json(
        stripe_billing::handle_webhook(&state.db, &state.config, signature, &body)
            .await?
            .into(),
    ))
}

#[utoipa::path(
    get,
    path = "/api/billing/usage",
    tag = "billing",
    responses((status = 200, description = "Current organization usage and quota", body = BillingUsageResponse))
)]
pub async fn get_usage(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<BillingUsageResponse>, AppError> {
    Ok(Json(BillingUsageResponse::from_summary(
        quota::usage_summary(&state.db, &tenant).await?,
        &state.config,
    )))
}

#[utoipa::path(
    post,
    path = "/api/billing/usage/rebuild",
    tag = "billing",
    responses((status = 200, description = "Rebuilt current organization usage counters", body = BillingUsageResponse))
)]
pub async fn rebuild_usage(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<BillingUsageResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_BILLING_MANAGER])?;
    quota::rebuild_usage_counters(&state.db, &tenant).await?;
    audit::record(
        &state.db,
        &tenant,
        "billing.usage.rebuild",
        "usage_counters",
        None,
        serde_json::json!({ "period": "current" }),
    )
    .await?;
    Ok(Json(BillingUsageResponse::from_summary(
        quota::usage_summary(&state.db, &tenant).await?,
        &state.config,
    )))
}

async fn load_actor_email(
    state: &AppState,
    tenant: &TenantContext,
) -> Result<Option<String>, AppError> {
    sqlx::query_scalar::<_, String>("SELECT email::text FROM users WHERE id = $1")
        .bind(tenant.user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(AppError::from)
}
impl PlanResponse {
    fn from_plan(plan: quota::PlanLimits, config: &Config) -> Self {
        let stripe_checkout_available = stripe_billing::price_id_for_plan(config, &plan).is_some();
        Self {
            id: plan.id,
            slug: plan.slug,
            name: plan.name,
            description: plan.description,
            price_monthly_cents: plan.price_monthly_cents,
            member_limit: plan.member_limit,
            content_limit: plan.content_limit,
            media_limit_mb: plan.media_limit_mb,
            api_requests_limit: plan.api_requests_limit,
            stripe_checkout_available,
            features: plan.features,
        }
    }
}

impl From<quota::SubscriptionSummary> for SubscriptionResponse {
    fn from(subscription: quota::SubscriptionSummary) -> Self {
        Self {
            organization_id: subscription.organization_id,
            plan_id: subscription.plan_id,
            plan_slug: subscription.plan_slug,
            plan_name: subscription.plan_name,
            status: subscription.status,
            provider: subscription.provider,
            current_period_start: subscription.current_period_start,
            current_period_end: subscription.current_period_end,
            cancel_at_period_end: subscription.cancel_at_period_end,
        }
    }
}

impl From<quota::UsageMetric> for UsageMetricResponse {
    fn from(metric: quota::UsageMetric) -> Self {
        Self {
            metric: metric.metric.to_owned(),
            used: metric.used,
            limit: metric.limit,
            remaining: metric.remaining,
            percent: metric.percent,
            near_limit: metric.near_limit,
            exceeded: metric.exceeded,
        }
    }
}

impl BillingUsageResponse {
    fn from_summary(summary: quota::UsageSummary, config: &Config) -> Self {
        Self {
            period_start: summary.period_start,
            plan: PlanResponse::from_plan(summary.plan, config),
            subscription: summary.subscription.into(),
            members: summary.members.into(),
            content_records: summary.content_records.into(),
            media_bytes: summary.media_bytes.into(),
            api_requests: summary.api_requests.into(),
        }
    }
}

impl From<stripe_billing::CheckoutSession> for CheckoutSessionResponse {
    fn from(session: stripe_billing::CheckoutSession) -> Self {
        Self {
            session_id: session.session_id,
            url: session.url,
        }
    }
}

impl From<stripe_billing::CustomerPortalSession> for CustomerPortalResponse {
    fn from(session: stripe_billing::CustomerPortalSession) -> Self {
        Self { url: session.url }
    }
}

impl From<stripe_billing::WebhookResult> for BillingWebhookResponse {
    fn from(result: stripe_billing::WebhookResult) -> Self {
        Self {
            event_id: result.event_id,
            event_type: result.event_type,
            status: result.status,
            already_processed: result.already_processed,
        }
    }
}
