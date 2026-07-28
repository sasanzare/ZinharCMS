use std::collections::HashSet;
use std::env;

use ipnet::IpNet;
use serde::Deserialize;
use thiserror::Error;

pub const JWT_CLOCK_SKEW_SECONDS: i64 = 30;
const MAX_JWT_KEY_COUNT: usize = 8;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JwtKeyStatus {
    Active,
    Previous,
    Retired,
}

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct JwtKeyConfig {
    pub kid: String,
    pub algorithm: String,
    pub status: JwtKeyStatus,
    pub(crate) secret: String,
    pub verify_until: Option<i64>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct JwtKeyRing {
    keys: Vec<JwtKeyConfig>,
}

impl JwtKeyRing {
    pub fn active(&self) -> &JwtKeyConfig {
        self.keys
            .iter()
            .find(|key| key.status == JwtKeyStatus::Active)
            .expect("validated JWT key rings always contain one active key")
    }

    pub fn verification_key(&self, kid: &str, now: i64) -> Option<&JwtKeyConfig> {
        self.keys.iter().find(|key| {
            key.kid == kid
                && match key.status {
                    JwtKeyStatus::Active => true,
                    JwtKeyStatus::Previous => key
                        .verify_until
                        .is_some_and(|verify_until| verify_until >= now),
                    JwtKeyStatus::Retired => false,
                }
        })
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_key_ring: JwtKeyRing,
    pub bootstrap_admin_email: Option<String>,
    pub bootstrap_admin_password: Option<String>,
    pub jwt_access_expiry: u64,
    pub jwt_refresh_expiry: u64,
    pub upload_dir: String,
    pub max_upload_size: u64,
    pub cors_origin: String,
    pub cookie_secure: bool,
    pub preview_ws_allowed_origins: String,
    pub preview_ticket_ttl_seconds: u64,
    pub preview_ticket_rate_limit_per_minute: i64,
    pub preview_revalidation_interval_seconds: u64,
    pub login_rate_limit_max_failures: i64,
    pub login_rate_limit_window_seconds: i64,
    pub trusted_proxy_cidrs: Vec<IpNet>,
    pub stripe_secret_key: Option<String>,
    pub stripe_webhook_secret: Option<String>,
    pub stripe_success_url: String,
    pub stripe_cancel_url: String,
    pub stripe_portal_return_url: String,
    pub stripe_pro_price_id: Option<String>,
    pub stripe_enterprise_price_id: Option<String>,
    pub app_base_url: String,
    pub email_provider: String,
    pub email_from: String,
    pub email_webhook_url: Option<String>,
    pub email_failure_mode: String,
    pub organization_rate_limit_per_minute: i64,
    pub organization_user_rate_limit_per_minute: i64,
    pub organization_rate_limit_burst: i64,
    pub security_cleanup_batch_size: i64,
    pub expired_session_retention_days: i64,
    pub revoked_session_retention_days: i64,
    pub compromised_session_retention_days: i64,
    pub security_token_retention_days: i64,
    pub security_audit_retention_days: i64,
    pub login_attempt_retention_days: i64,
    pub port: u16,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    Missing(&'static str),
    #[error("invalid value for {name}: {value}")]
    Invalid { name: &'static str, value: String },
    #[error("invalid JWT_KEY_RING: {0}")]
    InvalidJwtKeyRing(&'static str),
    #[error("BOOTSTRAP_ADMIN_EMAIL and BOOTSTRAP_ADMIN_PASSWORD must be set together")]
    IncompleteBootstrapAdmin,
    #[error("BOOTSTRAP_ADMIN_EMAIL must be a valid email address")]
    InvalidBootstrapAdminEmail,
    #[error(
        "BOOTSTRAP_ADMIN_PASSWORD must be at least 12 characters and must not be a placeholder"
    )]
    WeakBootstrapAdminPassword,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let jwt_access_expiry = parse_u64("JWT_ACCESS_EXPIRY", 3600)?;
        let jwt_key_ring = parse_jwt_key_ring(
            &get("JWT_KEY_RING", None)?,
            jwt_access_expiry,
            chrono::Utc::now().timestamp(),
        )?;
        let bootstrap_admin_email =
            get_optional("BOOTSTRAP_ADMIN_EMAIL").map(|email| email.trim().to_ascii_lowercase());
        let bootstrap_admin_password = get_optional("BOOTSTRAP_ADMIN_PASSWORD");
        validate_bootstrap_admin(
            bootstrap_admin_email.as_deref(),
            bootstrap_admin_password.as_deref(),
        )?;
        let cors_origin = get("CORS_ORIGIN", Some("http://localhost:5173"))?;
        let preview_revalidation_interval_seconds =
            parse_u64("PREVIEW_REVALIDATION_INTERVAL_SECONDS", 30)?;
        if !(30..=60).contains(&preview_revalidation_interval_seconds) {
            return Err(ConfigError::Invalid {
                name: "PREVIEW_REVALIDATION_INTERVAL_SECONDS",
                value: preview_revalidation_interval_seconds.to_string(),
            });
        }
        let preview_ticket_ttl_seconds = parse_u64("PREVIEW_TICKET_TTL_SECONDS", 30)?;
        if !(1..=60).contains(&preview_ticket_ttl_seconds) {
            return Err(ConfigError::Invalid {
                name: "PREVIEW_TICKET_TTL_SECONDS",
                value: preview_ticket_ttl_seconds.to_string(),
            });
        }
        let preview_ticket_rate_limit_per_minute =
            parse_i64("PREVIEW_TICKET_RATE_LIMIT_PER_MINUTE", 30)?;
        if preview_ticket_rate_limit_per_minute < 1 {
            return Err(ConfigError::Invalid {
                name: "PREVIEW_TICKET_RATE_LIMIT_PER_MINUTE",
                value: preview_ticket_rate_limit_per_minute.to_string(),
            });
        }
        let security_cleanup_batch_size = parse_i64("SECURITY_CLEANUP_BATCH_SIZE", 500)?;
        let expired_session_retention_days = parse_i64("EXPIRED_SESSION_RETENTION_DAYS", 30)?;
        let revoked_session_retention_days = parse_i64("REVOKED_SESSION_RETENTION_DAYS", 30)?;
        let compromised_session_retention_days =
            parse_i64("COMPROMISED_SESSION_RETENTION_DAYS", 180)?;
        let security_token_retention_days = parse_i64("SECURITY_TOKEN_RETENTION_DAYS", 7)?;
        let security_audit_retention_days = parse_i64("SECURITY_AUDIT_RETENTION_DAYS", 365)?;
        let login_attempt_retention_days = parse_i64("LOGIN_ATTEMPT_RETENTION_DAYS", 30)?;
        if !(1..=5000).contains(&security_cleanup_batch_size)
            || expired_session_retention_days < 1
            || revoked_session_retention_days < 1
            || compromised_session_retention_days < revoked_session_retention_days
            || security_token_retention_days < 1
            || security_audit_retention_days < 90
            || login_attempt_retention_days < 1
        {
            return Err(ConfigError::Invalid {
                name: "SECURITY_RETENTION_POLICY",
                value: "outside allowed bounds".to_owned(),
            });
        }

        Ok(Self {
            database_url: get("DATABASE_URL", None)?,
            redis_url: get("REDIS_URL", Some("redis://localhost:6379"))?,
            jwt_key_ring,
            bootstrap_admin_email,
            bootstrap_admin_password,
            jwt_access_expiry,
            jwt_refresh_expiry: parse_u64("JWT_REFRESH_EXPIRY", 604_800)?,
            upload_dir: get("UPLOAD_DIR", Some("./uploads"))?,
            max_upload_size: parse_u64("MAX_UPLOAD_SIZE", 52_428_800)?,
            preview_ws_allowed_origins: get("PREVIEW_WS_ALLOWED_ORIGINS", Some(&cors_origin))?,
            cors_origin,
            cookie_secure: parse_bool("COOKIE_SECURE", false)?,
            preview_ticket_ttl_seconds,
            preview_ticket_rate_limit_per_minute,
            preview_revalidation_interval_seconds,
            login_rate_limit_max_failures: parse_i64("LOGIN_RATE_LIMIT_MAX_FAILURES", 5)?,
            login_rate_limit_window_seconds: parse_i64("LOGIN_RATE_LIMIT_WINDOW_SECONDS", 900)?,
            trusted_proxy_cidrs: parse_trusted_proxy_cidrs(
                &env::var("TRUSTED_PROXY_CIDRS").unwrap_or_default(),
            )?,
            stripe_secret_key: get_optional("STRIPE_SECRET_KEY"),
            stripe_webhook_secret: get_optional("STRIPE_WEBHOOK_SECRET"),
            stripe_success_url: get(
                "STRIPE_SUCCESS_URL",
                Some("http://localhost:5173/billing?billing=success"),
            )?,
            stripe_cancel_url: get(
                "STRIPE_CANCEL_URL",
                Some("http://localhost:5173/billing?billing=cancelled"),
            )?,
            stripe_portal_return_url: get(
                "STRIPE_PORTAL_RETURN_URL",
                Some("http://localhost:5173/billing"),
            )?,
            stripe_pro_price_id: get_optional("STRIPE_PRO_PRICE_ID"),
            stripe_enterprise_price_id: get_optional("STRIPE_ENTERPRISE_PRICE_ID"),
            app_base_url: get("APP_BASE_URL", Some("http://localhost:5173"))?,
            email_provider: get("EMAIL_PROVIDER", Some("log"))?.to_ascii_lowercase(),
            email_from: get("EMAIL_FROM", Some("ZinharCMS <noreply@localhost>"))?,
            email_webhook_url: get_optional("EMAIL_WEBHOOK_URL"),
            email_failure_mode: get("EMAIL_FAILURE_MODE", Some("log"))?.to_ascii_lowercase(),
            organization_rate_limit_per_minute: parse_i64("ORG_RATE_LIMIT_PER_MINUTE", 600)?,
            organization_user_rate_limit_per_minute: parse_i64(
                "ORG_USER_RATE_LIMIT_PER_MINUTE",
                120,
            )?,
            organization_rate_limit_burst: parse_i64("ORG_RATE_LIMIT_BURST", 120)?,
            security_cleanup_batch_size,
            expired_session_retention_days,
            revoked_session_retention_days,
            compromised_session_retention_days,
            security_token_retention_days,
            security_audit_retention_days,
            login_attempt_retention_days,
            port: parse_u16("PORT", 8080)?,
        })
    }
}

fn parse_jwt_key_ring(
    value: &str,
    access_token_lifetime_seconds: u64,
    now: i64,
) -> Result<JwtKeyRing, ConfigError> {
    let keys: Vec<JwtKeyConfig> = serde_json::from_str(value)
        .map_err(|_| ConfigError::InvalidJwtKeyRing("must be a valid JSON array"))?;
    if keys.is_empty() || keys.len() > MAX_JWT_KEY_COUNT {
        return Err(ConfigError::InvalidJwtKeyRing(
            "must contain between one and eight keys",
        ));
    }

    let maximum_previous_window = i64::try_from(access_token_lifetime_seconds)
        .ok()
        .and_then(|lifetime| lifetime.checked_add(JWT_CLOCK_SKEW_SECONDS))
        .and_then(|window| now.checked_add(window))
        .ok_or(ConfigError::InvalidJwtKeyRing(
            "access-token lifetime is too large",
        ))?;
    let mut kids = HashSet::with_capacity(keys.len());
    let mut active_count = 0;

    for key in &keys {
        if key.kid.is_empty()
            || key.kid.len() > 64
            || !key
                .kid
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(ConfigError::InvalidJwtKeyRing(
                "kid must use 1-64 ASCII letters, digits, dots, underscores, or hyphens",
            ));
        }
        if !kids.insert(key.kid.as_str()) {
            return Err(ConfigError::InvalidJwtKeyRing("kid values must be unique"));
        }
        if key.algorithm != "HS256" {
            return Err(ConfigError::InvalidJwtKeyRing(
                "only the HS256 algorithm is supported",
            ));
        }
        if key.secret.len() < 32 || is_placeholder_secret(&key.secret) {
            return Err(ConfigError::InvalidJwtKeyRing(
                "key material must be at least 32 bytes and must not be a placeholder",
            ));
        }
        match key.status {
            JwtKeyStatus::Active => {
                active_count += 1;
                if key.verify_until.is_some() {
                    return Err(ConfigError::InvalidJwtKeyRing(
                        "the active key must not have verify_until",
                    ));
                }
            }
            JwtKeyStatus::Previous => {
                let verify_until = key.verify_until.ok_or(ConfigError::InvalidJwtKeyRing(
                    "a previous key requires verify_until",
                ))?;
                if verify_until > maximum_previous_window {
                    return Err(ConfigError::InvalidJwtKeyRing(
                        "a previous-key window cannot exceed the access-token lifetime plus clock skew",
                    ));
                }
            }
            JwtKeyStatus::Retired => {}
        }
    }

    if active_count != 1 {
        return Err(ConfigError::InvalidJwtKeyRing(
            "exactly one active key is required",
        ));
    }
    Ok(JwtKeyRing { keys })
}

fn get_optional(name: &'static str) -> Option<String> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => Some(value),
        _ => None,
    }
}

fn validate_bootstrap_admin(
    email: Option<&str>,
    password: Option<&str>,
) -> Result<(), ConfigError> {
    match (email, password) {
        (None, None) => Ok(()),
        (Some(_), None) | (None, Some(_)) => Err(ConfigError::IncompleteBootstrapAdmin),
        (Some(email), Some(password)) => {
            let valid_email = email.split_once('@').is_some_and(|(local, domain)| {
                !local.is_empty()
                    && !domain.is_empty()
                    && !domain.contains('@')
                    && !email.chars().any(char::is_whitespace)
            });
            if !valid_email {
                return Err(ConfigError::InvalidBootstrapAdminEmail);
            }
            if password.len() < 12 || is_placeholder_secret(password) {
                return Err(ConfigError::WeakBootstrapAdminPassword);
            }
            Ok(())
        }
    }
}

fn is_placeholder_secret(value: &str) -> bool {
    let normalized = value.trim().to_ascii_lowercase().replace(['-', ' '], "_");
    normalized.starts_with("change_me")
        || normalized.starts_with("changeme")
        || normalized.starts_with("replace_me")
        || normalized.starts_with("your_super_secret")
}

fn get(name: &'static str, default: Option<&str>) -> Result<String, ConfigError> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        _ => default
            .map(ToOwned::to_owned)
            .ok_or(ConfigError::Missing(name)),
    }
}

fn parse_u64(name: &'static str, default: u64) -> Result<u64, ConfigError> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => value
            .parse::<u64>()
            .map_err(|_| ConfigError::Invalid { name, value }),
        _ => Ok(default),
    }
}

fn parse_i64(name: &'static str, default: i64) -> Result<i64, ConfigError> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => value
            .parse::<i64>()
            .map_err(|_| ConfigError::Invalid { name, value }),
        _ => Ok(default),
    }
}

fn parse_bool(name: &'static str, default: bool) -> Result<bool, ConfigError> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => match value.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => Err(ConfigError::Invalid { name, value }),
        },
        _ => Ok(default),
    }
}

fn parse_trusted_proxy_cidrs(value: &str) -> Result<Vec<IpNet>, ConfigError> {
    if value.trim().is_empty() {
        return Ok(Vec::new());
    }

    value
        .split(',')
        .map(str::trim)
        .map(|candidate| {
            if candidate.is_empty() {
                return Err(ConfigError::Invalid {
                    name: "TRUSTED_PROXY_CIDRS",
                    value: value.to_owned(),
                });
            }
            candidate
                .parse::<IpNet>()
                .map_err(|_| ConfigError::Invalid {
                    name: "TRUSTED_PROXY_CIDRS",
                    value: candidate.to_owned(),
                })
        })
        .collect()
}

fn parse_u16(name: &'static str, default: u16) -> Result<u16, ConfigError> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => value
            .parse::<u16>()
            .map_err(|_| ConfigError::Invalid { name, value }),
        _ => Ok(default),
    }
}

#[cfg(test)]
impl Config {
    pub fn test_with_stripe_secret(webhook_secret: &str) -> Self {
        Self {
            database_url: "postgresql://localhost/test".to_owned(),
            redis_url: "redis://localhost:6379".to_owned(),
            jwt_key_ring: JwtKeyRing {
                keys: vec![JwtKeyConfig {
                    kid: "test-active".to_owned(),
                    algorithm: "HS256".to_owned(),
                    status: JwtKeyStatus::Active,
                    secret: "test-secret-with-at-least-32-characters".to_owned(),
                    verify_until: None,
                }],
            },
            bootstrap_admin_email: None,
            bootstrap_admin_password: None,
            jwt_access_expiry: 3600,
            jwt_refresh_expiry: 604_800,
            upload_dir: "./uploads".to_owned(),
            max_upload_size: 52_428_800,
            cors_origin: "http://localhost:5173".to_owned(),
            cookie_secure: false,
            preview_ws_allowed_origins: "http://localhost:5173".to_owned(),
            preview_ticket_ttl_seconds: 30,
            preview_ticket_rate_limit_per_minute: 30,
            preview_revalidation_interval_seconds: 30,
            login_rate_limit_max_failures: 5,
            login_rate_limit_window_seconds: 900,
            trusted_proxy_cidrs: Vec::new(),
            stripe_secret_key: Some("sk_test_local".to_owned()),
            stripe_webhook_secret: Some(webhook_secret.to_owned()),
            stripe_success_url: "http://localhost:5173/billing?billing=success".to_owned(),
            stripe_cancel_url: "http://localhost:5173/billing?billing=cancelled".to_owned(),
            stripe_portal_return_url: "http://localhost:5173/billing".to_owned(),
            stripe_pro_price_id: Some("price_pro_test".to_owned()),
            stripe_enterprise_price_id: Some("price_enterprise_test".to_owned()),
            app_base_url: "http://localhost:5173".to_owned(),
            email_provider: "log".to_owned(),
            email_from: "ZinharCMS <noreply@localhost>".to_owned(),
            email_webhook_url: None,
            email_failure_mode: "log".to_owned(),
            organization_rate_limit_per_minute: 600,
            organization_user_rate_limit_per_minute: 120,
            organization_rate_limit_burst: 120,
            security_cleanup_batch_size: 500,
            expired_session_retention_days: 30,
            revoked_session_retention_days: 30,
            compromised_session_retention_days: 180,
            security_token_retention_days: 7,
            security_audit_retention_days: 365,
            login_attempt_retention_days: 30,
            port: 8080,
        }
    }

    pub fn test_with_jwt_keys(keys: Vec<JwtKeyConfig>) -> Self {
        let mut config = Self::test_with_stripe_secret("test-webhook-secret");
        config.jwt_key_ring = JwtKeyRing { keys };
        config
    }
}

#[cfg(test)]
impl JwtKeyConfig {
    pub fn test(kid: &str, status: JwtKeyStatus, secret: &str, verify_until: Option<i64>) -> Self {
        Self {
            kid: kid.to_owned(),
            algorithm: "HS256".to_owned(),
            status,
            secret: secret.to_owned(),
            verify_until,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConfigError, JwtKeyStatus, is_placeholder_secret, parse_jwt_key_ring,
        parse_trusted_proxy_cidrs, validate_bootstrap_admin,
    };

    #[test]
    fn tracked_secret_placeholders_are_rejected() {
        assert!(is_placeholder_secret(
            "CHANGE_ME_WITH_A_RANDOM_SECRET_OF_AT_LEAST_32_CHARACTERS"
        ));
        assert!(is_placeholder_secret(
            "your-super-secret-jwt-key-min-32-chars"
        ));
        assert!(!is_placeholder_secret(
            "ci-only-jwt-signing-secret-not-for-production"
        ));
    }

    #[test]
    fn bootstrap_admin_requires_an_explicit_safe_pair() {
        assert!(validate_bootstrap_admin(None, None).is_ok());
        assert!(matches!(
            validate_bootstrap_admin(Some("owner@example.invalid"), None),
            Err(ConfigError::IncompleteBootstrapAdmin)
        ));
        assert!(matches!(
            validate_bootstrap_admin(
                Some("owner@example.invalid"),
                Some("CHANGE_ME_WITH_A_BOOTSTRAP_PASSWORD")
            ),
            Err(ConfigError::WeakBootstrapAdminPassword)
        ));
        assert!(matches!(
            validate_bootstrap_admin(
                Some("owner @example.invalid"),
                Some("local-only-bootstrap-password")
            ),
            Err(ConfigError::InvalidBootstrapAdminEmail)
        ));
        assert!(
            validate_bootstrap_admin(
                Some("owner@example.invalid"),
                Some("local-only-bootstrap-password")
            )
            .is_ok()
        );
    }

    #[test]
    fn trusted_proxy_configuration_rejects_invalid_cidrs() {
        assert!(parse_trusted_proxy_cidrs("").unwrap().is_empty());
        assert_eq!(
            parse_trusted_proxy_cidrs("10.0.0.0/8, 2001:db8::/32")
                .unwrap()
                .len(),
            2
        );
        assert!(matches!(
            parse_trusted_proxy_cidrs("10.0.0.0/99"),
            Err(ConfigError::Invalid { .. })
        ));
    }

    #[test]
    fn jwt_key_ring_requires_one_unique_strong_active_hs256_key() {
        let now = 1_800_000_000;
        let valid = r#"[
          {
            "kid": "key-2026-07",
            "algorithm": "HS256",
            "status": "active",
            "secret": "local-test-key-material-with-more-than-32-bytes"
          }
        ]"#;
        let ring = parse_jwt_key_ring(valid, 3600, now).unwrap();
        assert_eq!(ring.active().kid, "key-2026-07");
        assert_eq!(ring.active().status, JwtKeyStatus::Active);

        for rejected in [
            r#"[]"#,
            r#"[
              {"kid":"a","algorithm":"HS256","status":"active","secret":"first-local-test-key-material-over-32-bytes"},
              {"kid":"b","algorithm":"HS256","status":"active","secret":"second-local-test-key-material-over-32-bytes"}
            ]"#,
            r#"[
              {"kid":"duplicate","algorithm":"HS256","status":"active","secret":"first-local-test-key-material-over-32-bytes"},
              {"kid":"duplicate","algorithm":"HS256","status":"retired","secret":"second-local-test-key-material-over-32-bytes"}
            ]"#,
            r#"[
              {"kid":"weak","algorithm":"HS256","status":"active","secret":"too-short"}
            ]"#,
            r#"[
              {"kid":"placeholder","algorithm":"HS256","status":"active","secret":"CHANGE_ME_WITH_A_RANDOM_SECRET_OF_AT_LEAST_32_CHARACTERS"}
            ]"#,
            r#"[
              {"kid":"wrong-alg","algorithm":"RS256","status":"active","secret":"local-test-key-material-with-more-than-32-bytes"}
            ]"#,
        ] {
            assert!(parse_jwt_key_ring(rejected, 3600, now).is_err());
        }
    }

    #[test]
    fn previous_jwt_key_has_a_bounded_verification_window() {
        let now = 1_800_000_000;
        let valid = format!(
            r#"[
              {{"kid":"active","algorithm":"HS256","status":"active","secret":"active-local-test-key-material-over-32-bytes"}},
              {{"kid":"previous","algorithm":"HS256","status":"previous","secret":"previous-local-test-key-material-over-32-bytes","verify_until":{}}}
            ]"#,
            now + 3630
        );
        assert!(parse_jwt_key_ring(&valid, 3600, now).is_ok());

        let unbounded = format!(
            r#"[
              {{"kid":"active","algorithm":"HS256","status":"active","secret":"active-local-test-key-material-over-32-bytes"}},
              {{"kid":"previous","algorithm":"HS256","status":"previous","secret":"previous-local-test-key-material-over-32-bytes","verify_until":{}}}
            ]"#,
            now + 3631
        );
        assert!(parse_jwt_key_ring(&unbounded, 3600, now).is_err());
    }
}
