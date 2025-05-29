use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

// Organization Types
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub typical_structure: Option<String>,
    pub common_goals: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

// Organizations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i32,
    pub campaign_id: i32,
    pub organization_type_id: Option<i32>,
    pub name: String,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub founding_date: Option<NaiveDate>,
    pub headquarters_location_id: Option<i32>,
    pub territory_locations: Option<Vec<i32>>,
    pub size_category: Option<String>,
    pub influence_level: Option<i32>,
    pub wealth_level: Option<i32>,
    pub secrecy_level: Option<i32>,
    pub alignment: Option<String>,
    pub primary_goals: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
    pub resources: Option<Vec<String>>,
    pub symbols_and_colors: Option<JsonValue>,
    pub motto: Option<String>,
    pub reputation: Option<i32>,
    pub is_active: Option<bool>,
    pub dissolution_date: Option<NaiveDate>,
    pub dissolution_reason: Option<String>,
    pub parent_organization_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Organization Ranks
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationRank {
    pub id: i32,
    pub organization_id: i32,
    pub rank_name: String,
    pub rank_level: i32,
    pub description: Option<String>,
    pub responsibilities: Option<Vec<String>>,
    pub privileges: Option<Vec<String>>,
    pub requirements: Option<Vec<String>>,
    pub typical_salary: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// Organization Memberships
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationMembership {
    pub id: i32,
    pub organization_id: i32,
    pub npc_id: i32,
    pub rank_id: Option<i32>,
    pub join_date: NaiveDate,
    pub leave_date: Option<NaiveDate>,
    pub membership_status: Option<String>,
    pub loyalty_level: Option<i32>,
    pub contribution_level: Option<i32>,
    pub special_roles: Option<Vec<String>>,
    pub access_level: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Organization Relationships
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationRelationship {
    pub id: i32,
    pub organization1_id: i32,
    pub organization2_id: i32,
    pub relationship_type: String,
    pub relationship_strength: Option<i32>,
    pub description: Option<String>,
    pub formal_agreement: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_secret: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Religious Organizations (extends Organization)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReligiousOrganization {
    pub id: i32,
    pub organization_id: i32,
    pub deity_id: Option<i32>,
    pub religious_focus: Option<Vec<String>>,
    pub doctrine: Option<String>,
    pub religious_practices: Option<Vec<String>>,
    pub hierarchy_structure: Option<String>,
    pub initiation_requirements: Option<Vec<String>>,
    pub core_beliefs: Option<Vec<String>>,
    pub forbidden_acts: Option<Vec<String>>,
    pub holy_texts: Option<Vec<String>>,
    pub sacred_locations: Option<Vec<i32>>,
    pub pilgrimage_sites: Option<Vec<i32>>,
    pub religious_festivals: Option<JsonValue>,
    pub charitable_works: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {
    pub campaign_id: i32,
    pub organization_type_id: Option<i32>,
    pub name: String,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub headquarters_location_id: Option<i32>,
    pub size_category: Option<String>,
    pub influence_level: Option<i32>,
    pub primary_goals: Option<Vec<String>>,
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMembershipRequest {
    pub organization_id: i32,
    pub npc_id: i32,
    pub rank_id: Option<i32>,
    pub join_date: NaiveDate,
    pub membership_status: Option<String>,
    pub loyalty_level: Option<i32>,
    pub special_roles: Option<Vec<String>>,
}

// Traits for organization operations
pub trait OrganizationOperations {
    fn get_display_name(&self) -> &str;
    fn is_active(&self) -> bool;
    fn get_influence_level_name(&self) -> String;
    fn get_size_description(&self) -> String;
}

impl OrganizationOperations for Organization {
    fn get_display_name(&self) -> &str {
        self.short_name.as_deref().unwrap_or(&self.name)
    }
    
    fn is_active(&self) -> bool {
        self.is_active.unwrap_or(true) && self.dissolution_date.is_none()
    }
    
    fn get_influence_level_name(&self) -> String {
        match self.influence_level.unwrap_or(1) {
            10 => "World Power".to_string(),
            8..=9 => "National Power".to_string(),
            6..=7 => "Regional Power".to_string(),
            4..=5 => "Local Power".to_string(),
            2..=3 => "Minor Influence".to_string(),
            _ => "Negligible".to_string(),
        }
    }
    
    fn get_size_description(&self) -> String {
        match self.size_category.as_deref() {
            Some("massive") => "Massive (10,000+ members)".to_string(),
            Some("large") => "Large (1,000-10,000 members)".to_string(),
            Some("medium") => "Medium (100-1,000 members)".to_string(),
            Some("small") => "Small (10-100 members)".to_string(),
            Some("tiny") => "Tiny (2-10 members)".to_string(),
            _ => "Unknown Size".to_string(),
        }
    }
}

pub trait MembershipOperations {
    fn is_active_member(&self) -> bool;
    fn get_tenure_description(&self) -> String;
    fn get_loyalty_description(&self) -> String;
}

impl MembershipOperations for OrganizationMembership {
    fn is_active_member(&self) -> bool {
        self.membership_status.as_deref() == Some("active") && self.leave_date.is_none()
    }
    
    fn get_tenure_description(&self) -> String {
        // This would calculate duration from join_date to now or leave_date
        "Active".to_string() // Simplified for now
    }
    
    fn get_loyalty_description(&self) -> String {
        match self.loyalty_level.unwrap_or(5) {
            9..=10 => "Fanatically Loyal".to_string(),
            7..=8 => "Very Loyal".to_string(),
            5..=6 => "Loyal".to_string(),
            3..=4 => "Somewhat Loyal".to_string(),
            1..=2 => "Unreliable".to_string(),
            _ => "Disloyal".to_string(),
        }
    }
}