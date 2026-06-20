use std::path::PathBuf;

use axum::extract::{Extension, Multipart, Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::fs;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::media_processing::{is_supported_image_mime, process_image_variants};
use crate::services::rbac;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/media", get(list_media))
        .route("/api/media/upload", post(upload_media))
        .route(
            "/api/media/{id}",
            get(get_media).put(update_media).delete(delete_media),
        )
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

#[utoipa::path(
    get,
    path = "/api/media",
    tag = "media",
    responses((status = 200, description = "Media library", body = MediaListResponse))
)]
pub async fn list_media(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(query): Query<MediaListQuery>,
) -> Result<Json<MediaListResponse>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let data = if let Some(mime_type) = query.mime_type.as_deref() {
        sqlx::query_as::<_, MediaResponse>(
            r#"
            SELECT id,
                   filename,
                   url,
                   mime_type,
                   size,
                   alt_text,
                   caption,
                   uploader_id,
                   created_at,
                   updated_at
            FROM media
            WHERE mime_type = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(mime_type)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, MediaResponse>(
            r#"
            SELECT id,
                   filename,
                   url,
                   mime_type,
                   size,
                   alt_text,
                   caption,
                   uploader_id,
                   created_at,
                   updated_at
            FROM media
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

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
    mut multipart: Multipart,
) -> Result<Json<MediaDetailResponse>, AppError> {
    rbac::require_media_writer(&claims)?;

    let mut alt_text: Option<String> = None;
    let mut caption: Option<String> = None;
    let mut upload: Option<IncomingUpload> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|error| AppError::BadRequest(error.to_string()))?
    {
        let name = field.name().unwrap_or_default().to_owned();
        match name.as_str() {
            "alt_text" => {
                alt_text = Some(
                    field
                        .text()
                        .await
                        .map_err(|error| AppError::BadRequest(error.to_string()))?,
                );
            }
            "caption" => {
                caption = Some(
                    field
                        .text()
                        .await
                        .map_err(|error| AppError::BadRequest(error.to_string()))?,
                );
            }
            "file" => {
                let filename = field
                    .file_name()
                    .map(sanitize_filename)
                    .unwrap_or_else(|| "upload.bin".to_owned());
                let mime_type = field
                    .content_type()
                    .map(str::to_owned)
                    .unwrap_or_else(|| "application/octet-stream".to_owned());
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|error| AppError::BadRequest(error.to_string()))?;
                upload = Some(IncomingUpload {
                    filename,
                    mime_type,
                    bytes: bytes.to_vec(),
                });
            }
            _ => {}
        }
    }

    let upload = upload.ok_or_else(|| AppError::Validation("file field is required".to_owned()))?;
    if upload.bytes.len() as u64 > state.config.max_upload_size {
        return Err(AppError::Validation(format!(
            "file exceeds maximum size of {} bytes",
            state.config.max_upload_size
        )));
    }
    let detected_mime_type = validate_upload_type(&upload)?;

    let upload_dir = PathBuf::from(&state.config.upload_dir);
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;
    fs::create_dir_all(upload_dir.join("variants"))
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;

    let file_id = Uuid::now_v7();
    let extension = extension_for_mime(detected_mime_type);
    let stored_filename = format!("{file_id}.{extension}");
    let path = upload_dir.join(&stored_filename);
    fs::write(&path, &upload.bytes)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;

    let url = format!("/uploads/{stored_filename}");
    let media = sqlx::query_as::<_, MediaResponse>(
        r#"
        INSERT INTO media (id, filename, url, mime_type, size, alt_text, caption, uploader_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id,
                  filename,
                  url,
                  mime_type,
                  size,
                  alt_text,
                  caption,
                  uploader_id,
                  created_at,
                  updated_at
        "#,
    )
    .bind(file_id)
    .bind(&upload.filename)
    .bind(&url)
    .bind(detected_mime_type)
    .bind(upload.bytes.len() as i64)
    .bind(clean_optional_text(alt_text))
    .bind(clean_optional_text(caption))
    .bind(claims.sub)
    .fetch_one(&state.db)
    .await?;

    let mut variants = Vec::new();
    if is_supported_image_mime(detected_mime_type) {
        let processed =
            process_image_variants(upload.bytes, &upload_dir, &file_id.to_string()).await?;
        for variant in processed {
            let row = sqlx::query_as::<_, MediaVariantResponse>(
                r#"
                INSERT INTO media_variants (media_id, variant_name, url, width, height)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, media_id, variant_name, url, width, height, created_at
                "#,
            )
            .bind(media.id)
            .bind(variant.name)
            .bind(variant.url)
            .bind(variant.width)
            .bind(variant.height)
            .fetch_one(&state.db)
            .await?;
            variants.push(row);
        }
    }

    Ok(Json(MediaDetailResponse { media, variants }))
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
    Extension(_claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaDetailResponse>, AppError> {
    load_media_detail(&state, id).await.map(Json)
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
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<MediaUpdateRequest>,
) -> Result<Json<MediaDetailResponse>, AppError> {
    rbac::require_media_writer(&claims)?;

    sqlx::query(
        r#"
        UPDATE media
        SET alt_text = COALESCE($2, alt_text),
            caption = COALESCE($3, caption),
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(clean_optional_text(payload.alt_text))
    .bind(clean_optional_text(payload.caption))
    .execute(&state.db)
    .await?;

    load_media_detail(&state, id).await.map(Json)
}

#[utoipa::path(
    delete,
    path = "/api/media/{id}",
    tag = "media",
    params(("id" = Uuid, Path, description = "Media id")),
    responses((status = 200, description = "Deleted media", body = MediaDetailResponse))
)]
pub async fn delete_media(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaDetailResponse>, AppError> {
    rbac::require_any(&claims, &[rbac::ADMIN, rbac::EDITOR])?;
    let detail = load_media_detail(&state, id).await?;

    sqlx::query("DELETE FROM media WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    remove_file_for_url(&state.config.upload_dir, &detail.media.url).await;
    for variant in &detail.variants {
        remove_file_for_url(&state.config.upload_dir, &variant.url).await;
    }

    Ok(Json(detail))
}

async fn load_media_detail(state: &AppState, id: Uuid) -> Result<MediaDetailResponse, AppError> {
    let media = sqlx::query_as::<_, MediaResponse>(
        r#"
        SELECT id,
               filename,
               url,
               mime_type,
               size,
               alt_text,
               caption,
               uploader_id,
               created_at,
               updated_at
        FROM media
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    let variants = sqlx::query_as::<_, MediaVariantResponse>(
        r#"
        SELECT id, media_id, variant_name, url, width, height, created_at
        FROM media_variants
        WHERE media_id = $1
        ORDER BY variant_name ASC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(MediaDetailResponse { media, variants })
}

struct IncomingUpload {
    filename: String,
    mime_type: String,
    bytes: Vec<u8>,
}

fn validate_upload_type(upload: &IncomingUpload) -> Result<&'static str, AppError> {
    let allowed = [
        "image/jpeg",
        "image/png",
        "image/webp",
        "application/pdf",
        "text/plain",
    ];
    let detected = detect_mime_type(&upload.bytes).ok_or_else(|| {
        AppError::Validation("file type could not be detected from content".to_owned())
    })?;
    if !allowed.contains(&detected) {
        return Err(AppError::Validation(format!(
            "detected mime type '{detected}' is not allowed"
        )));
    }
    if upload.mime_type != "application/octet-stream" && upload.mime_type != detected {
        return Err(AppError::Validation(format!(
            "declared mime type '{}' does not match detected type '{detected}'",
            upload.mime_type
        )));
    }
    Ok(detected)
}

fn detect_mime_type(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        return Some("image/jpeg");
    }
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some("image/png");
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    if bytes.starts_with(b"%PDF-") {
        return Some("application/pdf");
    }
    if !bytes.is_empty() && !bytes.contains(&0) && std::str::from_utf8(bytes).is_ok() {
        return Some("text/plain");
    }
    None
}

fn extension_for_mime(mime_type: &str) -> &'static str {
    match mime_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "application/pdf" => "pdf",
        "text/plain" => "txt",
        _ => "bin",
    }
}

fn sanitize_filename(filename: &str) -> String {
    let sanitized: String = filename
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
                ch
            } else {
                '-'
            }
        })
        .collect();

    let sanitized: String = sanitized
        .trim_matches(|ch| matches!(ch, '-' | '.'))
        .chars()
        .take(180)
        .collect();
    if sanitized.is_empty() {
        "upload.bin".to_owned()
    } else {
        sanitized
    }
}
fn clean_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim().to_owned();
        if value.is_empty() { None } else { Some(value) }
    })
}

async fn remove_file_for_url(upload_dir: &str, url: &str) {
    let Some(relative) = url.strip_prefix("/uploads/") else {
        return;
    };
    if relative.contains("..") {
        return;
    }
    let path = PathBuf::from(upload_dir).join(relative);
    let _ = fs::remove_file(path).await;
}
