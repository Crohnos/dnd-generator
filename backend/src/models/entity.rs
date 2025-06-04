use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Entity {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub entity_type: String, // 'pc', 'npc', 'creature', 'flora', 'fauna'
    pub description: Option<String>,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerCharacter {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub player_name: String,
    pub race_id: Option<i32>,
    pub subrace_id: Option<i32>,
    pub class_id: Option<i32>,
    pub subclass_id: Option<i32>,
    pub background_id: Option<i32>,
    pub level: i32,
    pub experience_points: i32,
    pub hit_points: i32,
    pub armor_class: i32,
    pub ability_scores: JsonValue,
    pub skills: JsonValue,
    pub languages: Vec<String>,
    pub equipment: JsonValue,
    pub spells: JsonValue,
    pub backstory: Option<String>,
    pub personality_traits: JsonValue,
    pub ideals: JsonValue,
    pub bonds: JsonValue,
    pub flaws: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NpcEnhanced {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub race_id: Option<i32>,
    pub class_id: Option<i32>,
    pub occupation: Option<String>,
    pub level: Option<i32>,
    pub alignment: Option<String>,
    pub hit_points: Option<i32>,
    pub armor_class: Option<i32>,
    pub ability_scores: JsonValue,
    pub skills: JsonValue,
    pub languages: Vec<String>,
    pub equipment: JsonValue,
    pub personality_traits: JsonValue,
    pub ideals: JsonValue,
    pub bonds: JsonValue,
    pub flaws: JsonValue,
    pub backstory: Option<String>,
    pub role_in_story: Option<String>,
    pub relationship_to_pcs: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Creature {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub creature_type: String,
    pub size: String,
    pub alignment: String,
    pub armor_class: i32,
    pub hit_points: i32,
    pub hit_dice: String,
    pub speed: JsonValue,
    pub ability_scores: JsonValue,
    pub saving_throws: JsonValue,
    pub skills: JsonValue,
    pub damage_resistances: JsonValue,
    pub damage_immunities: JsonValue,
    pub condition_immunities: JsonValue,
    pub senses: JsonValue,
    pub languages: Vec<String>,
    pub challenge_rating: f32,
    pub proficiency_bonus: i32,
    pub actions: JsonValue,
    pub legendary_actions: JsonValue,
    pub lair_actions: JsonValue,
    pub regional_effects: JsonValue,
    pub habitat: JsonValue,
    pub diet: Option<String>,
    pub behavior: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Flora {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub plant_type: String,
    pub size: String,
    pub rarity: String,
    pub habitat: JsonValue,
    pub growing_conditions: JsonValue,
    pub appearance: String,
    pub uses: JsonValue,
    pub magical_properties: JsonValue,
    pub harvest_difficulty: Option<String>,
    pub seasonal_availability: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Fauna {
    pub id: i32,
    pub campaign_id: i32,
    pub entity_id: i32,
    pub animal_type: String,
    pub size: String,
    pub habitat: JsonValue,
    pub diet: String,
    pub behavior: JsonValue,
    pub social_structure: Option<String>,
    pub appearance: String,
    pub abilities: JsonValue,
    pub domestication_status: String,
    pub rarity: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}