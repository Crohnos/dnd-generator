-- D&D Campaign Generator - Complete Database Schema
-- Comprehensive world generation system with hierarchical relationships

-- ============================================================================
-- Core Functions and Triggers
-- ============================================================================

-- Create update_updated_at_column function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- ============================================================================
-- Core Campaign Tables
-- ============================================================================

-- Campaigns table
CREATE TABLE campaigns (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    setting TEXT,
    themes TEXT[] DEFAULT '{}',
    player_characters JSONB DEFAULT '[]',
    status TEXT DEFAULT 'created' CHECK (status IN ('created', 'generating', 'completed', 'error')),
    generation_phase TEXT,
    phase_progress INTEGER DEFAULT 0,
    total_phases INTEGER DEFAULT 9, -- Updated for 9-phase system
    current_phase_status TEXT,
    error_message TEXT,
    progression_type TEXT DEFAULT 'milestone',
    tone TEXT,
    difficulty TEXT DEFAULT 'medium',
    starting_level INTEGER DEFAULT 1,
    campaign_length TEXT DEFAULT 'medium',
    additional_notes TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_campaigns_updated_at BEFORE UPDATE
    ON campaigns FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_campaigns_status ON campaigns(status);
CREATE INDEX idx_campaigns_created_at ON campaigns(created_at DESC);

-- ============================================================================
-- Phase 1A: Core World Systems
-- ============================================================================

-- Time & Calendar Systems
CREATE TABLE calendar_systems (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    days_per_week INTEGER DEFAULT 7,
    weeks_per_month INTEGER DEFAULT 4,
    months_per_year INTEGER DEFAULT 12,
    month_names TEXT[],
    day_names TEXT[],
    holidays JSONB DEFAULT '[]',
    current_calendar_date JSONB, -- {year, month, day}
    seasons JSONB DEFAULT '[]',
    special_events JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_calendar_systems_updated_at BEFORE UPDATE
    ON calendar_systems FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Planes of Existence
CREATE TABLE planes (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    plane_type TEXT, -- Material, Elemental, Outer, etc.
    description TEXT,
    characteristics JSONB DEFAULT '{}', -- gravity, time flow, morphic traits
    access_methods TEXT[],
    native_creatures TEXT[],
    planar_traits JSONB DEFAULT '{}',
    alignment_influence TEXT,
    magic_effects TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_planes_updated_at BEFORE UPDATE
    ON planes FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Geography & Regions (hierarchical)
CREATE TABLE geography_regions (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    region_type TEXT, -- continent, kingdom, province, territory, etc.
    parent_region_id INTEGER REFERENCES geography_regions(id),
    plane_id INTEGER REFERENCES planes(id),
    climate TEXT,
    terrain_types TEXT[],
    natural_resources TEXT[],
    native_creatures TEXT[],
    hazards TEXT[],
    description TEXT,
    boundaries JSONB DEFAULT '{}',
    population_density TEXT,
    magical_properties TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_geography_regions_updated_at BEFORE UPDATE
    ON geography_regions FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_geography_regions_campaign_id ON geography_regions(campaign_id);
CREATE INDEX idx_geography_regions_parent ON geography_regions(parent_region_id);

-- Historical Timeline
CREATE TABLE historical_periods (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    period_name TEXT NOT NULL,
    start_year INTEGER NOT NULL,
    end_year INTEGER NOT NULL,
    description TEXT,
    major_events JSONB DEFAULT '[]',
    key_figures TEXT[],
    technological_level TEXT,
    political_structure TEXT,
    dominant_cultures TEXT[],
    significant_wars TEXT[],
    magical_events TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_historical_periods_updated_at BEFORE UPDATE
    ON historical_periods FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Economic Systems
CREATE TABLE economic_systems (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    region_id INTEGER REFERENCES geography_regions(id),
    currency_name TEXT NOT NULL,
    currency_abbreviation TEXT,
    exchange_rates JSONB DEFAULT '{}', -- cp, sp, gp, pp ratios
    trade_goods TEXT[],
    economic_model TEXT, -- feudalism, mercantilism, etc.
    taxation_system JSONB DEFAULT '{}',
    banking_system TEXT,
    guilds TEXT[],
    trade_routes TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_economic_systems_updated_at BEFORE UPDATE
    ON economic_systems FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Legal Systems
CREATE TABLE legal_systems (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    region_id INTEGER REFERENCES geography_regions(id),
    jurisdiction_name TEXT NOT NULL,
    law_type TEXT, -- common law, civil law, divine law, etc.
    enforcement_agency TEXT,
    court_system JSONB DEFAULT '{}',
    punishments JSONB DEFAULT '{}',
    legal_codes TEXT[],
    crime_rates JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_legal_systems_updated_at BEFORE UPDATE
    ON legal_systems FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Astronomy & Zodiac
CREATE TABLE celestial_bodies (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    body_type TEXT, -- sun, moon, planet, star, constellation
    description TEXT,
    orbital_period INTEGER, -- days
    phases TEXT[], -- for moons
    astrological_significance TEXT,
    magical_properties JSONB DEFAULT '{}',
    visibility TEXT[], -- which regions can see it
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_celestial_bodies_updated_at BEFORE UPDATE
    ON celestial_bodies FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- ============================================================================
-- Phase 1B: Character Building Systems
-- ============================================================================

-- Races & Subraces
CREATE TABLE races (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    parent_race_id INTEGER REFERENCES races(id), -- for subraces
    size_category TEXT DEFAULT 'Medium',
    speed INTEGER DEFAULT 30,
    ability_score_increases JSONB DEFAULT '{}',
    racial_traits JSONB DEFAULT '[]',
    languages TEXT[],
    proficiencies JSONB DEFAULT '{}',
    description TEXT,
    cultural_notes TEXT,
    lifespan_years INTEGER,
    physical_description TEXT,
    society_structure TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_races_updated_at BEFORE UPDATE
    ON races FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_races_campaign_id ON races(campaign_id);
CREATE INDEX idx_races_parent ON races(parent_race_id);

-- Classes & Subclasses
CREATE TABLE character_classes (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    parent_class_id INTEGER REFERENCES character_classes(id), -- for subclasses
    hit_die INTEGER DEFAULT 8,
    primary_ability TEXT[],
    saving_throw_proficiencies TEXT[],
    skill_proficiencies JSONB DEFAULT '{}',
    class_features JSONB DEFAULT '[]', -- level-based features
    spellcasting_ability TEXT,
    spell_progression JSONB DEFAULT '{}',
    equipment_proficiencies TEXT[],
    starting_equipment JSONB DEFAULT '[]',
    description TEXT,
    role_description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_character_classes_updated_at BEFORE UPDATE
    ON character_classes FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_character_classes_campaign_id ON character_classes(campaign_id);
CREATE INDEX idx_character_classes_parent ON character_classes(parent_class_id);

-- Feats
CREATE TABLE feats (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    prerequisites JSONB DEFAULT '{}',
    benefits JSONB DEFAULT '[]',
    description TEXT,
    source TEXT,
    feat_type TEXT, -- combat, general, skill, etc.
    repeatable BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_feats_updated_at BEFORE UPDATE
    ON feats FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Backgrounds
CREATE TABLE backgrounds (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    skill_proficiencies TEXT[],
    language_options JSONB DEFAULT '{}',
    tool_proficiencies TEXT[],
    equipment JSONB DEFAULT '[]',
    feature_name TEXT,
    feature_description TEXT,
    suggested_characteristics JSONB DEFAULT '{}',
    description TEXT,
    variants TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_backgrounds_updated_at BEFORE UPDATE
    ON backgrounds FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- ============================================================================
-- Phase 1C: Social Framework
-- ============================================================================

-- Languages
CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    language_type TEXT, -- common, exotic, secret, dead
    script TEXT,
    speakers TEXT[], -- race names that typically speak it
    regions TEXT[], -- where it's commonly spoken
    description TEXT,
    complexity TEXT, -- simple, moderate, complex
    writing_system TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_languages_updated_at BEFORE UPDATE
    ON languages FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Cultures
CREATE TABLE cultures (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    primary_race_id INTEGER REFERENCES races(id),
    geography_region_id INTEGER REFERENCES geography_regions(id),
    values JSONB DEFAULT '[]',
    traditions JSONB DEFAULT '[]',
    social_structure TEXT,
    common_occupations TEXT[],
    typical_alignment TEXT,
    relationship_to_magic TEXT,
    languages TEXT[],
    description TEXT,
    art_forms TEXT[],
    religious_practices TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_cultures_updated_at BEFORE UPDATE
    ON cultures FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Basic Factions
CREATE TABLE factions (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    faction_type TEXT, -- guild, cult, government, military, etc.
    alignment TEXT,
    goals TEXT[],
    methods TEXT[],
    resources TEXT[],
    reputation INTEGER, -- -100 to 100
    secrecy_level TEXT, -- public, semi-secret, secret
    membership_requirements TEXT[],
    ranks JSONB DEFAULT '[]',
    founded_date TEXT,
    current_status TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_factions_updated_at BEFORE UPDATE
    ON factions FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Pantheons
CREATE TABLE pantheons (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    pantheon_type TEXT, -- racial, regional, philosophical
    dominant_alignment TEXT,
    cultural_influence TEXT,
    primary_worshipers TEXT[],
    origin_story TEXT,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_pantheons_updated_at BEFORE UPDATE
    ON pantheons FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Deities
CREATE TABLE deities (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    pantheon_id INTEGER REFERENCES pantheons(id),
    name TEXT NOT NULL,
    title TEXT,
    alignment TEXT NOT NULL,
    domains TEXT[], -- Life, War, Knowledge, etc.
    symbol TEXT,
    holy_day TEXT,
    favored_weapon TEXT,
    divine_rank TEXT, -- Greater, Intermediate, Lesser, Demigod
    worshiper_alignments TEXT[],
    description TEXT,
    dogma TEXT,
    clergy_description TEXT,
    holy_symbol_description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_deities_updated_at BEFORE UPDATE
    ON deities FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- ============================================================================
-- Phase 2A: Core Entities System
-- ============================================================================

-- Core Entities (replaces NPCs table)
CREATE TABLE entities (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    entity_type TEXT NOT NULL, -- pc, npc, creature, monster
    race_id INTEGER REFERENCES races(id),
    class_id INTEGER REFERENCES character_classes(id),
    background_id INTEGER REFERENCES backgrounds(id),
    level_or_cr TEXT, -- "5" for level 5, "1/4" for CR 1/4
    ability_scores JSONB DEFAULT '{}', -- STR, DEX, CON, INT, WIS, CHA
    hit_points INTEGER,
    armor_class INTEGER,
    speed INTEGER,
    skills JSONB DEFAULT '{}',
    saving_throws JSONB DEFAULT '{}',
    damage_resistances TEXT[],
    damage_immunities TEXT[],
    condition_immunities TEXT[],
    senses JSONB DEFAULT '{}',
    languages TEXT[],
    special_abilities JSONB DEFAULT '[]',
    spells_known JSONB DEFAULT '[]',
    personality_traits TEXT[],
    ideals TEXT[],
    bonds TEXT[],
    flaws TEXT[],
    appearance TEXT,
    backstory TEXT,
    motivations TEXT[],
    secrets TEXT[],
    notes TEXT,
    pc_connection_type TEXT, -- ally, enemy, family, mentor, rival, etc.
    pc_connection_description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_entities_updated_at BEFORE UPDATE
    ON entities FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_entities_campaign_id ON entities(campaign_id);
CREATE INDEX idx_entities_type ON entities(entity_type);
CREATE INDEX idx_entities_race ON entities(race_id);

-- ============================================================================
-- Phase 2B: Hierarchical Locations System
-- ============================================================================

-- Core Locations (with hierarchy)
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    location_type TEXT NOT NULL, -- city, district, building, dungeon, wilderness, etc.
    parent_location_id INTEGER REFERENCES locations(id),
    geography_region_id INTEGER REFERENCES geography_regions(id),
    description TEXT,
    population INTEGER,
    government_type TEXT,
    notable_features TEXT[],
    climate TEXT,
    coordinates JSONB, -- {x, y, z} or lat/long
    size_category TEXT, -- tiny, small, medium, large, huge
    accessibility TEXT[], -- road, river, portal, etc.
    security_level TEXT,
    wealth_level TEXT,
    pc_significance TEXT, -- why this location matters to PCs
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_locations_updated_at BEFORE UPDATE
    ON locations FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_locations_campaign_id ON locations(campaign_id);
CREATE INDEX idx_locations_parent ON locations(parent_location_id);
CREATE INDEX idx_locations_type ON locations(location_type);

-- Dungeons (special location type)
CREATE TABLE dungeons (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    dungeon_type TEXT, -- cave, ruins, tower, temple, etc.
    levels INTEGER DEFAULT 1,
    difficulty_rating TEXT,
    primary_inhabitants TEXT[],
    traps_density TEXT,
    treasure_level TEXT,
    history TEXT,
    entrance_conditions TEXT[],
    layout_description TEXT,
    boss_encounters TEXT[],
    environmental_hazards TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Buildings (special location type)
CREATE TABLE buildings (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    building_type TEXT, -- shop, tavern, temple, residence, etc.
    floors INTEGER DEFAULT 1,
    capacity INTEGER,
    condition_state TEXT,
    architectural_style TEXT,
    special_features TEXT[],
    services_offered TEXT[],
    operating_hours TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Shops
CREATE TABLE shops (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    building_id INTEGER NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    shop_type TEXT NOT NULL,
    owner_entity_id INTEGER REFERENCES entities(id),
    specialties TEXT[],
    inventory_level TEXT,
    price_modifier NUMERIC(3,2) DEFAULT 1.0,
    reputation TEXT,
    special_services TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Taverns
CREATE TABLE taverns (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    building_id INTEGER NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    owner_entity_id INTEGER REFERENCES entities(id),
    atmosphere TEXT,
    specialties TEXT[],
    room_rates JSONB DEFAULT '{}',
    entertainment TEXT[],
    regular_clientele TEXT,
    rumors_available TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Temples
CREATE TABLE temples (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    building_id INTEGER NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    deity_id INTEGER REFERENCES deities(id),
    high_priest_entity_id INTEGER REFERENCES entities(id),
    services TEXT[],
    holy_days JSONB DEFAULT '[]',
    relics TEXT[],
    congregation_size TEXT,
    influence_level TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Phase 2C: Items & Equipment System
-- ============================================================================

-- Comprehensive Items
CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    item_type TEXT NOT NULL, -- weapon, armor, tool, consumable, treasure, etc.
    item_subtype TEXT, -- longsword, chain mail, thieves' tools, etc.
    rarity TEXT DEFAULT 'common',
    is_magical BOOLEAN DEFAULT FALSE,
    is_sentient BOOLEAN DEFAULT FALSE,
    requires_attunement BOOLEAN DEFAULT FALSE,
    description TEXT,
    weight_pounds NUMERIC(8,2),
    value_gp NUMERIC(10,2),
    damage_dice TEXT, -- for weapons
    armor_class INTEGER, -- for armor
    properties TEXT[], -- versatile, finesse, heavy, etc.
    pc_significance TEXT, -- why this item matters to PCs
    history TEXT,
    creator TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_items_updated_at BEFORE UPDATE
    ON items FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_items_campaign_id ON items(campaign_id);
CREATE INDEX idx_items_type ON items(item_type);
CREATE INDEX idx_items_rarity ON items(rarity);

-- Item Effects & Abilities
CREATE TABLE item_effects (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    effect_type TEXT NOT NULL, -- spell, ability, passive, curse
    effect_name TEXT NOT NULL,
    description TEXT,
    activation_method TEXT, -- action, bonus action, reaction, passive
    uses_per_day INTEGER,
    recharge_condition TEXT,
    spell_level INTEGER,
    save_dc INTEGER,
    damage_dice TEXT,
    range_feet INTEGER,
    duration TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Sentient Item Properties
CREATE TABLE sentient_item_properties (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    intelligence INTEGER NOT NULL,
    wisdom INTEGER NOT NULL,
    charisma INTEGER NOT NULL,
    alignment TEXT,
    languages TEXT[],
    senses TEXT[],
    personality TEXT,
    communication_methods TEXT[],
    special_purpose TEXT,
    conflict_conditions TEXT[],
    ego_score INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Phase 3A: Quest Hooks & Encounters
-- ============================================================================

-- Quest hooks table
CREATE TABLE quest_hooks (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    quest_type TEXT, -- main, side, personal, faction
    difficulty TEXT DEFAULT 'medium',
    estimated_sessions INTEGER,
    reward TEXT,
    related_entity_ids INTEGER[] DEFAULT '{}',
    related_location_ids INTEGER[] DEFAULT '{}',
    prerequisites TEXT[],
    consequences TEXT[],
    status TEXT DEFAULT 'available' CHECK (status IN ('available', 'active', 'completed')),
    pc_hook_type TEXT, -- personal, moral, reward, threat, mystery
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_quest_hooks_updated_at BEFORE UPDATE
    ON quest_hooks FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_quest_hooks_campaign_id ON quest_hooks(campaign_id);
CREATE INDEX idx_quest_hooks_status ON quest_hooks(status);

-- Encounters table
CREATE TABLE encounters (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    location_id INTEGER REFERENCES locations(id) ON DELETE SET NULL,
    encounter_type TEXT, -- combat, social, exploration, puzzle
    name TEXT NOT NULL,
    description TEXT,
    difficulty TEXT DEFAULT 'medium',
    enemies JSONB DEFAULT '[]',
    environmental_factors TEXT[],
    treasure TEXT[],
    experience_reward INTEGER,
    trigger_conditions TEXT[],
    resolution_options JSONB DEFAULT '[]',
    scaling_notes TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_encounters_updated_at BEFORE UPDATE
    ON encounters FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_encounters_campaign_id ON encounters(campaign_id);
CREATE INDEX idx_encounters_location_id ON encounters(location_id);

-- ============================================================================
-- Relationship Tables (Many-to-Many)
-- ============================================================================

-- Entity Relationships
CREATE TABLE entity_relationships (
    id SERIAL PRIMARY KEY,
    entity1_id INTEGER NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    entity2_id INTEGER NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    relationship_type TEXT NOT NULL, -- family, friend, enemy, ally, romantic, etc.
    relationship_strength INTEGER, -- -100 to 100
    description TEXT,
    is_mutual BOOLEAN DEFAULT TRUE,
    status TEXT DEFAULT 'active', -- active, former, complicated
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT different_entities CHECK (entity1_id != entity2_id)
);

CREATE TRIGGER update_entity_relationships_updated_at BEFORE UPDATE
    ON entity_relationships FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_entity_relationships_entity1 ON entity_relationships(entity1_id);
CREATE INDEX idx_entity_relationships_entity2 ON entity_relationships(entity2_id);

-- Entity-Location Associations
CREATE TABLE entity_locations (
    id SERIAL PRIMARY KEY,
    entity_id INTEGER NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    association_type TEXT NOT NULL, -- lives_in, works_at, owns, frequents, etc.
    duration TEXT, -- permanent, temporary, seasonal
    status TEXT DEFAULT 'current', -- current, former, planned
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_entity_locations_updated_at BEFORE UPDATE
    ON entity_locations FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_entity_locations_entity ON entity_locations(entity_id);
CREATE INDEX idx_entity_locations_location ON entity_locations(location_id);

-- Entity-Faction Memberships
CREATE TABLE entity_factions (
    id SERIAL PRIMARY KEY,
    entity_id INTEGER NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    faction_id INTEGER NOT NULL REFERENCES factions(id) ON DELETE CASCADE,
    rank TEXT,
    status TEXT DEFAULT 'active', -- active, inactive, expelled, undercover
    joined_date DATE,
    influence_level INTEGER, -- 1-10
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_entity_factions_updated_at BEFORE UPDATE
    ON entity_factions FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_entity_factions_entity ON entity_factions(entity_id);
CREATE INDEX idx_entity_factions_faction ON entity_factions(faction_id);

-- Entity-Item Ownership
CREATE TABLE entity_items (
    id SERIAL PRIMARY KEY,
    entity_id INTEGER NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    ownership_type TEXT, -- owns, carries, attuned, stolen, borrowed
    quantity INTEGER DEFAULT 1,
    condition_state TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_entity_items_updated_at BEFORE UPDATE
    ON entity_items FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_entity_items_entity ON entity_items(entity_id);
CREATE INDEX idx_entity_items_item ON entity_items(item_id);

-- Location-Item Placement
CREATE TABLE location_items (
    id SERIAL PRIMARY KEY,
    location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    placement_type TEXT, -- hidden, displayed, for_sale, treasure, stored
    quantity INTEGER DEFAULT 1,
    condition_to_access TEXT,
    discovered BOOLEAN DEFAULT FALSE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_location_items_updated_at BEFORE UPDATE
    ON location_items FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_location_items_location ON location_items(location_id);
CREATE INDEX idx_location_items_item ON location_items(item_id);

-- Quest-Entity Relationships
CREATE TABLE quest_entities (
    id SERIAL PRIMARY KEY,
    quest_hook_id INTEGER NOT NULL REFERENCES quest_hooks(id) ON DELETE CASCADE,
    entity_id INTEGER NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    role_in_quest TEXT, -- questgiver, ally, enemy, target, contact
    importance_level TEXT, -- critical, important, optional
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_quest_entities_quest ON quest_entities(quest_hook_id);
CREATE INDEX idx_quest_entities_entity ON quest_entities(entity_id);

-- Quest-Location Relationships
CREATE TABLE quest_locations (
    id SERIAL PRIMARY KEY,
    quest_hook_id INTEGER NOT NULL REFERENCES quest_hooks(id) ON DELETE CASCADE,
    location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    role_in_quest TEXT, -- start, destination, midpoint, optional
    importance_level TEXT, -- critical, important, optional
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_quest_locations_quest ON quest_locations(quest_hook_id);
CREATE INDEX idx_quest_locations_location ON quest_locations(location_id);

-- Faction Relationships (faction-to-faction)
CREATE TABLE faction_relationships (
    id SERIAL PRIMARY KEY,
    faction1_id INTEGER NOT NULL REFERENCES factions(id) ON DELETE CASCADE,
    faction2_id INTEGER NOT NULL REFERENCES factions(id) ON DELETE CASCADE,
    relationship_type TEXT NOT NULL, -- allied, enemy, neutral, suspicious, trading
    relationship_strength INTEGER, -- -100 to 100
    description TEXT,
    history TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT different_factions CHECK (faction1_id != faction2_id)
);

CREATE TRIGGER update_faction_relationships_updated_at BEFORE UPDATE
    ON faction_relationships FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

CREATE INDEX idx_faction_relationships_faction1 ON faction_relationships(faction1_id);
CREATE INDEX idx_faction_relationships_faction2 ON faction_relationships(faction2_id);

-- Race-Culture Associations
CREATE TABLE race_cultures (
    id SERIAL PRIMARY KEY,
    race_id INTEGER NOT NULL REFERENCES races(id) ON DELETE CASCADE,
    culture_id INTEGER NOT NULL REFERENCES cultures(id) ON DELETE CASCADE,
    prevalence TEXT, -- dominant, common, minority, rare
    regional_variations TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_race_cultures_race ON race_cultures(race_id);
CREATE INDEX idx_race_cultures_culture ON race_cultures(culture_id);

-- ============================================================================
-- Additional Indexes for Performance
-- ============================================================================

-- Composite indexes for common queries
CREATE INDEX idx_entities_campaign_type ON entities(campaign_id, entity_type);
CREATE INDEX idx_locations_campaign_type ON locations(campaign_id, location_type);
CREATE INDEX idx_items_campaign_type_rarity ON items(campaign_id, item_type, rarity);
CREATE INDEX idx_quest_hooks_campaign_status ON quest_hooks(campaign_id, status);
CREATE INDEX idx_encounters_campaign_type ON encounters(campaign_id, encounter_type);

-- ============================================================================
-- Views for Common Queries
-- ============================================================================

-- View for PC-connected NPCs with location info
CREATE VIEW pc_connected_npcs AS
SELECT 
    e.*,
    el.location_id,
    l.name as location_name
FROM entities e
LEFT JOIN entity_locations el ON e.id = el.entity_id AND el.association_type = 'lives_in'
LEFT JOIN locations l ON el.location_id = l.id
WHERE e.entity_type = 'npc' AND e.pc_connection_type IS NOT NULL;

-- View for faction hierarchies
CREATE VIEW faction_hierarchies AS
SELECT 
    f1.id as faction_id,
    f1.name as faction_name,
    f2.id as related_faction_id,
    f2.name as related_faction_name,
    fr.relationship_type,
    fr.relationship_strength
FROM factions f1
JOIN faction_relationships fr ON f1.id = fr.faction1_id
JOIN factions f2 ON fr.faction2_id = f2.id;

-- View for location hierarchies
CREATE VIEW location_hierarchies AS
WITH RECURSIVE location_tree AS (
    -- Base case: top-level locations
    SELECT id, name, location_type, parent_location_id, 1 as level, 
           ARRAY[id] as path, name as full_path
    FROM locations 
    WHERE parent_location_id IS NULL
    
    UNION ALL
    
    -- Recursive case: child locations
    SELECT l.id, l.name, l.location_type, l.parent_location_id, 
           lt.level + 1, lt.path || l.id, 
           lt.full_path || ' > ' || l.name
    FROM locations l
    JOIN location_tree lt ON l.parent_location_id = lt.id
)
SELECT * FROM location_tree;

-- ============================================================================
-- Additional Indexes for Campaign Isolation
-- ============================================================================

-- Indexes for new campaign_id columns to ensure fast filtering by campaign
CREATE INDEX idx_buildings_campaign_id ON buildings(campaign_id);
CREATE INDEX idx_dungeons_campaign_id ON dungeons(campaign_id);
CREATE INDEX idx_shops_campaign_id ON shops(campaign_id);
CREATE INDEX idx_taverns_campaign_id ON taverns(campaign_id);
CREATE INDEX idx_temples_campaign_id ON temples(campaign_id);