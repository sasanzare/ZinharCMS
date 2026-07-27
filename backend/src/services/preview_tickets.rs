use std::collections::HashSet;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::Utc;
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::AppError;

pub const PREVIEW_APPLICATION_PROTOCOL: &str = "zinhar.preview.v1";
pub const PREVIEW_TICKET_AUDIENCE: &str = "preview-websocket";
const PREVIEW_TICKET_PROTOCOL_PREFIX: &str = "zinhar.ticket.";
const PREVIEW_TICKET_KEY_PREFIX: &str = "zinhar:preview-ticket:v1:";
const PREVIEW_RATE_KEY_PREFIX: &str = "zinhar:preview-ticket-rate:v1:";
const RAW_TICKET_BYTES: usize = 32;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketRecord {
    pub audience: String,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub page_id: Uuid,
    pub auth_version: i64,
    pub issued_at: i64,
    pub expires_at: i64,
}

impl TicketRecord {
    pub fn validate_scope(&self, requested_page_id: Uuid, now: i64) -> Result<(), AppError> {
        if self.audience != PREVIEW_TICKET_AUDIENCE
            || self.page_id != requested_page_id
            || self.issued_at > now
            || self.expires_at < now
            || self.expires_at.saturating_sub(self.issued_at) > 60
        {
            return Err(AppError::Unauthorized(
                "preview ticket is invalid".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct IssuedPreviewTicket {
    pub raw_ticket: String,
    pub expires_at: i64,
}

pub fn validate_raw_ticket(raw_ticket: &str) -> Result<(), AppError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(raw_ticket)
        .map_err(|_| AppError::Unauthorized("preview ticket is invalid".to_owned()))?;
    if bytes.len() != RAW_TICKET_BYTES
        || !raw_ticket
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(AppError::Unauthorized(
            "preview ticket is invalid".to_owned(),
        ));
    }
    Ok(())
}

pub fn hash_ticket(raw_ticket: &str) -> Result<String, AppError> {
    validate_raw_ticket(raw_ticket)?;
    Ok(URL_SAFE_NO_PAD.encode(Sha256::digest(raw_ticket.as_bytes())))
}

pub fn parse_preview_protocols(values: &[String]) -> Result<String, AppError> {
    let protocols = values
        .iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    let stable_count = protocols
        .iter()
        .filter(|protocol| **protocol == PREVIEW_APPLICATION_PROTOCOL)
        .count();
    let tickets = protocols
        .iter()
        .filter_map(|protocol| protocol.strip_prefix(PREVIEW_TICKET_PROTOCOL_PREFIX))
        .collect::<Vec<_>>();
    let supported_count = stable_count + tickets.len();
    if stable_count != 1 || tickets.len() != 1 || supported_count != protocols.len() {
        return Err(AppError::Unauthorized(
            "preview WebSocket protocols are invalid".to_owned(),
        ));
    }
    validate_raw_ticket(tickets[0])?;
    Ok(tickets[0].to_owned())
}

pub fn canonical_origin(value: &str) -> Option<String> {
    if value == "null" {
        return None;
    }
    let url = reqwest::Url::parse(value).ok()?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.path() != "/"
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return None;
    }
    Some(url.origin().ascii_serialization())
}

pub fn parse_allowed_origins(value: &str) -> Result<HashSet<String>, String> {
    let origins = value
        .split(',')
        .map(str::trim)
        .map(|candidate| {
            canonical_origin(candidate)
                .ok_or_else(|| "preview WebSocket origin allowlist is invalid".to_owned())
        })
        .collect::<Result<HashSet<_>, _>>()?;
    if origins.is_empty() {
        return Err("preview WebSocket origin allowlist must not be empty".to_owned());
    }
    Ok(origins)
}

pub fn request_origin_is_allowed(
    origin: Option<&str>,
    allowed_origins: &HashSet<String>,
    allow_missing: bool,
) -> bool {
    match origin {
        Some(value) => {
            canonical_origin(value).is_some_and(|origin| allowed_origins.contains(&origin))
        }
        None => allow_missing,
    }
}

pub async fn issue_ticket(
    redis: &redis::Client,
    record: TicketRecord,
    ttl_seconds: u64,
    rate_limit_per_minute: i64,
) -> Result<IssuedPreviewTicket, AppError> {
    if !(1..=60).contains(&ttl_seconds) || rate_limit_per_minute < 1 {
        return Err(AppError::Internal(
            "preview ticket configuration is invalid".to_owned(),
        ));
    }
    enforce_rate_limit(redis, record.user_id, rate_limit_per_minute).await?;

    let mut connection = redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| preview_service_unavailable())?;
    let payload =
        serde_json::to_string(&record).map_err(|error| AppError::Internal(error.to_string()))?;

    for _ in 0..3 {
        let raw_ticket = generate_raw_ticket();
        let key = ticket_key(&raw_ticket)?;
        let stored: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(&payload)
            .arg("EX")
            .arg(ttl_seconds)
            .arg("NX")
            .query_async(&mut connection)
            .await
            .map_err(|_| preview_service_unavailable())?;
        if stored.as_deref() == Some("OK") {
            return Ok(IssuedPreviewTicket {
                raw_ticket,
                expires_at: record.expires_at,
            });
        }
    }

    Err(AppError::ServiceUnavailable(
        "preview ticket service is unavailable".to_owned(),
    ))
}

pub async fn consume_ticket(
    redis: &redis::Client,
    raw_ticket: &str,
) -> Result<Option<TicketRecord>, AppError> {
    let key = ticket_key(raw_ticket)?;
    let mut connection = redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| preview_service_unavailable())?;
    let payload: Option<String> = redis::cmd("GETDEL")
        .arg(key)
        .query_async(&mut connection)
        .await
        .map_err(|_| preview_service_unavailable())?;
    payload
        .map(|payload| {
            serde_json::from_str(&payload)
                .map_err(|_| AppError::Unauthorized("preview ticket is invalid".to_owned()))
        })
        .transpose()
}

pub fn new_ticket_record(
    user_id: Uuid,
    organization_id: Uuid,
    page_id: Uuid,
    auth_version: i64,
    ttl_seconds: u64,
) -> Result<TicketRecord, AppError> {
    if !(1..=60).contains(&ttl_seconds) {
        return Err(AppError::Internal(
            "preview ticket lifetime is invalid".to_owned(),
        ));
    }
    let issued_at = Utc::now().timestamp();
    let ttl = i64::try_from(ttl_seconds)
        .map_err(|_| AppError::Internal("preview ticket lifetime is invalid".to_owned()))?;
    Ok(TicketRecord {
        audience: PREVIEW_TICKET_AUDIENCE.to_owned(),
        user_id,
        organization_id,
        page_id,
        auth_version,
        issued_at,
        expires_at: issued_at.saturating_add(ttl),
    })
}

fn generate_raw_ticket() -> String {
    let mut bytes = [0_u8; RAW_TICKET_BYTES];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn ticket_key(raw_ticket: &str) -> Result<String, AppError> {
    Ok(format!(
        "{PREVIEW_TICKET_KEY_PREFIX}{}",
        hash_ticket(raw_ticket)?
    ))
}

async fn enforce_rate_limit(
    redis: &redis::Client,
    user_id: Uuid,
    rate_limit_per_minute: i64,
) -> Result<(), AppError> {
    let minute = Utc::now().timestamp().div_euclid(60);
    let key = format!("{PREVIEW_RATE_KEY_PREFIX}{user_id}:{minute}");
    let mut connection = redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| preview_service_unavailable())?;
    let count: i64 = redis::Script::new(
        r#"
        local count = redis.call('INCR', KEYS[1])
        if count == 1 then redis.call('EXPIRE', KEYS[1], ARGV[1]) end
        return count
        "#,
    )
    .key(key)
    .arg(60_i64)
    .invoke_async(&mut connection)
    .await
    .map_err(|_| preview_service_unavailable())?;
    if count > rate_limit_per_minute {
        return Err(AppError::TooManyRequests(
            "preview ticket rate limit exceeded".to_owned(),
        ));
    }
    Ok(())
}

fn preview_service_unavailable() -> AppError {
    AppError::ServiceUnavailable("preview ticket service is unavailable".to_owned())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{
        PREVIEW_APPLICATION_PROTOCOL, TicketRecord, consume_ticket, hash_ticket, issue_ticket,
        new_ticket_record, parse_preview_protocols, request_origin_is_allowed, ticket_key,
        validate_raw_ticket,
    };
    use redis::AsyncCommands;
    use uuid::Uuid;

    use crate::error::AppError;

    #[test]
    fn opaque_tickets_are_validated_and_hashed_without_retaining_the_secret() {
        let raw = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        assert!(validate_raw_ticket(raw).is_ok());
        let hash = hash_ticket(raw).unwrap();
        assert_ne!(hash, raw);
        assert!(!hash.contains(raw));
        assert!(validate_raw_ticket("short").is_err());
        assert!(validate_raw_ticket("bad+alphabet/").is_err());
    }

    #[test]
    fn websocket_protocol_parser_requires_one_stable_and_one_ticket_protocol() {
        let raw = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let parsed = parse_preview_protocols(&[format!(
            "{PREVIEW_APPLICATION_PROTOCOL}, zinhar.ticket.{raw}"
        )])
        .unwrap();
        assert_eq!(parsed, raw);

        assert!(parse_preview_protocols(&[PREVIEW_APPLICATION_PROTOCOL.to_owned()]).is_err());
        assert!(parse_preview_protocols(&[format!("zinhar.ticket.{raw}")]).is_err());
        assert!(
            parse_preview_protocols(&[format!(
                "{PREVIEW_APPLICATION_PROTOCOL}, zinhar.ticket.{raw}, zinhar.ticket.{raw}"
            )])
            .is_err()
        );
        assert!(
            parse_preview_protocols(&[format!(
                "{PREVIEW_APPLICATION_PROTOCOL}, zinhar.unsupported.v1, zinhar.ticket.{raw}"
            )])
            .is_err()
        );
    }

    #[test]
    fn websocket_origin_policy_is_exact_and_fails_closed() {
        let allowed = HashSet::from([
            "https://cms.example.invalid".to_owned(),
            "http://localhost:5173".to_owned(),
        ]);

        assert!(request_origin_is_allowed(
            Some("https://cms.example.invalid"),
            &allowed,
            false
        ));
        assert!(request_origin_is_allowed(
            Some("http://localhost:5173"),
            &allowed,
            false
        ));
        assert!(!request_origin_is_allowed(
            Some("https://evil.example.invalid"),
            &allowed,
            false
        ));
        assert!(!request_origin_is_allowed(Some("null"), &allowed, false));
        assert!(!request_origin_is_allowed(
            Some("https://cms.example.invalid/path"),
            &allowed,
            false
        ));
        assert!(!request_origin_is_allowed(None, &allowed, false));
        assert!(request_origin_is_allowed(None, &allowed, true));
    }

    #[test]
    fn ticket_record_scope_is_bound_to_the_expected_audience_and_resource() {
        let record = TicketRecord {
            audience: "preview-websocket".to_owned(),
            user_id: Uuid::now_v7(),
            organization_id: Uuid::now_v7(),
            page_id: Uuid::now_v7(),
            auth_version: 7,
            issued_at: 100,
            expires_at: 130,
        };

        assert!(record.validate_scope(record.page_id, 110).is_ok());
        assert!(record.validate_scope(Uuid::now_v7(), 110).is_err());
        assert!(record.validate_scope(record.page_id, 131).is_err());
        let wrong_audience = TicketRecord {
            audience: "another-service".to_owned(),
            ..record
        };
        assert!(
            wrong_audience
                .validate_scope(wrong_audience.page_id, 110)
                .is_err()
        );
    }

    #[tokio::test]
    async fn redis_ticket_storage_is_hash_only_expiring_and_single_use() {
        let Ok(redis_url) = std::env::var("PHASE3_TEST_REDIS_URL") else {
            return;
        };
        let client = redis::Client::open(redis_url).unwrap();
        let user_id = Uuid::now_v7();
        let record = new_ticket_record(user_id, Uuid::now_v7(), Uuid::now_v7(), 4, 30).unwrap();
        let issued = issue_ticket(&client, record.clone(), 30, 10).await.unwrap();
        let key = ticket_key(&issued.raw_ticket).unwrap();
        let mut connection = client.get_multiplexed_async_connection().await.unwrap();
        let stored: String = connection.get(&key).await.unwrap();
        assert!(!stored.contains(&issued.raw_ticket));
        let ttl: i64 = connection.ttl(&key).await.unwrap();
        assert!((1..=30).contains(&ttl));

        let (first, second) = tokio::join!(
            consume_ticket(&client, &issued.raw_ticket),
            consume_ticket(&client, &issued.raw_ticket),
        );
        let successful = [first.unwrap(), second.unwrap()]
            .into_iter()
            .filter(Option::is_some)
            .count();
        assert_eq!(successful, 1);
        assert!(
            consume_ticket(&client, &issued.raw_ticket)
                .await
                .unwrap()
                .is_none()
        );

        let minute = chrono::Utc::now().timestamp().div_euclid(60);
        let rate_key = format!("zinhar:preview-ticket-rate:v1:{user_id}:{minute}");
        let _: usize = connection.del(rate_key).await.unwrap();

        let limited_user = Uuid::now_v7();
        let limited_record =
            new_ticket_record(limited_user, Uuid::now_v7(), Uuid::now_v7(), 4, 30).unwrap();
        let limited = issue_ticket(&client, limited_record.clone(), 30, 1)
            .await
            .unwrap();
        let rate_limited = issue_ticket(&client, limited_record, 30, 1)
            .await
            .unwrap_err();
        assert!(matches!(rate_limited, AppError::TooManyRequests(_)));
        let _: usize = connection
            .del(ticket_key(&limited.raw_ticket).unwrap())
            .await
            .unwrap();
        let limited_rate_key = format!("zinhar:preview-ticket-rate:v1:{limited_user}:{minute}");
        let _: usize = connection.del(limited_rate_key).await.unwrap();

        let expiring_user = Uuid::now_v7();
        let expiring_record =
            new_ticket_record(expiring_user, Uuid::now_v7(), Uuid::now_v7(), 4, 30).unwrap();
        let expiring = issue_ticket(&client, expiring_record, 30, 10)
            .await
            .unwrap();
        let expiring_key = ticket_key(&expiring.raw_ticket).unwrap();
        let _: bool = connection.pexpire(&expiring_key, 1).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        assert!(
            consume_ticket(&client, &expiring.raw_ticket)
                .await
                .unwrap()
                .is_none()
        );
        let expiring_rate_key = format!("zinhar:preview-ticket-rate:v1:{expiring_user}:{minute}");
        let _: usize = connection.del(expiring_rate_key).await.unwrap();
    }

    #[tokio::test]
    async fn redis_failure_fails_ticket_issuance_closed() {
        let client = redis::Client::open("redis://127.0.0.1:0").unwrap();
        let record =
            new_ticket_record(Uuid::now_v7(), Uuid::now_v7(), Uuid::now_v7(), 1, 30).unwrap();
        let error = issue_ticket(&client, record, 30, 10).await.unwrap_err();
        assert!(matches!(error, AppError::ServiceUnavailable(_)));
    }
}
