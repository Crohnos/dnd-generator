use std::env;
use tokio;
use dnd_campaign_generator::services::{GraphQLClient, HasuraSchemaGenerator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Hasura Schema Generation");
    println!("=====================================");

    // Get admin secret from environment
    let admin_secret = env::var("HASURA_ADMIN_SECRET").unwrap_or_else(|_| "myadminsecretkey".to_string());
    
    // Create GraphQL client
    let graphql_client = GraphQLClient::new(admin_secret);
    
    // Create schema generator
    let mut schema_generator = HasuraSchemaGenerator::new(graphql_client);
    
    println!("1. Testing Hasura introspection...");
    match schema_generator.initialize().await {
        Ok(()) => {
            println!("   ✅ Successfully connected to Hasura and ran introspection");
        }
        Err(e) => {
            println!("   ❌ Failed to initialize schema generator: {}", e);
            return Ok(());
        }
    }
    
    println!("\n2. Testing available tables...");
    let available_tables = schema_generator.get_available_tables();
    println!("   📊 Found {} tables:", available_tables.len());
    
    // Expected tables for 9-phase system
    let expected_tables = vec![
        // Phase 1A
        "calendar_systems", "planes", "geography_regions", "historical_periods", 
        "economic_systems", "legal_systems", "celestial_bodies",
        // Phase 1B  
        "races", "character_classes", "feats", "backgrounds",
        // Phase 1C
        "languages", "cultures", "factions", "pantheons", "deities",
        // Phase 2A
        "entities", 
        // Phase 2B
        "locations", "dungeons", "buildings", "shops", "taverns", "temples",
        // Phase 2C
        "items", "item_effects", "sentient_item_properties", 
        // Phase 3A
        "quest_hooks", "encounters",
        // Relationships
        "entity_relationships", "entity_locations", "entity_factions", 
        "entity_items", "location_items", "faction_relationships",
        // Existing
        "campaigns"
    ];
    
    let mut found_count = 0;
    let mut missing_tables = Vec::new();
    
    for expected_table in &expected_tables {
        if available_tables.contains(&expected_table.to_string()) {
            found_count += 1;
            println!("   ✅ {}", expected_table);
        } else {
            missing_tables.push(expected_table);
            println!("   ❌ {}", expected_table);
        }
    }
    
    println!("\n3. Testing phase-specific schema generation...");
    
    // Test Phase 1A schemas
    match schema_generator.get_phase_1a_schemas() {
        Some(tool) => {
            println!("   ✅ Phase 1A schemas generated successfully");
            println!("      Tool name: {}", tool.name);
            println!("      Tables in schema: {:?}", 
                tool.input_schema.get("properties")
                    .and_then(|p| p.as_object())
                    .map(|obj| obj.keys().collect::<Vec<_>>())
                    .unwrap_or_default()
            );
        }
        None => {
            println!("   ❌ Failed to generate Phase 1A schemas");
        }
    }
    
    // Test Phase 2A schemas
    match schema_generator.get_phase_2a_schemas() {
        Some(tool) => {
            println!("   ✅ Phase 2A schemas generated successfully");
            println!("      Tool name: {}", tool.name);
        }
        None => {
            println!("   ❌ Failed to generate Phase 2A schemas");
        }
    }
    
    // Test Phase 3C schemas  
    match schema_generator.get_phase_3c_schemas() {
        Some(tool) => {
            println!("   ✅ Phase 3C schemas generated successfully");
            println!("      Tool name: {}", tool.name);
        }
        None => {
            println!("   ❌ Failed to generate Phase 3C schemas");
        }
    }
    
    println!("\n📊 Summary:");
    println!("   Tables found: {}/{}", found_count, expected_tables.len());
    
    if missing_tables.is_empty() {
        println!("   🎉 All expected tables are available!");
        println!("   ✅ Hasura schema generation test PASSED");
    } else {
        println!("   ⚠️  Missing tables: {:?}", missing_tables);
        println!("   ❌ Hasura schema generation test FAILED");
    }
    
    Ok(())
}