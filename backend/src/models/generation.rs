use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCampaignContent {
    pub npcs: Vec<GeneratedNPC>,
    pub locations: Vec<GeneratedLocation>,
    pub quest_hooks: Vec<GeneratedQuestHook>,
    pub plot_summary: String,
    pub central_conflict: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedNPC {
    pub name: String,
    pub role: String,
    pub description: String,
    pub personality: NPCPersonality,
    pub stats: Option<NPCStats>,
    pub secret_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCPersonality {
    pub traits: Vec<String>,
    pub motivation: String,
    pub fears: Option<Vec<String>>,
    pub connections: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCStats {
    pub race: String,
    pub class: Option<String>,
    pub level: Option<i32>,
    pub abilities: Option<AbilityScores>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedLocation {
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: String,
    pub description: String,
    pub properties: LocationProperties,
    pub connections: Vec<String>, // Names of connected locations
    pub resident_npcs: Vec<String>, // Names of NPCs at this location
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationProperties {
    pub atmosphere: String,
    pub notable_features: Vec<String>,
    pub hidden_elements: Option<Vec<String>>,
    pub danger_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedQuestHook {
    pub title: String,
    pub description: String,
    pub quest_giver: String, // NPC name
    pub objectives: Vec<String>,
    pub reward: String,
    pub difficulty: String,
    pub related_locations: Vec<String>, // Location names
    pub related_npcs: Vec<String>, // NPC names
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedEncounter {
    pub name: String,
    pub description: String,
    pub location: Option<String>, // Location name
    pub difficulty: String,
    pub enemies: Vec<Enemy>,
    pub environmental_factors: Option<String>,
    pub possible_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub count: i32,
    pub hp: i32,
    pub ac: i32,
    pub special_abilities: Option<Vec<String>>,
}