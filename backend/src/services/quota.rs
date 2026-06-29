use chrono::{DateTime, NaiveDate, Utc};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::rls;

const FREE_PLAN_SLUG: &str = "free";
const METRIC_MEMBERS: &str = "members";
const METRIC_CONTENT_RECORDS: &str = "content_records";
const METRIC_MEDIA_BYTES: &str = "media_bytes";
const METRIC_API_REQUESTS: &str = "api_requests";
const BYTES_PER_MIB: i64 = 1_048_576;

#[derive(Debug, Clone, FromRow)]
pub struct PlanLimits {
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
    pub stripe_price_id: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct SubscriptionSummary {
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

#[derive(Debug, Clone)]
pub struct UsageMetric {
    pub metric: &'static str,
    pub used: i64,
    pub limit: i64,
    pub remaining: Option<i64>,
    pub percent: Option<f64>,
    pub near_limit: bool,
    pub exceeded: bool,
}

#[derive(Debug, Clone)]
pub struct UsageSummary {
    pub period_start: NaiveDate,
    pub plan: PlanLimits,
    pub subscription: SubscriptionSummary,
    pub members: UsageMetric,
    pub content_records: UsageMetric,
    pub media_bytes: UsageMetric,
    pub api_requests: UsageMetric,
}

#[derive(Debug, FromRow)]
struct UsageRaw {
    period_start: NaiveDate,
    members: i64,
    content_records: i64,
    media_bytes: i64,
    api_requests: i64,
}

pub async fn list_plans(pool: &PgPool) -> Result<Vec<PlanLimits>, AppError> {
    sqlx::query_as::<_, PlanLimits>(
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
        WHERE is_active = true
        ORDER BY sort_order ASC, price_monthly_cents ASC, name ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

pub async fn load_current_plan(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<PlanLimits, AppError> {
    ensure_default_subscription(pool, tenant).await?;
    let mut db = rls::tenant_connection(pool, tenant).await?;
    if let Some(plan) = sqlx::query_as::<_, PlanLimits>(
        r#"
        SELECT p.id,
               p.slug,
               p.name,
               p.description,
               p.price_monthly_cents,
               p.member_limit,
               p.content_limit,
               p.media_limit_mb,
               p.api_requests_limit,
               p.features,
               p.stripe_price_id
        FROM organization_subscriptions subscription
        JOIN plans p ON p.id = subscription.plan_id
        WHERE subscription.organization_id = $1
          AND subscription.status IN (
            'trialing'::organization_subscription_status,
            'active'::organization_subscription_status,
            'past_due'::organization_subscription_status
          )
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_optional(db.as_mut())
    .await?
    {
        return Ok(plan);
    }

    sqlx::query_as::<_, PlanLimits>(
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
    .bind(FREE_PLAN_SLUG)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

pub async fn load_subscription(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<SubscriptionSummary, AppError> {
    ensure_default_subscription(pool, tenant).await?;
    load_subscription_row(pool, tenant).await
}

pub async fn change_plan(
    pool: &PgPool,
    tenant: &TenantContext,
    plan_slug: &str,
) -> Result<SubscriptionSummary, AppError> {
    let plan_slug = plan_slug.trim().to_ascii_lowercase();
    if plan_slug.is_empty() {
        return Err(AppError::Validation("plan slug is required".to_owned()));
    }

    let mut db = rls::tenant_connection(pool, tenant).await?;
    let plan_id =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM plans WHERE slug = $1 AND is_active = true")
            .bind(&plan_slug)
            .fetch_optional(db.as_mut())
            .await?
            .ok_or_else(|| AppError::NotFound("plan not found".to_owned()))?;

    sqlx::query(
        r#"
        INSERT INTO organization_subscriptions (
          organization_id,
          plan_id,
          status,
          provider,
          current_period_start,
          current_period_end
        )
        VALUES (
          $1,
          $2,
          'active'::organization_subscription_status,
          'manual',
          date_trunc('month', now()),
          date_trunc('month', now()) + interval '1 month'
        )
        ON CONFLICT (organization_id) DO UPDATE
        SET plan_id = EXCLUDED.plan_id,
            status = 'active'::organization_subscription_status,
            provider = 'manual',
            current_period_start = date_trunc('month', now()),
            current_period_end = date_trunc('month', now()) + interval '1 month',
            cancel_at_period_end = false,
            updated_at = now()
        "#,
    )
    .bind(tenant.organization_id)
    .bind(plan_id)
    .execute(db.as_mut())
    .await?;

    load_subscription_row(pool, tenant).await
}

pub async fn ensure_default_subscription(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<(), AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query(
        r#"
        INSERT INTO organization_subscriptions (organization_id, plan_id, status, provider)
        SELECT $1, id, 'active'::organization_subscription_status, 'manual'
        FROM plans
        WHERE slug = $2
        ON CONFLICT (organization_id) DO NOTHING
        "#,
    )
    .bind(tenant.organization_id)
    .bind(FREE_PLAN_SLUG)
    .execute(db.as_mut())
    .await?;
    Ok(())
}

pub async fn ensure_default_subscription_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        SELECT set_config('zinhar.organization_id', $1, true),
               set_config('zinhar.user_id', $2, true),
               set_config('zinhar.rls_bypass', 'false', true)
        "#,
    )
    .bind(organization_id.to_string())
    .bind(user_id.to_string())
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO organization_subscriptions (organization_id, plan_id, status, provider)
        SELECT $1, id, 'active'::organization_subscription_status, 'manual'
        FROM plans
        WHERE slug = $2
        ON CONFLICT (organization_id) DO NOTHING
        "#,
    )
    .bind(organization_id)
    .bind(FREE_PLAN_SLUG)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn ensure_content_capacity(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<(), AppError> {
    let plan = load_current_plan(pool, tenant).await?;
    if plan.content_limit < 0 {
        return Ok(());
    }
    let raw = load_usage_raw(pool, tenant).await?;
    if raw.content_records >= i64::from(plan.content_limit) {
        return Err(AppError::Validation(format!(
            "content quota exceeded for the {} plan",
            plan.name
        )));
    }
    Ok(())
}

pub async fn ensure_media_capacity(
    pool: &PgPool,
    tenant: &TenantContext,
    additional_bytes: i64,
) -> Result<(), AppError> {
    let plan = load_current_plan(pool, tenant).await?;
    if plan.media_limit_mb < 0 {
        return Ok(());
    }
    let raw = load_usage_raw(pool, tenant).await?;
    let limit_bytes = i64::from(plan.media_limit_mb).saturating_mul(BYTES_PER_MIB);
    if raw.media_bytes.saturating_add(additional_bytes) > limit_bytes {
        return Err(AppError::Validation(format!(
            "media storage quota exceeded for the {} plan",
            plan.name
        )));
    }
    Ok(())
}

pub async fn ensure_member_capacity(
    pool: &PgPool,
    tenant: &TenantContext,
    include_pending_invites: bool,
) -> Result<(), AppError> {
    ensure_member_capacity_for_org(
        pool,
        tenant.organization_id,
        tenant.user_id,
        include_pending_invites,
    )
    .await
}

pub async fn ensure_member_capacity_for_org(
    pool: &PgPool,
    organization_id: Uuid,
    user_id: Uuid,
    include_pending_invites: bool,
) -> Result<(), AppError> {
    let tenant = TenantContext {
        organization_id,
        organization_slug: String::new(),
        organization_name: String::new(),
        role: String::new(),
        user_id,
    };
    let plan = load_current_plan(pool, &tenant).await?;
    if plan.member_limit < 0 {
        return Ok(());
    }

    let mut db = rls::tenant_connection(pool, &tenant).await?;
    let active_members: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM organization_members
        WHERE organization_id = $1
          AND status = 'active'::organization_member_status
        "#,
    )
    .bind(organization_id)
    .fetch_one(db.as_mut())
    .await?;

    let pending_invites = if include_pending_invites {
        sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM organization_invitations
            WHERE organization_id = $1
              AND status = 'pending'::organization_invitation_status
              AND expires_at > now()
            "#,
        )
        .bind(organization_id)
        .fetch_one(db.as_mut())
        .await?
    } else {
        0
    };

    if active_members.saturating_add(pending_invites) >= i64::from(plan.member_limit) {
        return Err(AppError::Validation(format!(
            "member quota exceeded for the {} plan",
            plan.name
        )));
    }

    Ok(())
}

pub async fn check_and_record_api_request(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<(), AppError> {
    let plan = load_current_plan(pool, tenant).await?;
    let current = current_api_usage(pool, tenant).await?;
    if plan.api_requests_limit >= 0 && current >= i64::from(plan.api_requests_limit) {
        return Err(AppError::TooManyRequests(format!(
            "API request quota exceeded for the {} plan",
            plan.name
        )));
    }

    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query(
        r#"
        INSERT INTO usage_counters (organization_id, period_start, metric, value)
        VALUES ($1, date_trunc('month', now())::date, $2, 1)
        ON CONFLICT (organization_id, period_start, metric) DO UPDATE
        SET value = usage_counters.value + 1,
            updated_at = now()
        "#,
    )
    .bind(tenant.organization_id)
    .bind(METRIC_API_REQUESTS)
    .execute(db.as_mut())
    .await?;
    Ok(())
}

pub async fn usage_summary(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<UsageSummary, AppError> {
    let subscription = load_subscription(pool, tenant).await?;
    let plan = load_current_plan(pool, tenant).await?;
    rebuild_usage_counters(pool, tenant).await?;
    let raw = load_usage_raw(pool, tenant).await?;

    Ok(UsageSummary {
        period_start: raw.period_start,
        members: usage_metric(METRIC_MEMBERS, raw.members, i64::from(plan.member_limit)),
        content_records: usage_metric(
            METRIC_CONTENT_RECORDS,
            raw.content_records,
            i64::from(plan.content_limit),
        ),
        media_bytes: usage_metric(
            METRIC_MEDIA_BYTES,
            raw.media_bytes,
            i64::from(plan.media_limit_mb).saturating_mul(BYTES_PER_MIB),
        ),
        api_requests: usage_metric(
            METRIC_API_REQUESTS,
            raw.api_requests,
            i64::from(plan.api_requests_limit),
        ),
        plan,
        subscription,
    })
}

pub async fn rebuild_usage_counters(pool: &PgPool, tenant: &TenantContext) -> Result<(), AppError> {
    let raw = calculate_rebuildable_usage(pool, tenant).await?;
    upsert_rebuildable_counter(pool, tenant, METRIC_MEMBERS, raw.members).await?;
    upsert_rebuildable_counter(pool, tenant, METRIC_CONTENT_RECORDS, raw.content_records).await?;
    upsert_rebuildable_counter(pool, tenant, METRIC_MEDIA_BYTES, raw.media_bytes).await?;
    Ok(())
}

async fn load_subscription_row(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<SubscriptionSummary, AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query_as::<_, SubscriptionSummary>(
        r#"
        SELECT subscription.organization_id,
               subscription.plan_id,
               plan.slug as plan_slug,
               plan.name as plan_name,
               subscription.status::text as status,
               subscription.provider,
               subscription.current_period_start,
               subscription.current_period_end,
               subscription.cancel_at_period_end
        FROM organization_subscriptions subscription
        JOIN plans plan ON plan.id = subscription.plan_id
        WHERE subscription.organization_id = $1
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn calculate_rebuildable_usage(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<UsageRaw, AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query_as::<_, UsageRaw>(
        r#"
        SELECT date_trunc('month', now())::date as period_start,
               (
                 SELECT COUNT(*)
                 FROM organization_members
                 WHERE organization_id = $1
                   AND status = 'active'::organization_member_status
               ) as members,
               (
                 SELECT
                   (SELECT COUNT(*) FROM content_entries WHERE organization_id = $1)
                   +
                   (SELECT COUNT(*) FROM pages WHERE organization_id = $1)
               ) as content_records,
               (
                 SELECT COALESCE(SUM(size), 0)::BIGINT
                 FROM media
                 WHERE organization_id = $1
               ) as media_bytes,
               COALESCE((
                 SELECT value
                 FROM usage_counters
                 WHERE organization_id = $1
                   AND period_start = date_trunc('month', now())::date
                   AND metric = $2
               ), 0) as api_requests
        "#,
    )
    .bind(tenant.organization_id)
    .bind(METRIC_API_REQUESTS)
    .fetch_one(db.as_mut())
    .await
    .map_err(AppError::from)
}

async fn load_usage_raw(pool: &PgPool, tenant: &TenantContext) -> Result<UsageRaw, AppError> {
    let rebuilt = calculate_rebuildable_usage(pool, tenant).await?;
    let api_requests = current_api_usage(pool, tenant).await?;
    Ok(UsageRaw {
        api_requests,
        ..rebuilt
    })
}

async fn current_api_usage(pool: &PgPool, tenant: &TenantContext) -> Result<i64, AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COALESCE(value, 0)
        FROM usage_counters
        WHERE organization_id = $1
          AND period_start = date_trunc('month', now())::date
          AND metric = $2
        "#,
    )
    .bind(tenant.organization_id)
    .bind(METRIC_API_REQUESTS)
    .fetch_optional(db.as_mut())
    .await
    .map(|value| value.unwrap_or(0))
    .map_err(AppError::from)
}

async fn upsert_rebuildable_counter(
    pool: &PgPool,
    tenant: &TenantContext,
    metric: &str,
    value: i64,
) -> Result<(), AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query(
        r#"
        INSERT INTO usage_counters (organization_id, period_start, metric, value, rebuilt_at)
        VALUES ($1, date_trunc('month', now())::date, $2, $3, now())
        ON CONFLICT (organization_id, period_start, metric) DO UPDATE
        SET value = EXCLUDED.value,
            rebuilt_at = now(),
            updated_at = now()
        "#,
    )
    .bind(tenant.organization_id)
    .bind(metric)
    .bind(value)
    .execute(db.as_mut())
    .await?;
    Ok(())
}

fn usage_metric(metric: &'static str, used: i64, limit: i64) -> UsageMetric {
    if limit < 0 {
        return UsageMetric {
            metric,
            used,
            limit,
            remaining: None,
            percent: None,
            near_limit: false,
            exceeded: false,
        };
    }

    let remaining = limit.saturating_sub(used);
    let percent = if limit == 0 {
        Some(100.0)
    } else {
        Some(((used as f64 / limit as f64) * 100.0).min(100.0))
    };
    UsageMetric {
        metric,
        used,
        limit,
        remaining: Some(remaining),
        percent,
        near_limit: percent.is_some_and(|value| value >= 80.0),
        exceeded: used >= limit,
    }
}
