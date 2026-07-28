use chrono::{DateTime, Duration, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::error::AppError;
use crate::services::{jwt, security_audit};

const MAX_TOKEN_ISSUANCE_PER_HOUR: i64 = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenPurpose {
    PasswordReset,
    EmailVerification,
    EmailChange,
}

impl TokenPurpose {
    fn as_str(self) -> &'static str {
        match self {
            Self::PasswordReset => "password_reset",
            Self::EmailVerification => "email_verification",
            Self::EmailChange => "email_change",
        }
    }

    fn maximum_ttl_seconds(self) -> i64 {
        match self {
            Self::PasswordReset | Self::EmailChange => 3600,
            Self::EmailVerification => 86_400,
        }
    }
}

pub struct IssuedSecurityToken {
    pub raw_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenConsumption {
    Consumed,
    Rejected,
}

#[derive(FromRow)]
struct LockedSecurityToken {
    id: Uuid,
    binding_hash: Option<String>,
    expires_at: DateTime<Utc>,
    consumed_at: Option<DateTime<Utc>>,
    revoked_at: Option<DateTime<Utc>>,
}

pub async fn issue_token(
    pool: &PgPool,
    user_id: Uuid,
    purpose: TokenPurpose,
    binding: Option<&str>,
    ttl_seconds: i64,
) -> Result<IssuedSecurityToken, AppError> {
    if ttl_seconds <= 0 || ttl_seconds > purpose.maximum_ttl_seconds() {
        return Err(AppError::Validation(
            "security-token lifetime is invalid".to_owned(),
        ));
    }
    let raw_token = jwt::generate_refresh_token();
    let token_hash = jwt::hash_refresh_token(&raw_token);
    let binding_hash = binding.map(normalized_binding_hash);
    let expires_at = Utc::now() + Duration::seconds(ttl_seconds);
    let mut tx = pool.begin().await?;
    lock_user_tokens(&mut tx, user_id).await?;
    let is_active: Option<bool> =
        sqlx::query_scalar("SELECT is_active FROM users WHERE id = $1 FOR UPDATE")
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?;
    if is_active != Some(true) {
        tx.rollback().await?;
        return Err(AppError::Unauthorized("invalid account".to_owned()));
    }
    let recent_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM security_tokens
        WHERE user_id = $1
          AND purpose = $2
          AND created_at >= now() - INTERVAL '1 hour'
        "#,
    )
    .bind(user_id)
    .bind(purpose.as_str())
    .fetch_one(&mut *tx)
    .await?;
    if recent_count >= MAX_TOKEN_ISSUANCE_PER_HOUR {
        tx.rollback().await?;
        return Err(AppError::TooManyRequests(
            "security-token request limit exceeded".to_owned(),
        ));
    }
    revoke_tokens_in_transaction(&mut tx, user_id, purpose).await?;
    sqlx::query(
        r#"
        INSERT INTO security_tokens (
          user_id,
          purpose,
          token_hash,
          binding_hash,
          expires_at
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(user_id)
    .bind(purpose.as_str())
    .bind(token_hash)
    .bind(binding_hash)
    .bind(expires_at)
    .execute(&mut *tx)
    .await?;
    security_audit::record_in_transaction(
        &mut tx,
        security_audit::TOKEN_ISSUED,
        None,
        Some(user_id),
        serde_json::json!({
            "purpose": purpose.as_str(),
            "ttl_seconds": ttl_seconds
        }),
    )
    .await?;
    tx.commit().await?;

    Ok(IssuedSecurityToken {
        raw_token,
        expires_at,
    })
}

pub async fn consume_token(
    pool: &PgPool,
    raw_token: &str,
    purpose: TokenPurpose,
    user_id: Uuid,
    binding: Option<&str>,
) -> Result<TokenConsumption, AppError> {
    if raw_token.len() != 43 {
        return Ok(TokenConsumption::Rejected);
    }
    let token_hash = jwt::hash_refresh_token(raw_token);
    let expected_binding_hash = binding.map(normalized_binding_hash);
    let mut tx = pool.begin().await?;
    lock_user_tokens(&mut tx, user_id).await?;
    let token = sqlx::query_as::<_, LockedSecurityToken>(
        r#"
        SELECT id,
               binding_hash,
               expires_at,
               consumed_at,
               revoked_at
        FROM security_tokens
        WHERE token_hash = $1
          AND purpose = $2
          AND user_id = $3
        FOR UPDATE
        "#,
    )
    .bind(token_hash)
    .bind(purpose.as_str())
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(token) = token else {
        tx.rollback().await?;
        return Ok(TokenConsumption::Rejected);
    };
    if token.consumed_at.is_some() {
        security_audit::record_in_transaction(
            &mut tx,
            security_audit::TOKEN_REUSE_REJECTED,
            None,
            Some(user_id),
            serde_json::json!({ "purpose": purpose.as_str() }),
        )
        .await?;
        tx.commit().await?;
        return Ok(TokenConsumption::Rejected);
    }
    if token.revoked_at.is_some()
        || token.expires_at <= Utc::now()
        || token.binding_hash != expected_binding_hash
    {
        tx.commit().await?;
        return Ok(TokenConsumption::Rejected);
    }
    let updated = sqlx::query(
        r#"
        UPDATE security_tokens
        SET consumed_at = now()
        WHERE id = $1
          AND consumed_at IS NULL
          AND revoked_at IS NULL
          AND expires_at > now()
        "#,
    )
    .bind(token.id)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if updated == 1 {
        security_audit::record_in_transaction(
            &mut tx,
            security_audit::TOKEN_CONSUMED,
            Some(user_id),
            Some(user_id),
            serde_json::json!({ "purpose": purpose.as_str() }),
        )
        .await?;
    }
    tx.commit().await?;
    Ok(if updated == 1 {
        TokenConsumption::Consumed
    } else {
        TokenConsumption::Rejected
    })
}

pub async fn revoke_tokens(
    pool: &PgPool,
    user_id: Uuid,
    purpose: TokenPurpose,
) -> Result<u64, AppError> {
    let mut tx = pool.begin().await?;
    lock_user_tokens(&mut tx, user_id).await?;
    let revoked = revoke_tokens_in_transaction(&mut tx, user_id, purpose).await?;
    security_audit::record_in_transaction(
        &mut tx,
        security_audit::TOKEN_REVOKED,
        None,
        Some(user_id),
        serde_json::json!({
            "purpose": purpose.as_str(),
            "revoked_records": revoked
        }),
    )
    .await?;
    tx.commit().await?;
    Ok(revoked)
}

async fn revoke_tokens_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
    purpose: TokenPurpose,
) -> Result<u64, AppError> {
    let revoked = sqlx::query(
        r#"
        UPDATE security_tokens
        SET revoked_at = now()
        WHERE user_id = $1
          AND purpose = $2
          AND consumed_at IS NULL
          AND revoked_at IS NULL
        "#,
    )
    .bind(user_id)
    .bind(purpose.as_str())
    .execute(&mut **tx)
    .await?
    .rows_affected();
    Ok(revoked)
}

async fn lock_user_tokens(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1::text, 1))")
        .bind(user_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

fn normalized_binding_hash(binding: &str) -> String {
    jwt::hash_refresh_token(&binding.trim().to_lowercase())
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use super::{TokenConsumption, TokenPurpose, consume_token, issue_token, revoke_tokens};
    use crate::services::security_audit;

    #[tokio::test]
    async fn recovery_tokens_are_hashed_bound_single_use_and_atomic() {
        let Ok(database_url) = env::var("PHASE5_TEST_DATABASE_URL") else {
            return;
        };
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        let user_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, name)
            VALUES ($1, 'test-password-hash', 'Phase 5 Token Test')
            RETURNING id
            "#,
        )
        .bind(format!("phase5-token-{}@example.invalid", Uuid::now_v7()))
        .fetch_one(&pool)
        .await
        .unwrap();

        let issued = issue_token(&pool, user_id, TokenPurpose::PasswordReset, None, 3600)
            .await
            .unwrap();
        let stored_hash: String =
            sqlx::query_scalar("SELECT token_hash FROM security_tokens WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_ne!(stored_hash, issued.raw_token);
        assert!(!stored_hash.contains(&issued.raw_token));
        assert!(matches!(
            consume_token(
                &pool,
                &issued.raw_token,
                TokenPurpose::EmailVerification,
                user_id,
                None
            )
            .await
            .unwrap(),
            TokenConsumption::Rejected
        ));
        assert!(matches!(
            consume_token(
                &pool,
                &issued.raw_token,
                TokenPurpose::PasswordReset,
                Uuid::now_v7(),
                None
            )
            .await
            .unwrap(),
            TokenConsumption::Rejected
        ));

        let (first, second) = tokio::join!(
            consume_token(
                &pool,
                &issued.raw_token,
                TokenPurpose::PasswordReset,
                user_id,
                None
            ),
            consume_token(
                &pool,
                &issued.raw_token,
                TokenPurpose::PasswordReset,
                user_id,
                None
            )
        );
        let successes = [first.unwrap(), second.unwrap()]
            .into_iter()
            .filter(|result| matches!(result, TokenConsumption::Consumed))
            .count();
        assert_eq!(successes, 1);

        let revoked = issue_token(
            &pool,
            user_id,
            TokenPurpose::EmailVerification,
            Some("bound@example.invalid"),
            3600,
        )
        .await
        .unwrap();
        assert!(matches!(
            consume_token(
                &pool,
                &revoked.raw_token,
                TokenPurpose::EmailVerification,
                user_id,
                Some("wrong@example.invalid")
            )
            .await
            .unwrap(),
            TokenConsumption::Rejected
        ));
        revoke_tokens(&pool, user_id, TokenPurpose::EmailVerification)
            .await
            .unwrap();
        assert!(matches!(
            consume_token(
                &pool,
                &revoked.raw_token,
                TokenPurpose::EmailVerification,
                user_id,
                Some("bound@example.invalid")
            )
            .await
            .unwrap(),
            TokenConsumption::Rejected
        ));

        let expired = issue_token(&pool, user_id, TokenPurpose::EmailChange, None, 3600)
            .await
            .unwrap();
        sqlx::query(
            r#"
            UPDATE security_tokens
            SET created_at = now() - INTERVAL '2 hours',
                expires_at = now() - INTERVAL '1 hour'
            WHERE token_hash = $1
            "#,
        )
        .bind(crate::services::jwt::hash_refresh_token(&expired.raw_token))
        .execute(&pool)
        .await
        .unwrap();
        assert!(matches!(
            consume_token(
                &pool,
                &expired.raw_token,
                TokenPurpose::EmailChange,
                user_id,
                None
            )
            .await
            .unwrap(),
            TokenConsumption::Rejected
        ));

        for _ in 0..4 {
            issue_token(&pool, user_id, TokenPurpose::PasswordReset, None, 3600)
                .await
                .unwrap();
        }
        assert!(
            issue_token(&pool, user_id, TokenPurpose::PasswordReset, None, 3600)
                .await
                .is_err()
        );
        for (event_type, minimum_count) in [
            (security_audit::TOKEN_ISSUED, 1_i64),
            (security_audit::TOKEN_CONSUMED, 1_i64),
            (security_audit::TOKEN_REVOKED, 1_i64),
            (security_audit::TOKEN_REUSE_REJECTED, 1_i64),
        ] {
            let count: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM security_audit_events
                WHERE target_user_id = $1
                  AND event_type = $2
                "#,
            )
            .bind(user_id)
            .bind(event_type)
            .fetch_one(&pool)
            .await
            .unwrap();
            assert!(
                count >= minimum_count,
                "expected security audit event {event_type}"
            );
        }

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
