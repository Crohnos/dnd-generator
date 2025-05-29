use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

// Quest Types
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub typical_rewards: Option<Vec<String>>,
    pub common_complications: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

// Enhanced Quest Hooks
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EnhancedQuestHook {
    pub id: i32,
    pub campaign_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub reward: Option<String>,
    pub related_npc_ids: Option<Vec<i32>>,
    pub related_location_ids: Option<Vec<i32>>,
    pub status: Option<String>,
    
    // New enhanced fields
    pub quest_type_id: Option<i32>,
    pub patron_npc_id: Option<i32>,
    pub patron_organization_id: Option<i32>,
    pub target_location_id: Option<i32>,
    pub time_limit_days: Option<i32>,
    pub urgency_level: Option<i32>,
    pub secrecy_level: Option<i32>,
    pub moral_complexity: Option<i32>,
    pub required_skills: Option<Vec<String>>,
    pub complications: Option<Vec<String>>,
    pub success_consequences: Option<String>,
    pub failure_consequences: Option<String>,
    pub backstory_connections: Option<Vec<i32>>,
    pub prerequisite_quests: Option<Vec<i32>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Quest Progress Tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestProgress {
    pub id: i32,
    pub quest_hook_id: i32,
    pub session_number: Option<i32>,
    pub progress_description: String,
    pub completion_percentage: Option<i32>,
    pub complications_encountered: Option<Vec<String>>,
    pub npcs_met: Option<Vec<i32>>,
    pub locations_visited: Option<Vec<i32>>,
    pub items_gained: Option<Vec<String>>,
    pub information_learned: Option<Vec<String>>,
    pub decisions_made: Option<JsonValue>,
    pub next_steps: Option<Vec<String>>,
    pub dm_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Encounter Types
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EncounterType {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub typical_duration: Option<String>,
    pub common_resolutions: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

// Enhanced Encounters
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EnhancedEncounter {
    pub id: i32,
    pub campaign_id: i32,
    pub location_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub creatures: Option<JsonValue>,
    pub environmental_factors: Option<String>,
    
    // New enhanced fields
    pub encounter_type_id: Option<i32>,
    pub trigger_conditions: Option<Vec<String>>,
    pub npcs_involved: Option<Vec<i32>>,
    pub organizations_involved: Option<Vec<i32>>,
    pub required_skills: Option<Vec<String>>,
    pub possible_outcomes: Option<JsonValue>,
    pub rewards: Option<JsonValue>,
    pub consequences: Option<Vec<String>>,
    pub backstory_relevance: Option<Vec<i32>>,
    pub repeatable: Option<bool>,
    pub scaling_notes: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Random Events
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RandomEvent {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub event_type: Option<String>,
    pub trigger_probability: Option<rust_decimal::Decimal>,
    pub location_types: Option<Vec<String>>,
    pub seasonal_restrictions: Option<Vec<String>>,
    pub prerequisites: Option<Vec<String>>,
    pub immediate_effects: Option<Vec<String>>,
    pub long_term_consequences: Option<Vec<String>>,
    pub affected_locations: Option<Vec<i32>>,
    pub affected_npcs: Option<Vec<i32>>,
    pub affected_organizations: Option<Vec<i32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuestHookRequest {
    pub campaign_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub quest_type_id: Option<i32>,
    pub difficulty: Option<String>,
    pub patron_npc_id: Option<i32>,
    pub target_location_id: Option<i32>,
    pub urgency_level: Option<i32>,
    pub required_skills: Option<Vec<String>>,
    pub backstory_connections: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuestProgressRequest {
    pub quest_hook_id: i32,
    pub session_number: Option<i32>,
    pub progress_description: String,
    pub completion_percentage: Option<i32>,
    pub complications_encountered: Option<Vec<String>>,
    pub decisions_made: Option<JsonValue>,
    pub next_steps: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEncounterRequest {
    pub campaign_id: i32,
    pub location_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub encounter_type_id: Option<i32>,
    pub difficulty: Option<String>,
    pub npcs_involved: Option<Vec<i32>>,
    pub backstory_relevance: Option<Vec<i32>>,
}

// Traits for quest operations
pub trait QuestOperations {
    fn get_urgency_description(&self) -> String;
    fn get_complexity_description(&self) -> String;
    fn is_available(&self) -> bool;
    fn has_backstory_connections(&self) -> bool;
    fn get_time_pressure_description(&self) -> String;
}

impl QuestOperations for EnhancedQuestHook {
    fn get_urgency_description(&self) -> String {
        match self.urgency_level.unwrap_or(3) {
            5 => "Critical - Immediate Action Required".to_string(),
            4 => "High - Time Sensitive".to_string(),
            3 => "Moderate - Should Address Soon".to_string(),
            2 => "Low - When Convenient".to_string(),
            1 => "Background - Long Term".to_string(),
            _ => "Unknown Urgency".to_string(),
        }
    }
    
    fn get_complexity_description(&self) -> String {
        match self.moral_complexity.unwrap_or(3) {
            5 => "Morally Ambiguous - No Clear Right Answer".to_string(),
            4 => "Complex - Multiple Valid Perspectives".to_string(),
            3 => "Moderate - Some Moral Considerations".to_string(),
            2 => "Simple - Clear Good vs Bad".to_string(),
            1 => "Straightforward - Obvious Right Choice".to_string(),
            _ => "Unknown Complexity".to_string(),
        }
    }
    
    fn is_available(&self) -> bool {
        self.status.as_deref() == Some("available") && 
        self.prerequisite_quests.as_ref().map_or(true, |prereqs| prereqs.is_empty())
    }
    
    fn has_backstory_connections(&self) -> bool {
        self.backstory_connections.as_ref().map_or(false, |connections| !connections.is_empty())
    }
    
    fn get_time_pressure_description(&self) -> String {
        match self.time_limit_days {
            Some(days) if days <= 1 => "Immediate - Must Act Today".to_string(),
            Some(days) if days <= 7 => format!("Short Term - {} Days Remaining", days),
            Some(days) if days <= 30 => format!("Medium Term - {} Days Remaining", days),
            Some(days) => format!("Long Term - {} Days Remaining", days),
            None => "No Time Limit".to_string(),
        }
    }
}

pub trait ProgressOperations {
    fn get_progress_stage(&self) -> String;
    fn is_complete(&self) -> bool;
    fn get_next_milestone(&self) -> Option<String>;
}

impl ProgressOperations for QuestProgress {
    fn get_progress_stage(&self) -> String {
        match self.completion_percentage.unwrap_or(0) {
            100 => "Complete".to_string(),
            75..=99 => "Nearly Complete".to_string(),
            50..=74 => "Significant Progress".to_string(),
            25..=49 => "Making Progress".to_string(),
            1..=24 => "Just Started".to_string(),
            _ => "Not Started".to_string(),
        }
    }
    
    fn is_complete(&self) -> bool {
        self.completion_percentage.unwrap_or(0) >= 100
    }
    
    fn get_next_milestone(&self) -> Option<String> {
        self.next_steps.as_ref()?.first().cloned()
    }
}

pub trait EncounterOperations {
    fn get_difficulty_description(&self) -> String;
    fn is_combat_encounter(&self) -> bool;
    fn has_backstory_relevance(&self) -> bool;
    fn can_be_repeated(&self) -> bool;
}

impl EncounterOperations for EnhancedEncounter {
    fn get_difficulty_description(&self) -> String {
        match self.difficulty.as_deref() {
            Some("trivial") => "Trivial - Automatic Success".to_string(),
            Some("easy") => "Easy - Low Risk".to_string(),
            Some("medium") => "Medium - Moderate Challenge".to_string(),
            Some("hard") => "Hard - Significant Risk".to_string(),
            Some("deadly") => "Deadly - Life Threatening".to_string(),
            _ => "Unknown Difficulty".to_string(),
        }
    }
    
    fn is_combat_encounter(&self) -> bool {
        // Check if this is a combat encounter based on encounter type
        self.creatures.is_some() && 
        self.creatures.as_ref().map_or(false, |c| !c.as_array().unwrap_or(&vec![]).is_empty())
    }
    
    fn has_backstory_relevance(&self) -> bool {
        self.backstory_relevance.as_ref().map_or(false, |relevance| !relevance.is_empty())
    }
    
    fn can_be_repeated(&self) -> bool {
        self.repeatable.unwrap_or(false)
    }
}

// Utility functions for quest management
pub fn calculate_quest_priority(quest: &EnhancedQuestHook) -> i32 {
    let urgency = quest.urgency_level.unwrap_or(3);
    let backstory_bonus = if quest.has_backstory_connections() { 2 } else { 0 };
    let time_pressure = match quest.time_limit_days {
        Some(days) if days <= 7 => 3,
        Some(days) if days <= 30 => 1,
        _ => 0,
    };
    
    urgency + backstory_bonus + time_pressure
}

pub fn suggest_quest_chains(quests: &[EnhancedQuestHook]) -> Vec<Vec<i32>> {
    // Logic to identify potential quest chains based on:
    // - Location connections
    // - NPC relationships  
    // - Backstory connections
    // - Logical narrative flow
    
    vec![] // Simplified for now
}