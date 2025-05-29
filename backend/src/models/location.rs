use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    #[sqlx(rename = "type")]
    pub location_type: Option<String>,
    pub description: Option<String>,
    pub connections: Vec<i32>,
    pub properties: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub campaign_id: i32,
    pub name: String,
    pub location_type: Option<String>,
    pub description: Option<String>,
    pub connections: Option<Vec<i32>>,
    pub properties: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub location_type: Option<String>,
    pub description: Option<String>,
    pub connections: Option<Vec<i32>>,
    pub properties: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationNpc {
    pub id: i32,
    pub location_id: i32,
    pub npc_id: i32,
    pub relationship_type: String,
    pub created_at: DateTime<Utc>,
}