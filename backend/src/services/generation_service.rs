use crate::error::{ApiError, ApiResult};
use crate::models::Campaign;
use crate::services::{AnthropicClient, DatabaseService};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

pub struct GenerationService {
    database: Arc<DatabaseService>,
    anthropic: Arc<AnthropicClient>,
}

impl GenerationService {
    pub fn new(database: Arc<DatabaseService>, anthropic: Arc<AnthropicClient>) -> Self {
        Self { database, anthropic }
    }

    pub async fn generate_campaign_content(&self, campaign_id: i32) -> ApiResult<()> {
        info!("Starting content generation for campaign {}", campaign_id);
        
        // Update status to generating
        self.database.update_campaign_status(campaign_id, "generating").await?;
        
        // Get campaign details
        let campaign_detail = match self.database.get_campaign_by_id(campaign_id).await {
            Ok(detail) => detail,
            Err(e) => {
                error!("Failed to get campaign: {}", e);
                self.database.update_campaign_status(campaign_id, "error").await?;
                return Err(e);
            }
        };
        
        let campaign = campaign_detail.campaign;
        
        // Generate content
        let generated_content = match self.anthropic.generate_campaign_content(&campaign).await {
            Ok(content) => content,
            Err(e) => {
                error!("Failed to generate content: {}", e);
                self.database.update_campaign_status(campaign_id, "error").await?;
                return Err(e);
            }
        };
        
        // Save generated content
        match self.database.save_generated_content(campaign_id, &generated_content).await {
            Ok(_) => {
                info!("Successfully completed content generation for campaign {}", campaign_id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to save generated content: {}", e);
                self.database.update_campaign_status(campaign_id, "error").await?;
                Err(e)
            }
        }
    }
}