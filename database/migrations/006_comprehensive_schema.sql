-- 006_comprehensive_schema.sql
-- Comprehensive schema expansion to 100+ tables for deep world building

-- ===================================================================
-- CALENDAR AND TIME SYSTEMS
-- ===================================================================

-- Calendar systems for different campaign worlds
CREATE TABLE calendar_systems (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    months JSONB NOT NULL DEFAULT '[]', -- Array of month objects with names, days, seasons
    weekdays JSONB NOT NULL DEFAULT '[]', -- Array of weekday names
    year_length INTEGER DEFAULT 365,
    current_year INTEGER DEFAULT 1,
    current_month INTEGER DEFAULT 1,
    current_day INTEGER DEFAULT 1,
    special_events JSONB DEFAULT '[]', -- Holidays, festivals, etc.
    lunar_cycles JSONB DEFAULT '{}', -- Moon phases and cycles
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_calendar_systems_updated_at BEFORE UPDATE
    ON calendar_systems FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Historical events and timeline
CREATE TABLE historical_events (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    calendar_system_id INTEGER REFERENCES calendar_systems(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    description TEXT,
    event_type TEXT DEFAULT 'general', -- war, founding, disaster, discovery, etc.
    year INTEGER,
    month INTEGER,
    day INTEGER,
    duration_days INTEGER DEFAULT 1,
    importance_level INTEGER DEFAULT 5 CHECK (importance_level BETWEEN 1 AND 10),
    participants JSONB DEFAULT '[]', -- References to characters, organizations, locations
    consequences TEXT,
    public_knowledge BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_historical_events_updated_at BEFORE UPDATE
    ON historical_events FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- ENHANCED LOCATION SYSTEM
-- ===================================================================

-- Location types and categories
CREATE TABLE location_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL, -- settlement, wilderness, structure, plane, etc.
    default_properties JSONB DEFAULT '{}',
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert default location types
INSERT INTO location_types (name, category, default_properties) VALUES
('City', 'settlement', '{"population_range": "10000+", "government": true, "trade": true}'),
('Town', 'settlement', '{"population_range": "1000-10000", "government": true}'),
('Village', 'settlement', '{"population_range": "100-1000", "government": false}'),
('Hamlet', 'settlement', '{"population_range": "20-100", "government": false}'),
('Tavern', 'structure', '{"services": ["food", "lodging", "rumors"]}'),
('Temple', 'structure', '{"services": ["healing", "blessing", "information"]}'),
('Dungeon', 'structure', '{"danger_level": "variable", "treasure": true}'),
('Forest', 'wilderness', '{"travel_difficulty": "medium", "resources": ["wood", "game"]}'),
('Mountain', 'wilderness', '{"travel_difficulty": "hard", "resources": ["minerals", "stone"]}'),
('Plains', 'wilderness', '{"travel_difficulty": "easy", "resources": ["agriculture"]}');

-- Enhanced locations table with hierarchical structure
ALTER TABLE locations ADD COLUMN location_type_id INTEGER REFERENCES location_types(id);
ALTER TABLE locations ADD COLUMN parent_location_id INTEGER REFERENCES locations(id);
ALTER TABLE locations ADD COLUMN population INTEGER;
ALTER TABLE locations ADD COLUMN government_type TEXT;
ALTER TABLE locations ADD COLUMN economy JSONB DEFAULT '{}';
ALTER TABLE locations ADD COLUMN climate TEXT;
ALTER TABLE locations ADD COLUMN terrain TEXT;
ALTER TABLE locations ADD COLUMN danger_level INTEGER DEFAULT 1 CHECK (danger_level BETWEEN 1 AND 10);
ALTER TABLE locations ADD COLUMN notable_features TEXT[];
ALTER TABLE locations ADD COLUMN resources TEXT[];
ALTER TABLE locations ADD COLUMN travel_time_modifiers JSONB DEFAULT '{}';
ALTER TABLE locations ADD COLUMN is_secret BOOLEAN DEFAULT false;
ALTER TABLE locations ADD COLUMN discovery_requirements TEXT;

-- Location connections with detailed travel information
CREATE TABLE location_connections (
    id SERIAL PRIMARY KEY,
    from_location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    to_location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    connection_type TEXT DEFAULT 'road', -- road, path, river, teleporter, etc.
    distance_miles DECIMAL(10,2),
    travel_time_hours DECIMAL(10,2),
    difficulty TEXT DEFAULT 'normal', -- easy, normal, hard, extreme
    dangers TEXT[],
    requirements TEXT, -- what's needed to traverse this connection
    toll_cost INTEGER DEFAULT 0,
    is_secret BOOLEAN DEFAULT false,
    seasonal_availability TEXT[], -- which seasons this route is available
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(from_location_id, to_location_id)
);

CREATE TRIGGER update_location_connections_updated_at BEFORE UPDATE
    ON location_connections FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Location services and amenities
CREATE TABLE location_services (
    id SERIAL PRIMARY KEY,
    location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    service_type TEXT NOT NULL, -- inn, shop, temple, guild, etc.
    name TEXT NOT NULL,
    description TEXT,
    quality_level INTEGER DEFAULT 3 CHECK (quality_level BETWEEN 1 AND 5),
    services_offered TEXT[],
    prices JSONB DEFAULT '{}',
    availability_schedule TEXT,
    owner_npc_id INTEGER REFERENCES npcs(id) ON DELETE SET NULL,
    reputation INTEGER DEFAULT 0, -- -10 to +10
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_location_services_updated_at BEFORE UPDATE
    ON location_services FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- ENHANCED CHARACTER SYSTEM
-- ===================================================================

-- Character races and ethnicities
CREATE TABLE character_races (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    size_category TEXT DEFAULT 'Medium',
    average_lifespan INTEGER,
    common_alignments TEXT[],
    physical_traits TEXT[],
    cultural_traits TEXT[],
    languages TEXT[],
    subraces JSONB DEFAULT '[]',
    homeland_locations INTEGER[], -- references to locations
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Character backgrounds and professions
CREATE TABLE character_backgrounds (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    skill_proficiencies TEXT[],
    tool_proficiencies TEXT[],
    languages_known INTEGER DEFAULT 0,
    equipment JSONB DEFAULT '{}',
    feature_name TEXT,
    feature_description TEXT,
    personality_traits TEXT[],
    ideals TEXT[],
    bonds TEXT[],
    flaws TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Enhanced NPCs with deep characterization
ALTER TABLE npcs ADD COLUMN race_id INTEGER REFERENCES character_races(id);
ALTER TABLE npcs ADD COLUMN background_id INTEGER REFERENCES character_backgrounds(id);
ALTER TABLE npcs ADD COLUMN age INTEGER;
ALTER TABLE npcs ADD COLUMN physical_description TEXT;
ALTER TABLE npcs ADD COLUMN alignment TEXT;
ALTER TABLE npcs ADD COLUMN social_class TEXT; -- noble, merchant, commoner, criminal, etc.
ALTER TABLE npcs ADD COLUMN occupation TEXT;
ALTER TABLE npcs ADD COLUMN languages TEXT[];
ALTER TABLE npcs ADD COLUMN reputation INTEGER DEFAULT 0; -- -10 to +10
ALTER TABLE npcs ADD COLUMN wealth_level INTEGER DEFAULT 3 CHECK (wealth_level BETWEEN 1 AND 5);
ALTER TABLE npcs ADD COLUMN health_status TEXT DEFAULT 'healthy';
ALTER TABLE npcs ADD COLUMN current_location_id INTEGER REFERENCES locations(id);
ALTER TABLE npcs ADD COLUMN home_location_id INTEGER REFERENCES locations(id);
ALTER TABLE npcs ADD COLUMN family_members INTEGER[]; -- references to other NPCs
ALTER TABLE npcs ADD COLUMN allies INTEGER[]; -- references to other NPCs
ALTER TABLE npcs ADD COLUMN enemies INTEGER[]; -- references to other NPCs
ALTER TABLE npcs ADD COLUMN goals TEXT[];
ALTER TABLE npcs ADD COLUMN fears TEXT[];
ALTER TABLE npcs ADD COLUMN secrets TEXT[];
ALTER TABLE npcs ADD COLUMN schedule JSONB DEFAULT '{}'; -- daily/weekly routines
ALTER TABLE npcs ADD COLUMN is_dead BOOLEAN DEFAULT false;
ALTER TABLE npcs ADD COLUMN death_date DATE;
ALTER TABLE npcs ADD COLUMN death_cause TEXT;

-- NPC relationships matrix
CREATE TABLE npc_relationships (
    id SERIAL PRIMARY KEY,
    npc1_id INTEGER NOT NULL REFERENCES npcs(id) ON DELETE CASCADE,
    npc2_id INTEGER NOT NULL REFERENCES npcs(id) ON DELETE CASCADE,
    relationship_type TEXT NOT NULL, -- family, friend, enemy, romantic, business, etc.
    relationship_strength INTEGER DEFAULT 0 CHECK (relationship_strength BETWEEN -10 AND 10),
    description TEXT,
    is_secret BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(npc1_id, npc2_id),
    CHECK (npc1_id != npc2_id)
);

CREATE TRIGGER update_npc_relationships_updated_at BEFORE UPDATE
    ON npc_relationships FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- ORGANIZATIONS AND FACTIONS
-- ===================================================================

-- Organization types and structures
CREATE TABLE organization_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    typical_structure TEXT, -- hierarchical, democratic, anarchic, etc.
    common_goals TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO organization_types (name, description, typical_structure, common_goals) VALUES
('Guild', 'Professional trade organizations', 'hierarchical', ARRAY['profit', 'member protection', 'skill advancement']),
('Religious Order', 'Faith-based organizations', 'hierarchical', ARRAY['spread faith', 'protect believers', 'fight evil']),
('Noble House', 'Aristocratic families', 'hereditary', ARRAY['maintain power', 'increase wealth', 'family honor']),
('Criminal Organization', 'Illegal operations', 'hierarchical', ARRAY['profit', 'territory control', 'avoid law']),
('Military Unit', 'Armed forces', 'strict hierarchy', ARRAY['defend territory', 'follow orders', 'honor']),
('Scholarly Circle', 'Academic and research groups', 'democratic', ARRAY['knowledge', 'discovery', 'teaching']);

-- Organizations and factions
CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    organization_type_id INTEGER REFERENCES organization_types(id),
    name TEXT NOT NULL,
    short_name TEXT, -- acronym or common abbreviation
    description TEXT,
    founding_date DATE,
    headquarters_location_id INTEGER REFERENCES locations(id),
    territory_locations INTEGER[], -- array of location IDs under influence
    size_category TEXT DEFAULT 'small', -- tiny, small, medium, large, massive
    influence_level INTEGER DEFAULT 3 CHECK (influence_level BETWEEN 1 AND 10),
    wealth_level INTEGER DEFAULT 3 CHECK (wealth_level BETWEEN 1 AND 5),
    secrecy_level INTEGER DEFAULT 1 CHECK (secrecy_level BETWEEN 1 AND 5), -- how secret the org is
    alignment TEXT,
    primary_goals TEXT[],
    methods TEXT[], -- how they achieve their goals
    resources TEXT[], -- what they have access to
    symbols_and_colors JSONB DEFAULT '{}',
    motto TEXT,
    reputation INTEGER DEFAULT 0, -- -10 to +10
    is_active BOOLEAN DEFAULT true,
    dissolution_date DATE,
    dissolution_reason TEXT,
    parent_organization_id INTEGER REFERENCES organizations(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_organizations_updated_at BEFORE UPDATE
    ON organizations FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Organization hierarchy and ranks
CREATE TABLE organization_ranks (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    rank_name TEXT NOT NULL,
    rank_level INTEGER NOT NULL, -- 1 = highest, higher numbers = lower ranks
    description TEXT,
    responsibilities TEXT[],
    privileges TEXT[],
    requirements TEXT[],
    typical_salary INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(organization_id, rank_level)
);

-- Organization memberships
CREATE TABLE organization_memberships (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    npc_id INTEGER NOT NULL REFERENCES npcs(id) ON DELETE CASCADE,
    rank_id INTEGER REFERENCES organization_ranks(id),
    join_date DATE NOT NULL,
    leave_date DATE,
    membership_status TEXT DEFAULT 'active', -- active, inactive, expelled, retired
    loyalty_level INTEGER DEFAULT 5 CHECK (loyalty_level BETWEEN 1 AND 10),
    contribution_level INTEGER DEFAULT 3 CHECK (contribution_level BETWEEN 1 AND 5),
    special_roles TEXT[], -- what special functions they serve
    access_level INTEGER DEFAULT 1 CHECK (access_level BETWEEN 1 AND 5), -- security clearance
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(organization_id, npc_id)
);

CREATE TRIGGER update_organization_memberships_updated_at BEFORE UPDATE
    ON organization_memberships FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Inter-organization relationships
CREATE TABLE organization_relationships (
    id SERIAL PRIMARY KEY,
    organization1_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    organization2_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    relationship_type TEXT NOT NULL, -- allied, enemy, neutral, trade_partner, subsidiary, etc.
    relationship_strength INTEGER DEFAULT 0 CHECK (relationship_strength BETWEEN -10 AND 10),
    description TEXT,
    formal_agreement TEXT, -- treaty, contract, etc.
    start_date DATE,
    end_date DATE,
    is_secret BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(organization1_id, organization2_id),
    CHECK (organization1_id != organization2_id)
);

CREATE TRIGGER update_organization_relationships_updated_at BEFORE UPDATE
    ON organization_relationships FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- BACKSTORY ELEMENTS SYSTEM
-- ===================================================================

-- Player character backstory elements
CREATE TABLE backstory_element_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    examples TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO backstory_element_types (name, description, examples) VALUES
('Family Member', 'Relatives and family connections', ARRAY['parent', 'sibling', 'cousin', 'spouse', 'child']),
('Mentor/Teacher', 'People who taught or guided the character', ARRAY['master craftsman', 'academy professor', 'spiritual guide']),
('Friend/Ally', 'Close personal relationships and allies', ARRAY['childhood friend', 'fellow adventurer', 'trusted confidant']),
('Enemy/Rival', 'Antagonistic relationships', ARRAY['childhood bully', 'professional rival', 'sworn enemy']),
('Organization', 'Groups the character has connections to', ARRAY['former guild', 'military unit', 'criminal organization']),
('Location', 'Important places from the character backstory', ARRAY['hometown', 'training ground', 'site of tragedy']),
('Event', 'Significant occurrences in the character past', ARRAY['great victory', 'tragic loss', 'life-changing discovery']),
('Item/Heirloom', 'Important possessions with history', ARRAY['family sword', 'mysterious artifact', 'treasured memento']),
('Secret', 'Hidden aspects of the character past', ARRAY['hidden identity', 'shameful act', 'forbidden knowledge']),
('Goal/Motivation', 'What drives the character', ARRAY['seek revenge', 'find family', 'prove worthiness']);

-- Player character backstory elements
CREATE TABLE backstory_elements (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    element_type_id INTEGER NOT NULL REFERENCES backstory_element_types(id),
    player_character_name TEXT NOT NULL,
    element_name TEXT NOT NULL,
    description TEXT NOT NULL,
    importance_level INTEGER DEFAULT 5 CHECK (importance_level BETWEEN 1 AND 10),
    current_status TEXT DEFAULT 'unresolved', -- resolved, unresolved, ongoing
    connected_npc_id INTEGER REFERENCES npcs(id),
    connected_location_id INTEGER REFERENCES locations(id),
    connected_organization_id INTEGER REFERENCES organizations(id),
    connected_quest_hook_id INTEGER REFERENCES quest_hooks(id),
    integration_notes TEXT, -- how this has been woven into the campaign
    player_notes TEXT, -- private notes from the player
    dm_notes TEXT, -- private notes for the DM
    is_secret BOOLEAN DEFAULT false, -- hidden from other players
    reveal_trigger TEXT, -- what would reveal this secret
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_backstory_elements_updated_at BEFORE UPDATE
    ON backstory_elements FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- ENHANCED QUEST AND PLOT SYSTEM
-- ===================================================================

-- Quest types and categories
CREATE TABLE quest_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    typical_rewards TEXT[],
    common_complications TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO quest_types (name, description, typical_rewards, common_complications) VALUES
('Rescue Mission', 'Save someone or something', ARRAY['gratitude', 'payment', 'information'], ARRAY['time limit', 'hostage situation', 'false identity']),
('Investigation', 'Uncover truth or solve mystery', ARRAY['knowledge', 'allies', 'reputation'], ARRAY['false leads', 'cover-up', 'dangerous truth']),
('Delivery', 'Transport item or message', ARRAY['payment', 'safe passage', 'contacts'], ARRAY['bandits', 'wrong recipient', 'fragile cargo']),
('Elimination', 'Remove threat or target', ARRAY['bounty', 'territory', 'peace'], ARRAY['innocent bystanders', 'moral ambiguity', 'stronger than expected']),
('Acquisition', 'Obtain specific item or resource', ARRAY['item itself', 'payment', 'access'], ARRAY['heavy guard', 'cursed item', 'moral cost']),
('Diplomacy', 'Negotiate or mediate', ARRAY['peace treaty', 'trade rights', 'influence'], ARRAY['extremist factions', 'sabotage', 'cultural misunderstanding']);

-- Enhanced quest hooks
ALTER TABLE quest_hooks ADD COLUMN quest_type_id INTEGER REFERENCES quest_types(id);
ALTER TABLE quest_hooks ADD COLUMN patron_npc_id INTEGER REFERENCES npcs(id);
ALTER TABLE quest_hooks ADD COLUMN patron_organization_id INTEGER REFERENCES organizations(id);
ALTER TABLE quest_hooks ADD COLUMN target_location_id INTEGER REFERENCES locations(id);
ALTER TABLE quest_hooks ADD COLUMN time_limit_days INTEGER;
ALTER TABLE quest_hooks ADD COLUMN urgency_level INTEGER DEFAULT 3 CHECK (urgency_level BETWEEN 1 AND 5);
ALTER TABLE quest_hooks ADD COLUMN secrecy_level INTEGER DEFAULT 1 CHECK (secrecy_level BETWEEN 1 AND 5);
ALTER TABLE quest_hooks ADD COLUMN moral_complexity INTEGER DEFAULT 3 CHECK (moral_complexity BETWEEN 1 AND 5);
ALTER TABLE quest_hooks ADD COLUMN required_skills TEXT[];
ALTER TABLE quest_hooks ADD COLUMN complications TEXT[];
ALTER TABLE quest_hooks ADD COLUMN success_consequences TEXT;
ALTER TABLE quest_hooks ADD COLUMN failure_consequences TEXT;
ALTER TABLE quest_hooks ADD COLUMN backstory_connections INTEGER[]; -- references to backstory_elements
ALTER TABLE quest_hooks ADD COLUMN prerequisite_quests INTEGER[]; -- references to other quest_hooks

-- Quest progress tracking
CREATE TABLE quest_progress (
    id SERIAL PRIMARY KEY,
    quest_hook_id INTEGER NOT NULL REFERENCES quest_hooks(id) ON DELETE CASCADE,
    session_number INTEGER,
    progress_description TEXT NOT NULL,
    completion_percentage INTEGER DEFAULT 0 CHECK (completion_percentage BETWEEN 0 AND 100),
    complications_encountered TEXT[],
    npcs_met INTEGER[],
    locations_visited INTEGER[],
    items_gained TEXT[],
    information_learned TEXT[],
    decisions_made JSONB DEFAULT '{}',
    next_steps TEXT[],
    dm_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ===================================================================
-- ENHANCED ENCOUNTERS AND EVENTS
-- ===================================================================

-- Encounter types and categories
CREATE TABLE encounter_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL, -- combat, social, exploration, puzzle, etc.
    description TEXT,
    typical_duration TEXT,
    common_resolutions TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO encounter_types (name, category, description, typical_duration, common_resolutions) VALUES
('Combat - Bandits', 'combat', 'Highway robbery or territorial defense', '10-30 minutes', ARRAY['defeat enemies', 'negotiate passage', 'flee']),
('Combat - Wildlife', 'combat', 'Aggressive or territorial animals', '5-20 minutes', ARRAY['defeat creature', 'scare off', 'animal handling']),
('Social - Negotiation', 'social', 'Diplomatic discussion or bargaining', '20-60 minutes', ARRAY['successful deal', 'compromise', 'breakdown']),
('Social - Investigation', 'social', 'Gathering information through conversation', '15-45 minutes', ARRAY['learn truth', 'partial information', 'misdirection']),
('Exploration - Trap', 'exploration', 'Hidden dangers in environment', '5-15 minutes', ARRAY['disarm safely', 'trigger but survive', 'avoid entirely']),
('Exploration - Discovery', 'exploration', 'Finding something significant', '10-30 minutes', ARRAY['full revelation', 'partial clues', 'miss opportunity']),
('Puzzle - Riddle', 'puzzle', 'Mental challenge requiring lateral thinking', '15-45 minutes', ARRAY['solve correctly', 'creative solution', 'give up']),
('Puzzle - Mechanical', 'puzzle', 'Physical manipulation challenge', '10-30 minutes', ARRAY['unlock mechanism', 'force through', 'find alternative']);

-- Enhanced encounters
ALTER TABLE encounters ADD COLUMN encounter_type_id INTEGER REFERENCES encounter_types(id);
ALTER TABLE encounters ADD COLUMN trigger_conditions TEXT[];
ALTER TABLE encounters ADD COLUMN npcs_involved INTEGER[];
ALTER TABLE encounters ADD COLUMN organizations_involved INTEGER[];
ALTER TABLE encounters ADD COLUMN required_skills TEXT[];
ALTER TABLE encounters ADD COLUMN possible_outcomes JSONB DEFAULT '{}';
ALTER TABLE encounters ADD COLUMN rewards JSONB DEFAULT '{}';
ALTER TABLE encounters ADD COLUMN consequences TEXT[];
ALTER TABLE encounters ADD COLUMN backstory_relevance INTEGER[]; -- references to backstory_elements
ALTER TABLE encounters ADD COLUMN repeatable BOOLEAN DEFAULT false;
ALTER TABLE encounters ADD COLUMN scaling_notes TEXT; -- how to adjust for party level

-- Random events and occurrences
CREATE TABLE random_events (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    event_type TEXT DEFAULT 'minor', -- minor, major, catastrophic
    trigger_probability DECIMAL(3,2) DEFAULT 0.1, -- 0.0 to 1.0
    location_types TEXT[], -- where this can occur
    seasonal_restrictions TEXT[], -- when this can occur
    prerequisites TEXT[], -- what must be true for this to happen
    immediate_effects TEXT[],
    long_term_consequences TEXT[],
    affected_locations INTEGER[],
    affected_npcs INTEGER[],
    affected_organizations INTEGER[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_random_events_updated_at BEFORE UPDATE
    ON random_events FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- ECONOMY AND TRADE SYSTEM
-- ===================================================================

-- Currencies and monetary systems
CREATE TABLE currencies (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    abbreviation TEXT,
    base_unit TEXT, -- copper, gold, etc.
    exchange_rate DECIMAL(10,4) DEFAULT 1.0, -- rate to campaign's base currency
    regions_used INTEGER[], -- location IDs where this currency is accepted
    issuing_authority TEXT, -- kingdom, empire, etc.
    physical_description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trade goods and commodities
CREATE TABLE trade_goods (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL, -- food, raw_materials, crafted_goods, luxuries, etc.
    base_value DECIMAL(10,2),
    weight_pounds DECIMAL(10,2),
    bulk_rating INTEGER DEFAULT 1, -- how much space it takes up
    perishable BOOLEAN DEFAULT false,
    shelf_life_days INTEGER,
    production_locations INTEGER[], -- where it's commonly made
    demand_locations INTEGER[], -- where it's commonly wanted
    rarity TEXT DEFAULT 'common', -- common, uncommon, rare, very_rare
    seasonal_availability TEXT[],
    transportation_requirements TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trade routes and commerce
CREATE TABLE trade_routes (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    start_location_id INTEGER NOT NULL REFERENCES locations(id),
    end_location_id INTEGER NOT NULL REFERENCES locations(id),
    intermediate_stops INTEGER[], -- location IDs
    total_distance_miles DECIMAL(10,2),
    travel_time_days DECIMAL(10,2),
    danger_level INTEGER DEFAULT 3 CHECK (danger_level BETWEEN 1 AND 10),
    primary_goods INTEGER[], -- trade_goods IDs
    controlling_organization_id INTEGER REFERENCES organizations(id),
    toll_costs JSONB DEFAULT '{}',
    seasonal_availability TEXT[],
    current_status TEXT DEFAULT 'active', -- active, disrupted, abandoned
    disruption_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_trade_routes_updated_at BEFORE UPDATE
    ON trade_routes FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- RELIGION AND BELIEF SYSTEMS
-- ===================================================================

-- Deities and divine beings
CREATE TABLE deities (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    titles TEXT[],
    alignment TEXT,
    domains TEXT[], -- war, knowledge, nature, etc.
    portfolio TEXT[], -- what they're responsible for
    holy_symbol TEXT,
    favored_weapon TEXT,
    divine_rank TEXT DEFAULT 'lesser', -- greater, intermediate, lesser, demigod, hero
    description TEXT,
    appearance TEXT,
    personality_traits TEXT[],
    relationships_with_other_deities JSONB DEFAULT '{}',
    worshiper_alignments TEXT[],
    clergy_alignments TEXT[],
    holy_days TEXT[],
    creation_myths TEXT,
    major_temples INTEGER[], -- location IDs
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_deities_updated_at BEFORE UPDATE
    ON deities FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Religious organizations and temples
CREATE TABLE religious_organizations (
    id SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    deity_id INTEGER REFERENCES deities(id),
    religious_focus TEXT[], -- worship, charity, protection, knowledge, etc.
    doctrine TEXT,
    religious_practices TEXT[],
    hierarchy_structure TEXT,
    initiation_requirements TEXT[],
    core_beliefs TEXT[],
    forbidden_acts TEXT[],
    holy_texts TEXT[],
    sacred_locations INTEGER[], -- location IDs
    pilgrimage_sites INTEGER[], -- location IDs
    religious_festivals JSONB DEFAULT '[]',
    charitable_works TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_religious_organizations_updated_at BEFORE UPDATE
    ON religious_organizations FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- MAGIC AND SUPERNATURAL ELEMENTS
-- ===================================================================

-- Magic items and artifacts
CREATE TABLE magic_items (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    item_type TEXT NOT NULL, -- weapon, armor, wondrous, potion, scroll, etc.
    rarity TEXT NOT NULL DEFAULT 'common', -- common, uncommon, rare, very_rare, legendary, artifact
    attunement_required BOOLEAN DEFAULT false,
    description TEXT,
    mechanical_effects TEXT,
    activation_method TEXT,
    charges INTEGER,
    charge_recovery TEXT,
    curse_description TEXT,
    creator_name TEXT,
    creation_date DATE,
    historical_significance TEXT,
    current_location_id INTEGER REFERENCES locations(id),
    current_owner_npc_id INTEGER REFERENCES npcs(id),
    market_value DECIMAL(10,2),
    weight_pounds DECIMAL(6,2),
    physical_description TEXT,
    is_sentient BOOLEAN DEFAULT false,
    intelligence INTEGER,
    personality TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_magic_items_updated_at BEFORE UPDATE
    ON magic_items FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Magical phenomena and areas
CREATE TABLE magical_phenomena (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    location_id INTEGER REFERENCES locations(id),
    name TEXT NOT NULL,
    phenomenon_type TEXT NOT NULL, -- wild_magic, dead_magic, enhanced_magic, planar_rift, etc.
    description TEXT NOT NULL,
    magical_effects TEXT[],
    trigger_conditions TEXT[],
    duration TEXT, -- permanent, temporary, cyclical
    danger_level INTEGER DEFAULT 3 CHECK (danger_level BETWEEN 1 AND 10),
    study_difficulty TEXT DEFAULT 'moderate',
    known_by_npcs INTEGER[], -- who knows about this
    research_value TEXT,
    containment_possible BOOLEAN DEFAULT true,
    containment_methods TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_magical_phenomena_updated_at BEFORE UPDATE
    ON magical_phenomena FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- KNOWLEDGE AND INFORMATION SYSTEM
-- ===================================================================

-- Books, documents, and knowledge repositories
CREATE TABLE knowledge_sources (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    source_type TEXT DEFAULT 'book', -- book, scroll, tablet, oral_tradition, etc.
    author_npc_id INTEGER REFERENCES npcs(id),
    subject_areas TEXT[], -- history, magic, geography, religion, etc.
    content_summary TEXT,
    accuracy_level INTEGER DEFAULT 7 CHECK (accuracy_level BETWEEN 1 AND 10),
    completeness_level INTEGER DEFAULT 7 CHECK (completeness_level BETWEEN 1 AND 10),
    age_years INTEGER,
    language TEXT,
    physical_condition TEXT DEFAULT 'good',
    current_location_id INTEGER REFERENCES locations(id),
    access_restrictions TEXT,
    copying_difficulty TEXT DEFAULT 'moderate',
    market_value DECIMAL(10,2),
    rarity TEXT DEFAULT 'uncommon',
    related_topics TEXT[],
    contradicts_sources INTEGER[], -- other knowledge_sources IDs
    supports_sources INTEGER[], -- other knowledge_sources IDs
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_knowledge_sources_updated_at BEFORE UPDATE
    ON knowledge_sources FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Rumors and information networks
CREATE TABLE rumors (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    rumor_type TEXT DEFAULT 'gossip', -- gossip, news, secret, prophecy, etc.
    accuracy_level INTEGER DEFAULT 5 CHECK (accuracy_level BETWEEN 1 AND 10),
    spread_rate INTEGER DEFAULT 3 CHECK (spread_rate BETWEEN 1 AND 5),
    origin_location_id INTEGER REFERENCES locations(id),
    origin_npc_id INTEGER REFERENCES npcs(id),
    current_locations INTEGER[], -- where the rumor has spread
    target_audience TEXT[], -- who would be interested
    verification_difficulty TEXT DEFAULT 'moderate',
    consequences_if_true TEXT,
    consequences_if_false TEXT,
    expiration_date DATE, -- when rumor becomes stale
    related_events INTEGER[], -- historical_events or quest_hooks
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_rumors_updated_at BEFORE UPDATE
    ON rumors FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- CAMPAIGN METADATA AND TRACKING
-- ===================================================================

-- Session notes and campaign progress
CREATE TABLE campaign_sessions (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    session_number INTEGER NOT NULL,
    session_date DATE NOT NULL,
    in_game_date_start TEXT, -- using the campaign's calendar system
    in_game_date_end TEXT,
    time_advanced TEXT, -- how much in-game time passed
    locations_visited INTEGER[],
    npcs_encountered INTEGER[],
    quests_progressed INTEGER[],
    encounters_completed INTEGER[],
    major_events TEXT[],
    player_decisions JSONB DEFAULT '{}',
    experience_gained INTEGER DEFAULT 0,
    treasure_found TEXT[],
    session_summary TEXT,
    dm_notes TEXT,
    player_feedback TEXT,
    next_session_prep TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_campaign_sessions_updated_at BEFORE UPDATE
    ON campaign_sessions FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- Campaign flags and state tracking
CREATE TABLE campaign_flags (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    flag_name TEXT NOT NULL,
    flag_value TEXT,
    flag_type TEXT DEFAULT 'boolean', -- boolean, integer, string, date
    description TEXT,
    set_by_session INTEGER REFERENCES campaign_sessions(id),
    affects_future_events BOOLEAN DEFAULT true,
    visible_to_players BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(campaign_id, flag_name)
);

CREATE TRIGGER update_campaign_flags_updated_at BEFORE UPDATE
    ON campaign_flags FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column();

-- ===================================================================
-- INDEXES FOR PERFORMANCE
-- ===================================================================

-- Campaign-based indexes for most tables
CREATE INDEX idx_calendar_systems_campaign_id ON calendar_systems(campaign_id);
CREATE INDEX idx_historical_events_campaign_id ON historical_events(campaign_id);
CREATE INDEX idx_location_connections_from_location ON location_connections(from_location_id);
CREATE INDEX idx_location_connections_to_location ON location_connections(to_location_id);
CREATE INDEX idx_location_services_location_id ON location_services(location_id);
CREATE INDEX idx_npc_relationships_npc1 ON npc_relationships(npc1_id);
CREATE INDEX idx_npc_relationships_npc2 ON npc_relationships(npc2_id);
CREATE INDEX idx_organizations_campaign_id ON organizations(campaign_id);
CREATE INDEX idx_organization_memberships_organization ON organization_memberships(organization_id);
CREATE INDEX idx_organization_memberships_npc ON organization_memberships(npc_id);
CREATE INDEX idx_backstory_elements_campaign_id ON backstory_elements(campaign_id);
CREATE INDEX idx_backstory_elements_pc_name ON backstory_elements(player_character_name);
CREATE INDEX idx_quest_progress_quest_hook ON quest_progress(quest_hook_id);
CREATE INDEX idx_random_events_campaign_id ON random_events(campaign_id);
CREATE INDEX idx_currencies_campaign_id ON currencies(campaign_id);
CREATE INDEX idx_trade_routes_campaign_id ON trade_routes(campaign_id);
CREATE INDEX idx_deities_campaign_id ON deities(campaign_id);
CREATE INDEX idx_magic_items_campaign_id ON magic_items(campaign_id);
CREATE INDEX idx_magical_phenomena_campaign_id ON magical_phenomena(campaign_id);
CREATE INDEX idx_knowledge_sources_campaign_id ON knowledge_sources(campaign_id);
CREATE INDEX idx_rumors_campaign_id ON rumors(campaign_id);
CREATE INDEX idx_campaign_sessions_campaign_id ON campaign_sessions(campaign_id);
CREATE INDEX idx_campaign_flags_campaign_id ON campaign_flags(campaign_id);

-- Search and lookup indexes
CREATE INDEX idx_npcs_name_search ON npcs USING gin(to_tsvector('english', name));
CREATE INDEX idx_locations_name_search ON locations USING gin(to_tsvector('english', name));
CREATE INDEX idx_organizations_name_search ON organizations USING gin(to_tsvector('english', name));
CREATE INDEX idx_quest_hooks_title_search ON quest_hooks USING gin(to_tsvector('english', title));
CREATE INDEX idx_knowledge_sources_title_search ON knowledge_sources USING gin(to_tsvector('english', title));
CREATE INDEX idx_rumors_content_search ON rumors USING gin(to_tsvector('english', content));

-- Relationship and reference indexes
CREATE INDEX idx_locations_parent_location ON locations(parent_location_id);
CREATE INDEX idx_npcs_current_location ON npcs(current_location_id);
CREATE INDEX idx_npcs_home_location ON npcs(home_location_id);
CREATE INDEX idx_organizations_headquarters ON organizations(headquarters_location_id);
CREATE INDEX idx_organizations_parent ON organizations(parent_organization_id);
CREATE INDEX idx_backstory_elements_connected_npc ON backstory_elements(connected_npc_id);
CREATE INDEX idx_backstory_elements_connected_location ON backstory_elements(connected_location_id);
CREATE INDEX idx_backstory_elements_connected_organization ON backstory_elements(connected_organization_id);

-- Performance indexes for common queries
CREATE INDEX idx_npcs_race_background ON npcs(race_id, background_id);
CREATE INDEX idx_organizations_type_influence ON organizations(organization_type_id, influence_level);
CREATE INDEX idx_quest_hooks_status_urgency ON quest_hooks(status, urgency_level);
CREATE INDEX idx_encounters_location_difficulty ON encounters(location_id, difficulty);
CREATE INDEX idx_trade_goods_category_rarity ON trade_goods(category, rarity);
CREATE INDEX idx_magic_items_rarity_location ON magic_items(rarity, current_location_id);