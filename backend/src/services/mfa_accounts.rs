use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, Duration, Utc};
use rand_core::{OsRng, RngCore};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;
use zeroize::{Zeroize, Zeroizing};

use crate::config::MfaEncryptionKeyRing;
use crate::error::AppError;
use crate::services::mfa::{
    EncryptedTotpSecret, decrypt_totp_secret, encrypt_totp_secret, generate_totp_material,
    verify_totp_at,
};
use crate::services::password;
use crate::services::security_audit;
use crate::services::sessions::MfaAuthenticationMethod;

const RECOVERY_CODE_COUNT: usize = 10;
const RECOVERY_CODE_BYTES: usize = 15;
const RECOVERY_ALPHABET: &[u8; 32] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

#[derive(Debug, Serialize)]
pub struct MfaAccountStatus {
    pub enabled: bool,
    pub enrollment_pending: bool,
    pub recovery_codes_remaining: i64,
}

#[derive(Debug)]
pub struct EnrollmentStart {
    pub enrollment_id: Uuid,
    pub manual_secret: String,
    pub provisioning_uri: String,
    pub qr_code_base64: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MfaProofKind {
    Totp,
    Recovery,
}

#[derive(Debug, FromRow)]
struct LockedMfaRecord {
    user_id: Uuid,
    status: String,
    enrollment_id: Uuid,
    secret_ciphertext: Vec<u8>,
    secret_nonce: Vec<u8>,
    encryption_kid: String,
    encryption_version: i16,
    pending_expires_at: Option<DateTime<Utc>>,
    last_accepted_totp_step: Option<i64>,
}

pub async fn account_status(pool: &PgPool, user_id: Uuid) -> Result<MfaAccountStatus, AppError> {
    let row: Option<(String, Option<DateTime<Utc>>)> =
        sqlx::query_as("SELECT status, pending_expires_at FROM user_mfa WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
    let now = Utc::now();
    let enabled = row.as_ref().is_some_and(|(status, _)| status == "enabled");
    let enrollment_pending = row.as_ref().is_some_and(|(status, expires_at)| {
        status == "pending" && expires_at.is_some_and(|value| value > now)
    });
    let recovery_codes_remaining = if enabled {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM mfa_recovery_codes WHERE user_id = $1 AND used_at IS NULL",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?
    } else {
        0
    };
    Ok(MfaAccountStatus {
        enabled,
        enrollment_pending,
        recovery_codes_remaining,
    })
}

pub async fn is_enabled(pool: &PgPool, user_id: Uuid) -> Result<bool, AppError> {
    sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM user_mfa WHERE user_id = $1 AND status = 'enabled')",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

pub async fn begin_enrollment(
    pool: &PgPool,
    user_id: Uuid,
    account_label: &str,
    issuer: &str,
    key_ring: &MfaEncryptionKeyRing,
    ttl_seconds: u64,
) -> Result<EnrollmentStart, AppError> {
    if !(60..=900).contains(&ttl_seconds) {
        return Err(AppError::Internal(
            "MFA enrollment lifetime is invalid".to_owned(),
        ));
    }
    let enrollment_id = Uuid::now_v7();
    let material = generate_totp_material(issuer, account_label)?;
    let encrypted = encrypt_totp_secret(key_ring, user_id, enrollment_id, &material.secret)?;
    let ttl = i64::try_from(ttl_seconds)
        .map_err(|_| AppError::Internal("MFA enrollment lifetime is invalid".to_owned()))?;
    let expires_at = Utc::now() + Duration::seconds(ttl);
    let mut tx = pool.begin().await?;
    lock_user(&mut tx, user_id).await?;
    let existing: Option<String> =
        sqlx::query_scalar("SELECT status FROM user_mfa WHERE user_id = $1 FOR UPDATE")
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?;
    if existing.as_deref() == Some("enabled") {
        tx.rollback().await?;
        return Err(AppError::Conflict("MFA is already enabled".to_owned()));
    }
    sqlx::query(
        r#"
        INSERT INTO user_mfa (
          user_id,
          status,
          enrollment_id,
          secret_ciphertext,
          secret_nonce,
          encryption_kid,
          encryption_version,
          pending_expires_at
        )
        VALUES ($1, 'pending', $2, $3, $4, $5, $6, $7)
        ON CONFLICT (user_id) DO UPDATE
        SET status = 'pending',
            enrollment_id = EXCLUDED.enrollment_id,
            secret_ciphertext = EXCLUDED.secret_ciphertext,
            secret_nonce = EXCLUDED.secret_nonce,
            encryption_kid = EXCLUDED.encryption_kid,
            encryption_version = EXCLUDED.encryption_version,
            pending_expires_at = EXCLUDED.pending_expires_at,
            enabled_at = NULL,
            last_accepted_totp_step = NULL,
            created_at = now(),
            updated_at = now()
        WHERE user_mfa.status = 'pending'
        "#,
    )
    .bind(user_id)
    .bind(enrollment_id)
    .bind(&encrypted.ciphertext)
    .bind(&encrypted.nonce)
    .bind(&encrypted.kid)
    .bind(encrypted.version)
    .bind(expires_at)
    .execute(&mut *tx)
    .await?;
    security_audit::record_in_transaction(
        &mut tx,
        security_audit::MFA_ENROLLMENT_STARTED,
        Some(user_id),
        Some(user_id),
        serde_json::json!({ "expires_in_seconds": ttl_seconds }),
    )
    .await?;
    tx.commit().await?;
    Ok(EnrollmentStart {
        enrollment_id,
        manual_secret: material.manual_secret,
        provisioning_uri: material.provisioning_uri,
        qr_code_base64: material.qr_code_base64,
        expires_at,
    })
}

pub async fn confirm_enrollment(
    pool: &PgPool,
    user_id: Uuid,
    code: &str,
    key_ring: &MfaEncryptionKeyRing,
) -> Result<Vec<String>, AppError> {
    let now = Utc::now();
    let mut tx = pool.begin().await?;
    lock_user(&mut tx, user_id).await?;
    let record = lock_mfa_record(&mut tx, user_id).await?;
    if record.status != "pending" || record.pending_expires_at.is_none_or(|value| value <= now) {
        tx.rollback().await?;
        return Err(invalid_proof());
    }
    let secret = Zeroizing::new(decrypt_record(key_ring, &record, now.timestamp())?);
    let accepted_step =
        verify_totp_at(&secret, code, now.timestamp())?.ok_or_else(invalid_proof)?;
    let recovery_codes = generate_recovery_codes()?;
    let recovery_records = hash_recovery_codes(&recovery_codes)?;

    sqlx::query(
        r#"
        UPDATE user_mfa
        SET status = 'enabled',
            pending_expires_at = NULL,
            enabled_at = $2,
            last_accepted_totp_step = $3,
            updated_at = $2
        WHERE user_id = $1
          AND status = 'pending'
        "#,
    )
    .bind(user_id)
    .bind(now)
    .bind(accepted_step)
    .execute(&mut *tx)
    .await?;
    replace_recovery_codes(&mut tx, user_id, &recovery_records).await?;
    revoke_sessions_for_mfa_change(&mut tx, user_id, "mfa_enabled").await?;
    security_audit::record_in_transaction(
        &mut tx,
        security_audit::MFA_ENABLED,
        Some(user_id),
        Some(user_id),
        serde_json::json!({ "recovery_codes_issued": RECOVERY_CODE_COUNT }),
    )
    .await?;
    tx.commit().await?;
    Ok(recovery_codes)
}

pub async fn verify_enabled_mfa(
    pool: &PgPool,
    user_id: Uuid,
    proof_kind: MfaProofKind,
    code: &str,
    key_ring: &MfaEncryptionKeyRing,
) -> Result<MfaAuthenticationMethod, AppError> {
    let now = Utc::now();
    let mut tx = pool.begin().await?;
    lock_user(&mut tx, user_id).await?;
    let record = lock_mfa_record(&mut tx, user_id).await?;
    if record.status != "enabled" {
        tx.rollback().await?;
        return Err(invalid_proof());
    }
    let secret = Zeroizing::new(decrypt_record(key_ring, &record, now.timestamp())?);
    let method = match proof_kind {
        MfaProofKind::Totp => {
            let step = verify_totp_at(&secret, code, now.timestamp())?.ok_or_else(invalid_proof)?;
            if record
                .last_accepted_totp_step
                .is_some_and(|last_step| step <= last_step)
            {
                tx.rollback().await?;
                return Err(invalid_proof());
            }
            sqlx::query(
                r#"
                UPDATE user_mfa
                SET last_accepted_totp_step = $2,
                    updated_at = $3
                WHERE user_id = $1
                "#,
            )
            .bind(user_id)
            .bind(step)
            .bind(now)
            .execute(&mut *tx)
            .await?;
            MfaAuthenticationMethod::Totp
        }
        MfaProofKind::Recovery => {
            consume_recovery_code(&mut tx, user_id, code, now).await?;
            MfaAuthenticationMethod::Recovery
        }
    };
    lazy_reencrypt(&mut tx, key_ring, &record, &secret, now.timestamp()).await?;
    tx.commit().await?;
    Ok(method)
}

pub async fn regenerate_recovery_codes(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<String>, AppError> {
    let recovery_codes = generate_recovery_codes()?;
    let recovery_records = hash_recovery_codes(&recovery_codes)?;
    let mut tx = pool.begin().await?;
    lock_user(&mut tx, user_id).await?;
    let enabled: Option<bool> =
        sqlx::query_scalar("SELECT status = 'enabled' FROM user_mfa WHERE user_id = $1 FOR UPDATE")
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?;
    if enabled != Some(true) {
        tx.rollback().await?;
        return Err(AppError::Conflict("MFA is not enabled".to_owned()));
    }
    replace_recovery_codes(&mut tx, user_id, &recovery_records).await?;
    security_audit::record_in_transaction(
        &mut tx,
        security_audit::MFA_RECOVERY_CODES_REGENERATED,
        Some(user_id),
        Some(user_id),
        serde_json::json!({ "recovery_codes_issued": RECOVERY_CODE_COUNT }),
    )
    .await?;
    tx.commit().await?;
    Ok(recovery_codes)
}

pub async fn disable_mfa(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;
    lock_user(&mut tx, user_id).await?;
    let invalidated_recovery_codes =
        sqlx::query("DELETE FROM mfa_recovery_codes WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut *tx)
            .await?
            .rows_affected();
    let deleted = sqlx::query("DELETE FROM user_mfa WHERE user_id = $1 AND status = 'enabled'")
        .bind(user_id)
        .execute(&mut *tx)
        .await?
        .rows_affected();
    if deleted != 1 {
        tx.rollback().await?;
        return Err(AppError::Conflict("MFA is not enabled".to_owned()));
    }
    revoke_sessions_for_mfa_change(&mut tx, user_id, "mfa_disabled").await?;
    security_audit::record_in_transaction(
        &mut tx,
        security_audit::MFA_DISABLED,
        Some(user_id),
        Some(user_id),
        serde_json::json!({
            "sessions_revoked": true,
            "recovery_codes_invalidated": invalidated_recovery_codes,
        }),
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn delete_expired_pending_enrollments(
    tx: &mut Transaction<'_, Postgres>,
    batch_size: i64,
) -> Result<u64, AppError> {
    sqlx::query(
        r#"
        WITH candidates AS (
          SELECT user_id
          FROM user_mfa
          WHERE status = 'pending'
            AND pending_expires_at <= now()
          ORDER BY user_id
          LIMIT $1
          FOR UPDATE SKIP LOCKED
        )
        DELETE FROM user_mfa enrollment
        USING candidates
        WHERE enrollment.user_id = candidates.user_id
        "#,
    )
    .bind(batch_size)
    .execute(&mut **tx)
    .await
    .map(|result| result.rows_affected())
    .map_err(AppError::from)
}

async fn lock_user(tx: &mut Transaction<'_, Postgres>, user_id: Uuid) -> Result<(), AppError> {
    let active: Option<bool> =
        sqlx::query_scalar("SELECT is_active FROM users WHERE id = $1 FOR UPDATE")
            .bind(user_id)
            .fetch_optional(&mut **tx)
            .await?;
    if active != Some(true) {
        return Err(AppError::Unauthorized("invalid credentials".to_owned()));
    }
    Ok(())
}

async fn lock_mfa_record(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<LockedMfaRecord, AppError> {
    sqlx::query_as::<_, LockedMfaRecord>(
        r#"
        SELECT user_id,
               status,
               enrollment_id,
               secret_ciphertext,
               secret_nonce,
               encryption_kid,
               encryption_version,
               pending_expires_at,
               last_accepted_totp_step
        FROM user_mfa
        WHERE user_id = $1
        FOR UPDATE
        "#,
    )
    .bind(user_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(invalid_proof)
}

fn decrypt_record(
    key_ring: &MfaEncryptionKeyRing,
    record: &LockedMfaRecord,
    now: i64,
) -> Result<Vec<u8>, AppError> {
    decrypt_totp_secret(
        key_ring,
        record.user_id,
        record.enrollment_id,
        &EncryptedTotpSecret {
            ciphertext: record.secret_ciphertext.clone(),
            nonce: record.secret_nonce.clone(),
            kid: record.encryption_kid.clone(),
            version: record.encryption_version,
        },
        now,
    )
}

async fn lazy_reencrypt(
    tx: &mut Transaction<'_, Postgres>,
    key_ring: &MfaEncryptionKeyRing,
    record: &LockedMfaRecord,
    secret: &[u8],
    now: i64,
) -> Result<(), AppError> {
    if record.encryption_kid == key_ring.active().kid {
        return Ok(());
    }
    let encrypted = encrypt_totp_secret(key_ring, record.user_id, record.enrollment_id, secret)?;
    let updated = sqlx::query(
        r#"
        UPDATE user_mfa
        SET secret_ciphertext = $2,
            secret_nonce = $3,
            encryption_kid = $4,
            encryption_version = $5,
            updated_at = to_timestamp($6)
        WHERE user_id = $1
          AND encryption_kid = $7
        "#,
    )
    .bind(record.user_id)
    .bind(encrypted.ciphertext)
    .bind(encrypted.nonce)
    .bind(encrypted.kid)
    .bind(encrypted.version)
    .bind(now)
    .bind(&record.encryption_kid)
    .execute(&mut **tx)
    .await?
    .rows_affected();
    if updated != 1 {
        return Err(AppError::Internal(
            "MFA key rotation update failed".to_owned(),
        ));
    }
    Ok(())
}

fn generate_recovery_codes() -> Result<Vec<String>, AppError> {
    let mut codes = Vec::with_capacity(RECOVERY_CODE_COUNT);
    for _ in 0..RECOVERY_CODE_COUNT {
        let mut random = [0_u8; RECOVERY_CODE_BYTES];
        OsRng.fill_bytes(&mut random);
        let mut encoded = String::with_capacity(29);
        let mut buffer = 0_u32;
        let mut bits = 0_u8;
        let mut characters = 0_usize;
        for byte in random {
            buffer = (buffer << 8) | u32::from(byte);
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                if characters > 0 && characters.is_multiple_of(4) {
                    encoded.push('-');
                }
                let index = ((buffer >> bits) & 31) as usize;
                encoded.push(RECOVERY_ALPHABET[index] as char);
                characters += 1;
            }
        }
        codes.push(encoded);
        random.zeroize();
    }
    Ok(codes)
}

fn hash_recovery_codes(codes: &[String]) -> Result<Vec<(String, String)>, AppError> {
    codes
        .iter()
        .map(|code| {
            let normalized = normalize_recovery_code(code)?;
            Ok((
                recovery_lookup_hash(&normalized),
                password::hash_password(&normalized)?,
            ))
        })
        .collect()
}

fn normalize_recovery_code(code: &str) -> Result<String, AppError> {
    let normalized = code
        .bytes()
        .filter(|byte| *byte != b'-' && !byte.is_ascii_whitespace())
        .map(|byte| byte.to_ascii_uppercase())
        .collect::<Vec<_>>();
    if normalized.len() != 24
        || !normalized
            .iter()
            .all(|byte| RECOVERY_ALPHABET.contains(byte))
    {
        return Err(invalid_proof());
    }
    String::from_utf8(normalized).map_err(|_| invalid_proof())
}

fn recovery_lookup_hash(normalized: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(normalized.as_bytes()))
}

async fn replace_recovery_codes(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    records: &[(String, String)],
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM mfa_recovery_codes WHERE user_id = $1")
        .bind(user_id)
        .execute(&mut **tx)
        .await?;
    for (index, (lookup_hash, verifier_hash)) in records.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO mfa_recovery_codes (
              user_id,
              code_position,
              lookup_hash,
              verifier_hash
            )
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(user_id)
        .bind(
            i16::try_from(index + 1)
                .map_err(|_| AppError::Internal("recovery code position overflow".to_owned()))?,
        )
        .bind(lookup_hash)
        .bind(verifier_hash)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

async fn consume_recovery_code(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    code: &str,
    now: DateTime<Utc>,
) -> Result<(), AppError> {
    let normalized = normalize_recovery_code(code)?;
    let lookup_hash = recovery_lookup_hash(&normalized);
    let record: Option<(Uuid, String)> = sqlx::query_as(
        r#"
        SELECT id, verifier_hash
        FROM mfa_recovery_codes
        WHERE user_id = $1
          AND lookup_hash = $2
          AND used_at IS NULL
        FOR UPDATE
        "#,
    )
    .bind(user_id)
    .bind(lookup_hash)
    .fetch_optional(&mut **tx)
    .await?;
    let Some((id, verifier_hash)) = record else {
        return Err(invalid_proof());
    };
    if !password::verify_password(&normalized, &verifier_hash)? {
        return Err(invalid_proof());
    }
    let consumed =
        sqlx::query("UPDATE mfa_recovery_codes SET used_at = $2 WHERE id = $1 AND used_at IS NULL")
            .bind(id)
            .bind(now)
            .execute(&mut **tx)
            .await?
            .rows_affected();
    if consumed != 1 {
        return Err(invalid_proof());
    }
    Ok(())
}

async fn revoke_sessions_for_mfa_change(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    reason: &str,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE refresh_token_families
        SET revoked_at = COALESCE(revoked_at, now()),
            revocation_reason = COALESCE(revocation_reason, $2)
        WHERE user_id = $1
          AND revoked_at IS NULL
        "#,
    )
    .bind(user_id)
    .bind(reason)
    .execute(&mut **tx)
    .await?;
    sqlx::query(
        r#"
        UPDATE refresh_tokens
        SET revoked_at = COALESCE(revoked_at, now())
        WHERE user_id = $1
          AND revoked_at IS NULL
        "#,
    )
    .bind(user_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

fn invalid_proof() -> AppError {
    AppError::Unauthorized("MFA verification failed".to_owned())
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::postgres::PgPoolOptions;
    use totp_rs::{Algorithm, TOTP};
    use uuid::Uuid;

    use super::{
        MfaProofKind, RECOVERY_CODE_COUNT, begin_enrollment, confirm_enrollment, disable_mfa,
        generate_recovery_codes, normalize_recovery_code, recovery_lookup_hash, verify_enabled_mfa,
    };
    use crate::config::{Config, MfaEncryptionKeyConfig, MfaEncryptionKeyStatus};
    use crate::services::sessions::issue_refresh_family;

    #[test]
    fn recovery_codes_are_high_entropy_human_readable_and_hashable() {
        let first = generate_recovery_codes().unwrap();
        let second = generate_recovery_codes().unwrap();
        assert_eq!(first.len(), RECOVERY_CODE_COUNT);
        assert_eq!(second.len(), RECOVERY_CODE_COUNT);
        assert_ne!(first, second);
        for code in first {
            assert_eq!(code.len(), 29);
            let normalized = normalize_recovery_code(&code).unwrap();
            assert_eq!(normalized.len(), 24);
            assert_eq!(recovery_lookup_hash(&normalized).len(), 43);
            assert!(!recovery_lookup_hash(&normalized).contains(&normalized));
        }
        assert!(normalize_recovery_code("invalid-code").is_err());
    }

    #[tokio::test]
    async fn live_mfa_enrollment_replay_recovery_and_key_rotation_are_fail_closed() {
        let Ok(database_url) = env::var("PHASE6_TEST_DATABASE_URL") else {
            return;
        };
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(&database_url)
            .await
            .expect("the dedicated Phase 6 database must be reachable");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Phase 6 migrations must apply");
        let user_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, name)
            VALUES ($1, 'phase6-test-password-hash', 'Phase 6 MFA Test')
            RETURNING id
            "#,
        )
        .bind(format!("phase6-mfa-{}@example.invalid", Uuid::now_v7()))
        .fetch_one(&pool)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'author'
            "#,
        )
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
        let existing_session = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        let old_config = Config::test_with_mfa_keys(vec![MfaEncryptionKeyConfig::test(
            "mfa-old",
            MfaEncryptionKeyStatus::Active,
            [31_u8; 32],
            None,
        )]);
        let enrollment = begin_enrollment(
            &pool,
            user_id,
            "phase6@example.invalid",
            "ZinharCMS Test",
            &old_config.mfa_encryption_key_ring,
            600,
        )
        .await
        .unwrap();
        let secret = base32::decode(
            base32::Alphabet::Rfc4648 { padding: false },
            &enrollment.manual_secret,
        )
        .unwrap();
        let verifier = TOTP::new(
            Algorithm::SHA1,
            6,
            0,
            30,
            secret.clone(),
            None,
            "account".to_owned(),
        )
        .unwrap();
        let current_code = verifier.generate(chrono::Utc::now().timestamp() as u64);
        let recovery_codes = confirm_enrollment(
            &pool,
            user_id,
            &current_code,
            &old_config.mfa_encryption_key_ring,
        )
        .await
        .unwrap();
        assert_eq!(recovery_codes.len(), RECOVERY_CODE_COUNT);
        let stored: (Vec<u8>, String) = sqlx::query_as(
            "SELECT secret_ciphertext, encryption_kid FROM user_mfa WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(stored.0.len(), 36);
        assert!(
            !stored
                .0
                .windows(secret.len())
                .any(|window| window == secret)
        );
        assert_eq!(stored.1, "mfa-old");
        let session_revoked: Option<chrono::DateTime<chrono::Utc>> =
            sqlx::query_scalar("SELECT revoked_at FROM refresh_token_families WHERE id = $1")
                .bind(existing_session.family_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(session_revoked.is_some());

        assert!(
            verify_enabled_mfa(
                &pool,
                user_id,
                MfaProofKind::Totp,
                &current_code,
                &old_config.mfa_encryption_key_ring,
            )
            .await
            .is_err()
        );
        let next_step_code =
            verifier.generate((chrono::Utc::now().timestamp() as u64).saturating_add(30));
        let (first_totp, second_totp) = tokio::join!(
            verify_enabled_mfa(
                &pool,
                user_id,
                MfaProofKind::Totp,
                &next_step_code,
                &old_config.mfa_encryption_key_ring,
            ),
            verify_enabled_mfa(
                &pool,
                user_id,
                MfaProofKind::Totp,
                &next_step_code,
                &old_config.mfa_encryption_key_ring,
            )
        );
        assert_eq!(
            [first_totp.is_ok(), second_totp.is_ok()]
                .into_iter()
                .filter(|ok| *ok)
                .count(),
            1
        );
        let recovery_code = recovery_codes[0].clone();
        let (first, second) = tokio::join!(
            verify_enabled_mfa(
                &pool,
                user_id,
                MfaProofKind::Recovery,
                &recovery_code,
                &old_config.mfa_encryption_key_ring,
            ),
            verify_enabled_mfa(
                &pool,
                user_id,
                MfaProofKind::Recovery,
                &recovery_code,
                &old_config.mfa_encryption_key_ring,
            )
        );
        assert_eq!(
            [first.is_ok(), second.is_ok()]
                .into_iter()
                .filter(|ok| *ok)
                .count(),
            1
        );

        let now = chrono::Utc::now().timestamp();
        let rotated_config = Config::test_with_mfa_keys(vec![
            MfaEncryptionKeyConfig::test(
                "mfa-new",
                MfaEncryptionKeyStatus::Active,
                [47_u8; 32],
                None,
            ),
            MfaEncryptionKeyConfig::test(
                "mfa-old",
                MfaEncryptionKeyStatus::Previous,
                [31_u8; 32],
                Some(now + 600),
            ),
        ]);
        verify_enabled_mfa(
            &pool,
            user_id,
            MfaProofKind::Recovery,
            &recovery_codes[1],
            &rotated_config.mfa_encryption_key_ring,
        )
        .await
        .unwrap();
        let rotated_kid: String =
            sqlx::query_scalar("SELECT encryption_kid FROM user_mfa WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(rotated_kid, "mfa-new");
        let raw_code_stored: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
              SELECT 1
              FROM mfa_recovery_codes
              WHERE lookup_hash = $1 OR verifier_hash = $1
            )
            "#,
        )
        .bind(&recovery_codes[1])
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(!raw_code_stored);

        let session_before_disable = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        disable_mfa(&pool, user_id).await.unwrap();
        let remaining_mfa_rows: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM user_mfa WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        let remaining_recovery_rows: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM mfa_recovery_codes WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        let session_revoked_after_disable: Option<chrono::DateTime<chrono::Utc>> =
            sqlx::query_scalar("SELECT revoked_at FROM refresh_token_families WHERE id = $1")
                .bind(session_before_disable.family_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(remaining_mfa_rows, 0);
        assert_eq!(remaining_recovery_rows, 0);
        assert!(session_revoked_after_disable.is_some());

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
        pool.close().await;
    }
}
