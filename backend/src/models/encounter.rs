use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Encounter {
    pub id: i32,
    pub campaign_id: i32,
    pub location_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: String,
    pub creatures: JsonValue,
    pub environmental_factors: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEncounterRequest {
    pub campaign_id: i32,
    pub location_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub creatures: Option<JsonValue>,
    pub environmental_factors: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEncounterRequest {
    pub location_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub creatures: Option<JsonValue>,
    pub environmental_factors: Option<String>,
}