use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationType {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub can_have_children: bool,
    pub typical_size: Option<String>,
    pub common_features: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationEnhanced {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub location_type_id: i32,
    pub parent_location_id: Option<i32>,
    pub description: String,
    pub size: Option<String>,
    pub population: Option<i32>,
    pub government: Option<String>,
    pub economy: JsonValue,
    pub culture: JsonValue,
    pub notable_features: JsonValue,
    pub climate: Option<String>,
    pub geography: JsonValue,
    pub defenses: JsonValue,
    pub threats: JsonValue,
    pub resources: JsonValue,
    pub secrets: JsonValue,
    pub atmosphere: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Dungeon {
    pub id: i32,
    pub campaign_id: i32,
    pub location_id: i32,
    pub dungeon_type: String,
    pub size: String,
    pub difficulty: String,
    pub theme: String,
    pub creator: Option<String>,
    pub age: Option<String>,
    pub current_state: String,
    pub levels: i32,
    pub rooms: JsonValue,
    pub traps: JsonValue,
    pub treasures: JsonValue,
    pub inhabitants: JsonValue,
    pub entry_points: JsonValue,
    pub environmental_hazards: JsonValue,
    pub magical_effects: JsonValue,
    pub history: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Shop {
    pub id: i32,
    pub campaign_id: i32,
    pub location_id: i32,
    pub shop_type: String,
    pub owner_entity_id: Option<i32>,
    pub reputation: String,
    pub quality: String,
    pub price_range: String,
    pub specialty_items: JsonValue,
    pub regular_inventory: JsonValue,
    pub services_offered: JsonValue,
    pub operating_hours: JsonValue,
    pub employee_count: i32,
    pub shop_quirks: JsonValue,
    pub customer_base: JsonValue,
    pub business_model: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tavern {
    pub id: i32,
    pub campaign_id: i32,
    pub location_id: i32,
    pub tavern_type: String,
    pub owner_entity_id: Option<i32>,
    pub reputation: String,
    pub clientele: JsonValue,
    pub atmosphere: String,
    pub room_count: i32,
    pub room_quality: String,
    pub food_quality: String,
    pub drink_specialties: JsonValue,
    pub entertainment: JsonValue,
    pub staff: JsonValue,
    pub prices: JsonValue,
    pub notable_patrons: JsonValue,
    pub tavern_secrets: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Temple {
    pub id: i32,
    pub campaign_id: i32,
    pub location_id: i32,
    pub deity_id: Option<i32>,
    pub pantheon_id: Option<i32>,
    pub temple_type: String,
    pub size: String,
    pub architecture_style: String,
    pub clergy_count: i32,
    pub head_priest_entity_id: Option<i32>,
    pub services_offered: JsonValue,
    pub holy_artifacts: JsonValue,
    pub temple_hierarchy: JsonValue,
    pub daily_rituals: JsonValue,
    pub temple_wealth: String,
    pub political_influence: String,
    pub temple_secrets: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationConnection {
    pub id: i32,
    pub campaign_id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub connection_type: String,
    pub distance: Option<f32>,
    pub travel_time: Option<String>,
    pub difficulty: String,
    pub description: Option<String>,
    pub hazards: JsonValue,
    pub landmarks: JsonValue,
    pub cost: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}