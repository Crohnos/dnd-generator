use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestHook {
    pub id: i32,
    pub campaign_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: String,
    pub reward: Option<String>,
    pub related_npc_ids: Vec<i32>,
    pub related_location_ids: Vec<i32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuestHookRequest {
    pub campaign_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub reward: Option<String>,
    pub related_npc_ids: Option<Vec<i32>>,
    pub related_location_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuestHookRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub reward: Option<String>,
    pub related_npc_ids: Option<Vec<i32>>,
    pub related_location_ids: Option<Vec<i32>>,
    pub status: Option<String>,
}