use crate::error::{ApiError, ApiResult};
use crate::models::{Campaign, CampaignDetail, CreateCampaignRequest, UpdateCampaignRequest, Npc, Location, QuestHook, Encounter};
use sqlx::PgPool;

pub struct CampaignService {
    pool: PgPool,
}

impl CampaignService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_campaign(&self, req: CreateCampaignRequest) -> ApiResult<Campaign> {
        // Merge enhanced data into metadata
        let mut metadata = req.metadata.unwrap_or(serde_json::json!({}));
        
        if let Some(world_building) = req.world_building {
            metadata["world_building"] = serde_json::to_value(world_building).unwrap_or(serde_json::json!({}));
        }
        
        if let Some(campaign_specifics) = req.campaign_specifics {
            metadata["campaign_specifics"] = serde_json::to_value(campaign_specifics).unwrap_or(serde_json::json!({}));
        }
        
        if let Some(generation_preferences) = req.generation_preferences {
            metadata["generation_preferences"] = serde_json::to_value(generation_preferences).unwrap_or(serde_json::json!({}));
        }

        let campaign = sqlx::query_as::<_, Campaign>(
            r#"
            INSERT INTO campaigns (
                name, setting, themes, player_characters, 
                progression_type, tone, difficulty, starting_level, 
                campaign_length, additional_notes, metadata,
                status, generation_phase, phase_progress, total_phases, current_phase_status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'created', NULL, 0, 3, NULL)
            RETURNING id, name, setting, themes, player_characters, status, 
                     generation_phase, phase_progress, total_phases, current_phase_status, error_message,
                     progression_type, tone, difficulty, starting_level, campaign_length, 
                     additional_notes, metadata, created_at, updated_at
            "#,
        )
        .bind(req.name)
        .bind(req.setting)
        .bind(&req.themes)
        .bind(req.player_characters.unwrap_or(serde_json::json!([])))
        .bind(req.progression_type.unwrap_or("milestone".to_string()))
        .bind(req.tone.unwrap_or("balanced".to_string()))
        .bind(req.difficulty.unwrap_or("medium".to_string()))
        .bind(req.starting_level.unwrap_or(1))
        .bind(req.campaign_length.unwrap_or("medium".to_string()))
        .bind(req.additional_notes)
        .bind(metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(campaign)
    }

    pub async fn get_campaign(&self, id: i32) -> ApiResult<Campaign> {
        let campaign = sqlx::query_as::<_, Campaign>(
            r#"
            SELECT id, name, setting, themes, player_characters, status, 
                   generation_phase, phase_progress, total_phases, current_phase_status, error_message,
                   progression_type, tone, difficulty, starting_level, campaign_length, 
                   additional_notes, metadata, created_at, updated_at
            FROM campaigns
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::Database(e),
        })?;

        Ok(campaign)
    }

    pub async fn list_campaigns(&self) -> ApiResult<Vec<Campaign>> {
        let campaigns = sqlx::query_as::<_, Campaign>(
            r#"
            SELECT id, name, setting, themes, player_characters, status, 
                   generation_phase, phase_progress, total_phases, current_phase_status, error_message,
                   progression_type, tone, difficulty, starting_level, campaign_length, 
                   additional_notes, metadata, created_at, updated_at
            FROM campaigns
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(campaigns)
    }

    pub async fn update_campaign(&self, id: i32, req: UpdateCampaignRequest) -> ApiResult<Campaign> {
        let campaign = sqlx::query_as::<_, Campaign>(
            r#"
            UPDATE campaigns 
            SET 
                name = COALESCE($2, name),
                setting = COALESCE($3, setting),
                themes = COALESCE($4, themes),
                player_characters = COALESCE($5, player_characters),
                status = COALESCE($6, status),
                metadata = COALESCE($7, metadata),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING id, name, setting, themes, player_characters, status, 
                     generation_phase, phase_progress, total_phases, current_phase_status, error_message,
                     progression_type, tone, difficulty, starting_level, campaign_length, 
                     additional_notes, metadata, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(req.name)
        .bind(req.setting)
        .bind(req.themes)
        .bind(req.player_characters)
        .bind(req.status)
        .bind(req.metadata)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::Database(e),
        })?;

        Ok(campaign)
    }

    pub async fn delete_campaign(&self, id: i32) -> ApiResult<()> {
        let result = sqlx::query("DELETE FROM campaigns WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }

    pub async fn get_campaign_detail(&self, id: i32) -> ApiResult<CampaignDetail> {
        // Get the campaign
        let campaign = self.get_campaign(id).await?;

        // Get related NPCs
        let npcs = sqlx::query_as::<_, Npc>(
            r#"
            SELECT id, campaign_id, name, role, description, personality, stats, secret_info, created_at, updated_at
            FROM npcs
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        // Get related locations
        let locations = sqlx::query_as::<_, Location>(
            r#"
            SELECT id, campaign_id, name, type, description, connections, properties, created_at, updated_at
            FROM locations
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        // Get related quest hooks - TODO: Implement from enhanced schema
        let quest_hooks = Vec::new();

        // Get related encounters - TODO: Implement from enhanced schema  
        let encounters = Vec::new();

        Ok(CampaignDetail {
            campaign,
            npcs,
            locations,
            quest_hooks,
            encounters,
        })
    }
}