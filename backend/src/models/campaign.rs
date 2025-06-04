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
    pub generation_phase: Option<String>,
    pub phase_progress: i32,
    pub total_phases: i32,
    pub current_phase_status: Option<String>,
    pub error_message: Option<String>,
    pub progression_type: String,
    pub tone: String,
    pub difficulty: String,
    pub starting_level: i32,
    pub campaign_length: String,
    pub additional_notes: Option<String>,
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
    pub progression_type: Option<String>,
    pub tone: Option<String>,
    pub difficulty: Option<String>,
    pub starting_level: Option<i32>,
    pub campaign_length: Option<String>,
    pub additional_notes: Option<String>,
    pub metadata: Option<JsonValue>,
    // Enhanced fields for world building
    pub world_building: Option<WorldBuildingConfig>,
    pub campaign_specifics: Option<CampaignSpecifics>,
    pub generation_preferences: Option<GenerationPreferences>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldBuildingConfig {
    pub calendar_system: Option<CalendarConfig>,
    pub pantheon: Option<PantheonConfig>,
    pub geography: Option<GeographyConfig>,
    pub economic_system: Option<EconomicConfig>,
    pub political_landscape: Option<PoliticalConfig>,
    pub historical_context: Option<HistoricalConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarConfig {
    pub name: String,
    pub months_per_year: i32,
    pub days_per_month: i32,
    pub current_year: i32,
    pub special_days: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PantheonConfig {
    pub name: String,
    pub deities: Vec<DeityConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeityConfig {
    pub name: String,
    pub domains: Vec<String>,
    pub alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographyConfig {
    pub continents: Vec<String>,
    pub major_cities: Vec<String>,
    pub climate_zones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicConfig {
    pub currency: CurrencyConfig,
    pub major_trade_goods: Vec<String>,
    pub trade_routes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyConfig {
    pub copper: String,
    pub silver: String,
    pub gold: String,
    pub platinum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalConfig {
    pub government_type: String,
    pub ruling_factions: Vec<FactionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionConfig {
    pub name: String,
    pub goals: String,
    pub resources: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalConfig {
    pub major_events: Vec<HistoricalEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEvent {
    pub name: String,
    pub years_ago: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignSpecifics {
    pub starting_location: String,
    pub initial_quest_hooks: Vec<String>,
    pub recurring_villains: Vec<VillainConfig>,
    pub major_locations: Vec<MajorLocationConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VillainConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub villain_type: String,
    pub goals: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorLocationConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: String,
    pub significance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationPreferences {
    pub npc_depth: String,
    pub location_detail: String,
    pub quest_complexity: String,
    pub encounter_variety: String,
    pub magic_item_frequency: String,
    pub faction_involvement: String,
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