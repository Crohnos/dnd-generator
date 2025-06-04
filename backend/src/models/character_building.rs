use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Race {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub size: String,
    pub speed: i32,
    pub languages: Vec<String>,
    pub racial_traits: JsonValue,
    pub ability_score_increases: JsonValue,
    pub lifespan: Option<String>,
    pub alignment_tendencies: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subrace {
    pub id: i32,
    pub campaign_id: i32,
    pub race_id: i32,
    pub name: String,
    pub description: String,
    pub additional_traits: JsonValue,
    pub additional_ability_scores: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Class {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub hit_die: String,
    pub primary_ability: Vec<String>,
    pub saving_throw_proficiencies: Vec<String>,
    pub skill_proficiencies: JsonValue,
    pub equipment_proficiencies: JsonValue,
    pub class_features: JsonValue,
    pub spellcasting_ability: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subclass {
    pub id: i32,
    pub campaign_id: i32,
    pub class_id: i32,
    pub name: String,
    pub description: String,
    pub additional_features: JsonValue,
    pub additional_spells: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Feat {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub prerequisites: JsonValue,
    pub benefits: JsonValue,
    pub feat_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Background {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub skill_proficiencies: Vec<String>,
    pub language_proficiencies: Vec<String>,
    pub tool_proficiencies: Vec<String>,
    pub equipment: JsonValue,
    pub feature: JsonValue,
    pub suggested_characteristics: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Language {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub language_type: String,
    pub script: Option<String>,
    pub speakers: JsonValue,
    pub prevalence: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Condition {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub effects: JsonValue,
    pub duration_type: String,
    pub removal_conditions: JsonValue,
    pub severity: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Curse {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub curse_type: String,
    pub effects: JsonValue,
    pub triggers: JsonValue,
    pub removal_methods: JsonValue,
    pub severity: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}