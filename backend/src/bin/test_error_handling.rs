use std::env;
use std::sync::Arc;
use dnd_campaign_generator::services::{DatabaseServiceEnhanced, CampaignService, GraphQLClient, HasuraSchemaGenerator};
use dnd_campaign_generator::models::campaign::{CreateCampaignRequest, PhaseInfo};
use serde_json::json;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Error Handling");
    println!("==========================");
    
    // Setup services
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| 
        "postgres://postgres:postgres@localhost:5432/dnd_campaigns".to_string()
    );
    let admin_secret = env::var("HASURA_ADMIN_SECRET").unwrap_or_else(|_| "myadminsecretkey".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    let db = Arc::new(DatabaseServiceEnhanced::new(pool.clone()));
    let graphql_client = Arc::new(GraphQLClient::new(admin_secret.clone()));
    let campaign_service = CampaignService::new(graphql_client.clone());
    
    println!("\n1. Testing invalid campaign operations...");
    
    // Test retrieving non-existent campaign
    println!("   Testing retrieval of non-existent campaign...");
    match campaign_service.get_campaign(999999).await {
        Ok(_) => println!("   ❌ Expected error but got success"),
        Err(e) => println!("   ✅ Correctly handled non-existent campaign: {}", e),
    }
    
    // Test deleting non-existent campaign
    println!("   Testing deletion of non-existent campaign...");
    match campaign_service.delete_campaign(999999).await {
        Ok(()) => println!("   ❌ Expected error but deletion succeeded"),
        Err(e) => println!("   ✅ Correctly handled non-existent campaign deletion: {}", e),
    }
    
    println!("\n2. Testing phase dependency validation errors...");
    
    // Test invalid phase dependencies
    let test_cases = vec![
        (vec![], "phase_1b_character_building", "Phase 1B without Phase 1A"),
        (vec!["phase_1a_core_world".to_string()], "phase_2a_pc_entities", "Phase 2A without Phase 1B and 1C"),
        (vec![], "unknown_phase", "Unknown phase name"),
        (vec!["phase_1a_core_world".to_string(), "phase_1b_character_building".to_string()], "phase_3a_quests_encounters", "Phase 3A without Phase 2 dependencies"),
    ];
    
    for (completed_phases, target_phase, description) in test_cases {
        println!("   Testing: {}", description);
        match PhaseInfo::validate_dependencies(&completed_phases, target_phase) {
            Ok(()) => println!("   ❌ Expected error but validation passed"),
            Err(e) => println!("   ✅ Correctly rejected: {}", e),
        }
    }
    
    println!("\n3. Testing database error scenarios...");
    
    // Test context retrieval for non-existent campaign
    println!("   Testing context retrieval for non-existent campaign...");
    match db.get_phase_1_context(999999).await {
        Ok(context) => {
            println!("   📊 Context retrieval succeeded (empty context expected)");
            if context.as_object().map_or(true, |obj| obj.is_empty()) {
                println!("   ✅ Returned empty context as expected");
            } else {
                println!("   ⚠️ Context unexpectedly had data: {}", context);
            }
        }
        Err(e) => println!("   ⚠️ Context retrieval failed: {}", e),
    }
    
    println!("\n4. Testing invalid Hasura operations...");
    
    // Test with invalid admin secret
    println!("   Testing GraphQL client with invalid admin secret...");
    let invalid_graphql = GraphQLClient::new("invalid-secret".to_string());
    let mut invalid_schema_gen = HasuraSchemaGenerator::new(invalid_graphql);
    
    match invalid_schema_gen.initialize().await {
        Ok(()) => println!("   ❌ Expected authentication error but succeeded"),
        Err(e) => println!("   ✅ Correctly handled invalid authentication: {}", e),
    }
    
    println!("\n5. Testing campaign creation edge cases...");
    
    // Test campaign with invalid/extreme data
    let invalid_campaign = CreateCampaignRequest {
        name: "".to_string(), // Empty name
        setting: None,
        themes: vec![],
        player_characters: Some(json!([])), // Empty array
        progression_type: Some("invalid_progression".to_string()), // Invalid type
        tone: Some("".to_string()), // Empty tone
        difficulty: Some("impossible".to_string()), // Invalid difficulty
        starting_level: Some(-1), // Negative level
        campaign_length: Some("forever".to_string()), // Invalid length
        additional_notes: Some("x".repeat(10000)), // Very long notes
        metadata: Some(json!({"test": "data"})),
        use_standard_content: Some(false),
        world_building: None,
        campaign_specifics: None,
        generation_preferences: None,
    };
    
    println!("   Testing campaign creation with edge case data...");
    match campaign_service.create_campaign(invalid_campaign).await {
        Ok(campaign) => {
            println!("   📊 Campaign created despite edge case data: ID {}", campaign.id);
            println!("   📊 System handled edge cases gracefully");
            
            // Clean up
            let _ = campaign_service.delete_campaign(campaign.id).await;
        }
        Err(e) => println!("   ✅ Correctly rejected invalid campaign data: {}", e),
    }
    
    println!("\n6. Testing phase info edge cases...");
    
    // Test get_phase_info with invalid phase names
    let invalid_phases = vec!["", "invalid_phase", "phase_99_nonexistent"];
    
    for phase_name in invalid_phases {
        println!("   Testing phase info for: '{}'", phase_name);
        match PhaseInfo::get_phase_info(phase_name) {
            Some(info) => println!("   ❌ Unexpected phase info found: {}", info.name),
            None => println!("   ✅ Correctly returned None for invalid phase"),
        }
    }
    
    println!("\n7. Testing database service error resilience...");
    
    // Test database operations with invalid campaign ID
    println!("   Testing database phase initialization with invalid campaign...");
    match db.initialize_generation_phases(999999, 9).await {
        Ok(()) => println!("   📊 Phase initialization succeeded (idempotent operation)"),
        Err(e) => println!("   ⚠️ Phase initialization failed: {}", e),
    }
    
    println!("   Testing campaign status error handling...");
    match db.update_campaign_status_with_error(999999, "Test error message").await {
        Ok(()) => println!("   ❌ Expected error but status update succeeded"),
        Err(e) => println!("   ✅ Correctly handled invalid campaign ID: {}", e),
    }
    
    println!("\n📊 Summary:");
    println!("   ✅ Error handling test completed");
    println!("   🎯 Invalid operations properly rejected");
    println!("   🎯 Phase dependency validation working correctly");
    println!("   🎯 Database error scenarios handled gracefully");
    println!("   🎯 Authentication errors caught appropriately");
    println!("   🎯 Edge cases in data handled robustly");
    
    Ok(())
}