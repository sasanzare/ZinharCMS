use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::Utc;
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;

const RAW_CHALLENGE_BYTES: usize = 32;
const PRE_AUTH_KEY_PREFIX: &str = "zinhar:mfa:pre-auth:v1:";
const STEP_UP_KEY_PREFIX: &str = "zinhar:mfa:step-up:v1:";
const STEP_UP_GRANT_KEY_PREFIX: &str = "zinhar:mfa:step-up-grant:v1:";
const ATTEMPT_LOCK_PREFIX: &str = "zinhar:mfa:attempt-lock:v1:";
const ATTEMPT_FAILURE_PREFIX: &str = "zinhar:mfa:attempt-failure:v1:";
const RATE_LIMIT_PREFIX: &str = "zinhar:mfa:rate:v1:";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PreAuthRecord {
    pub user_id: Uuid,
    pub auth_version: i64,
    pub authenticated_at: i64,
    pub issued_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum StepUpScope {
    SessionLogoutAll,
    PrivilegedSessionRevocation,
    MfaDisable,
    MfaRecoveryRegenerate,
    OrganizationAdministration,
    WebhookAdministration,
    BillingAdministration,
    MarketplaceAdministration,
    MarketplacePayout,
}

impl StepUpScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SessionLogoutAll => "session_logout_all",
            Self::PrivilegedSessionRevocation => "privileged_session_revocation",
            Self::MfaDisable => "mfa_disable",
            Self::MfaRecoveryRegenerate => "mfa_recovery_regenerate",
            Self::OrganizationAdministration => "organization_administration",
            Self::WebhookAdministration => "webhook_administration",
            Self::BillingAdministration => "billing_administration",
            Self::MarketplaceAdministration => "marketplace_administration",
            Self::MarketplacePayout => "marketplace_payout",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StepUpRecord {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub auth_version: i64,
    pub scope: StepUpScope,
    pub issued_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct StepUpGrant {
    user_id: Uuid,
    session_id: Uuid,
    auth_version: i64,
    scope: StepUpScope,
    issued_at: i64,
    expires_at: i64,
}

#[derive(Debug)]
pub struct IssuedChallenge {
    pub raw_value: String,
    pub expires_at: i64,
}

#[derive(Debug)]
pub struct Attempt<T> {
    pub record: T,
    owner: String,
}

pub async fn issue_pre_auth(
    redis: &redis::Client,
    user_id: Uuid,
    auth_version: i64,
    authenticated_at: i64,
    ttl_seconds: u64,
) -> Result<IssuedChallenge, AppError> {
    let now = Utc::now().timestamp();
    let expires_at = checked_expiry(now, ttl_seconds, 60, 300)?;
    issue_record(
        redis,
        PRE_AUTH_KEY_PREFIX,
        &PreAuthRecord {
            user_id,
            auth_version,
            authenticated_at,
            issued_at: now,
            expires_at,
        },
        ttl_seconds,
        expires_at,
    )
    .await
}

pub async fn issue_step_up_challenge(
    redis: &redis::Client,
    claims: &Claims,
    scope: StepUpScope,
    ttl_seconds: u64,
) -> Result<IssuedChallenge, AppError> {
    if claims.aal != 2 || claims.mfa_time.is_none() {
        return Err(AppError::Forbidden(
            "an MFA-authenticated session is required".to_owned(),
        ));
    }
    let now = Utc::now().timestamp();
    let expires_at = checked_expiry(now, ttl_seconds, 60, 600)?;
    issue_record(
        redis,
        STEP_UP_KEY_PREFIX,
        &StepUpRecord {
            user_id: claims.sub,
            session_id: claims.sid,
            auth_version: claims.ver,
            scope,
            issued_at: now,
            expires_at,
        },
        ttl_seconds,
        expires_at,
    )
    .await
}

pub async fn begin_pre_auth_attempt(
    redis: &redis::Client,
    raw_value: &str,
) -> Result<Attempt<PreAuthRecord>, AppError> {
    begin_attempt(redis, PRE_AUTH_KEY_PREFIX, raw_value).await
}

pub async fn begin_step_up_attempt(
    redis: &redis::Client,
    raw_value: &str,
) -> Result<Attempt<StepUpRecord>, AppError> {
    begin_attempt(redis, STEP_UP_KEY_PREFIX, raw_value).await
}

pub async fn complete_pre_auth_attempt(
    redis: &redis::Client,
    raw_value: &str,
    attempt: &Attempt<PreAuthRecord>,
) -> Result<(), AppError> {
    complete_attempt(redis, PRE_AUTH_KEY_PREFIX, raw_value, &attempt.owner).await
}

pub async fn complete_step_up_attempt(
    redis: &redis::Client,
    raw_value: &str,
    attempt: &Attempt<StepUpRecord>,
) -> Result<(), AppError> {
    complete_attempt(redis, STEP_UP_KEY_PREFIX, raw_value, &attempt.owner).await
}

pub async fn reject_pre_auth_attempt(
    redis: &redis::Client,
    raw_value: &str,
    attempt: &Attempt<PreAuthRecord>,
    maximum_attempts: i64,
    ttl_seconds: u64,
) -> Result<(), AppError> {
    reject_attempt(
        redis,
        PRE_AUTH_KEY_PREFIX,
        raw_value,
        &attempt.owner,
        maximum_attempts,
        ttl_seconds,
    )
    .await
}

pub async fn reject_step_up_attempt(
    redis: &redis::Client,
    raw_value: &str,
    attempt: &Attempt<StepUpRecord>,
    maximum_attempts: i64,
    ttl_seconds: u64,
) -> Result<(), AppError> {
    reject_attempt(
        redis,
        STEP_UP_KEY_PREFIX,
        raw_value,
        &attempt.owner,
        maximum_attempts,
        ttl_seconds,
    )
    .await
}

pub async fn issue_step_up_grant(
    redis: &redis::Client,
    record: &StepUpRecord,
    ttl_seconds: u64,
) -> Result<IssuedChallenge, AppError> {
    let now = Utc::now().timestamp();
    let expires_at = checked_expiry(now, ttl_seconds, 60, 600)?;
    issue_record(
        redis,
        STEP_UP_GRANT_KEY_PREFIX,
        &StepUpGrant {
            user_id: record.user_id,
            session_id: record.session_id,
            auth_version: record.auth_version,
            scope: record.scope,
            issued_at: now,
            expires_at,
        },
        ttl_seconds,
        expires_at,
    )
    .await
}

pub async fn consume_step_up_grant(
    redis: &redis::Client,
    raw_value: &str,
    claims: &Claims,
    expected_scope: StepUpScope,
) -> Result<(), AppError> {
    let key = record_key(STEP_UP_GRANT_KEY_PREFIX, raw_value)?;
    let mut connection = connection(redis).await?;
    let payload: Option<String> = redis::cmd("GETDEL")
        .arg(key)
        .query_async(&mut connection)
        .await
        .map_err(|_| service_unavailable())?;
    let grant: StepUpGrant = payload
        .as_deref()
        .ok_or_else(invalid_challenge)
        .and_then(|value| serde_json::from_str(value).map_err(|_| invalid_challenge()))?;
    let now = Utc::now().timestamp();
    if grant.user_id != claims.sub
        || grant.session_id != claims.sid
        || grant.auth_version != claims.ver
        || grant.scope != expected_scope
        || grant.issued_at > now
        || grant.expires_at < now
        || claims.aal != 2
    {
        return Err(invalid_challenge());
    }
    Ok(())
}

pub async fn enforce_rate_limit(
    redis: &redis::Client,
    bucket: &str,
    subject: &str,
    maximum_attempts: i64,
) -> Result<(), AppError> {
    if maximum_attempts < 1
        || bucket.is_empty()
        || !bucket
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
    {
        return Err(AppError::Internal(
            "MFA rate-limit configuration is invalid".to_owned(),
        ));
    }
    let subject_hash = URL_SAFE_NO_PAD.encode(Sha256::digest(subject.as_bytes()));
    let minute = Utc::now().timestamp().div_euclid(60);
    let key = format!("{RATE_LIMIT_PREFIX}{bucket}:{subject_hash}:{minute}");
    let script = redis::Script::new(
        r#"
        local count = redis.call('INCR', KEYS[1])
        if count == 1 then
          redis.call('EXPIRE', KEYS[1], 60)
        end
        return count
        "#,
    );
    let mut connection = connection(redis).await?;
    let count: i64 = script
        .key(key)
        .invoke_async(&mut connection)
        .await
        .map_err(|_| service_unavailable())?;
    if count > maximum_attempts {
        return Err(AppError::TooManyRequests(
            "MFA verification rate limit exceeded".to_owned(),
        ));
    }
    Ok(())
}

async fn issue_record<T: Serialize>(
    redis: &redis::Client,
    prefix: &str,
    record: &T,
    ttl_seconds: u64,
    expires_at: i64,
) -> Result<IssuedChallenge, AppError> {
    let payload =
        serde_json::to_string(record).map_err(|error| AppError::Internal(error.to_string()))?;
    let mut connection = connection(redis).await?;
    for _ in 0..3 {
        let raw_value = random_value(RAW_CHALLENGE_BYTES);
        let key = record_key(prefix, &raw_value)?;
        let stored: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(&payload)
            .arg("EX")
            .arg(ttl_seconds)
            .arg("NX")
            .query_async(&mut connection)
            .await
            .map_err(|_| service_unavailable())?;
        if stored.as_deref() == Some("OK") {
            return Ok(IssuedChallenge {
                raw_value,
                expires_at,
            });
        }
    }
    Err(service_unavailable())
}

async fn begin_attempt<T: DeserializeOwned>(
    redis: &redis::Client,
    prefix: &str,
    raw_value: &str,
) -> Result<Attempt<T>, AppError> {
    let digest = challenge_digest(raw_value)?;
    let record_key = format!("{prefix}{digest}");
    let lock_key = format!("{ATTEMPT_LOCK_PREFIX}{digest}");
    let owner = random_value(16);
    let mut connection = connection(redis).await?;
    let locked: Option<String> = redis::cmd("SET")
        .arg(&lock_key)
        .arg(&owner)
        .arg("EX")
        .arg(30)
        .arg("NX")
        .query_async(&mut connection)
        .await
        .map_err(|_| service_unavailable())?;
    if locked.as_deref() != Some("OK") {
        return Err(AppError::TooManyRequests(
            "MFA verification is already in progress".to_owned(),
        ));
    }
    let payload: Option<String> = redis::cmd("GET")
        .arg(&record_key)
        .query_async(&mut connection)
        .await
        .map_err(|_| service_unavailable())?;
    let Some(payload) = payload else {
        release_lock(&mut connection, &lock_key, &owner).await?;
        return Err(invalid_challenge());
    };
    let record = serde_json::from_str(&payload).map_err(|_| invalid_challenge())?;
    Ok(Attempt { record, owner })
}

async fn complete_attempt(
    redis: &redis::Client,
    prefix: &str,
    raw_value: &str,
    owner: &str,
) -> Result<(), AppError> {
    let digest = challenge_digest(raw_value)?;
    let record_key = format!("{prefix}{digest}");
    let lock_key = format!("{ATTEMPT_LOCK_PREFIX}{digest}");
    let failure_key = format!("{ATTEMPT_FAILURE_PREFIX}{digest}");
    let script = redis::Script::new(
        r#"
        if redis.call('GET', KEYS[2]) ~= ARGV[1] then
          return 0
        end
        local existed = redis.call('DEL', KEYS[1])
        redis.call('DEL', KEYS[2])
        redis.call('DEL', KEYS[3])
        return existed
        "#,
    );
    let mut connection = connection(redis).await?;
    let completed: i64 = script
        .key(record_key)
        .key(lock_key)
        .key(failure_key)
        .arg(owner)
        .invoke_async(&mut connection)
        .await
        .map_err(|_| service_unavailable())?;
    if completed != 1 {
        return Err(invalid_challenge());
    }
    Ok(())
}

async fn reject_attempt(
    redis: &redis::Client,
    prefix: &str,
    raw_value: &str,
    owner: &str,
    maximum_attempts: i64,
    ttl_seconds: u64,
) -> Result<(), AppError> {
    let digest = challenge_digest(raw_value)?;
    let record_key = format!("{prefix}{digest}");
    let lock_key = format!("{ATTEMPT_LOCK_PREFIX}{digest}");
    let failure_key = format!("{ATTEMPT_FAILURE_PREFIX}{digest}");
    let script = redis::Script::new(
        r#"
        if redis.call('GET', KEYS[2]) ~= ARGV[1] then
          return -1
        end
        local failures = redis.call('INCR', KEYS[3])
        if failures == 1 then
          redis.call('EXPIRE', KEYS[3], ARGV[3])
        end
        if failures >= tonumber(ARGV[2]) then
          redis.call('DEL', KEYS[1])
        end
        redis.call('DEL', KEYS[2])
        return failures
        "#,
    );
    let mut connection = connection(redis).await?;
    let failures: i64 = script
        .key(record_key)
        .key(lock_key)
        .key(failure_key)
        .arg(owner)
        .arg(maximum_attempts)
        .arg(ttl_seconds)
        .invoke_async(&mut connection)
        .await
        .map_err(|_| service_unavailable())?;
    if failures < 0 {
        return Err(invalid_challenge());
    }
    Ok(())
}

async fn release_lock(
    connection: &mut redis::aio::MultiplexedConnection,
    lock_key: &str,
    owner: &str,
) -> Result<(), AppError> {
    let script = redis::Script::new(
        r#"
        if redis.call('GET', KEYS[1]) == ARGV[1] then
          return redis.call('DEL', KEYS[1])
        end
        return 0
        "#,
    );
    let _: i64 = script
        .key(lock_key)
        .arg(owner)
        .invoke_async(connection)
        .await
        .map_err(|_| service_unavailable())?;
    Ok(())
}

async fn connection(redis: &redis::Client) -> Result<redis::aio::MultiplexedConnection, AppError> {
    redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| service_unavailable())
}

fn checked_expiry(now: i64, ttl_seconds: u64, minimum: u64, maximum: u64) -> Result<i64, AppError> {
    if !(minimum..=maximum).contains(&ttl_seconds) {
        return Err(AppError::Internal(
            "MFA challenge lifetime is invalid".to_owned(),
        ));
    }
    let ttl = i64::try_from(ttl_seconds)
        .map_err(|_| AppError::Internal("MFA challenge lifetime is invalid".to_owned()))?;
    now.checked_add(ttl)
        .ok_or_else(|| AppError::Internal("MFA challenge expiry overflow".to_owned()))
}

fn random_value(bytes: usize) -> String {
    let mut value = vec![0_u8; bytes];
    OsRng.fill_bytes(&mut value);
    URL_SAFE_NO_PAD.encode(value)
}

fn record_key(prefix: &str, raw_value: &str) -> Result<String, AppError> {
    Ok(format!("{prefix}{}", challenge_digest(raw_value)?))
}

fn challenge_digest(raw_value: &str) -> Result<String, AppError> {
    let decoded = URL_SAFE_NO_PAD
        .decode(raw_value)
        .map_err(|_| invalid_challenge())?;
    if decoded.len() != RAW_CHALLENGE_BYTES
        || !raw_value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(invalid_challenge());
    }
    Ok(URL_SAFE_NO_PAD.encode(Sha256::digest(raw_value.as_bytes())))
}

fn invalid_challenge() -> AppError {
    AppError::Unauthorized("MFA challenge is invalid or expired".to_owned())
}

fn service_unavailable() -> AppError {
    AppError::ServiceUnavailable("MFA challenge service is unavailable".to_owned())
}

#[cfg(test)]
mod tests {
    use std::env;

    use uuid::Uuid;

    use super::{
        RATE_LIMIT_PREFIX, StepUpScope, begin_pre_auth_attempt, challenge_digest,
        complete_pre_auth_attempt, enforce_rate_limit, issue_pre_auth, random_value, record_key,
    };

    #[test]
    fn opaque_challenges_are_random_hash_keyed_and_strictly_validated() {
        let first = random_value(32);
        let second = random_value(32);
        assert_ne!(first, second);
        assert_eq!(first.len(), 43);
        let digest = challenge_digest(&first).unwrap();
        assert_eq!(digest.len(), 43);
        assert_ne!(digest, first);
        assert!(challenge_digest("short").is_err());
        assert_eq!(
            StepUpScope::MarketplacePayout.as_str(),
            "marketplace_payout"
        );
    }

    #[tokio::test]
    async fn live_redis_challenges_and_rate_limits_are_distributed_and_expiry_bound() {
        let Ok(redis_url) = env::var("PHASE6_TEST_REDIS_URL") else {
            return;
        };
        let redis = redis::Client::open(redis_url.clone()).unwrap();
        let issued = issue_pre_auth(
            &redis,
            Uuid::now_v7(),
            3,
            chrono::Utc::now().timestamp(),
            60,
        )
        .await
        .unwrap();
        let (first, second) = tokio::join!(
            begin_pre_auth_attempt(&redis, &issued.raw_value),
            begin_pre_auth_attempt(&redis, &issued.raw_value)
        );
        assert_eq!(
            [first.is_ok(), second.is_ok()]
                .into_iter()
                .filter(|ok| *ok)
                .count(),
            1
        );
        let attempt = first.or(second).unwrap();
        complete_pre_auth_attempt(&redis, &issued.raw_value, &attempt)
            .await
            .unwrap();
        assert!(
            begin_pre_auth_attempt(&redis, &issued.raw_value)
                .await
                .is_err()
        );

        let expired = issue_pre_auth(
            &redis,
            Uuid::now_v7(),
            4,
            chrono::Utc::now().timestamp(),
            60,
        )
        .await
        .unwrap();
        let key = record_key(super::PRE_AUTH_KEY_PREFIX, &expired.raw_value).unwrap();
        let mut connection = redis.get_multiplexed_async_connection().await.unwrap();
        let _: i64 = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut connection)
            .await
            .unwrap();
        assert!(
            begin_pre_auth_attempt(&redis, &expired.raw_value)
                .await
                .is_err()
        );

        let rate_subject = format!("phase6-rate-{}", Uuid::now_v7());
        let second_worker = redis::Client::open(redis_url).unwrap();
        let (first_rate, second_rate) = tokio::join!(
            enforce_rate_limit(&redis, "totp-login", &rate_subject, 1),
            enforce_rate_limit(&second_worker, "totp-login", &rate_subject, 1)
        );
        assert_eq!(
            [first_rate.is_ok(), second_rate.is_ok()]
                .into_iter()
                .filter(|ok| *ok)
                .count(),
            1
        );
        assert!(
            enforce_rate_limit(&redis, "recovery-login", &rate_subject, 1)
                .await
                .is_ok()
        );
        assert!(
            enforce_rate_limit(
                &second_worker,
                "totp-login",
                &format!("phase6-rate-{}", Uuid::now_v7()),
                1,
            )
            .await
            .is_ok()
        );

        let rate_keys: Vec<String> = redis::cmd("KEYS")
            .arg(format!("{RATE_LIMIT_PREFIX}*"))
            .query_async(&mut connection)
            .await
            .unwrap();
        assert!(!rate_keys.is_empty());
        for key in &rate_keys {
            assert!(!key.contains(&rate_subject));
            let ttl: i64 = redis::cmd("TTL")
                .arg(key)
                .query_async(&mut connection)
                .await
                .unwrap();
            assert!((1..=60).contains(&ttl));
        }
        let _: i64 = redis::cmd("DEL")
            .arg(&rate_keys)
            .query_async(&mut connection)
            .await
            .unwrap();

        let unavailable = redis::Client::open("redis://127.0.0.1:1").unwrap();
        assert!(
            enforce_rate_limit(&unavailable, "totp-login", "fail-closed", 1)
                .await
                .is_err()
        );
    }
}
