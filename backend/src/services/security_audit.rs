use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::AppError;

pub const SESSION_REVOKED: &str = "security.session.revoked";
pub const LOGOUT_ALL: &str = "security.session.logout_all";
pub const PRIVILEGED_SESSION_REVOCATION: &str = "security.session.privileged_revocation";
pub const TOKEN_ISSUED: &str = "security.recovery_token.issued";
pub const TOKEN_CONSUMED: &str = "security.recovery_token.consumed";
pub const TOKEN_REVOKED: &str = "security.recovery_token.revoked";
pub const TOKEN_REUSE_REJECTED: &str = "security.recovery_token.reuse_rejected";
pub const CLEANUP_COMPLETED: &str = "security.cleanup.completed";
pub const MFA_ENROLLMENT_STARTED: &str = "security.mfa.enrollment_started";
pub const MFA_ENABLED: &str = "security.mfa.enabled";
pub const MFA_DISABLED: &str = "security.mfa.disabled";
pub const MFA_RECOVERY_CODES_REGENERATED: &str = "security.mfa.recovery_codes_regenerated";
pub const MFA_LOGIN_COMPLETED: &str = "security.mfa.login_completed";
pub const STEP_UP_COMPLETED: &str = "security.mfa.step_up_completed";

pub async fn record(
    pool: &PgPool,
    event_type: &str,
    actor_user_id: Option<Uuid>,
    target_user_id: Option<Uuid>,
    metadata: Value,
) -> Result<(), AppError> {
    validate_metadata(&metadata)?;
    sqlx::query(
        r#"
        INSERT INTO security_audit_events (
          event_type,
          actor_user_id,
          target_user_id,
          metadata
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(event_type)
    .bind(actor_user_id)
    .bind(target_user_id)
    .bind(metadata)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn record_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    event_type: &str,
    actor_user_id: Option<Uuid>,
    target_user_id: Option<Uuid>,
    metadata: Value,
) -> Result<(), AppError> {
    validate_metadata(&metadata)?;
    sqlx::query(
        r#"
        INSERT INTO security_audit_events (
          event_type,
          actor_user_id,
          target_user_id,
          metadata
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(event_type)
    .bind(actor_user_id)
    .bind(target_user_id)
    .bind(metadata)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

fn validate_metadata(metadata: &Value) -> Result<(), AppError> {
    let Some(object) = metadata.as_object() else {
        return Err(AppError::Internal(
            "security audit metadata must be an object".to_owned(),
        ));
    };
    if object.keys().any(|key| {
        let key = key.to_ascii_lowercase();
        [
            "authorization",
            "cookie",
            "hash",
            "password",
            "secret",
            "token",
        ]
        .iter()
        .any(|forbidden| key.contains(forbidden))
    }) {
        return Err(AppError::Internal(
            "security audit metadata contains a forbidden field".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::validate_metadata;

    #[test]
    fn security_audit_metadata_rejects_secret_shaped_fields() {
        assert!(validate_metadata(&json!({ "revoked_sessions": 2 })).is_ok());
        for metadata in [
            json!({ "access_token": "redacted" }),
            json!({ "refreshHash": "redacted" }),
            json!({ "cookie_value": "redacted" }),
            json!({ "password": "redacted" }),
        ] {
            assert!(validate_metadata(&metadata).is_err());
        }
    }
}
