use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CalendarSystem {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub months: JsonValue,         // JSONB array of month names/objects
    pub weekdays: JsonValue,       // JSONB array of weekday names
    pub year_length: Option<i32>,  // Total days in a year
    pub current_year: Option<i32>,
    pub current_month: Option<i32>,
    pub current_day: Option<i32>,
    pub special_events: Option<JsonValue>,
    pub lunar_cycles: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorldHistory {
    pub id: i32,
    pub campaign_id: i32,
    pub era_name: String,
    pub start_year: i32,
    pub end_year: i32,
    pub description: String,
    pub major_events: JsonValue,
    pub significance: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Plane {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub plane_type: String,
    pub accessibility: String,
    pub native_creatures: JsonValue,
    pub planar_traits: JsonValue,
    pub notable_locations: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Pantheon {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub origin_culture: Option<String>,
    pub pantheon_type: String,
    pub influence_level: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Deity {
    pub id: i32,
    pub campaign_id: i32,
    pub pantheon_id: Option<i32>,
    pub name: String,
    pub title: Option<String>,
    pub alignment: String,
    pub domains: Vec<String>,
    pub symbol: Option<String>,
    pub description: String,
    pub worshippers: JsonValue,
    pub holy_days: JsonValue,
    pub clergy_structure: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GeographyRegion {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub region_type: String,
    pub climate: String,
    pub terrain: String,
    pub notable_features: JsonValue,
    pub native_flora: JsonValue,
    pub native_fauna: JsonValue,
    pub resources: JsonValue,
    pub hazards: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EconomicSystem {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub economic_type: String,
    pub base_currency: String,
    pub currency_system: JsonValue,
    pub trade_routes: JsonValue,
    pub major_exports: JsonValue,
    pub major_imports: JsonValue,
    pub taxation_system: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LegalSystem {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub government_type: String,
    pub ruling_body: String,
    pub law_enforcement: JsonValue,
    pub court_system: JsonValue,
    pub punishment_system: JsonValue,
    pub citizen_rights: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Astronomy {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub celestial_bodies: JsonValue,
    pub constellations: JsonValue,
    pub astronomical_events: JsonValue,
    pub calendar_influence: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ZodiacSign {
    pub id: i32,
    pub campaign_id: i32,
    pub astronomy_id: i32,
    pub name: String,
    pub symbol: String,
    pub time_period: String,
    pub associated_traits: JsonValue,
    pub influence: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}