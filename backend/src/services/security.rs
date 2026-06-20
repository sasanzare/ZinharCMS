use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use serde_json::Value;
use sqlx::PgPool;

use crate::error::AppError;
use crate::services::entry_validation::FieldSchemaDocument;

pub const LOGIN_RATE_LIMIT_MAX_FAILURES: i64 = 5;
pub const LOGIN_RATE_LIMIT_WINDOW_SECONDS: i64 = 15 * 60;

pub async fn require_login_allowed(
    db: &PgPool,
    ip_address: &str,
    max_failures: i64,
    window_seconds: i64,
) -> Result<(), AppError> {
    let since = Utc::now() - Duration::seconds(window_seconds);
    let failed_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM login_attempts
        WHERE ip_address = $1
          AND succeeded = FALSE
          AND attempted_at >= $2
        "#,
    )
    .bind(ip_address)
    .bind(since)
    .fetch_one(db)
    .await?;

    if failed_count >= max_failures {
        Err(AppError::TooManyRequests(
            "too many failed login attempts; try again later".to_owned(),
        ))
    } else {
        Ok(())
    }
}
pub async fn record_login_attempt(
    db: &PgPool,
    email: &str,
    ip_address: &str,
    succeeded: bool,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO login_attempts (email, ip_address, succeeded)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(email)
    .bind(ip_address)
    .bind(succeeded)
    .execute(db)
    .await?;

    Ok(())
}

pub fn client_ip(headers: &HeaderMap, fallback: std::net::IpAddr) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| value.parse::<std::net::IpAddr>().is_ok())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| fallback.to_string())
}

pub fn sanitize_entry_data(fields: &FieldSchemaDocument, data: Value) -> Value {
    let Some(mut object) = data.as_object().cloned() else {
        return data;
    };

    for field in fields
        .fields
        .iter()
        .filter(|field| field.field_type == "richtext")
    {
        if let Some(value) = object.get(&field.name).and_then(Value::as_str) {
            object.insert(field.name.clone(), Value::String(sanitize_richtext(value)));
        }
    }

    Value::Object(object)
}

pub fn sanitize_richtext(value: &str) -> String {
    let without_blocks = ["script", "style", "iframe", "object", "embed"]
        .into_iter()
        .fold(value.to_owned(), remove_tag_block);
    sanitize_tags(&without_blocks)
}

fn remove_tag_block(input: String, tag: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let lower = input.to_ascii_lowercase();
    let mut cursor = 0;
    let start_pattern = format!("<{tag}");
    let end_pattern = format!("</{tag}>");

    while let Some(relative_start) = lower[cursor..].find(&start_pattern) {
        let start = cursor + relative_start;
        output.push_str(&input[cursor..start]);
        let search_from = start + start_pattern.len();
        if let Some(relative_end) = lower[search_from..].find(&end_pattern) {
            cursor = search_from + relative_end + end_pattern.len();
        } else {
            cursor = input.len();
            break;
        }
    }

    output.push_str(&input[cursor..]);
    output
}

fn sanitize_tags(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(relative_start) = input[cursor..].find('<') {
        let start = cursor + relative_start;
        output.push_str(&input[cursor..start]);
        let Some(relative_end) = input[start..].find('>') else {
            output.push_str("&lt;");
            cursor = start + 1;
            continue;
        };
        let end = start + relative_end;
        if let Some(tag) = sanitize_html_tag(&input[start + 1..end]) {
            output.push_str(&tag);
        }
        cursor = end + 1;
    }
    output.push_str(&input[cursor..]);
    output
}

fn sanitize_html_tag(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.starts_with('!') || trimmed.starts_with('?') {
        return None;
    }

    let closing = trimmed.starts_with('/');
    let tag_body = trimmed.trim_start_matches('/').trim_start();
    let tag_name = tag_body
        .split(|ch: char| ch.is_ascii_whitespace() || ch == '/')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let allowed = matches!(
        tag_name.as_str(),
        "a" | "b"
            | "blockquote"
            | "br"
            | "code"
            | "em"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "i"
            | "img"
            | "li"
            | "ol"
            | "p"
            | "pre"
            | "span"
            | "strong"
            | "u"
            | "ul"
    );
    if !allowed {
        return None;
    }

    if closing {
        Some(format!("</{tag_name}>"))
    } else {
        Some(format!("<{tag_name}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn richtext_removes_script_blocks() {
        assert_eq!(
            sanitize_richtext("<p>safe</p><script>alert(1)</script>"),
            "<p>safe</p>"
        );
    }

    #[test]
    fn richtext_strips_attributes() {
        assert_eq!(
            sanitize_richtext("<img src=x onerror=alert(1)><a href=\"javascript:alert(1)\">x</a>"),
            "<img><a>x</a>"
        );
    }
}
