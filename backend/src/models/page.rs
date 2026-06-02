use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PageStatus {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub page_json: Value,
    pub status: PageStatus,
    pub author_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct PageVersion {
    pub id: Uuid,
    pub page_id: Uuid,
    pub version: i32,
    pub page_json: Value,
    pub snapshot_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ComponentRegistryItem {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub props_schema: Value,
    pub is_system: bool,
}
