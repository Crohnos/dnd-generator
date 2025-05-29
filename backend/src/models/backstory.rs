use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Backstory Element Types
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BackstoryElementType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub examples: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

// Backstory Elements
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BackstoryElement {
    pub id: i32,
    pub campaign_id: i32,
    pub element_type_id: i32,
    pub player_character_name: String,
    pub element_name: String,
    pub description: String,
    pub importance_level: Option<i32>,
    pub current_status: Option<String>,
    pub connected_npc_id: Option<i32>,
    pub connected_location_id: Option<i32>,
    pub connected_organization_id: Option<i32>,
    pub connected_quest_hook_id: Option<i32>,
    pub integration_notes: Option<String>,
    pub player_notes: Option<String>,
    pub dm_notes: Option<String>,
    pub is_secret: Option<bool>,
    pub reveal_trigger: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Extended backstory information for comprehensive PC integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCharacterProfile {
    pub name: String,
    pub class: Option<String>,
    pub race: Option<String>,
    pub background: Option<String>,
    pub backstory_elements: Vec<BackstoryElement>,
    pub goals: Vec<String>,
    pub fears: Vec<String>,
    pub relationships: Vec<String>,
    pub secrets: Vec<String>,
    pub motivations: Vec<String>,
}

// Backstory integration with campaign elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackstoryIntegration {
    pub backstory_element_id: i32,
    pub integration_type: String, // quest, npc, location, organization, item
    pub target_id: i32,
    pub integration_level: i32, // 1-5, how deeply integrated
    pub story_hooks: Vec<String>,
    pub potential_conflicts: Vec<String>,
    pub narrative_opportunities: Vec<String>,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBackstoryElementRequest {
    pub campaign_id: i32,
    pub element_type_id: i32,
    pub player_character_name: String,
    pub element_name: String,
    pub description: String,
    pub importance_level: Option<i32>,
    pub connected_npc_id: Option<i32>,
    pub connected_location_id: Option<i32>,
    pub connected_organization_id: Option<i32>,
    pub player_notes: Option<String>,
    pub is_secret: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBackstoryElementRequest {
    pub current_status: Option<String>,
    pub connected_quest_hook_id: Option<i32>,
    pub integration_notes: Option<String>,
    pub dm_notes: Option<String>,
    pub reveal_trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackstoryAnalysisRequest {
    pub player_character_name: String,
    pub backstory_text: String,
    pub character_goals: Option<Vec<String>>,
    pub character_fears: Option<Vec<String>>,
    pub important_relationships: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackstoryAnalysisResult {
    pub extracted_elements: Vec<ExtractedBackstoryElement>,
    pub suggested_integrations: Vec<SuggestedIntegration>,
    pub story_potential_score: i32,
    pub complexity_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedBackstoryElement {
    pub element_type: String,
    pub name: String,
    pub description: String,
    pub importance_level: i32,
    pub suggested_connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedIntegration {
    pub integration_type: String,
    pub target_name: String,
    pub connection_strength: i32,
    pub story_hooks: Vec<String>,
    pub implementation_notes: String,
}

// Traits for backstory operations
pub trait BackstoryOperations {
    fn get_importance_description(&self) -> String;
    fn is_resolved(&self) -> bool;
    fn has_active_connections(&self) -> bool;
    fn get_integration_opportunities(&self) -> Vec<String>;
}

impl BackstoryOperations for BackstoryElement {
    fn get_importance_description(&self) -> String {
        match self.importance_level.unwrap_or(5) {
            9..=10 => "Critical to Character".to_string(),
            7..=8 => "Very Important".to_string(),
            5..=6 => "Moderately Important".to_string(),
            3..=4 => "Minor Detail".to_string(),
            1..=2 => "Background Detail".to_string(),
            _ => "Unknown Importance".to_string(),
        }
    }
    
    fn is_resolved(&self) -> bool {
        self.current_status.as_deref() == Some("resolved")
    }
    
    fn has_active_connections(&self) -> bool {
        self.connected_npc_id.is_some() || 
        self.connected_location_id.is_some() || 
        self.connected_organization_id.is_some() || 
        self.connected_quest_hook_id.is_some()
    }
    
    fn get_integration_opportunities(&self) -> Vec<String> {
        let mut opportunities = Vec::new();
        
        if self.connected_npc_id.is_none() {
            opportunities.push("Could be connected to an NPC".to_string());
        }
        if self.connected_location_id.is_none() {
            opportunities.push("Could be tied to a location".to_string());
        }
        if self.connected_organization_id.is_none() {
            opportunities.push("Could involve an organization".to_string());
        }
        if self.connected_quest_hook_id.is_none() {
            opportunities.push("Could generate a quest".to_string());
        }
        
        if self.is_secret.unwrap_or(false) && self.reveal_trigger.is_none() {
            opportunities.push("Needs a reveal trigger".to_string());
        }
        
        opportunities
    }
}

// Utility functions for backstory analysis
pub fn calculate_backstory_complexity(elements: &[BackstoryElement]) -> String {
    let total_importance: i32 = elements.iter()
        .map(|e| e.importance_level.unwrap_or(5))
        .sum();
    
    let connection_count = elements.iter()
        .filter(|e| e.has_active_connections())
        .count();
    
    let complexity_score = total_importance + (connection_count as i32 * 5);
    
    match complexity_score {
        score if score >= 80 => "Highly Complex".to_string(),
        score if score >= 50 => "Moderately Complex".to_string(),
        score if score >= 25 => "Simple".to_string(),
        _ => "Minimal".to_string(),
    }
}

pub fn prioritize_integration_elements(elements: &[BackstoryElement]) -> Vec<&BackstoryElement> {
    let mut sorted_elements: Vec<&BackstoryElement> = elements.iter().collect();
    
    sorted_elements.sort_by(|a, b| {
        let a_score = a.importance_level.unwrap_or(5) * 
                     if a.has_active_connections() { 1 } else { 2 };
        let b_score = b.importance_level.unwrap_or(5) * 
                     if b.has_active_connections() { 1 } else { 2 };
        
        b_score.cmp(&a_score)
    });
    
    sorted_elements
}