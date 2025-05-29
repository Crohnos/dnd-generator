use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Npc {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub role: Option<String>,
    pub description: Option<String>,
    pub personality: JsonValue,
    pub stats: JsonValue,
    pub secret_info: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNpcRequest {
    pub campaign_id: i32,
    pub name: String,
    pub role: Option<String>,
    pub description: Option<String>,
    pub personality: Option<JsonValue>,
    pub stats: Option<JsonValue>,
    pub secret_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNpcRequest {
    pub name: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    pub personality: Option<JsonValue>,
    pub stats: Option<JsonValue>,
    pub secret_info: Option<String>,
}