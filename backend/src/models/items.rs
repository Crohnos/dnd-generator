use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ItemType {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub typical_properties: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub campaign_id: i32,
    pub name: String,
    pub item_type_id: i32,
    pub description: String,
    pub rarity: String,
    pub value: i32,
    pub weight: f32,
    pub magical: bool,
    pub attunement_required: bool,
    pub cursed: bool,
    pub properties: JsonValue,
    pub lore: Option<String>,
    pub creator: Option<String>,
    pub age: Option<String>,
    pub condition_state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ItemAbility {
    pub id: i32,
    pub campaign_id: i32,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub ability_type: String,
    pub usage_type: String,
    pub charges: Option<i32>,
    pub recharge_condition: Option<String>,
    pub activation_cost: Option<String>,
    pub range_area: Option<String>,
    pub duration: Option<String>,
    pub saving_throw: JsonValue,
    pub damage: JsonValue,
    pub effects: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Weapon {
    pub id: i32,
    pub campaign_id: i32,
    pub item_id: i32,
    pub weapon_type: String,
    pub damage_dice: String,
    pub damage_type: String,
    pub weapon_properties: Vec<String>,
    pub range_normal: Option<i32>,
    pub range_long: Option<i32>,
    pub finesse: bool,
    pub light: bool,
    pub heavy: bool,
    pub reach: bool,
    pub thrown: bool,
    pub two_handed: bool,
    pub versatile: bool,
    pub ammunition: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Armor {
    pub id: i32,
    pub campaign_id: i32,
    pub item_id: i32,
    pub armor_type: String,
    pub armor_class: i32,
    pub dex_modifier_cap: Option<i32>,
    pub strength_requirement: Option<i32>,
    pub stealth_disadvantage: bool,
    pub don_time: String,
    pub doff_time: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Potion {
    pub id: i32,
    pub campaign_id: i32,
    pub item_id: i32,
    pub potion_type: String,
    pub effect: JsonValue,
    pub duration: Option<String>,
    pub onset_time: Option<String>,
    pub side_effects: JsonValue,
    pub brewing_difficulty: String,
    pub ingredients: JsonValue,
    pub brewing_time: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SpellScroll {
    pub id: i32,
    pub campaign_id: i32,
    pub item_id: i32,
    pub spell_name: String,
    pub spell_level: i32,
    pub school_of_magic: String,
    pub casting_time: String,
    pub range_area: String,
    pub duration: String,
    pub components: JsonValue,
    pub spell_description: String,
    pub save_dc: Option<i32>,
    pub attack_bonus: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SentientItem {
    pub id: i32,
    pub campaign_id: i32,
    pub item_id: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub alignment: String,
    pub personality_traits: JsonValue,
    pub ideals: JsonValue,
    pub bonds: JsonValue,
    pub flaws: JsonValue,
    pub languages: Vec<String>,
    pub senses: JsonValue,
    pub communication_method: String,
    pub ego_score: i32,
    pub purpose: String,
    pub special_purpose: Option<String>,
    pub conflict_resolution: JsonValue,
    pub backstory: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}