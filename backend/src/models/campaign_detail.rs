use crate::models::{Campaign, Npc, Location, QuestHook, Encounter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignDetail {
    #[serde(flatten)]
    pub campaign: Campaign,
    pub npcs: Vec<Npc>,
    pub locations: Vec<Location>,
    pub quest_hooks: Vec<QuestHook>,
    pub encounters: Vec<Encounter>,
}