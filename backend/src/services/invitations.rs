use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::error::AppError;
use crate::services::{jwt, security_audit};

#[derive(Clone, Debug, FromRow)]
pub struct AcceptedInvitation {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
}

pub async fn accept_invitation(
    pool: &PgPool,
    user_id: Uuid,
    raw_token: &str,
) -> Result<Option<AcceptedInvitation>, AppError> {
    if raw_token.len() != 43 {
        return Ok(None);
    }
    let token_hash = jwt::hash_refresh_token(raw_token);
    let mut tx = pool.begin().await?;
    let invitation = sqlx::query_as::<_, AcceptedInvitation>(
        r#"
        SELECT invitation.id,
               invitation.organization_id,
               invitation.role::text AS role
        FROM organization_invitations invitation
        JOIN users recipient
          ON lower(recipient.email::text) = lower(invitation.email::text)
        JOIN organizations organization
          ON organization.id = invitation.organization_id
        WHERE invitation.token_hash = $1
          AND invitation.status = 'pending'::organization_invitation_status
          AND invitation.expires_at > now()
          AND recipient.id = $2
          AND recipient.is_active = true
          AND organization.status = 'active'::organization_status
        FOR UPDATE OF invitation, organization
        "#,
    )
    .bind(token_hash)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(invitation) = invitation else {
        tx.rollback().await?;
        return Ok(None);
    };

    sqlx::query(
        r#"
        SELECT set_config('zinhar.organization_id', $1, true),
               set_config('zinhar.user_id', $2, true),
               set_config('zinhar.rls_bypass', 'false', true)
        "#,
    )
    .bind(invitation.organization_id.to_string())
    .bind(user_id.to_string())
    .execute(&mut *tx)
    .await?;
    let member_limit: i32 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(
          (
            SELECT plan.member_limit
            FROM organization_subscriptions subscription
            JOIN plans plan ON plan.id = subscription.plan_id
            WHERE subscription.organization_id = $1
              AND subscription.status IN (
                'trialing'::organization_subscription_status,
                'active'::organization_subscription_status,
                'past_due'::organization_subscription_status
              )
            LIMIT 1
          ),
          (
            SELECT plan.member_limit
            FROM plans plan
            WHERE plan.slug = 'free'
              AND plan.is_active = true
            LIMIT 1
          )
        )
        "#,
    )
    .bind(invitation.organization_id)
    .fetch_one(&mut *tx)
    .await?;
    if member_limit >= 0 {
        let active_members: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM organization_members
            WHERE organization_id = $1
              AND status = 'active'::organization_member_status
            "#,
        )
        .bind(invitation.organization_id)
        .fetch_one(&mut *tx)
        .await?;
        if active_members >= i64::from(member_limit) {
            tx.rollback().await?;
            return Err(AppError::Validation(
                "organization member quota is exhausted".to_owned(),
            ));
        }
    }

    sqlx::query(
        r#"
        INSERT INTO organization_members (
          organization_id,
          user_id,
          role,
          status,
          joined_at
        )
        VALUES (
          $1,
          $2,
          $3::organization_member_role,
          'active'::organization_member_status,
          now()
        )
        ON CONFLICT (organization_id, user_id) DO UPDATE
        SET role = EXCLUDED.role,
            status = 'active'::organization_member_status,
            joined_at = COALESCE(organization_members.joined_at, now()),
            updated_at = now()
        "#,
    )
    .bind(invitation.organization_id)
    .bind(user_id)
    .bind(&invitation.role)
    .execute(&mut *tx)
    .await?;
    let consumed = sqlx::query(
        r#"
        UPDATE organization_invitations
        SET status = 'accepted'::organization_invitation_status,
            token_hash = NULL,
            accepted_at = now(),
            updated_at = now()
        WHERE id = $1
          AND status = 'pending'::organization_invitation_status
        "#,
    )
    .bind(invitation.id)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if consumed != 1 {
        tx.rollback().await?;
        return Ok(None);
    }
    security_audit::record_in_transaction(
        &mut tx,
        "security.invitation.accepted",
        Some(user_id),
        Some(user_id),
        serde_json::json!({
            "organization_id": invitation.organization_id,
            "role": &invitation.role
        }),
    )
    .await?;
    tx.commit().await?;
    Ok(Some(invitation))
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use super::accept_invitation;
    use crate::services::jwt;

    #[tokio::test]
    async fn invitation_is_recipient_role_and_organization_bound_and_consumed_once() {
        let Ok(database_url) = env::var("PHASE5_TEST_DATABASE_URL") else {
            return;
        };
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        let email = format!("phase5-invite-{}@example.invalid", Uuid::now_v7());
        let user_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, password_hash, name)
            VALUES ($1, 'invitation-test-password-hash', 'Phase 5 Invitation Test')
            RETURNING id
            "#,
        )
        .bind(&email)
        .fetch_one(&pool)
        .await
        .unwrap();
        let organization_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO organizations (name, slug, owner_id)
            VALUES ('Phase 5 Invitation Test', $1, $2)
            RETURNING id
            "#,
        )
        .bind(format!("phase5-invite-{}", Uuid::now_v7().simple()))
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        let raw_token = jwt::generate_refresh_token();
        sqlx::query(
            r#"
            INSERT INTO organization_invitations (
              organization_id, email, role, token_hash, status, expires_at
            )
            VALUES (
              $1, $2, 'viewer', $3, 'pending', now() + INTERVAL '1 hour'
            )
            "#,
        )
        .bind(organization_id)
        .bind(&email)
        .bind(jwt::hash_refresh_token(&raw_token))
        .execute(&pool)
        .await
        .unwrap();

        let (first, second) = tokio::join!(
            accept_invitation(&pool, user_id, &raw_token),
            accept_invitation(&pool, user_id, &raw_token)
        );
        let accepted = [first.unwrap(), second.unwrap()]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(accepted.len(), 1);
        assert_eq!(accepted[0].organization_id, organization_id);
        assert_eq!(accepted[0].role, "viewer");
        let stored: (String, Option<String>) = sqlx::query_as(
            r#"
            SELECT status::text, token_hash
            FROM organization_invitations
            WHERE organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(stored.0, "accepted");
        assert!(stored.1.is_none());

        let limited_plan_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO plans (
              slug,
              name,
              member_limit,
              content_limit,
              media_limit_mb,
              api_requests_limit
            )
            VALUES ($1, 'Phase 5 Invitation Limit', 2, -1, -1, -1)
            RETURNING id
            "#,
        )
        .bind(format!("phase5-invite-limit-{}", Uuid::now_v7().simple()))
        .fetch_one(&pool)
        .await
        .unwrap();
        let mut subscription_tx = pool.begin().await.unwrap();
        sqlx::query(
            r#"
            SELECT set_config('zinhar.organization_id', $1, true),
                   set_config('zinhar.user_id', $2, true),
                   set_config('zinhar.rls_bypass', 'false', true)
            "#,
        )
        .bind(organization_id.to_string())
        .bind(user_id.to_string())
        .execute(&mut *subscription_tx)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO organization_subscriptions (
              organization_id,
              plan_id,
              status,
              provider
            )
            VALUES (
              $1,
              $2,
              'active'::organization_subscription_status,
              'phase5-test'
            )
            ON CONFLICT (organization_id) DO UPDATE
            SET plan_id = EXCLUDED.plan_id,
                status = EXCLUDED.status,
                provider = EXCLUDED.provider,
                updated_at = now()
            "#,
        )
        .bind(organization_id)
        .bind(limited_plan_id)
        .execute(&mut *subscription_tx)
        .await
        .unwrap();
        subscription_tx.commit().await.unwrap();

        let mut competing_users = Vec::new();
        for index in 0..2 {
            let competing_email = format!(
                "phase5-invite-race-{index}-{}@example.invalid",
                Uuid::now_v7()
            );
            let competing_user_id: Uuid = sqlx::query_scalar(
                r#"
                INSERT INTO users (email, password_hash, name)
                VALUES ($1, 'invitation-race-password-hash', 'Phase 5 Invitation Race')
                RETURNING id
                "#,
            )
            .bind(&competing_email)
            .fetch_one(&pool)
            .await
            .unwrap();
            let competing_raw_token = jwt::generate_refresh_token();
            sqlx::query(
                r#"
                INSERT INTO organization_invitations (
                  organization_id,
                  email,
                  role,
                  token_hash,
                  status,
                  expires_at
                )
                VALUES (
                  $1,
                  $2,
                  'viewer',
                  $3,
                  'pending',
                  now() + INTERVAL '1 hour'
                )
                "#,
            )
            .bind(organization_id)
            .bind(&competing_email)
            .bind(jwt::hash_refresh_token(&competing_raw_token))
            .execute(&pool)
            .await
            .unwrap();
            competing_users.push((competing_user_id, competing_raw_token));
        }
        let (third, fourth) = tokio::join!(
            accept_invitation(&pool, competing_users[0].0, competing_users[0].1.as_str()),
            accept_invitation(&pool, competing_users[1].0, competing_users[1].1.as_str())
        );
        let competing_successes = [third, fourth]
            .into_iter()
            .filter(|result| matches!(result, Ok(Some(_))))
            .count();
        assert_eq!(competing_successes, 1);
        let member_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM organization_members
            WHERE organization_id = $1
              AND status = 'active'::organization_member_status
            "#,
        )
        .bind(organization_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(member_count, 2);

        sqlx::query("DELETE FROM organizations WHERE id = $1")
            .bind(organization_id)
            .execute(&pool)
            .await
            .unwrap();
        for cleanup_user_id in [user_id, competing_users[0].0, competing_users[1].0] {
            sqlx::query("DELETE FROM users WHERE id = $1")
                .bind(cleanup_user_id)
                .execute(&pool)
                .await
                .unwrap();
        }
        sqlx::query("DELETE FROM plans WHERE id = $1")
            .bind(limited_plan_id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
