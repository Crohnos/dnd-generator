use crate::config::Config;
use crate::services::{AnthropicClient, CampaignService, DatabaseServiceEnhanced, GenerationServiceEnhanced, GraphQLClient, HasuraSchemaGenerator};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub campaign_service: Arc<CampaignService>,
    pub database_service_enhanced: Arc<DatabaseServiceEnhanced>,
    pub generation_service: Arc<GenerationServiceEnhanced>,
    pub schema_generator: Arc<RwLock<HasuraSchemaGenerator>>,
}

impl AppState {
    pub async fn new(pool: PgPool, config: &Config) -> crate::error::ApiResult<Self> {
        let database_service_enhanced = Arc::new(DatabaseServiceEnhanced::new(pool.clone()));
        let graphql_client = Arc::new(GraphQLClient::new(
            std::env::var("HASURA_ADMIN_SECRET").unwrap_or_else(|_| "myadminsecretkey".to_string())
        ));
        
        // Initialize schema generator
        let mut schema_generator = HasuraSchemaGenerator::new(graphql_client.as_ref().clone());
        schema_generator.initialize().await?;
        let schema_generator = Arc::new(RwLock::new(schema_generator));
        
        let anthropic_client = Arc::new(AnthropicClient::new(config.anthropic_api_key.clone()));
        let generation_service = Arc::new(GenerationServiceEnhanced::new(
            database_service_enhanced.clone(),
            graphql_client,
            anthropic_client,
            schema_generator.clone(),
        ));
        
        Ok(Self {
            campaign_service: Arc::new(CampaignService::new(pool)),
            database_service_enhanced,
            generation_service,
            schema_generator,
        })
    }
}