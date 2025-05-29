use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Campaign {
    pub id: i32,
    pub name: String,
    pub setting: Option<String>,
    pub themes: Vec<String>,
    pub player_characters: JsonValue,
    pub status: String,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub setting: Option<String>,
    pub themes: Vec<String>,
    pub player_characters: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCampaignRequest {
    pub name: Option<String>,
    pub setting: Option<String>,
    pub themes: Option<Vec<String>>,
    pub player_characters: Option<JsonValue>,
    pub status: Option<String>,
    pub metadata: Option<JsonValue>,
}