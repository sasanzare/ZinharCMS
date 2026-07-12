use axum::extract::{Extension, Path, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::{marketplace_analytics, rbac, rls};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/marketplace/creators/{creator_id}/analytics",
            get(get_creator_analytics),
        )
        .route("/api/marketplace/analytics/admin", get(get_admin_analytics))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceCreatorAnalyticsResponse {
    pub creator_id: Uuid,
    pub listing_count: i64,
    pub total_installs: i64,
    pub active_installs: i64,
    pub purchase_attempts: i64,
    pub completed_purchases: i64,
    pub refunded_purchases: i64,
    pub gross_revenue_cents: i64,
    pub creator_revenue_cents: i64,
    pub conversion_rate: f64,
    pub error_count: i64,
    pub products: Vec<MarketplaceCreatorProductAnalyticsResponse>,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct MarketplaceCreatorProductAnalyticsResponse {
    pub listing_id: Uuid,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub product_type: String,
    pub pricing_type: String,
    pub total_installs: i64,
    pub active_installs: i64,
    pub purchase_attempts: i64,
    pub completed_purchases: i64,
    pub refunded_purchases: i64,
    pub gross_revenue_cents: i64,
    pub creator_revenue_cents: i64,
    pub conversion_rate: f64,
    pub error_count: i64,
    pub report_count: i64,
    pub average_rating: f64,
    pub rating_count: i64,
    pub last_activity_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceAdminAnalyticsResponse {
    pub generated_at: DateTime<Utc>,
    pub submission_count_30d: i64,
    pub submission_rate_per_day: f64,
    pub average_approval_hours: f64,
    pub total_installs: i64,
    pub active_installs: i64,
    pub refund_count: i64,
    pub report_count: i64,
    pub critical_report_count: i64,
    pub blocked_package_count: i64,
    pub risky_products: Vec<MarketplaceAdminRiskProductResponse>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceAdminRiskProductResponse {
    pub listing_id: Uuid,
    pub title: String,
    pub slug: String,
    pub creator_display_name: String,
    pub status: String,
    pub product_type: String,
    pub security_risk_level: String,
    pub report_count: i64,
    pub critical_report_count: i64,
    pub blocked_package_count: i64,
    pub refund_count: i64,
    pub error_count: i64,
    pub active_installs: i64,
}

#[derive(Debug, FromRow)]
struct MarketplaceCreatorProductAnalyticsRow {
    listing_id: Uuid,
    title: String,
    slug: String,
    status: String,
    product_type: String,
    pricing_type: String,
    total_installs: i64,
    active_installs: i64,
    purchase_attempts: i64,
    completed_purchases: i64,
    refunded_purchases: i64,
    failed_purchases: i64,
    gross_revenue_cents: i64,
    creator_revenue_cents: i64,
    failed_validation_count: i64,
    report_count: i64,
    average_rating: f64,
    rating_count: i64,
    last_activity_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
struct MarketplaceAdminAnalyticsSummaryRow {
    submission_count_30d: i64,
    submission_rate_per_day: f64,
    average_approval_hours: f64,
    total_installs: i64,
    active_installs: i64,
    refund_count: i64,
    report_count: i64,
    critical_report_count: i64,
    blocked_package_count: i64,
}

impl From<MarketplaceCreatorProductAnalyticsRow> for MarketplaceCreatorProductAnalyticsResponse {
    fn from(row: MarketplaceCreatorProductAnalyticsRow) -> Self {
        Self {
            listing_id: row.listing_id,
            title: row.title,
            slug: row.slug,
            status: row.status,
            product_type: row.product_type,
            pricing_type: row.pricing_type,
            total_installs: row.total_installs,
            active_installs: row.active_installs,
            purchase_attempts: row.purchase_attempts,
            completed_purchases: row.completed_purchases,
            refunded_purchases: row.refunded_purchases,
            gross_revenue_cents: row.gross_revenue_cents,
            creator_revenue_cents: row.creator_revenue_cents,
            conversion_rate: marketplace_analytics::conversion_rate(
                row.completed_purchases,
                row.purchase_attempts,
            ),
            error_count: marketplace_analytics::error_count(
                row.failed_validation_count,
                row.failed_purchases,
                row.report_count,
            ),
            report_count: row.report_count,
            average_rating: row.average_rating,
            rating_count: row.rating_count,
            last_activity_at: row.last_activity_at,
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/marketplace/creators/{creator_id}/analytics",
    tag = "marketplace",
    params(("creator_id" = Uuid, Path, description = "Marketplace creator id")),
    responses((status = 200, description = "Creator-owned Marketplace product analytics", body = MarketplaceCreatorAnalyticsResponse))
)]
pub async fn get_creator_analytics(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(creator_id): Path<Uuid>,
) -> Result<Json<MarketplaceCreatorAnalyticsResponse>, AppError> {
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    ensure_creator_owner(&mut *tx, creator_id, claims.sub).await?;

    let products = sqlx::query_as::<_, MarketplaceCreatorProductAnalyticsRow>(
        r#"
        SELECT listing.id as listing_id,
               listing.title,
               listing.slug,
               listing.status,
               listing.product_type,
               listing.pricing_type,
               COALESCE(installs.total_installs, 0)::BIGINT as total_installs,
               COALESCE(installs.active_installs, 0)::BIGINT as active_installs,
               COALESCE(purchases.purchase_attempts, 0)::BIGINT as purchase_attempts,
               COALESCE(purchases.completed_purchases, 0)::BIGINT as completed_purchases,
               COALESCE(purchases.refunded_purchases, 0)::BIGINT as refunded_purchases,
               COALESCE(purchases.failed_purchases, 0)::BIGINT as failed_purchases,
               COALESCE(ledger.gross_revenue_cents, 0)::BIGINT as gross_revenue_cents,
               COALESCE(ledger.creator_revenue_cents, 0)::BIGINT as creator_revenue_cents,
               COALESCE(version_errors.failed_validation_count, 0)::BIGINT as failed_validation_count,
               COALESCE(reports.report_count, 0)::BIGINT as report_count,
               COALESCE(ratings.average_rating, 0)::DOUBLE PRECISION as average_rating,
               COALESCE(ratings.rating_count, 0)::BIGINT as rating_count,
               GREATEST(
                   listing.updated_at,
                   COALESCE(installs.last_activity_at, listing.updated_at),
                   COALESCE(purchases.last_activity_at, listing.updated_at),
                   COALESCE(reports.last_activity_at, listing.updated_at)
               ) as last_activity_at
        FROM marketplace_listings listing
        LEFT JOIN LATERAL (
            SELECT COUNT(*)::BIGINT as total_installs,
                   COUNT(*) FILTER (WHERE installation.status = 'active')::BIGINT as active_installs,
                   MAX(installation.updated_at) as last_activity_at
            FROM marketplace_installations installation
            WHERE installation.listing_id = listing.id
        ) installs ON true
        LEFT JOIN LATERAL (
            SELECT COUNT(*)::BIGINT as purchase_attempts,
                   COUNT(*) FILTER (WHERE purchase.status = 'completed')::BIGINT as completed_purchases,
                   COUNT(*) FILTER (WHERE purchase.status = 'refunded')::BIGINT as refunded_purchases,
                   COUNT(*) FILTER (WHERE purchase.status = 'failed')::BIGINT as failed_purchases,
                   MAX(purchase.updated_at) as last_activity_at
            FROM marketplace_purchases purchase
            WHERE purchase.listing_id = listing.id
        ) purchases ON true
        LEFT JOIN LATERAL (
            SELECT COALESCE(SUM(entry.gross_cents + entry.adjustment_cents), 0)::BIGINT as gross_revenue_cents,
                   COALESCE(SUM(entry.creator_share_cents + entry.adjustment_cents), 0)::BIGINT as creator_revenue_cents
            FROM marketplace_revenue_ledger entry
            WHERE entry.listing_id = listing.id
              AND entry.creator_id = listing.creator_id
        ) ledger ON true
        LEFT JOIN LATERAL (
            SELECT COUNT(*)::BIGINT as failed_validation_count
            FROM marketplace_versions version
            WHERE version.listing_id = listing.id
              AND (
                  version.status = 'blocked'
                  OR version.validation_status = 'failed'
                  OR version.security_risk_level IN ('high', 'critical')
              )
        ) version_errors ON true
        LEFT JOIN LATERAL (
            SELECT COUNT(*)::BIGINT as report_count,
                   MAX(report.updated_at) as last_activity_at
            FROM marketplace_abuse_reports report
            WHERE report.listing_id = listing.id
        ) reports ON true
        LEFT JOIN LATERAL (
            SELECT AVG(review.rating)::DOUBLE PRECISION as average_rating,
                   COUNT(*)::BIGINT as rating_count
            FROM marketplace_product_reviews review
            WHERE review.listing_id = listing.id
              AND review.status = 'published'
        ) ratings ON true
        WHERE listing.creator_id = $1
        ORDER BY failed_validation_count DESC, active_installs DESC, listing.updated_at DESC
        "#,
    )
    .bind(creator_id)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    let products: Vec<MarketplaceCreatorProductAnalyticsResponse> =
        products.into_iter().map(Into::into).collect();
    let purchase_attempts = products
        .iter()
        .map(|product| product.purchase_attempts)
        .sum();
    let completed_purchases = products
        .iter()
        .map(|product| product.completed_purchases)
        .sum();

    Ok(Json(MarketplaceCreatorAnalyticsResponse {
        creator_id,
        listing_count: products.len() as i64,
        total_installs: products.iter().map(|product| product.total_installs).sum(),
        active_installs: products.iter().map(|product| product.active_installs).sum(),
        purchase_attempts,
        completed_purchases,
        refunded_purchases: products
            .iter()
            .map(|product| product.refunded_purchases)
            .sum(),
        gross_revenue_cents: products
            .iter()
            .map(|product| product.gross_revenue_cents)
            .sum(),
        creator_revenue_cents: products
            .iter()
            .map(|product| product.creator_revenue_cents)
            .sum(),
        conversion_rate: marketplace_analytics::conversion_rate(
            completed_purchases,
            purchase_attempts,
        ),
        error_count: products.iter().map(|product| product.error_count).sum(),
        products,
    }))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/analytics/admin",
    tag = "marketplace",
    responses((status = 200, description = "Internal Marketplace health and risk analytics", body = MarketplaceAdminAnalyticsResponse))
)]
pub async fn get_admin_analytics(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MarketplaceAdminAnalyticsResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;

    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let summary = sqlx::query_as::<_, MarketplaceAdminAnalyticsSummaryRow>(
        r#"
        SELECT
            (SELECT COUNT(*)::BIGINT
             FROM marketplace_submissions submission
             WHERE submission.created_at >= now() - interval '30 days') as submission_count_30d,
            ((SELECT COUNT(*)::DOUBLE PRECISION
              FROM marketplace_submissions submission
              WHERE submission.created_at >= now() - interval '30 days') / 30.0)::DOUBLE PRECISION as submission_rate_per_day,
            COALESCE((
                SELECT AVG(EXTRACT(EPOCH FROM (event.created_at - submission.created_at)) / 3600.0)::DOUBLE PRECISION
                FROM marketplace_review_events event
                JOIN marketplace_submissions submission ON submission.id = event.submission_id
                WHERE event.action = 'approve'
                  AND event.created_at >= now() - interval '30 days'
            ), 0)::DOUBLE PRECISION as average_approval_hours,
            (SELECT COUNT(*)::BIGINT FROM marketplace_installations) as total_installs,
            (SELECT COUNT(*)::BIGINT FROM marketplace_installations WHERE status = 'active') as active_installs,
            (SELECT COUNT(*)::BIGINT FROM marketplace_purchases WHERE status = 'refunded') as refund_count,
            (SELECT COUNT(*)::BIGINT FROM marketplace_abuse_reports) as report_count,
            (SELECT COUNT(*)::BIGINT FROM marketplace_abuse_reports WHERE severity = 'critical') as critical_report_count,
            (SELECT COUNT(DISTINCT version.id)::BIGINT
             FROM marketplace_versions version
             WHERE version.status = 'blocked'
                OR version.validation_status = 'failed'
                OR version.security_risk_level IN ('high', 'critical')) as blocked_package_count
        "#,
    )
    .fetch_one(&mut *tx)
    .await?;

    let risky_products = sqlx::query_as::<_, MarketplaceAdminRiskProductResponse>(
        r#"
        SELECT *
        FROM (
            SELECT listing.id as listing_id,
                   listing.title,
                   listing.slug,
                   creator.display_name as creator_display_name,
                   listing.status,
                   listing.product_type,
                   COALESCE(risk.security_risk_level, 'unreviewed') as security_risk_level,
                   COALESCE(reports.report_count, 0)::BIGINT as report_count,
                   COALESCE(reports.critical_report_count, 0)::BIGINT as critical_report_count,
                   COALESCE(version_errors.blocked_package_count, 0)::BIGINT as blocked_package_count,
                   COALESCE(purchases.refund_count, 0)::BIGINT as refund_count,
                   (
                       COALESCE(version_errors.blocked_package_count, 0)
                       + COALESCE(reports.report_count, 0)
                       + COALESCE(purchases.refund_count, 0)
                       + COALESCE(purchases.failed_purchase_count, 0)
                   )::BIGINT as error_count,
                   COALESCE(installs.active_installs, 0)::BIGINT as active_installs
            FROM marketplace_listings listing
            JOIN marketplace_creators creator ON creator.id = listing.creator_id
            LEFT JOIN LATERAL (
                SELECT version.security_risk_level
                FROM marketplace_versions version
                WHERE version.listing_id = listing.id
                ORDER BY CASE version.security_risk_level
                    WHEN 'critical' THEN 4
                    WHEN 'high' THEN 3
                    WHEN 'medium' THEN 2
                    WHEN 'low' THEN 1
                    ELSE 0
                END DESC,
                version.updated_at DESC
                LIMIT 1
            ) risk ON true
            LEFT JOIN LATERAL (
                SELECT COUNT(*)::BIGINT as report_count,
                       COUNT(*) FILTER (WHERE report.severity = 'critical')::BIGINT as critical_report_count
                FROM marketplace_abuse_reports report
                WHERE report.listing_id = listing.id
            ) reports ON true
            LEFT JOIN LATERAL (
                SELECT COUNT(*)::BIGINT as blocked_package_count
                FROM marketplace_versions version
                WHERE version.listing_id = listing.id
                  AND (
                      version.status = 'blocked'
                      OR version.validation_status = 'failed'
                      OR version.security_risk_level IN ('high', 'critical')
                  )
            ) version_errors ON true
            LEFT JOIN LATERAL (
                SELECT COUNT(*) FILTER (WHERE purchase.status = 'refunded')::BIGINT as refund_count,
                       COUNT(*) FILTER (WHERE purchase.status = 'failed')::BIGINT as failed_purchase_count
                FROM marketplace_purchases purchase
                WHERE purchase.listing_id = listing.id
            ) purchases ON true
            LEFT JOIN LATERAL (
                SELECT COUNT(*) FILTER (WHERE installation.status = 'active')::BIGINT as active_installs
                FROM marketplace_installations installation
                WHERE installation.listing_id = listing.id
            ) installs ON true
        ) ranked
        WHERE error_count > 0
           OR security_risk_level IN ('high', 'critical')
           OR critical_report_count > 0
           OR blocked_package_count > 0
           OR refund_count > 0
        ORDER BY critical_report_count DESC,
                 blocked_package_count DESC,
                 error_count DESC,
                 refund_count DESC,
                 active_installs DESC
        LIMIT 10
        "#,
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(MarketplaceAdminAnalyticsResponse {
        generated_at: Utc::now(),
        submission_count_30d: summary.submission_count_30d,
        submission_rate_per_day: summary.submission_rate_per_day,
        average_approval_hours: summary.average_approval_hours,
        total_installs: summary.total_installs,
        active_installs: summary.active_installs,
        refund_count: summary.refund_count,
        report_count: summary.report_count,
        critical_report_count: summary.critical_report_count,
        blocked_package_count: summary.blocked_package_count,
        risky_products,
    }))
}

async fn ensure_creator_owner(
    db: &mut sqlx::PgConnection,
    creator_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let owner: Option<Uuid> =
        sqlx::query_scalar("SELECT user_id FROM marketplace_creators WHERE id = $1")
            .bind(creator_id)
            .fetch_optional(db)
            .await?;

    if owner == Some(user_id) {
        Ok(())
    } else {
        Err(AppError::Forbidden(
            "only the creator owner can view analytics for this profile".to_owned(),
        ))
    }
}
