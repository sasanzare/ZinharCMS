use std::sync::Arc;

use sqlx::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: PgPool,
    pub redis: redis::Client,
}

impl AppState {
    pub fn new(config: Config, db: PgPool, redis: redis::Client) -> Self {
        Self {
            config: Arc::new(config),
            db,
            redis,
        }
    }
}
