use std::path::PathBuf;

use axum::extract::{Extension, Multipart, Path, State};
use axum::routing::{get, patch, post, put};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::FromRow;
use tokio::fs;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::services::marketplace_manifest::MARKETPLACE_MANIFEST_SCHEMA_VERSION;
use crate::services::marketplace_package::{
    marketplace_package_object_key, sha256_hex, validate_package_size,
};
use crate::services::marketplace_submission::{
    ListingReviewInput, normalize_optional_text, sanitize_screenshot_urls,
    validate_creator_profile, validate_creator_verification_status, validate_listing_for_review,
    validate_listing_review_input,
};
use crate::services::marketplace_validation::evaluate_marketplace_package;
use crate::services::{audit, quota, rbac};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
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
        .route("/api/marketplace/review/reports", get(list_review_reports))
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
    use super::{sanitize_filename, screenshots_from_value};
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
}
