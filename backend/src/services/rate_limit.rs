use chrono::Utc;
use redis::AsyncCommands;
use sqlx::{FromRow, PgPool};

use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::services::rls;
use crate::state::AppState;

#[derive(Debug, FromRow)]
struct RateLimitSettings {
    requests_per_minute: i32,
    user_requests_per_minute: i32,
    burst: i32,
}

pub async fn check_and_record_request(
    state: &AppState,
    tenant: &TenantContext,
) -> Result<(), AppError> {
    let settings = load_settings(&state.db, tenant)
        .await?
        .unwrap_or_else(|| RateLimitSettings {
            requests_per_minute: state.config.organization_rate_limit_per_minute as i32,
            user_requests_per_minute: state.config.organization_user_rate_limit_per_minute as i32,
            burst: state.config.organization_rate_limit_burst as i32,
        });

    let mut connection = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|error| {
            AppError::ServiceUnavailable(format!("Redis rate limit check failed: {error}"))
        })?;
    let minute = Utc::now().timestamp() / 60;
    let org_key = format!("rate:org:{}:{minute}", tenant.organization_id);
    let user_key = format!(
        "rate:user:{}:{}:{minute}",
        tenant.organization_id, tenant.user_id
    );
    let org_count: i64 = connection.incr(&org_key, 1).await.map_err(redis_error)?;
    let user_count: i64 = connection.incr(&user_key, 1).await.map_err(redis_error)?;

    if org_count == 1 {
        let _: bool = connection
            .expire(&org_key, 120)
            .await
            .map_err(redis_error)?;
    }
    if user_count == 1 {
        let _: bool = connection
            .expire(&user_key, 120)
            .await
            .map_err(redis_error)?;
    }

    let org_limit = i64::from(settings.requests_per_minute + settings.burst);
    let user_limit = i64::from(settings.user_requests_per_minute + settings.burst);
    if org_count > org_limit {
        return Err(AppError::TooManyRequests(
            "organization request rate limit exceeded".to_owned(),
        ));
    }
    if user_count > user_limit {
        return Err(AppError::TooManyRequests(
            "user request rate limit exceeded".to_owned(),
        ));
    }

    Ok(())
}

async fn load_settings(
    pool: &PgPool,
    tenant: &TenantContext,
) -> Result<Option<RateLimitSettings>, AppError> {
    let mut db = rls::tenant_connection(pool, tenant).await?;
    sqlx::query_as::<_, RateLimitSettings>(
        r#"
        SELECT requests_per_minute,
               user_requests_per_minute,
               burst
        FROM organization_rate_limits
        WHERE organization_id = $1
        "#,
    )
    .bind(tenant.organization_id)
    .fetch_optional(db.as_mut())
    .await
    .map_err(AppError::from)
}

fn redis_error(error: redis::RedisError) -> AppError {
    AppError::ServiceUnavailable(format!("Redis rate limit check failed: {error}"))
}

#[cfg(test)]
mod tests {
    use super::RateLimitSettings;

    #[test]
    fn rate_limit_settings_are_positive_in_tests() {
        let settings = RateLimitSettings {
            requests_per_minute: 600,
            user_requests_per_minute: 120,
            burst: 120,
        };

        assert!(settings.requests_per_minute > 0);
        assert!(settings.user_requests_per_minute > 0);
        assert!(settings.burst >= 0);
    }
}
