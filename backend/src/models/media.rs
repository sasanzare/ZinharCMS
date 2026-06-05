use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct Media {
    pub id: Uuid,
    pub filename: String,
    pub url: String,
    pub mime_type: String,
    pub size: i64,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub uploader_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct MediaVariant {
    pub id: Uuid,
    pub media_id: Uuid,
    pub variant_name: String,
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}
