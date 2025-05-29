# UPGRADE PLAN PHASE 4: ANTHROPIC API PROMPT ENGINEERING

## Overview
This phase focuses on evolving the AI generation system from creating 4 entity types to generating 100+ interconnected entities using advanced prompt engineering, multi-stage generation, and consistency validation. The core focus is creating a lived-in world based on player character backstories rather than plotting.

## Current State
- Single API call generating 4 entity types
- ~2000 token responses
- Basic JSON structure
- Simple relationships

## Target State
- Multi-stage generation (10-15 API calls)
- 100k+ token context utilization
- Complex hierarchical generation
- Relationship-aware content creation
- Consistency validation between stages
- Progress tracking at entity level

## Implementation Steps

### Step 1: Prompt Architecture Design
**Duration**: 4 days

1. **Generation Strategy Framework**
   ```rust
   // src/services/generation/strategy.rs
   use serde::{Serialize, Deserialize};
   
   #[derive(Debug, Clone)]
   pub struct GenerationStrategy {
       pub phases: Vec<GenerationPhase>,
       pub dependencies: HashMap<String, Vec<String>>,
       pub validation_rules: Vec<ValidationRule>,
   }
   
   #[derive(Debug, Clone)]
   pub struct GenerationPhase {
       pub id: String,
       pub name: String,
       pub entity_types: Vec<EntityType>,
       pub max_tokens: usize,
       pub temperature: f32,
       pub retry_attempts: u32,
       pub depends_on: Vec<String>,
   }
   
   impl GenerationStrategy {
       pub fn campaign_full() -> Self {
           Self {
               phases: vec![
                   GenerationPhase {
                       id: "pc_backstory_analysis".to_string(),
                       name: "PC Backstory Analysis & Integration".to_string(),
                       entity_types: vec![
                           EntityType::BackstoryElements,
                           EntityType::BackstoryNPCs,
                           EntityType::BackstoryLocations,
                           EntityType::BackstoryOrganizations,
                       ],
                       max_tokens: 8000,
                       temperature: 0.7,
                       retry_attempts: 3,
                       depends_on: vec![],
                   },
                   GenerationPhase {
                       id: "world_foundation".to_string(),
                       name: "World Foundation Based on PC Context".to_string(),
                       entity_types: vec![
                           EntityType::CalendarSystem,
                           EntityType::Planes,
                           EntityType::Deities,
                           EntityType::HistoricalEras,
                       ],
                       max_tokens: 8000,
                       temperature: 0.8,
                       retry_attempts: 3,
                       depends_on: vec!["pc_backstory_analysis".to_string()],
                   },
                   GenerationPhase {
                       id: "geography".to_string(),
                       name: "Geography & Locations".to_string(),
                       entity_types: vec![
                           EntityType::Continents,
                           EntityType::Regions,
                           EntityType::Cities,
                           EntityType::PointsOfInterest,
                       ],
                       max_tokens: 10000,
                       temperature: 0.7,
                       retry_attempts: 3,
                       depends_on: vec!["world_foundation".to_string()],
                   },
                   GenerationPhase {
                       id: "cultures_races".to_string(),
                       name: "Races & Cultures".to_string(),
                       entity_types: vec![
                           EntityType::Races,
                           EntityType::Cultures,
                           EntityType::Languages,
                       ],
                       max_tokens: 8000,
                       temperature: 0.7,
                       retry_attempts: 3,
                       depends_on: vec!["geography".to_string()],
                   },
                   // ... more phases
               ],
               dependencies: HashMap::new(),
               validation_rules: vec![],
           }
       }
   }
   ```

2. **Prompt Template System**
   ```rust
   // src/services/generation/templates.rs
   use handlebars::Handlebars;
   use std::collections::HashMap;
   
   pub struct PromptTemplates {
       handlebars: Handlebars<'static>,
   }
   
   impl PromptTemplates {
       pub fn new() -> Self {
           let mut handlebars = Handlebars::new();
           
           // Register templates
           handlebars.register_template_string(
               "world_foundation",
               include_str!("../../templates/world_foundation.hbs")
           ).unwrap();
           
           handlebars.register_template_string(
               "geography",
               include_str!("../../templates/geography.hbs")
           ).unwrap();
           
           Self { handlebars }
       }
       
       pub fn render_phase_prompt(
           &self,
           phase: &GenerationPhase,
           context: &GenerationContext,
       ) -> Result<String> {
           let mut data = HashMap::new();
           data.insert("campaign", &context.campaign);
           data.insert("player_characters", &context.player_characters);
           data.insert("previous_phases", &context.completed_phases);
           data.insert("themes", &context.themes);
           
           self.handlebars.render(&phase.id, &data)
               .map_err(|e| anyhow::anyhow!("Template render error: {}", e))
       }
   }
   ```

3. **Template Examples**
   ```handlebars
   <!-- templates/world_foundation.hbs -->
   You are creating the foundational worldbuilding for a D&D campaign based on the player characters' backstories. The world should feel lived-in and directly connected to the PCs' histories.
   
   Campaign: {{campaign.name}}
   Setting: {{campaign.setting}}
   Themes: {{#each themes}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
   
   Player Characters:
   {{#each player_characters}}
   - {{name}} ({{race}} {{class}})
     Background: {{background}}
     Backstory: {{backstory}}
     Important NPCs: {{important_npcs}}
     Home/Origin: {{origin_location}}
   {{/each}}
   
   Previously Identified Backstory Elements:
   {{#each previous_phases.pc_backstory_analysis}}
   - {{type}}: {{name}} (Connected to {{connected_pc}})
   {{/each}}
   
   Generate the following interconnected world elements, ensuring they naturally incorporate and expand upon the PC backstories:
   
   <world_foundation>
   {
     "calendar_system": {
       "name": "string",
       "days_per_year": number,
       "months": [
         {
           "name": "string",
           "days": number,
           "season": "string"
         }
       ],
       "current_year": number,
       "major_holidays": [
         {
           "name": "string",
           "date": "string",
           "significance": "string"
         }
       ]
     },
     "cosmology": {
       "planes": [
         {
           "name": "string",
           "description": "string",
           "alignment": "string",
           "inhabitants": ["string"],
           "access_methods": ["string"]
         }
       ],
       "creation_myth": "string"
     },
     "pantheon": {
       "deities": [
         {
           "name": "string",
           "titles": ["string"],
           "domains": ["string"],
           "alignment": "string",
           "holy_symbol": "string",
           "worshippers": "string",
           "personality": "string",
           "relationships": {
             "allies": ["string"],
             "enemies": ["string"]
           }
         }
       ]
     },
     "historical_context": {
       "current_era": "string",
       "major_past_events": [
         {
           "name": "string",
           "years_ago": number,
           "description": "string",
           "lasting_effects": ["string"]
         }
       ],
       "pc_connections": [
         {
           "element": "string",
           "connected_to_pc": "string",
           "how_it_connects": "string"
         }
       ]
     }
   }
   </world_foundation>
   
   Ensure all elements reflect the campaign themes and provide hooks for player character backgrounds.
   ```

### Step 2: Multi-Stage Generation Pipeline
**Duration**: 4 days

1. **Generation Orchestrator**
   ```rust
   // src/services/generation/orchestrator.rs
   use tokio::sync::mpsc;
   use std::sync::Arc;
   
   pub struct GenerationOrchestrator {
       anthropic: Arc<AnthropicClient>,
       database: Arc<DatabaseService>,
       templates: Arc<PromptTemplates>,
       progress_tx: mpsc::Sender<GenerationProgress>,
   }
   
   impl GenerationOrchestrator {
       pub async fn generate_campaign(
           &self,
           campaign_id: i32,
           context: GenerationContext,
       ) -> Result<()> {
           let strategy = GenerationStrategy::campaign_full();
           let mut completed_phases = HashMap::new();
           
           for phase in &strategy.phases {
               // Check dependencies
               for dep in &phase.depends_on {
                   if !completed_phases.contains_key(dep) {
                       return Err(anyhow::anyhow!(
                           "Dependency {} not satisfied for phase {}",
                           dep, phase.id
                       ));
                   }
               }
               
               // Send progress update
               self.progress_tx.send(GenerationProgress {
                   campaign_id,
                   phase: phase.name.clone(),
                   progress: 0.0,
                   status: "starting".to_string(),
               }).await?;
               
               // Generate phase content
               let result = self.generate_phase(phase, &context, &completed_phases).await?;
               
               // Validate results
               self.validate_phase_results(&phase, &result).await?;
               
               // Store results
               self.store_phase_results(campaign_id, &phase, &result).await?;
               
               // Update completed phases
               completed_phases.insert(phase.id.clone(), result);
               
               // Send progress update
               self.progress_tx.send(GenerationProgress {
                   campaign_id,
                   phase: phase.name.clone(),
                   progress: 1.0,
                   status: "completed".to_string(),
               }).await?;
           }
           
           Ok(())
       }
       
       async fn generate_phase(
           &self,
           phase: &GenerationPhase,
           context: &GenerationContext,
           completed: &HashMap<String, PhaseResult>,
       ) -> Result<PhaseResult> {
           let mut context = context.clone();
           context.completed_phases = completed.clone();
           
           let prompt = self.templates.render_phase_prompt(phase, &context)?;
           
           let mut attempts = 0;
           loop {
               attempts += 1;
               
               match self.anthropic.generate_with_retry(
                   &prompt,
                   phase.max_tokens,
                   phase.temperature,
               ).await {
                   Ok(response) => {
                       match self.parse_phase_response(&response, phase) {
                           Ok(result) => return Ok(result),
                           Err(e) if attempts < phase.retry_attempts => {
                               tracing::warn!(
                                   "Parse error in phase {}, attempt {}: {}",
                                   phase.id, attempts, e
                               );
                               continue;
                           }
                           Err(e) => return Err(e),
                       }
                   }
                   Err(e) if attempts < phase.retry_attempts => {
                       tracing::warn!(
                           "Generation error in phase {}, attempt {}: {}",
                           phase.id, attempts, e
                       );
                       tokio::time::sleep(Duration::from_secs(2_u64.pow(attempts))).await;
                       continue;
                   }
                   Err(e) => return Err(e),
               }
           }
       }
   }
   ```

2. **Consistency Validation**
   ```rust
   // src/services/generation/validation.rs
   #[derive(Debug)]
   pub struct ValidationRule {
       pub name: String,
       pub phase_dependencies: Vec<String>,
       pub validate: Box<dyn Fn(&PhaseResult, &HashMap<String, PhaseResult>) -> Result<()>>,
   }
   
   impl GenerationOrchestrator {
       fn create_validation_rules() -> Vec<ValidationRule> {
           vec![
               ValidationRule {
                   name: "deity_culture_consistency".to_string(),
                   phase_dependencies: vec!["cultures_races".to_string(), "world_foundation".to_string()],
                   validate: Box::new(|current, completed| {
                       let cultures = &completed["cultures_races"].data["cultures"];
                       let deities = &completed["world_foundation"].data["pantheon"]["deities"];
                       
                       // Ensure each culture references valid deities
                       for culture in cultures.as_array().unwrap() {
                           if let Some(patron_deity) = culture.get("patron_deity") {
                               let deity_exists = deities.as_array().unwrap()
                                   .iter()
                                   .any(|d| d["name"] == patron_deity);
                               
                               if !deity_exists {
                                   return Err(anyhow::anyhow!(
                                       "Culture {} references non-existent deity {}",
                                       culture["name"], patron_deity
                                   ));
                               }
                           }
                       }
                       Ok(())
                   }),
               },
               // More validation rules...
           ]
       }
   }
   ```

### Step 3: Context Management
**Duration**: 3 days

1. **Context Window Optimization**
   ```rust
   // src/services/generation/context.rs
   pub struct ContextManager {
       max_context_tokens: usize,
       tokenizer: Arc<Tokenizer>,
   }
   
   impl ContextManager {
       pub fn new(max_tokens: usize) -> Self {
           Self {
               max_context_tokens: max_tokens,
               tokenizer: Arc::new(Tokenizer::new()),
           }
       }
       
       pub fn build_phase_context(
           &self,
           phase: &GenerationPhase,
           full_context: &GenerationContext,
           completed_phases: &HashMap<String, PhaseResult>,
       ) -> Result<OptimizedContext> {
           let mut context = OptimizedContext::new();
           
           // Always include campaign basics
           context.add_section("campaign", &full_context.campaign, Priority::High);
           
           // Include relevant player characters
           let relevant_pcs = self.filter_relevant_pcs(
               &full_context.player_characters,
               phase,
           );
           context.add_section("player_characters", &relevant_pcs, Priority::High);
           
           // Include dependent phase results
           for dep_id in &phase.depends_on {
               if let Some(dep_result) = completed_phases.get(dep_id) {
                   let summary = self.summarize_phase_result(dep_result);
                   context.add_section(
                       &format!("previous_{}", dep_id),
                       &summary,
                       Priority::Medium,
                   );
               }
           }
           
           // Optimize to fit context window
           context.optimize_to_fit(self.max_context_tokens, &self.tokenizer)?;
           
           Ok(context)
       }
       
       fn summarize_phase_result(&self, result: &PhaseResult) -> Value {
           // Create a condensed version focusing on names and relationships
           match result.phase_type {
               PhaseType::Geography => {
                   json!({
                       "locations": result.data["locations"]
                           .as_array()
                           .unwrap()
                           .iter()
                           .map(|loc| json!({
                               "name": loc["name"],
                               "type": loc["type"],
                               "parent": loc.get("parent_name"),
                           }))
                           .collect::<Vec<_>>()
                   })
               }
               _ => result.data.clone(),
           }
       }
   }
   ```

2. **Incremental Generation**
   ```rust
   // src/services/generation/incremental.rs
   pub struct IncrementalGenerator {
       chunk_size: usize,
       overlap_size: usize,
   }
   
   impl IncrementalGenerator {
       pub async fn generate_large_set<T>(
           &self,
           anthropic: &AnthropicClient,
           base_prompt: &str,
           total_items: usize,
           parser: impl Fn(&str) -> Result<Vec<T>>,
       ) -> Result<Vec<T>> {
           let mut all_items = Vec::new();
           let mut context_items = Vec::new();
           
           let chunks = (total_items + self.chunk_size - 1) / self.chunk_size;
           
           for i in 0..chunks {
               let start = i * self.chunk_size;
               let end = ((i + 1) * self.chunk_size).min(total_items);
               
               let prompt = format!(
                   "{}\n\nGenerate items {} to {} of {}.\n\nPreviously generated (for context):\n{:?}",
                   base_prompt,
                   start + 1,
                   end,
                   total_items,
                   context_items
               );
               
               let response = anthropic.generate(&prompt, 4000, 0.7).await?;
               let items = parser(&response)?;
               
               all_items.extend(items.clone());
               
               // Keep last N items for context
               context_items = items
                   .into_iter()
                   .skip(items.len().saturating_sub(self.overlap_size))
                   .collect();
           }
           
           Ok(all_items)
       }
   }
   ```

### Step 4: Progress Tracking
**Duration**: 2 days

1. **Granular Progress Reporting**
   ```rust
   // src/models/generation_progress.rs
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct DetailedProgress {
       pub campaign_id: i32,
       pub overall_progress: f32,
       pub current_phase: PhaseProgress,
       pub completed_phases: Vec<PhaseProgress>,
       pub entity_counts: EntityCounts,
       pub estimated_completion: DateTime<Utc>,
       pub warnings: Vec<String>,
   }
   
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct PhaseProgress {
       pub id: String,
       pub name: String,
       pub status: PhaseStatus,
       pub progress: f32,
       pub started_at: DateTime<Utc>,
       pub completed_at: Option<DateTime<Utc>>,
       pub entities_generated: HashMap<String, usize>,
       pub retry_count: usize,
   }
   
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub enum PhaseStatus {
       Pending,
       Running,
       Validating,
       Storing,
       Completed,
       Failed(String),
       Retrying(usize),
   }
   ```

2. **Real-time Updates**
   ```rust
   // src/services/generation/progress_tracker.rs
   pub struct ProgressTracker {
       tx: mpsc::Sender<DetailedProgress>,
       state: Arc<RwLock<GenerationState>>,
   }
   
   impl ProgressTracker {
       pub async fn update_phase_progress(
           &self,
           phase_id: &str,
           progress: f32,
           entities: HashMap<String, usize>,
       ) -> Result<()> {
           let mut state = self.state.write().await;
           
           if let Some(phase) = state.phases.get_mut(phase_id) {
               phase.progress = progress;
               phase.entities_generated = entities;
           }
           
           let detailed = self.calculate_detailed_progress(&state);
           self.tx.send(detailed).await?;
           
           Ok(())
       }
       
       fn calculate_detailed_progress(&self, state: &GenerationState) -> DetailedProgress {
           let total_phases = state.phases.len() as f32;
           let completed = state.phases.values()
               .filter(|p| matches!(p.status, PhaseStatus::Completed))
               .count() as f32;
           
           let current_phase_progress = state.current_phase
               .as_ref()
               .and_then(|id| state.phases.get(id))
               .map(|p| p.progress)
               .unwrap_or(0.0);
           
           let overall_progress = (completed + current_phase_progress) / total_phases;
           
           // Estimate completion based on average phase duration
           let avg_duration = self.calculate_average_phase_duration(state);
           let remaining_phases = total_phases - completed - 1.0;
           let estimated_completion = Utc::now() + 
               Duration::seconds((avg_duration * remaining_phases) as i64);
           
           DetailedProgress {
               campaign_id: state.campaign_id,
               overall_progress,
               current_phase: state.current_phase
                   .as_ref()
                   .and_then(|id| state.phases.get(id))
                   .cloned()
                   .unwrap_or_default(),
               completed_phases: state.phases.values()
                   .filter(|p| matches!(p.status, PhaseStatus::Completed))
                   .cloned()
                   .collect(),
               entity_counts: self.aggregate_entity_counts(state),
               estimated_completion,
               warnings: state.warnings.clone(),
           }
       }
   }
   ```

### Step 5: Error Recovery
**Duration**: 2 days

1. **Partial Generation Recovery**
   ```rust
   // src/services/generation/recovery.rs
   pub struct GenerationRecovery {
       database: Arc<DatabaseService>,
       storage: Arc<RecoveryStorage>,
   }
   
   impl GenerationRecovery {
       pub async fn save_checkpoint(
           &self,
           campaign_id: i32,
           phase_id: &str,
           partial_result: &PartialResult,
       ) -> Result<()> {
           let checkpoint = GenerationCheckpoint {
               campaign_id,
               phase_id: phase_id.to_string(),
               timestamp: Utc::now(),
               partial_data: partial_result.clone(),
               completed_entities: partial_result.count_entities(),
           };
           
           self.storage.save_checkpoint(&checkpoint).await?;
           Ok(())
       }
       
       pub async fn resume_from_checkpoint(
           &self,
           campaign_id: i32,
       ) -> Result<Option<GenerationResume>> {
           if let Some(checkpoint) = self.storage.get_latest_checkpoint(campaign_id).await? {
               // Load completed phases from database
               let completed = self.database
                   .get_completed_generation_phases(campaign_id)
                   .await?;
               
               Ok(Some(GenerationResume {
                   last_phase: checkpoint.phase_id,
                   partial_data: checkpoint.partial_data,
                   completed_phases: completed,
                   resume_from_entity: checkpoint.completed_entities,
               }))
           } else {
               Ok(None)
           }
       }
       
       pub async fn handle_phase_failure(
           &self,
           campaign_id: i32,
           phase: &GenerationPhase,
           error: &anyhow::Error,
           attempt: usize,
       ) -> Result<RecoveryAction> {
           match error.downcast_ref::<GenerationError>() {
               Some(GenerationError::TokenLimit) => {
                   // Reduce scope and retry
                   Ok(RecoveryAction::ReduceScope {
                       new_max_tokens: phase.max_tokens / 2,
                       split_into_chunks: true,
                   })
               }
               Some(GenerationError::InvalidJson(_)) if attempt < 3 => {
                   // Retry with clearer instructions
                   Ok(RecoveryAction::RetryWithClarification {
                       additional_instructions: "Ensure response is valid JSON wrapped in XML tags.",
                   })
               }
               Some(GenerationError::RateLimited) => {
                   // Wait and retry
                   Ok(RecoveryAction::WaitAndRetry {
                       delay_seconds: 60,
                   })
               }
               _ => Ok(RecoveryAction::Fail),
           }
       }
   }
   ```

## Testing Strategy

1. **Prompt Testing**
   - Template rendering verification
   - Context size calculations
   - Output format validation

2. **Integration Testing**
   - Multi-phase generation flow
   - Dependency resolution
   - Recovery scenarios

3. **Content Quality Testing**
   - Consistency validation
   - Relationship integrity
   - Theme adherence

## Success Metrics

- 95% generation success rate
- Average generation time <5 minutes
- Context utilization >80%
- Entity relationship consistency >99%
- Successful recovery from 90% of failures

## Next Phase
Proceed to [UPGRADE_PLAN_P5.md](./UPGRADE_PLAN_P5.md) for Frontend Form Complexity.