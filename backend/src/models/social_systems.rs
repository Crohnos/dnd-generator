use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Faction {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub faction_type: String,
    pub description: String,
    pub goals: JsonValue,
    pub methods: JsonValue,
    pub alignment: String,
    pub size: String,
    pub influence: String,
    pub resources: JsonValue,
    pub territory: JsonValue,
    pub leadership_structure: JsonValue,
    pub notable_members: JsonValue,
    pub allies: JsonValue,
    pub enemies: JsonValue,
    pub secrets: JsonValue,
    pub public_reputation: String,
    pub recruitment_methods: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Culture {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub primary_race: Option<String>,
    pub values: JsonValue,
    pub traditions: JsonValue,
    pub social_structure: JsonValue,
    pub government_preference: Option<String>,
    pub economic_focus: JsonValue,
    pub religious_beliefs: JsonValue,
    pub art_and_music: JsonValue,
    pub clothing_style: JsonValue,
    pub cuisine: JsonValue,
    pub architecture: JsonValue,
    pub language_id: Option<i32>,
    pub notable_achievements: JsonValue,
    pub historical_events: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityRelationship {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_a_id: i32,
    pub entity_b_id: i32,
    pub relationship_type: String,
    pub description: Option<String>,
    pub strength: String,
    pub status: String,
    pub history: JsonValue,
    pub secrets: JsonValue,
    pub public_knowledge: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FactionRelationship {
    pub id: i32,
    pub campaign_id: i32,
    pub faction_a_id: i32,
    pub faction_b_id: i32,
    pub relationship_type: String,
    pub description: Option<String>,
    pub strength: String,
    pub status: String,
    pub history: JsonValue,
    pub treaties: JsonValue,
    pub ongoing_conflicts: JsonValue,
    pub trade_relations: JsonValue,
    pub public_knowledge: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityLocation {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub location_id: i32,
    pub relationship_type: String,
    pub description: Option<String>,
    pub frequency: String,
    pub time_periods: JsonValue,
    pub role_at_location: Option<String>,
    pub secrets: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityFaction {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub faction_id: i32,
    pub rank: Option<String>,
    pub role: Option<String>,
    pub loyalty_level: String,
    pub join_date: Option<DateTime<Utc>>,
    pub contributions: JsonValue,
    pub secrets_known: JsonValue,
    pub reputation_within: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityItem {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub item_id: i32,
    pub relationship_type: String,
    pub quantity: i32,
    pub condition_state: String,
    pub how_acquired: Option<String>,
    pub sentimental_value: Option<String>,
    pub location_stored: Option<String>,
    pub attuned: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EntityCulture {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub culture_id: i32,
    pub relationship_type: String,
    pub integration_level: String,
    pub cultural_knowledge: JsonValue,
    pub cultural_practices: JsonValue,
    pub cultural_conflicts: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}