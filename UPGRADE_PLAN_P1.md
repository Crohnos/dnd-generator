# UPGRADE PLAN PHASE 1: DATABASE SCHEMA EXPANSION

## Overview
This phase focuses on expanding the database from 4 tables to 100+ tables to support comprehensive D&D world-building based on player character backstories. We'll implement a phased migration approach while maintaining backward compatibility.

## Current State
- 4 core tables: campaigns, npcs, locations, quest_hooks, encounters
- Basic JSONB storage for flexible data
- Simple foreign key relationships
- PostgreSQL with basic indexes

## Target State
- 100+ tables covering all D&D world-building aspects
- Hierarchical data structures (location trees, organization hierarchies) tied to PC backstories
- Complex many-to-many relationships
- Soft delete support across all tables
- Full-text search capabilities
- Optimized indexes for performance

## Implementation Steps

### Step 1: Migration Infrastructure Setup
**Duration**: 2 days

1. **Create Migration Versioning System**
   ```sql
   -- 006_migration_infrastructure.sql
   CREATE TABLE schema_versions (
       version INTEGER PRIMARY KEY,
       name TEXT NOT NULL,
       applied_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
       rollback_sql TEXT
   );
   
   CREATE TABLE migration_logs (
       id SERIAL PRIMARY KEY,
       version INTEGER NOT NULL,
       action TEXT CHECK (action IN ('apply', 'rollback')),
       started_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
       completed_at TIMESTAMPTZ,
       error_message TEXT
   );
   ```

2. **Add Soft Delete Support Function**
   ```sql
   -- 007_soft_delete_infrastructure.sql
   CREATE OR REPLACE FUNCTION add_soft_delete_to_table(table_name TEXT)
   RETURNS VOID AS $$
   BEGIN
       EXECUTE format('ALTER TABLE %I ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ', table_name);
       EXECUTE format('CREATE INDEX IF NOT EXISTS idx_%I_deleted_at ON %I(deleted_at) WHERE deleted_at IS NULL', table_name, table_name);
   END;
   $$ LANGUAGE plpgsql;
   ```

3. **Update Existing Tables**
   - Add soft delete columns to existing tables
   - Create partial indexes for performance
   - Update foreign key constraints to handle soft deletes

### Step 2: Core World Building Tables
**Duration**: 3 days

1. **Calendar & Time System**
   ```sql
   -- 008_calendar_system.sql
   CREATE TABLE calendar_systems (
       id SERIAL PRIMARY KEY,
       campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
       name TEXT NOT NULL,
       days_per_year INTEGER,
       current_date JSONB DEFAULT '{}',
       moon_phases JSONB DEFAULT '[]',
       deleted_at TIMESTAMPTZ
   );
   
   CREATE TABLE calendar_months (
       id SERIAL PRIMARY KEY,
       calendar_system_id INTEGER NOT NULL REFERENCES calendar_systems(id) ON DELETE CASCADE,
       name TEXT NOT NULL,
       days_in_month INTEGER,
       season TEXT,
       order_number INTEGER,
       deleted_at TIMESTAMPTZ
   );
   ```

2. **Geography Hierarchy**
   ```sql
   -- 009_geography_hierarchy.sql
   CREATE TABLE location_types (
       id SERIAL PRIMARY KEY,
       type_name TEXT NOT NULL UNIQUE,
       hierarchy_level INTEGER
   );
   
   -- Migrate existing locations to new schema
   ALTER TABLE locations 
       ADD COLUMN parent_location_id INTEGER REFERENCES locations(id),
       ADD COLUMN location_type_id INTEGER REFERENCES location_types(id),
       ADD COLUMN population INTEGER,
       ADD COLUMN danger_level INTEGER CHECK(danger_level BETWEEN 1 AND 10),
       ADD COLUMN map_coordinates JSONB;
   ```

3. **Races & Cultures**
   ```sql
   -- 010_races_cultures.sql
   CREATE TABLE races (
       id SERIAL PRIMARY KEY,
       campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
       name TEXT NOT NULL,
       description TEXT,
       abilities JSONB DEFAULT '{}',
       deleted_at TIMESTAMPTZ
   );
   
   CREATE TABLE cultures (
       id SERIAL PRIMARY KEY,
       campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
       name TEXT NOT NULL,
       values JSONB DEFAULT '[]',
       traditions JSONB DEFAULT '[]',
       deleted_at TIMESTAMPTZ
   );
   ```

### Step 3: Character System Enhancement
**Duration**: 4 days

1. **Enhanced Character Tables**
   ```sql
   -- 011_character_enhancement.sql
   CREATE TABLE characters (
       id SERIAL PRIMARY KEY,
       campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
       character_type TEXT CHECK(character_type IN ('pc', 'npc')),
       name TEXT NOT NULL,
       race_id INTEGER REFERENCES races(id),
       culture_id INTEGER REFERENCES cultures(id),
       -- 5-bullet method fields
       core_identity TEXT,
       primary_motivation TEXT,
       distinctive_quirk TEXT,
       current_situation TEXT,
       hidden_information TEXT,
       -- PC backstory connection fields
       backstory_summary TEXT,
       connected_to_pc_id INTEGER REFERENCES characters(id),
       connection_type TEXT,
       deleted_at TIMESTAMPTZ
   );
   
   -- Migrate existing NPCs
   INSERT INTO characters (campaign_id, character_type, name, description, core_identity)
   SELECT campaign_id, 'npc', name, description, role FROM npcs;
   ```

2. **Character Relationships**
   ```sql
   -- 012_character_relationships.sql
   CREATE TABLE character_relationships (
       id SERIAL PRIMARY KEY,
       character_id_1 INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
       character_id_2 INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
       relationship_type TEXT,
       description TEXT,
       deleted_at TIMESTAMPTZ,
       CHECK (character_id_1 < character_id_2)
   );
   ```

### Step 4: Organizations & Factions
**Duration**: 3 days

1. **Organization Tables**
   ```sql
   -- 013_organizations.sql
   CREATE TABLE organizations (
       id SERIAL PRIMARY KEY,
       campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
       name TEXT NOT NULL,
       organization_type TEXT,
       goals JSONB DEFAULT '[]',
       headquarters_location_id INTEGER REFERENCES locations(id),
       deleted_at TIMESTAMPTZ
   );
   
   CREATE TABLE organization_ranks (
       id SERIAL PRIMARY KEY,
       organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
       rank_name TEXT NOT NULL,
       rank_level INTEGER,
       privileges JSONB DEFAULT '[]',
       deleted_at TIMESTAMPTZ
   );
   ```

### Step 5: Story Architecture
**Duration**: 4 days

1. **Story Arc System**
   ```sql
   -- 014_story_architecture.sql
   CREATE TABLE story_arcs (
       id SERIAL PRIMARY KEY,
       campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
       name TEXT NOT NULL,
       arc_type TEXT CHECK(arc_type IN ('personal', 'convergent', 'world', 'mystery')),
       current_phase TEXT CHECK(current_phase IN ('setup', 'confrontation', 'resolution')),
       pc_connections JSONB DEFAULT '[]',
       deleted_at TIMESTAMPTZ
   );
   
   CREATE TABLE scene_templates (
       id SERIAL PRIMARY KEY,
       campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
       story_arc_id INTEGER REFERENCES story_arcs(id),
       name TEXT NOT NULL,
       scene_purpose TEXT,
       trigger_conditions JSONB DEFAULT '[]',
       deleted_at TIMESTAMPTZ
   );
   ```

### Step 6: Full-Text Search Implementation
**Duration**: 2 days

1. **Search Infrastructure**
   ```sql
   -- 015_search_infrastructure.sql
   CREATE EXTENSION IF NOT EXISTS pg_trgm;
   
   CREATE OR REPLACE FUNCTION create_search_index(table_name TEXT, columns TEXT[])
   RETURNS VOID AS $$
   DECLARE
       col TEXT;
       search_text TEXT = '';
   BEGIN
       FOREACH col IN ARRAY columns
       LOOP
           search_text := search_text || format('COALESCE(%I::TEXT, '''') || '' '' || ', col);
       END LOOP;
       search_text := rtrim(search_text, ' || '' '' || ');
       
       EXECUTE format('ALTER TABLE %I ADD COLUMN search_text TEXT GENERATED ALWAYS AS (%s) STORED', 
                      table_name, search_text);
       EXECUTE format('CREATE INDEX idx_%I_search ON %I USING gin(search_text gin_trgm_ops)', 
                      table_name, table_name);
   END;
   $$ LANGUAGE plpgsql;
   ```

## Migration Strategy

### Phase 1: Non-Breaking Additions (Week 1)
- Add new tables without modifying existing ones
- Create junction tables for new relationships
- Implement soft delete infrastructure

### Phase 2: Data Migration (Week 2)
- Migrate NPCs to unified characters table
- Populate new fields with sensible defaults
- Create backward-compatibility views

### Phase 3: API Updates (Week 3)
- Update GraphQL schema incrementally
- Maintain old endpoints during transition
- Add new endpoints for new features

### Phase 4: Deprecation (Week 4)
- Mark old tables as deprecated
- Update documentation
- Plan removal timeline

## Testing Strategy

1. **Migration Testing**
   - Test each migration up and down
   - Verify data integrity after migrations
   - Performance test with large datasets

2. **Backward Compatibility**
   - Ensure existing queries still work
   - Test API endpoints remain functional
   - Verify no data loss

3. **Performance Testing**
   - Index effectiveness on large datasets
   - Query performance benchmarks
   - Full-text search performance

## Rollback Plan

1. **Each Migration Reversible**
   - Store rollback SQL with each migration
   - Test rollback procedures
   - Document dependencies

2. **Data Backup Strategy**
   - Backup before each migration phase
   - Point-in-time recovery capability
   - Test restore procedures

## Success Metrics

- All migrations complete without data loss
- Query performance maintains or improves
- Zero downtime during migration
- Full-text search returns results in <100ms
- All existing functionality remains intact

## Next Phase
Once database schema is expanded, proceed to [UPGRADE_PLAN_P2.md](./UPGRADE_PLAN_P2.md) for Backend Model Generation updates.