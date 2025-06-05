use crate::error::{ApiError, ApiResult};
use crate::models::{Campaign, PhaseInfo, TOTAL_PHASES};
use crate::services::{AnthropicClient, DatabaseServiceEnhanced, GraphQLClient, HasuraSchemaGenerator, Tool};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

pub struct GenerationServiceEnhanced {
    database: Arc<DatabaseServiceEnhanced>,
    graphql: Arc<GraphQLClient>,
    anthropic: Arc<AnthropicClient>,
    schema_generator: Arc<RwLock<HasuraSchemaGenerator>>,
}

#[derive(Debug)]
struct GenerationPhase {
    name: String,
    description: String,
    max_tokens: u32,
    temperature: f32,
}

impl GenerationServiceEnhanced {
    pub fn new(
        database: Arc<DatabaseServiceEnhanced>, 
        graphql: Arc<GraphQLClient>, 
        anthropic: Arc<AnthropicClient>,
        schema_generator: Arc<RwLock<HasuraSchemaGenerator>>
    ) -> Self {
        Self { database, graphql, anthropic, schema_generator }
    }

    pub async fn generate_campaign_content(&self, campaign_id: i32) -> ApiResult<()> {
        info!("Starting enhanced 9-phase content generation for campaign {}", campaign_id);

        // Get phase configuration from the campaign model
        let phase_infos = PhaseInfo::get_all_phases();
        let phases: Vec<GenerationPhase> = phase_infos.iter().map(|phase_info| {
            GenerationPhase {
                name: phase_info.name.clone(),
                description: phase_info.description.clone(),
                max_tokens: match phase_info.name.as_str() {
                    "phase_1a_core_world" => 8000,
                    "phase_1b_character_building" => 6000,
                    "phase_1c_social_framework" => 7000,
                    "phase_2a_pc_entities" => 8000,
                    "phase_2b_pc_locations" => 8000,
                    "phase_2c_pc_items" => 6000,
                    "phase_3a_quests_encounters" => 10000,
                    "phase_3b_world_population" => 12000,
                    "phase_3c_relationships" => 8000,
                    _ => 8000,
                },
                temperature: if phase_info.name.starts_with("phase_1") { 0.7 } 
                            else if phase_info.name.starts_with("phase_2") { 0.8 }
                            else { 0.9 },
            }
        }).collect();

        // Initialize phase tracking
        self.database.initialize_generation_phases(campaign_id, TOTAL_PHASES).await?;

        // Track completed phases for dependency validation
        let mut completed_phases: Vec<String> = Vec::new();

        // Execute each phase with dependency validation
        for (phase_number, phase) in phases.iter().enumerate() {
            info!("Starting Phase {}: {}", phase_number + 1, phase.name);
            
            // Validate dependencies before executing phase
            if let Err(e) = PhaseInfo::validate_dependencies(&completed_phases, &phase.name) {
                error!("Phase dependency validation failed: {}", e);
                self.database.update_campaign_status_with_error(
                    campaign_id,
                    &format!("Dependency validation failed for phase {}: {}", phase_number + 1, e),
                ).await?;
                return Err(ApiError::BadRequest(e));
            }
            
            match self.execute_phase(campaign_id, phase, phase_number as i32 + 1).await {
                Ok(_) => {
                    self.database.update_generation_phase(
                        campaign_id,
                        &phase.name,
                        ((phase_number + 1) * 100 / phases.len()) as i32,
                        Some("completed"),
                    ).await?;
                    completed_phases.push(phase.name.clone());
                    info!("Completed Phase {}: {}", phase_number + 1, phase.name);
                }
                Err(e) => {
                    error!("Failed Phase {}: {} - Error: {}", phase_number + 1, phase.name, e);
                    self.database.update_campaign_status_with_error(
                        campaign_id,
                        &format!("Failed in phase {}: {}", phase_number + 1, e),
                    ).await?;
                    return Err(e);
                }
            }
        }

        // Mark campaign as ready
        self.database.update_generation_phase(campaign_id, "completed", 100, Some("all_phases_complete")).await?;
        self.database.update_campaign_status_completed(campaign_id).await?;
        info!("Successfully completed all 9 phases for campaign {}", campaign_id);

        Ok(())
    }

    async fn execute_phase(&self, campaign_id: i32, phase: &GenerationPhase, phase_number: i32) -> ApiResult<()> {
        match phase.name.as_str() {
            "phase_1a_core_world" => self.execute_phase_1a_core_world(campaign_id, phase, phase_number).await,
            "phase_1b_character_building" => self.execute_phase_1b_character_building(campaign_id, phase, phase_number).await,
            "phase_1c_social_framework" => self.execute_phase_1c_social_framework(campaign_id, phase, phase_number).await,
            "phase_2a_pc_entities" => self.execute_phase_2a_pc_entities(campaign_id, phase, phase_number).await,
            "phase_2b_pc_locations" => self.execute_phase_2b_pc_locations(campaign_id, phase, phase_number).await,
            "phase_2c_pc_items" => self.execute_phase_2c_pc_items(campaign_id, phase, phase_number).await,
            "phase_3a_quests_encounters" => self.execute_phase_3a_quests_encounters(campaign_id, phase, phase_number).await,
            "phase_3b_world_population" => self.execute_phase_3b_world_population(campaign_id, phase, phase_number).await,
            "phase_3c_relationships" => self.execute_phase_3c_relationships(campaign_id, phase, phase_number).await,
            _ => Err(ApiError::BadRequest(format!("Unknown phase: {}", phase.name))),
        }
    }

    // Phase 1A: Core World Systems
    async fn execute_phase_1a_core_world(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 1A: Core World Systems for campaign {}", campaign_id);

        // Get campaign details for context
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        // Build prompt with campaign context
        let prompt = format!(
            "You are creating core world systems for the D&D 5e campaign: '{}'\n\n\
            Setting: {}\n\
            Themes: {}\n\
            Tone: {}\n\
            Campaign Length: {}\n\n\
            {}\n\n\
            Generate comprehensive foundational world systems that are independent of player characters.\n\
            Focus on creating: calendar systems, planes of existence, geography, historical periods, \
            economic systems, legal frameworks, and celestial bodies.\n\n\
            Use the provided tool to structure your response with all required data.",
            campaign.name,
            campaign.setting.as_deref().unwrap_or("Fantasy world"),
            campaign.themes.join(", "),
            campaign.tone,
            campaign.campaign_length,
            campaign.additional_notes.as_deref().unwrap_or("")
        );

        // Get phase-specific tool schema from Hasura
        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_1a_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 1A schemas".to_string()))?;

        // Call AI with tool to generate content
        info!("Calling Anthropic API for Phase 1A generation...");
        let response = match self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await {
            Ok(resp) => {
                info!("Successfully received response from Anthropic API");
                resp
            },
            Err(e) => {
                error!("Failed to generate content with Anthropic API: {:?}", e);
                return Err(e);
            }
        };
        
        // Save content using database service
        info!("Saving Phase 1A content to database...");
        self.save_phase_1a_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 1B: Character Building Systems
    async fn execute_phase_1b_character_building(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 1B: Character Building Systems for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let phase_1a_context = self.database.get_phase_1a_context(campaign_id).await?;
        
        let prompt = format!(
            "You are creating character building systems for the D&D 5e campaign: '{}'\n\n\
            Setting: {}\n\
            Themes: {}\n\
            Tone: {}\n\n\
            World Context from Phase 1A:\n\
            {}\n\n\
            Generate character creation systems including races, classes, feats, and backgrounds \
            that fit the established world. Ensure racial origins tie to the geography and \
            cultural elements align with the world's tone and themes.\n\n\
            Use the provided tool to structure your response.",
            campaign.name,
            campaign.setting.as_deref().unwrap_or("Fantasy world"),
            campaign.themes.join(", "),
            campaign.tone,
            serde_json::to_string_pretty(&phase_1a_context).unwrap_or_default()
        );

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_1b_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 1B schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_1b_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 1C: Social Framework
    async fn execute_phase_1c_social_framework(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 1C: Social Framework for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let phase_1_context = self.database.get_phase_1_context(campaign_id).await?;
        
        let prompt = format!(
            "You are creating social framework for the D&D 5e campaign: '{}'\n\n\
            Setting: {}\n\
            Themes: {}\n\
            Tone: {}\n\n\
            Established World Systems:\n\
            {}\n\n\
            Generate social and religious systems including languages, cultures, factions, \
            pantheons, and deities. Build on the established geography, races, and world history. \
            Create interconnected social systems that reflect the campaign's themes.\n\n\
            Use the provided tool to structure your response.",
            campaign.name,
            campaign.setting.as_deref().unwrap_or("Fantasy world"),
            campaign.themes.join(", "),
            campaign.tone,
            serde_json::to_string_pretty(&phase_1_context).unwrap_or_default()
        );

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_1c_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 1C schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_1c_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 2A: PC-Connected Entities
    async fn execute_phase_2a_pc_entities(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 2A: PC-Connected Entities for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let world_context = self.database.get_phase_1_context(campaign_id).await?;
        
        let mut prompt = format!(
            "You are creating PC-connected entities for the D&D 5e campaign: '{}'\n\n\
            Setting: {}\n\
            Themes: {}\n\
            Tone: {}\n\n\
            Player Characters:\n",
            campaign.name,
            campaign.setting.as_deref().unwrap_or("Fantasy world"),
            campaign.themes.join(", "),
            campaign.tone
        );

        // Add PC details
        if let Some(pcs) = campaign.player_characters.as_array() {
            for pc in pcs {
                prompt.push_str(&format!(
                    "- {} ({} {}, Level {}): {}\n",
                    pc.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    pc.get("race").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    pc.get("class").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    pc.get("level").and_then(|v| v.as_i64()).unwrap_or(1),
                    pc.get("backstory").and_then(|v| v.as_str()).unwrap_or("No backstory")
                ));
            }
        }

        prompt.push_str(&format!(
            "\n\nEstablished World Context:\n\
            {}\n\n\
            Generate NPCs and entities that have direct connections to the player character backstories. \
            Each entity should have clear relationships to specific PCs based on their backgrounds, \
            motivations, and story hooks. Focus on family members, mentors, rivals, allies, and \
            significant figures from their past.\n\n\
            Use the provided tool to structure your response.",
            serde_json::to_string_pretty(&world_context).unwrap_or_default()
        ));

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_2a_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 2A schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_2a_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 2B: PC-Connected Locations
    async fn execute_phase_2b_pc_locations(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 2B: PC-Connected Locations for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let phase_context = self.database.get_phase_2_context(campaign_id).await?;
        
        let prompt = format!(
            "You are creating PC-connected locations for the D&D 5e campaign: '{}'\n\n\
            Established Context:\n\
            {}\n\n\
            Generate locations that are directly tied to PC backstories and the entities created in Phase 2A. \
            Create hierarchical locations (cities → districts → buildings) that serve as homes, origins, \
            training grounds, and significant places from PC histories. Each location should have clear \
            connections to specific player characters.\n\n\
            Use the provided tool to structure your response.",
            campaign.name,
            serde_json::to_string_pretty(&phase_context).unwrap_or_default()
        );

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_2b_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 2B schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_2b_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 2C: PC-Connected Items
    async fn execute_phase_2c_pc_items(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 2C: PC-Connected Items for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let phase_context = self.database.get_phase_2_context(campaign_id).await?;
        
        let prompt = format!(
            "You are creating PC-connected items for the D&D 5e campaign: '{}'\n\n\
            Established Context:\n\
            {}\n\n\
            Generate equipment, artifacts, and magical items that are relevant to PC stories and \
            connected to the entities and locations from previous phases. Create items with personal \
            significance: family heirlooms, training weapons, artifacts from mentors, quest items, \
            and tools that tie into PC backstories and future plot development.\n\n\
            Use the provided tool to structure your response.",
            campaign.name,
            serde_json::to_string_pretty(&phase_context).unwrap_or_default()
        );

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_2c_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 2C schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_2c_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 3A: Quest Hooks & Encounters
    async fn execute_phase_3a_quests_encounters(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 3A: Quest Hooks & Encounters for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let all_context = self.database.get_phase_3_context(campaign_id).await?;
        
        let prompt = format!(
            "You are creating quest hooks and encounters for the D&D 5e campaign: '{}'\n\n\
            Complete Context from Previous Phases:\n\
            {}\n\n\
            Generate adventure hooks, missions, and encounters that build on all the established content. \
            Create quests that involve the PC-connected entities, utilize the established locations, \
            and incorporate the world systems. Design encounters that challenge the party and advance \
            the overall campaign narrative while respecting the tone and themes.\n\n\
            Use the provided tool to structure your response.",
            campaign.name,
            serde_json::to_string_pretty(&all_context).unwrap_or_default()
        );

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_3a_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 3A schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_3a_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 3B: World Population
    async fn execute_phase_3b_world_population(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 3B: World Population for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let all_context = self.database.get_phase_3_context(campaign_id).await?;
        
        let prompt = format!(
            "You are populating the world for the D&D 5e campaign: '{}'\n\n\
            Established Context:\n\
            {}\n\n\
            Generate additional world population to flesh out the established locations. Create shops, \
            taverns, temples, and other businesses that make the world feel alive. Populate locations \
            with additional NPCs, services, and points of interest that support the established narrative \
            and provide resources for the party's adventures.\n\n\
            Use the provided tool to structure your response.",
            campaign.name,
            serde_json::to_string_pretty(&all_context).unwrap_or_default()
        );

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_3b_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 3B schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_3b_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 3C: Final Relationships
    async fn execute_phase_3c_relationships(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing Phase 3C: Final Relationships for campaign {}", campaign_id);

        let campaign = self.database.get_campaign(campaign_id).await?;
        let all_context = self.database.get_phase_3_context(campaign_id).await?;
        
        let prompt = format!(
            "You are creating final relationships for the D&D 5e campaign: '{}'\n\n\
            Complete Campaign Context:\n\
            {}\n\n\
            Generate the final relationship network that connects all entities, locations, factions, \
            and items created in previous phases. Create entity-to-entity relationships, establish \
            faction alliances and rivalries, connect entities to their home locations, assign item \
            ownership, and create the social web that makes the campaign world feel interconnected \
            and alive.\n\n\
            Use the provided tool to structure your response.",
            campaign.name,
            serde_json::to_string_pretty(&all_context).unwrap_or_default()
        );

        let schema_gen = self.schema_generator.read().await;
        let tool = schema_gen.get_phase_3c_schemas()
            .ok_or_else(|| ApiError::BadRequest("Failed to generate Phase 3C schemas".to_string()))?;

        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        self.save_phase_3c_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 2: PC Connected Content  
    async fn execute_pc_connected_phase(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing PC Connected Phase for campaign {}", campaign_id);

        // Get world building data for context
        let world_data = self.database.get_world_building_data(campaign_id).await?;
        
        // Generate PC-connected prompt and tool
        let (prompt, tool) = self.build_pc_connected_prompt_with_tool(campaign_id, &world_data).await?;
        
        // Call AI with tool to generate PC-connected content
        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        
        // Save PC-connected content
        self.save_pc_connected_content(campaign_id, &response).await?;

        Ok(())
    }

    // Phase 3: World Population
    async fn execute_world_population_phase(&self, campaign_id: i32, phase: &GenerationPhase, _phase_number: i32) -> ApiResult<()> {
        info!("Executing World Population Phase for campaign {}", campaign_id);

        // Get context from previous phases
        let world_data = self.database.get_world_building_data(campaign_id).await?;
        let pc_data = self.database.get_pc_connected_data(campaign_id).await?;
        
        // Generate world population prompt and tool
        let (prompt, tool) = self.build_world_population_prompt_with_tool(campaign_id, &world_data, &pc_data).await?;
        
        // Call AI with tool to generate world population content
        let response = self.anthropic.generate_with_tool(&prompt, tool, phase.max_tokens, phase.temperature).await?;
        
        // Save world population content using GraphQL
        self.save_world_population_content_graphql(campaign_id, &response).await?;

        Ok(())
    }

    // AI Content Generation
    async fn generate_ai_content(&self, prompt: &str, max_tokens: u32, temperature: f32) -> ApiResult<JsonValue> {
        info!("Calling AI with {} tokens max, temperature {}", max_tokens, temperature);
        
        // Call the actual Anthropic API
        let response = self.anthropic.generate_content(prompt, max_tokens, temperature).await?;
        
        // Parse the response as JSON
        match serde_json::from_str::<JsonValue>(&response) {
            Ok(json) => Ok(json),
            Err(e) => {
                warn!("Failed to parse AI response as JSON: {}", e);
                warn!("First 200 chars of response: {}", &response.chars().take(200).collect::<String>());
                
                // Determine which phase this is based on the prompt content
                if prompt.contains("world building content") {
                    // Phase 1: World Building fallback
                    Ok(json!({
                    "calendar_systems": [{
                        "name": "Default Calendar",
                        "months": [
                            {"name": "First", "days": 30},
                            {"name": "Second", "days": 30},
                            {"name": "Third", "days": 30},
                            {"name": "Fourth", "days": 30},
                            {"name": "Fifth", "days": 30},
                            {"name": "Sixth", "days": 30},
                            {"name": "Seventh", "days": 30},
                            {"name": "Eighth", "days": 30},
                            {"name": "Ninth", "days": 30},
                            {"name": "Tenth", "days": 30},
                            {"name": "Eleventh", "days": 30},
                            {"name": "Twelfth", "days": 30}
                        ],
                        "weekdays": ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"],
                        "year_length": 360,
                        "current_year": 1247,
                        "current_month": 1,
                        "current_day": 1,
                        "special_events": ["Spring Festival", "Harvest Moon", "Winter Solstice"],
                        "lunar_cycles": {"main_moon": {"name": "Luna", "cycle_days": 28}}
                    }],
                    "world_history": [{
                        "name": "The Current Age",
                        "start_year": 0,
                        "end_year": 1247,
                        "description": "The modern era",
                        "major_events": ["The kingdom founding", "The great war", "The peace treaty"]
                    }],
                    "planes": [{
                        "name": "Material Plane",
                        "type": "Primary",
                        "description": "The main world",
                        "properties": {"gravity": "normal", "time": "normal", "magic": "normal"},
                        "access_methods": ["Native"]
                    }],
                    "pantheons": [{
                        "name": "The Gods",
                        "description": "Divine beings",
                        "alignment": "Various",
                        "domains": ["Life", "Death", "War", "Peace"]
                    }],
                    "deities": [{
                        "name": "The Creator",
                        "alignment": "Neutral",
                        "domains": ["Creation", "Knowledge"],
                        "symbols": ["Circle", "Book"],
                        "description": "The first deity"
                    }],
                    "geography_regions": [{
                        "name": "Central Lands",
                        "type": "Mixed",
                        "description": "The main continent",
                        "climate": "Temperate",
                        "terrain_features": ["Plains", "Forests", "Mountains"]
                    }],
                    "economic_systems": [{
                        "name": "Standard Economy",
                        "type": "Market",
                        "description": "Traditional fantasy economy",
                        "currency_types": ["Gold", "Silver", "Copper"],
                        "trade_goods": ["Food", "Weapons", "Magic items"]
                    }],
                    "legal_systems": [{
                        "name": "Kingdom Law",
                        "type": "Monarchic",
                        "description": "Laws of the land",
                        "laws": ["No murder", "No theft", "Pay taxes"],
                        "enforcement_methods": ["Guards", "Courts", "Prisons"]
                    }],
                    "astronomy": [{
                        "name": "Standard Sky",
                        "description": "Sun, moon, and stars",
                        "celestial_bodies": ["Sun", "Moon", "Stars"],
                        "constellations": ["Bear", "Dragon", "Crown"],
                        "calendar_influences": {"month_length": "lunar", "seasons": "solar"}
                    }]
                }))
                } else if prompt.contains("PC-connected content") {
                    // Phase 2: PC Connected fallback
                    Ok(json!({
                        "entities": [{
                            "name": "Test Mentor",
                            "entity_type": "npc",
                            "description": "A wise mentor figure",
                            "connection_to_pc": "Test Character - Former teacher",
                            "metadata": {
                                "personality": "Wise and patient",
                                "goals": "Guide the next generation",
                                "secrets": "Knows about the time prophecy"
                            }
                        }],
                        "pc_backstory_npcs": [{
                            "name": "Eldara the Chronicler",
                            "connected_pc": "Test Character",
                            "relationship": "Former Master",
                            "description": "An elderly wizard who taught the PC about time magic",
                            "current_location": "Tower of Hours",
                            "plot_hooks": ["Missing research notes", "Strange time anomalies"]
                        }],
                        "pc_home_locations": [{
                            "name": "Chronos Academy",
                            "connected_pc": "Test Character",
                            "type": "school",
                            "description": "A prestigious academy for temporal studies",
                            "notable_features": ["Great Library of Hours", "Temporal Observatory"],
                            "current_events": ["Mysterious disappearances", "Time distortions"]
                        }],
                        "pc_related_items": [{
                            "name": "Student's Hourglass",
                            "connected_pc": "Test Character",
                            "item_type": "wondrous item",
                            "description": "A small hourglass given to promising students",
                            "magical": true,
                            "properties": {
                                "rarity": "uncommon",
                                "value": "100gp",
                                "special_abilities": ["Can detect temporal anomalies"]
                            }
                        }],
                        "pc_factions": [{
                            "name": "The Timekeepers Guild",
                            "connected_pcs": ["Test Character"],
                            "type": "guild",
                            "description": "Scholars dedicated to studying and protecting the timestream",
                            "goals": ["Preserve temporal integrity", "Study time magic"],
                            "notable_members": ["Archchronarch Valdris", "Keeper Theron"],
                            "headquarters": "The Eternal Citadel"
                        }]
                    }))
                } else if prompt.contains("world population content") {
                    // Phase 3: World Population fallback
                    Ok(json!({
                        "additional_npcs": [{
                            "name": "Bren the Barkeep",
                            "occupation": "Tavern Owner",
                            "location": "The Sundial Inn",
                            "description": "A jovial halfling who runs the local tavern",
                            "personality": "Friendly and gossipy",
                            "plot_hooks": ["Knows everyone in town", "Heard rumors about time distortions"]
                        }],
                        "locations": [{
                            "name": "The Sundial Inn",
                            "type": "tavern",
                            "description": "A cozy tavern with a large sundial in the courtyard",
                            "notable_features": ["Famous ale", "Sundial courtyard", "Secret meeting room"],
                            "inhabitants": ["Bren the Barkeep", "Local patrons"],
                            "secrets": ["Hidden basement", "Smuggling operation"]
                        }],
                        "creatures": [{
                            "name": "Time Wisp",
                            "type": "aberration",
                            "habitat": "Areas of temporal instability",
                            "description": "Ethereal beings that feed on temporal energy",
                            "behavior": "Curious but dangerous",
                            "challenge_rating": "2"
                        }],
                        "flora": [{
                            "name": "Chronos Bloom",
                            "type": "flower",
                            "habitat": "Near temporal rifts",
                            "description": "Flowers that bloom and wilt in accelerated cycles",
                            "properties": {
                                "medicinal": true,
                                "magical": true,
                                "uses": ["Potion ingredient", "Time magic focus"]
                            }
                        }],
                        "quest_hooks": [{
                            "title": "The Missing Hour",
                            "description": "People are losing an hour of memory each night",
                            "quest_giver": "Town Mayor",
                            "location": "Timekeeper Village",
                            "rewards": ["500gp", "Temporal ward charm"],
                            "difficulty": "medium",
                            "connected_npcs": ["Eldara the Chronicler", "Mayor Aldric"],
                            "objectives": ["Investigate memory loss", "Find the source", "Stop the phenomenon"]
                        }],
                        "organizations": [{
                            "name": "The Sundial Merchants",
                            "type": "merchant",
                            "description": "Local merchant guild dealing in temporal artifacts",
                            "goals": ["Control artifact trade", "Profit from time magic"],
                            "notable_members": ["Guildmaster Vex", "Treasurer Mira"],
                            "resources": "Moderate wealth and connections",
                            "influence": "local"
                        }],
                        "rumors": [{
                            "text": "The old clock tower chimes at impossible hours",
                            "source": "Tavern gossip",
                            "truth_level": "true",
                            "related_to": "The Missing Hour quest"
                        }]
                    }))
                } else {
                    // Generic fallback
                    Err(ApiError::BadRequest("Failed to parse AI response and could not determine phase".to_string()))
                }
            }
        }
    }

    // Phase 1: Save World Building Content
    async fn save_world_building_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving world building content for campaign {}", campaign_id);
        
        let mut tx = self.database.begin_transaction().await?;

        // Save calendar systems
        if let Some(calendars) = content.get("calendar_systems").and_then(|v| v.as_array()) {
            for calendar in calendars {
                self.database.save_calendar_system(&mut tx, campaign_id, calendar).await?;
            }
        }

        // Save world history periods
        if let Some(history) = content.get("world_history").and_then(|v| v.as_array()) {
            for period in history {
                self.database.save_world_history_period(&mut tx, campaign_id, period).await?;
            }
        }

        // Save planes
        if let Some(planes) = content.get("planes").and_then(|v| v.as_array()) {
            for plane in planes {
                self.database.save_plane(&mut tx, campaign_id, plane).await?;
            }
        }

        // Save pantheons and deities
        if let Some(pantheons) = content.get("pantheons").and_then(|v| v.as_array()) {
            for pantheon in pantheons {
                let pantheon_id = self.database.save_pantheon(&mut tx, campaign_id, pantheon).await?;
                
                // Save deities for this pantheon
                if let Some(deities) = content.get("deities").and_then(|v| v.as_array()) {
                    for deity in deities {
                        self.database.save_deity(&mut tx, campaign_id, Some(pantheon_id), deity).await?;
                    }
                }
            }
        }

        // Save geography regions
        if let Some(regions) = content.get("geography_regions").and_then(|v| v.as_array()) {
            for region in regions {
                self.database.save_geography_region(&mut tx, campaign_id, region).await?;
            }
        }

        // Save economic systems
        if let Some(economy) = content.get("economic_systems").and_then(|v| v.as_array()).and_then(|arr| arr.first()) {
            self.database.save_economic_system(&mut tx, campaign_id, economy).await?;
        }

        // Save legal systems  
        if let Some(legal) = content.get("legal_systems").and_then(|v| v.as_array()).and_then(|arr| arr.first()) {
            self.database.save_legal_system(&mut tx, campaign_id, legal).await?;
        }

        // Save astronomy and zodiac signs
        if let Some(astronomy) = content.get("astronomy").and_then(|v| v.as_array()).and_then(|arr| arr.first()) {
            let astronomy_id = self.database.save_astronomy(&mut tx, campaign_id, astronomy).await?;
            
            // Save zodiac signs if present
            if let Some(zodiac_signs) = content.get("zodiac_signs").and_then(|v| v.as_array()) {
                for zodiac in zodiac_signs {
                    self.database.save_zodiac_sign(&mut tx, campaign_id, astronomy_id, zodiac).await?;
                }
            }
        }

        // Save languages
        if let Some(languages) = content.get("languages").and_then(|v| v.as_array()) {
            for language in languages {
                self.database.save_language(&mut tx, campaign_id, language).await?;
            }
        } else {
            // Save default Common language
            let common_language = json!({
                "name": "Common",
                "type": "Standard",
                "description": "The trade tongue",
                "script": "Common",
                "speakers": ["Humans", "Most civilized races"]
            });
            self.database.save_language(&mut tx, campaign_id, &common_language).await?;
        }

        // Save races and subraces
        if let Some(races) = content.get("races").and_then(|v| v.as_array()) {
            for race in races {
                let race_id = self.database.save_race(&mut tx, campaign_id, race).await?;
                
                // Save subraces if present
                if let Some(subraces) = race.get("subraces").and_then(|v| v.as_array()) {
                    for subrace in subraces {
                        self.database.save_subrace(&mut tx, campaign_id, race_id, subrace).await?;
                    }
                }
            }
        } else {
            // Save default Human race
            let human_race = json!({
                "name": "Human",
                "description": "Versatile and ambitious",
                "traits": ["Extra skill", "Extra feat"],
                "ability_modifiers": {},
                "size": "Medium", 
                "speed": 30
            });
            self.database.save_race(&mut tx, campaign_id, &human_race).await?;
        }

        // Save classes and subclasses
        if let Some(classes) = content.get("classes").and_then(|v| v.as_array()) {
            for class in classes {
                let class_id = self.database.save_class(&mut tx, campaign_id, class).await?;
                
                // Save subclasses if present
                if let Some(subclasses) = class.get("subclasses").and_then(|v| v.as_array()) {
                    for subclass in subclasses {
                        self.database.save_subclass(&mut tx, campaign_id, class_id, subclass).await?;
                    }
                }
            }
        } else {
            // Save default Fighter class
            let fighter_class = json!({
                "name": "Fighter",
                "description": "Master of martial combat",
                "hit_die": 10,
                "primary_abilities": ["Strength", "Dexterity"],
                "saving_throws": ["Strength", "Constitution"]
            });
            self.database.save_class(&mut tx, campaign_id, &fighter_class).await?;
        }
        
        // Save backgrounds
        if let Some(backgrounds) = content.get("backgrounds").and_then(|v| v.as_array()) {
            for background in backgrounds {
                self.database.save_background(&mut tx, campaign_id, background).await?;
            }
        }
        
        // Save cultures
        if let Some(cultures) = content.get("cultures").and_then(|v| v.as_array()) {
            for culture in cultures {
                self.database.save_culture(&mut tx, campaign_id, culture).await?;
            }
        }
        
        // Save feats
        if let Some(feats) = content.get("feats").and_then(|v| v.as_array()) {
            for feat in feats {
                self.database.save_feat(&mut tx, campaign_id, feat).await?;
            }
        }
        
        // Save spells
        if let Some(spells) = content.get("spells").and_then(|v| v.as_array()) {
            for spell in spells {
                self.database.save_spell(&mut tx, campaign_id, spell).await?;
            }
        }

        // Save metadata
        self.database.update_campaign_metadata(&mut tx, campaign_id, "world_building", content).await?;

        tx.commit().await?;
        info!("Successfully saved world building content for campaign {}", campaign_id);

        Ok(())
    }

    // Phase 2: Save PC Connected Content
    async fn save_pc_connected_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving PC connected content for campaign {}", campaign_id);
        
        let mut tx = self.database.begin_transaction().await?;
        let mut entity_mapping = HashMap::new();
        let mut location_mapping = HashMap::new();

        // First, get campaign to access player characters
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        // Create entities for player characters if not already done
        if let Some(pcs) = campaign.player_characters.as_array() {
            for pc in pcs {
                let pc_name = pc.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown PC");
                let entity_data = json!({
                    "name": pc_name,
                    "description": pc.get("backstory").and_then(|v| v.as_str()).unwrap_or(""),
                    "metadata": pc
                });
                
                let entity_id = self.database.save_entity(&mut tx, campaign_id, "pc", &entity_data).await?;
                entity_mapping.insert(pc_name.to_string(), entity_id);
                
                // Save as player character
                self.database.save_player_character(&mut tx, campaign_id, entity_id, pc).await?;
            }
        }

        // Process entities from Phase 2 content
        if let Some(entities) = content.get("entities").and_then(|v| v.as_array()) {
            for entity in entities {
                let entity_type = entity.get("entity_type").and_then(|v| v.as_str()).unwrap_or("npc");
                let name = entity.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                
                let entity_id = self.database.save_entity(&mut tx, campaign_id, entity_type, entity).await?;
                entity_mapping.insert(name.to_string(), entity_id);
                
                // If it's an NPC, save to non_player_characters table
                if entity_type == "npc" {
                    self.database.save_non_player_character(&mut tx, entity_id, entity).await?;
                }
                
                // Create entity relationships based on connection_to_pc
                if let Some(connection) = entity.get("connection_to_pc").and_then(|v| v.as_str()) {
                    // Parse connection string to find PC name
                    let parts: Vec<&str> = connection.split(" - ").collect();
                    if let Some(pc_name) = parts.first() {
                        if let Some(&pc_entity_id) = entity_mapping.get(*pc_name) {
                            let relationship = json!({
                                "relationship_type": "connected",
                                "description": connection
                            });
                            self.database.save_entity_relationship(&mut tx, pc_entity_id, entity_id, &relationship).await?;
                        }
                    }
                }
            }
        }

        // Process PC backstory NPCs
        if let Some(backstory_npcs) = content.get("pc_backstory_npcs").and_then(|v| v.as_array()) {
            for npc in backstory_npcs {
                let name = npc.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown NPC");
                
                // Create entity
                let entity_data = json!({
                    "name": name,
                    "description": npc.get("description").and_then(|v| v.as_str()).unwrap_or(""),
                    "metadata": npc
                });
                
                let entity_id = self.database.save_entity(&mut tx, campaign_id, "npc", &entity_data).await?;
                entity_mapping.insert(name.to_string(), entity_id);
                
                // Save as NPC
                self.database.save_non_player_character(&mut tx, entity_id, npc).await?;
                
                // Create relationship to PC
                if let Some(connected_pc) = npc.get("connected_pc").and_then(|v| v.as_str()) {
                    if let Some(&pc_entity_id) = entity_mapping.get(connected_pc) {
                        let relationship = json!({
                            "relationship_type": npc.get("relationship").and_then(|v| v.as_str()).unwrap_or("knows"),
                            "description": npc.get("relationship").and_then(|v| v.as_str()).unwrap_or("")
                        });
                        self.database.save_entity_relationship(&mut tx, pc_entity_id, entity_id, &relationship).await?;
                    }
                }
            }
        }

        // Process PC home locations
        if let Some(home_locations) = content.get("pc_home_locations").and_then(|v| v.as_array()) {
            for location in home_locations {
                let name = location.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Location");
                
                let location_id = self.database.save_location(&mut tx, campaign_id, location).await?;
                location_mapping.insert(name.to_string(), location_id);
                
                // Create connection to PC
                if let Some(connected_pc) = location.get("connected_pc").and_then(|v| v.as_str()) {
                    if let Some(&pc_entity_id) = entity_mapping.get(connected_pc) {
                        let entity_location = json!({
                            "relationship_type": "home",
                            "notes": "PC's home location"
                        });
                        self.database.save_entity_location(&mut tx, pc_entity_id, location_id, &entity_location).await?;
                    }
                }
            }
        }

        // Process PC related items
        if let Some(related_items) = content.get("pc_related_items").and_then(|v| v.as_array()) {
            for item in related_items {
                let item_id = self.database.save_item(&mut tx, campaign_id, item).await?;
                
                // Create connection to PC
                if let Some(connected_pc) = item.get("connected_pc").and_then(|v| v.as_str()) {
                    if let Some(&pc_entity_id) = entity_mapping.get(connected_pc) {
                        self.database.save_entity_item(&mut tx, pc_entity_id, item_id).await?;
                    }
                }
            }
        }

        // Process PC factions
        if let Some(factions) = content.get("pc_factions").and_then(|v| v.as_array()) {
            for faction in factions {
                let faction_id = self.database.save_faction(&mut tx, campaign_id, faction).await?;
                
                // Create connections to PCs
                if let Some(connected_pcs) = faction.get("connected_pcs").and_then(|v| v.as_array()) {
                    for pc_name in connected_pcs {
                        if let Some(pc_name_str) = pc_name.as_str() {
                            if let Some(&pc_entity_id) = entity_mapping.get(pc_name_str) {
                                let membership = json!({
                                    "rank": "member",
                                    "join_date": "campaign start"
                                });
                                self.database.save_entity_faction(&mut tx, pc_entity_id, faction_id, &membership).await?;
                            }
                        }
                    }
                }
            }
        }

        // Save metadata
        self.database.update_campaign_metadata(&mut tx, campaign_id, "pc_connected", content).await?;

        tx.commit().await?;
        info!("Successfully saved PC connected content for campaign {}", campaign_id);

        Ok(())
    }

    // Phase 3: Save World Population Content
    async fn save_world_population_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving world population content for campaign {}", campaign_id);
        
        let mut tx = self.database.begin_transaction().await?;
        let mut entity_mapping = HashMap::new();
        let mut location_mapping = HashMap::new();
        let mut faction_mapping = HashMap::new();
        
        // Get existing entities from previous phases
        let existing_entities = self.database.get_campaign_entities(&mut tx, campaign_id).await?;
        for entity in existing_entities {
            if let Some(name) = entity.get("name").and_then(|v| v.as_str()) {
                if let Some(id) = entity.get("id").and_then(|v| v.as_i64()) {
                    entity_mapping.insert(name.to_string(), id as i32);
                }
            }
        }

        // Process additional NPCs
        if let Some(additional_npcs) = content.get("additional_npcs").and_then(|v| v.as_array()) {
            for npc in additional_npcs {
                let name = npc.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown NPC");
                
                // Create entity
                let entity_data = json!({
                    "name": name,
                    "description": npc.get("description").and_then(|v| v.as_str()).unwrap_or(""),
                    "metadata": npc
                });
                
                let entity_id = self.database.save_entity(&mut tx, campaign_id, "npc", &entity_data).await?;
                entity_mapping.insert(name.to_string(), entity_id);
                
                // Save as NPC
                let npc_data = json!({
                    "role": npc.get("occupation").and_then(|v| v.as_str()).unwrap_or("Citizen"),
                    "personality": npc.get("personality"),
                    "secret_info": npc.get("secrets").and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|v| v.as_str()).unwrap_or(""),
                    "stats": {}
                });
                self.database.save_non_player_character(&mut tx, entity_id, &npc_data).await?;
            }
        }

        // Process locations
        if let Some(locations) = content.get("locations").and_then(|v| v.as_array()) {
            for location in locations {
                let name = location.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Location");
                
                let location_id = self.database.save_location(&mut tx, campaign_id, location).await?;
                location_mapping.insert(name.to_string(), location_id);
                
                // Create entity-location relationships for inhabitants
                if let Some(inhabitants) = location.get("inhabitants").and_then(|v| v.as_array()) {
                    for inhabitant in inhabitants {
                        if let Some(inhabitant_name) = inhabitant.as_str() {
                            if let Some(&entity_id) = entity_mapping.get(inhabitant_name) {
                                let entity_location = json!({
                                    "relationship_type": "resides",
                                    "notes": "Lives or works here"
                                });
                                self.database.save_entity_location(&mut tx, entity_id, location_id, &entity_location).await?;
                            }
                        }
                    }
                }
                
                // Handle specific location types
                let location_type = location.get("type").and_then(|v| v.as_str()).unwrap_or("settlement");
                match location_type {
                    "shop" => { self.database.save_shop(&mut tx, location_id, location).await?; },
                    "tavern" => { self.database.save_tavern(&mut tx, location_id, location).await?; },
                    "temple" => { self.database.save_temple(&mut tx, location_id, location).await?; },
                    "dungeon" => { self.database.save_dungeon(&mut tx, location_id, location).await?; },
                    _ => {}
                };
            }
        }

        // Process shops (new separate array from tool-based generation)
        if let Some(shops) = content.get("shops").and_then(|v| v.as_array()) {
            for shop in shops {
                let shop_name = shop.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Shop");
                let location_name = shop.get("location").and_then(|v| v.as_str()).unwrap_or("");
                
                // Find or create the location
                let location_id = if let Some(&id) = location_mapping.get(location_name) {
                    id
                } else {
                    // Create a new location for this shop
                    let location_data = json!({
                        "name": location_name,
                        "type": "settlement",
                        "description": format!("Location of {}", shop_name)
                    });
                    let new_id = self.database.save_location(&mut tx, campaign_id, &location_data).await?;
                    location_mapping.insert(location_name.to_string(), new_id);
                    new_id
                };
                
                self.database.save_shop(&mut tx, location_id, shop).await?;
            }
        }

        // Process taverns (new separate array from tool-based generation)
        if let Some(taverns) = content.get("taverns").and_then(|v| v.as_array()) {
            for tavern in taverns {
                let tavern_name = tavern.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Tavern");
                let location_name = tavern.get("location").and_then(|v| v.as_str()).unwrap_or("");
                
                // Find or create the location
                let location_id = if let Some(&id) = location_mapping.get(location_name) {
                    id
                } else {
                    // Create a new location for this tavern
                    let location_data = json!({
                        "name": location_name,
                        "type": "settlement",
                        "description": format!("Location of {}", tavern_name)
                    });
                    let new_id = self.database.save_location(&mut tx, campaign_id, &location_data).await?;
                    location_mapping.insert(location_name.to_string(), new_id);
                    new_id
                };
                
                self.database.save_tavern(&mut tx, location_id, tavern).await?;
            }
        }

        // Process temples (new separate array from tool-based generation)
        if let Some(temples) = content.get("temples").and_then(|v| v.as_array()) {
            for temple in temples {
                let temple_name = temple.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Temple");
                let location_name = temple.get("location").and_then(|v| v.as_str()).unwrap_or("");
                
                // Find or create the location
                let location_id = if let Some(&id) = location_mapping.get(location_name) {
                    id
                } else {
                    // Create a new location for this temple
                    let location_data = json!({
                        "name": location_name,
                        "type": "settlement",
                        "description": format!("Location of {}", temple_name)
                    });
                    let new_id = self.database.save_location(&mut tx, campaign_id, &location_data).await?;
                    location_mapping.insert(location_name.to_string(), new_id);
                    new_id
                };
                
                self.database.save_temple(&mut tx, location_id, temple).await?;
            }
        }

        // Process magic items (new separate array from tool-based generation)
        if let Some(magic_items) = content.get("magic_items").and_then(|v| v.as_array()) {
            for magic_item in magic_items {
                self.database.save_magic_item(&mut tx, campaign_id, magic_item).await?;
            }
        }

        // Process sentient items (new separate array from tool-based generation)
        if let Some(sentient_items) = content.get("sentient_items").and_then(|v| v.as_array()) {
            for sentient_item in sentient_items {
                // Create sentient magic item (magic_items table handles sentience internally)
                self.database.save_magic_item(&mut tx, campaign_id, sentient_item).await?;
            }
        }

        // Process encounters (new separate array from tool-based generation)
        if let Some(encounters) = content.get("encounters").and_then(|v| v.as_array()) {
            for encounter in encounters {
                let location_name = encounter.get("location").and_then(|v| v.as_str()).unwrap_or("");
                let location_id = location_mapping.get(location_name).copied().unwrap_or(0);
                self.database.save_encounter(&mut tx, campaign_id, location_id, encounter).await?;
            }
        }

        // Process faction relationships (new from tool-based generation)
        if let Some(faction_relationships) = content.get("faction_relationships").and_then(|v| v.as_array()) {
            for relationship in faction_relationships {
                let faction_a_name = relationship.get("faction_a").and_then(|v| v.as_str()).unwrap_or("");
                let faction_b_name = relationship.get("faction_b").and_then(|v| v.as_str()).unwrap_or("");
                
                if let (Some(&faction_a_id), Some(&faction_b_id)) = (
                    faction_mapping.get(faction_a_name),
                    faction_mapping.get(faction_b_name)
                ) {
                    self.database.save_faction_relationship(&mut tx, faction_a_id, faction_b_id, relationship).await?;
                }
            }
        }

        // Process entity relationships (new from tool-based generation)
        if let Some(entity_relationships) = content.get("entity_relationships").and_then(|v| v.as_array()) {
            for relationship in entity_relationships {
                let entity_a_name = relationship.get("entity_a").and_then(|v| v.as_str()).unwrap_or("");
                let entity_b_name = relationship.get("entity_b").and_then(|v| v.as_str()).unwrap_or("");
                
                if let (Some(&entity_a_id), Some(&entity_b_id)) = (
                    entity_mapping.get(entity_a_name),
                    entity_mapping.get(entity_b_name)
                ) {
                    self.database.save_entity_relationship(&mut tx, entity_a_id, entity_b_id, relationship).await?;
                }
            }
        }

        // Process creatures
        if let Some(creatures) = content.get("creatures").and_then(|v| v.as_array()) {
            for creature in creatures {
                let name = creature.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Creature");
                
                // Create entity
                let entity_data = json!({
                    "name": name,
                    "description": creature.get("description").and_then(|v| v.as_str()).unwrap_or(""),
                    "metadata": creature
                });
                
                let entity_id = self.database.save_entity(&mut tx, campaign_id, "creature", &entity_data).await?;
                entity_mapping.insert(name.to_string(), entity_id);
                
                // Save as creature
                self.database.save_creature(&mut tx, entity_id, creature).await?;
            }
        }

        // Process flora
        if let Some(flora) = content.get("flora").and_then(|v| v.as_array()) {
            for plant in flora {
                let name = plant.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Plant");
                
                // Create entity
                let entity_data = json!({
                    "name": name,
                    "description": plant.get("description").and_then(|v| v.as_str()).unwrap_or(""),
                    "metadata": plant
                });
                
                let entity_id = self.database.save_entity(&mut tx, campaign_id, "flora", &entity_data).await?;
                entity_mapping.insert(name.to_string(), entity_id);
                
                // Save as flora
                self.database.save_flora(&mut tx, entity_id, plant).await?;
            }
        }

        // Process organizations/factions
        if let Some(organizations) = content.get("organizations").and_then(|v| v.as_array()) {
            for org in organizations {
                let name = org.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Organization");
                let faction_id = self.database.save_faction(&mut tx, campaign_id, org).await?;
                faction_mapping.insert(name.to_string(), faction_id);
                
                // Create connections to notable members
                if let Some(members) = org.get("notable_members").and_then(|v| v.as_array()) {
                    for member in members {
                        if let Some(member_name) = member.as_str() {
                            // Try to find or create the member entity
                            let entity_id = if let Some(&id) = entity_mapping.get(member_name) {
                                id
                            } else {
                                // Create a new NPC entity for this member
                                let entity_data = json!({
                                    "name": member_name,
                                    "description": format!("Member of {}", name),
                                    "metadata": {"faction": name}
                                });
                                let new_id = self.database.save_entity(&mut tx, campaign_id, "npc", &entity_data).await?;
                                entity_mapping.insert(member_name.to_string(), new_id);
                                new_id
                            };
                            
                            let membership = json!({
                                "rank": "notable member",
                                "join_date": "unknown"
                            });
                            self.database.save_entity_faction(&mut tx, entity_id, faction_id, &membership).await?;
                        }
                    }
                }
            }
        }

        // Process quest hooks
        if let Some(quest_hooks) = content.get("quest_hooks").and_then(|v| v.as_array()) {
            for quest in quest_hooks {
                // Save quest (note: quest table doesn't exist, so this is a no-op with warning)
                self.database.save_quest(&mut tx, campaign_id, quest, &entity_mapping, &location_mapping).await?;
            }
        }

        // Process rumors (as lore entries)
        if let Some(rumors) = content.get("rumors").and_then(|v| v.as_array()) {
            for rumor in rumors {
                let lore_data = json!({
                    "title": "Rumor",
                    "content": rumor.get("text").and_then(|v| v.as_str()).unwrap_or(""),
                    "category": "rumor",
                    "source": rumor.get("source").and_then(|v| v.as_str()).unwrap_or("unknown"),
                    "truth_level": rumor.get("truth_level").and_then(|v| v.as_str()).unwrap_or("unknown"),
                    "related_to": rumor.get("related_to").and_then(|v| v.as_str()).unwrap_or("")
                });
                self.database.save_lore_entry(&mut tx, campaign_id, &lore_data).await?;
            }
        }

        // Note: Removed backward compatibility saves to old tables

        // Save metadata
        self.database.update_campaign_metadata(&mut tx, campaign_id, "world_population", content).await?;

        tx.commit().await?;
        info!("Successfully saved world population content for campaign {}", campaign_id);

        Ok(())
    }

    // Prompt Building Methods
    async fn build_world_building_prompt(&self, campaign_id: i32) -> ApiResult<String> {
        // Get campaign details
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        let mut prompt = format!(
            "You are creating a detailed D&D 5e campaign world for: '{}'\n\n",
            campaign.name
        );
        
        if let Some(setting) = &campaign.setting {
            prompt.push_str(&format!("Setting: {}\n", setting));
        }
        
        prompt.push_str(&format!("Themes: {}\n", campaign.themes.join(", ")));
        prompt.push_str(&format!("Tone: {}\n", campaign.tone));
        prompt.push_str(&format!("Campaign Length: {}\n\n", campaign.campaign_length));
        
        if let Some(notes) = &campaign.additional_notes {
            prompt.push_str(&format!("Additional Context: {}\n\n", notes));
        }
        
        // Add metadata context if available
        if let Some(world_building) = campaign.metadata.get("world_building") {
            prompt.push_str(&format!("World Building Preferences: {}\n\n", world_building));
        }
        
        prompt.push_str(
            "Generate comprehensive world building content for this D&D campaign.\n\n\
            IMPORTANT: Return ONLY a valid JSON object. Do not include any text before or after the JSON.\n\
            Do not include markdown code blocks or any other formatting.\n\
            Start your response with { and end with }\n\n\
            Required JSON structure:\n\
            {\n\
              \"calendar_systems\": [{\n\
                \"name\": \"string\",\n\
                \"months\": [{\"name\": \"string\", \"days\": number}],\n\
                \"weekdays\": [\"array of weekday names\"],\n\
                \"year_length\": number,\n\
                \"current_year\": number,\n\
                \"current_month\": number,\n\
                \"current_day\": number,\n\
                \"special_events\": [\"array of special event descriptions\"],\n\
                \"lunar_cycles\": {\"moon_name\": {\"name\": \"string\", \"cycle_days\": number}}\n\
              }],\n\
              \"world_history\": [{\n\
                \"name\": \"string (era name)\",\n\
                \"start_year\": number,\n\
                \"end_year\": number,\n\
                \"description\": \"string\",\n\
                \"major_events\": [\"array of event descriptions\"]\n\
              }],\n\
              \"planes\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (Material/Elemental/Transitive/etc)\",\n\
                \"description\": \"string\",\n\
                \"properties\": {\"gravity\": \"string\", \"time\": \"string\", \"magic\": \"string\"},\n\
                \"access_methods\": [\"array of ways to reach this plane\"]\n\
              }],\n\
              \"pantheons\": [{\n\
                \"name\": \"string\",\n\
                \"description\": \"string\",\n\
                \"alignment\": \"string\",\n\
                \"domains\": [\"array of divine domains\"]\n\
              }],\n\
              \"deities\": [{\n\
                \"name\": \"string\",\n\
                \"alignment\": \"string (e.g. Lawful Good)\",\n\
                \"domains\": [\"array of domains\"],\n\
                \"symbols\": [\"array of holy symbols\"],\n\
                \"description\": \"string\"\n\
              }],\n\
              \"geography_regions\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (Mountains/Forest/Desert/etc)\",\n\
                \"description\": \"string\",\n\
                \"climate\": \"string\",\n\
                \"terrain_features\": [\"array of notable features\"]\n\
              }],\n\
              \"economic_systems\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (Barter/Market/Command/etc)\",\n\
                \"description\": \"string\",\n\
                \"currency_types\": [\"array of currency names\"],\n\
                \"trade_goods\": [\"array of major trade items\"]\n\
              }],\n\
              \"legal_systems\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (Monarchic/Democratic/Theocratic/etc)\",\n\
                \"description\": \"string\",\n\
                \"laws\": [\"array of major laws\"],\n\
                \"enforcement_methods\": [\"array of enforcement types\"]\n\
              }],\n\
              \"astronomy\": [{\n\
                \"name\": \"string\",\n\
                \"description\": \"string\",\n\
                \"celestial_bodies\": [\"array of celestial objects\"],\n\
                \"constellations\": [\"array of constellation names\"],\n\
                \"calendar_influences\": {\"month_length\": \"string\", \"seasons\": \"string\"}\n\
              }],\n\
              \"zodiac_signs\": [{\n\
                \"name\": \"string\",\n\
                \"symbol\": \"string\",\n\
                \"time_period\": \"string (e.g. 'First Month, Days 1-30')\",\n\
                \"traits\": [\"array of personality traits\"],\n\
                \"element\": \"string\",\n\
                \"description\": \"string\"\n\
              }],\n\
              \"languages\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (Standard/Exotic/Secret)\",\n\
                \"description\": \"string\",\n\
                \"script\": \"string\",\n\
                \"speakers\": [\"array of who speaks this language\"]\n\
              }],\n\
              \"races\": [{\n\
                \"name\": \"string\",\n\
                \"description\": \"string\",\n\
                \"size\": \"string\",\n\
                \"speed\": number,\n\
                \"traits\": [\"array of racial traits\"],\n\
                \"ability_modifiers\": {\"strength\": number, \"dexterity\": number, etc.},\n\
                \"subraces\": [{\"name\": \"string\", \"description\": \"string\", \"additional_traits\": [\"array\"]}]\n\
              }],\n\
              \"classes\": [{\n\
                \"name\": \"string\",\n\
                \"description\": \"string\",\n\
                \"hit_die\": number,\n\
                \"primary_abilities\": [\"array of ability names\"],\n\
                \"saving_throws\": [\"array of ability names\"],\n\
                \"subclasses\": [{\"name\": \"string\", \"description\": \"string\", \"features\": [\"array\"]}]\n\
              }],\n\
              \"backgrounds\": [{\n\
                \"name\": \"string\",\n\
                \"description\": \"string\",\n\
                \"skill_proficiencies\": [\"array of skills\"],\n\
                \"languages\": [\"array of language choices\"],\n\
                \"equipment\": [\"array of starting equipment\"],\n\
                \"feature\": {\"name\": \"string\", \"description\": \"string\"}\n\
              }],\n\
              \"cultures\": [{\n\
                \"name\": \"string\",\n\
                \"description\": \"string\",\n\
                \"values\": [\"array of cultural values\"],\n\
                \"traditions\": [\"array of traditions\"],\n\
                \"common_names\": [\"array of example names\"],\n\
                \"languages\": [\"array of commonly spoken languages\"]\n\
              }],\n\
              \"feats\": [{\n\
                \"name\": \"string\",\n\
                \"description\": \"string\",\n\
                \"prerequisites\": {\"level\": number, \"abilities\": {}, \"other\": \"string\"},\n\
                \"benefits\": {\"description\": \"string\", \"mechanics\": [\"array\"]}\n\
              }],\n\
              \"spells\": [{\n\
                \"name\": \"string\",\n\
                \"level\": number,\n\
                \"school\": \"string (e.g. Evocation, Necromancy)\",\n\
                \"casting_time\": \"string\",\n\
                \"range\": \"string\",\n\
                \"components\": {\"verbal\": boolean, \"somatic\": boolean, \"material\": \"string\"},\n\
                \"duration\": \"string\",\n\
                \"description\": \"string\",\n\
                \"higher_levels\": \"string (optional)\"\n\
              }]\n\
            }\n\n\
            Create rich, interconnected content that reflects the campaign's themes and setting.\
            Include at least 2-3 entries for each category to populate the world.\
            Ensure all content is appropriate for the tone and themes specified.\
            Make the world feel lived-in and believable within a fantasy context."
        );
        
        Ok(prompt)
    }

    async fn build_pc_connected_prompt(&self, campaign_id: i32, world_data: &JsonValue) -> ApiResult<String> {
        // Get campaign details
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        let mut prompt = format!(
            "You are creating PC-connected content for the D&D 5e campaign: '{}'\n\n",
            campaign.name
        );
        
        // Add player character details
        prompt.push_str("Player Characters:\n");
        if let Some(pcs) = campaign.player_characters.as_array() {
            for pc in pcs {
                prompt.push_str(&format!(
                    "- {} ({} {}, Level {}): {}\n",
                    pc.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    pc.get("race").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    pc.get("class").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                    pc.get("level").and_then(|v| v.as_i64()).unwrap_or(1),
                    pc.get("backstory").and_then(|v| v.as_str()).unwrap_or("No backstory")
                ));
            }
        }
        
        prompt.push_str("\n\nWorld Context Summary:\n");
        if let Some(deities) = world_data.get("deities").and_then(|v| v.as_array()) {
            prompt.push_str(&format!("- {} deities established\n", deities.len()));
        }
        if let Some(planes) = world_data.get("planes").and_then(|v| v.as_array()) {
            prompt.push_str(&format!("- {} planes defined\n", planes.len()));
        }
        
        prompt.push_str(
            "\n\nGenerate PC-connected content that ties directly to the player character backstories.\n\n\
            IMPORTANT: Return ONLY a valid JSON object. Do not include any text before or after the JSON.\n\
            Do not include markdown code blocks or any other formatting.\n\
            Start your response with { and end with }\n\n\
            Required JSON structure:\n\
            {\n\
              \"entities\": [{\n\
                \"name\": \"string\",\n\
                \"entity_type\": \"npc\",\n\
                \"description\": \"string\",\n\
                \"connection_to_pc\": \"string (which PC and how)\",\n\
                \"metadata\": {\"personality\": \"string\", \"goals\": \"string\", \"secrets\": \"string\"}\n\
              }],\n\
              \"pc_backstory_npcs\": [{\n\
                \"name\": \"string\",\n\
                \"connected_pc\": \"string (PC name)\",\n\
                \"relationship\": \"string\",\n\
                \"description\": \"string\",\n\
                \"current_location\": \"string\",\n\
                \"plot_hooks\": [\"array of strings\"]\n\
              }],\n\
              \"pc_home_locations\": [{\n\
                \"name\": \"string\",\n\
                \"connected_pc\": \"string (PC name)\",\n\
                \"type\": \"string (city/town/village/etc)\",\n\
                \"description\": \"string\",\n\
                \"notable_features\": [\"array of strings\"],\n\
                \"current_events\": [\"array of strings\"]\n\
              }],\n\
              \"pc_related_items\": [{\n\
                \"name\": \"string\",\n\
                \"connected_pc\": \"string (PC name)\",\n\
                \"item_type\": \"string\",\n\
                \"description\": \"string\",\n\
                \"magical\": boolean,\n\
                \"properties\": {\"rarity\": \"string\", \"value\": \"string\", \"special_abilities\": [\"array\"]}\n\
              }],\n\
              \"pc_factions\": [{\n\
                \"name\": \"string\",\n\
                \"connected_pcs\": [\"array of PC names\"],\n\
                \"type\": \"string (guild/order/criminal/etc)\",\n\
                \"description\": \"string\",\n\
                \"goals\": [\"array of strings\"],\n\
                \"notable_members\": [\"array of strings\"],\n\
                \"headquarters\": \"string\"\n\
              }]\n\
            }\n\n\
            Create content that directly connects to each PC's backstory, class, and motivations.\
            Ensure all NPCs, locations, items, and factions have clear connections to specific PCs."
        );
        
        Ok(prompt)
    }

    async fn build_world_population_prompt(&self, campaign_id: i32, world_data: &JsonValue, pc_data: &JsonValue) -> ApiResult<String> {
        // Get campaign details
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        let mut prompt = format!(
            "You are populating the world for the D&D 5e campaign: '{}'\n\n",
            campaign.name
        );
        
        prompt.push_str(&format!("Setting: {}\n", campaign.setting.as_deref().unwrap_or("Fantasy world")));
        prompt.push_str(&format!("Themes: {}\n", campaign.themes.join(", ")));
        prompt.push_str(&format!("Tone: {}\n\n", campaign.tone));
        
        // Summarize existing world elements
        prompt.push_str("Established World Elements:\n");
        if let Some(calendar) = world_data.get("calendar_systems").and_then(|v| v.as_array()).and_then(|a| a.first()) {
            if let Some(name) = calendar.get("name").and_then(|v| v.as_str()) {
                prompt.push_str(&format!("- Calendar: {}\n", name));
            }
        }
        if let Some(regions) = world_data.get("geography_regions").and_then(|v| v.as_array()) {
            prompt.push_str(&format!("- {} geographic regions\n", regions.len()));
        }
        
        // Summarize PC-connected elements
        if let Some(entities) = pc_data.get("entities").and_then(|v| v.as_array()) {
            prompt.push_str(&format!("- {} PC-connected entities\n", entities.len()));
        }
        
        prompt.push_str(
            "\n\nGenerate additional world population content to flesh out the campaign world.\n\n\
            IMPORTANT: Return ONLY a valid JSON object. Do not include any text before or after the JSON.\n\
            Do not include markdown code blocks or any other formatting.\n\
            Start your response with { and end with }\n\n\
            Required JSON structure:\n\
            {\n\
              \"additional_npcs\": [{\n\
                \"name\": \"string\",\n\
                \"occupation\": \"string\",\n\
                \"location\": \"string\",\n\
                \"description\": \"string\",\n\
                \"personality\": \"string\",\n\
                \"plot_hooks\": [\"array of potential quest hooks\"]\n\
              }],\n\
              \"locations\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (tavern/shop/temple/dungeon/etc)\",\n\
                \"description\": \"string\",\n\
                \"notable_features\": [\"array of strings\"],\n\
                \"inhabitants\": [\"array of NPC names or types\"],\n\
                \"secrets\": [\"array of hidden elements\"]\n\
              }],\n\
              \"creatures\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (beast/monstrosity/etc)\",\n\
                \"habitat\": \"string\",\n\
                \"description\": \"string\",\n\
                \"behavior\": \"string\",\n\
                \"challenge_rating\": \"string\"\n\
              }],\n\
              \"flora\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (tree/flower/fungus/etc)\",\n\
                \"habitat\": \"string\",\n\
                \"description\": \"string\",\n\
                \"properties\": {\"medicinal\": boolean, \"magical\": boolean, \"uses\": [\"array\"]}\n\
              }],\n\
              \"quest_hooks\": [{\n\
                \"title\": \"string\",\n\
                \"description\": \"string\",\n\
                \"quest_giver\": \"string\",\n\
                \"location\": \"string\",\n\
                \"rewards\": [\"array of rewards\"],\n\
                \"difficulty\": \"string (easy/medium/hard/deadly)\",\n\
                \"connected_npcs\": [\"array of NPC names\"],\n\
                \"objectives\": [\"array of quest objectives\"]\n\
              }],\n\
              \"organizations\": [{\n\
                \"name\": \"string\",\n\
                \"type\": \"string (guild/cult/merchant/military/etc)\",\n\
                \"description\": \"string\",\n\
                \"goals\": [\"array of organization goals\"],\n\
                \"notable_members\": [\"array of member names/titles\"],\n\
                \"resources\": \"string\",\n\
                \"influence\": \"string (local/regional/global)\"\n\
              }],\n\
              \"rumors\": [{\n\
                \"text\": \"string\",\n\
                \"source\": \"string (where players might hear this)\",\n\
                \"truth_level\": \"string (true/partial/false)\",\n\
                \"related_to\": \"string (what quest/npc/location this relates to)\"\n\
              }]\n\
            }\n\n\
            Create a rich, interconnected world with NPCs, locations, creatures, and plot hooks.\
            Ensure content fits the campaign's themes and tone.\
            Make the world feel alive with organizations, rumors, and secrets."
        );
        
        Ok(prompt)
    }
    
    // Tool-based prompt builders
    async fn build_world_building_prompt_with_tool(&self, campaign_id: i32) -> ApiResult<(String, Tool)> {
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        let mut prompt = format!(
            "You are creating a detailed D&D 5e campaign world for: '{}'\n\n",
            campaign.name
        );
        
        if let Some(setting) = &campaign.setting {
            prompt.push_str(&format!("Setting: {}\n", setting));
        }
        
        prompt.push_str(&format!("Themes: {}\n", campaign.themes.join(", ")));
        prompt.push_str(&format!("Tone: {}\n", campaign.tone));
        prompt.push_str(&format!("Campaign Length: {}\n\n", campaign.campaign_length));
        
        if let Some(notes) = &campaign.additional_notes {
            prompt.push_str(&format!("Additional Context: {}\n\n", notes));
        }
        
        prompt.push_str("Generate comprehensive world building content for this D&D campaign.\n\n");
        prompt.push_str("Use the generate_world_building tool to create the content.");
        
        let tool = Tool {
            name: "generate_world_building".to_string(),
            description: "Generates comprehensive world building content for a D&D campaign".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "calendar_systems": {
                        "type": "array",
                        "description": "Calendar systems used in the world",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {"type": "string"},
                                "months": {"type": "array", "items": {"type": "object"}},
                                "weekdays": {"type": "array", "items": {"type": "string"}},
                                "year_length": {"type": "integer"},
                                "current_year": {"type": "integer"},
                                "current_month": {"type": "integer"},
                                "current_day": {"type": "integer"},
                                "special_events": {"type": "array", "items": {"type": "string"}},
                                "lunar_cycles": {"type": "object"}
                            },
                            "required": ["name", "months", "weekdays", "year_length", "current_year"]
                        }
                    },
                    "currencies": {
                        "type": "array",
                        "description": "Currency systems used in different regions",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {"type": "string"},
                                "description": {"type": "string"},
                                "denominations": {"type": "array", "items": {"type": "object"}},
                                "exchange_rates": {"type": "object"},
                                "regions_used": {"type": "array", "items": {"type": "string"}}
                            },
                            "required": ["name", "description", "denominations"]
                        }
                    },
                    "historical_events": {
                        "type": "array",
                        "description": "Major historical events that shaped the world",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {"type": "string"},
                                "year": {"type": "integer"},
                                "description": {"type": "string"},
                                "impact": {"type": "string"},
                                "related_locations": {"type": "array", "items": {"type": "string"}},
                                "key_figures": {"type": "array", "items": {"type": "string"}}
                            },
                            "required": ["name", "year", "description", "impact"]
                        }
                    },
                    "trade_routes": {
                        "type": "array",
                        "description": "Major trade routes connecting regions",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {"type": "string"},
                                "start_location": {"type": "string"},
                                "end_location": {"type": "string"},
                                "major_stops": {"type": "array", "items": {"type": "string"}},
                                "goods_traded": {"type": "array", "items": {"type": "string"}},
                                "hazards": {"type": "array", "items": {"type": "string"}},
                                "travel_time_days": {"type": "integer"}
                            },
                            "required": ["name", "start_location", "end_location", "goods_traded"]
                        }
                    },
                    "world_history": {"type": "array", "items": {"type": "object"}},
                    "planes": {"type": "array", "items": {"type": "object"}},
                    "pantheons": {"type": "array", "items": {"type": "object"}},
                    "deities": {"type": "array", "items": {"type": "object"}},
                    "geography_regions": {"type": "array", "items": {"type": "object"}},
                    "economic_systems": {"type": "array", "items": {"type": "object"}},
                    "legal_systems": {"type": "array", "items": {"type": "object"}},
                    "astronomy": {"type": "array", "items": {"type": "object"}},
                    "zodiac_signs": {"type": "array", "items": {"type": "object"}},
                    "languages": {"type": "array", "items": {"type": "object"}},
                    "races": {"type": "array", "items": {"type": "object"}},
                    "classes": {"type": "array", "items": {"type": "object"}},
                    "backgrounds": {"type": "array", "items": {"type": "object"}},
                    "cultures": {"type": "array", "items": {"type": "object"}},
                    "feats": {"type": "array", "items": {"type": "object"}},
                    "spells": {"type": "array", "items": {"type": "object"}}
                },
                "required": ["calendar_systems", "pantheons", "deities", "geography_regions", "currencies", "historical_events"]
            })
        };
        
        Ok((prompt, tool))
    }
    
    async fn build_pc_connected_prompt_with_tool(&self, campaign_id: i32, world_data: &JsonValue) -> ApiResult<(String, Tool)> {
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        let mut prompt = format!(
            "You are creating PC-connected content for the D&D 5e campaign: '{}'\n\n",
            campaign.name
        );
        
        // Add player character information
        prompt.push_str("Player Characters:\n");
        for pc in campaign.player_characters.as_array().unwrap_or(&vec![]) {
            if let (Some(name), Some(race), Some(class), Some(level)) = (
                pc.get("name").and_then(|v| v.as_str()),
                pc.get("race").and_then(|v| v.as_str()),
                pc.get("class").and_then(|v| v.as_str()),
                pc.get("level").and_then(|v| v.as_i64()),
            ) {
                prompt.push_str(&format!("- {} ({} {}, Level {})", name, race, class, level));
                if let Some(backstory) = pc.get("backstory").and_then(|v| v.as_str()) {
                    prompt.push_str(&format!(": {}", backstory));
                }
                prompt.push_str("\n");
            }
        }
        
        prompt.push_str("\nGenerate PC-connected content that ties directly to the player character backstories.\n");
        prompt.push_str("Use the generate_pc_connected tool to create the content.");
        
        let tool = Tool {
            name: "generate_pc_connected".to_string(),
            description: "Generates content connected to player characters".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "entities": {"type": "array", "items": {"type": "object"}},
                    "pc_backstory_npcs": {"type": "array", "items": {"type": "object"}},
                    "pc_home_locations": {"type": "array", "items": {"type": "object"}},
                    "pc_related_items": {"type": "array", "items": {"type": "object"}},
                    "pc_factions": {"type": "array", "items": {"type": "object"}}
                },
                "required": ["entities", "pc_backstory_npcs", "pc_home_locations", "pc_related_items", "pc_factions"]
            })
        };
        
        Ok((prompt, tool))
    }
    
    async fn build_world_population_prompt_with_tool(&self, campaign_id: i32, world_data: &JsonValue, pc_data: &JsonValue) -> ApiResult<(String, Tool)> {
        let campaign = self.database.get_campaign(campaign_id).await?;
        
        let mut prompt = format!(
            "You are populating the world for the D&D 5e campaign: '{}'\n\n",
            campaign.name
        );
        
        prompt.push_str(&format!("Setting: {}\n", campaign.setting.as_deref().unwrap_or("Fantasy world")));
        prompt.push_str(&format!("Themes: {}\n", campaign.themes.join(", ")));
        prompt.push_str(&format!("Tone: {}\n\n", campaign.tone));
        
        prompt.push_str("Generate comprehensive world population content including all NPCs, locations, quest hooks, encounters, shops, taverns, temples, and magic items.\n");
        prompt.push_str("Use the populate_world tool to create all the content in one structured response.\n");
        
        // Get auto-generated schemas from Hasura for all tables in this phase
        let schema_gen = self.schema_generator.read().await;
        let mut properties = json!({});
        
        // All tables that should be populated in the World Population phase
        let tables = [
            "locations", "npcs", "quest_hooks", "encounters", 
            "magic_items", "shops", "taverns", "temples", "location_npcs"
        ];
        
        // Build comprehensive schema combining all table schemas
        for table in &tables {
            if let Some(table_schema) = schema_gen.get_insert_input_schema(table) {
                properties[table] = json!({
                    "type": "array",
                    "description": format!("Array of {} to insert", table),
                    "items": table_schema
                });
            }
        }
        
        let tool = Tool {
            name: "populate_world".to_string(),
            description: "Generates comprehensive world population including all locations, NPCs, quest hooks, encounters, and supporting entities".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": properties,
                "required": ["locations", "npcs", "quest_hooks", "encounters"]
            }),
        };
        
        Ok((prompt, tool))
    }

    // Phase-specific save methods using GraphQL/Hasura mutations
    async fn save_phase_1a_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 1A content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_1a_data(campaign_id, content).await?;
        info!("Saved Phase 1A entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_1b_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 1B content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_1b_data(campaign_id, content).await?;
        info!("Saved Phase 1B entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_1c_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 1C content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_1c_data(campaign_id, content).await?;
        info!("Saved Phase 1C entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_2a_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 2A content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_2a_data(campaign_id, content).await?;
        info!("Saved Phase 2A entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_2b_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 2B content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_2b_data(campaign_id, content).await?;
        info!("Saved Phase 2B entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_2c_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 2C content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_2c_data(campaign_id, content).await?;
        info!("Saved Phase 2C entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_3a_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 3A content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_3a_data(campaign_id, content).await?;
        info!("Saved Phase 3A entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_3b_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 3B content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_3b_data(campaign_id, content).await?;
        info!("Saved Phase 3B entities: {:?}", saved_entities);
        
        Ok(())
    }

    async fn save_phase_3c_content(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving Phase 3C content for campaign {}", campaign_id);
        
        let saved_entities = self.graphql.save_phase_3c_data(campaign_id, content).await?;
        info!("Saved Phase 3C entities: {:?}", saved_entities);
        
        Ok(())
    }

    // New GraphQL-based save method for world population content
    async fn save_world_population_content_graphql(&self, campaign_id: i32, content: &JsonValue) -> ApiResult<()> {
        info!("Saving world population content via GraphQL for campaign {}", campaign_id);

        // Process additional NPCs
        if let Some(additional_npcs) = content.get("additional_npcs").and_then(|v| v.as_array()) {
            for npc in additional_npcs {
                match self.graphql.save_npc(campaign_id, npc).await {
                    Ok(npc_id) => {
                        info!("Saved NPC {} with ID {}", 
                            npc.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"), npc_id);
                    }
                    Err(e) => {
                        warn!("Failed to save NPC {}: {}", 
                            npc.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"), e);
                    }
                }
            }
        }

        Ok(())
    }
}
