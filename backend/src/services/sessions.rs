use chrono::{DateTime, Duration, Utc};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::AppError;
use crate::middleware::auth::Claims;
use crate::services::jwt;

#[derive(Debug, Clone)]
pub struct IssuedRefreshToken {
    pub raw_token: String,
    pub token_id: Uuid,
    pub family_id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct CurrentAuthIdentity {
    pub user_id: Uuid,
    pub role: String,
    pub auth_version: i64,
}

#[derive(Debug)]
pub enum RefreshRotation {
    Rotated {
        issued: IssuedRefreshToken,
        identity: CurrentAuthIdentity,
    },
    Rejected,
}

#[derive(Debug, FromRow)]
struct LockedRefreshToken {
    token_id: Uuid,
    family_id: Uuid,
    user_id: Uuid,
    token_expires_at: DateTime<Utc>,
    token_revoked_at: Option<DateTime<Utc>>,
    rotated_at: Option<DateTime<Utc>>,
    family_expires_at: DateTime<Utc>,
    family_revoked_at: Option<DateTime<Utc>>,
    compromised_at: Option<DateTime<Utc>>,
    is_active: bool,
    auth_version: i64,
    role: Option<String>,
}

pub async fn issue_refresh_family(
    pool: &PgPool,
    user_id: Uuid,
    ttl_seconds: i64,
) -> Result<IssuedRefreshToken, AppError> {
    let raw_token = jwt::generate_refresh_token();
    issue_refresh_family_with_token(pool, user_id, &raw_token, ttl_seconds).await
}

async fn issue_refresh_family_with_token(
    pool: &PgPool,
    user_id: Uuid,
    raw_token: &str,
    ttl_seconds: i64,
) -> Result<IssuedRefreshToken, AppError> {
    if ttl_seconds <= 0 {
        return Err(AppError::Internal(
            "refresh token lifetime must be positive".to_owned(),
        ));
    }
    let mut tx = pool.begin().await?;
    let is_active: Option<bool> =
        sqlx::query_scalar("SELECT is_active FROM users WHERE id = $1 FOR UPDATE")
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await?;
    if is_active != Some(true) {
        tx.rollback().await?;
        return Err(AppError::Unauthorized("invalid credentials".to_owned()));
    }

    let issued = insert_family_and_token(
        &mut tx,
        user_id,
        raw_token,
        Utc::now() + Duration::seconds(ttl_seconds),
    )
    .await?;
    tx.commit().await?;
    Ok(issued)
}

async fn insert_family_and_token(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    raw_token: &str,
    expires_at: DateTime<Utc>,
) -> Result<IssuedRefreshToken, AppError> {
    let family_id = Uuid::now_v7();
    let token_id = Uuid::now_v7();
    sqlx::query(
        r#"
        INSERT INTO refresh_token_families (id, user_id, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(family_id)
    .bind(user_id)
    .bind(expires_at)
    .execute(&mut **tx)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (
          id,
          user_id,
          family_id,
          token_hash,
          expires_at
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(token_id)
    .bind(user_id)
    .bind(family_id)
    .bind(jwt::hash_refresh_token(raw_token))
    .bind(expires_at)
    .execute(&mut **tx)
    .await?;

    Ok(IssuedRefreshToken {
        raw_token: raw_token.to_owned(),
        token_id,
        family_id,
    })
}

pub async fn rotate_refresh_token(
    pool: &PgPool,
    presented_token: &str,
    ttl_seconds: i64,
) -> Result<RefreshRotation, AppError> {
    let successor = jwt::generate_refresh_token();
    rotate_refresh_token_with_successor(pool, presented_token, &successor, ttl_seconds).await
}

async fn rotate_refresh_token_with_successor(
    pool: &PgPool,
    presented_token: &str,
    successor_token: &str,
    ttl_seconds: i64,
) -> Result<RefreshRotation, AppError> {
    if ttl_seconds <= 0 {
        return Err(AppError::Internal(
            "refresh token lifetime must be positive".to_owned(),
        ));
    }
    let token_hash = jwt::hash_refresh_token(presented_token);
    let mut tx = pool.begin().await?;
    let Some(record) = lock_refresh_token(&mut tx, &token_hash).await? else {
        tx.rollback().await?;
        return Ok(RefreshRotation::Rejected);
    };

    let now = Utc::now();
    if record.family_revoked_at.is_some()
        || record.compromised_at.is_some()
        || record.family_expires_at <= now
    {
        tx.commit().await?;
        return Ok(RefreshRotation::Rejected);
    }

    if record.rotated_at.is_some() {
        compromise_family(&mut tx, record.family_id).await?;
        tx.commit().await?;
        return Ok(RefreshRotation::Rejected);
    }

    if record.token_revoked_at.is_some() || record.token_expires_at <= now {
        tx.commit().await?;
        return Ok(RefreshRotation::Rejected);
    }

    let Some(role) = record.role else {
        revoke_family_in_transaction(&mut tx, record.family_id).await?;
        tx.commit().await?;
        return Ok(RefreshRotation::Rejected);
    };
    if !record.is_active {
        revoke_family_in_transaction(&mut tx, record.family_id).await?;
        tx.commit().await?;
        return Ok(RefreshRotation::Rejected);
    }

    let successor_id = Uuid::now_v7();
    let successor_expires_at = (now + Duration::seconds(ttl_seconds)).min(record.family_expires_at);
    if successor_expires_at <= now {
        tx.commit().await?;
        return Ok(RefreshRotation::Rejected);
    }
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (
          id,
          user_id,
          family_id,
          token_hash,
          expires_at,
          predecessor_token_id
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(successor_id)
    .bind(record.user_id)
    .bind(record.family_id)
    .bind(jwt::hash_refresh_token(successor_token))
    .bind(successor_expires_at)
    .bind(record.token_id)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        UPDATE refresh_tokens
        SET rotated_at = $2,
            revoked_at = $2,
            successor_token_id = $3
        WHERE id = $1
        "#,
    )
    .bind(record.token_id)
    .bind(now)
    .bind(successor_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    Ok(RefreshRotation::Rotated {
        issued: IssuedRefreshToken {
            raw_token: successor_token.to_owned(),
            token_id: successor_id,
            family_id: record.family_id,
        },
        identity: CurrentAuthIdentity {
            user_id: record.user_id,
            role,
            auth_version: record.auth_version,
        },
    })
}

async fn lock_refresh_token(
    tx: &mut Transaction<'_, Postgres>,
    token_hash: &str,
) -> Result<Option<LockedRefreshToken>, AppError> {
    sqlx::query_as::<_, LockedRefreshToken>(
        r#"
        SELECT rt.id AS token_id,
               rt.family_id,
               rt.user_id,
               rt.expires_at AS token_expires_at,
               rt.revoked_at AS token_revoked_at,
               rt.rotated_at,
               family.expires_at AS family_expires_at,
               family.revoked_at AS family_revoked_at,
               family.compromised_at,
               users.is_active,
               users.auth_version,
               global_role.role
        FROM refresh_tokens rt
        JOIN refresh_token_families family ON family.id = rt.family_id
        JOIN users ON users.id = rt.user_id
        LEFT JOIN LATERAL (
          SELECT roles.name AS role
          FROM user_roles
          JOIN roles ON roles.id = user_roles.role_id
          WHERE user_roles.user_id = users.id
          ORDER BY CASE roles.name
            WHEN 'super_admin' THEN 1
            WHEN 'admin' THEN 2
            WHEN 'editor' THEN 3
            WHEN 'author' THEN 4
            WHEN 'viewer' THEN 5
            ELSE 99
          END
          LIMIT 1
        ) global_role ON TRUE
        WHERE rt.token_hash = $1
        FOR UPDATE OF rt, family, users
        "#,
    )
    .bind(token_hash)
    .fetch_optional(&mut **tx)
    .await
    .map_err(AppError::from)
}

async fn compromise_family(
    tx: &mut Transaction<'_, Postgres>,
    family_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE refresh_token_families
        SET compromised_at = COALESCE(compromised_at, now()),
            revoked_at = COALESCE(revoked_at, now())
        WHERE id = $1
        "#,
    )
    .bind(family_id)
    .execute(&mut **tx)
    .await?;
    revoke_family_tokens(tx, family_id).await
}

async fn revoke_family_in_transaction(
    tx: &mut Transaction<'_, Postgres>,
    family_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE refresh_token_families
        SET revoked_at = COALESCE(revoked_at, now())
        WHERE id = $1
        "#,
    )
    .bind(family_id)
    .execute(&mut **tx)
    .await?;
    revoke_family_tokens(tx, family_id).await
}

async fn revoke_family_tokens(
    tx: &mut Transaction<'_, Postgres>,
    family_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE refresh_tokens
        SET revoked_at = COALESCE(revoked_at, now())
        WHERE family_id = $1
        "#,
    )
    .bind(family_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn revoke_refresh_family(pool: &PgPool, presented_token: &str) -> Result<bool, AppError> {
    let token_hash = jwt::hash_refresh_token(presented_token);
    let mut tx = pool.begin().await?;
    let family_id: Option<Uuid> = sqlx::query_scalar(
        r#"
        SELECT family.id
        FROM refresh_tokens token
        JOIN refresh_token_families family ON family.id = token.family_id
        WHERE token.token_hash = $1
        FOR UPDATE OF token, family
        "#,
    )
    .bind(token_hash)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(family_id) = family_id else {
        tx.rollback().await?;
        return Ok(false);
    };
    revoke_family_in_transaction(&mut tx, family_id).await?;
    tx.commit().await?;
    Ok(true)
}

pub async fn load_current_auth_identity(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<CurrentAuthIdentity>, AppError> {
    sqlx::query_as::<_, CurrentAuthIdentity>(
        r#"
        SELECT users.id AS user_id,
               roles.name AS role,
               users.auth_version
        FROM users
        JOIN user_roles ON user_roles.user_id = users.id
        JOIN roles ON roles.id = user_roles.role_id
        WHERE users.id = $1
          AND users.is_active = true
        ORDER BY CASE roles.name
          WHEN 'super_admin' THEN 1
          WHEN 'admin' THEN 2
          WHEN 'editor' THEN 3
          WHEN 'author' THEN 4
          WHEN 'viewer' THEN 5
          ELSE 99
        END
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
}

pub async fn validate_access_claims(pool: &PgPool, mut claims: Claims) -> Result<Claims, AppError> {
    let identity = load_current_auth_identity(pool, claims.sub)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid bearer token".to_owned()))?;
    if claims.ver != identity.auth_version || claims.role != identity.role {
        return Err(AppError::Unauthorized("invalid bearer token".to_owned()));
    }
    claims.role = identity.role;
    claims.ver = identity.auth_version;
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::PgPool;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use crate::middleware::auth::Claims;

    use super::{
        RefreshRotation, issue_refresh_family, load_current_auth_identity, revoke_refresh_family,
        rotate_refresh_token, rotate_refresh_token_with_successor, validate_access_claims,
    };

    async fn phase2_pool() -> Option<PgPool> {
        let database_url = env::var("PHASE2_TEST_DATABASE_URL").ok()?;
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("the dedicated Phase 2 database must be reachable");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Phase 2 migrations must apply to the dedicated database");
        Some(pool)
    }

    async fn create_user(pool: &PgPool) -> Uuid {
        let user_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, name)
            VALUES ($1, 'test-password-hash', 'Phase 2 Test User')
            RETURNING id
            "#,
        )
        .bind(format!("phase2-{}@example.invalid", Uuid::now_v7()))
        .fetch_one(pool)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'author'
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
        user_id
    }

    async fn delete_user(pool: &PgPool, user_id: Uuid) {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn refresh_family_rotation_reuse_and_rollback_are_atomic() {
        let Some(pool) = phase2_pool().await else {
            return;
        };
        let user_id = create_user(&pool).await;

        let initial = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        let stored_hash: String =
            sqlx::query_scalar("SELECT token_hash FROM refresh_tokens WHERE id = $1")
                .bind(initial.token_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_ne!(stored_hash, initial.raw_token);
        assert!(!stored_hash.contains(&initial.raw_token));

        let rotated = rotate_refresh_token(&pool, &initial.raw_token, 3600)
            .await
            .unwrap();
        let successor = match rotated {
            RefreshRotation::Rotated { issued, identity } => {
                assert_eq!(identity.user_id, user_id);
                assert_eq!(identity.role, "author");
                issued
            }
            RefreshRotation::Rejected => panic!("the current token must rotate"),
        };
        let predecessor_successor: Option<Uuid> =
            sqlx::query_scalar("SELECT successor_token_id FROM refresh_tokens WHERE id = $1")
                .bind(initial.token_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(predecessor_successor, Some(successor.token_id));

        let reused = rotate_refresh_token(&pool, &initial.raw_token, 3600)
            .await
            .unwrap();
        assert!(matches!(reused, RefreshRotation::Rejected));
        let compromised_at: Option<chrono::DateTime<chrono::Utc>> =
            sqlx::query_scalar("SELECT compromised_at FROM refresh_token_families WHERE id = $1")
                .bind(initial.family_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(compromised_at.is_some());
        assert!(matches!(
            rotate_refresh_token(&pool, &successor.raw_token, 3600)
                .await
                .unwrap(),
            RefreshRotation::Rejected
        ));

        let concurrent = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        let (first, second) = tokio::join!(
            rotate_refresh_token(&pool, &concurrent.raw_token, 3600),
            rotate_refresh_token(&pool, &concurrent.raw_token, 3600)
        );
        let successful_rotations = [first.unwrap(), second.unwrap()]
            .into_iter()
            .filter(|result| matches!(result, RefreshRotation::Rotated { .. }))
            .count();
        assert_eq!(successful_rotations, 1);

        let rollback = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        let failed = rotate_refresh_token_with_successor(
            &pool,
            &rollback.raw_token,
            &rollback.raw_token,
            3600,
        )
        .await;
        assert!(failed.is_err());
        let rotation_time: Option<chrono::DateTime<chrono::Utc>> =
            sqlx::query_scalar("SELECT rotated_at FROM refresh_tokens WHERE id = $1")
                .bind(rollback.token_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(rotation_time.is_none());
        assert!(matches!(
            rotate_refresh_token(&pool, &rollback.raw_token, 3600)
                .await
                .unwrap(),
            RefreshRotation::Rotated { .. }
        ));

        let unrelated = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        assert!(matches!(
            rotate_refresh_token(&pool, &unrelated.raw_token, 3600)
                .await
                .unwrap(),
            RefreshRotation::Rotated { .. }
        ));

        delete_user(&pool, user_id).await;
    }

    #[tokio::test]
    async fn expired_revoked_and_deactivated_sessions_are_rejected() {
        let Some(pool) = phase2_pool().await else {
            return;
        };
        let user_id = create_user(&pool).await;

        let expired = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        sqlx::query(
            r#"
            UPDATE refresh_token_families
            SET created_at = now() - INTERVAL '2 hours',
                expires_at = now() - INTERVAL '1 hour'
            WHERE id = $1
            "#,
        )
        .bind(expired.family_id)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "UPDATE refresh_tokens SET expires_at = now() - INTERVAL '1 hour' WHERE id = $1",
        )
        .bind(expired.token_id)
        .execute(&pool)
        .await
        .unwrap();
        assert!(matches!(
            rotate_refresh_token(&pool, &expired.raw_token, 3600)
                .await
                .unwrap(),
            RefreshRotation::Rejected
        ));

        let revoked = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        assert!(
            revoke_refresh_family(&pool, &revoked.raw_token)
                .await
                .unwrap()
        );
        assert!(matches!(
            rotate_refresh_token(&pool, &revoked.raw_token, 3600)
                .await
                .unwrap(),
            RefreshRotation::Rejected
        ));

        let inactive = issue_refresh_family(&pool, user_id, 3600).await.unwrap();
        sqlx::query("UPDATE users SET is_active = false WHERE id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
        assert!(matches!(
            rotate_refresh_token(&pool, &inactive.raw_token, 3600)
                .await
                .unwrap(),
            RefreshRotation::Rejected
        ));

        delete_user(&pool, user_id).await;
    }

    #[tokio::test]
    async fn access_claims_follow_active_state_version_and_global_role() {
        let Some(pool) = phase2_pool().await else {
            return;
        };
        let user_id = create_user(&pool).await;
        let identity = load_current_auth_identity(&pool, user_id)
            .await
            .unwrap()
            .unwrap();
        let claims = Claims {
            sub: user_id,
            role: identity.role.clone(),
            ver: identity.auth_version,
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
        };
        assert!(validate_access_claims(&pool, claims.clone()).await.is_ok());

        sqlx::query("UPDATE users SET is_active = false WHERE id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
        assert!(validate_access_claims(&pool, claims.clone()).await.is_err());
        sqlx::query("UPDATE users SET is_active = true WHERE id = $1")
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
        assert!(validate_access_claims(&pool, claims.clone()).await.is_err());

        let mut tx = pool.begin().await.unwrap();
        sqlx::query(
            r#"
            DELETE FROM user_roles
            WHERE user_id = $1
              AND role_id = (SELECT id FROM roles WHERE name = 'author')
            "#,
        )
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT $1, id FROM roles WHERE name = 'viewer'
            "#,
        )
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .unwrap();
        tx.commit().await.unwrap();

        assert!(validate_access_claims(&pool, claims).await.is_err());
        let updated = load_current_auth_identity(&pool, user_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.role, "viewer");
        let updated_claims = Claims {
            sub: user_id,
            role: updated.role.clone(),
            ver: updated.auth_version,
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
        };
        let authoritative = validate_access_claims(&pool, updated_claims).await.unwrap();
        assert_eq!(authoritative.role, "viewer");

        let stale_claims = Claims {
            ver: authoritative.ver - 1,
            ..authoritative.clone()
        };
        assert!(validate_access_claims(&pool, stale_claims).await.is_err());
        let unknown_claims = Claims {
            sub: Uuid::now_v7(),
            ..authoritative
        };
        assert!(validate_access_claims(&pool, unknown_claims).await.is_err());

        delete_user(&pool, user_id).await;
    }
}
