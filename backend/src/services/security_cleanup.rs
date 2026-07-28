use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::config::Config;
use crate::error::AppError;
use crate::services::security_audit;

#[derive(Clone, Debug)]
pub struct RetentionPolicy {
    pub batch_size: i64,
    pub expired_session_days: i64,
    pub revoked_session_days: i64,
    pub compromised_session_days: i64,
    pub finalized_security_token_days: i64,
    pub audit_event_days: i64,
    pub login_attempt_days: i64,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            batch_size: 500,
            expired_session_days: 30,
            revoked_session_days: 30,
            compromised_session_days: 180,
            finalized_security_token_days: 7,
            audit_event_days: 365,
            login_attempt_days: 30,
        }
    }
}

impl From<&Config> for RetentionPolicy {
    fn from(config: &Config) -> Self {
        Self {
            batch_size: config.security_cleanup_batch_size,
            expired_session_days: config.expired_session_retention_days,
            revoked_session_days: config.revoked_session_retention_days,
            compromised_session_days: config.compromised_session_retention_days,
            finalized_security_token_days: config.security_token_retention_days,
            audit_event_days: config.security_audit_retention_days,
            login_attempt_days: config.login_attempt_retention_days,
        }
    }
}

impl RetentionPolicy {
    pub fn validate(&self) -> Result<(), AppError> {
        if !(1..=5000).contains(&self.batch_size)
            || self.expired_session_days < 1
            || self.revoked_session_days < 1
            || self.compromised_session_days < self.revoked_session_days
            || self.finalized_security_token_days < 1
            || self.audit_event_days < 90
            || self.login_attempt_days < 1
        {
            return Err(AppError::Internal(
                "security retention policy is invalid".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct CleanupResult {
    pub session_families: u64,
    pub security_tokens: u64,
    pub invitation_tokens: u64,
    pub login_attempts: u64,
    pub audit_events: u64,
}

impl CleanupResult {
    pub fn total_affected(&self) -> u64 {
        self.session_families
            + self.security_tokens
            + self.invitation_tokens
            + self.login_attempts
            + self.audit_events
    }
}

pub async fn run_cleanup(
    pool: &PgPool,
    policy: &RetentionPolicy,
) -> Result<CleanupResult, AppError> {
    policy.validate()?;
    let now = Utc::now();
    let expired_session_cutoff = now - Duration::days(policy.expired_session_days);
    let revoked_session_cutoff = now - Duration::days(policy.revoked_session_days);
    let compromised_session_cutoff = now - Duration::days(policy.compromised_session_days);
    let finalized_token_cutoff = now - Duration::days(policy.finalized_security_token_days);
    let audit_cutoff = now - Duration::days(policy.audit_event_days);
    let login_cutoff = now - Duration::days(policy.login_attempt_days);
    let mut tx = pool.begin().await?;

    let session_families = sqlx::query(
        r#"
        WITH candidates AS (
          SELECT id
          FROM refresh_token_families
          WHERE (
              compromised_at IS NOT NULL
              AND compromised_at < $1
            )
            OR (
              compromised_at IS NULL
              AND revoked_at IS NOT NULL
              AND revoked_at < $2
            )
            OR (
              compromised_at IS NULL
              AND revoked_at IS NULL
              AND expires_at < $3
            )
          ORDER BY id
          LIMIT $4
          FOR UPDATE SKIP LOCKED
        )
        DELETE FROM refresh_token_families family
        USING candidates
        WHERE family.id = candidates.id
        "#,
    )
    .bind(compromised_session_cutoff)
    .bind(revoked_session_cutoff)
    .bind(expired_session_cutoff)
    .bind(policy.batch_size)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    let security_tokens = sqlx::query(
        r#"
        WITH candidates AS (
          SELECT id
          FROM security_tokens
          WHERE (
              consumed_at IS NOT NULL
              AND consumed_at < $1
            )
            OR (
              revoked_at IS NOT NULL
              AND revoked_at < $1
            )
            OR expires_at < $1
          ORDER BY id
          LIMIT $2
          FOR UPDATE SKIP LOCKED
        )
        DELETE FROM security_tokens token
        USING candidates
        WHERE token.id = candidates.id
        "#,
    )
    .bind(finalized_token_cutoff)
    .bind(policy.batch_size)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    let invitation_tokens = sqlx::query(
        r#"
        WITH candidates AS (
          SELECT id
          FROM organization_invitations
          WHERE token_hash IS NOT NULL
            AND (
              status <> 'pending'::organization_invitation_status
              OR expires_at <= now()
            )
          ORDER BY id
          LIMIT $1
          FOR UPDATE SKIP LOCKED
        )
        UPDATE organization_invitations invitation
        SET token_hash = NULL,
            status = CASE
              WHEN invitation.status = 'pending'::organization_invitation_status
                THEN 'expired'::organization_invitation_status
              ELSE invitation.status
            END,
            updated_at = now()
        FROM candidates
        WHERE invitation.id = candidates.id
        "#,
    )
    .bind(policy.batch_size)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    let login_attempts = sqlx::query(
        r#"
        WITH candidates AS (
          SELECT id
          FROM login_attempts
          WHERE attempted_at < $1
          ORDER BY id
          LIMIT $2
          FOR UPDATE SKIP LOCKED
        )
        DELETE FROM login_attempts attempt
        USING candidates
        WHERE attempt.id = candidates.id
        "#,
    )
    .bind(login_cutoff)
    .bind(policy.batch_size)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    let audit_events = sqlx::query(
        r#"
        WITH candidates AS (
          SELECT id
          FROM security_audit_events
          WHERE created_at < $1
          ORDER BY id
          LIMIT $2
          FOR UPDATE SKIP LOCKED
        )
        DELETE FROM security_audit_events event
        USING candidates
        WHERE event.id = candidates.id
        "#,
    )
    .bind(audit_cutoff)
    .bind(policy.batch_size)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    let result = CleanupResult {
        session_families,
        security_tokens,
        invitation_tokens,
        login_attempts,
        audit_events,
    };
    security_audit::record_in_transaction(
        &mut tx,
        security_audit::CLEANUP_COMPLETED,
        None,
        None,
        serde_json::json!({
            "session_families": result.session_families,
            "security_records": result.security_tokens,
            "invitation_records": result.invitation_tokens,
            "login_attempts": result.login_attempts,
            "audit_events": result.audit_events
        }),
    )
    .await?;
    tx.commit().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use super::{RetentionPolicy, run_cleanup};
    use crate::services::sessions::issue_refresh_family;

    #[tokio::test]
    async fn cleanup_is_bounded_idempotent_and_preserves_active_sessions() {
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
            VALUES ($1, 'cleanup-test-password-hash', 'Phase 5 Cleanup Test')
            RETURNING id
            "#,
        )
        .bind(format!("phase5-cleanup-{}@example.invalid", Uuid::now_v7()))
        .fetch_one(&pool)
        .await
        .unwrap();
        let active = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        let expired = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        sqlx::query(
            r#"
            UPDATE refresh_token_families
            SET created_at = now() - INTERVAL '60 days',
                last_used_at = now() - INTERVAL '60 days',
                expires_at = now() - INTERVAL '40 days'
            WHERE id = $1
            "#,
        )
        .bind(expired.family_id)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "UPDATE refresh_tokens SET expires_at = now() - INTERVAL '40 days' WHERE family_id = $1",
        )
        .bind(expired.family_id)
        .execute(&pool)
        .await
        .unwrap();
        let retained_compromise = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        sqlx::query(
            r#"
            UPDATE refresh_token_families
            SET created_at = now() - INTERVAL '120 days',
                last_used_at = now() - INTERVAL '120 days',
                expires_at = now() - INTERVAL '100 days',
                revoked_at = now() - INTERVAL '100 days',
                compromised_at = now() - INTERVAL '100 days',
                revocation_reason = 'reuse_detected'
            WHERE id = $1
            "#,
        )
        .bind(retained_compromise.family_id)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO security_tokens (
              user_id, purpose, token_hash, expires_at, created_at
            )
            VALUES (
              $1, 'password_reset', $2,
              now() - INTERVAL '19 days',
              now() - INTERVAL '20 days'
            )
            "#,
        )
        .bind(user_id)
        .bind("A".repeat(43))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO login_attempts (email, ip_address, succeeded, attempted_at)
            VALUES ($1, '127.0.0.1', false, now() - INTERVAL '40 days')
            "#,
        )
        .bind(format!("phase5-cleanup-{}@example.invalid", Uuid::now_v7()))
        .execute(&pool)
        .await
        .unwrap();
        let old_audit_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO security_audit_events (event_type, target_user_id, created_at)
            VALUES ('security.test.old', $1, now() - INTERVAL '400 days')
            RETURNING id
            "#,
        )
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let policy = RetentionPolicy {
            batch_size: 2,
            ..RetentionPolicy::default()
        };
        let first = run_cleanup(&pool, &policy).await.unwrap();
        assert!(first.total_affected() <= 10);
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM refresh_token_families WHERE id = $1"
            )
            .bind(active.family_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM refresh_token_families WHERE id = $1"
            )
            .bind(expired.family_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
            0
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM refresh_token_families WHERE id = $1"
            )
            .bind(retained_compromise.family_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM security_audit_events WHERE id = $1"
            )
            .bind(old_audit_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
            0
        );
        let second = run_cleanup(&pool, &policy).await.unwrap();
        assert!(second.total_affected() <= 10);

        let rollback_family = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        sqlx::query(
            r#"
            UPDATE refresh_token_families
            SET created_at = now() - INTERVAL '60 days',
                last_used_at = now() - INTERVAL '60 days',
                expires_at = now() - INTERVAL '40 days'
            WHERE id = $1
            "#,
        )
        .bind(rollback_family.family_id)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET expires_at = now() - INTERVAL '40 days'
            WHERE family_id = $1
            "#,
        )
        .bind(rollback_family.family_id)
        .execute(&pool)
        .await
        .unwrap();
        let guarded_attempt_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO login_attempts (email, ip_address, succeeded, attempted_at)
            VALUES ($1, '127.0.0.1', false, now() - INTERVAL '40 days')
            RETURNING id
            "#,
        )
        .bind(format!(
            "phase5-rollback-{}@example.invalid",
            Uuid::now_v7()
        ))
        .fetch_one(&pool)
        .await
        .unwrap();
        let guard_table = format!("phase5_cleanup_guard_{}", Uuid::now_v7().simple());
        sqlx::query(&format!(
            "CREATE TABLE {guard_table} (attempt_id UUID NOT NULL REFERENCES login_attempts(id))"
        ))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(&format!(
            "INSERT INTO {guard_table} (attempt_id) VALUES ($1)"
        ))
        .bind(guarded_attempt_id)
        .execute(&pool)
        .await
        .unwrap();
        let rollback_result = run_cleanup(
            &pool,
            &RetentionPolicy {
                batch_size: 50,
                ..RetentionPolicy::default()
            },
        )
        .await;
        assert!(rollback_result.is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM refresh_token_families WHERE id = $1"
            )
            .bind(rollback_family.family_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
            1
        );
        sqlx::query(&format!("DROP TABLE {guard_table}"))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM login_attempts WHERE id = $1")
            .bind(guarded_attempt_id)
            .execute(&pool)
            .await
            .unwrap();

        let mut concurrent_attempt_ids = Vec::new();
        for index in 0..4 {
            let attempt_id: Uuid = sqlx::query_scalar(
                r#"
                INSERT INTO login_attempts (
                  email,
                  ip_address,
                  succeeded,
                  attempted_at
                )
                VALUES ($1, '127.0.0.1', false, now() - INTERVAL '40 days')
                RETURNING id
                "#,
            )
            .bind(format!(
                "phase5-concurrent-cleanup-{index}-{}@example.invalid",
                Uuid::now_v7()
            ))
            .fetch_one(&pool)
            .await
            .unwrap();
            concurrent_attempt_ids.push(attempt_id);
        }
        let concurrent_policy = RetentionPolicy {
            batch_size: 2,
            ..RetentionPolicy::default()
        };
        let (concurrent_first, concurrent_second) = tokio::join!(
            run_cleanup(&pool, &concurrent_policy),
            run_cleanup(&pool, &concurrent_policy)
        );
        assert!(concurrent_first.is_ok());
        assert!(concurrent_second.is_ok());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM login_attempts WHERE id = ANY($1)")
                .bind(&concurrent_attempt_ids)
                .fetch_one(&pool)
                .await
                .unwrap(),
            0
        );

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
