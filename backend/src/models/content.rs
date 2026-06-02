use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    Draft,
    PendingReview,
    Published,
    Archived,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ContentType {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub fields: Value,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ContentEntry {
    pub id: Uuid,
    pub type_id: Uuid,
    pub data: Value,
    pub status: ContentStatus,
    pub version: i32,
    pub author_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
