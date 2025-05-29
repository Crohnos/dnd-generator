use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Magic Items
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MagicItem {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub item_type: String,
    pub rarity: String,
    pub attunement_required: Option<bool>,
    pub description: Option<String>,
    pub mechanical_effects: Option<String>,
    pub activation_method: Option<String>,
    pub charges: Option<i32>,
    pub charge_recovery: Option<String>,
    pub curse_description: Option<String>,
    pub creator_name: Option<String>,
    pub creation_date: Option<NaiveDate>,
    pub historical_significance: Option<String>,
    pub current_location_id: Option<i32>,
    pub current_owner_npc_id: Option<i32>,
    pub market_value: Option<rust_decimal::Decimal>,
    pub weight_pounds: Option<rust_decimal::Decimal>,
    pub physical_description: Option<String>,
    pub is_sentient: Option<bool>,
    pub intelligence: Option<i32>,
    pub personality: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Magical Phenomena
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MagicalPhenomenon {
    pub id: i32,
    pub campaign_id: i32,
    pub location_id: Option<i32>,
    pub name: String,
    pub phenomenon_type: String,
    pub description: String,
    pub magical_effects: Option<Vec<String>>,
    pub trigger_conditions: Option<Vec<String>>,
    pub duration: Option<String>,
    pub danger_level: Option<i32>,
    pub study_difficulty: Option<String>,
    pub known_by_npcs: Option<Vec<i32>>,
    pub research_value: Option<String>,
    pub containment_possible: Option<bool>,
    pub containment_methods: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Knowledge Sources (Books, Scrolls, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KnowledgeSource {
    pub id: i32,
    pub campaign_id: i32,
    pub title: String,
    pub source_type: Option<String>,
    pub author_npc_id: Option<i32>,
    pub subject_areas: Option<Vec<String>>,
    pub content_summary: Option<String>,
    pub accuracy_level: Option<i32>,
    pub completeness_level: Option<i32>,
    pub age_years: Option<i32>,
    pub language: Option<String>,
    pub physical_condition: Option<String>,
    pub current_location_id: Option<i32>,
    pub access_restrictions: Option<String>,
    pub copying_difficulty: Option<String>,
    pub market_value: Option<rust_decimal::Decimal>,
    pub rarity: Option<String>,
    pub related_topics: Option<Vec<String>>,
    pub contradicts_sources: Option<Vec<i32>>,
    pub supports_sources: Option<Vec<i32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Rumors and Information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Rumor {
    pub id: i32,
    pub campaign_id: i32,
    pub content: String,
    pub rumor_type: Option<String>,
    pub accuracy_level: Option<i32>,
    pub spread_rate: Option<i32>,
    pub origin_location_id: Option<i32>,
    pub origin_npc_id: Option<i32>,
    pub current_locations: Option<Vec<i32>>,
    pub target_audience: Option<Vec<String>>,
    pub verification_difficulty: Option<String>,
    pub consequences_if_true: Option<String>,
    pub consequences_if_false: Option<String>,
    pub expiration_date: Option<NaiveDate>,
    pub related_events: Option<Vec<i32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Campaign Session Tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CampaignSession {
    pub id: i32,
    pub campaign_id: i32,
    pub session_number: i32,
    pub session_date: NaiveDate,
    pub in_game_date_start: Option<String>,
    pub in_game_date_end: Option<String>,
    pub time_advanced: Option<String>,
    pub locations_visited: Option<Vec<i32>>,
    pub npcs_encountered: Option<Vec<i32>>,
    pub quests_progressed: Option<Vec<i32>>,
    pub encounters_completed: Option<Vec<i32>>,
    pub major_events: Option<Vec<String>>,
    pub player_decisions: Option<serde_json::Value>,
    pub experience_gained: Option<i32>,
    pub treasure_found: Option<Vec<String>>,
    pub session_summary: Option<String>,
    pub dm_notes: Option<String>,
    pub player_feedback: Option<String>,
    pub next_session_prep: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Campaign Flags for state tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CampaignFlag {
    pub id: i32,
    pub campaign_id: i32,
    pub flag_name: String,
    pub flag_value: Option<String>,
    pub flag_type: Option<String>,
    pub description: Option<String>,
    pub set_by_session: Option<i32>,
    pub affects_future_events: Option<bool>,
    pub visible_to_players: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMagicItemRequest {
    pub campaign_id: i32,
    pub name: String,
    pub item_type: String,
    pub rarity: String,
    pub description: Option<String>,
    pub mechanical_effects: Option<String>,
    pub current_location_id: Option<i32>,
    pub current_owner_npc_id: Option<i32>,
    pub is_sentient: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKnowledgeSourceRequest {
    pub campaign_id: i32,
    pub title: String,
    pub source_type: Option<String>,
    pub author_npc_id: Option<i32>,
    pub subject_areas: Option<Vec<String>>,
    pub content_summary: Option<String>,
    pub current_location_id: Option<i32>,
    pub access_restrictions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRumorRequest {
    pub campaign_id: i32,
    pub content: String,
    pub rumor_type: Option<String>,
    pub accuracy_level: Option<i32>,
    pub origin_location_id: Option<i32>,
    pub origin_npc_id: Option<i32>,
    pub target_audience: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub campaign_id: i32,
    pub session_number: i32,
    pub session_date: NaiveDate,
    pub session_summary: Option<String>,
    pub locations_visited: Option<Vec<i32>>,
    pub npcs_encountered: Option<Vec<i32>>,
    pub major_events: Option<Vec<String>>,
}

// Traits for magic and knowledge operations
pub trait MagicItemOperations {
    fn get_rarity_description(&self) -> String;
    fn is_cursed(&self) -> bool;
    fn requires_attunement(&self) -> bool;
    fn get_power_level(&self) -> i32;
    fn is_available(&self) -> bool;
}

impl MagicItemOperations for MagicItem {
    fn get_rarity_description(&self) -> String {
        match self.rarity.as_str() {
            "common" => "Common - Minor magical properties".to_string(),
            "uncommon" => "Uncommon - Moderate magical abilities".to_string(),
            "rare" => "Rare - Significant magical power".to_string(),
            "very_rare" => "Very Rare - Considerable magical might".to_string(),
            "legendary" => "Legendary - Extraordinary magical power".to_string(),
            "artifact" => "Artifact - Ultimate magical item".to_string(),
            _ => "Unknown Rarity".to_string(),
        }
    }
    
    fn is_cursed(&self) -> bool {
        self.curse_description.is_some()
    }
    
    fn requires_attunement(&self) -> bool {
        self.attunement_required.unwrap_or(false)
    }
    
    fn get_power_level(&self) -> i32 {
        match self.rarity.as_str() {
            "common" => 1,
            "uncommon" => 2,
            "rare" => 3,
            "very_rare" => 4,
            "legendary" => 5,
            "artifact" => 6,
            _ => 0,
        }
    }
    
    fn is_available(&self) -> bool {
        self.current_owner_npc_id.is_none() || self.current_location_id.is_some()
    }
}

pub trait KnowledgeOperations {
    fn get_reliability_score(&self) -> i32;
    fn is_accessible(&self) -> bool;
    fn get_research_difficulty(&self) -> String;
    fn conflicts_with_other_sources(&self) -> bool;
}

impl KnowledgeOperations for KnowledgeSource {
    fn get_reliability_score(&self) -> i32 {
        let accuracy = self.accuracy_level.unwrap_or(5);
        let completeness = self.completeness_level.unwrap_or(5);
        (accuracy + completeness) / 2
    }
    
    fn is_accessible(&self) -> bool {
        self.access_restrictions.is_none() || 
        self.access_restrictions.as_deref() == Some("public")
    }
    
    fn get_research_difficulty(&self) -> String {
        match self.copying_difficulty.as_deref() {
            Some("trivial") => "Trivial - Can be quickly reviewed".to_string(),
            Some("easy") => "Easy - Straightforward to study".to_string(),
            Some("moderate") => "Moderate - Requires focused study".to_string(),
            Some("hard") => "Hard - Requires expertise".to_string(),
            Some("extreme") => "Extreme - Requires specialized knowledge".to_string(),
            _ => "Unknown Difficulty".to_string(),
        }
    }
    
    fn conflicts_with_other_sources(&self) -> bool {
        self.contradicts_sources.as_ref().map_or(false, |sources| !sources.is_empty())
    }
}

pub trait RumorOperations {
    fn is_reliable(&self) -> bool;
    fn is_spreading(&self) -> bool;
    fn get_verification_description(&self) -> String;
    fn is_expired(&self) -> bool;
}

impl RumorOperations for Rumor {
    fn is_reliable(&self) -> bool {
        self.accuracy_level.unwrap_or(5) >= 7
    }
    
    fn is_spreading(&self) -> bool {
        self.spread_rate.unwrap_or(1) >= 3
    }
    
    fn get_verification_description(&self) -> String {
        match self.verification_difficulty.as_deref() {
            Some("trivial") => "Trivial - Easily verified".to_string(),
            Some("easy") => "Easy - Simple investigation".to_string(),
            Some("moderate") => "Moderate - Requires investigation".to_string(),
            Some("hard") => "Hard - Difficult to verify".to_string(),
            Some("impossible") => "Impossible - Cannot be verified".to_string(),
            _ => "Unknown Verification Difficulty".to_string(),
        }
    }
    
    fn is_expired(&self) -> bool {
        if let Some(expiration) = self.expiration_date {
            let today = chrono::Utc::now().date_naive();
            expiration < today
        } else {
            false
        }
    }
}

// Utility functions for campaign management
pub fn calculate_session_impact(session: &CampaignSession) -> i32 {
    let locations_count = session.locations_visited.as_ref().map_or(0, |l| l.len() as i32);
    let npcs_count = session.npcs_encountered.as_ref().map_or(0, |n| n.len() as i32);
    let events_count = session.major_events.as_ref().map_or(0, |e| e.len() as i32);
    
    locations_count + npcs_count + (events_count * 2)
}

pub fn suggest_related_knowledge(source: &KnowledgeSource, all_sources: &[KnowledgeSource]) -> Vec<i32> {
    let mut related = Vec::new();
    
    if let Some(topics) = &source.related_topics {
        for other_source in all_sources {
            if other_source.id != source.id {
                if let Some(other_topics) = &other_source.subject_areas {
                    for topic in topics {
                        if other_topics.contains(topic) {
                            related.push(other_source.id);
                            break;
                        }
                    }
                }
            }
        }
    }
    
    related
}