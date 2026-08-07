use std::path::{Path as FsPath, PathBuf};
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Extension, Multipart, Path, Query, State};
use axum::http::header::{
    ACCEPT_RANGES, CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE,
    PRAGMA, RANGE,
};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::middleware::tenant::TenantContext;
use crate::services::file_security::{
    FileKind, MAX_METADATA_BYTES, MalwareScanOutcome, NoopMalwareScanner, SecureTempUpload,
    StagedUpload, UploadPurpose, cleanup_stale_processing_directories,
    cleanup_stale_temporary_files, content_disposition, detect_file_kind, media_storage_key,
    policy_for, publish_generated_file, secure_join, secure_join_no_symlinks,
    validate_detected_kind, validate_pdf_structure,
};
use crate::services::media_processing::{
    ProcessedImageSet, process_image_upload, remove_processing_directory,
};
use crate::services::{audit, file_cleanup, quota, rbac, rls};
use crate::state::AppState;

const PRIVATE_CACHE_CONTROL: &str = "private, no-store, max-age=0";
const PUBLIC_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/media", get(list_media))
        .route("/api/media/upload", post(upload_media))
        .route("/api/media/{id}/download", get(download_media))
        .route(
            "/api/media/{id}",
            get(get_media).put(update_media).delete(delete_media),
        )
}

pub fn public_router() -> Router<AppState> {
    Router::new().route("/uploads/public/{*path}", get(deliver_public_media))
}

#[derive(Debug, Deserialize)]
pub struct MediaListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MediaUpdateRequest {
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MediaResponse {
    pub id: Uuid,
    pub filename: String,
    pub url: String,
    pub mime_type: String,
    pub size: i64,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub uploader_id: Option<Uuid>,
    pub visibility: String,
    pub verification_status: String,
    pub lifecycle_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct MediaVariantResponse {
    pub id: Uuid,
    pub media_id: Uuid,
    pub variant_name: String,
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaDetailResponse {
    pub media: MediaResponse,
    pub variants: Vec<MediaVariantResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaListResponse {
    pub data: Vec<MediaResponse>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, FromRow)]
struct FileDeliveryRow {
    filename: String,
    mime_type: String,
    storage_key: String,
}

#[utoipa::path(
    get,
    path = "/api/media",
    tag = "media",
    responses((status = 200, description = "Media library", body = MediaListResponse))
)]
pub async fn list_media(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Query(query): Query<MediaListQuery>,
) -> Result<Json<MediaListResponse>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let data = sqlx::query_as::<_, MediaResponse>(
        r#"
        SELECT id, filename, url, mime_type, size, alt_text, caption, uploader_id,
               visibility, verification_status, lifecycle_status, created_at, updated_at
        FROM media
        WHERE organization_id = $1
          AND lifecycle_status = 'active'
          AND ($2::TEXT IS NULL OR mime_type = $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(tenant.organization_id)
    .bind(query.mime_type.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(db.as_mut())
    .await?;
    Ok(Json(MediaListResponse {
        data,
        page,
        per_page,
    }))
}

#[utoipa::path(
    post,
    path = "/api/media/upload",
    tag = "media",
    responses((status = 200, description = "Uploaded media", body = MediaDetailResponse))
)]
pub async fn upload_media(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(tenant): Extension<TenantContext>,
    mut multipart: Multipart,
) -> Result<Json<MediaDetailResponse>, AppError> {
    rbac::require_org_media_writer(&tenant.role)?;
    let storage_root = PathBuf::from(&state.config.upload_dir);
    let _ = cleanup_stale_temporary_files(
        &storage_root,
        Duration::from_secs(24 * 60 * 60),
        state.config.security_cleanup_batch_size.clamp(1, 1_000) as usize,
    )
    .await;
    let _ = cleanup_stale_processing_directories(
        &storage_root,
        Duration::from_secs(24 * 60 * 60),
        state.config.security_cleanup_batch_size.clamp(1, 1_000) as usize,
    )
    .await;
    let _ = file_cleanup::reconcile_stale_publishing_media(
        &state,
        &tenant,
        state.config.security_cleanup_batch_size.clamp(1, 100),
    )
    .await;
    let max_media_bytes = policy_for(UploadPurpose::PrivateDocument)
        .max_bytes
        .min(state.config.max_upload_size);
    let mut alt_text = None;
    let mut caption = None;
    let mut staged_upload = None;
    let mut metadata_bytes = 0usize;
    let mut part_count = 0u64;
    let metadata_limit = state
        .config
        .max_upload_metadata_bytes
        .min(MAX_METADATA_BYTES as u64) as usize;

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|error| AppError::BadRequest(error.to_string()))?
    {
        part_count += 1;
        if part_count > state.config.max_upload_parts {
            return Err(AppError::Validation(
                "multipart request contains too many parts".to_owned(),
            ));
        }
        let name = field.name().unwrap_or_default().to_owned();
        match name.as_str() {
            "alt_text" | "caption" => {
                let value = read_text_field_bounded(&mut field, metadata_limit).await?;
                metadata_bytes = metadata_bytes.saturating_add(value.len());
                if metadata_bytes > metadata_limit {
                    return Err(AppError::Validation(
                        "upload metadata exceeds the allowed size".to_owned(),
                    ));
                }
                if name == "alt_text" {
                    alt_text = clean_optional_text(Some(value));
                } else {
                    caption = clean_optional_text(Some(value));
                }
            }
            "file" => {
                if staged_upload.is_some() {
                    return Err(AppError::Validation(
                        "exactly one file field is allowed".to_owned(),
                    ));
                }
                let filename = field.file_name().unwrap_or("upload.bin").to_owned();
                let declared_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_owned();
                let mut temp = SecureTempUpload::create(
                    &storage_root,
                    max_media_bytes,
                    &filename,
                    &declared_type,
                )
                .await
                .map_err(|error| AppError::Internal(error.to_string()))?;
                while let Some(chunk) = field
                    .chunk()
                    .await
                    .map_err(|error| AppError::BadRequest(error.to_string()))?
                {
                    temp.write_chunk(&chunk)
                        .await
                        .map_err(|error| AppError::Validation(error.to_string()))?;
                }
                staged_upload = Some(
                    temp.finish()
                        .await
                        .map_err(|error| AppError::Validation(error.to_string()))?,
                );
            }
            _ => {}
        }
    }

    let staged =
        staged_upload.ok_or_else(|| AppError::Validation("file field is required".to_owned()))?;
    let prefix = staged
        .read_prefix(8_192)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let kind = detect_file_kind(&prefix).ok_or_else(|| {
        AppError::Validation("file type could not be detected from content".to_owned())
    })?;
    let purpose = if matches!(kind, FileKind::Jpeg | FileKind::Png | FileKind::Webp) {
        UploadPurpose::PublicImage
    } else if matches!(kind, FileKind::Pdf | FileKind::Text) {
        UploadPurpose::PrivateDocument
    } else {
        return Err(AppError::Validation(
            "only raster images, PDF documents, and plain text files are allowed".to_owned(),
        ));
    };
    if staged.size > policy_for(purpose).max_bytes {
        return Err(AppError::Validation(format!(
            "file exceeds maximum size of {} bytes",
            policy_for(purpose).max_bytes
        )));
    }
    validate_detected_kind(purpose, &staged.declared_content_type, &prefix)
        .map_err(|error| AppError::Validation(error.to_string()))?;
    match kind {
        FileKind::Pdf => {
            let suffix = staged
                .read_suffix(4_096)
                .await
                .map_err(|error| AppError::Internal(error.to_string()))?;
            validate_pdf_structure(&prefix, &suffix)
                .map_err(|error| AppError::Validation(error.to_string()))?;
        }
        FileKind::Text => staged
            .validate_utf8_text()
            .await
            .map_err(|error| AppError::Validation(error.to_string()))?,
        _ => {}
    }

    let media_id = Uuid::now_v7();
    let scanner = NoopMalwareScanner;
    let scan_status = match scanner.scan_verdict(&staged.sha256) {
        MalwareScanOutcome::Unavailable => "unavailable",
        MalwareScanOutcome::Clean => "clean",
        MalwareScanOutcome::Infected => "infected",
        MalwareScanOutcome::Error => "error",
    };
    let upload = MediaUploadContext {
        uploader_id: claims.sub,
        media_id,
        kind,
        alt_text,
        caption,
        scan_status,
    };
    let detail = match purpose {
        UploadPurpose::PublicImage => upload_public_image(&state, &tenant, staged, upload).await?,
        UploadPurpose::PrivateDocument => {
            upload_private_document(&state, &tenant, staged, upload).await?
        }
        UploadPurpose::MarketplacePackage => unreachable!(),
    };

    audit::record(
        &state.db,
        &tenant,
        "media.upload",
        "media",
        Some(detail.media.id),
        json!({
            "mime_type": detail.media.mime_type,
            "size": detail.media.size,
            "visibility": detail.media.visibility,
            "verification_status": detail.media.verification_status,
            "malware_scan_status": scan_status,
        }),
    )
    .await?;
    Ok(Json(detail))
}

struct MediaUploadContext {
    uploader_id: Uuid,
    media_id: Uuid,
    kind: FileKind,
    alt_text: Option<String>,
    caption: Option<String>,
    scan_status: &'static str,
}

async fn upload_public_image(
    state: &AppState,
    tenant: &TenantContext,
    staged: StagedUpload,
    upload: MediaUploadContext,
) -> Result<MediaDetailResponse, AppError> {
    let storage_root = PathBuf::from(&state.config.upload_dir);
    let processed = process_image_upload(
        staged.path(),
        &storage_root,
        tenant.organization_id,
        upload.media_id,
    )
    .await?;
    let source_size = staged.size as i64;
    let source_sha256 = staged.sha256.clone();
    let total_size = processed
        .variants
        .iter()
        .try_fold(processed.original.size, |total, variant| {
            total.checked_add(variant.size)
        })
        .ok_or_else(|| AppError::Validation("processed image size overflowed".to_owned()))?;

    quota::ensure_default_subscription(&state.db, tenant).await?;
    let mut tx = rls::begin_tenant_transaction(&state.db, tenant).await?;
    quota::ensure_media_capacity_in_transaction(&mut tx, tenant.organization_id, total_size as i64)
        .await?;
    let media = insert_media_row(
        &mut tx,
        tenant.organization_id,
        upload.uploader_id,
        upload.media_id,
        &staged.original_filename,
        &processed.original.url,
        "image/webp",
        total_size as i64,
        upload.alt_text,
        upload.caption,
        &processed.original.storage_key,
        &source_sha256,
        &processed.original.sha256,
        source_size,
        "public",
        upload.scan_status,
        json!({
            "source_mime_type": upload.kind.mime_type(),
            "normalization": "decoded_and_reencoded_webp",
            "width": processed.original.width,
            "height": processed.original.height,
        }),
    )
    .await?;
    let variants =
        insert_variant_rows(&mut tx, tenant.organization_id, upload.media_id, &processed).await?;
    tx.commit().await?;

    let keys = image_storage_keys(&processed);
    let publish_result = publish_image_set(&storage_root, &processed).await;
    remove_processing_directory(&processed.original.path).await;
    staged
        .remove()
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;
    if let Err(error) = publish_result {
        rollback_publish(state, tenant, upload.media_id, &keys).await?;
        return Err(error);
    }
    activate_media(state, tenant, upload.media_id).await?;
    Ok(MediaDetailResponse {
        media: MediaResponse {
            lifecycle_status: "active".to_owned(),
            ..media
        },
        variants,
    })
}

async fn upload_private_document(
    state: &AppState,
    tenant: &TenantContext,
    staged: StagedUpload,
    upload: MediaUploadContext,
) -> Result<MediaDetailResponse, AppError> {
    let storage_root = PathBuf::from(&state.config.upload_dir);
    let storage_key = media_storage_key(
        UploadPurpose::PrivateDocument,
        tenant.organization_id,
        upload.media_id,
        upload.kind.extension(),
    )
    .map_err(|error| AppError::Internal(error.to_string()))?;
    let download_url = format!("/api/media/{}/download", upload.media_id);

    quota::ensure_default_subscription(&state.db, tenant).await?;
    let mut tx = rls::begin_tenant_transaction(&state.db, tenant).await?;
    quota::ensure_media_capacity_in_transaction(
        &mut tx,
        tenant.organization_id,
        staged.size as i64,
    )
    .await?;
    let media = insert_media_row(
        &mut tx,
        tenant.organization_id,
        upload.uploader_id,
        upload.media_id,
        &staged.original_filename,
        &download_url,
        upload.kind.mime_type(),
        staged.size as i64,
        upload.alt_text,
        upload.caption,
        &storage_key,
        &staged.sha256,
        &staged.sha256,
        staged.size as i64,
        "restricted",
        upload.scan_status,
        json!({"delivery": "authenticated_attachment"}),
    )
    .await?;
    tx.commit().await?;

    if let Err(error) = staged.persist(&storage_root, &storage_key).await {
        rollback_publish(
            state,
            tenant,
            upload.media_id,
            std::slice::from_ref(&storage_key),
        )
        .await?;
        return Err(AppError::Internal(error.to_string()));
    }
    activate_media(state, tenant, upload.media_id).await?;
    Ok(MediaDetailResponse {
        media: MediaResponse {
            lifecycle_status: "active".to_owned(),
            ..media
        },
        variants: Vec::new(),
    })
}

#[allow(clippy::too_many_arguments)]
async fn insert_media_row(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    organization_id: Uuid,
    uploader_id: Uuid,
    media_id: Uuid,
    filename: &str,
    url: &str,
    mime_type: &str,
    size: i64,
    alt_text: Option<String>,
    caption: Option<String>,
    storage_key: &str,
    source_sha256: &str,
    stored_sha256: &str,
    source_size: i64,
    visibility: &str,
    scan_status: &str,
    security_metadata: serde_json::Value,
) -> Result<MediaResponse, AppError> {
    sqlx::query_as::<_, MediaResponse>(
        r#"
        INSERT INTO media (
          id, organization_id, filename, url, mime_type, size, alt_text, caption, uploader_id,
          storage_key, source_sha256, stored_sha256, source_size, visibility,
          verification_status, malware_scan_status, lifecycle_status, security_metadata
        )
        VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9,
          $10, $11, $12, $13, $14, 'verified', $15, 'publishing', $16
        )
        RETURNING id, filename, url, mime_type, size, alt_text, caption, uploader_id,
                  visibility, verification_status, lifecycle_status, created_at, updated_at
        "#,
    )
    .bind(media_id)
    .bind(organization_id)
    .bind(filename)
    .bind(url)
    .bind(mime_type)
    .bind(size)
    .bind(alt_text)
    .bind(caption)
    .bind(uploader_id)
    .bind(storage_key)
    .bind(source_sha256)
    .bind(stored_sha256)
    .bind(source_size)
    .bind(visibility)
    .bind(scan_status)
    .bind(security_metadata)
    .fetch_one(&mut **tx)
    .await
    .map_err(AppError::from)
}

async fn insert_variant_rows(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    organization_id: Uuid,
    media_id: Uuid,
    processed: &ProcessedImageSet,
) -> Result<Vec<MediaVariantResponse>, AppError> {
    let mut rows = Vec::with_capacity(processed.variants.len());
    for variant in &processed.variants {
        rows.push(
            sqlx::query_as::<_, MediaVariantResponse>(
                r#"
                INSERT INTO media_variants (
                  organization_id, media_id, variant_name, url, width, height,
                  storage_key, stored_sha256, verification_status, lifecycle_status
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'verified', 'publishing')
                RETURNING id, media_id, variant_name, url, width, height, created_at
                "#,
            )
            .bind(organization_id)
            .bind(media_id)
            .bind(&variant.name)
            .bind(&variant.url)
            .bind(variant.width)
            .bind(variant.height)
            .bind(&variant.storage_key)
            .bind(&variant.sha256)
            .fetch_one(&mut **tx)
            .await?,
        );
    }
    Ok(rows)
}

async fn publish_image_set(
    storage_root: &FsPath,
    processed: &ProcessedImageSet,
) -> Result<(), AppError> {
    publish_generated_file(
        &processed.original.path,
        storage_root,
        &processed.original.storage_key,
    )
    .await
    .map_err(|error| AppError::Internal(error.to_string()))?;
    for variant in &processed.variants {
        publish_generated_file(&variant.path, storage_root, &variant.storage_key)
            .await
            .map_err(|error| AppError::Internal(error.to_string()))?;
    }
    Ok(())
}

fn image_storage_keys(processed: &ProcessedImageSet) -> Vec<String> {
    std::iter::once(processed.original.storage_key.clone())
        .chain(
            processed
                .variants
                .iter()
                .map(|variant| variant.storage_key.clone()),
        )
        .collect()
}

async fn activate_media(
    state: &AppState,
    tenant: &TenantContext,
    media_id: Uuid,
) -> Result<(), AppError> {
    let mut tx = rls::begin_tenant_transaction(&state.db, tenant).await?;
    sqlx::query(
        "UPDATE media SET lifecycle_status = 'active', published_at = now(), updated_at = now() WHERE id = $1 AND organization_id = $2 AND lifecycle_status = 'publishing'",
    )
    .bind(media_id)
    .bind(tenant.organization_id)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE media_variants SET lifecycle_status = 'active' WHERE media_id = $1 AND organization_id = $2 AND lifecycle_status = 'publishing'",
    )
    .bind(media_id)
    .bind(tenant.organization_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

async fn rollback_publish(
    state: &AppState,
    tenant: &TenantContext,
    media_id: Uuid,
    storage_keys: &[String],
) -> Result<(), AppError> {
    let mut tx = rls::begin_tenant_transaction(&state.db, tenant).await?;
    sqlx::query(
        "UPDATE media SET lifecycle_status = 'failed', updated_at = now() WHERE id = $1 AND organization_id = $2",
    )
    .bind(media_id)
    .bind(tenant.organization_id)
    .execute(&mut *tx)
    .await?;
    file_cleanup::enqueue_media_cleanup(
        &mut tx,
        tenant.organization_id,
        media_id,
        storage_keys,
        "publish_rollback",
    )
    .await?;
    tx.commit().await?;
    let _ = file_cleanup::process_tenant_cleanup_jobs(
        state,
        tenant,
        state.config.security_cleanup_batch_size.clamp(1, 100),
    )
    .await;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/media/{id}",
    tag = "media",
    params(("id" = Uuid, Path, description = "Media id")),
    responses((status = 200, description = "Media detail", body = MediaDetailResponse))
)]
pub async fn get_media(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaDetailResponse>, AppError> {
    load_media_detail(&state, &tenant, id).await.map(Json)
}

#[utoipa::path(
    put,
    path = "/api/media/{id}",
    tag = "media",
    params(("id" = Uuid, Path, description = "Media id")),
    request_body = MediaUpdateRequest,
    responses((status = 200, description = "Updated media", body = MediaDetailResponse))
)]
pub async fn update_media(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<MediaUpdateRequest>,
) -> Result<Json<MediaDetailResponse>, AppError> {
    rbac::require_org_media_writer(&tenant.role)?;
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let changed = sqlx::query(
        r#"
        UPDATE media
        SET alt_text = COALESCE($3, alt_text),
            caption = COALESCE($4, caption),
            updated_at = now()
        WHERE id = $1 AND organization_id = $2 AND lifecycle_status = 'active'
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .bind(clean_optional_text(payload.alt_text))
    .bind(clean_optional_text(payload.caption))
    .execute(db.as_mut())
    .await?
    .rows_affected();
    if changed == 0 {
        return Err(AppError::NotFound("media not found".to_owned()));
    }
    load_media_detail(&state, &tenant, id).await.map(Json)
}

#[utoipa::path(
    delete,
    path = "/api/media/{id}",
    tag = "media",
    params(("id" = Uuid, Path, description = "Media id")),
    responses((status = 200, description = "Media queued for deletion", body = MediaDetailResponse))
)]
pub async fn delete_media(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaDetailResponse>, AppError> {
    rbac::require_org_any(&tenant.role, &[rbac::ORG_ADMIN, rbac::ORG_EDITOR])?;
    let detail = load_media_detail(&state, &tenant, id).await?;
    let mut tx = rls::begin_tenant_transaction(&state.db, &tenant).await?;
    let storage_keys = sqlx::query_scalar::<_, String>(
        r#"
        SELECT storage_key FROM media WHERE id = $1 AND organization_id = $2
        UNION ALL
        SELECT storage_key FROM media_variants WHERE media_id = $1 AND organization_id = $2
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_all(&mut *tx)
    .await?;
    let changed = sqlx::query(
        "UPDATE media SET lifecycle_status = 'deletion_pending', deleted_at = now(), updated_at = now() WHERE id = $1 AND organization_id = $2 AND lifecycle_status = 'active'",
    )
    .bind(id)
    .bind(tenant.organization_id)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if changed == 0 {
        return Err(AppError::NotFound("media not found".to_owned()));
    }
    sqlx::query(
        "UPDATE media_variants SET lifecycle_status = 'deletion_pending' WHERE media_id = $1 AND organization_id = $2",
    )
    .bind(id)
    .bind(tenant.organization_id)
    .execute(&mut *tx)
    .await?;
    file_cleanup::enqueue_media_cleanup(
        &mut tx,
        tenant.organization_id,
        id,
        &storage_keys,
        "delete",
    )
    .await?;
    tx.commit().await?;
    let _ = file_cleanup::process_tenant_cleanup_jobs(
        &state,
        &tenant,
        state.config.security_cleanup_batch_size.clamp(1, 100),
    )
    .await;

    audit::record(
        &state.db,
        &tenant,
        "media.delete",
        "media",
        Some(detail.media.id),
        json!({
            "filename": detail.media.filename,
            "storage_objects": storage_keys.len(),
            "cleanup": "durable_job",
        }),
    )
    .await?;
    Ok(Json(detail))
}

#[utoipa::path(
    get,
    path = "/api/media/{id}/download",
    tag = "media",
    params(("id" = Uuid, Path, description = "Media id")),
    responses(
        (status = 200, description = "Authorized private attachment"),
        (status = 416, description = "Range requests are unsupported")
    )
)]
pub async fn download_media(
    State(state): State<AppState>,
    Extension(tenant): Extension<TenantContext>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let mut db = rls::tenant_connection(&state.db, &tenant).await?;
    let row = sqlx::query_as::<_, FileDeliveryRow>(
        r#"
        SELECT filename, mime_type, storage_key
        FROM media
        WHERE id = $1 AND organization_id = $2
          AND visibility = 'restricted'
          AND lifecycle_status = 'active'
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_optional(db.as_mut())
    .await?
    .ok_or_else(|| AppError::NotFound("media not found".to_owned()))?;
    stream_file_response(&state, row, &headers, false).await
}

pub async fn deliver_public_media(
    State(state): State<AppState>,
    Path(path): Path<String>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let storage_key = format!("public/{path}");
    secure_join(FsPath::new(&state.config.upload_dir), &storage_key)
        .map_err(|_| AppError::NotFound("media not found".to_owned()))?;
    let mut tx = rls::begin_bypass_transaction(&state.db).await?;
    let row = sqlx::query_as::<_, FileDeliveryRow>(
        r#"
        SELECT media.filename, media.mime_type, media.storage_key
        FROM media
        WHERE media.storage_key = $1
          AND media.visibility = 'public'
          AND media.verification_status = 'verified'
          AND media.lifecycle_status = 'active'
        UNION ALL
        SELECT media.filename, 'image/webp', variant.storage_key
        FROM media_variants variant
        JOIN media ON media.id = variant.media_id
        WHERE variant.storage_key = $1
          AND variant.verification_status = 'verified'
          AND variant.lifecycle_status = 'active'
          AND media.visibility = 'public'
          AND media.verification_status = 'verified'
          AND media.lifecycle_status = 'active'
        LIMIT 1
        "#,
    )
    .bind(&storage_key)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    let row = row.ok_or_else(|| AppError::NotFound("media not found".to_owned()))?;
    stream_file_response(&state, row, &headers, true).await
}

async fn stream_file_response(
    state: &AppState,
    row: FileDeliveryRow,
    request_headers: &HeaderMap,
    public_inline: bool,
) -> Result<Response, AppError> {
    let path = secure_join_no_symlinks(FsPath::new(&state.config.upload_dir), &row.storage_key)
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let file = File::open(&path)
        .await
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => AppError::NotFound("media not found".to_owned()),
            _ => AppError::Internal(error.to_string()),
        })?;
    let length = file
        .metadata()
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?
        .len();
    if request_headers.contains_key(RANGE) {
        return Response::builder()
            .status(StatusCode::RANGE_NOT_SATISFIABLE)
            .header(CONTENT_RANGE, format!("bytes */{length}"))
            .header(ACCEPT_RANGES, "none")
            .body(Body::empty())
            .map_err(|error| AppError::Internal(error.to_string()));
    }

    let disposition = content_disposition(
        if public_inline && row.mime_type.starts_with("image/") {
            "inline"
        } else {
            "attachment"
        },
        &row.filename,
    )
    .map_err(|error| AppError::Internal(error.to_string()))?;
    let content_type = HeaderValue::from_str(&row.mime_type)
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let cache_control = if public_inline {
        PUBLIC_CACHE_CONTROL
    } else {
        PRIVATE_CACHE_CONTROL
    };
    let mut builder = Response::builder();
    builder = builder
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, content_type)
        .header(CONTENT_LENGTH, length)
        .header(CONTENT_DISPOSITION, disposition)
        .header(CACHE_CONTROL, cache_control)
        .header(ACCEPT_RANGES, "none")
        .header("x-content-type-options", "nosniff")
        .header("content-security-policy", "default-src 'none'; sandbox");
    if !public_inline {
        builder = builder.header(PRAGMA, "no-cache");
    }
    builder
        .body(Body::from_stream(ReaderStream::new(file)))
        .map_err(|error| AppError::Internal(error.to_string()))
}

async fn load_media_detail(
    state: &AppState,
    tenant: &TenantContext,
    id: Uuid,
) -> Result<MediaDetailResponse, AppError> {
    let mut db = rls::tenant_connection(&state.db, tenant).await?;
    let media = sqlx::query_as::<_, MediaResponse>(
        r#"
        SELECT id, filename, url, mime_type, size, alt_text, caption, uploader_id,
               visibility, verification_status, lifecycle_status, created_at, updated_at
        FROM media
        WHERE id = $1 AND organization_id = $2 AND lifecycle_status = 'active'
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_one(db.as_mut())
    .await?;
    let variants = sqlx::query_as::<_, MediaVariantResponse>(
        r#"
        SELECT id, media_id, variant_name, url, width, height, created_at
        FROM media_variants
        WHERE media_id = $1 AND organization_id = $2 AND lifecycle_status = 'active'
        ORDER BY variant_name
        "#,
    )
    .bind(id)
    .bind(tenant.organization_id)
    .fetch_all(db.as_mut())
    .await?;
    Ok(MediaDetailResponse { media, variants })
}

fn clean_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim();
        if value.is_empty() {
            None
        } else {
            Some(value.chars().take(4_096).collect())
        }
    })
}

async fn read_text_field_bounded(
    field: &mut axum::extract::multipart::Field<'_>,
    limit: usize,
) -> Result<String, AppError> {
    let mut bytes = Vec::with_capacity(limit.min(1_024));
    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|error| AppError::BadRequest(error.to_string()))?
    {
        if bytes.len().saturating_add(chunk.len()) > limit {
            return Err(AppError::Validation(
                "multipart text field exceeds the allowed size".to_owned(),
            ));
        }
        bytes.extend_from_slice(&chunk);
    }
    String::from_utf8(bytes)
        .map_err(|_| AppError::Validation("multipart text field must be UTF-8".to_owned()))
}

#[cfg(test)]
mod tests {
    use super::clean_optional_text;

    #[test]
    fn metadata_is_trimmed_and_bounded() {
        assert_eq!(
            clean_optional_text(Some("  caption  ".to_owned())).as_deref(),
            Some("caption")
        );
        assert_eq!(
            clean_optional_text(Some("x".repeat(5_000)))
                .expect("metadata")
                .len(),
            4_096
        );
    }
}
