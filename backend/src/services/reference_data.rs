use crate::error::ApiResult;
use crate::services::GraphQLClient;
use serde_json::json;
use std::sync::Arc;
use tracing::info;

pub struct ReferenceDataService {
    graphql_client: Arc<GraphQLClient>,
}

impl ReferenceDataService {
    pub fn new(graphql_client: Arc<GraphQLClient>) -> Self {
        Self { graphql_client }
    }

    pub async fn seed_standard_dnd_content(&self, campaign_id: i32) -> ApiResult<()> {
        info!("Seeding standard D&D 5e reference data for campaign {}", campaign_id);
        
        // Seed backgrounds first as they're referenced by entities
        self.seed_backgrounds(campaign_id).await?;
        
        // Note: Classes and races are already populated by the migration
        // We could add more reference data here if needed
        
        info!("Successfully seeded reference data for campaign {}", campaign_id);
        Ok(())
    }

    async fn seed_backgrounds(&self, campaign_id: i32) -> ApiResult<()> {
        let backgrounds = vec![
            json!({
                "campaign_id": campaign_id,
                "name": "Acolyte",
                "description": "You have spent your life in the service of a temple to a specific god or pantheon of gods.",
                "skill_proficiencies": ["Insight", "Religion"],
                "equipment": ["Holy symbol", "Prayer book", "5 sticks of incense", "Vestments", "Common clothes", "Belt pouch with 15 gp"],
                "feature_name": "Shelter of the Faithful",
                "feature_description": "As an acolyte, you command the respect of those who share your faith. You and your companions can receive free healing and care at temples and shrines of your faith."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Criminal",
                "description": "You are an experienced criminal with a history of breaking the law.",
                "skill_proficiencies": ["Deception", "Stealth"],
                "tool_proficiencies": ["One type of gaming set", "Thieves' tools"],
                "equipment": ["Crowbar", "Dark common clothes with hood", "Belt pouch with 15 gp"],
                "feature_name": "Criminal Contact",
                "feature_description": "You have a reliable and trustworthy contact who acts as your liaison to a network of other criminals."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Folk Hero",
                "description": "You come from a humble social rank, but you are destined for so much more.",
                "skill_proficiencies": ["Animal Handling", "Survival"],
                "tool_proficiencies": ["One type of artisan's tools", "Vehicles (land)"],
                "equipment": ["Artisan's tools", "Shovel", "Iron pot", "Common clothes", "Belt pouch with 10 gp"],
                "feature_name": "Rustic Hospitality",
                "feature_description": "Since you come from the ranks of the common folk, you fit in among them with ease. You can find a place to hide, rest, or recuperate among commoners."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Noble",
                "description": "You understand wealth, power, and privilege.",
                "skill_proficiencies": ["History", "Persuasion"],
                "tool_proficiencies": ["One type of gaming set"],
                "equipment": ["Fine clothes", "Signet ring", "Scroll of pedigree", "Purse with 25 gp"],
                "feature_name": "Position of Privilege",
                "feature_description": "Thanks to your noble birth, people are inclined to think the best of you. You are welcome in high society."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Sage",
                "description": "You spent years learning the lore of the multiverse.",
                "skill_proficiencies": ["Arcana", "History"],
                "equipment": ["Bottle of black ink", "Quill", "Small knife", "Letter from dead colleague", "Common clothes", "Belt pouch with 10 gp"],
                "feature_name": "Researcher",
                "feature_description": "When you attempt to learn or recall a piece of lore, if you do not know that information, you often know where and from whom you can obtain it."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Soldier",
                "description": "War has been your life for as long as you care to remember.",
                "skill_proficiencies": ["Athletics", "Intimidation"],
                "tool_proficiencies": ["One type of gaming set", "Vehicles (land)"],
                "equipment": ["Insignia of rank", "Trophy from fallen enemy", "Deck of cards", "Common clothes", "Belt pouch with 10 gp"],
                "feature_name": "Military Rank",
                "feature_description": "You have a military rank from your career as a soldier. Soldiers loyal to your former military organization still recognize your authority."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Hermit",
                "description": "You lived in seclusion for a formative part of your life.",
                "skill_proficiencies": ["Medicine", "Religion"],
                "tool_proficiencies": ["Herbalism kit"],
                "equipment": ["Scroll case with notes", "Winter blanket", "Common clothes", "Herbalism kit", "5 gp"],
                "feature_name": "Discovery",
                "feature_description": "The quiet seclusion of your extended hermitage gave you access to a unique and powerful discovery."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Guild Artisan",
                "description": "You are a member of an artisan guild.",
                "skill_proficiencies": ["Insight", "Persuasion"],
                "tool_proficiencies": ["One type of artisan's tools"],
                "equipment": ["Artisan's tools", "Letter of introduction from guild", "Traveler's clothes", "Belt pouch with 15 gp"],
                "feature_name": "Guild Membership",
                "feature_description": "As a member of your guild, you can rely on certain benefits that membership provides. Your guild offers lodging and food if necessary."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Outlander",
                "description": "You grew up in the wilds, far from civilization.",
                "skill_proficiencies": ["Athletics", "Survival"],
                "tool_proficiencies": ["One type of musical instrument"],
                "equipment": ["Staff", "Hunting trap", "Trophy from animal", "Traveler's clothes", "Belt pouch with 10 gp"],
                "feature_name": "Wanderer",
                "feature_description": "You have an excellent memory for maps and geography, and you can always recall the general layout of terrain, settlements, and other features."
            }),
            json!({
                "campaign_id": campaign_id,
                "name": "Entertainer",
                "description": "You thrive in front of an audience.",
                "skill_proficiencies": ["Acrobatics", "Performance"],
                "tool_proficiencies": ["Disguise kit", "One type of musical instrument"],
                "equipment": ["Musical instrument", "Favor from admirer", "Costume", "Belt pouch with 15 gp"],
                "feature_name": "By Popular Demand",
                "feature_description": "You can always find a place to perform. You receive free lodging and food of a modest standard as long as you perform each night."
            })
        ];

        let count = backgrounds.len();
        for background in backgrounds {
            self.graphql_client.insert_one("backgrounds", background).await?;
        }

        info!("Seeded {} backgrounds for campaign {}", count, campaign_id);
        Ok(())
    }
}