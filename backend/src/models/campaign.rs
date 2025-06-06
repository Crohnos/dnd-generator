use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

// Phase constants for 9-phase system
pub const TOTAL_PHASES: i32 = 9;

pub const PHASE_1A_CORE_WORLD: &str = "phase_1a_core_world";
pub const PHASE_1B_CHARACTER_BUILDING: &str = "phase_1b_character_building";
pub const PHASE_1C_SOCIAL_FRAMEWORK: &str = "phase_1c_social_framework";
pub const PHASE_2A_PC_ENTITIES: &str = "phase_2a_pc_entities";
pub const PHASE_2B_PC_LOCATIONS: &str = "phase_2b_pc_locations";
pub const PHASE_2C_PC_ITEMS: &str = "phase_2c_pc_items";
pub const PHASE_3A_QUESTS_ENCOUNTERS: &str = "phase_3a_quests_encounters";
pub const PHASE_3B_WORLD_POPULATION: &str = "phase_3b_world_population";
pub const PHASE_3C_RELATIONSHIPS: &str = "phase_3c_relationships";

#[derive(Debug, Clone)]
pub struct PhaseInfo {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub tables: Vec<String>,
    pub number: i32,
}

impl PhaseInfo {
    pub fn get_all_phases() -> Vec<PhaseInfo> {
        vec![
            PhaseInfo {
                name: PHASE_1A_CORE_WORLD.to_string(),
                description: "Core world systems: calendar, planes, geography, history, economics, legal systems, astronomy".to_string(),
                dependencies: vec![], // No dependencies
                tables: vec![
                    "calendar_systems".to_string(),
                    "planes".to_string(),
                    "geography_regions".to_string(),
                    "historical_periods".to_string(),
                    "economic_systems".to_string(),
                    "legal_systems".to_string(),
                    "celestial_bodies".to_string(),
                ],
                number: 1,
            },
            PhaseInfo {
                name: PHASE_1B_CHARACTER_BUILDING.to_string(),
                description: "Character building systems: races, classes, feats, backgrounds".to_string(),
                dependencies: vec![PHASE_1A_CORE_WORLD.to_string()],
                tables: vec![
                    "races".to_string(),
                    "character_classes".to_string(),
                    "feats".to_string(),
                    "backgrounds".to_string(),
                ],
                number: 2,
            },
            PhaseInfo {
                name: PHASE_1C_SOCIAL_FRAMEWORK.to_string(),
                description: "Social framework: languages, cultures, factions, pantheons, deities".to_string(),
                dependencies: vec![PHASE_1A_CORE_WORLD.to_string(), PHASE_1B_CHARACTER_BUILDING.to_string()],
                tables: vec![
                    "languages".to_string(),
                    "cultures".to_string(),
                    "factions".to_string(),
                    "pantheons".to_string(),
                    "deities".to_string(),
                ],
                number: 3,
            },
            PhaseInfo {
                name: PHASE_2A_PC_ENTITIES.to_string(),
                description: "PC-connected entities: NPCs with direct PC connections".to_string(),
                dependencies: vec![PHASE_1A_CORE_WORLD.to_string(), PHASE_1B_CHARACTER_BUILDING.to_string(), PHASE_1C_SOCIAL_FRAMEWORK.to_string()],
                tables: vec!["entities".to_string()],
                number: 4,
            },
            PhaseInfo {
                name: PHASE_2B_PC_LOCATIONS.to_string(),
                description: "PC-connected locations: hierarchical places tied to PC backstories".to_string(),
                dependencies: vec![PHASE_2A_PC_ENTITIES.to_string()],
                tables: vec![
                    "locations".to_string(),
                    "dungeons".to_string(),
                    "buildings".to_string(),
                ],
                number: 5,
            },
            PhaseInfo {
                name: PHASE_2C_PC_ITEMS.to_string(),
                description: "PC-connected items: equipment and artifacts relevant to PC stories".to_string(),
                dependencies: vec![PHASE_2A_PC_ENTITIES.to_string(), PHASE_2B_PC_LOCATIONS.to_string()],
                tables: vec![
                    "items".to_string(),
                    "item_effects".to_string(),
                    "sentient_item_properties".to_string(),
                ],
                number: 6,
            },
            PhaseInfo {
                name: PHASE_3A_QUESTS_ENCOUNTERS.to_string(),
                description: "Quest hooks and encounters: adventures and challenges".to_string(),
                dependencies: vec![PHASE_2A_PC_ENTITIES.to_string(), PHASE_2B_PC_LOCATIONS.to_string(), PHASE_2C_PC_ITEMS.to_string()],
                tables: vec![
                    "quest_hooks".to_string(),
                    "encounters".to_string(),
                ],
                number: 7,
            },
            PhaseInfo {
                name: PHASE_3B_WORLD_POPULATION.to_string(),
                description: "World population: additional NPCs, shops, taverns, temples".to_string(),
                dependencies: vec![PHASE_2B_PC_LOCATIONS.to_string()],
                tables: vec![
                    "shops".to_string(),
                    "taverns".to_string(),
                    "temples".to_string(),
                ],
                number: 8,
            },
            PhaseInfo {
                name: PHASE_3C_RELATIONSHIPS.to_string(),
                description: "Final relationships: all entity-to-entity, entity-to-location, faction connections".to_string(),
                dependencies: vec![PHASE_3A_QUESTS_ENCOUNTERS.to_string(), PHASE_3B_WORLD_POPULATION.to_string()],
                tables: vec![
                    "entity_relationships".to_string(),
                    "entity_locations".to_string(),
                    "entity_factions".to_string(),
                    "faction_relationships".to_string(),
                    "entity_items".to_string(),
                    "location_items".to_string(),
                ],
                number: 9,
            },
        ]
    }

    pub fn get_phase_info(phase_name: &str) -> Option<PhaseInfo> {
        let all_phases = Self::get_all_phases();
        all_phases.into_iter().find(|p| p.name == phase_name)
    }

    pub fn validate_dependencies(completed_phases: &[String], target_phase: &str) -> Result<(), String> {
        let all_phases = Self::get_all_phases();
        let target_phase_info = all_phases
            .iter()
            .find(|p| p.name == target_phase)
            .ok_or_else(|| format!("Unknown phase: {}", target_phase))?;

        for dependency in &target_phase_info.dependencies {
            if !completed_phases.contains(dependency) {
                return Err(format!(
                    "Phase '{}' requires '{}' to be completed first",
                    target_phase, dependency
                ));
            }
        }

        Ok(())
    }
}

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
    pub use_standard_content: Option<bool>,
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