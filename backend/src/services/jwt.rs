use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::Utc;
use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::config::{Config, JWT_CLOCK_SKEW_SECONDS};
use crate::error::AppError;
use crate::middleware::auth::Claims;

type HmacSha256 = Hmac<Sha256>;

const MAX_JWT_LENGTH: usize = 8192;
const MAX_JWT_HEADER_SEGMENT_LENGTH: usize = 1024;
const MAX_JWT_PAYLOAD_SEGMENT_LENGTH: usize = 4096;
const MAX_JWT_SIGNATURE_SEGMENT_LENGTH: usize = 128;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct JwtHeader<'a> {
    alg: &'a str,
    typ: &'a str,
    kid: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OwnedJwtHeader {
    alg: String,
    typ: String,
    kid: String,
}

pub fn sign_access_token(
    user_id: uuid::Uuid,
    role: &str,
    auth_version: i64,
    config: &Config,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims =
        Claims {
            sub: user_id,
            role: role.to_owned(),
            ver: auth_version,
            exp: now
                .checked_add(i64::try_from(config.jwt_access_expiry).map_err(|_| {
                    AppError::Internal("access-token lifetime is too large".to_owned())
                })?)
                .ok_or_else(|| AppError::Internal("access-token expiry overflow".to_owned()))?,
            iat: now,
        };

    let active_key = config.jwt_key_ring.active();
    sign_claims(&claims, &active_key.kid, &active_key.secret)
}

pub fn verify_access_token(token: &str, config: &Config) -> Result<Claims, AppError> {
    if token.is_empty() || token.len() > MAX_JWT_LENGTH {
        return Err(invalid_token());
    }
    let mut parts = token.split('.');
    let header_segment = parts.next().ok_or_else(invalid_token)?;
    let payload = parts.next().ok_or_else(invalid_token)?;
    let signature = parts.next().ok_or_else(invalid_token)?;

    if parts.next().is_some()
        || header_segment.is_empty()
        || header_segment.len() > MAX_JWT_HEADER_SEGMENT_LENGTH
        || payload.is_empty()
        || payload.len() > MAX_JWT_PAYLOAD_SEGMENT_LENGTH
        || signature.is_empty()
        || signature.len() > MAX_JWT_SIGNATURE_SEGMENT_LENGTH
    {
        return Err(invalid_token());
    }

    let header_bytes = URL_SAFE_NO_PAD
        .decode(header_segment)
        .map_err(|_| invalid_token())?;
    let header: OwnedJwtHeader =
        serde_json::from_slice(&header_bytes).map_err(|_| invalid_token())?;
    if header.alg != "HS256" || header.typ != "JWT" {
        return Err(invalid_token());
    }
    let now = Utc::now().timestamp();
    let verification_key = config
        .jwt_key_ring
        .verification_key(&header.kid, now)
        .ok_or_else(invalid_token)?;
    let signing_input = format!("{header_segment}.{payload}");
    verify_signature(
        signing_input.as_bytes(),
        signature,
        &verification_key.secret,
    )?;

    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| invalid_token())?;
    let claims: Claims = serde_json::from_slice(&payload_bytes).map_err(|_| invalid_token())?;
    let maximum_lifetime = i64::try_from(config.jwt_access_expiry).map_err(|_| invalid_token())?;
    if claims.ver <= 0
        || claims.role.is_empty()
        || claims.role.len() > 64
        || claims.iat <= 0
        || claims.exp <= claims.iat
        || claims.exp.saturating_sub(claims.iat) > maximum_lifetime
        || claims.iat > now.saturating_add(JWT_CLOCK_SKEW_SECONDS)
        || claims.exp < now.saturating_sub(JWT_CLOCK_SKEW_SECONDS)
    {
        return Err(invalid_token());
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

fn sign_claims(claims: &Claims, kid: &str, secret: &str) -> Result<String, AppError> {
    let header = JwtHeader {
        alg: "HS256",
        typ: "JWT",
        kid,
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

fn verify_signature(bytes: &[u8], signature: &str, secret: &str) -> Result<(), AppError> {
    let signature = URL_SAFE_NO_PAD
        .decode(signature)
        .map_err(|_| invalid_token())?;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|error| AppError::Internal(error.to_string()))?;
    mac.update(bytes);
    mac.verify_slice(&signature).map_err(|_| invalid_token())
}

fn invalid_token() -> AppError {
    AppError::Unauthorized("invalid access token".to_owned())
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn sign_claims_for_test(
    user_id: uuid::Uuid,
    role: &str,
    auth_version: i64,
    issued_at: i64,
    expires_at: i64,
    kid: &str,
    algorithm: &str,
    secret: &str,
) -> String {
    let claims = Claims {
        sub: user_id,
        role: role.to_owned(),
        ver: auth_version,
        exp: expires_at,
        iat: issued_at,
    };
    let header = JwtHeader {
        alg: algorithm,
        typ: "JWT",
        kid,
    };
    let header = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).unwrap());
    let payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).unwrap());
    let signing_input = format!("{header}.{payload}");
    let signature = sign_bytes(signing_input.as_bytes(), secret).unwrap();
    format!("{signing_input}.{signature}")
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use serde_json::json;
    use uuid::Uuid;

    use super::{sign_access_token, sign_bytes, sign_claims_for_test, verify_access_token};
    use crate::config::{Config, JwtKeyConfig, JwtKeyStatus};

    #[test]
    fn access_token_signature_verification_rejects_tampering() {
        let config = Config::test_with_stripe_secret("test-webhook-secret");
        let token = sign_access_token(Uuid::now_v7(), "author", 1, &config).unwrap();
        assert!(verify_access_token(&token, &config).is_ok());

        let mut parts = token.split('.').collect::<Vec<_>>();
        parts[2] = "invalid-signature";
        assert!(verify_access_token(&parts.join("."), &config).is_err());
    }

    #[test]
    fn access_tokens_use_active_kid_and_bounded_previous_key_verification() {
        let now = chrono::Utc::now().timestamp();
        let config = Config::test_with_jwt_keys(vec![
            JwtKeyConfig::test(
                "key-b",
                JwtKeyStatus::Active,
                "active-local-test-key-material-over-32-bytes",
                None,
            ),
            JwtKeyConfig::test(
                "key-a",
                JwtKeyStatus::Previous,
                "previous-local-test-key-material-over-32-bytes",
                Some(now + 3630),
            ),
        ]);

        let current = sign_access_token(Uuid::now_v7(), "author", 1, &config).unwrap();
        let header: serde_json::Value = serde_json::from_slice(
            &URL_SAFE_NO_PAD
                .decode(current.split('.').next().unwrap())
                .unwrap(),
        )
        .unwrap();
        assert_eq!(header["kid"], "key-b");
        assert!(verify_access_token(&current, &config).is_ok());

        let old = sign_claims_for_test(
            Uuid::now_v7(),
            "author",
            1,
            now,
            now + 3600,
            "key-a",
            "HS256",
            "previous-local-test-key-material-over-32-bytes",
        );
        assert!(verify_access_token(&old, &config).is_ok());

        let removed = Config::test_with_jwt_keys(vec![JwtKeyConfig::test(
            "key-b",
            JwtKeyStatus::Active,
            "active-local-test-key-material-over-32-bytes",
            None,
        )]);
        assert!(verify_access_token(&old, &removed).is_err());
    }

    #[test]
    fn access_token_rejects_unknown_retired_missing_kid_and_algorithm_confusion() {
        let now = chrono::Utc::now().timestamp();
        let config = Config::test_with_jwt_keys(vec![
            JwtKeyConfig::test(
                "active",
                JwtKeyStatus::Active,
                "active-local-test-key-material-over-32-bytes",
                None,
            ),
            JwtKeyConfig::test(
                "retired",
                JwtKeyStatus::Retired,
                "retired-local-test-key-material-over-32-bytes",
                None,
            ),
        ]);

        for token in [
            sign_claims_for_test(
                Uuid::now_v7(),
                "author",
                1,
                now,
                now + 3600,
                "unknown",
                "HS256",
                "active-local-test-key-material-over-32-bytes",
            ),
            sign_claims_for_test(
                Uuid::now_v7(),
                "author",
                1,
                now,
                now + 3600,
                "retired",
                "HS256",
                "retired-local-test-key-material-over-32-bytes",
            ),
            sign_claims_for_test(
                Uuid::now_v7(),
                "author",
                1,
                now,
                now + 3600,
                "active",
                "none",
                "active-local-test-key-material-over-32-bytes",
            ),
            sign_claims_for_test(
                Uuid::now_v7(),
                "author",
                1,
                now,
                now + 3600,
                "active",
                "HS512",
                "active-local-test-key-material-over-32-bytes",
            ),
        ] {
            assert!(verify_access_token(&token, &config).is_err());
        }

        let no_kid_header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&json!({
                "sub": Uuid::now_v7(),
                "role": "author",
                "ver": 1,
                "exp": now + 3600,
                "iat": now
            }))
            .unwrap(),
        );
        let input = format!("{no_kid_header}.{payload}");
        let signature = sign_bytes(
            input.as_bytes(),
            "active-local-test-key-material-over-32-bytes",
        )
        .unwrap();
        assert!(verify_access_token(&format!("{input}.{signature}"), &config).is_err());
    }

    #[test]
    fn access_token_rejects_excessive_or_invalid_lifetimes() {
        let config = Config::test_with_stripe_secret("test-webhook-secret");
        let now = chrono::Utc::now().timestamp();
        for (issued_at, expires_at) in [
            (now, now + 3601),
            (now + 31, now + 3600),
            (now, now),
            (0, now + 3600),
        ] {
            let token = sign_claims_for_test(
                Uuid::now_v7(),
                "author",
                1,
                issued_at,
                expires_at,
                "test-active",
                "HS256",
                "test-secret-with-at-least-32-characters",
            );
            assert!(verify_access_token(&token, &config).is_err());
        }
    }

    #[test]
    fn access_token_rejects_claims_without_authentication_version() {
        let config = Config::test_with_stripe_secret("test-webhook-secret");
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT","kid":"test-active"}"#);
        let payload = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&json!({
                "sub": Uuid::now_v7(),
                "role": "author",
                "exp": chrono::Utc::now().timestamp() + 3600,
                "iat": chrono::Utc::now().timestamp()
            }))
            .unwrap(),
        );
        let signing_input = format!("{header}.{payload}");
        let signature = sign_bytes(
            signing_input.as_bytes(),
            "test-secret-with-at-least-32-characters",
        )
        .unwrap();

        assert!(verify_access_token(&format!("{signing_input}.{signature}"), &config).is_err());
    }
}
