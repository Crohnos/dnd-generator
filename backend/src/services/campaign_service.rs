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
        let campaign = sqlx::query_as::<_, Campaign>(
            r#"
            INSERT INTO campaigns (name, setting, themes, player_characters, metadata)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, setting, themes, player_characters, status, metadata, created_at, updated_at
            "#,
        )
        .bind(req.name)
        .bind(req.setting)
        .bind(&req.themes)
        .bind(req.player_characters.unwrap_or(serde_json::json!([])))
        .bind(req.metadata.unwrap_or(serde_json::json!({})))
        .fetch_one(&self.pool)
        .await?;

        Ok(campaign)
    }

    pub async fn get_campaign(&self, id: i32) -> ApiResult<Campaign> {
        let campaign = sqlx::query_as::<_, Campaign>(
            r#"
            SELECT id, name, setting, themes, player_characters, status, metadata, created_at, updated_at
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
            SELECT id, name, setting, themes, player_characters, status, metadata, created_at, updated_at
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
            RETURNING id, name, setting, themes, player_characters, status, metadata, created_at, updated_at
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

        // Get related quest hooks
        let quest_hooks = sqlx::query_as::<_, QuestHook>(
            r#"
            SELECT id, campaign_id, title, description, difficulty, reward, related_npc_ids, related_location_ids, status, created_at, updated_at
            FROM quest_hooks
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        // Get related encounters
        let encounters = sqlx::query_as::<_, Encounter>(
            r#"
            SELECT id, campaign_id, location_id, title, description, difficulty, creatures, environmental_factors, created_at, updated_at
            FROM encounters
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(CampaignDetail {
            campaign,
            npcs,
            locations,
            quest_hooks,
            encounters,
        })
    }
}