# D&D Campaign Generator - 9-Phase Migration Plan

## Overview
Migration from the current 3-phase system to a comprehensive 9-phase generation system with 40+ tables supporting hierarchical locations, complex relationships, and deep world-building.

## Phase System Overview

### Phase 1: World Foundation (Independent of PCs)
- **1A: Core World Systems** - Calendar, planes, geography, history, economics, legal systems, astronomy
- **1B: Character Building** - Races, classes, feats, backgrounds
- **1C: Social Framework** - Languages, cultures, factions, pantheons, deities

### Phase 2: PC-Connected Content
- **2A: PC Entities** - NPCs with direct PC connections (family, mentors, rivals, etc.)
- **2B: PC Locations** - Places tied to PC backstories (hierarchical: cities → districts → buildings)
- **2C: PC Items** - Equipment and artifacts relevant to PC stories

### Phase 3: World Population
- **3A: Quest Hooks & Encounters** - Adventures and challenges
- **3B: World Population** - Additional NPCs, creatures, shops, taverns, temples
- **3C: Final Relationships** - All entity-to-entity, entity-to-location, faction relationships

## Implementation Tasks

### 1. Database Updates ✅
- [x] Create comprehensive schema with 40+ tables
- [x] Add hierarchical location support
- [x] Create relationship tables for many-to-many connections
- [x] Add proper indexes and constraints
- [x] Create helpful views for common queries
- [x] Fixed calendar_systems table creation (current_date → current_calendar_date)

### 2. Hasura Configuration ✅
- [x] Track all new tables in Hasura
- [x] Configure table relationships
- [x] Set up permissions
- [x] Generate GraphQL schema
- [x] Test introspection queries

### 3. Backend - Generation Service
- [ ] Update phase configuration (3 → 9)
- [ ] Create 9 phase-specific prompt methods
- [ ] Create 9 phase-specific tool building methods
- [ ] Update phase execution logic
- [ ] Add context passing between phases
- [ ] Update error handling for 9 phases

### 4. Backend - Hasura Schema Generator
- [ ] Add support for 40+ new tables
- [ ] Create phase-specific schema bundling:
  - `get_phase_1a_schemas()` - World systems tables
  - `get_phase_1b_schemas()` - Character building tables
  - `get_phase_1c_schemas()` - Social framework tables
  - `get_phase_2a_schemas()` - Entity table (PC-connected)
  - `get_phase_2b_schemas()` - Location hierarchy tables
  - `get_phase_2c_schemas()` - Item system tables
  - `get_phase_3a_schemas()` - Quest & encounter tables
  - `get_phase_3b_schemas()` - Population tables
  - `get_phase_3c_schemas()` - Relationship tables

### 5. Backend - Database Service
- [ ] Add save methods for new tables:
  - Calendar, planes, geography methods
  - Race, class, feat, background methods
  - Culture, language, faction methods
  - Entity save with PC connections
  - Hierarchical location saves
  - Item and effect saves
  - Relationship table saves
- [ ] Create batch save operations
- [ ] Add transaction support for complex saves

### 6. Backend - GraphQL Client
- [ ] Add mutations for all new tables
- [ ] Create batch mutation methods
- [ ] Update error handling
- [ ] Add relationship mutations

### 7. Frontend - UI Updates
- [ ] Update progress bar for 9 phases
- [ ] Add phase-specific status messages
- [ ] Update campaign detail views
- [ ] Add new entity type displays

### 8. Frontend - GraphQL Integration
- [ ] Update GraphQL queries for new tables
- [ ] Create nested queries for relationships
- [ ] Update subscriptions for real-time updates
- [ ] Regenerate TypeScript types

### 9. Testing & Validation
- [ ] Test each phase independently
- [ ] Verify data dependencies between phases
- [ ] Test error recovery
- [ ] Performance testing with large datasets
- [ ] End-to-end campaign generation test

## Technical Considerations

### API Call Strategy
- Each phase = 1 Anthropic API call
- Average response time: 30-60 seconds per phase
- Total generation time: ~5-10 minutes
- Clear progress indication for users

### Data Volume Estimates
- Phase 1A: ~20-50 records
- Phase 1B: ~30-60 records  
- Phase 1C: ~40-80 records
- Phase 2A: ~20-40 PC-connected NPCs
- Phase 2B: ~30-50 hierarchical locations
- Phase 2C: ~20-40 significant items
- Phase 3A: ~15-30 quest hooks
- Phase 3B: ~100-200 world entities
- Phase 3C: ~200-400 relationships

### Tool Schema Approach
Each phase gets ONE comprehensive tool that includes all relevant tables for that phase:

```json
{
  "name": "generate_phase_1a",
  "description": "Generate core world systems",
  "input_schema": {
    "type": "object",
    "properties": {
      "calendar_systems": { "type": "array", "items": {...} },
      "planes": { "type": "array", "items": {...} },
      "geography_regions": { "type": "array", "items": {...} },
      "historical_periods": { "type": "array", "items": {...} },
      "economic_systems": { "type": "array", "items": {...} },
      "legal_systems": { "type": "array", "items": {...} },
      "celestial_bodies": { "type": "array", "items": {...} }
    },
    "required": ["calendar_systems", "planes", "geography_regions"]
  }
}
```

## Progress Status

### ✅ Completed
- **Database Schema**: All 40+ tables created with proper relationships and indexes
- **Hasura Configuration**: All tables tracked, relationships configured, introspection working
- **Metadata Generation**: Auto-generated 37 Hasura metadata files using Rust
- **GraphQL Schema**: All new tables accessible via GraphQL API

### 🚧 In Progress
- **Backend Generation Service**: Need to implement 9-phase system

### ⏳ Remaining
- Backend Database Service updates
- Backend GraphQL Client updates  
- Frontend UI updates
- Frontend GraphQL Integration
- Testing & Validation

## Key Achievements So Far
1. **37 new tables** successfully created and tracked
2. **Complex relationships** properly configured (hierarchical locations, entity connections)
3. **Auto-generation tools** working (Rust metadata generator, GraphQL introspection)
4. **Zero breaking changes** to existing campaign table
5. **Performance optimized** with proper indexes and views

## Success Criteria
- All 9 phases complete successfully
- Data properly linked through relationships
- Hierarchical locations working correctly
- PC connections clearly established
- World feels cohesive and interconnected
- Generation completes in under 10 minutes
- Clear progress feedback throughout

## Rollback Plan
- Keep existing 3-phase system operational
- Use feature flag to toggle between systems
- Maintain backwards compatibility for existing campaigns
- Document any breaking changes