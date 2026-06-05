use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::Utc;
use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::config::Config;
use crate::error::AppError;
use crate::middleware::auth::Claims;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize)]
struct JwtHeader {
    alg: &'static str,
    typ: &'static str,
}

pub fn sign_access_token(
    user_id: uuid::Uuid,
    role: &str,
    config: &Config,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id,
        role: role.to_owned(),
        exp: now + config.jwt_access_expiry as i64,
        iat: now,
    };

    sign_claims(&claims, &config.jwt_secret)
}

pub fn verify_access_token(token: &str, config: &Config) -> Result<Claims, AppError> {
    let mut parts = token.split('.');
    let header = parts
        .next()
        .ok_or_else(|| AppError::Unauthorized("invalid token".to_owned()))?;
    let payload = parts
        .next()
        .ok_or_else(|| AppError::Unauthorized("invalid token".to_owned()))?;
    let signature = parts
        .next()
        .ok_or_else(|| AppError::Unauthorized("invalid token".to_owned()))?;

    if parts.next().is_some() {
        return Err(AppError::Unauthorized("invalid token".to_owned()));
    }

    let signing_input = format!("{header}.{payload}");
    let expected = sign_bytes(signing_input.as_bytes(), &config.jwt_secret)?;
    if expected != signature {
        return Err(AppError::Unauthorized("invalid token signature".to_owned()));
    }

    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| AppError::Unauthorized("invalid token payload".to_owned()))?;
    let claims: Claims = serde_json::from_slice(&payload_bytes)
        .map_err(|_| AppError::Unauthorized("invalid token claims".to_owned()))?;

    if claims.exp < Utc::now().timestamp() {
        return Err(AppError::Unauthorized("token expired".to_owned()));
    }

    Ok(claims)
}

pub fn generate_refresh_token() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_refresh_token(token: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(token.as_bytes()))
}

fn sign_claims(claims: &Claims, secret: &str) -> Result<String, AppError> {
    let header = JwtHeader {
        alg: "HS256",
        typ: "JWT",
    };
    let header = URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(&header).map_err(|error| AppError::Internal(error.to_string()))?,
    );
    let payload = URL_SAFE_NO_PAD
        .encode(serde_json::to_vec(claims).map_err(|error| AppError::Internal(error.to_string()))?);
    let signing_input = format!("{header}.{payload}");
    let signature = sign_bytes(signing_input.as_bytes(), secret)?;

    Ok(format!("{signing_input}.{signature}"))
}

fn sign_bytes(bytes: &[u8], secret: &str) -> Result<String, AppError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|error| AppError::Internal(error.to_string()))?;
    mac.update(bytes);
    Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
}
