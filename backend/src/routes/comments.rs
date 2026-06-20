use axum::extract::{Extension, Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::rbac;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/comments", get(list_comments).post(create_comment))
        .route(
            "/api/comments/{id}/resolve",
            post(resolve_comment).delete(unresolve_comment),
        )
        .route(
            "/api/comments/{id}",
            get(get_comment).delete(delete_comment),
        )
}

#[derive(Debug, Deserialize)]
pub struct CommentListQuery {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub include_resolved: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CommentRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub body: String,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct CommentResponse {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub body: String,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/api/comments",
    tag = "comments",
    responses((status = 200, description = "Entity comments", body = [CommentResponse]))
)]
pub async fn list_comments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<CommentListQuery>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    rbac::require_comment_reader(&claims)?;
    validate_entity_type(&query.entity_type)?;

    let rows = if query.include_resolved.unwrap_or(false) {
        sqlx::query_as::<_, CommentResponse>(
            r#"
            SELECT c.id,
                   c.entity_type,
                   c.entity_id,
                   c.body,
                   c.author_id,
                   u.name as author_name,
                   c.resolved_at,
                   c.resolved_by,
                   c.created_at,
                   c.updated_at
            FROM comments c
            LEFT JOIN users u ON u.id = c.author_id
            WHERE c.entity_type = $1 AND c.entity_id = $2
            ORDER BY c.created_at DESC
            "#,
        )
        .bind(&query.entity_type)
        .bind(query.entity_id)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, CommentResponse>(
            r#"
            SELECT c.id,
                   c.entity_type,
                   c.entity_id,
                   c.body,
                   c.author_id,
                   u.name as author_name,
                   c.resolved_at,
                   c.resolved_by,
                   c.created_at,
                   c.updated_at
            FROM comments c
            LEFT JOIN users u ON u.id = c.author_id
            WHERE c.entity_type = $1 AND c.entity_id = $2 AND c.resolved_at IS NULL
            ORDER BY c.created_at DESC
            "#,
        )
        .bind(&query.entity_type)
        .bind(query.entity_id)
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(rows))
}

#[utoipa::path(
    post,
    path = "/api/comments",
    tag = "comments",
    request_body = CommentRequest,
    responses((status = 200, description = "Created comment", body = CommentResponse))
)]
pub async fn create_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CommentRequest>,
) -> Result<Json<CommentResponse>, AppError> {
    rbac::require_comment_writer(&claims)?;
    validate_comment_request(&payload)?;
    ensure_entity_exists(&state, &payload.entity_type, payload.entity_id).await?;

    let row = sqlx::query_as::<_, CommentResponse>(
        r#"
        INSERT INTO comments (entity_type, entity_id, body, author_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id,
                  entity_type,
                  entity_id,
                  body,
                  author_id,
                  NULL::text as author_name,
                  resolved_at,
                  resolved_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(payload.entity_type)
    .bind(payload.entity_id)
    .bind(payload.body.trim())
    .bind(claims.sub)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    get,
    path = "/api/comments/{id}",
    tag = "comments",
    params(("id" = Uuid, Path, description = "Comment id")),
    responses((status = 200, description = "Comment", body = CommentResponse))
)]
pub async fn get_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<CommentResponse>, AppError> {
    rbac::require_comment_reader(&claims)?;
    load_comment(&state, id).await.map(Json)
}

#[utoipa::path(
    post,
    path = "/api/comments/{id}/resolve",
    tag = "comments",
    params(("id" = Uuid, Path, description = "Comment id")),
    responses((status = 200, description = "Resolved comment", body = CommentResponse))
)]
pub async fn resolve_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<CommentResponse>, AppError> {
    rbac::require_comment_manager(&claims)?;
    let row = sqlx::query_as::<_, CommentResponse>(
        r#"
        UPDATE comments
        SET resolved_at = now(),
            resolved_by = $2,
            updated_at = now()
        WHERE id = $1
        RETURNING id,
                  entity_type,
                  entity_id,
                  body,
                  author_id,
                  NULL::text as author_name,
                  resolved_at,
                  resolved_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .bind(claims.sub)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/api/comments/{id}/resolve",
    tag = "comments",
    params(("id" = Uuid, Path, description = "Comment id")),
    responses((status = 200, description = "Unresolved comment", body = CommentResponse))
)]
pub async fn unresolve_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<CommentResponse>, AppError> {
    rbac::require_comment_manager(&claims)?;
    let row = sqlx::query_as::<_, CommentResponse>(
        r#"
        UPDATE comments
        SET resolved_at = NULL,
            resolved_by = NULL,
            updated_at = now()
        WHERE id = $1
        RETURNING id,
                  entity_type,
                  entity_id,
                  body,
                  author_id,
                  NULL::text as author_name,
                  resolved_at,
                  resolved_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row))
}

#[utoipa::path(
    delete,
    path = "/api/comments/{id}",
    tag = "comments",
    params(("id" = Uuid, Path, description = "Comment id")),
    responses((status = 200, description = "Deleted comment", body = CommentResponse))
)]
pub async fn delete_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<CommentResponse>, AppError> {
    rbac::require_comment_manager(&claims)?;
    let row = sqlx::query_as::<_, CommentResponse>(
        r#"
        DELETE FROM comments
        WHERE id = $1
        RETURNING id,
                  entity_type,
                  entity_id,
                  body,
                  author_id,
                  NULL::text as author_name,
                  resolved_at,
                  resolved_by,
                  created_at,
                  updated_at
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row))
}

async fn load_comment(state: &AppState, id: Uuid) -> Result<CommentResponse, AppError> {
    sqlx::query_as::<_, CommentResponse>(
        r#"
        SELECT c.id,
               c.entity_type,
               c.entity_id,
               c.body,
               c.author_id,
               u.name as author_name,
               c.resolved_at,
               c.resolved_by,
               c.created_at,
               c.updated_at
        FROM comments c
        LEFT JOIN users u ON u.id = c.author_id
        WHERE c.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)
}

async fn ensure_entity_exists(
    state: &AppState,
    entity_type: &str,
    entity_id: Uuid,
) -> Result<(), AppError> {
    match entity_type {
        "entry" => {
            sqlx::query_scalar::<_, Uuid>("SELECT id FROM content_entries WHERE id = $1")
                .bind(entity_id)
                .fetch_one(&state.db)
                .await?;
        }
        "page" => {
            sqlx::query_scalar::<_, Uuid>("SELECT id FROM pages WHERE id = $1")
                .bind(entity_id)
                .fetch_one(&state.db)
                .await?;
        }
        _ => validate_entity_type(entity_type)?,
    }
    Ok(())
}

fn validate_comment_request(payload: &CommentRequest) -> Result<(), AppError> {
    validate_entity_type(&payload.entity_type)?;
    if payload.body.trim().is_empty() {
        return Err(AppError::Validation("comment body is required".to_owned()));
    }
    Ok(())
}

fn validate_entity_type(entity_type: &str) -> Result<(), AppError> {
    match entity_type {
        "entry" | "page" => Ok(()),
        other => Err(AppError::Validation(format!(
            "entity_type '{other}' is not supported"
        ))),
    }
}
