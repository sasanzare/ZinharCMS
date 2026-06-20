use std::fmt::Write as _;

use axum::extract::{Path, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::cache::{self, DEFAULT_TTL_SECONDS};
use crate::services::entry_validation::is_valid_slug;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/content/{type_slug}", get(list_public_entries))
        .route(
            "/api/v1/content/{type_slug}/{id_or_slug}",
            get(get_public_entry),
        )
        .route("/api/v1/pages", get(list_public_pages))
        .route("/api/v1/pages/{slug}", get(get_public_page))
        .route("/api/v1/settings/public", get(public_settings))
        .route("/api/v1/navigation", get(public_navigation))
        .route("/api/v1/sitemap.xml", get(sitemap))
        .route("/api/v1/robots.txt", get(robots))
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeliveryListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
    pub locale: Option<String>,
    pub author_id: Option<Uuid>,
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeliveryDetailQuery {
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NavigationQuery {
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicEntryResponse {
    pub id: Uuid,
    pub type_slug: String,
    pub data: Value,
    pub version: i32,
    pub published_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PublicEntryListResponse {
    pub data: Vec<PublicEntryResponse>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicPageResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub page_json: Value,
    pub metadata: Value,
    pub published_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PublicPageListResponse {
    pub data: Vec<PublicPageResponse>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct NavigationItemResponse {
    pub id: Uuid,
    pub label: String,
    pub url: String,
    pub parent_id: Option<Uuid>,
    pub position: i32,
    pub locale: String,
}

#[derive(Debug, FromRow)]
struct PublicSettingRow {
    key: String,
    value: Value,
}

#[derive(Debug, FromRow)]
struct SitemapPageRow {
    slug: String,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct SitemapEntryRow {
    type_slug: String,
    slug: String,
    updated_at: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/api/v1/content/{type_slug}",
    tag = "delivery",
    params(("type_slug" = String, Path, description = "Content type slug")),
    responses((status = 200, description = "Published entries", body = PublicEntryListResponse))
)]
pub async fn list_public_entries(
    State(state): State<AppState>,
    Path(type_slug): Path<String>,
    Query(query): Query<DeliveryListQuery>,
) -> Result<Json<PublicEntryListResponse>, AppError> {
    if !is_valid_slug(&type_slug) {
        return Err(AppError::Validation("type_slug is invalid".to_owned()));
    }
    let normalized = NormalizedListQuery::from(query)?;
    let cache_key = format!("delivery:content:{type_slug}:{}", normalized.cache_suffix());
    let db = state.db.clone();
    let type_slug_for_fetch = type_slug.clone();
    let fetch_query = normalized.clone();
    let response = cache::get_or_set_json(
        &state.redis,
        &cache_key,
        DEFAULT_TTL_SECONDS,
        || async move { fetch_public_entries(&db, &type_slug_for_fetch, &fetch_query).await },
    )
    .await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/content/{type_slug}/{id_or_slug}",
    tag = "delivery",
    params(
        ("type_slug" = String, Path, description = "Content type slug"),
        ("id_or_slug" = String, Path, description = "Entry UUID or data.slug")
    ),
    responses((status = 200, description = "Published entry", body = PublicEntryResponse))
)]
pub async fn get_public_entry(
    State(state): State<AppState>,
    Path((type_slug, id_or_slug)): Path<(String, String)>,
    Query(query): Query<DeliveryDetailQuery>,
) -> Result<Json<PublicEntryResponse>, AppError> {
    if !is_valid_slug(&type_slug) {
        return Err(AppError::Validation("type_slug is invalid".to_owned()));
    }
    if Uuid::parse_str(&id_or_slug).is_err() && !is_valid_slug(&id_or_slug) {
        return Err(AppError::Validation(
            "id_or_slug must be a UUID or slug".to_owned(),
        ));
    }

    let locale = normalize_locale(query.locale)?;
    let locale_key = locale.as_deref().unwrap_or("all");
    let cache_key = format!("delivery:content:{type_slug}:detail:{id_or_slug}:locale={locale_key}");
    let db = state.db.clone();
    let type_slug_for_fetch = type_slug.clone();
    let id_or_slug_for_fetch = id_or_slug.clone();
    let response = cache::get_or_set_json(
        &state.redis,
        &cache_key,
        DEFAULT_TTL_SECONDS,
        || async move {
            fetch_public_entry(
                &db,
                &type_slug_for_fetch,
                &id_or_slug_for_fetch,
                locale.as_deref(),
            )
            .await
        },
    )
    .await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/pages",
    tag = "delivery",
    responses((status = 200, description = "Published pages", body = PublicPageListResponse))
)]
pub async fn list_public_pages(
    State(state): State<AppState>,
    Query(query): Query<DeliveryListQuery>,
) -> Result<Json<PublicPageListResponse>, AppError> {
    let normalized = NormalizedListQuery::from(query)?;
    let cache_key = format!("delivery:pages:{}", normalized.cache_suffix());
    let db = state.db.clone();
    let fetch_query = normalized.clone();
    let response = cache::get_or_set_json(
        &state.redis,
        &cache_key,
        DEFAULT_TTL_SECONDS,
        || async move { fetch_public_pages(&db, &fetch_query).await },
    )
    .await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/pages/{slug}",
    tag = "delivery",
    params(("slug" = String, Path, description = "Page slug")),
    responses((status = 200, description = "Published page", body = PublicPageResponse))
)]
pub async fn get_public_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicPageResponse>, AppError> {
    if !is_valid_slug(&slug) {
        return Err(AppError::Validation("slug is invalid".to_owned()));
    }

    let cache_key = format!("delivery:page:{slug}");
    let db = state.db.clone();
    let slug_for_fetch = slug.clone();
    let response = cache::get_or_set_json(
        &state.redis,
        &cache_key,
        DEFAULT_TTL_SECONDS,
        || async move { fetch_public_page(&db, &slug_for_fetch).await },
    )
    .await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/settings/public",
    tag = "delivery",
    responses((status = 200, description = "Public settings", body = Object))
)]
pub async fn public_settings(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    let db = state.db.clone();
    let response = cache::get_or_set_json(
        &state.redis,
        "delivery:settings:public",
        DEFAULT_TTL_SECONDS,
        || async move { fetch_public_settings(&db).await },
    )
    .await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/navigation",
    tag = "delivery",
    responses((status = 200, description = "Public navigation", body = [NavigationItemResponse]))
)]
pub async fn public_navigation(
    State(state): State<AppState>,
    Query(query): Query<NavigationQuery>,
) -> Result<Json<Vec<NavigationItemResponse>>, AppError> {
    let locale = normalize_locale(query.locale)?;
    let locale_key = locale.as_deref().unwrap_or("all");
    let cache_key = format!("delivery:navigation:{locale_key}");
    let db = state.db.clone();
    let response = cache::get_or_set_json(
        &state.redis,
        &cache_key,
        DEFAULT_TTL_SECONDS,
        || async move { fetch_navigation(&db, locale.as_deref()).await },
    )
    .await?;

    Ok(Json(response))
}

pub async fn sitemap(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let db = state.db.clone();
    let body = cache::get_or_set_json(
        &state.redis,
        "delivery:sitemap",
        DEFAULT_TTL_SECONDS,
        || async move { build_sitemap(&db).await },
    )
    .await?;

    Ok(([(CONTENT_TYPE, "application/xml; charset=utf-8")], body))
}

pub async fn robots(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let db = state.db.clone();
    let body = cache::get_or_set_json(
        &state.redis,
        "delivery:robots",
        DEFAULT_TTL_SECONDS,
        || async move { build_robots(&db).await },
    )
    .await?;

    Ok(([(CONTENT_TYPE, "text/plain; charset=utf-8")], body))
}

pub async fn invalidate_content_cache(state: &AppState, type_slug: &str) {
    cache::invalidate_prefix(&state.redis, &format!("delivery:content:{type_slug}:")).await;
    cache::invalidate(&state.redis, "delivery:sitemap").await;
}

pub async fn invalidate_page_cache(state: &AppState) {
    cache::invalidate_prefix(&state.redis, "delivery:page:").await;
    cache::invalidate_prefix(&state.redis, "delivery:pages:").await;
    cache::invalidate(&state.redis, "delivery:sitemap").await;
    cache::invalidate(&state.redis, "delivery:robots").await;
}

#[derive(Clone)]
struct NormalizedListQuery {
    page: i64,
    per_page: i64,
    sort_column: &'static str,
    sort_direction: &'static str,
    locale: Option<String>,
    author_id: Option<Uuid>,
    filter: Option<(String, String)>,
}

impl NormalizedListQuery {
    fn from(query: DeliveryListQuery) -> Result<Self, AppError> {
        let (sort_column, sort_direction) = parse_sort(query.sort.as_deref())?;
        Ok(Self {
            page: query.page.unwrap_or(1).max(1),
            per_page: query.per_page.unwrap_or(20).clamp(1, 100),
            sort_column,
            sort_direction,
            locale: normalize_locale(query.locale)?,
            author_id: query.author_id,
            filter: parse_filter(query.filter.as_deref())?,
        })
    }

    fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }

    fn cache_suffix(&self) -> String {
        let filter = self
            .filter
            .as_ref()
            .map(|(field, value)| format!("{field}={value}"))
            .unwrap_or_else(|| "none".to_owned());
        format!(
            "page={}:per_page={}:sort={}:{}:locale={}:author_id={}:filter={}",
            self.page,
            self.per_page,
            self.sort_column,
            self.sort_direction,
            self.locale.as_deref().unwrap_or("all"),
            self.author_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "all".to_owned()),
            filter
        )
    }
}

async fn fetch_public_entries(
    db: &PgPool,
    type_slug: &str,
    query: &NormalizedListQuery,
) -> Result<PublicEntryListResponse, AppError> {
    let mut builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT e.id,
               ct.slug as type_slug,
               e.data,
               e.version,
               e.published_at,
               e.updated_at
        FROM content_entries e
        JOIN content_types ct ON ct.id = e.type_id
        WHERE ct.slug =
        "#,
    );
    builder.push_bind(type_slug);
    builder.push(" AND e.status = 'published'::content_status");
    push_entry_filters(&mut builder, query);
    builder
        .push(" ORDER BY e.")
        .push(query.sort_column)
        .push(" ")
        .push(query.sort_direction)
        .push(" LIMIT ")
        .push_bind(query.per_page)
        .push(" OFFSET ")
        .push_bind(query.offset());

    let data = builder
        .build_query_as::<PublicEntryResponse>()
        .fetch_all(db)
        .await?;

    Ok(PublicEntryListResponse {
        data,
        page: query.page,
        per_page: query.per_page,
    })
}

async fn fetch_public_entry(
    db: &PgPool,
    type_slug: &str,
    id_or_slug: &str,
    locale: Option<&str>,
) -> Result<PublicEntryResponse, AppError> {
    let uuid = Uuid::parse_str(id_or_slug).ok();
    let mut builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT e.id,
               ct.slug as type_slug,
               e.data,
               e.version,
               e.published_at,
               e.updated_at
        FROM content_entries e
        JOIN content_types ct ON ct.id = e.type_id
        WHERE ct.slug =
        "#,
    );
    builder.push_bind(type_slug);
    builder.push(" AND e.status = 'published'::content_status");
    if let Some(uuid) = uuid {
        builder.push(" AND e.id = ");
        builder.push_bind(uuid);
    } else {
        builder.push(" AND e.data ->> 'slug' = ");
        builder.push_bind(id_or_slug);
    }
    if let Some(locale) = locale {
        builder.push(" AND e.data ->> 'locale' = ");
        builder.push_bind(locale);
    }

    builder
        .build_query_as::<PublicEntryResponse>()
        .fetch_one(db)
        .await
        .map_err(AppError::from)
}

async fn fetch_public_pages(
    db: &PgPool,
    query: &NormalizedListQuery,
) -> Result<PublicPageListResponse, AppError> {
    let sort_column = match query.sort_column {
        "created_at" => "created_at",
        "updated_at" => "updated_at",
        "published_at" => "published_at",
        _ => "title",
    };
    let mut builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT id,
               title,
               slug,
               page_json,
               COALESCE(page_json->'metadata', '{}'::jsonb) as metadata,
               published_at,
               updated_at
        FROM pages
        WHERE status = 'published'::page_status
        "#,
    );
    if let Some(locale) = query.locale.as_deref() {
        builder.push(" AND page_json->'metadata'->>'locale' = ");
        builder.push_bind(locale);
    }
    builder
        .push(" ORDER BY ")
        .push(sort_column)
        .push(" ")
        .push(query.sort_direction)
        .push(" LIMIT ")
        .push_bind(query.per_page)
        .push(" OFFSET ")
        .push_bind(query.offset());

    let data = builder
        .build_query_as::<PublicPageResponse>()
        .fetch_all(db)
        .await?;

    Ok(PublicPageListResponse {
        data,
        page: query.page,
        per_page: query.per_page,
    })
}

async fn fetch_public_page(db: &PgPool, slug: &str) -> Result<PublicPageResponse, AppError> {
    sqlx::query_as::<_, PublicPageResponse>(
        r#"
        SELECT id,
               title,
               slug,
               page_json,
               COALESCE(page_json->'metadata', '{}'::jsonb) as metadata,
               published_at,
               updated_at
        FROM pages
        WHERE slug = $1 AND status = 'published'::page_status
        "#,
    )
    .bind(slug)
    .fetch_one(db)
    .await
    .map_err(AppError::from)
}

async fn fetch_public_settings(db: &PgPool) -> Result<Value, AppError> {
    let rows = sqlx::query_as::<_, PublicSettingRow>(
        r#"
        SELECT key, value
        FROM public_settings
        WHERE is_public = TRUE
        ORDER BY key ASC
        "#,
    )
    .fetch_all(db)
    .await?;
    let mut object = Map::new();
    for row in rows {
        object.insert(row.key, row.value);
    }
    Ok(Value::Object(object))
}

async fn fetch_navigation(
    db: &PgPool,
    locale: Option<&str>,
) -> Result<Vec<NavigationItemResponse>, AppError> {
    let mut builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT id, label, url, parent_id, position, locale
        FROM navigation_items
        WHERE is_public = TRUE
        "#,
    );
    if let Some(locale) = locale {
        builder.push(" AND locale = ");
        builder.push_bind(locale);
    }
    builder.push(" ORDER BY locale ASC, position ASC, label ASC");

    builder
        .build_query_as::<NavigationItemResponse>()
        .fetch_all(db)
        .await
        .map_err(AppError::from)
}

fn push_entry_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    query: &'a NormalizedListQuery,
) {
    if let Some(locale) = query.locale.as_deref() {
        builder.push(" AND e.data ->> 'locale' = ");
        builder.push_bind(locale);
    }
    if let Some(author_id) = query.author_id {
        builder.push(" AND e.author_id = ");
        builder.push_bind(author_id);
    }
    if let Some((field, value)) = query.filter.as_ref() {
        builder.push(" AND e.data ->> ");
        builder.push_bind(field);
        builder.push(" = ");
        builder.push_bind(value);
    }
}

async fn build_sitemap(db: &PgPool) -> Result<String, AppError> {
    let site_url = load_site_url(db).await?;
    let pages = sqlx::query_as::<_, SitemapPageRow>(
        r#"
        SELECT slug, updated_at
        FROM pages
        WHERE status = 'published'::page_status
        ORDER BY slug ASC
        "#,
    )
    .fetch_all(db)
    .await?;
    let entries = sqlx::query_as::<_, SitemapEntryRow>(
        r#"
        SELECT ct.slug as type_slug,
               e.data->>'slug' as slug,
               e.updated_at
        FROM content_entries e
        JOIN content_types ct ON ct.id = e.type_id
        WHERE e.status = 'published'::content_status
          AND e.data ? 'slug'
          AND length(e.data->>'slug') > 0
        ORDER BY ct.slug ASC, slug ASC
        "#,
    )
    .fetch_all(db)
    .await?;

    let mut body = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    body.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    for page in pages {
        let path = if page.slug == "home" {
            "/".to_owned()
        } else {
            format!("/{}", page.slug)
        };
        push_sitemap_url(&mut body, &site_url, &path, page.updated_at);
    }
    for entry in entries {
        let path = format!("/{}/{}", entry.type_slug, entry.slug);
        push_sitemap_url(&mut body, &site_url, &path, entry.updated_at);
    }
    body.push_str("</urlset>");

    Ok(body)
}

async fn build_robots(db: &PgPool) -> Result<String, AppError> {
    let site_url = load_site_url(db).await?;
    Ok(format!(
        "User-agent: *\nAllow: /\nSitemap: {site_url}/api/v1/sitemap.xml\n"
    ))
}

async fn load_site_url(db: &PgPool) -> Result<String, AppError> {
    let row = sqlx::query_as::<_, PublicSettingRow>(
        r#"
        SELECT key, value
        FROM public_settings
        WHERE key = 'site_url' AND is_public = TRUE
        "#,
    )
    .fetch_optional(db)
    .await?;
    let site_url = row
        .and_then(|row| row.value.as_str().map(ToOwned::to_owned))
        .unwrap_or_else(|| "http://localhost:5173".to_owned());
    Ok(site_url.trim_end_matches('/').to_owned())
}

fn push_sitemap_url(body: &mut String, site_url: &str, path: &str, updated_at: DateTime<Utc>) {
    let _ = write!(
        body,
        "<url><loc>{}{}</loc><lastmod>{}</lastmod></url>",
        xml_escape(site_url),
        xml_escape(path),
        updated_at.to_rfc3339()
    );
}

fn parse_sort(sort: Option<&str>) -> Result<(&'static str, &'static str), AppError> {
    let Some(sort) = sort else {
        return Ok(("created_at", "DESC"));
    };
    let (field, direction) = sort.split_once(':').unwrap_or((sort, "desc"));
    let field = match field {
        "created_at" => "created_at",
        "updated_at" => "updated_at",
        "published_at" => "published_at",
        "title" => "title",
        other => {
            return Err(AppError::Validation(format!(
                "sort field '{other}' is not supported"
            )));
        }
    };
    let direction = match direction.to_ascii_lowercase().as_str() {
        "asc" => "ASC",
        "desc" => "DESC",
        other => {
            return Err(AppError::Validation(format!(
                "sort direction '{other}' is not supported"
            )));
        }
    };
    Ok((field, direction))
}

fn parse_filter(filter: Option<&str>) -> Result<Option<(String, String)>, AppError> {
    let Some(filter) = filter else {
        return Ok(None);
    };
    let (field, value) = filter
        .split_once('=')
        .or_else(|| filter.split_once(':'))
        .ok_or_else(|| AppError::Validation("filter must use field=value".to_owned()))?;
    if !is_valid_filter_field(field) || value.trim().is_empty() {
        return Err(AppError::Validation(
            "filter must use a JSON field name and non-empty value".to_owned(),
        ));
    }
    Ok(Some((field.trim().to_owned(), value.trim().to_owned())))
}

fn normalize_locale(locale: Option<String>) -> Result<Option<String>, AppError> {
    let Some(locale) = locale else {
        return Ok(None);
    };
    let locale = locale.trim();
    if locale.is_empty() {
        return Ok(None);
    }
    let valid = locale
        .chars()
        .all(|ch| ch.is_ascii_alphabetic() || ch == '-')
        && (2..=8).contains(&locale.len());
    if !valid {
        return Err(AppError::Validation("locale is invalid".to_owned()));
    }
    Ok(Some(locale.to_owned()))
}

fn is_valid_filter_field(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() || first == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_filter_as_field_value() {
        assert_eq!(
            parse_filter(Some("slug=welcome")).unwrap(),
            Some(("slug".to_owned(), "welcome".to_owned()))
        );
    }

    #[test]
    fn rejects_invalid_filter_field() {
        assert!(parse_filter(Some("bad-field=value")).is_err());
    }

    #[test]
    fn xml_escape_handles_reserved_chars() {
        assert_eq!(xml_escape("a&b<c>"), "a&amp;b&lt;c&gt;");
    }
}
