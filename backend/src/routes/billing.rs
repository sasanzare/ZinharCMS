use axum::extract::{Extension, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::{quota, rbac};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/billing/plans", get(list_plans))
        .route("/api/billing/subscription", get(get_subscription))
        .route("/api/billing/subscription", put(change_subscription_plan))
        .route("/api/billing/usage", get(get_usage))
        .route("/api/billing/usage/rebuild", post(rebuild_usage))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChangePlanRequest {
    pub plan_slug: String,
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
        .map(PlanResponse::from)
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
    Ok(Json(
        quota::change_plan(&state.db, &tenant, &payload.plan_slug)
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
    Ok(Json(quota::usage_summary(&state.db, &tenant).await?.into()))
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
    Ok(Json(quota::usage_summary(&state.db, &tenant).await?.into()))
}

impl From<quota::PlanLimits> for PlanResponse {
    fn from(plan: quota::PlanLimits) -> Self {
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

impl From<quota::UsageSummary> for BillingUsageResponse {
    fn from(summary: quota::UsageSummary) -> Self {
        Self {
            period_start: summary.period_start,
            plan: summary.plan.into(),
            subscription: summary.subscription.into(),
            members: summary.members.into(),
            content_records: summary.content_records.into(),
            media_bytes: summary.media_bytes.into(),
            api_requests: summary.api_requests.into(),
        }
    }
}
