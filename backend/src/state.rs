use crate::config::Config;
use crate::services::{AnthropicClient, CampaignService, DatabaseService, GenerationService};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub campaign_service: Arc<CampaignService>,
    pub database_service: Arc<DatabaseService>,
    pub generation_service: Arc<GenerationService>,
}

impl AppState {
    pub fn new(pool: PgPool, config: &Config) -> Self {
        let database_service = Arc::new(DatabaseService::new(pool.clone()));
        let anthropic_client = Arc::new(AnthropicClient::new(config.anthropic_api_key.clone()));
        let generation_service = Arc::new(GenerationService::new(
            database_service.clone(),
            anthropic_client,
        ));
        
        Self {
            campaign_service: Arc::new(CampaignService::new(pool)),
            database_service,
            generation_service,
        }
    }
}