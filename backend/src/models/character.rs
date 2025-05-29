use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

// Character Races
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterRace {
    pub id: i32,
    pub name: String,
    pub size_category: Option<String>,
    pub average_lifespan: Option<i32>,
    pub common_alignments: Option<Vec<String>>,
    pub physical_traits: Option<Vec<String>>,
    pub cultural_traits: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub subraces: Option<JsonValue>,
    pub homeland_locations: Option<Vec<i32>>,
    pub created_at: DateTime<Utc>,
}

// Character Backgrounds
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterBackground {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub skill_proficiencies: Option<Vec<String>>,
    pub tool_proficiencies: Option<Vec<String>>,
    pub languages_known: Option<i32>,
    pub equipment: Option<JsonValue>,
    pub feature_name: Option<String>,
    pub feature_description: Option<String>,
    pub personality_traits: Option<Vec<String>>,
    pub ideals: Option<Vec<String>>,
    pub bonds: Option<Vec<String>>,
    pub flaws: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

// Enhanced NPC model (extends existing)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EnhancedNpc {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub role: Option<String>,
    pub description: Option<String>,
    pub personality: Option<JsonValue>,
    pub stats: Option<JsonValue>,
    pub secret_info: Option<String>,
    
    // New enhanced fields
    pub race_id: Option<i32>,
    pub background_id: Option<i32>,
    pub age: Option<i32>,
    pub physical_description: Option<String>,
    pub alignment: Option<String>,
    pub social_class: Option<String>,
    pub occupation: Option<String>,
    pub languages: Option<Vec<String>>,
    pub reputation: Option<i32>,
    pub wealth_level: Option<i32>,
    pub health_status: Option<String>,
    pub current_location_id: Option<i32>,
    pub home_location_id: Option<i32>,
    pub family_members: Option<Vec<i32>>,
    pub allies: Option<Vec<i32>>,
    pub enemies: Option<Vec<i32>>,
    pub goals: Option<Vec<String>>,
    pub fears: Option<Vec<String>>,
    pub secrets: Option<Vec<String>>,
    pub schedule: Option<JsonValue>,
    pub is_dead: Option<bool>,
    pub death_date: Option<NaiveDate>,
    pub death_cause: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// NPC Relationships
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NpcRelationship {
    pub id: i32,
    pub npc1_id: i32,
    pub npc2_id: i32,
    pub relationship_type: String,
    pub relationship_strength: Option<i32>,
    pub description: Option<String>,
    pub is_secret: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNpcRequest {
    pub campaign_id: i32,
    pub name: String,
    pub role: Option<String>,
    pub description: Option<String>,
    pub race_id: Option<i32>,
    pub background_id: Option<i32>,
    pub age: Option<i32>,
    pub physical_description: Option<String>,
    pub alignment: Option<String>,
    pub occupation: Option<String>,
    pub current_location_id: Option<i32>,
    pub home_location_id: Option<i32>,
    pub goals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRelationshipRequest {
    pub npc1_id: i32,
    pub npc2_id: i32,
    pub relationship_type: String,
    pub relationship_strength: Option<i32>,
    pub description: Option<String>,
    pub is_secret: Option<bool>,
}

// Trait for common NPC operations
pub trait NpcOperations {
    fn get_full_name(&self) -> &str;
    fn get_location_name(&self) -> Option<String>;
    fn is_available(&self) -> bool;
    fn get_reputation_level(&self) -> String;
}

impl NpcOperations for EnhancedNpc {
    fn get_full_name(&self) -> &str {
        &self.name
    }
    
    fn get_location_name(&self) -> Option<String> {
        // This would be populated by joining with locations table
        None
    }
    
    fn is_available(&self) -> bool {
        !self.is_dead.unwrap_or(false) && self.health_status.as_deref() != Some("unconscious")
    }
    
    fn get_reputation_level(&self) -> String {
        match self.reputation.unwrap_or(0) {
            r if r >= 8 => "Legendary".to_string(),
            r if r >= 5 => "Well Known".to_string(),
            r if r >= 0 => "Neutral".to_string(),
            r if r >= -5 => "Disliked".to_string(),
            _ => "Despised".to_string(),
        }
    }
}