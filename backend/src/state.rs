use std::collections::HashMap;
use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;

use crate::config::Config;

pub type PagePreviewChannels = Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: PgPool,
    pub redis: redis::Client,
    pub page_preview_channels: PagePreviewChannels,
}

impl AppState {
    pub fn new(config: Config, db: PgPool, redis: redis::Client) -> Self {
        Self {
            config: Arc::new(config),
            db,
            redis,
            page_preview_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
