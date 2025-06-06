use crate::error::{ApiError, ApiResult};
use crate::models::{Campaign, CampaignDetail, CreateCampaignRequest, UpdateCampaignRequest, Npc, Location, QuestHook, Encounter};
use crate::services::{GraphQLClient, ReferenceDataService};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct CampaignService {
    graphql_client: Arc<GraphQLClient>,
    reference_data_service: Arc<ReferenceDataService>,
}

impl CampaignService {
    pub fn new(graphql_client: Arc<GraphQLClient>) -> Self {
        let reference_data_service = Arc::new(ReferenceDataService::new(graphql_client.clone()));
        Self { 
            graphql_client,
            reference_data_service,
        }
    }

    pub async fn create_campaign(&self, req: CreateCampaignRequest) -> ApiResult<Campaign> {
        // Merge enhanced data into metadata
        let mut metadata = req.metadata.unwrap_or(serde_json::json!({}));
        
        // Store the use_standard_content flag in metadata
        let use_standard_content = req.use_standard_content.unwrap_or(false);
        metadata["use_standard_content"] = json!(use_standard_content);
        
        if let Some(world_building) = req.world_building {
            metadata["world_building"] = serde_json::to_value(world_building).unwrap_or(serde_json::json!({}));
        }
        
        if let Some(campaign_specifics) = req.campaign_specifics {
            metadata["campaign_specifics"] = serde_json::to_value(campaign_specifics).unwrap_or(serde_json::json!({}));
        }
        
        if let Some(generation_preferences) = req.generation_preferences {
            metadata["generation_preferences"] = serde_json::to_value(generation_preferences).unwrap_or(serde_json::json!({}));
        }

        let campaign_object = json!({
            "name": req.name,
            "setting": req.setting,
            "themes": req.themes,
            "player_characters": req.player_characters.unwrap_or(serde_json::json!([])),
            "progression_type": req.progression_type.unwrap_or("milestone".to_string()),
            "tone": req.tone.unwrap_or("balanced".to_string()),
            "difficulty": req.difficulty.unwrap_or("medium".to_string()),
            "starting_level": req.starting_level.unwrap_or(1),
            "campaign_length": req.campaign_length.unwrap_or("medium".to_string()),
            "additional_notes": req.additional_notes,
            "metadata": metadata,
            "status": "created",
            "phase_progress": 0,
            "total_phases": 9
        });

        let result = self.graphql_client.insert_one("campaigns", campaign_object).await?;
        
        // Convert GraphQL result to Campaign model
        let campaign: Campaign = serde_json::from_value(result)
            .map_err(|e| ApiError::BadRequest(format!("Failed to parse campaign: {}", e)))?;

        // If use_standard_content is true, seed the reference data
        if use_standard_content {
            if let Err(e) = self.reference_data_service.seed_standard_dnd_content(campaign.id).await {
                tracing::error!("Failed to seed standard D&D content for campaign {}: {}", campaign.id, e);
                // We don't fail the campaign creation if seeding fails
                // The generation can still proceed with custom content
            }
        }

        Ok(campaign)
    }

    pub async fn get_campaign(&self, id: i32) -> ApiResult<Campaign> {
        let query = format!(
            r#"
            query GetCampaign($id: Int!) {{
                campaigns_by_pk(id: $id) {{
                    id
                    name
                    setting
                    themes
                    player_characters
                    status
                    generation_phase
                    phase_progress
                    total_phases
                    current_phase_status
                    error_message
                    progression_type
                    tone
                    difficulty
                    starting_level
                    campaign_length
                    additional_notes
                    metadata
                    created_at
                    updated_at
                }}
            }}
            "#
        );

        let variables = json!({
            "id": id
        });

        let result = self.graphql_client.execute(&query, Some(variables)).await?;
        
        let campaign_data = result
            .get("campaigns_by_pk")
            .ok_or_else(|| ApiError::NotFound)?;

        if campaign_data.is_null() {
            return Err(ApiError::NotFound);
        }

        let campaign: Campaign = serde_json::from_value(campaign_data.clone())
            .map_err(|e| ApiError::BadRequest(format!("Failed to parse campaign: {}", e)))?;

        Ok(campaign)
    }

    pub async fn list_campaigns(&self) -> ApiResult<Vec<Campaign>> {
        let query = r#"
            query ListCampaigns {
                campaigns(order_by: {created_at: desc}) {
                    id
                    name
                    setting
                    themes
                    player_characters
                    status
                    generation_phase
                    phase_progress
                    total_phases
                    current_phase_status
                    error_message
                    progression_type
                    tone
                    difficulty
                    starting_level
                    campaign_length
                    additional_notes
                    metadata
                    created_at
                    updated_at
                }
            }
        "#;

        let result = self.graphql_client.execute(query, None).await?;
        
        let campaigns_data = result
            .get("campaigns")
            .ok_or_else(|| ApiError::BadRequest("No campaigns data in response".to_string()))?;

        let campaigns: Vec<Campaign> = serde_json::from_value(campaigns_data.clone())
            .map_err(|e| ApiError::BadRequest(format!("Failed to parse campaigns: {}", e)))?;

        Ok(campaigns)
    }

    pub async fn update_campaign(&self, id: i32, req: UpdateCampaignRequest) -> ApiResult<Campaign> {
        let mut set_object = json!({});
        
        if let Some(name) = req.name {
            set_object["name"] = json!(name);
        }
        if let Some(setting) = req.setting {
            set_object["setting"] = json!(setting);
        }
        if let Some(themes) = req.themes {
            set_object["themes"] = json!(themes);
        }
        if let Some(player_characters) = req.player_characters {
            set_object["player_characters"] = json!(player_characters);
        }
        if let Some(status) = req.status {
            set_object["status"] = json!(status);
        }
        if let Some(metadata) = req.metadata {
            set_object["metadata"] = json!(metadata);
        }

        let pk_columns = json!({
            "id": id
        });

        let result = self.graphql_client.update_by_pk("campaigns", pk_columns, set_object).await?;
        
        if result.is_null() {
            return Err(ApiError::NotFound);
        }

        let campaign: Campaign = serde_json::from_value(result)
            .map_err(|e| ApiError::BadRequest(format!("Failed to parse campaign: {}", e)))?;

        Ok(campaign)
    }

    pub async fn delete_campaign(&self, id: i32) -> ApiResult<()> {
        let query = r#"
            mutation DeleteCampaign($id: Int!) {
                delete_campaigns_by_pk(id: $id) {
                    id
                }
            }
        "#;

        let variables = json!({
            "id": id
        });

        let result = self.graphql_client.execute(query, Some(variables)).await?;
        
        let deleted_campaign = result
            .get("delete_campaigns_by_pk")
            .ok_or_else(|| ApiError::NotFound)?;

        if deleted_campaign.is_null() {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }

    pub async fn get_campaign_detail(&self, id: i32) -> ApiResult<CampaignDetail> {
        let query = r#"
            query GetCampaignDetail($id: Int!) {
                campaigns_by_pk(id: $id) {
                    id
                    name
                    setting
                    themes
                    player_characters
                    status
                    generation_phase
                    phase_progress
                    total_phases
                    current_phase_status
                    error_message
                    progression_type
                    tone
                    difficulty
                    starting_level
                    campaign_length
                    additional_notes
                    metadata
                    created_at
                    updated_at
                    npcs(order_by: {created_at: asc}) {
                        id
                        campaign_id
                        name
                        role
                        description
                        personality
                        stats
                        secret_info
                        created_at
                        updated_at
                    }
                    locations(order_by: {created_at: asc}) {
                        id
                        campaign_id
                        name
                        location_type
                        description
                        properties
                        created_at
                        updated_at
                    }
                    quest_hooks(order_by: {created_at: asc}) {
                        id
                        campaign_id
                        title
                        description
                        difficulty
                        reward
                        created_at
                        updated_at
                    }
                    encounters(order_by: {created_at: asc}) {
                        id
                        campaign_id
                        name
                        description
                        difficulty
                        location_id
                        created_at
                        updated_at
                    }
                }
            }
        "#;

        let variables = json!({
            "id": id
        });

        let result = self.graphql_client.execute(query, Some(variables)).await?;
        
        let campaign_data = result
            .get("campaigns_by_pk")
            .ok_or_else(|| ApiError::NotFound)?;

        if campaign_data.is_null() {
            return Err(ApiError::NotFound);
        }

        // Extract campaign
        let campaign: Campaign = serde_json::from_value(campaign_data.clone())
            .map_err(|e| ApiError::BadRequest(format!("Failed to parse campaign: {}", e)))?;

        // Extract NPCs
        let npcs: Vec<Npc> = campaign_data
            .get("npcs")
            .map(|npcs_data| serde_json::from_value(npcs_data.clone()).unwrap_or_default())
            .unwrap_or_default();

        // Extract locations  
        let locations: Vec<Location> = campaign_data
            .get("locations")
            .map(|locations_data| serde_json::from_value(locations_data.clone()).unwrap_or_default())
            .unwrap_or_default();

        // Extract quest hooks
        let quest_hooks: Vec<QuestHook> = campaign_data
            .get("quest_hooks")
            .map(|quest_hooks_data| serde_json::from_value(quest_hooks_data.clone()).unwrap_or_default())
            .unwrap_or_default();

        // Extract encounters
        let encounters: Vec<Encounter> = campaign_data
            .get("encounters")
            .map(|encounters_data| serde_json::from_value(encounters_data.clone()).unwrap_or_default())
            .unwrap_or_default();

        Ok(CampaignDetail {
            campaign,
            npcs,
            locations,
            quest_hooks,
            encounters,
        })
    }
}