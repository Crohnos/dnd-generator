use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

// Calendar Systems
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CalendarSystem {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub months: JsonValue,
    pub weekdays: JsonValue,
    pub year_length: Option<i32>,
    pub current_year: Option<i32>,
    pub current_month: Option<i32>,
    pub current_day: Option<i32>,
    pub special_events: Option<JsonValue>,
    pub lunar_cycles: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Historical Events
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HistoricalEvent {
    pub id: i32,
    pub campaign_id: i32,
    pub calendar_system_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub duration_days: Option<i32>,
    pub importance_level: Option<i32>,
    pub participants: Option<JsonValue>,
    pub consequences: Option<String>,
    pub public_knowledge: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Location Types
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationType {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub default_properties: Option<JsonValue>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Enhanced Locations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EnhancedLocation {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub connections: Option<Vec<i32>>,
    pub properties: Option<JsonValue>,
    
    // New enhanced fields
    pub location_type_id: Option<i32>,
    pub parent_location_id: Option<i32>,
    pub population: Option<i32>,
    pub government_type: Option<String>,
    pub economy: Option<JsonValue>,
    pub climate: Option<String>,
    pub terrain: Option<String>,
    pub danger_level: Option<i32>,
    pub notable_features: Option<Vec<String>>,
    pub resources: Option<Vec<String>>,
    pub travel_time_modifiers: Option<JsonValue>,
    pub is_secret: Option<bool>,
    pub discovery_requirements: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Location Connections
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationConnection {
    pub id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub connection_type: Option<String>,
    pub distance_miles: Option<rust_decimal::Decimal>,
    pub travel_time_hours: Option<rust_decimal::Decimal>,
    pub difficulty: Option<String>,
    pub dangers: Option<Vec<String>>,
    pub requirements: Option<String>,
    pub toll_cost: Option<i32>,
    pub is_secret: Option<bool>,
    pub seasonal_availability: Option<Vec<String>>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Location Services
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationService {
    pub id: i32,
    pub location_id: i32,
    pub service_type: String,
    pub name: String,
    pub description: Option<String>,
    pub quality_level: Option<i32>,
    pub services_offered: Option<Vec<String>>,
    pub prices: Option<JsonValue>,
    pub availability_schedule: Option<String>,
    pub owner_npc_id: Option<i32>,
    pub reputation: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Deities
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Deity {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub titles: Option<Vec<String>>,
    pub alignment: Option<String>,
    pub domains: Option<Vec<String>>,
    pub portfolio: Option<Vec<String>>,
    pub holy_symbol: Option<String>,
    pub favored_weapon: Option<String>,
    pub divine_rank: Option<String>,
    pub description: Option<String>,
    pub appearance: Option<String>,
    pub personality_traits: Option<Vec<String>>,
    pub relationships_with_other_deities: Option<JsonValue>,
    pub worshiper_alignments: Option<Vec<String>>,
    pub clergy_alignments: Option<Vec<String>>,
    pub holy_days: Option<Vec<String>>,
    pub creation_myths: Option<String>,
    pub major_temples: Option<Vec<i32>>,
    pub is_active: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Trade Goods
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradeGood {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub base_value: Option<rust_decimal::Decimal>,
    pub weight_pounds: Option<rust_decimal::Decimal>,
    pub bulk_rating: Option<i32>,
    pub perishable: Option<bool>,
    pub shelf_life_days: Option<i32>,
    pub production_locations: Option<Vec<i32>>,
    pub demand_locations: Option<Vec<i32>>,
    pub rarity: Option<String>,
    pub seasonal_availability: Option<Vec<String>>,
    pub transportation_requirements: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

// Trade Routes
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradeRoute {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub start_location_id: i32,
    pub end_location_id: i32,
    pub intermediate_stops: Option<Vec<i32>>,
    pub total_distance_miles: Option<rust_decimal::Decimal>,
    pub travel_time_days: Option<rust_decimal::Decimal>,
    pub danger_level: Option<i32>,
    pub primary_goods: Option<Vec<i32>>,
    pub controlling_organization_id: Option<i32>,
    pub toll_costs: Option<JsonValue>,
    pub seasonal_availability: Option<Vec<String>>,
    pub current_status: Option<String>,
    pub disruption_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Currencies
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Currency {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub abbreviation: Option<String>,
    pub base_unit: Option<String>,
    pub exchange_rate: Option<rust_decimal::Decimal>,
    pub regions_used: Option<Vec<i32>>,
    pub issuing_authority: Option<String>,
    pub physical_description: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub campaign_id: i32,
    pub name: String,
    pub location_type_id: Option<i32>,
    pub parent_location_id: Option<i32>,
    pub description: Option<String>,
    pub population: Option<i32>,
    pub government_type: Option<String>,
    pub climate: Option<String>,
    pub terrain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHistoricalEventRequest {
    pub campaign_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub year: Option<i32>,
    pub importance_level: Option<i32>,
    pub consequences: Option<String>,
    pub public_knowledge: Option<bool>,
}

// Traits for world operations
pub trait LocationOperations {
    fn get_full_hierarchy(&self) -> String;
    fn get_population_category(&self) -> String;
    fn is_settlement(&self) -> bool;
    fn get_danger_description(&self) -> String;
}

impl LocationOperations for EnhancedLocation {
    fn get_full_hierarchy(&self) -> String {
        // This would build the full location hierarchy path
        self.name.clone() // Simplified for now
    }
    
    fn get_population_category(&self) -> String {
        match self.population.unwrap_or(0) {
            p if p >= 100000 => "Metropolis".to_string(),
            p if p >= 25000 => "Large City".to_string(),
            p if p >= 5000 => "City".to_string(),
            p if p >= 1000 => "Town".to_string(),
            p if p >= 100 => "Village".to_string(),
            p if p >= 20 => "Hamlet".to_string(),
            p if p > 0 => "Outpost".to_string(),
            _ => "Uninhabited".to_string(),
        }
    }
    
    fn is_settlement(&self) -> bool {
        matches!(self.r#type.as_deref(), 
            Some("city") | Some("town") | Some("village") | Some("hamlet"))
    }
    
    fn get_danger_description(&self) -> String {
        match self.danger_level.unwrap_or(1) {
            10 => "Apocalyptic".to_string(),
            8..=9 => "Deadly".to_string(),
            6..=7 => "Dangerous".to_string(),
            4..=5 => "Moderate Risk".to_string(),
            2..=3 => "Minor Risk".to_string(),
            _ => "Safe".to_string(),
        }
    }
}

pub trait CalendarOperations {
    fn get_current_date_string(&self) -> String;
    fn advance_time(&mut self, days: i32);
    fn get_season(&self) -> Option<String>;
}

impl CalendarOperations for CalendarSystem {
    fn get_current_date_string(&self) -> String {
        format!("Year {}, Month {}, Day {}", 
                self.current_year.unwrap_or(1),
                self.current_month.unwrap_or(1),
                self.current_day.unwrap_or(1))
    }
    
    fn advance_time(&mut self, days: i32) {
        // Simplified time advancement - would need proper calendar logic
        if let Some(current_day) = self.current_day {
            self.current_day = Some(current_day + days);
        }
    }
    
    fn get_season(&self) -> Option<String> {
        // This would determine season based on current month
        // Using the months JSON structure
        None // Simplified for now
    }
}