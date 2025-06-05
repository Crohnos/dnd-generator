use dnd_campaign_generator::services::{GraphQLClient, HasuraSchemaGenerator};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the schema generator
    let graphql_client = GraphQLClient::new("myadminsecretkey".to_string());
    let mut schema_generator = HasuraSchemaGenerator::new(graphql_client);
    
    // Initialize schemas from Hasura
    schema_generator.initialize().await?;
    
    // Test Phase 2B schemas
    if let Some(tool) = schema_generator.get_phase_2b_schemas() {
        println!("Phase 2B Tool Schema:");
        println!("{}", serde_json::to_string_pretty(&tool.input_schema)?);
        
        // Specifically check buildings schema
        if let Some(properties) = tool.input_schema.get("properties") {
            if let Some(buildings) = properties.get("buildings") {
                println!("\nBuildings array schema:");
                println!("{}", serde_json::to_string_pretty(buildings)?);
                
                if let Some(items) = buildings.get("items") {
                    println!("\nBuildings item schema (this is what Claude gets):");
                    println!("{}", serde_json::to_string_pretty(items)?);
                    
                    if let Some(item_props) = items.get("properties") {
                        if item_props.get("campaign_id").is_some() {
                            println!("\n❌ ERROR: Found campaign_id in buildings schema!");
                        } else {
                            println!("\n✅ Good: No campaign_id in buildings schema");
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to get Phase 2B schemas");
    }
    
    Ok(())
}