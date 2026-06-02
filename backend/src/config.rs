use std::env;

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_access_expiry: u64,
    pub jwt_refresh_expiry: u64,
    pub upload_dir: String,
    pub max_upload_size: u64,
    pub cors_origin: String,
    pub port: u16,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    Missing(&'static str),
    #[error("invalid value for {name}: {value}")]
    Invalid { name: &'static str, value: String },
    #[error("JWT_SECRET must be at least 32 characters")]
    WeakJwtSecret,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let jwt_secret = get("JWT_SECRET", None)?;
        if jwt_secret.len() < 32 {
            return Err(ConfigError::WeakJwtSecret);
        }

        Ok(Self {
            database_url: get("DATABASE_URL", None)?,
            redis_url: get("REDIS_URL", Some("redis://localhost:6379"))?,
            jwt_secret,
            jwt_access_expiry: parse_u64("JWT_ACCESS_EXPIRY", 3600)?,
            jwt_refresh_expiry: parse_u64("JWT_REFRESH_EXPIRY", 604_800)?,
            upload_dir: get("UPLOAD_DIR", Some("./uploads"))?,
            max_upload_size: parse_u64("MAX_UPLOAD_SIZE", 52_428_800)?,
            cors_origin: get("CORS_ORIGIN", Some("http://localhost:5173"))?,
            port: parse_u16("PORT", 8080)?,
        })
    }
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

fn parse_u16(name: &'static str, default: u16) -> Result<u16, ConfigError> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => value
            .parse::<u16>()
            .map_err(|_| ConfigError::Invalid { name, value }),
        _ => Ok(default),
    }
}
