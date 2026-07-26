use std::env;

use thiserror::Error;

#[derive(Clone, Eq, PartialEq)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub bootstrap_admin_email: Option<String>,
    pub bootstrap_admin_password: Option<String>,
    pub jwt_access_expiry: u64,
    pub jwt_refresh_expiry: u64,
    pub upload_dir: String,
    pub max_upload_size: u64,
    pub cors_origin: String,
    pub cookie_secure: bool,
    pub login_rate_limit_max_failures: i64,
    pub login_rate_limit_window_seconds: i64,
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
    pub port: u16,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    Missing(&'static str),
    #[error("invalid value for {name}: {value}")]
    Invalid { name: &'static str, value: String },
    #[error("JWT_SECRET must be at least 32 characters and must not be a placeholder")]
    WeakJwtSecret,
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
        let jwt_secret = get("JWT_SECRET", None)?;
        if jwt_secret.len() < 32 || is_placeholder_secret(&jwt_secret) {
            return Err(ConfigError::WeakJwtSecret);
        }
        let bootstrap_admin_email =
            get_optional("BOOTSTRAP_ADMIN_EMAIL").map(|email| email.trim().to_ascii_lowercase());
        let bootstrap_admin_password = get_optional("BOOTSTRAP_ADMIN_PASSWORD");
        validate_bootstrap_admin(
            bootstrap_admin_email.as_deref(),
            bootstrap_admin_password.as_deref(),
        )?;

        Ok(Self {
            database_url: get("DATABASE_URL", None)?,
            redis_url: get("REDIS_URL", Some("redis://localhost:6379"))?,
            jwt_secret,
            bootstrap_admin_email,
            bootstrap_admin_password,
            jwt_access_expiry: parse_u64("JWT_ACCESS_EXPIRY", 3600)?,
            jwt_refresh_expiry: parse_u64("JWT_REFRESH_EXPIRY", 604_800)?,
            upload_dir: get("UPLOAD_DIR", Some("./uploads"))?,
            max_upload_size: parse_u64("MAX_UPLOAD_SIZE", 52_428_800)?,
            cors_origin: get("CORS_ORIGIN", Some("http://localhost:5173"))?,
            cookie_secure: parse_bool("COOKIE_SECURE", false)?,
            login_rate_limit_max_failures: parse_i64("LOGIN_RATE_LIMIT_MAX_FAILURES", 5)?,
            login_rate_limit_window_seconds: parse_i64("LOGIN_RATE_LIMIT_WINDOW_SECONDS", 900)?,
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
            port: parse_u16("PORT", 8080)?,
        })
    }
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
            jwt_secret: "test-secret-with-at-least-32-characters".to_owned(),
            bootstrap_admin_email: None,
            bootstrap_admin_password: None,
            jwt_access_expiry: 3600,
            jwt_refresh_expiry: 604_800,
            upload_dir: "./uploads".to_owned(),
            max_upload_size: 52_428_800,
            cors_origin: "http://localhost:5173".to_owned(),
            cookie_secure: false,
            login_rate_limit_max_failures: 5,
            login_rate_limit_window_seconds: 900,
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
            port: 8080,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigError, is_placeholder_secret, validate_bootstrap_admin};

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
}
