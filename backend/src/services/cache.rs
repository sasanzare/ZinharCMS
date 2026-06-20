use std::future::Future;

use redis::AsyncCommands;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::AppError;

pub const DEFAULT_TTL_SECONDS: u64 = 300;

pub async fn get_or_set_json<T, F, Fut>(
    redis: &redis::Client,
    key: &str,
    ttl_seconds: u64,
    fetch: F,
) -> Result<T, AppError>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, AppError>>,
{
    let Ok(mut connection) = redis.get_multiplexed_async_connection().await else {
        return fetch().await;
    };

    if let Ok(cached) = connection.get::<_, String>(key).await
        && let Ok(value) = serde_json::from_str::<T>(&cached)
    {
        return Ok(value);
    }

    let value = fetch().await?;
    if let Ok(serialized) = serde_json::to_string(&value) {
        let _ = connection
            .set_ex::<_, _, ()>(key, serialized, ttl_seconds)
            .await;
    }

    Ok(value)
}

pub async fn invalidate(redis: &redis::Client, key: &str) {
    let Ok(mut connection) = redis.get_multiplexed_async_connection().await else {
        return;
    };
    let _ = connection.del::<_, ()>(key).await;
}

pub async fn invalidate_prefix(redis: &redis::Client, prefix: &str) {
    let Ok(mut connection) = redis.get_multiplexed_async_connection().await else {
        return;
    };
    let pattern = format!("{prefix}*");
    let Ok(keys) = redis::cmd("KEYS")
        .arg(&pattern)
        .query_async::<Vec<String>>(&mut connection)
        .await
    else {
        return;
    };
    if !keys.is_empty() {
        let _ = connection.del::<_, ()>(keys).await;
    }
}
