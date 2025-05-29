use crate::error::{ApiError, ApiResult};
use crate::models::{
    Campaign, CampaignDetail, CreateCampaignRequest, UpdateCampaignRequest, 
    Npc, Location, QuestHook, Encounter, GeneratedCampaignContent,
    CreateNpcRequest, CreateLocationRequest, CreateQuestHookRequest
};
use sqlx::{PgPool, Transaction, Postgres};
use std::collections::HashMap;
use tracing::{info, error};

pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Campaign operations
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

    pub async fn get_campaign_by_id(&self, id: i32) -> ApiResult<CampaignDetail> {
        // Get the campaign
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

        // Get all related data
        let npcs = self.get_campaign_npcs(id).await?;
        let locations = self.get_campaign_locations(id).await?;
        let quest_hooks = self.get_campaign_quest_hooks(id).await?;
        let encounters = self.get_campaign_encounters(id).await?;

        Ok(CampaignDetail {
            campaign,
            npcs,
            locations,
            quest_hooks,
            encounters,
        })
    }

    pub async fn update_campaign_status(&self, id: i32, status: &str) -> ApiResult<()> {
        let result = sqlx::query(
            "UPDATE campaigns SET status = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        )
        .bind(id)
        .bind(status)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }

    pub async fn save_generated_content(&self, campaign_id: i32, content: &GeneratedCampaignContent) -> ApiResult<()> {
        let mut tx = self.pool.begin().await?;
        
        info!("Saving generated content for campaign {}", campaign_id);
        
        // First, update campaign metadata with plot summary and conflict
        let metadata = serde_json::json!({
            "plot_summary": content.plot_summary,
            "central_conflict": content.central_conflict,
            "generation_timestamp": chrono::Utc::now()
        });
        
        sqlx::query(
            "UPDATE campaigns SET metadata = metadata || $2, updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        )
        .bind(campaign_id)
        .bind(&metadata)
        .execute(&mut *tx)
        .await?;

        // Create maps to track name -> id mappings
        let mut npc_name_to_id: HashMap<String, i32> = HashMap::new();
        let mut location_name_to_id: HashMap<String, i32> = HashMap::new();

        // Insert NPCs
        for generated_npc in &content.npcs {
            let personality = serde_json::to_value(&generated_npc.personality)?;
            let stats = if let Some(stats) = &generated_npc.stats {
                serde_json::json!({
                    "race": stats.race,
                    "class": stats.class,
                    "level": stats.level,
                    "abilities": stats.abilities
                })
            } else {
                serde_json::json!({})
            };

            let npc_id: i32 = sqlx::query_scalar(
                r#"
                INSERT INTO npcs (campaign_id, name, role, description, personality, stats, secret_info)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id
                "#
            )
            .bind(campaign_id)
            .bind(&generated_npc.name)
            .bind(&generated_npc.role)
            .bind(&generated_npc.description)
            .bind(&personality)
            .bind(&stats)
            .bind(&generated_npc.secret_info)
            .fetch_one(&mut *tx)
            .await?;

            npc_name_to_id.insert(generated_npc.name.clone(), npc_id);
        }

        // Insert locations
        for generated_location in &content.locations {
            let properties = serde_json::to_value(&generated_location.properties)?;

            let location_id: i32 = sqlx::query_scalar(
                r#"
                INSERT INTO locations (campaign_id, name, type, description, properties)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id
                "#
            )
            .bind(campaign_id)
            .bind(&generated_location.name)
            .bind(&generated_location.location_type)
            .bind(&generated_location.description)
            .bind(&properties)
            .fetch_one(&mut *tx)
            .await?;

            location_name_to_id.insert(generated_location.name.clone(), location_id);
        }

        // Update location connections
        for generated_location in &content.locations {
            if let Some(&location_id) = location_name_to_id.get(&generated_location.name) {
                let connection_ids: Vec<i32> = generated_location.connections
                    .iter()
                    .filter_map(|name| location_name_to_id.get(name).copied())
                    .collect();

                if !connection_ids.is_empty() {
                    sqlx::query(
                        "UPDATE locations SET connections = $2 WHERE id = $1"
                    )
                    .bind(location_id)
                    .bind(&connection_ids)
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }

        // Create NPC-location relationships
        for generated_location in &content.locations {
            if let Some(&location_id) = location_name_to_id.get(&generated_location.name) {
                for npc_name in &generated_location.resident_npcs {
                    if let Some(&npc_id) = npc_name_to_id.get(npc_name) {
                        sqlx::query(
                            r#"
                            INSERT INTO location_npcs (location_id, npc_id, relationship_type)
                            VALUES ($1, $2, 'resident')
                            ON CONFLICT (location_id, npc_id) DO NOTHING
                            "#
                        )
                        .bind(location_id)
                        .bind(npc_id)
                        .execute(&mut *tx)
                        .await?;
                    }
                }
            }
        }

        // Insert quest hooks
        for generated_quest in &content.quest_hooks {
            // Map NPC and location names to IDs
            let related_npc_ids: Vec<i32> = generated_quest.related_npcs
                .iter()
                .filter_map(|name| npc_name_to_id.get(name).copied())
                .collect();

            let related_location_ids: Vec<i32> = generated_quest.related_locations
                .iter()
                .filter_map(|name| location_name_to_id.get(name).copied())
                .collect();

            let requirements = serde_json::json!({
                "objectives": generated_quest.objectives
            });

            sqlx::query(
                r#"
                INSERT INTO quest_hooks 
                (campaign_id, title, description, difficulty, reward, 
                 related_npc_ids, related_location_ids, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, 'available')
                "#
            )
            .bind(campaign_id)
            .bind(&generated_quest.title)
            .bind(&generated_quest.description)
            .bind(&generated_quest.difficulty.to_lowercase())
            .bind(&generated_quest.reward)
            .bind(&related_npc_ids)
            .bind(&related_location_ids)
            .execute(&mut *tx)
            .await?;
        }

        // Update campaign status to ready
        sqlx::query(
            "UPDATE campaigns SET status = 'ready', updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        )
        .bind(campaign_id)
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;
        
        info!("Successfully saved all generated content for campaign {}", campaign_id);
        Ok(())
    }

    // Helper methods
    async fn get_campaign_npcs(&self, campaign_id: i32) -> ApiResult<Vec<Npc>> {
        let npcs = sqlx::query_as::<_, Npc>(
            r#"
            SELECT id, campaign_id, name, role, description, personality, stats, secret_info, created_at, updated_at
            FROM npcs
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(campaign_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(npcs)
    }

    async fn get_campaign_locations(&self, campaign_id: i32) -> ApiResult<Vec<Location>> {
        let locations = sqlx::query_as::<_, Location>(
            r#"
            SELECT id, campaign_id, name, type, description, connections, properties, created_at, updated_at
            FROM locations
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(campaign_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(locations)
    }

    async fn get_campaign_quest_hooks(&self, campaign_id: i32) -> ApiResult<Vec<QuestHook>> {
        let quest_hooks = sqlx::query_as::<_, QuestHook>(
            r#"
            SELECT id, campaign_id, title, description, difficulty, reward, related_npc_ids, related_location_ids, status, created_at, updated_at
            FROM quest_hooks
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(campaign_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(quest_hooks)
    }

    async fn get_campaign_encounters(&self, campaign_id: i32) -> ApiResult<Vec<Encounter>> {
        let encounters = sqlx::query_as::<_, Encounter>(
            r#"
            SELECT id, campaign_id, location_id, title, description, difficulty, creatures, environmental_factors, created_at, updated_at
            FROM encounters
            WHERE campaign_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(campaign_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(encounters)
    }
}