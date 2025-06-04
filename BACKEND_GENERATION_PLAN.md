# Backend Generation Service Implementation Plan

## Overview
Transform the current 3-phase generation system into a comprehensive 9-phase system that leverages the new 40+ table schema with auto-generated tool schemas from Hasura.

## Implementation Order & Strategy

### Phase 1: Core Infrastructure Updates
**Goal**: Update the foundational generation system to support 9 phases

#### 1.1 Update Generation Phase Configuration
- **File**: `src/services/generation_service_enhanced.rs`
- **Changes**:
  - Update `GenerationPhase` struct to include phase metadata
  - Define all 9 phases with their table dependencies
  - Update phase execution logic from 3 → 9 phases
  - Add phase dependency validation

#### 1.2 Update Hasura Schema Generator Integration
- **File**: `src/services/hasura_schema_generator.rs`
- **Changes**:
  - Add phase-specific schema bundling methods
  - Create `get_phase_schemas(phase_name)` method
  - Add table groupings for each phase
  - Test schema generation for all phases

#### 1.3 Update Campaign Status Tracking
- **File**: `src/models/campaign.rs`
- **Changes**:
  - Update total_phases from 3 to 9
  - Add phase naming constants
  - Update status progression logic

### Phase 2: Individual Phase Implementation
**Goal**: Implement each of the 9 generation phases one by one

#### 2.1 Phase 1A: Core World Systems
- **Tables**: `calendar_systems`, `planes`, `geography_regions`, `historical_periods`, `economic_systems`, `legal_systems`, `celestial_bodies`
- **Method**: `build_phase_1a_prompt_with_tool()`
- **Focus**: World-independent foundational systems
- **Dependencies**: None (first phase)

#### 2.2 Phase 1B: Character Building Systems  
- **Tables**: `races`, `character_classes`, `feats`, `backgrounds`
- **Method**: `build_phase_1b_prompt_with_tool()`
- **Focus**: Player character creation systems
- **Dependencies**: Phase 1A (geography for racial origins)

#### 2.3 Phase 1C: Social Framework
- **Tables**: `languages`, `cultures`, `factions`, `pantheons`, `deities`
- **Method**: `build_phase_1c_prompt_with_tool()`
- **Focus**: Social and religious systems
- **Dependencies**: Phase 1A + 1B (races, geography)

#### 2.4 Phase 2A: PC-Connected Entities
- **Tables**: `entities` (with PC connections)
- **Method**: `build_phase_2a_prompt_with_tool()`
- **Focus**: NPCs directly tied to PC backstories
- **Dependencies**: All Phase 1 data + PC backstories

#### 2.5 Phase 2B: PC-Connected Locations
- **Tables**: `locations`, `dungeons`, `buildings`
- **Method**: `build_phase_2b_prompt_with_tool()`
- **Focus**: Places significant to PC stories
- **Dependencies**: Phase 1 + 2A (geography, entities)

#### 2.6 Phase 2C: PC-Connected Items
- **Tables**: `items`, `item_effects`, `sentient_item_properties`
- **Method**: `build_phase_2c_prompt_with_tool()`
- **Focus**: Equipment and artifacts for PCs
- **Dependencies**: Phase 1 + 2A + 2B (entities, locations)

#### 2.7 Phase 3A: Quest Hooks & Encounters
- **Tables**: `quest_hooks`, `encounters`
- **Method**: `build_phase_3a_prompt_with_tool()`
- **Focus**: Adventures and challenges
- **Dependencies**: All previous phases

#### 2.8 Phase 3B: World Population
- **Tables**: `shops`, `taverns`, `temples` (additional world content)
- **Method**: `build_phase_3b_prompt_with_tool()`
- **Focus**: Populate locations with businesses and services
- **Dependencies**: Phase 2B (locations, buildings)

#### 2.9 Phase 3C: Final Relationships
- **Tables**: All relationship tables (`entity_relationships`, `entity_locations`, `entity_factions`, etc.)
- **Method**: `build_phase_3c_prompt_with_tool()`
- **Focus**: Connect all entities, locations, and factions
- **Dependencies**: All previous phases

### Phase 3: Context Management & Data Flow
**Goal**: Ensure proper data passing between phases

#### 3.1 Context Retrieval Methods
- **File**: `src/services/database_enhanced.rs`
- **Methods**:
  - `get_phase_1_context(campaign_id)` - World foundation data
  - `get_phase_2_context(campaign_id)` - PC-connected data
  - `get_phase_3_context(campaign_id)` - All previous data
- **Purpose**: Provide context to subsequent phases

#### 3.2 Phase Dependency Validation
- **Logic**: Ensure each phase has required context
- **Error Handling**: Graceful failure if dependencies missing
- **Recovery**: Ability to retry phases with missing data

### Phase 4: Save Method Implementation
**Goal**: Create save methods for all new tables

#### 4.1 Individual Table Save Methods
- **File**: `src/services/database_enhanced.rs`
- **New Methods**:
  - Phase 1A: `save_calendar_system()`, `save_plane()`, `save_geography_region()`, etc.
  - Phase 1B: `save_race()`, `save_character_class()`, `save_feat()`, `save_background()`
  - Phase 1C: `save_language()`, `save_culture()`, `save_faction()`, `save_pantheon()`, `save_deity()`
  - Phase 2A: `save_entity()` (enhanced for PC connections)
  - Phase 2B: `save_location()` (hierarchical), `save_dungeon()`, `save_building()`
  - Phase 2C: `save_item()`, `save_item_effect()`, `save_sentient_properties()`
  - Phase 3A: `save_quest_hook()`, `save_encounter()` (updated)
  - Phase 3B: `save_shop()`, `save_tavern()`, `save_temple()`
  - Phase 3C: All relationship save methods

#### 4.2 Batch Save Operations
- **Purpose**: Save arrays of related data efficiently
- **Transaction Support**: Ensure data consistency
- **Error Recovery**: Rollback on partial failures

#### 4.3 Hierarchical Save Logic
- **Locations**: Handle parent-child relationships
- **Entities**: Manage complex entity data
- **Items**: Link effects and sentient properties

### Phase 5: Error Handling & Recovery
**Goal**: Robust error handling for complex 9-phase system

#### 5.1 Phase-Specific Error Handling
- **Phase Timeouts**: Different timeouts for different complexity levels
- **Retry Logic**: Smart retry for transient failures
- **Partial Success**: Handle when some tables save but others fail

#### 5.2 Recovery Mechanisms
- **Phase Restart**: Ability to restart individual phases
- **Context Rebuild**: Regenerate context if corrupted
- **Rollback**: Remove partial data on critical failures

## Implementation Timeline

### Week 1: Foundation (Phase 1)
- [ ] Update generation phase configuration
- [ ] Enhance Hasura schema generator
- [ ] Update campaign status tracking
- [ ] Test basic 9-phase flow

### Week 2: Core Phases (Phase 2.1-2.4)
- [ ] Implement Phase 1A (Core World Systems)
- [ ] Implement Phase 1B (Character Building)
- [ ] Implement Phase 1C (Social Framework) 
- [ ] Implement Phase 2A (PC Entities)
- [ ] Test world foundation generation

### Week 3: PC Content (Phase 2.5-2.7)
- [ ] Implement Phase 2B (PC Locations)
- [ ] Implement Phase 2C (PC Items)
- [ ] Implement Phase 3A (Quests & Encounters)
- [ ] Test PC-connected content generation

### Week 4: World Population (Phase 2.8-2.9)
- [ ] Implement Phase 3B (World Population)
- [ ] Implement Phase 3C (Relationships)
- [ ] Test complete 9-phase generation
- [ ] Performance optimization

### Week 5: Integration & Testing
- [ ] Context management implementation
- [ ] Save methods for all tables
- [ ] Error handling & recovery
- [ ] End-to-end testing

## Success Metrics

### Functional Requirements
- [ ] All 9 phases complete successfully in sequence
- [ ] Data dependencies properly maintained between phases
- [ ] PC backstories properly integrated into world content
- [ ] Hierarchical relationships working (locations, entities)
- [ ] Error recovery working for all failure scenarios

### Performance Requirements
- [ ] Each phase completes within 60-90 seconds
- [ ] Total generation time under 10 minutes
- [ ] Database operations efficient (batch saves, transactions)
- [ ] Memory usage stable throughout generation

### Quality Requirements
- [ ] Generated content feels cohesive and interconnected
- [ ] PC connections clearly established and meaningful
- [ ] World systems (economy, religion, politics) integrated
- [ ] No orphaned data or broken relationships

## Risk Mitigation

### High Risk Areas
1. **Phase Dependencies**: Complex data flow between phases
   - **Mitigation**: Extensive testing of context passing
2. **Database Performance**: Large number of insert operations
   - **Mitigation**: Batch operations, transaction optimization
3. **AI Response Quality**: Ensuring coherent content across phases
   - **Mitigation**: Careful prompt engineering, context management

### Rollback Strategy
- Maintain existing 3-phase system as fallback
- Feature flag to toggle between systems
- Comprehensive testing in development environment
- Staged rollout to production

## Next Steps
1. Start with **Phase 1.1**: Update generation phase configuration
2. Create basic 9-phase structure with placeholder methods
3. Implement one complete phase end-to-end as proof of concept
4. Iterate and refine based on initial results