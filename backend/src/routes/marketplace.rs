use std::cmp::Ordering;
use std::path::PathBuf;

use axum::extract::{Extension, Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, patch, post, put};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{FromRow, PgConnection, Postgres, Transaction};
use tokio::fs;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::routes::marketplace_runtime;
use crate::services::marketplace_catalog::{catalog_compatibility_report, is_catalog_compatible};
use crate::services::marketplace_feedback::{
    REPORT_DESCRIPTION_MAX, REVIEW_BODY_MAX, validate_rating, validate_report_type,
    validate_severity, validate_text,
};
use crate::services::marketplace_installation::{
    CLEANUP_POLICY_PRESERVE, LifecycleAction, approved_permission_snapshot,
    canonicalize_permission_value, compare_semver, permission_reapproval_required,
    permissions_are_subset, permissions_from_manifest, validate_lifecycle_action,
    validate_mvp_product_type, validate_newer_version, verify_stored_artifact,
};
use crate::services::marketplace_manifest::MARKETPLACE_MANIFEST_SCHEMA_VERSION;
use crate::services::marketplace_package::{
    marketplace_package_object_key, sha256_hex, validate_package_size,
};
use crate::services::marketplace_review::{
    MODERATION_EMERGENCY_BLOCK, MODERATION_SUSPEND_LISTING, MODERATION_UNPUBLISH_VERSION,
    validate_moderation_action, validate_review_decision,
};
use crate::services::marketplace_submission::{
    ListingReviewInput, normalize_optional_text, sanitize_screenshot_urls,
    validate_creator_profile, validate_creator_verification_status, validate_listing_for_review,
    validate_listing_review_input,
};
use crate::services::marketplace_validation::evaluate_marketplace_package;
use crate::services::{audit, quota, rbac, rls};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/marketplace/catalog", get(list_catalog))
        .route(
            "/api/marketplace/catalog/{listing_slug}",
            get(get_catalog_listing),
        )
        .route(
            "/api/marketplace/listings/{listing_id}/reviews",
            get(list_product_reviews).post(create_product_review),
        )
        .route(
            "/api/marketplace/reviews/{review_id}/moderation",
            patch(moderate_product_review),
        )
        .route("/api/marketplace/reviews", get(list_product_review_queue))
        .route(
            "/api/marketplace/listings/{listing_id}/reports",
            post(create_abuse_report),
        )
        .route("/api/marketplace/reports", get(list_abuse_reports))
        .route(
            "/api/marketplace/reports/{report_id}",
            patch(resolve_abuse_report),
        )
        .route(
            "/api/marketplace/installations",
            get(list_installations).post(install_marketplace_product),
        )
        .route(
            "/api/marketplace/installations/{installation_id}/updates",
            get(check_installation_updates),
        )
        .route(
            "/api/marketplace/installations/{installation_id}/enable",
            post(enable_installation),
        )
        .route(
            "/api/marketplace/installations/{installation_id}/disable",
            post(disable_installation),
        )
        .route(
            "/api/marketplace/installations/{installation_id}/uninstall",
            post(uninstall_installation),
        )
        .route(
            "/api/marketplace/installations/{installation_id}/update",
            post(update_installation),
        )
        .route(
            "/api/marketplace/installations/{installation_id}/rollback",
            post(rollback_installation),
        )
        .route(
            "/api/marketplace/creator",
            get(get_creator).post(request_creator),
        )
        .route(
            "/api/marketplace/creators/{creator_id}/verification",
            patch(update_creator_verification),
        )
        .route(
            "/api/marketplace/listings",
            get(list_creator_listings).post(create_listing),
        )
        .route(
            "/api/marketplace/listings/{listing_id}",
            put(update_listing),
        )
        .route(
            "/api/marketplace/listings/{listing_id}/submit",
            post(submit_listing),
        )
        .route(
            "/api/marketplace/listings/{listing_id}/versions/upload",
            post(upload_listing_version),
        )
        .route(
            "/api/marketplace/listings/{listing_id}/submissions",
            get(list_listing_submissions),
        )
        .route("/api/marketplace/review/queue", get(list_review_queue))
        .route("/api/marketplace/review/events", get(list_review_events))
        .route("/api/marketplace/review/reports", get(list_review_reports))
        .route(
            "/api/marketplace/review/submissions/{submission_id}",
            patch(review_submission),
        )
        .route(
            "/api/marketplace/review/listings/{listing_id}/moderation",
            post(moderate_listing),
        )
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatorProfileRequest {
    pub slug: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub support_email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatorVerificationRequest {
    pub status: String,
    pub verification_notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListingRequest {
    pub product_type: String,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub description: String,
    pub category: String,
    pub pricing_type: String,
    pub price_cents: i32,
    pub license: String,
    pub support_url: Option<String>,
    pub screenshots: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatorStateResponse {
    pub creator: Option<CreatorProfileResponse>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct CreatorProfileResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub slug: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub status: String,
    pub payout_status: String,
    pub support_email: Option<String>,
    pub verification_notes: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub metadata: Value,
    pub requested_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ListingResponse {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub product_type: String,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub description: String,
    pub category: String,
    pub status: String,
    pub pricing_type: String,
    pub price_cents: i32,
    pub license: String,
    pub support_url: Option<String>,
    pub screenshots: Value,
    pub metadata: Value,
    pub submitted_by: Option<Uuid>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PackageVersionResponse {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub version: String,
    pub manifest_schema_version: String,
    pub manifest_json: Value,
    pub artifact_object_key: String,
    pub artifact_sha256: String,
    pub artifact_size_bytes: i64,
    pub artifact_file_name: String,
    pub artifact_content_type: String,
    pub storage_metadata: Value,
    pub validation_status: String,
    pub validation_report: Value,
    pub security_risk_level: String,
    pub compatibility_report: Value,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceSubmissionResponse {
    pub id: Uuid,
    pub version_id: Uuid,
    pub submitted_by: Option<Uuid>,
    pub review_status: String,
    pub risk_level: String,
    pub review_notes: Option<String>,
    pub validation_report: Value,
    pub metadata: Value,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VersionSubmissionResponse {
    pub version: PackageVersionResponse,
    pub submission: MarketplaceSubmissionResponse,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceValidationReportResponse {
    pub listing_id: Uuid,
    pub listing_title: String,
    pub listing_slug: String,
    pub creator_id: Uuid,
    pub creator_display_name: String,
    pub version_id: Uuid,
    pub version: String,
    pub version_status: String,
    pub validation_status: String,
    pub security_risk_level: String,
    pub validation_report: Value,
    pub compatibility_report: Value,
    pub submission_id: Uuid,
    pub review_status: String,
    pub risk_level: String,
    pub review_notes: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Default, ToSchema)]
pub struct CatalogQuery {
    pub search: Option<String>,
    pub category: Option<String>,
    pub product_type: Option<String>,
    pub pricing_type: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceCatalogItemResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub category: String,
    pub product_type: String,
    pub pricing_type: String,
    pub price_cents: i32,
    pub creator_display_name: String,
    pub latest_version_id: Uuid,
    pub latest_version: String,
    pub badge: String,
    pub rating_average: f64,
    pub rating_count: i64,
    pub active_installations: i64,
    pub compatibility_report: Value,
    pub permissions: Value,
    pub screenshots: Value,
    pub support_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceCatalogVersionResponse {
    pub id: Uuid,
    pub version: String,
    pub compatibility_report: Value,
    pub permissions: Value,
    pub changelog: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceCatalogReviewResponse {
    pub author: String,
    pub rating: i32,
    pub body: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceProductReviewRequest {
    pub version_id: Option<Uuid>,
    pub rating: i32,
    pub body: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceProductReviewModerationRequest {
    pub status: String,
    pub moderation_reason: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceProductReviewResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub listing_id: Uuid,
    pub version_id: Option<Uuid>,
    pub author_id: Uuid,
    pub author: String,
    pub rating: i16,
    pub body: String,
    pub status: String,
    pub moderation_reason: Option<String>,
    pub moderated_by: Option<Uuid>,
    pub moderated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceProductReviewListResponse {
    pub id: Uuid,
    pub author: String,
    pub rating: i16,
    pub body: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceAbuseReportRequest {
    pub version_id: Option<Uuid>,
    pub report_type: String,
    pub severity: String,
    pub description: String,
    #[serde(default = "empty_json_object")]
    pub evidence: Value,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceAbuseReportResolutionRequest {
    pub status: String,
    pub resolution_note: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceAbuseReportResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub listing_id: Uuid,
    pub version_id: Option<Uuid>,
    pub reporter_id: Uuid,
    pub report_type: String,
    pub severity: String,
    pub description: String,
    pub evidence: Value,
    pub status: String,
    pub resolution_note: Option<String>,
    pub notification_status: String,
    pub critical_notified_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceCatalogDetailResponse {
    pub item: MarketplaceCatalogItemResponse,
    pub description: String,
    pub license: String,
    pub support_url: Option<String>,
    pub screenshots: Value,
    pub permissions: Value,
    pub changelog: Value,
    pub versions: Vec<MarketplaceCatalogVersionResponse>,
    pub reviews: Vec<MarketplaceCatalogReviewResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceInstallRequest {
    pub listing_id: Uuid,
    pub version_id: Uuid,
    pub approved_permissions: Vec<String>,
    pub purchase_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MarketplaceInstallationUpdateRequest {
    pub version_id: Uuid,
    pub changelog_confirmed: bool,
    pub approved_permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceInstallationResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub listing_id: Uuid,
    pub listing_title: String,
    pub listing_slug: String,
    pub product_type: String,
    pub pricing_type: String,
    pub version_id: Uuid,
    pub installed_version: String,
    pub status: String,
    pub permissions: Value,
    pub permission_approved_by: Option<Uuid>,
    pub permission_approved_at: Option<DateTime<Utc>>,
    pub rollback_version_id: Option<Uuid>,
    pub rollback_version: Option<String>,
    pub cleanup_policy: String,
    pub version_pinned: bool,
    pub installed_by: Option<Uuid>,
    pub installed_at: DateTime<Utc>,
    pub enabled_at: DateTime<Utc>,
    pub disabled_at: Option<DateTime<Utc>>,
    pub uninstalled_at: Option<DateTime<Utc>>,
    pub version_changed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MarketplaceInstallationUpdateResponse {
    pub installation_id: Uuid,
    pub current_version_id: Uuid,
    pub current_version: String,
    pub current_status: String,
    pub version_pinned: bool,
    pub update_available: bool,
    pub target_version_id: Option<Uuid>,
    pub target_version: Option<String>,
    pub changelog: Value,
    pub permissions: Value,
    pub permission_reapproval_required: bool,
    pub compatibility_report: Value,
    pub reasons: Vec<String>,
}

#[derive(Debug, FromRow)]
struct MarketplaceInstallationDbRow {
    id: Uuid,
    organization_id: Uuid,
    listing_id: Uuid,
    listing_title: String,
    listing_slug: String,
    product_type: String,
    pricing_type: String,
    version_id: Uuid,
    installed_version: String,
    status: String,
    permissions: Value,
    permission_approved_by: Option<Uuid>,
    permission_approved_at: Option<DateTime<Utc>>,
    rollback_version_id: Option<Uuid>,
    rollback_version: Option<String>,
    cleanup_policy: String,
    version_pinned: bool,
    installed_by: Option<Uuid>,
    installed_at: DateTime<Utc>,
    enabled_at: DateTime<Utc>,
    disabled_at: Option<DateTime<Utc>>,
    uninstalled_at: Option<DateTime<Utc>>,
    version_changed_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct InstallationLifecycleRow {
    listing_id: Uuid,
    version_id: Uuid,
    current_version: String,
    status: String,
    permissions_json: Value,
    permission_approved_by: Option<Uuid>,
    permission_approved_at: Option<DateTime<Utc>>,
    rollback_version_id: Option<Uuid>,
    metadata: Value,
    version_pinned: bool,
    product_type: String,
    pricing_type: String,
    listing_status: String,
    current_version_status: String,
    current_manifest_json: Value,
    current_artifact_object_key: String,
    current_artifact_sha256: String,
    current_artifact_size_bytes: i64,
    current_validation_status: String,
    current_security_risk_level: String,
}

#[derive(Debug, FromRow)]
struct MarketplaceVersionGateRow {
    version_id: Uuid,
    version: String,
    version_status: String,
    manifest_json: Value,
    artifact_object_key: String,
    artifact_sha256: String,
    artifact_size_bytes: i64,
    validation_status: String,
    security_risk_level: String,
}

#[derive(Debug, FromRow)]
struct InstallationCandidateRow {
    listing_id: Uuid,
    listing_title: String,
    listing_slug: String,
    product_type: String,
    pricing_type: String,
    listing_status: String,
    version_id: Uuid,
    version: String,
    version_status: String,
    manifest_json: Value,
    artifact_object_key: String,
    artifact_sha256: String,
    artifact_size_bytes: i64,
    validation_status: String,
    security_risk_level: String,
}

#[derive(Debug, Clone, FromRow)]
struct MarketplaceCatalogRow {
    id: Uuid,
    title: String,
    slug: String,
    summary: String,
    description: String,
    category: String,
    product_type: String,
    pricing_type: String,
    price_cents: i32,
    license: String,
    support_url: Option<String>,
    screenshots: Value,
    creator_display_name: String,
    version_id: Uuid,
    version: String,
    manifest_json: Value,
    active_installations: i64,
    rating_average: f64,
    rating_count: i64,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct MarketplaceCatalogVersionRow {
    id: Uuid,
    version: String,
    manifest_json: Value,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReviewDecisionRequest {
    pub decision: String,
    pub internal_comment: Option<String>,
    pub creator_message: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ModerationRequest {
    pub action: String,
    pub version_id: Option<Uuid>,
    pub reason: String,
    pub internal_comment: Option<String>,
    pub creator_message: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MarketplaceReviewEventResponse {
    pub id: Uuid,
    pub submission_id: Option<Uuid>,
    pub listing_id: Uuid,
    pub listing_title: String,
    pub version_id: Option<Uuid>,
    pub version: Option<String>,
    pub actor_id: Option<Uuid>,
    pub actor_email: Option<String>,
    pub action: String,
    pub previous_status: Option<String>,
    pub next_status: String,
    pub internal_comment: Option<String>,
    pub creator_message: Option<String>,
    pub reason: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct ReviewContextRow {
    submission_id: Uuid,
    listing_id: Uuid,
    version_id: Uuid,
    review_status: String,
    version_status: String,
    listing_status: String,
    validation_status: String,
    security_risk_level: String,
}

#[derive(Debug, FromRow)]
struct ListingModerationRow {
    listing_id: Uuid,
    listing_status: String,
}

#[derive(Debug, FromRow)]
struct VersionModerationRow {
    version_id: Uuid,
    version_status: String,
}

#[derive(Debug, FromRow)]
struct ApprovedVersionCountRow {
    approved_versions: i64,
}

#[derive(Debug, FromRow)]
struct InsertedReviewEventRow {
    id: Uuid,
}

#[derive(Debug, FromRow)]
struct CreatorOwnershipRow {
    id: Uuid,
}

#[derive(Debug, FromRow)]
struct ListingSubmissionRow {
    creator_id: Uuid,
    creator_slug: String,
    creator_status: String,
    product_type: String,
    title: String,
    slug: String,
    summary: String,
    description: String,
    category: String,
    pricing_type: String,
    price_cents: i32,
    license: String,
    support_url: Option<String>,
    screenshots: Value,
}

#[derive(Debug)]
struct IncomingPackageUpload {
    filename: String,
    content_type: String,
    bytes: Vec<u8>,
}

pub async fn list_catalog(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<CatalogQuery>,
) -> Result<Json<Vec<MarketplaceCatalogItemResponse>>, AppError> {
    let plan = quota::load_current_plan(&state.db, &tenant).await?;
    let search = clean_query_param(query.search);
    let category = clean_query_param(query.category);
    let product_type = clean_query_param(query.product_type);
    let pricing_type = clean_query_param(query.pricing_type);
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;

    let rows = sqlx::query_as::<_, MarketplaceCatalogRow>(
        r#"
        SELECT listing.id,
               listing.title,
               listing.slug,
               listing.summary,
               listing.description,
               listing.category,
               listing.product_type,
               listing.pricing_type,
               listing.price_cents,
               listing.license,
               listing.support_url,
               listing.screenshots,
               creator.display_name as creator_display_name,
               version.id as version_id,
               version.version,
               version.manifest_json,
               COALESCE(installs.active_installations, 0)::BIGINT as active_installations,
               COALESCE(ratings.rating_average, 0)::DOUBLE PRECISION as rating_average,
               COALESCE(ratings.rating_count, 0)::BIGINT as rating_count,
               listing.updated_at
        FROM marketplace_listings listing
        JOIN marketplace_creators creator ON creator.id = listing.creator_id
        JOIN LATERAL (
            SELECT id, version, manifest_json, created_at
            FROM marketplace_versions version
            WHERE version.listing_id = listing.id
              AND version.status = 'approved'
              AND version.validation_status IN ('passed', 'warning')
              AND version.security_risk_level IN ('low', 'medium')
            ORDER BY version.created_at DESC
            LIMIT 1
        ) version ON true
        LEFT JOIN LATERAL (
            SELECT COUNT(*)::BIGINT as active_installations
            FROM marketplace_installations installation
            WHERE installation.listing_id = listing.id
              AND installation.status = 'active'
        ) installs ON true
        LEFT JOIN LATERAL (
            SELECT AVG(review.rating)::DOUBLE PRECISION as rating_average,
                   COUNT(*)::BIGINT as rating_count
            FROM marketplace_product_reviews review
            WHERE review.listing_id = listing.id
              AND review.status = 'published'
        ) ratings ON true
        WHERE listing.status = 'approved'
          AND ($1::text IS NULL
               OR listing.title ILIKE ('%' || $1 || '%')
               OR listing.summary ILIKE ('%' || $1 || '%')
               OR listing.category ILIKE ('%' || $1 || '%')
               OR listing.slug ILIKE ('%' || $1 || '%')
               OR creator.display_name ILIKE ('%' || $1 || '%'))
          AND ($2::text IS NULL OR listing.category = $2)
          AND ($3::text IS NULL OR listing.product_type = $3)
          AND ($4::text IS NULL OR listing.pricing_type = $4)
        ORDER BY active_installations DESC, listing.updated_at DESC
        LIMIT 100
        "#,
    )
    .bind(search.as_deref())
    .bind(category.as_deref())
    .bind(product_type.as_deref())
    .bind(pricing_type.as_deref())
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;

    let items = rows
        .iter()
        .filter_map(|row| catalog_item_from_row(row, &plan.slug))
        .collect();

    Ok(Json(items))
}

pub async fn get_catalog_listing(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_slug): Path<String>,
) -> Result<Json<MarketplaceCatalogDetailResponse>, AppError> {
    let slug = listing_slug.trim().to_ascii_lowercase();
    if slug.is_empty() {
        return Err(AppError::BadRequest("listing slug is required".to_owned()));
    }

    let plan = quota::load_current_plan(&state.db, &tenant).await?;
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let row = sqlx::query_as::<_, MarketplaceCatalogRow>(
        r#"
        SELECT listing.id,
               listing.title,
               listing.slug,
               listing.summary,
               listing.description,
               listing.category,
               listing.product_type,
               listing.pricing_type,
               listing.price_cents,
               listing.license,
               listing.support_url,
               listing.screenshots,
               creator.display_name as creator_display_name,
               version.id as version_id,
               version.version,
               version.manifest_json,
               COALESCE(installs.active_installations, 0)::BIGINT as active_installations,
               COALESCE(ratings.rating_average, 0)::DOUBLE PRECISION as rating_average,
               COALESCE(ratings.rating_count, 0)::BIGINT as rating_count,
               listing.updated_at
        FROM marketplace_listings listing
        JOIN marketplace_creators creator ON creator.id = listing.creator_id
        JOIN LATERAL (
            SELECT id, version, manifest_json, created_at
            FROM marketplace_versions version
            WHERE version.listing_id = listing.id
              AND version.status = 'approved'
              AND version.validation_status IN ('passed', 'warning')
              AND version.security_risk_level IN ('low', 'medium')
            ORDER BY version.created_at DESC
            LIMIT 1
        ) version ON true
        LEFT JOIN LATERAL (
            SELECT COUNT(*)::BIGINT as active_installations
            FROM marketplace_installations installation
            WHERE installation.listing_id = listing.id
              AND installation.status = 'active'
        ) installs ON true
        LEFT JOIN LATERAL (
            SELECT AVG(review.rating)::DOUBLE PRECISION as rating_average,
                   COUNT(*)::BIGINT as rating_count
            FROM marketplace_product_reviews review
            WHERE review.listing_id = listing.id
              AND review.status = 'published'
        ) ratings ON true
        WHERE listing.status = 'approved'
          AND listing.slug = $1
        "#,
    )
    .bind(&slug)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("catalog listing not found".to_owned()))?;

    let item = catalog_item_from_row(&row, &plan.slug)
        .ok_or_else(|| AppError::NotFound("catalog listing not found".to_owned()))?;
    let versions = sqlx::query_as::<_, MarketplaceCatalogVersionRow>(
        r#"
        SELECT id, version, manifest_json, created_at
        FROM marketplace_versions
        WHERE listing_id = $1
          AND status = 'approved'
          AND validation_status IN ('passed', 'warning')
          AND security_risk_level IN ('low', 'medium')
        ORDER BY created_at DESC
        "#,
    )
    .bind(row.id)
    .fetch_all(&mut *tx)
    .await?
    .iter()
    .filter_map(|version| catalog_version_from_row(version, &plan.slug))
    .collect::<Vec<_>>();
    let reviews = sqlx::query_as::<_, MarketplaceCatalogReviewResponse>(
        r#"
        SELECT user_account.name as author, review.rating::integer as rating, review.body, review.created_at
        FROM marketplace_product_reviews review
        JOIN users user_account ON user_account.id = review.author_id
        WHERE review.listing_id = $1
          AND review.status = 'published'
        ORDER BY review.created_at DESC
        "#,
    )
    .bind(row.id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;

    Ok(Json(MarketplaceCatalogDetailResponse {
        description: row.description,
        license: row.license,
        support_url: row.support_url.clone(),
        screenshots: row.screenshots.clone(),
        permissions: item.permissions.clone(),
        changelog: manifest_value(&row.manifest_json, "changelog"),
        versions,
        reviews,
        item,
    }))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/listings/{listing_id}/reviews",
    tag = "marketplace",
    params(("listing_id" = Uuid, Path, description = "Marketplace listing id")),
    responses((status = 200, description = "Published reviews and the caller organization's review", body = [MarketplaceProductReviewListResponse]))
)]
pub async fn list_product_reviews(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_id): Path<Uuid>,
) -> Result<Json<Vec<MarketplaceProductReviewListResponse>>, AppError> {
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let reviews = sqlx::query_as::<_, MarketplaceProductReviewListResponse>(
        r#"
        SELECT review.id, user_account.name as author, review.rating, review.body,
               review.status, review.created_at, review.updated_at
        FROM marketplace_product_reviews review
        JOIN users user_account ON user_account.id = review.author_id
        WHERE review.listing_id = $1
          AND (review.status = 'published' OR review.organization_id = $2)
        ORDER BY review.created_at DESC
        "#,
    )
    .bind(listing_id)
    .bind(tenant.organization_id)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(reviews))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/reviews",
    tag = "marketplace",
    responses((status = 200, description = "Pending customer-review moderation queue", body = [MarketplaceProductReviewResponse]))
)]
pub async fn list_product_review_queue(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MarketplaceProductReviewResponse>>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let reviews = sqlx::query_as::<_, MarketplaceProductReviewResponse>(
        r#"
        SELECT review.id, review.organization_id, review.listing_id, review.version_id,
               review.author_id, user_account.name as author, review.rating, review.body,
               review.status, review.moderation_reason, review.moderated_by, review.moderated_at,
               review.created_at, review.updated_at
        FROM marketplace_product_reviews review
        JOIN users user_account ON user_account.id = review.author_id
        WHERE review.status = 'pending'
        ORDER BY review.created_at ASC
        "#,
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(reviews))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/listings/{listing_id}/reviews",
    tag = "marketplace",
    request_body = MarketplaceProductReviewRequest,
    params(("listing_id" = Uuid, Path, description = "Marketplace listing id")),
    responses((status = 201, description = "Review submitted for moderation", body = MarketplaceProductReviewResponse))
)]
pub async fn create_product_review(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_id): Path<Uuid>,
    Json(payload): Json<MarketplaceProductReviewRequest>,
) -> Result<(StatusCode, Json<MarketplaceProductReviewResponse>), AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    validate_rating(payload.rating).map_err(AppError::Validation)?;
    validate_text(&payload.body, "review body", 3, REVIEW_BODY_MAX)
        .map_err(AppError::Validation)?;

    let body = payload.body.trim();
    let mut tx = rls::begin_organization_transaction(
        &state.db,
        tenant.organization_id,
        Some(tenant.user_id),
    )
    .await?;
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM marketplace_listings WHERE id = $1)")
            .bind(listing_id)
            .fetch_one(&mut *tx)
            .await?;
    if !exists {
        return Err(AppError::NotFound(
            "Marketplace listing not found".to_owned(),
        ));
    }
    if let Some(version_id) = payload.version_id {
        let version_matches: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM marketplace_versions WHERE id = $1 AND listing_id = $2)",
        )
        .bind(version_id)
        .bind(listing_id)
        .fetch_one(&mut *tx)
        .await?;
        if !version_matches {
            return Err(AppError::Validation(
                "review version does not belong to the listing".to_owned(),
            ));
        }
    }
    let owns_product: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM marketplace_installations
            WHERE organization_id = $1 AND listing_id = $2 AND status <> 'uninstalled'
        ) OR EXISTS (
            SELECT 1 FROM marketplace_purchases
            WHERE organization_id = $1 AND listing_id = $2 AND status = 'completed'
        )
        "#,
    )
    .bind(tenant.organization_id)
    .bind(listing_id)
    .fetch_one(&mut *tx)
    .await?;
    if !owns_product {
        return Err(AppError::Forbidden(
            "only an organization that installed or purchased this product can leave a review"
                .to_owned(),
        ));
    }
    let review = sqlx::query_as::<_, MarketplaceProductReviewResponse>(
        r#"
        INSERT INTO marketplace_product_reviews (
          organization_id, listing_id, version_id, author_id, rating, body, status,
          moderation_reason, moderated_by, moderated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, 'pending', NULL, NULL, NULL)
        ON CONFLICT (organization_id, listing_id) DO UPDATE SET
          version_id = EXCLUDED.version_id,
          author_id = EXCLUDED.author_id,
          rating = EXCLUDED.rating,
          body = EXCLUDED.body,
          status = 'pending',
          moderation_reason = NULL,
          moderated_by = NULL,
          moderated_at = NULL,
          updated_at = now()
        RETURNING id, organization_id, listing_id, version_id, author_id,
          (SELECT name FROM users WHERE id = author_id) as author, rating, body, status,
          moderation_reason, moderated_by, moderated_at, created_at, updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(listing_id)
    .bind(payload.version_id)
    .bind(tenant.user_id)
    .bind(payload.rating as i16)
    .bind(body)
    .fetch_one(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "marketplace.customer_review.submit",
        "marketplace_product_review",
        Some(review.id),
        json!({"listing_id": listing_id, "version_id": payload.version_id, "rating": payload.rating}),
    )
    .await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(review)))
}

#[utoipa::path(
    patch,
    path = "/api/marketplace/reviews/{review_id}/moderation",
    tag = "marketplace",
    request_body = MarketplaceProductReviewModerationRequest,
    params(("review_id" = Uuid, Path, description = "Customer review id")),
    responses((status = 200, description = "Moderated customer review", body = MarketplaceProductReviewResponse))
)]
pub async fn moderate_product_review(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(review_id): Path<Uuid>,
    Json(payload): Json<MarketplaceProductReviewModerationRequest>,
) -> Result<Json<MarketplaceProductReviewResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let status = payload.status.trim();
    if !matches!(status, "published" | "rejected") {
        return Err(AppError::Validation(
            "review moderation status must be published or rejected".to_owned(),
        ));
    }
    let reason = payload
        .moderation_reason
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    if let Some(reason) = reason {
        validate_text(reason, "moderation reason", 3, REVIEW_BODY_MAX)
            .map_err(AppError::Validation)?;
    }
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let review = sqlx::query_as::<_, MarketplaceProductReviewResponse>(
        r#"
        UPDATE marketplace_product_reviews review
        SET status = $2, moderation_reason = $3, moderated_by = $4, moderated_at = now(), updated_at = now()
        WHERE review.id = $1
        RETURNING review.id, review.organization_id, review.listing_id, review.version_id,
          review.author_id, (SELECT name FROM users WHERE id = review.author_id) as author,
          review.rating, review.body, review.status, review.moderation_reason, review.moderated_by,
          review.moderated_at, review.created_at, review.updated_at
        "#,
    )
    .bind(review_id)
    .bind(status)
    .bind(reason)
    .bind(claims.sub)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("customer review not found".to_owned()))?;
    audit::record_in_transaction(
        &mut tx,
        review.organization_id,
        Some(claims.sub),
        "marketplace.customer_review.moderate",
        "marketplace_product_review",
        Some(review.id),
        json!({"status": status}),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(review))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/listings/{listing_id}/reports",
    tag = "marketplace",
    request_body = MarketplaceAbuseReportRequest,
    params(("listing_id" = Uuid, Path, description = "Marketplace listing id")),
    responses((status = 201, description = "Abuse report queued", body = MarketplaceAbuseReportResponse))
)]
pub async fn create_abuse_report(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_id): Path<Uuid>,
    Json(payload): Json<MarketplaceAbuseReportRequest>,
) -> Result<(StatusCode, Json<MarketplaceAbuseReportResponse>), AppError> {
    validate_report_type(payload.report_type.trim()).map_err(AppError::Validation)?;
    validate_severity(payload.severity.trim()).map_err(AppError::Validation)?;
    validate_text(
        &payload.description,
        "report description",
        10,
        REPORT_DESCRIPTION_MAX,
    )
    .map_err(AppError::Validation)?;
    if !payload.evidence.is_object() {
        return Err(AppError::Validation(
            "report evidence must be a JSON object".to_owned(),
        ));
    }
    let report_type = payload.report_type.trim();
    let severity = payload.severity.trim();
    let description = payload.description.trim();
    let is_critical = severity == "critical";
    let mut tx = rls::begin_organization_transaction(
        &state.db,
        tenant.organization_id,
        Some(tenant.user_id),
    )
    .await?;
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM marketplace_listings WHERE id = $1)")
            .bind(listing_id)
            .fetch_one(&mut *tx)
            .await?;
    if !exists {
        return Err(AppError::NotFound(
            "Marketplace listing not found".to_owned(),
        ));
    }
    if let Some(version_id) = payload.version_id {
        let version_matches: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM marketplace_versions WHERE id = $1 AND listing_id = $2)",
        )
        .bind(version_id)
        .bind(listing_id)
        .fetch_one(&mut *tx)
        .await?;
        if !version_matches {
            return Err(AppError::Validation(
                "report version does not belong to the listing".to_owned(),
            ));
        }
    }
    let report = sqlx::query_as::<_, MarketplaceAbuseReportResponse>(
        r#"
        INSERT INTO marketplace_abuse_reports (
          organization_id, listing_id, version_id, reporter_id, report_type, severity,
          description, evidence, notification_status, critical_notified_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8,
          CASE WHEN $6 = 'critical' THEN 'created' ELSE 'not_required' END,
          CASE WHEN $6 = 'critical' THEN now() ELSE NULL END)
        RETURNING id, organization_id, listing_id, version_id, reporter_id, report_type,
          severity, description, evidence, status, resolution_note, notification_status,
          critical_notified_at, resolved_by, resolved_at, created_at, updated_at
        "#,
    )
    .bind(tenant.organization_id)
    .bind(listing_id)
    .bind(payload.version_id)
    .bind(tenant.user_id)
    .bind(report_type)
    .bind(severity)
    .bind(description)
    .bind(&payload.evidence)
    .fetch_one(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "marketplace.abuse_report.submit",
        "marketplace_abuse_report",
        Some(report.id),
        json!({"listing_id": listing_id, "version_id": payload.version_id, "report_type": report_type, "severity": severity}),
    )
    .await?;
    if is_critical {
        sqlx::query(
            r#"
            INSERT INTO marketplace_internal_notifications (
              abuse_report_id, notification_type, recipient_role, payload
            ) VALUES ($1, 'critical_abuse_report', 'admin', $2)
            ON CONFLICT (abuse_report_id) DO NOTHING
            "#,
        )
        .bind(report.id)
        .bind(json!({"listing_id": listing_id, "organization_id": tenant.organization_id, "severity": severity}))
        .execute(&mut *tx)
        .await?;
        audit::record_in_transaction(
            &mut tx,
            tenant.organization_id,
            Some(tenant.user_id),
            "marketplace.abuse_report.critical_notification",
            "marketplace_abuse_report",
            Some(report.id),
            json!({"listing_id": listing_id, "notification_status": "created"}),
        )
        .await?;
    }
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(report)))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/reports",
    tag = "marketplace",
    responses((status = 200, description = "Global abuse moderation queue", body = [MarketplaceAbuseReportResponse]))
)]
pub async fn list_abuse_reports(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MarketplaceAbuseReportResponse>>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let reports = sqlx::query_as::<_, MarketplaceAbuseReportResponse>(
        "SELECT id, organization_id, listing_id, version_id, reporter_id, report_type, severity, description, evidence, status, resolution_note, notification_status, critical_notified_at, resolved_by, resolved_at, created_at, updated_at FROM marketplace_abuse_reports WHERE status IN ('open', 'investigating') ORDER BY CASE severity WHEN 'critical' THEN 0 WHEN 'high' THEN 1 WHEN 'medium' THEN 2 ELSE 3 END, created_at ASC",
    )
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(reports))
}

#[utoipa::path(
    patch,
    path = "/api/marketplace/reports/{report_id}",
    tag = "marketplace",
    request_body = MarketplaceAbuseReportResolutionRequest,
    params(("report_id" = Uuid, Path, description = "Abuse report id")),
    responses((status = 200, description = "Updated abuse report", body = MarketplaceAbuseReportResponse))
)]
pub async fn resolve_abuse_report(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(report_id): Path<Uuid>,
    Json(payload): Json<MarketplaceAbuseReportResolutionRequest>,
) -> Result<Json<MarketplaceAbuseReportResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let status = payload.status.trim();
    if !matches!(status, "investigating" | "resolved" | "dismissed") {
        return Err(AppError::Validation(
            "report status must be investigating, resolved, or dismissed".to_owned(),
        ));
    }
    let note = payload
        .resolution_note
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    if let Some(note) = note {
        validate_text(note, "resolution note", 3, REPORT_DESCRIPTION_MAX)
            .map_err(AppError::Validation)?;
    }
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let report = sqlx::query_as::<_, MarketplaceAbuseReportResponse>(
        r#"
        UPDATE marketplace_abuse_reports
        SET status = $2,
            resolution_note = $3,
            notification_status = CASE WHEN notification_status = 'created' THEN 'acknowledged' ELSE notification_status END,
            resolved_by = CASE WHEN $2 IN ('resolved', 'dismissed') THEN $4 ELSE NULL END,
            resolved_at = CASE WHEN $2 IN ('resolved', 'dismissed') THEN now() ELSE NULL END,
            updated_at = now()
        WHERE id = $1
        RETURNING id, organization_id, listing_id, version_id, reporter_id, report_type, severity,
          description, evidence, status, resolution_note, notification_status, critical_notified_at,
          resolved_by, resolved_at, created_at, updated_at
        "#,
    )
    .bind(report_id)
    .bind(status)
    .bind(note)
    .bind(claims.sub)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("abuse report not found".to_owned()))?;
    if matches!(status, "resolved" | "dismissed") {
        sqlx::query(
            "UPDATE marketplace_internal_notifications SET status = 'acknowledged', acknowledged_at = now() WHERE abuse_report_id = $1 AND status = 'unread'",
        )
        .bind(report.id)
        .execute(&mut *tx)
        .await?;
    }
    audit::record_in_transaction(
        &mut tx,
        report.organization_id,
        Some(claims.sub),
        "marketplace.abuse_report.moderate",
        "marketplace_abuse_report",
        Some(report.id),
        json!({"status": status}),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(report))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/installations",
    tag = "marketplace",
    responses((status = 200, description = "Current organization Marketplace installations", body = [MarketplaceInstallationResponse]))
)]
pub async fn list_installations(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
) -> Result<Json<Vec<MarketplaceInstallationResponse>>, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let rows = sqlx::query_as::<_, MarketplaceInstallationDbRow>(&installation_select_sql(
        "WHERE installation.organization_id = $1 AND installation.status <> 'uninstalled'",
    ))
    .bind(tenant.organization_id)
    .fetch_all(db.as_mut())
    .await?;

    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/installations",
    tag = "marketplace",
    request_body = MarketplaceInstallRequest,
    responses((status = 201, description = "Marketplace product installed", body = MarketplaceInstallationResponse))
)]
pub async fn install_marketplace_product(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<MarketplaceInstallRequest>,
) -> Result<(StatusCode, Json<MarketplaceInstallationResponse>), AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    rbac::require_org_marketplace_permission_approver(&tenant.role)?;
    let plan = quota::load_current_plan(&state.db, &tenant).await?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    if let Some(reason) =
        marketplace_runtime::active_kill_switch_reason(&mut tx, tenant.organization_id).await?
    {
        return Err(AppError::Conflict(format!(
            "Marketplace installations are blocked by an active kill switch: {reason}"
        )));
    }
    let candidate = load_install_candidate(&mut tx, payload.listing_id, payload.version_id).await?;
    if candidate.pricing_type == "paid" {
        let purchase_id = payload.purchase_id.ok_or_else(|| {
            AppError::Conflict(
                "paid Marketplace installation requires a completed purchase entitlement"
                    .to_owned(),
            )
        })?;
        ensure_paid_entitlement(
            &mut tx,
            tenant.organization_id,
            purchase_id,
            candidate.listing_id,
            candidate.version_id,
        )
        .await?;
    }
    let compatibility_report = validate_install_gate(
        &candidate.product_type,
        &candidate.pricing_type,
        &candidate.listing_status,
        &candidate.version_status,
        &candidate.validation_status,
        &candidate.security_risk_level,
        &candidate.manifest_json,
        &plan.slug,
        candidate.pricing_type == "paid" && payload.purchase_id.is_some(),
    )?;
    let permissions =
        approved_permission_snapshot(&candidate.manifest_json, &payload.approved_permissions)
            .map_err(|error| AppError::Validation(error.message))?;
    verify_stored_artifact(
        &state.config.upload_dir,
        &candidate.artifact_object_key,
        candidate.artifact_size_bytes,
        &candidate.artifact_sha256,
    )
    .await
    .map_err(|error| AppError::Conflict(error.message))?;

    let installation_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO marketplace_installations (
          organization_id, listing_id, version_id, installed_by, status,
          permissions_json, permission_approved_by, permission_approved_at,
          cleanup_policy, version_pinned, enabled_at, version_changed_at, metadata
        )
        VALUES ($1, $2, $3, $4, 'active', $5, $4, now(), $6, TRUE, now(), now(), $7)
        RETURNING id
        "#,
    )
    .bind(tenant.organization_id)
    .bind(candidate.listing_id)
    .bind(candidate.version_id)
    .bind(tenant.user_id)
    .bind(&permissions)
    .bind(CLEANUP_POLICY_PRESERVE)
    .bind(json!({
        "phase": "v3.6",
        "install_source": "marketplace_catalog",
        "compatibility_report": compatibility_report,
    }))
    .fetch_one(&mut *tx)
    .await?;

    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "marketplace.installation.install",
        "marketplace_installation",
        Some(installation_id),
        json!({
            "listing_id": candidate.listing_id,
            "listing_slug": candidate.listing_slug,
            "listing_title": candidate.listing_title,
            "version_id": candidate.version_id,
            "version": candidate.version,
            "package_checksum": candidate.artifact_sha256,
            "permissions": permissions,
            "decision": "installed",
            "cleanup_policy": CLEANUP_POLICY_PRESERVE,
        }),
    )
    .await?;
    let installation =
        load_installation_in_transaction(&mut tx, tenant.organization_id, installation_id).await?;
    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(installation.into())))
}

#[utoipa::path(
    get,
    path = "/api/marketplace/installations/{installation_id}/updates",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Marketplace installation id")),
    responses((status = 200, description = "Compatible update check", body = MarketplaceInstallationUpdateResponse))
)]
pub async fn check_installation_updates(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
) -> Result<Json<MarketplaceInstallationUpdateResponse>, AppError> {
    let plan = quota::load_current_plan(&state.db, &tenant).await?;
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let installation =
        load_installation_on_connection(db.as_mut(), tenant.organization_id, installation_id)
            .await?;

    if let Err(error) = validate_lifecycle_action(&installation.status, LifecycleAction::Update) {
        return Ok(Json(MarketplaceInstallationUpdateResponse {
            installation_id,
            current_version_id: installation.version_id,
            current_version: installation.current_version,
            current_status: installation.status,
            version_pinned: installation.version_pinned,
            update_available: false,
            target_version_id: None,
            target_version: None,
            changelog: Value::Null,
            permissions: json!([]),
            permission_reapproval_required: false,
            compatibility_report: json!({}),
            reasons: vec![error.message],
        }));
    }

    let versions = sqlx::query_as::<_, MarketplaceVersionGateRow>(
        r#"
        SELECT id as version_id,
               version,
               status as version_status,
               manifest_json,
               artifact_object_key,
               artifact_sha256,
               artifact_size_bytes,
               validation_status,
               security_risk_level
        FROM marketplace_versions
        WHERE listing_id = $1
          AND status = 'approved'
          AND validation_status IN ('passed', 'warning')
          AND security_risk_level IN ('low', 'medium')
        "#,
    )
    .bind(installation.listing_id)
    .fetch_all(db.as_mut())
    .await?;

    let mut best: Option<(MarketplaceVersionGateRow, Value)> = None;
    for version in versions {
        if compare_semver(&version.version, &installation.current_version)
            != Some(Ordering::Greater)
        {
            continue;
        }
        let Ok(report) = validate_install_gate(
            &installation.product_type,
            &installation.pricing_type,
            &installation.listing_status,
            &version.version_status,
            &version.validation_status,
            &version.security_risk_level,
            &version.manifest_json,
            &plan.slug,
            false,
        ) else {
            continue;
        };
        let replace = best.as_ref().is_none_or(|(current_best, _)| {
            compare_semver(&version.version, &current_best.version) == Some(Ordering::Greater)
        });
        if replace {
            best = Some((version, report));
        }
    }

    let Some((target, compatibility_report)) = best else {
        return Ok(Json(MarketplaceInstallationUpdateResponse {
            installation_id,
            current_version_id: installation.version_id,
            current_version: installation.current_version,
            current_status: installation.status,
            version_pinned: installation.version_pinned,
            update_available: false,
            target_version_id: None,
            target_version: None,
            changelog: Value::Null,
            permissions: json!([]),
            permission_reapproval_required: false,
            compatibility_report: json!({}),
            reasons: vec!["no newer approved compatible version is available".to_owned()],
        }));
    };
    let permissions = permissions_from_manifest(&target.manifest_json)
        .map_err(|error| AppError::Validation(error.message))?;
    let reapproval =
        permission_reapproval_required(&installation.permissions_json, &target.manifest_json)
            .map_err(|error| AppError::Validation(error.message))?;

    Ok(Json(MarketplaceInstallationUpdateResponse {
        installation_id,
        current_version_id: installation.version_id,
        current_version: installation.current_version,
        current_status: installation.status,
        version_pinned: installation.version_pinned,
        update_available: true,
        target_version_id: Some(target.version_id),
        target_version: Some(target.version),
        changelog: target
            .manifest_json
            .get("changelog")
            .cloned()
            .unwrap_or(Value::Null),
        permissions: json!(permissions),
        permission_reapproval_required: reapproval,
        compatibility_report,
        reasons: Vec::new(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/installations/{installation_id}/enable",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Marketplace installation id")),
    responses((status = 200, description = "Marketplace installation enabled", body = MarketplaceInstallationResponse))
)]
pub async fn enable_installation(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
) -> Result<Json<MarketplaceInstallationResponse>, AppError> {
    change_installation_status(&state, &tenant, installation_id, LifecycleAction::Enable).await
}

#[utoipa::path(
    post,
    path = "/api/marketplace/installations/{installation_id}/disable",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Marketplace installation id")),
    responses((status = 200, description = "Marketplace installation disabled", body = MarketplaceInstallationResponse))
)]
pub async fn disable_installation(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
) -> Result<Json<MarketplaceInstallationResponse>, AppError> {
    change_installation_status(&state, &tenant, installation_id, LifecycleAction::Disable).await
}

#[utoipa::path(
    post,
    path = "/api/marketplace/installations/{installation_id}/uninstall",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Marketplace installation id")),
    responses((status = 200, description = "Marketplace installation soft-uninstalled", body = MarketplaceInstallationResponse))
)]
pub async fn uninstall_installation(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
) -> Result<Json<MarketplaceInstallationResponse>, AppError> {
    change_installation_status(&state, &tenant, installation_id, LifecycleAction::Uninstall).await
}

#[utoipa::path(
    post,
    path = "/api/marketplace/installations/{installation_id}/update",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Marketplace installation id")),
    request_body = MarketplaceInstallationUpdateRequest,
    responses((status = 200, description = "Marketplace installation updated", body = MarketplaceInstallationResponse))
)]
pub async fn update_installation(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
    Json(payload): Json<MarketplaceInstallationUpdateRequest>,
) -> Result<Json<MarketplaceInstallationResponse>, AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    if !payload.changelog_confirmed {
        return Err(AppError::Validation(
            "changelog_confirmed must be true before updating an installation".to_owned(),
        ));
    }

    let plan = quota::load_current_plan(&state.db, &tenant).await?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let installation = load_installation_lifecycle_in_transaction(
        &mut tx,
        tenant.organization_id,
        installation_id,
        true,
    )
    .await?;
    validate_lifecycle_action(&installation.status, LifecycleAction::Update)
        .map_err(|error| AppError::Conflict(error.message))?;
    if installation.pricing_type == "paid" {
        ensure_active_listing_entitlement(&mut tx, tenant.organization_id, installation.listing_id)
            .await?;
    }
    let target =
        load_version_gate_in_transaction(&mut tx, installation.listing_id, payload.version_id)
            .await?;
    let compatibility_report = validate_install_gate(
        &installation.product_type,
        &installation.pricing_type,
        &installation.listing_status,
        &target.version_status,
        &target.validation_status,
        &target.security_risk_level,
        &target.manifest_json,
        &plan.slug,
        installation.pricing_type == "paid",
    )?;
    validate_newer_version(&installation.current_version, &target.version)
        .map_err(|error| AppError::Conflict(error.message))?;

    let requires_reapproval =
        permission_reapproval_required(&installation.permissions_json, &target.manifest_json)
            .map_err(|error| AppError::Validation(error.message))?;
    let (permissions, approved_by, approved_at) = if requires_reapproval {
        rbac::require_org_marketplace_permission_approver(&tenant.role)?;
        let approved = payload.approved_permissions.as_deref().ok_or_else(|| {
            AppError::Validation(
                "approved_permissions is required when an update changes permissions".to_owned(),
            )
        })?;
        let snapshot = approved_permission_snapshot(&target.manifest_json, approved)
            .map_err(|error| AppError::Validation(error.message))?;
        (snapshot, Some(tenant.user_id), Some(Utc::now()))
    } else {
        if let Some(approved) = payload.approved_permissions.as_deref() {
            approved_permission_snapshot(&target.manifest_json, approved)
                .map_err(|error| AppError::Validation(error.message))?;
        }
        (
            json!(
                permissions_from_manifest(&target.manifest_json)
                    .map_err(|error| AppError::Validation(error.message))?
            ),
            installation.permission_approved_by.or(Some(tenant.user_id)),
            installation.permission_approved_at.or(Some(Utc::now())),
        )
    };
    verify_stored_artifact(
        &state.config.upload_dir,
        &target.artifact_object_key,
        target.artifact_size_bytes,
        &target.artifact_sha256,
    )
    .await
    .map_err(|error| AppError::Conflict(error.message))?;

    let metadata = metadata_with_rollback_snapshot(
        &installation,
        &compatibility_report,
        "update",
        target.version_id,
    );
    sqlx::query(
        r#"
        UPDATE marketplace_installations
        SET rollback_version_id = version_id,
            version_id = $3,
            permissions_json = $4,
            permission_approved_by = $5,
            permission_approved_at = $6,
            version_pinned = TRUE,
            version_changed_at = now(),
            metadata = $7,
            updated_at = now()
        WHERE id = $1 AND organization_id = $2
        "#,
    )
    .bind(installation_id)
    .bind(tenant.organization_id)
    .bind(target.version_id)
    .bind(&permissions)
    .bind(approved_by)
    .bind(approved_at)
    .bind(&metadata)
    .execute(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "marketplace.installation.update",
        "marketplace_installation",
        Some(installation_id),
        json!({
            "listing_id": installation.listing_id,
            "previous_version_id": installation.version_id,
            "previous_version": installation.current_version,
            "version_id": target.version_id,
            "version": target.version,
            "package_checksum": target.artifact_sha256,
            "permissions": permissions,
            "permission_reapproved": requires_reapproval,
            "changelog_confirmed": true,
            "decision": "updated",
        }),
    )
    .await?;
    let updated =
        load_installation_in_transaction(&mut tx, tenant.organization_id, installation_id).await?;
    tx.commit().await?;

    Ok(Json(updated.into()))
}

#[utoipa::path(
    post,
    path = "/api/marketplace/installations/{installation_id}/rollback",
    tag = "marketplace",
    params(("installation_id" = Uuid, Path, description = "Marketplace installation id")),
    responses((status = 200, description = "Marketplace installation rolled back", body = MarketplaceInstallationResponse))
)]
pub async fn rollback_installation(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(installation_id): Path<Uuid>,
) -> Result<Json<MarketplaceInstallationResponse>, AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    let plan = quota::load_current_plan(&state.db, &tenant).await?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let installation = load_installation_lifecycle_in_transaction(
        &mut tx,
        tenant.organization_id,
        installation_id,
        true,
    )
    .await?;
    validate_lifecycle_action(&installation.status, LifecycleAction::Rollback)
        .map_err(|error| AppError::Conflict(error.message))?;
    if installation.pricing_type == "paid" {
        ensure_active_listing_entitlement(&mut tx, tenant.organization_id, installation.listing_id)
            .await?;
    }
    let rollback_version_id = installation.rollback_version_id.ok_or_else(|| {
        AppError::Conflict("installation does not have a rollback version".to_owned())
    })?;
    let target =
        load_version_gate_in_transaction(&mut tx, installation.listing_id, rollback_version_id)
            .await?;
    let compatibility_report = validate_rollback_gate(
        &installation.product_type,
        &installation.pricing_type,
        &installation.listing_status,
        &target.version_status,
        &target.validation_status,
        &target.security_risk_level,
        &target.manifest_json,
        &plan.slug,
    )?;
    if target.version_id == installation.version_id {
        return Err(AppError::Conflict(
            "rollback version is already installed".to_owned(),
        ));
    }
    verify_stored_artifact(
        &state.config.upload_dir,
        &target.artifact_object_key,
        target.artifact_size_bytes,
        &target.artifact_sha256,
    )
    .await
    .map_err(|error| AppError::Conflict(error.message))?;

    let target_permissions = permissions_from_manifest(&target.manifest_json)
        .map_err(|error| AppError::Validation(error.message))?;
    let current_permissions = canonicalize_permission_value(
        &installation.permissions_json,
        "current permission snapshot",
    )
    .map_err(|error| AppError::Validation(error.message))?;
    let stored_rollback_permissions = installation
        .metadata
        .get("rollback_permissions")
        .map(|value| canonicalize_permission_value(value, "rollback permission snapshot"))
        .transpose()
        .map_err(|error| AppError::Validation(error.message))?;
    let has_stored_approval = stored_rollback_permissions
        .as_ref()
        .is_some_and(|permissions| permissions == &target_permissions);
    if !has_stored_approval && !permissions_are_subset(&target_permissions, &current_permissions) {
        return Err(AppError::Conflict(
            "rollback target requires permissions that are not in the approved rollback snapshot"
                .to_owned(),
        ));
    }
    let (approved_by, approved_at) = if has_stored_approval {
        rollback_approval_from_metadata(&installation.metadata)
            .unwrap_or((tenant.user_id, Utc::now()))
    } else {
        (
            installation
                .permission_approved_by
                .unwrap_or(tenant.user_id),
            installation.permission_approved_at.unwrap_or_else(Utc::now),
        )
    };
    let permissions = json!(target_permissions);
    let metadata = metadata_with_rollback_snapshot(
        &installation,
        &compatibility_report,
        "rollback",
        target.version_id,
    );
    sqlx::query(
        r#"
        UPDATE marketplace_installations
        SET rollback_version_id = version_id,
            version_id = $3,
            permissions_json = $4,
            permission_approved_by = $5,
            permission_approved_at = $6,
            version_pinned = TRUE,
            version_changed_at = now(),
            metadata = $7,
            updated_at = now()
        WHERE id = $1 AND organization_id = $2
        "#,
    )
    .bind(installation_id)
    .bind(tenant.organization_id)
    .bind(target.version_id)
    .bind(&permissions)
    .bind(approved_by)
    .bind(approved_at)
    .bind(&metadata)
    .execute(&mut *tx)
    .await?;
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        "marketplace.installation.rollback",
        "marketplace_installation",
        Some(installation_id),
        json!({
            "listing_id": installation.listing_id,
            "previous_version_id": installation.version_id,
            "previous_version": installation.current_version,
            "version_id": target.version_id,
            "version": target.version,
            "package_checksum": target.artifact_sha256,
            "permissions": permissions,
            "decision": "rolled_back",
        }),
    )
    .await?;
    let rolled_back =
        load_installation_in_transaction(&mut tx, tenant.organization_id, installation_id).await?;
    tx.commit().await?;

    Ok(Json(rolled_back.into()))
}

async fn change_installation_status(
    state: &AppState,
    tenant: &TenantContext,
    installation_id: Uuid,
    action: LifecycleAction,
) -> Result<Json<MarketplaceInstallationResponse>, AppError> {
    rbac::require_org_marketplace_installer(&tenant.role)?;
    let plan = if action == LifecycleAction::Enable {
        Some(quota::load_current_plan(&state.db, tenant).await?)
    } else {
        None
    };
    let mut tx = rls::begin_tenant_transaction(&state.db, tenant).await?;
    let installation = load_installation_lifecycle_in_transaction(
        &mut tx,
        tenant.organization_id,
        installation_id,
        true,
    )
    .await?;
    validate_lifecycle_action(&installation.status, action)
        .map_err(|error| AppError::Conflict(error.message))?;
    if action == LifecycleAction::Enable
        && let Some(reason) =
            marketplace_runtime::active_kill_switch_reason(&mut tx, tenant.organization_id).await?
    {
        return Err(AppError::Conflict(format!(
            "Marketplace runtime is blocked by an active kill switch: {reason}"
        )));
    }

    let mut action_metadata = json!({
        "phase": "v3.6",
        "cleanup_policy": CLEANUP_POLICY_PRESERVE,
    });
    if action == LifecycleAction::Enable {
        if installation.pricing_type == "paid" {
            ensure_active_listing_entitlement(
                &mut tx,
                tenant.organization_id,
                installation.listing_id,
            )
            .await?;
        }
        let compatibility_report = validate_rollback_gate(
            &installation.product_type,
            &installation.pricing_type,
            &installation.listing_status,
            &installation.current_version_status,
            &installation.current_validation_status,
            &installation.current_security_risk_level,
            &installation.current_manifest_json,
            &plan.expect("enable plan is loaded").slug,
        )?;
        let approved = canonicalize_permission_value(
            &installation.permissions_json,
            "current permission snapshot",
        )
        .map_err(|error| AppError::Validation(error.message))?;
        approved_permission_snapshot(&installation.current_manifest_json, &approved)
            .map_err(|error| AppError::Conflict(error.message))?;
        verify_stored_artifact(
            &state.config.upload_dir,
            &installation.current_artifact_object_key,
            installation.current_artifact_size_bytes,
            &installation.current_artifact_sha256,
        )
        .await
        .map_err(|error| AppError::Conflict(error.message))?;
        action_metadata["compatibility_report"] = compatibility_report;
    }

    let (audit_action, decision) = match action {
        LifecycleAction::Enable => {
            sqlx::query(
                r#"
                UPDATE marketplace_installations
                SET status = 'active', enabled_at = now(), disabled_at = NULL,
                    metadata = metadata || $3, updated_at = now()
                WHERE id = $1 AND organization_id = $2
                "#,
            )
            .bind(installation_id)
            .bind(tenant.organization_id)
            .bind(&action_metadata)
            .execute(&mut *tx)
            .await?;
            ("marketplace.installation.enable", "enabled")
        }
        LifecycleAction::Disable => {
            sqlx::query(
                r#"
                UPDATE marketplace_installations
                SET status = 'disabled', disabled_at = now(),
                    metadata = metadata || $3, updated_at = now()
                WHERE id = $1 AND organization_id = $2
                "#,
            )
            .bind(installation_id)
            .bind(tenant.organization_id)
            .bind(&action_metadata)
            .execute(&mut *tx)
            .await?;
            ("marketplace.installation.disable", "disabled")
        }
        LifecycleAction::Uninstall => {
            sqlx::query(
                r#"
                UPDATE marketplace_installations
                SET status = 'uninstalled', disabled_at = COALESCE(disabled_at, now()),
                    uninstalled_at = now(), cleanup_policy = $3,
                    metadata = metadata || $4, updated_at = now()
                WHERE id = $1 AND organization_id = $2
                "#,
            )
            .bind(installation_id)
            .bind(tenant.organization_id)
            .bind(CLEANUP_POLICY_PRESERVE)
            .bind(&action_metadata)
            .execute(&mut *tx)
            .await?;
            ("marketplace.installation.uninstall", "uninstalled")
        }
        LifecycleAction::Update | LifecycleAction::Rollback => {
            unreachable!("version mutations use dedicated handlers")
        }
    };
    audit::record_in_transaction(
        &mut tx,
        tenant.organization_id,
        Some(tenant.user_id),
        audit_action,
        "marketplace_installation",
        Some(installation_id),
        json!({
            "listing_id": installation.listing_id,
            "version_id": installation.version_id,
            "version": installation.current_version,
            "package_checksum": installation.current_artifact_sha256,
            "permissions": installation.permissions_json,
            "decision": decision,
            "cleanup_policy": CLEANUP_POLICY_PRESERVE,
            "organization_data_deleted": false,
        }),
    )
    .await?;
    let changed =
        load_installation_in_transaction(&mut tx, tenant.organization_id, installation_id).await?;
    tx.commit().await?;

    Ok(Json(changed.into()))
}
pub async fn get_creator(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<CreatorStateResponse>, AppError> {
    let creator = load_creator_profile(&state, claims.sub).await?;
    Ok(Json(CreatorStateResponse { creator }))
}

pub async fn request_creator(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<CreatorProfileRequest>,
) -> Result<Json<CreatorProfileResponse>, AppError> {
    ensure_active_user(&state, claims.sub).await?;
    let bio = normalize_optional_text(payload.bio);
    let support_email = normalize_optional_text(payload.support_email);
    map_validation(validate_creator_profile(
        &payload.slug,
        &payload.display_name,
        bio.as_deref(),
        support_email.as_deref(),
    ))?;

    let creator = sqlx::query_as::<_, CreatorProfileResponse>(
        r#"
        INSERT INTO marketplace_creators (user_id, slug, display_name, bio, status, support_email, metadata, requested_at)
        VALUES ($1, $2, $3, $4, 'pending', $5, $6, now())
        ON CONFLICT (user_id) DO UPDATE
        SET slug = EXCLUDED.slug,
            display_name = EXCLUDED.display_name,
            bio = EXCLUDED.bio,
            support_email = EXCLUDED.support_email,
            status = CASE WHEN marketplace_creators.status = 'rejected' THEN 'pending' ELSE marketplace_creators.status END,
            requested_at = CASE WHEN marketplace_creators.status = 'rejected' THEN now() ELSE marketplace_creators.requested_at END,
            updated_at = now()
        RETURNING id, user_id, slug, display_name, bio, status, payout_status, support_email,
                  verification_notes, verified_by, verified_at, metadata, requested_at, created_at, updated_at
        "#,
    )
    .bind(claims.sub)
    .bind(payload.slug.trim())
    .bind(payload.display_name.trim())
    .bind(bio.as_deref())
    .bind(support_email.as_deref())
    .bind(json!({ "source": "creator-portal" }))
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.creator.request",
        "marketplace_creator",
        Some(creator.id),
        json!({ "status": creator.status, "slug": creator.slug }),
    )
    .await?;

    Ok(Json(creator))
}

pub async fn update_creator_verification(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(creator_id): Path<Uuid>,
    Json(payload): Json<CreatorVerificationRequest>,
) -> Result<Json<CreatorProfileResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let status = payload.status.trim();
    if !validate_creator_verification_status(status) {
        return Err(AppError::Validation(
            "creator status must be pending, approved, suspended, or rejected".to_owned(),
        ));
    }
    let notes = normalize_optional_text(payload.verification_notes);
    let verifier = (status != "pending").then_some(claims.sub);

    let creator = sqlx::query_as::<_, CreatorProfileResponse>(
        r#"
        UPDATE marketplace_creators
        SET status = $2,
            verification_notes = $3,
            verified_by = $4,
            verified_at = CASE WHEN $4::uuid IS NULL THEN NULL ELSE now() END,
            updated_at = now()
        WHERE id = $1
        RETURNING id, user_id, slug, display_name, bio, status, payout_status, support_email,
                  verification_notes, verified_by, verified_at, metadata, requested_at, created_at, updated_at
        "#,
    )
    .bind(creator_id)
    .bind(status)
    .bind(notes.as_deref())
    .bind(verifier)
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.creator.verify",
        "marketplace_creator",
        Some(creator.id),
        json!({ "status": creator.status }),
    )
    .await?;

    Ok(Json(creator))
}
pub async fn list_creator_listings(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ListingResponse>>, AppError> {
    let Some(creator) = load_creator_for_user(&state, claims.sub).await? else {
        return Ok(Json(Vec::new()));
    };

    let sql = listing_select_sql("WHERE creator_id = $1");
    let rows = sqlx::query_as::<_, ListingResponse>(&sql)
        .bind(creator.id)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(rows))
}

pub async fn create_listing(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Json(payload): Json<ListingRequest>,
) -> Result<Json<ListingResponse>, AppError> {
    let creator = require_creator_for_user(&state, claims.sub).await?;
    let screenshots = sanitize_screenshot_urls(&payload.screenshots);
    validate_listing_payload(&payload, &screenshots)?;
    let support_url = normalize_optional_text(payload.support_url.clone());

    let listing = sqlx::query_as::<_, ListingResponse>(
        r#"
        INSERT INTO marketplace_listings (
          creator_id, product_type, title, slug, summary, description, category, status,
          pricing_type, price_cents, license, support_url, screenshots, metadata
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'draft', $8, $9, $10, $11, $12, $13)
        RETURNING id, creator_id, product_type, title, slug, summary, description, category,
                  status, pricing_type, price_cents, license, support_url, screenshots, metadata,
                  submitted_by, submitted_at, created_at, updated_at
        "#,
    )
    .bind(creator.id)
    .bind(payload.product_type.trim())
    .bind(payload.title.trim())
    .bind(payload.slug.trim())
    .bind(payload.summary.trim())
    .bind(payload.description.trim())
    .bind(payload.category.trim())
    .bind(payload.pricing_type.trim())
    .bind(payload.price_cents)
    .bind(payload.license.trim())
    .bind(support_url.as_deref())
    .bind(json!(screenshots))
    .bind(listing_metadata(&payload, &screenshots))
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.listing.create",
        "marketplace_listing",
        Some(listing.id),
        json!({ "creator_id": creator.id, "slug": listing.slug, "status": listing.status }),
    )
    .await?;

    Ok(Json(listing))
}

pub async fn update_listing(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_id): Path<Uuid>,
    Json(payload): Json<ListingRequest>,
) -> Result<Json<ListingResponse>, AppError> {
    let creator = require_creator_for_user(&state, claims.sub).await?;
    ensure_listing_editable(&state, listing_id, creator.id).await?;
    let screenshots = sanitize_screenshot_urls(&payload.screenshots);
    validate_listing_payload(&payload, &screenshots)?;
    let support_url = normalize_optional_text(payload.support_url.clone());

    let listing = sqlx::query_as::<_, ListingResponse>(
        r#"
        UPDATE marketplace_listings
        SET product_type = $3,
            title = $4,
            slug = $5,
            summary = $6,
            description = $7,
            category = $8,
            pricing_type = $9,
            price_cents = $10,
            license = $11,
            support_url = $12,
            screenshots = $13,
            metadata = $14,
            updated_at = now()
        WHERE id = $1 AND creator_id = $2
        RETURNING id, creator_id, product_type, title, slug, summary, description, category,
                  status, pricing_type, price_cents, license, support_url, screenshots, metadata,
                  submitted_by, submitted_at, created_at, updated_at
        "#,
    )
    .bind(listing_id)
    .bind(creator.id)
    .bind(payload.product_type.trim())
    .bind(payload.title.trim())
    .bind(payload.slug.trim())
    .bind(payload.summary.trim())
    .bind(payload.description.trim())
    .bind(payload.category.trim())
    .bind(payload.pricing_type.trim())
    .bind(payload.price_cents)
    .bind(payload.license.trim())
    .bind(support_url.as_deref())
    .bind(json!(screenshots))
    .bind(listing_metadata(&payload, &screenshots))
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.listing.update",
        "marketplace_listing",
        Some(listing.id),
        json!({ "creator_id": creator.id, "slug": listing.slug, "status": listing.status }),
    )
    .await?;

    Ok(Json(listing))
}

pub async fn submit_listing(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_id): Path<Uuid>,
) -> Result<Json<ListingResponse>, AppError> {
    let row = load_listing_for_submission(&state, claims.sub, listing_id).await?;
    validate_listing_row_for_review(&row)?;

    let listing = sqlx::query_as::<_, ListingResponse>(
        r#"
        UPDATE marketplace_listings
        SET status = 'submitted',
            submitted_by = $2,
            submitted_at = now(),
            updated_at = now()
        WHERE id = $1
        RETURNING id, creator_id, product_type, title, slug, summary, description, category,
                  status, pricing_type, price_cents, license, support_url, screenshots, metadata,
                  submitted_by, submitted_at, created_at, updated_at
        "#,
    )
    .bind(listing_id)
    .bind(claims.sub)
    .fetch_one(&state.db)
    .await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.listing.submit",
        "marketplace_listing",
        Some(listing.id),
        json!({ "creator_id": row.creator_id, "status": listing.status }),
    )
    .await?;

    Ok(Json(listing))
}
pub async fn upload_listing_version(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<VersionSubmissionResponse>, AppError> {
    let row = load_listing_for_submission(&state, claims.sub, listing_id).await?;
    validate_listing_row_for_review(&row)?;

    let mut manifest: Option<Value> = None;
    let mut upload: Option<IncomingPackageUpload> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|error| AppError::BadRequest(error.to_string()))?
    {
        let name = field.name().unwrap_or_default().to_owned();
        match name.as_str() {
            "manifest" | "manifest_json" => {
                let text = field
                    .text()
                    .await
                    .map_err(|error| AppError::BadRequest(error.to_string()))?;
                manifest = Some(serde_json::from_str(&text).map_err(|error| {
                    AppError::Validation(format!("manifest JSON is invalid: {error}"))
                })?);
            }
            "file" => {
                let filename = field
                    .file_name()
                    .map(sanitize_filename)
                    .unwrap_or_else(|| "marketplace-package.zip".to_owned());
                let content_type = field
                    .content_type()
                    .map(str::to_owned)
                    .unwrap_or_else(|| "application/octet-stream".to_owned());
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|error| AppError::BadRequest(error.to_string()))?;
                upload = Some(IncomingPackageUpload {
                    filename,
                    content_type,
                    bytes: bytes.to_vec(),
                });
            }
            _ => {}
        }
    }

    let manifest =
        manifest.ok_or_else(|| AppError::Validation("manifest field is required".to_owned()))?;
    if !manifest.is_object() {
        return Err(AppError::Validation(
            "manifest field must be a JSON object".to_owned(),
        ));
    }
    validate_manifest_matches_listing(&manifest, &row)?;
    let upload = upload.ok_or_else(|| AppError::Validation("file field is required".to_owned()))?;
    validate_package_size(upload.bytes.len() as u64)
        .map_err(|error| AppError::Validation(error.to_string()))?;

    let package_version = manifest
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| AppError::Validation("manifest version is required".to_owned()))?;
    let checksum = sha256_hex(&upload.bytes);
    let object_key =
        marketplace_package_object_key(&row.creator_slug, &row.slug, package_version, &checksum)
            .map_err(|error| AppError::Validation(error.to_string()))?;
    persist_package_artifact(&state, &object_key, &upload.bytes).await?;

    let organization_plan_slug = quota::load_subscription(&state.db, &tenant)
        .await?
        .plan_slug;
    let validation_decision = evaluate_marketplace_package(
        &manifest,
        &upload.bytes,
        &checksum,
        &upload.filename,
        &row.product_type,
        &organization_plan_slug,
    );
    let validation_report = validation_decision.validation_report.clone();
    let compatibility_report = validation_decision.compatibility_report.clone();
    let listing_status = if validation_decision.version_status == "blocked" {
        "blocked"
    } else {
        "submitted"
    };

    let storage_metadata = json!({
        "uploaded_by": claims.sub,
        "original_filename": upload.filename.clone(),
        "storage": "local",
        "source": "creator-portal",
        "validation_status": validation_decision.validation_status.clone(),
        "security_risk_level": validation_decision.security_risk_level.clone()
    });

    let mut tx = state.db.begin().await?;
    let version = sqlx::query_as::<_, PackageVersionResponse>(
        r#"
        INSERT INTO marketplace_versions (
          listing_id, version, manifest_schema_version, manifest_json, artifact_object_key,
          artifact_sha256, artifact_size_bytes, artifact_file_name, artifact_content_type,
          storage_metadata, validation_status, validation_report, security_risk_level,
          compatibility_report, status, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING id, listing_id, version, manifest_schema_version, manifest_json,
                  artifact_object_key, artifact_sha256, artifact_size_bytes, artifact_file_name,
                  artifact_content_type, storage_metadata, validation_status, validation_report,
                  security_risk_level, compatibility_report, status, created_by, created_at, updated_at
        "#,
    )
    .bind(listing_id)
    .bind(package_version)
    .bind(MARKETPLACE_MANIFEST_SCHEMA_VERSION)
    .bind(&manifest)
    .bind(&object_key)
    .bind(&checksum)
    .bind(upload.bytes.len() as i64)
    .bind(&upload.filename)
    .bind(&upload.content_type)
    .bind(&storage_metadata)
    .bind(&validation_decision.validation_status)
    .bind(&validation_report)
    .bind(&validation_decision.security_risk_level)
    .bind(&compatibility_report)
    .bind(&validation_decision.version_status)
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await?;

    let submission = sqlx::query_as::<_, MarketplaceSubmissionResponse>(
        r#"
        INSERT INTO marketplace_submissions (
          version_id, submitted_by, review_status, risk_level, validation_report, metadata
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, version_id, submitted_by, review_status, risk_level, review_notes,
                  validation_report, metadata, reviewed_by, reviewed_at, created_at, updated_at
        "#,
    )
    .bind(version.id)
    .bind(claims.sub)
    .bind(&validation_decision.submission_review_status)
    .bind(&validation_decision.security_risk_level)
    .bind(&validation_report)
    .bind(json!({
        "listing_id": listing_id,
        "creator_id": row.creator_id,
        "validation_status": validation_decision.validation_status.clone(),
        "compatibility": compatibility_report.clone()
    }))
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE marketplace_listings
        SET status = $2,
            submitted_by = $3,
            submitted_at = COALESCE(submitted_at, now()),
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(listing_id)
    .bind(listing_status)
    .bind(claims.sub)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.version.submit",
        "marketplace_version",
        Some(version.id),
        json!({
            "listing_id": listing_id,
            "submission_id": submission.id,
            "version": version.version.clone(),
            "checksum": checksum,
            "validation_status": version.validation_status.clone(),
            "security_risk_level": version.security_risk_level.clone(),
            "version_status": version.status.clone()
        }),
    )
    .await?;

    Ok(Json(VersionSubmissionResponse {
        version,
        submission,
    }))
}

pub async fn list_listing_submissions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(listing_id): Path<Uuid>,
) -> Result<Json<Vec<MarketplaceValidationReportResponse>>, AppError> {
    let creator = require_creator_for_user(&state, claims.sub).await?;
    let reports = sqlx::query_as::<_, MarketplaceValidationReportResponse>(
        r#"
        SELECT listing.id as listing_id,
               listing.title as listing_title,
               listing.slug as listing_slug,
               creator.id as creator_id,
               creator.display_name as creator_display_name,
               version.id as version_id,
               version.version,
               version.status as version_status,
               version.validation_status,
               version.security_risk_level,
               version.validation_report,
               version.compatibility_report,
               submission.id as submission_id,
               submission.review_status,
               submission.risk_level,
               submission.review_notes,
               submission.created_at as submitted_at,
               submission.updated_at
        FROM marketplace_submissions submission
        JOIN marketplace_versions version ON version.id = submission.version_id
        JOIN marketplace_listings listing ON listing.id = version.listing_id
        JOIN marketplace_creators creator ON creator.id = listing.creator_id
        WHERE listing.id = $1 AND listing.creator_id = $2
        ORDER BY submission.created_at DESC
        "#,
    )
    .bind(listing_id)
    .bind(creator.id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(reports))
}

pub async fn list_review_queue(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MarketplaceValidationReportResponse>>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let reports = sqlx::query_as::<_, MarketplaceValidationReportResponse>(
        r#"
        SELECT listing.id as listing_id,
               listing.title as listing_title,
               listing.slug as listing_slug,
               creator.id as creator_id,
               creator.display_name as creator_display_name,
               version.id as version_id,
               version.version,
               version.status as version_status,
               version.validation_status,
               version.security_risk_level,
               version.validation_report,
               version.compatibility_report,
               submission.id as submission_id,
               submission.review_status,
               submission.risk_level,
               submission.review_notes,
               submission.created_at as submitted_at,
               submission.updated_at
        FROM marketplace_submissions submission
        JOIN marketplace_versions version ON version.id = submission.version_id
        JOIN marketplace_listings listing ON listing.id = version.listing_id
        JOIN marketplace_creators creator ON creator.id = listing.creator_id
        WHERE submission.review_status IN ('queued', 'validating', 'blocked')
           OR listing.status = 'submitted'
        ORDER BY CASE submission.review_status
                   WHEN 'blocked' THEN 0
                   WHEN 'queued' THEN 1
                   WHEN 'validating' THEN 2
                   ELSE 3
                 END,
                 submission.created_at ASC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(reports))
}

pub async fn review_submission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(submission_id): Path<Uuid>,
    Json(payload): Json<ReviewDecisionRequest>,
) -> Result<Json<MarketplaceReviewEventResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let context = sqlx::query_as::<_, ReviewContextRow>(
        r#"
        SELECT submission.id as submission_id,
               listing.id as listing_id,
               version.id as version_id,
               submission.review_status,
               version.status as version_status,
               listing.status as listing_status,
               version.validation_status,
               version.security_risk_level
        FROM marketplace_submissions submission
        JOIN marketplace_versions version ON version.id = submission.version_id
        JOIN marketplace_listings listing ON listing.id = version.listing_id
        WHERE submission.id = $1
        "#,
    )
    .bind(submission_id)
    .fetch_one(&state.db)
    .await?;

    let transition = validate_review_decision(
        &payload.decision,
        &context.version_status,
        &context.validation_status,
        &context.security_risk_level,
    )?;
    let internal_comment = normalize_optional_text(payload.internal_comment);
    let creator_message = normalize_optional_text(payload.creator_message);
    let reason = review_reason(
        payload.decision.trim(),
        internal_comment.as_deref(),
        creator_message.as_deref(),
    );
    let decision = payload.decision.trim().to_owned();
    let metadata = json!({
        "phase": "v3.4",
        "decision": decision,
        "previous": {
            "submission_status": context.review_status,
            "version_status": context.version_status,
            "listing_status": context.listing_status
        },
        "next": {
            "submission_status": transition.submission_status,
            "version_status": transition.version_status,
            "listing_status": transition.listing_status
        }
    });

    let mut tx = state.db.begin().await?;
    sqlx::query(
        r#"
        UPDATE marketplace_submissions
        SET review_status = $2,
            review_notes = $3,
            reviewed_by = $4,
            reviewed_at = now(),
            metadata = metadata || $5::jsonb,
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(context.submission_id)
    .bind(transition.submission_status)
    .bind(creator_message.as_deref())
    .bind(claims.sub)
    .bind(&metadata)
    .execute(&mut *tx)
    .await?;

    sqlx::query("UPDATE marketplace_versions SET status = $2, updated_at = now() WHERE id = $1")
        .bind(context.version_id)
        .bind(transition.version_status)
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE marketplace_listings SET status = $2, updated_at = now() WHERE id = $1")
        .bind(context.listing_id)
        .bind(transition.listing_status)
        .execute(&mut *tx)
        .await?;

    let event_id = sqlx::query_as::<_, InsertedReviewEventRow>(
        r#"
        INSERT INTO marketplace_review_events (
          submission_id, listing_id, version_id, actor_id, action, previous_status, next_status,
          internal_comment, creator_message, reason, metadata
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id
        "#,
    )
    .bind(context.submission_id)
    .bind(context.listing_id)
    .bind(context.version_id)
    .bind(claims.sub)
    .bind(&decision)
    .bind(&context.review_status)
    .bind(transition.submission_status)
    .bind(internal_comment.as_deref())
    .bind(creator_message.as_deref())
    .bind(&reason)
    .bind(&metadata)
    .fetch_one(&mut *tx)
    .await?
    .id;

    tx.commit().await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.review.decision",
        "marketplace_submission",
        Some(context.submission_id),
        json!({
            "listing_id": context.listing_id,
            "version_id": context.version_id,
            "event_id": event_id,
            "decision": decision,
            "next_status": transition.submission_status
        }),
    )
    .await?;

    Ok(Json(load_review_event(&state, event_id).await?))
}

pub async fn moderate_listing(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    Path(listing_id): Path<Uuid>,
    Json(payload): Json<ModerationRequest>,
) -> Result<Json<MarketplaceReviewEventResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    validate_moderation_action(
        &payload.action,
        &payload.reason,
        payload.version_id.is_some(),
    )?;
    let reason = payload.reason.trim().to_owned();
    let internal_comment = normalize_optional_text(payload.internal_comment);
    let creator_message = normalize_optional_text(payload.creator_message);
    let action = payload.action.trim().to_owned();
    let listing = sqlx::query_as::<_, ListingModerationRow>(
        r#"
        SELECT id as listing_id, status as listing_status
        FROM marketplace_listings
        WHERE id = $1
        "#,
    )
    .bind(listing_id)
    .fetch_one(&state.db)
    .await?;

    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let (event_version_id, previous_status, next_status) = match action.as_str() {
        MODERATION_SUSPEND_LISTING => {
            let metadata = moderation_metadata(
                &action,
                &reason,
                internal_comment.as_deref(),
                creator_message.as_deref(),
            );
            sqlx::query(
                r#"
                UPDATE marketplace_listings
                SET status = 'suspended', metadata = metadata || $2::jsonb, updated_at = now()
                WHERE id = $1
                "#,
            )
            .bind(listing.listing_id)
            .bind(&metadata)
            .execute(&mut *tx)
            .await?;
            (None, listing.listing_status.clone(), "suspended".to_owned())
        }
        MODERATION_UNPUBLISH_VERSION => {
            let version_id = payload.version_id.expect("validated version id");
            let version = sqlx::query_as::<_, VersionModerationRow>(
                r#"
                SELECT id as version_id, status as version_status
                FROM marketplace_versions
                WHERE id = $1 AND listing_id = $2
                "#,
            )
            .bind(version_id)
            .bind(listing.listing_id)
            .fetch_one(&mut *tx)
            .await?;

            sqlx::query("UPDATE marketplace_versions SET status = 'deprecated', updated_at = now() WHERE id = $1")
                .bind(version.version_id)
                .execute(&mut *tx)
                .await?;

            let approved = sqlx::query_as::<_, ApprovedVersionCountRow>(
                r#"
                SELECT COUNT(*)::BIGINT as approved_versions
                FROM marketplace_versions
                WHERE listing_id = $1 AND status = 'approved'
                "#,
            )
            .bind(listing.listing_id)
            .fetch_one(&mut *tx)
            .await?;
            let listing_status = if approved.approved_versions > 0 {
                "approved"
            } else {
                "changes_requested"
            };
            let metadata = moderation_metadata(
                &action,
                &reason,
                internal_comment.as_deref(),
                creator_message.as_deref(),
            );
            sqlx::query(
                r#"
                UPDATE marketplace_listings
                SET status = $2, metadata = metadata || $3::jsonb, updated_at = now()
                WHERE id = $1
                "#,
            )
            .bind(listing.listing_id)
            .bind(listing_status)
            .bind(&metadata)
            .execute(&mut *tx)
            .await?;
            (
                Some(version.version_id),
                version.version_status,
                "deprecated".to_owned(),
            )
        }
        MODERATION_EMERGENCY_BLOCK => {
            let metadata = moderation_metadata(
                &action,
                &reason,
                internal_comment.as_deref(),
                creator_message.as_deref(),
            );
            sqlx::query(
                r#"
                UPDATE marketplace_listings
                SET status = 'blocked', metadata = metadata || $2::jsonb, updated_at = now()
                WHERE id = $1
                "#,
            )
            .bind(listing.listing_id)
            .bind(&metadata)
            .execute(&mut *tx)
            .await?;
            sqlx::query(
                r#"
                UPDATE marketplace_versions
                SET status = 'blocked', updated_at = now()
                WHERE listing_id = $1 AND status IN ('submitted', 'validating', 'approved')
                "#,
            )
            .bind(listing.listing_id)
            .execute(&mut *tx)
            .await?;
            sqlx::query(
                r#"
                UPDATE marketplace_installations
                SET status = 'blocked', updated_at = now()
                WHERE listing_id = $1 AND status <> 'uninstalled'
                "#,
            )
            .bind(listing.listing_id)
            .execute(&mut *tx)
            .await?;
            (None, listing.listing_status.clone(), "blocked".to_owned())
        }
        _ => unreachable!("validated moderation action"),
    };

    let metadata = json!({
        "phase": "v3.4",
        "action": action,
        "reason": reason,
        "version_id": event_version_id
    });
    let event_id = sqlx::query_as::<_, InsertedReviewEventRow>(
        r#"
        INSERT INTO marketplace_review_events (
          submission_id, listing_id, version_id, actor_id, action, previous_status, next_status,
          internal_comment, creator_message, reason, metadata
        )
        VALUES (NULL, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id
        "#,
    )
    .bind(listing.listing_id)
    .bind(event_version_id)
    .bind(claims.sub)
    .bind(&action)
    .bind(previous_status)
    .bind(&next_status)
    .bind(internal_comment.as_deref())
    .bind(creator_message.as_deref())
    .bind(&reason)
    .bind(&metadata)
    .fetch_one(&mut *tx)
    .await?
    .id;

    tx.commit().await?;

    audit::record(
        &state.db,
        &tenant,
        "marketplace.moderation.action",
        "marketplace_listing",
        Some(listing.listing_id),
        json!({
            "event_id": event_id,
            "action": action,
            "version_id": event_version_id,
            "next_status": next_status
        }),
    )
    .await?;

    Ok(Json(load_review_event(&state, event_id).await?))
}

pub async fn list_review_events(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MarketplaceReviewEventResponse>>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let events = sqlx::query_as::<_, MarketplaceReviewEventResponse>(&review_event_select_sql(
        "ORDER BY event.created_at DESC LIMIT 100",
    ))
    .fetch_all(&state.db)
    .await?;

    Ok(Json(events))
}

pub async fn list_review_reports(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MarketplaceValidationReportResponse>>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN])?;
    let reports = sqlx::query_as::<_, MarketplaceValidationReportResponse>(
        r#"
        SELECT listing.id as listing_id,
               listing.title as listing_title,
               listing.slug as listing_slug,
               creator.id as creator_id,
               creator.display_name as creator_display_name,
               version.id as version_id,
               version.version,
               version.status as version_status,
               version.validation_status,
               version.security_risk_level,
               version.validation_report,
               version.compatibility_report,
               submission.id as submission_id,
               submission.review_status,
               submission.risk_level,
               submission.review_notes,
               submission.created_at as submitted_at,
               submission.updated_at
        FROM marketplace_submissions submission
        JOIN marketplace_versions version ON version.id = submission.version_id
        JOIN marketplace_listings listing ON listing.id = version.listing_id
        JOIN marketplace_creators creator ON creator.id = listing.creator_id
        ORDER BY submission.created_at DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(reports))
}

async fn load_review_event(
    state: &AppState,
    event_id: Uuid,
) -> Result<MarketplaceReviewEventResponse, AppError> {
    sqlx::query_as::<_, MarketplaceReviewEventResponse>(&review_event_select_sql(
        "WHERE event.id = $1",
    ))
    .bind(event_id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

fn review_event_select_sql(where_clause: &str) -> String {
    format!(
        r#"
        SELECT event.id,
               event.submission_id,
               event.listing_id,
               listing.title as listing_title,
               event.version_id,
               version.version,
               event.actor_id,
               users.email as actor_email,
               event.action,
               event.previous_status,
               event.next_status,
               event.internal_comment,
               event.creator_message,
               event.reason,
               event.metadata,
               event.created_at
        FROM marketplace_review_events event
        JOIN marketplace_listings listing ON listing.id = event.listing_id
        LEFT JOIN marketplace_versions version ON version.id = event.version_id
        LEFT JOIN users ON users.id = event.actor_id
        {where_clause}
        "#
    )
}

fn review_reason(
    decision: &str,
    internal_comment: Option<&str>,
    creator_message: Option<&str>,
) -> String {
    creator_message
        .or(internal_comment)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| match decision {
            "approve" => "Approved for Marketplace publication".to_owned(),
            "reject" => "Rejected during Marketplace review".to_owned(),
            "request_changes" => "Changes requested by Marketplace review".to_owned(),
            _ => "Marketplace review decision".to_owned(),
        })
}

fn moderation_metadata(
    action: &str,
    reason: &str,
    internal_comment: Option<&str>,
    creator_message: Option<&str>,
) -> Value {
    json!({
        "phase": "v3.4",
        "moderation": {
            "action": action,
            "reason": reason,
            "internal_comment": internal_comment,
            "creator_message": creator_message
        }
    })
}

impl From<MarketplaceInstallationDbRow> for MarketplaceInstallationResponse {
    fn from(row: MarketplaceInstallationDbRow) -> Self {
        Self {
            id: row.id,
            organization_id: row.organization_id,
            listing_id: row.listing_id,
            listing_title: row.listing_title,
            listing_slug: row.listing_slug,
            product_type: row.product_type,
            pricing_type: row.pricing_type,
            version_id: row.version_id,
            installed_version: row.installed_version,
            status: row.status,
            permissions: row.permissions,
            permission_approved_by: row.permission_approved_by,
            permission_approved_at: row.permission_approved_at,
            rollback_version_id: row.rollback_version_id,
            rollback_version: row.rollback_version,
            cleanup_policy: row.cleanup_policy,
            version_pinned: row.version_pinned,
            installed_by: row.installed_by,
            installed_at: row.installed_at,
            enabled_at: row.enabled_at,
            disabled_at: row.disabled_at,
            uninstalled_at: row.uninstalled_at,
            version_changed_at: row.version_changed_at,
            updated_at: row.updated_at,
        }
    }
}

fn installation_select_sql(where_clause: &str) -> String {
    format!(
        r#"
        SELECT installation.id,
               installation.organization_id,
               installation.listing_id,
               listing.title as listing_title,
               listing.slug as listing_slug,
               listing.product_type,
               listing.pricing_type,
               installation.version_id,
               version.version as installed_version,
               installation.status,
               installation.permissions_json as permissions,
               installation.permission_approved_by,
               installation.permission_approved_at,
               installation.rollback_version_id,
               rollback.version as rollback_version,
               installation.cleanup_policy,
               installation.version_pinned,
               installation.installed_by,
               installation.installed_at,
               installation.enabled_at,
               installation.disabled_at,
               installation.uninstalled_at,
               installation.version_changed_at,
               installation.updated_at
        FROM marketplace_installations installation
        JOIN marketplace_listings listing ON listing.id = installation.listing_id
        JOIN marketplace_versions version ON version.id = installation.version_id
        LEFT JOIN marketplace_versions rollback ON rollback.id = installation.rollback_version_id
        {where_clause}
        ORDER BY CASE installation.status WHEN 'active' THEN 0 WHEN 'disabled' THEN 1
                     WHEN 'blocked' THEN 2 ELSE 3 END,
                 installation.updated_at DESC
        "#,
    )
}

async fn load_installation_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    installation_id: Uuid,
) -> Result<MarketplaceInstallationDbRow, AppError> {
    sqlx::query_as::<_, MarketplaceInstallationDbRow>(&installation_select_sql(
        "WHERE installation.id = $1 AND installation.organization_id = $2",
    ))
    .bind(installation_id)
    .bind(organization_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| AppError::NotFound("Marketplace installation not found".to_owned()))
}

async fn load_installation_on_connection(
    db: &mut PgConnection,
    organization_id: Uuid,
    installation_id: Uuid,
) -> Result<InstallationLifecycleRow, AppError> {
    sqlx::query_as::<_, InstallationLifecycleRow>(&installation_lifecycle_select_sql(false))
        .bind(installation_id)
        .bind(organization_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Marketplace installation not found".to_owned()))
}

async fn load_installation_lifecycle_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    installation_id: Uuid,
    for_update: bool,
) -> Result<InstallationLifecycleRow, AppError> {
    sqlx::query_as::<_, InstallationLifecycleRow>(&installation_lifecycle_select_sql(for_update))
        .bind(installation_id)
        .bind(organization_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Marketplace installation not found".to_owned()))
}

fn installation_lifecycle_select_sql(for_update: bool) -> String {
    let lock = if for_update {
        "FOR UPDATE OF installation"
    } else {
        ""
    };
    format!(
        r#"
        SELECT installation.listing_id,
               installation.version_id,
               version.version as current_version,
               installation.status,
               installation.permissions_json,
               installation.permission_approved_by,
               installation.permission_approved_at,
               installation.rollback_version_id,
               installation.metadata,
               installation.version_pinned,
               listing.product_type,
               listing.pricing_type,
               listing.status as listing_status,
               version.status as current_version_status,
               version.manifest_json as current_manifest_json,
               version.artifact_object_key as current_artifact_object_key,
               version.artifact_sha256 as current_artifact_sha256,
               version.artifact_size_bytes as current_artifact_size_bytes,
               version.validation_status as current_validation_status,
               version.security_risk_level as current_security_risk_level
        FROM marketplace_installations installation
        JOIN marketplace_listings listing ON listing.id = installation.listing_id
        JOIN marketplace_versions version ON version.id = installation.version_id
        WHERE installation.id = $1 AND installation.organization_id = $2
        {lock}
        "#,
    )
}

async fn load_install_candidate(
    tx: &mut Transaction<'_, Postgres>,
    listing_id: Uuid,
    version_id: Uuid,
) -> Result<InstallationCandidateRow, AppError> {
    sqlx::query_as::<_, InstallationCandidateRow>(
        r#"
        SELECT listing.id as listing_id,
               listing.title as listing_title,
               listing.slug as listing_slug,
               listing.product_type,
               listing.pricing_type,
               listing.status as listing_status,
               version.id as version_id,
               version.version,
               version.status as version_status,
               version.manifest_json,
               version.artifact_object_key,
               version.artifact_sha256,
               version.artifact_size_bytes,
               version.validation_status,
               version.security_risk_level
        FROM marketplace_listings listing
        JOIN marketplace_versions version ON version.listing_id = listing.id
        WHERE listing.id = $1 AND version.id = $2
        FOR SHARE OF listing, version
        "#,
    )
    .bind(listing_id)
    .bind(version_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| AppError::NotFound("approved Marketplace version not found".to_owned()))
}

async fn load_version_gate_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    listing_id: Uuid,
    version_id: Uuid,
) -> Result<MarketplaceVersionGateRow, AppError> {
    sqlx::query_as::<_, MarketplaceVersionGateRow>(
        r#"
        SELECT id as version_id,
               version,
               status as version_status,
               manifest_json,
               artifact_object_key,
               artifact_sha256,
               artifact_size_bytes,
               validation_status,
               security_risk_level
        FROM marketplace_versions version
        WHERE listing_id = $1 AND id = $2
        FOR SHARE OF version
        "#,
    )
    .bind(listing_id)
    .bind(version_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| AppError::NotFound("Marketplace version not found for this listing".to_owned()))
}

#[allow(clippy::too_many_arguments)]
async fn ensure_paid_entitlement(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    purchase_id: Uuid,
    listing_id: Uuid,
    version_id: Uuid,
) -> Result<(), AppError> {
    let entitled: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM marketplace_entitlements entitlement JOIN marketplace_purchases purchase ON purchase.id = entitlement.purchase_id WHERE entitlement.organization_id = $1 AND entitlement.purchase_id = $2 AND entitlement.listing_id = $3 AND entitlement.version_id = $4 AND entitlement.status = 'active' AND purchase.status = 'completed')",
    )
    .bind(organization_id)
    .bind(purchase_id)
    .bind(listing_id)
    .bind(version_id)
    .fetch_one(&mut **tx)
    .await?;
    if entitled {
        Ok(())
    } else {
        Err(AppError::Conflict(
            "completed Marketplace entitlement was not found for this organization/version"
                .to_owned(),
        ))
    }
}

async fn ensure_active_listing_entitlement(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    listing_id: Uuid,
) -> Result<(), AppError> {
    let entitled: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM marketplace_entitlements entitlement JOIN marketplace_purchases purchase ON purchase.id = entitlement.purchase_id WHERE entitlement.organization_id = $1 AND entitlement.listing_id = $2 AND entitlement.status = 'active' AND purchase.status = 'completed')",
    )
    .bind(organization_id)
    .bind(listing_id)
    .fetch_one(&mut **tx)
    .await?;
    if entitled {
        Ok(())
    } else {
        Err(AppError::Conflict(
            "active Marketplace entitlement was not found for this paid product".to_owned(),
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_install_gate(
    product_type: &str,
    pricing_type: &str,
    listing_status: &str,
    version_status: &str,
    validation_status: &str,
    security_risk_level: &str,
    manifest: &Value,
    organization_plan: &str,
    allow_paid: bool,
) -> Result<Value, AppError> {
    validate_marketplace_version_gate(
        product_type,
        pricing_type,
        listing_status,
        version_status,
        validation_status,
        security_risk_level,
        manifest,
        organization_plan,
        false,
        allow_paid,
    )
}

#[allow(clippy::too_many_arguments)]
fn validate_rollback_gate(
    product_type: &str,
    pricing_type: &str,
    listing_status: &str,
    version_status: &str,
    validation_status: &str,
    security_risk_level: &str,
    manifest: &Value,
    organization_plan: &str,
) -> Result<Value, AppError> {
    validate_marketplace_version_gate(
        product_type,
        pricing_type,
        listing_status,
        version_status,
        validation_status,
        security_risk_level,
        manifest,
        organization_plan,
        true,
        pricing_type == "paid",
    )
}

#[allow(clippy::too_many_arguments)]
fn validate_marketplace_version_gate(
    product_type: &str,
    pricing_type: &str,
    listing_status: &str,
    version_status: &str,
    validation_status: &str,
    security_risk_level: &str,
    manifest: &Value,
    organization_plan: &str,
    allow_deprecated: bool,
    allow_paid: bool,
) -> Result<Value, AppError> {
    if pricing_type != "free" && !(allow_paid && pricing_type == "paid") {
        return Err(AppError::Conflict(
            "Marketplace entitlement is required before installing paid or custom products"
                .to_owned(),
        ));
    }
    validate_mvp_product_type(product_type).map_err(|error| AppError::Conflict(error.message))?;
    if listing_status != "approved" {
        return Err(AppError::Conflict(
            "Marketplace listing is not approved for installation".to_owned(),
        ));
    }
    let version_allowed =
        version_status == "approved" || (allow_deprecated && version_status == "deprecated");
    if !version_allowed {
        return Err(AppError::Conflict(
            "Marketplace version is not approved for this lifecycle action".to_owned(),
        ));
    }
    if !matches!(validation_status, "passed" | "warning") {
        return Err(AppError::Conflict(
            "Marketplace version has not passed package validation".to_owned(),
        ));
    }
    if !matches!(security_risk_level, "low" | "medium") {
        return Err(AppError::Conflict(
            "Marketplace version is blocked by its security risk level".to_owned(),
        ));
    }

    let report = catalog_compatibility_report(manifest, organization_plan);
    if !is_catalog_compatible(&report) {
        let reasons = report
            .get("reasons")
            .and_then(Value::as_array)
            .map(|reasons| {
                reasons
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_else(|| "unknown compatibility failure".to_owned());
        return Err(AppError::Conflict(format!(
            "Marketplace version is incompatible with the active organization: {reasons}"
        )));
    }
    Ok(report)
}

fn metadata_with_rollback_snapshot(
    installation: &InstallationLifecycleRow,
    compatibility_report: &Value,
    action: &str,
    target_version_id: Uuid,
) -> Value {
    let mut metadata = installation
        .metadata
        .as_object()
        .cloned()
        .unwrap_or_default();
    metadata.insert(
        "rollback_permissions".to_owned(),
        installation.permissions_json.clone(),
    );
    metadata.insert(
        "rollback_permission_approved_by".to_owned(),
        json!(installation.permission_approved_by),
    );
    metadata.insert(
        "rollback_permission_approved_at".to_owned(),
        json!(installation.permission_approved_at),
    );
    metadata.insert(
        "rollback_from_version_id".to_owned(),
        json!(installation.version_id),
    );
    metadata.insert("last_lifecycle_action".to_owned(), json!(action));
    metadata.insert(
        "last_target_version_id".to_owned(),
        json!(target_version_id),
    );
    metadata.insert(
        "compatibility_report".to_owned(),
        compatibility_report.clone(),
    );
    Value::Object(metadata)
}

fn rollback_approval_from_metadata(metadata: &Value) -> Option<(Uuid, DateTime<Utc>)> {
    let approved_by = metadata
        .get("rollback_permission_approved_by")?
        .as_str()
        .and_then(|value| Uuid::parse_str(value).ok())?;
    let approved_at = metadata
        .get("rollback_permission_approved_at")?
        .as_str()
        .and_then(|value| DateTime::parse_from_rfc3339(value).ok())?
        .with_timezone(&Utc);
    Some((approved_by, approved_at))
}

async fn ensure_active_user(state: &AppState, user_id: Uuid) -> Result<(), AppError> {
    let active: bool = sqlx::query_scalar("SELECT is_active FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.db)
        .await?;
    if active {
        Ok(())
    } else {
        Err(AppError::Forbidden(
            "inactive users cannot request creator profiles".to_owned(),
        ))
    }
}

async fn load_creator_profile(
    state: &AppState,
    user_id: Uuid,
) -> Result<Option<CreatorProfileResponse>, AppError> {
    let sql = creator_select_sql("WHERE user_id = $1");
    sqlx::query_as::<_, CreatorProfileResponse>(&sql)
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(AppError::from)
}

async fn load_creator_for_user(
    state: &AppState,
    user_id: Uuid,
) -> Result<Option<CreatorOwnershipRow>, AppError> {
    sqlx::query_as::<_, CreatorOwnershipRow>(
        r#"
        SELECT id
        FROM marketplace_creators
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(AppError::from)
}

async fn require_creator_for_user(
    state: &AppState,
    user_id: Uuid,
) -> Result<CreatorOwnershipRow, AppError> {
    load_creator_for_user(state, user_id).await?.ok_or_else(|| {
        AppError::Validation("request a creator profile before creating listings".to_owned())
    })
}

async fn ensure_listing_editable(
    state: &AppState,
    listing_id: Uuid,
    creator_id: Uuid,
) -> Result<(), AppError> {
    let status: String = sqlx::query_scalar(
        r#"
        SELECT status
        FROM marketplace_listings
        WHERE id = $1 AND creator_id = $2
        "#,
    )
    .bind(listing_id)
    .bind(creator_id)
    .fetch_one(&state.db)
    .await?;

    if matches!(status.as_str(), "draft" | "changes_requested") {
        Ok(())
    } else {
        Err(AppError::Validation(
            "only draft or changes requested listings can be edited".to_owned(),
        ))
    }
}

async fn load_listing_for_submission(
    state: &AppState,
    user_id: Uuid,
    listing_id: Uuid,
) -> Result<ListingSubmissionRow, AppError> {
    sqlx::query_as::<_, ListingSubmissionRow>(
        r#"
        SELECT listing.creator_id,
               creator.slug as creator_slug,
               creator.status as creator_status,
               listing.product_type,
               listing.title,
               listing.slug,
               listing.summary,
               listing.description,
               listing.category,
               listing.pricing_type,
               listing.price_cents,
               listing.license,
               listing.support_url,
               listing.screenshots
        FROM marketplace_listings listing
        JOIN marketplace_creators creator ON creator.id = listing.creator_id
        WHERE listing.id = $1 AND creator.user_id = $2
        "#,
    )
    .bind(listing_id)
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

fn creator_select_sql(where_clause: &str) -> String {
    format!(
        r#"
        SELECT id, user_id, slug, display_name, bio, status, payout_status, support_email,
               verification_notes, verified_by, verified_at, metadata, requested_at, created_at, updated_at
        FROM marketplace_creators
        {where_clause}
        "#
    )
}

fn listing_select_sql(where_clause: &str) -> String {
    format!(
        r#"
        SELECT id, creator_id, product_type, title, slug, summary, description, category,
               status, pricing_type, price_cents, license, support_url, screenshots, metadata,
               submitted_by, submitted_at, created_at, updated_at
        FROM marketplace_listings
        {where_clause}
        ORDER BY updated_at DESC
        "#
    )
}

fn clean_query_param(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn catalog_item_from_row(
    row: &MarketplaceCatalogRow,
    plan_slug: &str,
) -> Option<MarketplaceCatalogItemResponse> {
    let compatibility_report = catalog_compatibility_report(&row.manifest_json, plan_slug);
    if !is_catalog_compatible(&compatibility_report) {
        return None;
    }

    Some(MarketplaceCatalogItemResponse {
        id: row.id,
        title: row.title.clone(),
        slug: row.slug.clone(),
        summary: row.summary.clone(),
        category: row.category.clone(),
        product_type: row.product_type.clone(),
        pricing_type: row.pricing_type.clone(),
        price_cents: row.price_cents,
        creator_display_name: row.creator_display_name.clone(),
        latest_version_id: row.version_id,
        latest_version: row.version.clone(),
        badge: "Compatible".to_owned(),
        rating_average: row.rating_average,
        rating_count: row.rating_count,
        active_installations: row.active_installations,
        compatibility_report,
        permissions: manifest_array(&row.manifest_json, "permissions"),
        screenshots: row.screenshots.clone(),
        support_url: row.support_url.clone(),
        updated_at: row.updated_at,
    })
}

fn catalog_version_from_row(
    row: &MarketplaceCatalogVersionRow,
    plan_slug: &str,
) -> Option<MarketplaceCatalogVersionResponse> {
    let compatibility_report = catalog_compatibility_report(&row.manifest_json, plan_slug);
    if !is_catalog_compatible(&compatibility_report) {
        return None;
    }

    Some(MarketplaceCatalogVersionResponse {
        id: row.id,
        version: row.version.clone(),
        compatibility_report,
        permissions: manifest_array(&row.manifest_json, "permissions"),
        changelog: manifest_value(&row.manifest_json, "changelog"),
        created_at: row.created_at,
    })
}

fn empty_json_object() -> Value {
    json!({})
}

fn manifest_array(manifest: &Value, key: &str) -> Value {
    manifest
        .get(key)
        .and_then(Value::as_array)
        .cloned()
        .map(Value::Array)
        .unwrap_or_else(|| json!([]))
}

fn manifest_value(manifest: &Value, key: &str) -> Value {
    manifest.get(key).cloned().unwrap_or_else(|| json!([]))
}
fn validate_listing_payload(
    payload: &ListingRequest,
    screenshots: &[String],
) -> Result<(), AppError> {
    map_validation(validate_listing_review_input(&ListingReviewInput {
        product_type: &payload.product_type,
        title: &payload.title,
        slug: &payload.slug,
        summary: &payload.summary,
        description: &payload.description,
        category: &payload.category,
        pricing_type: &payload.pricing_type,
        price_cents: payload.price_cents,
        license: &payload.license,
        support_url: payload.support_url.as_deref(),
        screenshots,
    }))
}

fn validate_listing_row_for_review(row: &ListingSubmissionRow) -> Result<(), AppError> {
    let screenshots = screenshots_from_value(&row.screenshots)?;
    map_validation(validate_listing_for_review(
        &row.creator_status,
        &ListingReviewInput {
            product_type: &row.product_type,
            title: &row.title,
            slug: &row.slug,
            summary: &row.summary,
            description: &row.description,
            category: &row.category,
            pricing_type: &row.pricing_type,
            price_cents: row.price_cents,
            license: &row.license,
            support_url: row.support_url.as_deref(),
            screenshots: &screenshots,
        },
    ))
}

fn screenshots_from_value(value: &Value) -> Result<Vec<String>, AppError> {
    let Some(items) = value.as_array() else {
        return Err(AppError::Validation(
            "listing screenshots must be an array".to_owned(),
        ));
    };
    Ok(items
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect())
}

fn listing_metadata(payload: &ListingRequest, screenshots: &[String]) -> Value {
    json!({
        "description": payload.description.trim(),
        "screenshots": screenshots,
        "price_cents": payload.price_cents,
        "license": payload.license.trim(),
        "support_url": normalize_optional_text(payload.support_url.clone()),
        "phase": "v3.3"
    })
}

fn validate_manifest_matches_listing(
    manifest: &Value,
    row: &ListingSubmissionRow,
) -> Result<(), AppError> {
    let manifest_type = manifest
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if manifest_type != row.product_type {
        return Err(AppError::Validation(format!(
            "manifest type '{manifest_type}' does not match listing type '{}'",
            row.product_type
        )));
    }

    Ok(())
}

async fn persist_package_artifact(
    state: &AppState,
    object_key: &str,
    bytes: &[u8],
) -> Result<(), AppError> {
    let mut path = PathBuf::from(&state.config.upload_dir);
    for segment in object_key.split('/') {
        path.push(segment);
    }

    let parent = path
        .parent()
        .ok_or_else(|| AppError::Internal("package object key has no parent path".to_owned()))?;
    fs::create_dir_all(parent)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;
    fs::write(&path, bytes)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;
    Ok(())
}

fn sanitize_filename(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_')
        })
        .collect();
    if sanitized.is_empty() {
        "marketplace-package.zip".to_owned()
    } else {
        sanitized
    }
}

fn map_validation(result: Result<(), Vec<String>>) -> Result<(), AppError> {
    result.map_err(|errors| AppError::Validation(errors.join("; ")))
}

#[cfg(test)]
mod tests {
    use super::{
        sanitize_filename, screenshots_from_value, validate_install_gate, validate_rollback_gate,
    };
    use serde_json::json;

    #[test]
    fn filename_sanitizer_removes_path_characters() {
        assert_eq!(sanitize_filename("../package v1.zip"), "..packagev1.zip");
        assert_eq!(sanitize_filename(""), "marketplace-package.zip");
    }

    #[test]
    fn screenshots_must_be_json_array() {
        assert_eq!(
            screenshots_from_value(&json!(["https://example.com/a.png"]))
                .unwrap()
                .len(),
            1
        );
        assert!(screenshots_from_value(&json!({ "url": "https://example.com/a.png" })).is_err());
    }

    #[test]
    fn deprecated_version_is_rollback_only() {
        let manifest = json!({
            "permissions": ["page.read"],
            "compatibility": {
                "min_zinhar_version": "0.1.0",
                "max_zinhar_version": "99.0.0",
                "required_plan": "free"
            }
        });
        assert!(
            validate_install_gate(
                "component_pack",
                "free",
                "approved",
                "deprecated",
                "passed",
                "low",
                &manifest,
                "free",
                false,
            )
            .is_err()
        );
        assert!(
            validate_rollback_gate(
                "component_pack",
                "free",
                "approved",
                "deprecated",
                "passed",
                "low",
                &manifest,
                "free",
            )
            .is_ok()
        );
    }
}
