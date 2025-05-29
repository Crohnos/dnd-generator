-- =====================================================
-- MODERN D&D CAMPAIGN DATABASE SCHEMA WITH SQLITE FEATURES
-- =====================================================
-- Focused on creating a lived-in world from PC backstories
-- Using JSON columns, FTS5, generated columns, and more

PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL; -- Better concurrent access

-- =====================================================
-- CORE CAMPAIGN & WORLD TABLES
-- =====================================================

CREATE TABLE campaigns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    genre TEXT,
    tone TEXT,
    magic_level TEXT CHECK(magic_level IN ('low', 'medium', 'high', 'very_high')),
    technology_level TEXT,
    starting_level INTEGER DEFAULT 1,
    expected_ending_level INTEGER,
    progression_type TEXT CHECK(progression_type IN ('milestone', 'experience', 'hybrid')),
    player_count_min INTEGER,
    player_count_max INTEGER,
    estimated_sessions INTEGER,
    current_year INTEGER,
    calendar_system TEXT,
    campaign_pillar_balance JSON DEFAULT '{"combat": 40, "social": 30, "exploration": 30}' 
        CHECK(json_valid(campaign_pillar_balance) AND 
              json_extract(campaign_pillar_balance, '$.combat') + 
              json_extract(campaign_pillar_balance, '$.social') + 
              json_extract(campaign_pillar_balance, '$.exploration') = 100),
    player_agency_level TEXT CHECK(player_agency_level IN ('railroad', 'guided', 'sandbox', 'full_sandbox')),
    prep_style_preference TEXT CHECK(prep_style_preference IN ('high_prep', 'medium_prep', 'improvisational')),
    failure_consequence_style TEXT CHECK(failure_consequence_style IN ('harsh', 'fail_forward', 'mixed')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP -- Soft delete support
) STRICT;

CREATE TABLE campaign_themes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    theme TEXT NOT NULL,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE house_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    rule_name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    negotiability_level TEXT CHECK(negotiability_level IN ('firm', 'flexible', 'experimental')),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

-- =====================================================
-- TEMPORAL & CALENDAR SYSTEM
-- =====================================================

CREATE TABLE calendar_systems (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    days_per_year INTEGER,
    current_date JSON DEFAULT '{}' CHECK(json_valid(current_date)),
    moon_phases JSON DEFAULT '[]' CHECK(json_valid(moon_phases)),
    zodiac_system JSON DEFAULT '[]' CHECK(json_valid(zodiac_system)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE calendar_months (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    calendar_system_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    days_in_month INTEGER,
    season TEXT,
    order_number INTEGER,
    deleted_at TIMESTAMP,
    FOREIGN KEY (calendar_system_id) REFERENCES calendar_systems(id) ON DELETE CASCADE
);

CREATE TABLE holidays (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    calendar_system_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    date JSON DEFAULT '{}' CHECK(json_valid(date)),
    significance TEXT,
    observance_customs TEXT,
    is_regional BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (calendar_system_id) REFERENCES calendar_systems(id) ON DELETE CASCADE
);

CREATE TABLE astronomical_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    event_type TEXT CHECK(event_type IN ('comet', 'eclipse', 'alignment', 'other')),
    frequency TEXT,
    next_occurrence TEXT,
    significance TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

-- =====================================================
-- PLAYER CHARACTERS
-- =====================================================

CREATE TABLE player_characters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    player_name TEXT NOT NULL,
    character_name TEXT NOT NULL,
    race TEXT,
    class TEXT,
    level INTEGER DEFAULT 1,
    backstory TEXT,
    goals JSON DEFAULT '[]' CHECK(json_valid(goals)),
    fears JSON DEFAULT '[]' CHECK(json_valid(fears)),
    unfinished_business TEXT,
    mentioned_npcs JSON DEFAULT '[]' CHECK(json_valid(mentioned_npcs)),
    mentioned_locations JSON DEFAULT '[]' CHECK(json_valid(mentioned_locations)),
    mentioned_organizations JSON DEFAULT '[]' CHECK(json_valid(mentioned_organizations)),
    mentioned_items JSON DEFAULT '[]' CHECK(json_valid(mentioned_items)),
    hometown TEXT,
    family_details TEXT,
    mentor_details TEXT,
    rival_details TEXT,
    -- Player preference fields
    player_type TEXT CHECK(player_type IN ('actor', 'explorer', 'power_gamer', 'instigator', 'social', 'watcher')),
    player_notes TEXT,
    -- Generated column for search
    search_text TEXT GENERATED ALWAYS AS (
        character_name || ' ' || COALESCE(backstory, '') || ' ' || 
        COALESCE(unfinished_business, '') || ' ' || COALESCE(hometown, '') || ' ' ||
        COALESCE(player_notes, '')
    ) STORED,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
) STRICT;

-- =====================================================
-- GEOGRAPHY & LOCATIONS  
-- =====================================================

CREATE TABLE location_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type_name TEXT NOT NULL UNIQUE,
    hierarchy_level INTEGER -- 1=continent, 2=region, 3=city, 4=district, 5=building, etc.
);

CREATE TABLE locations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    parent_location_id INTEGER,
    location_type_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    population INTEGER,
    wealth_level TEXT,
    danger_level INTEGER CHECK(danger_level BETWEEN 1 AND 10),
    climate TEXT,
    terrain_type TEXT,
    notable_features TEXT,
    current_events TEXT,
    secrets TEXT,
    map_coordinates JSON DEFAULT NULL CHECK(map_coordinates IS NULL OR json_valid(map_coordinates)),
    construction_materials TEXT,
    architectural_style TEXT,
    water_source TEXT,
    local_superstitions TEXT,
    ghost_stories TEXT,
    acoustic_properties TEXT,
    power_structure TEXT,
    discovery_method TEXT,
    revisit_value TEXT,
    instability_factors TEXT,
    emotional_resonance TEXT,
    -- Generated columns
    population_density REAL GENERATED ALWAYS AS (
        CASE 
            WHEN population IS NOT NULL AND json_extract(map_coordinates, '$.area_sq_km') IS NOT NULL 
            THEN CAST(population AS REAL) / json_extract(map_coordinates, '$.area_sq_km')
            ELSE NULL 
        END
    ) STORED,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_location_id) REFERENCES locations(id) ON DELETE SET NULL,
    FOREIGN KEY (location_type_id) REFERENCES location_types(id)
);

-- R-Tree index for spatial queries (if coordinates stored as {lat, lng})
CREATE VIRTUAL TABLE locations_rtree USING rtree(
    id,
    min_lat, max_lat,
    min_lng, max_lng
);

CREATE TABLE location_sensory_details (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    sense_type TEXT CHECK(sense_type IN ('sight', 'sound', 'smell', 'touch', 'taste')),
    description TEXT,
    time_of_day TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE location_defenses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    defense_type TEXT,
    description TEXT,
    strength_rating INTEGER,
    weakness TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE location_hidden_features (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    feature_type TEXT CHECK(feature_type IN ('passage', 'room', 'treasure', 'threat', 'history')),
    description TEXT,
    discovery_dc INTEGER,
    discovery_method TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE travel_routes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    from_location_id INTEGER NOT NULL,
    to_location_id INTEGER NOT NULL,
    route_type TEXT,
    distance REAL,
    travel_time_days REAL,
    difficulty TEXT,
    hazards TEXT,
    seasonal_accessibility JSON DEFAULT '{}' CHECK(json_valid(seasonal_accessibility)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (from_location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (to_location_id) REFERENCES locations(id) ON DELETE CASCADE
);

-- =====================================================
-- RACES, CULTURES & LANGUAGES
-- =====================================================

CREATE TABLE races (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    lifespan_min INTEGER,
    lifespan_max INTEGER,
    size_category TEXT,
    speed INTEGER DEFAULT 30,
    abilities JSON DEFAULT '{}' CHECK(json_valid(abilities)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE cultures (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    values JSON DEFAULT '[]' CHECK(json_valid(values)),
    taboos JSON DEFAULT '[]' CHECK(json_valid(taboos)),
    traditions JSON DEFAULT '[]' CHECK(json_valid(traditions)),
    art_style TEXT,
    architecture_style TEXT,
    cuisine TEXT,
    clothing_style TEXT,
    marriage_customs TEXT,
    funeral_rites TEXT,
    coming_of_age_ceremonies TEXT,
    hospitality_customs TEXT,
    gift_giving_etiquette TEXT,
    honor_shame_concepts TEXT,
    privacy_expectations TEXT,
    personal_space_norms TEXT,
    time_keeping_attitudes TEXT,
    outsider_attitudes TEXT,
    cultural_exports JSON DEFAULT '[]' CHECK(json_valid(cultural_exports)),
    cultural_blind_spots JSON DEFAULT '[]' CHECK(json_valid(cultural_blind_spots)),
    generational_conflicts TEXT,
    diaspora_communities TEXT,
    cultural_status TEXT CHECK(cultural_status IN ('thriving', 'stable', 'declining', 'renaissance')),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    script TEXT,
    typical_speakers TEXT,
    is_secret BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE race_culture_associations (
    race_id INTEGER NOT NULL,
    culture_id INTEGER NOT NULL,
    is_primary BOOLEAN DEFAULT FALSE,
    population_percentage REAL,
    integration_difficulty TEXT,
    deleted_at TIMESTAMP,
    PRIMARY KEY (race_id, culture_id),
    FOREIGN KEY (race_id) REFERENCES races(id) ON DELETE CASCADE,
    FOREIGN KEY (culture_id) REFERENCES cultures(id) ON DELETE CASCADE
);

CREATE TABLE culture_languages (
    culture_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    prevalence TEXT CHECK(prevalence IN ('primary', 'secondary', 'scholarly')),
    literacy_rate REAL,
    deleted_at TIMESTAMP,
    PRIMARY KEY (culture_id, language_id),
    FOREIGN KEY (culture_id) REFERENCES cultures(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE
);

-- =====================================================
-- COSMOLOGY & RELIGION
-- =====================================================

CREATE TABLE planes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    plane_type TEXT,
    alignment TEXT,
    traits JSON DEFAULT '[]' CHECK(json_valid(traits)),
    accessibility TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE deities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    titles JSON DEFAULT '[]' CHECK(json_valid(titles)),
    alignment TEXT,
    portfolio JSON DEFAULT '[]' CHECK(json_valid(portfolio)),
    holy_symbol TEXT,
    favored_weapon TEXT,
    domains JSON DEFAULT '[]' CHECK(json_valid(domains)),
    description TEXT,
    intervention_frequency TEXT CHECK(intervention_frequency IN ('never', 'rare', 'occasional', 'frequent')),
    power_manifestation_style TEXT,
    dietary_restrictions TEXT,
    prayer_schedule TEXT,
    is_dead BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE pantheons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    creation_myth TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE deity_pantheon_memberships (
    deity_id INTEGER NOT NULL,
    pantheon_id INTEGER NOT NULL,
    role TEXT,
    rank TEXT,
    deleted_at TIMESTAMP,
    PRIMARY KEY (deity_id, pantheon_id),
    FOREIGN KEY (deity_id) REFERENCES deities(id) ON DELETE CASCADE,
    FOREIGN KEY (pantheon_id) REFERENCES pantheons(id) ON DELETE CASCADE
);

CREATE TABLE religious_orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    deity_id INTEGER,
    name TEXT NOT NULL,
    order_type TEXT,
    description TEXT,
    hierarchy TEXT,
    requirements TEXT,
    titles_by_rank JSON DEFAULT '[]' CHECK(json_valid(titles_by_rank)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (deity_id) REFERENCES deities(id) ON DELETE SET NULL
);

CREATE TABLE sacred_sites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    deity_id INTEGER,
    religious_order_id INTEGER,
    site_type TEXT,
    significance TEXT,
    pilgrimage_value TEXT,
    miracles_witnessed JSON DEFAULT '[]' CHECK(json_valid(miracles_witnessed)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (deity_id) REFERENCES deities(id) ON DELETE SET NULL,
    FOREIGN KEY (religious_order_id) REFERENCES religious_orders(id) ON DELETE SET NULL
);

CREATE TABLE religious_conflicts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    conflict_type TEXT CHECK(conflict_type IN ('schism', 'heresy', 'holy_war', 'theological_debate')),
    description TEXT,
    participating_religions JSON DEFAULT '[]' CHECK(json_valid(participating_religions)),
    status TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

-- =====================================================
-- CHARACTERS (NPCs)
-- =====================================================

CREATE TABLE characters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    character_type TEXT CHECK(character_type IN ('pc', 'npc')),
    name TEXT NOT NULL,
    race_id INTEGER,
    culture_id INTEGER,
    class TEXT,
    level INTEGER DEFAULT 1,
    alignment TEXT,
    age INTEGER,
    gender TEXT,
    height TEXT,
    weight TEXT,
    appearance TEXT,
    personality_traits JSON DEFAULT '[]' CHECK(json_valid(personality_traits)),
    ideals JSON DEFAULT '[]' CHECK(json_valid(ideals)),
    bonds JSON DEFAULT '[]' CHECK(json_valid(bonds)),
    flaws JSON DEFAULT '[]' CHECK(json_valid(flaws)),
    backstory TEXT,
    secrets TEXT,
    current_location_id INTEGER,
    status TEXT DEFAULT 'active',
    -- 5-bullet method fields
    core_identity TEXT,
    primary_motivation TEXT,
    distinctive_quirk TEXT,
    current_situation TEXT,
    hidden_information TEXT,
    -- Additional detail fields
    voice_description TEXT,
    speech_patterns TEXT,
    reaction_tendencies JSON DEFAULT '{}' CHECK(json_valid(reaction_tendencies)),
    daily_routine JSON DEFAULT '[]' CHECK(json_valid(daily_routine)),
    combat_tactics TEXT,
    knowledge_areas JSON DEFAULT '[]' CHECK(json_valid(knowledge_areas)),
    stress_response TEXT,
    personal_philosophy TEXT,
    favorite_food_drink TEXT,
    hobbies JSON DEFAULT '[]' CHECK(json_valid(hobbies)),
    pet_peeves JSON DEFAULT '[]' CHECK(json_valid(pet_peeves)),
    prized_possessions JSON DEFAULT '[]' CHECK(json_valid(prized_possessions)),
    recurring_dreams TEXT,
    childhood_trauma_joy TEXT,
    mentor_influence TEXT,
    greatest_achievement TEXT,
    greatest_failure TEXT,
    secret_shame TEXT,
    phobias JSON DEFAULT '[]' CHECK(json_valid(phobias)),
    collections TEXT,
    reading_preferences TEXT,
    music_preferences TEXT,
    fashion_preferences TEXT,
    sleep_patterns TEXT,
    health_conditions JSON DEFAULT '[]' CHECK(json_valid(health_conditions)),
    financial_status TEXT,
    debts JSON DEFAULT '[]' CHECK(json_valid(debts)),
    -- PC backstory connection fields
    connected_to_pc_id INTEGER,
    pc_connection_type TEXT,
    pc_connection_details TEXT,
    -- Generated column for age calculation
    age_at_campaign_start INTEGER GENERATED ALWAYS AS (
        CASE 
            WHEN age IS NOT NULL AND campaign_id IS NOT NULL 
            THEN age - (SELECT current_year - starting_year FROM campaigns WHERE id = campaign_id)
            ELSE age
        END
    ) STORED,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (race_id) REFERENCES races(id) ON DELETE SET NULL,
    FOREIGN KEY (culture_id) REFERENCES cultures(id) ON DELETE SET NULL,
    FOREIGN KEY (current_location_id) REFERENCES locations(id) ON DELETE SET NULL
) STRICT;

CREATE TABLE character_stats (
    character_id INTEGER PRIMARY KEY,
    strength INTEGER,
    dexterity INTEGER,
    constitution INTEGER,
    intelligence INTEGER,
    wisdom INTEGER,
    charisma INTEGER,
    hit_points_max INTEGER,
    hit_points_current INTEGER,
    armor_class INTEGER,
    speed INTEGER,
    -- Generated columns for modifiers
    strength_modifier INTEGER GENERATED ALWAYS AS ((strength - 10) / 2) STORED,
    dexterity_modifier INTEGER GENERATED ALWAYS AS ((dexterity - 10) / 2) STORED,
    constitution_modifier INTEGER GENERATED ALWAYS AS ((constitution - 10) / 2) STORED,
    intelligence_modifier INTEGER GENERATED ALWAYS AS ((intelligence - 10) / 2) STORED,
    wisdom_modifier INTEGER GENERATED ALWAYS AS ((wisdom - 10) / 2) STORED,
    charisma_modifier INTEGER GENERATED ALWAYS AS ((charisma - 10) / 2) STORED,
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    skill_name TEXT NOT NULL,
    proficiency_level TEXT,
    modifier INTEGER,
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_languages (
    character_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    proficiency_level TEXT DEFAULT 'fluent',
    deleted_at TIMESTAMP,
    PRIMARY KEY (character_id, language_id),
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE
);

CREATE TABLE character_relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id_1 INTEGER NOT NULL,
    character_id_2 INTEGER NOT NULL,
    relationship_type TEXT,
    description TEXT,
    attitude_1_to_2 TEXT,
    attitude_2_to_1 TEXT,
    history TEXT,
    power_differential TEXT,
    information_asymmetry TEXT,
    relationship_volatility TEXT CHECK(relationship_volatility IN ('stable', 'tense', 'volatile', 'explosive')),
    public_vs_private TEXT,
    unresolved_tensions TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id_1) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (character_id_2) REFERENCES characters(id) ON DELETE CASCADE,
    CHECK (character_id_1 < character_id_2)
);

-- =====================================================
-- ORGANIZATIONS & FACTIONS
-- =====================================================

CREATE TABLE organizations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    organization_type TEXT,
    description TEXT,
    goals JSON DEFAULT '[]' CHECK(json_valid(goals)),
    resources JSON DEFAULT '{}' CHECK(json_valid(resources)),
    influence_level TEXT,
    headquarters_location_id INTEGER,
    symbol TEXT,
    motto TEXT,
    entry_requirements JSON DEFAULT '[]' CHECK(json_valid(entry_requirements)),
    internal_politics TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (headquarters_location_id) REFERENCES locations(id) ON DELETE SET NULL
);

CREATE TABLE organization_ranks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    rank_name TEXT NOT NULL,
    rank_level INTEGER,
    privileges JSON DEFAULT '[]' CHECK(json_valid(privileges)),
    requirements JSON DEFAULT '[]' CHECK(json_valid(requirements)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE
);

CREATE TABLE character_organization_memberships (
    character_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    rank_id INTEGER,
    join_date TEXT,
    status TEXT DEFAULT 'active',
    notes TEXT,
    secret_member BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    PRIMARY KEY (character_id, organization_id),
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE,
    FOREIGN KEY (rank_id) REFERENCES organization_ranks(id) ON DELETE SET NULL
);

CREATE TABLE organization_relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id_1 INTEGER NOT NULL,
    organization_id_2 INTEGER NOT NULL,
    relationship_type TEXT,
    description TEXT,
    public_stance TEXT,
    actual_stance TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (organization_id_1) REFERENCES organizations(id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id_2) REFERENCES organizations(id) ON DELETE CASCADE,
    CHECK (organization_id_1 < organization_id_2)
);

-- =====================================================
-- ITEMS & EQUIPMENT
-- =====================================================

CREATE TABLE item_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type_name TEXT NOT NULL UNIQUE,
    category TEXT
);

CREATE TABLE items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    item_type_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    rarity TEXT,
    value_gp REAL,
    weight_lbs REAL,
    is_magical BOOLEAN DEFAULT FALSE,
    magical_properties JSON DEFAULT '{}' CHECK(json_valid(magical_properties)),
    requires_attunement BOOLEAN DEFAULT FALSE,
    charges_max INTEGER,
    charges_current INTEGER,
    cursed BOOLEAN DEFAULT FALSE,
    sentient BOOLEAN DEFAULT FALSE,
    history TEXT,
    creation_method TEXT,
    cultural_associations JSON DEFAULT '[]' CHECK(json_valid(cultural_associations)),
    narrative_weight TEXT CHECK(narrative_weight IN ('flavor', 'minor', 'major', 'critical')),
    disposal_difficulty TEXT,
    unintended_uses JSON DEFAULT '[]' CHECK(json_valid(unintended_uses)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (item_type_id) REFERENCES item_types(id)
);

CREATE TABLE item_ownership (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL,
    owner_character_id INTEGER,
    location_id INTEGER,
    quantity INTEGER DEFAULT 1,
    is_equipped BOOLEAN DEFAULT FALSE,
    notes TEXT,
    emotional_significance TEXT,
    acquisition_story TEXT,
    willing_to_sacrifice BOOLEAN,
    deleted_at TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_character_id) REFERENCES characters(id) ON DELETE SET NULL,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL,
    CHECK ((owner_character_id IS NOT NULL AND location_id IS NULL) OR 
           (owner_character_id IS NULL AND location_id IS NOT NULL))
);

-- =====================================================
-- ECONOMY & TRADE
-- =====================================================

CREATE TABLE currencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    abbreviation TEXT,
    value_in_gp REAL,
    description TEXT,
    minting_authority TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE trade_goods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    category TEXT,
    base_price_gp REAL,
    weight_per_unit REAL,
    unit_name TEXT DEFAULT 'piece',
    perishable BOOLEAN DEFAULT FALSE,
    seasonal_availability JSON DEFAULT '{}' CHECK(json_valid(seasonal_availability)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE location_trade_prices (
    location_id INTEGER NOT NULL,
    trade_good_id INTEGER NOT NULL,
    price_modifier REAL DEFAULT 1.0,
    availability TEXT,
    notes TEXT,
    deleted_at TIMESTAMP,
    PRIMARY KEY (location_id, trade_good_id),
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (trade_good_id) REFERENCES trade_goods(id) ON DELETE CASCADE
);

CREATE TABLE shops (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    shop_type TEXT,
    description TEXT,
    owner_character_id INTEGER,
    wealth_level TEXT,
    reputation TEXT,
    specialties JSON DEFAULT '[]' CHECK(json_valid(specialties)),
    operating_hours JSON DEFAULT '{}' CHECK(json_valid(operating_hours)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_character_id) REFERENCES characters(id) ON DELETE SET NULL
);

CREATE TABLE shop_inventory (
    shop_id INTEGER NOT NULL,
    item_id INTEGER,
    trade_good_id INTEGER,
    quantity INTEGER,
    price_modifier REAL DEFAULT 1.0,
    deleted_at TIMESTAMP,
    PRIMARY KEY (shop_id, COALESCE(item_id, 0), COALESCE(trade_good_id, 0)),
    FOREIGN KEY (shop_id) REFERENCES shops(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (trade_good_id) REFERENCES trade_goods(id) ON DELETE CASCADE,
    CHECK ((item_id IS NOT NULL AND trade_good_id IS NULL) OR 
           (item_id IS NULL AND trade_good_id IS NOT NULL))
);

CREATE TABLE economic_conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    condition_type TEXT CHECK(condition_type IN ('boom', 'stable', 'recession', 'depression')),
    inflation_rate REAL,
    unemployment_rate REAL,
    major_exports JSON DEFAULT '[]' CHECK(json_valid(major_exports)),
    major_imports JSON DEFAULT '[]' CHECK(json_valid(major_imports)),
    trade_dependencies JSON DEFAULT '[]' CHECK(json_valid(trade_dependencies)),
    monopolies JSON DEFAULT '[]' CHECK(json_valid(monopolies)),
    black_market_activity TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

-- =====================================================
-- INFRASTRUCTURE
-- =====================================================

CREATE TABLE infrastructure (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    infrastructure_type TEXT,
    name TEXT,
    description TEXT,
    condition TEXT CHECK(condition IN ('excellent', 'good', 'fair', 'poor', 'ruined')),
    maintenance_cost_monthly INTEGER,
    strategic_importance TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE public_services (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    service_type TEXT CHECK(service_type IN ('postal', 'transportation', 'sanitation', 'water', 'lighting', 'heating')),
    description TEXT,
    quality TEXT CHECK(quality IN ('excellent', 'good', 'adequate', 'poor', 'nonexistent')),
    cost_to_citizens TEXT,
    coverage_percentage REAL,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

-- =====================================================
-- LAW & JUSTICE
-- =====================================================

CREATE TABLE legal_systems (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    location_id INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    authority_source TEXT,
    corruption_level TEXT CHECK(corruption_level IN ('none', 'low', 'moderate', 'high', 'endemic')),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
);

CREATE TABLE laws (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    legal_system_id INTEGER NOT NULL,
    law_code TEXT,
    law_category TEXT,
    description TEXT,
    typical_punishment TEXT,
    severity TEXT,
    loopholes JSON DEFAULT '[]' CHECK(json_valid(loopholes)),
    enforcement_consistency TEXT CHECK(enforcement_consistency IN ('strict', 'fair', 'inconsistent', 'corrupt')),
    deleted_at TIMESTAMP,
    FOREIGN KEY (legal_system_id) REFERENCES legal_systems(id) ON DELETE CASCADE
);

CREATE TABLE crimes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    law_id INTEGER NOT NULL,
    location_id INTEGER,
    crime_date TEXT,
    description TEXT,
    witnesses JSON DEFAULT '[]' CHECK(json_valid(witnesses)),
    status TEXT DEFAULT 'accused',
    punishment TEXT,
    bribe_amount INTEGER,
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (law_id) REFERENCES laws(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
);

-- =====================================================
-- ENTERTAINMENT & LEISURE
-- =====================================================

CREATE TABLE entertainment_venues (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    venue_type TEXT CHECK(venue_type IN ('theater', 'arena', 'tavern', 'gambling_den', 'fighting_pit', 'race_track')),
    name TEXT NOT NULL,
    description TEXT,
    typical_events JSON DEFAULT '[]' CHECK(json_valid(typical_events)),
    admission_cost TEXT,
    social_class_clientele TEXT,
    owner_id INTEGER,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES characters(id) ON DELETE SET NULL
);

CREATE TABLE performances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venue_id INTEGER NOT NULL,
    performance_type TEXT,
    performer_names JSON DEFAULT '[]' CHECK(json_valid(performer_names)),
    schedule JSON DEFAULT '{}' CHECK(json_valid(schedule)),
    popularity TEXT CHECK(popularity IN ('unknown', 'local', 'regional', 'famous')),
    typical_audience TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (venue_id) REFERENCES entertainment_venues(id) ON DELETE CASCADE
);

CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    game_type TEXT CHECK(game_type IN ('board', 'card', 'dice', 'sport', 'children')),
    name TEXT NOT NULL,
    rules TEXT,
    typical_stakes TEXT,
    cultural_origin_id INTEGER,
    required_equipment JSON DEFAULT '[]' CHECK(json_valid(required_equipment)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (cultural_origin_id) REFERENCES cultures(id) ON DELETE SET NULL
);

-- =====================================================
-- HEALTH & MEDICINE
-- =====================================================

CREATE TABLE diseases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    transmission_method TEXT,
    symptoms JSON DEFAULT '[]' CHECK(json_valid(symptoms)),
    progression JSON DEFAULT '[]' CHECK(json_valid(progression)),
    treatment_magical TEXT,
    treatment_mundane TEXT,
    mortality_rate REAL,
    contagiousness TEXT CHECK(contagiousness IN ('none', 'low', 'moderate', 'high', 'extreme')),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE medical_facilities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    facility_type TEXT CHECK(facility_type IN ('temple', 'hospital', 'clinic', 'herbalist', 'plague_house')),
    name TEXT NOT NULL,
    quality TEXT CHECK(quality IN ('excellent', 'good', 'adequate', 'poor')),
    services_offered JSON DEFAULT '[]' CHECK(json_valid(services_offered)),
    typical_patients TEXT,
    costs JSON DEFAULT '{}' CHECK(json_valid(costs)),
    staff_count INTEGER,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE medical_practitioners (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    practitioner_type TEXT CHECK(practitioner_type IN ('cleric', 'physician', 'herbalist', 'surgeon', 'midwife')),
    specializations JSON DEFAULT '[]' CHECK(json_valid(specializations)),
    reputation TEXT,
    methods TEXT,
    success_rate REAL,
    typical_fees JSON DEFAULT '{}' CHECK(json_valid(typical_fees)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

-- =====================================================
-- KNOWLEDGE & LORE
-- =====================================================

CREATE TABLE libraries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    access_requirements JSON DEFAULT '[]' CHECK(json_valid(access_requirements)),
    specialties JSON DEFAULT '[]' CHECK(json_valid(specialties)),
    curator_character_id INTEGER,
    catalog_system TEXT,
    notable_sections JSON DEFAULT '[]' CHECK(json_valid(notable_sections)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (curator_character_id) REFERENCES characters(id) ON DELETE SET NULL
);

CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    author TEXT,
    subject TEXT,
    language_id INTEGER,
    rarity TEXT,
    condition TEXT,
    magical_properties JSON DEFAULT '{}' CHECK(json_valid(magical_properties)),
    content_summary TEXT,
    dangerous_knowledge BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE SET NULL
);

CREATE TABLE library_collections (
    library_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    quantity INTEGER DEFAULT 1,
    restricted_access BOOLEAN DEFAULT FALSE,
    special_requirements TEXT,
    deleted_at TIMESTAMP,
    PRIMARY KEY (library_id, book_id),
    FOREIGN KEY (library_id) REFERENCES libraries(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);

CREATE TABLE world_mysteries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    mystery_type TEXT CHECK(mystery_type IN ('ancient', 'disappearance', 'phenomenon', 'conspiracy', 'prophecy')),
    description TEXT,
    known_clues JSON DEFAULT '[]' CHECK(json_valid(known_clues)),
    false_theories JSON DEFAULT '[]' CHECK(json_valid(false_theories)),
    true_explanation TEXT,
    discovery_requirements JSON DEFAULT '[]' CHECK(json_valid(discovery_requirements)),
    consequences_if_solved TEXT,
    consequences_if_unsolved TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

-- =====================================================
-- NATURAL WORLD
-- =====================================================

CREATE TABLE flora (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    scientific_name TEXT,
    description TEXT,
    habitat TEXT,
    uses JSON DEFAULT '[]' CHECK(json_valid(uses)),
    rarity TEXT,
    magical_properties JSON DEFAULT '{}' CHECK(json_valid(magical_properties)),
    preparation_methods JSON DEFAULT '[]' CHECK(json_valid(preparation_methods)),
    seasonal_availability JSON DEFAULT '{}' CHECK(json_valid(seasonal_availability)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE fauna (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    creature_type TEXT,
    size TEXT,
    habitat TEXT,
    behavior TEXT,
    diet TEXT,
    danger_level INTEGER,
    domesticatable BOOLEAN DEFAULT FALSE,
    migration_patterns JSON DEFAULT '{}' CHECK(json_valid(migration_patterns)),
    breeding_season TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE location_flora (
    location_id INTEGER NOT NULL,
    flora_id INTEGER NOT NULL,
    abundance TEXT,
    notes TEXT,
    deleted_at TIMESTAMP,
    PRIMARY KEY (location_id, flora_id),
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (flora_id) REFERENCES flora(id) ON DELETE CASCADE
);

CREATE TABLE location_fauna (
    location_id INTEGER NOT NULL,
    fauna_id INTEGER NOT NULL,
    population_density TEXT,
    migration_pattern TEXT,
    notes TEXT,
    deleted_at TIMESTAMP,
    PRIMARY KEY (location_id, fauna_id),
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (fauna_id) REFERENCES fauna(id) ON DELETE CASCADE
);

CREATE TABLE natural_phenomena (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    location_id INTEGER,
    phenomenon_type TEXT,
    name TEXT NOT NULL,
    description TEXT,
    frequency TEXT,
    predictability TEXT,
    effects JSON DEFAULT '[]' CHECK(json_valid(effects)),
    local_beliefs JSON DEFAULT '[]' CHECK(json_valid(local_beliefs)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
);

-- =====================================================
-- UNDERWORLD ELEMENTS
-- =====================================================

CREATE TABLE criminal_organizations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    specialty TEXT CHECK(specialty IN ('thieves', 'assassins', 'smugglers', 'forgery', 'protection')),
    territory JSON DEFAULT '[]' CHECK(json_valid(territory)),
    methods JSON DEFAULT '[]' CHECK(json_valid(methods)),
    code_of_conduct JSON DEFAULT '[]' CHECK(json_valid(code_of_conduct)),
    recruitment_methods TEXT,
    law_enforcement_infiltration TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE
);

CREATE TABLE black_markets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    market_type TEXT,
    typical_goods JSON DEFAULT '[]' CHECK(json_valid(typical_goods)),
    access_requirements JSON DEFAULT '[]' CHECK(json_valid(access_requirements)),
    typical_prices TEXT,
    raid_frequency TEXT,
    protection_arrangement TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE safe_houses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id INTEGER NOT NULL,
    owner_organization_id INTEGER,
    security_level TEXT CHECK(security_level IN ('basic', 'moderate', 'high', 'fortress')),
    amenities JSON DEFAULT '[]' CHECK(json_valid(amenities)),
    escape_routes JSON DEFAULT '[]' CHECK(json_valid(escape_routes)),
    capacity INTEGER,
    current_occupants JSON DEFAULT '[]' CHECK(json_valid(current_occupants)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_organization_id) REFERENCES organizations(id) ON DELETE SET NULL
);

-- =====================================================
-- ADDITIONAL RELATIONSHIP TABLES
-- =====================================================

CREATE TABLE character_location_relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    location_id INTEGER NOT NULL,
    relationship_type TEXT CHECK(relationship_type IN ('birthplace', 'residence', 'workplace', 'owns_property', 'banned_from', 'has_bounty', 'hideout', 'emotional_significance', 'regular_visitor', 'protector')),
    description TEXT,
    start_date TEXT,
    end_date TEXT,
    is_secret BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE location_relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    location_id_1 INTEGER NOT NULL,
    location_id_2 INTEGER NOT NULL,
    relationship_type TEXT CHECK(relationship_type IN ('trade_partner', 'military_ally', 'at_war', 'cultural_influence', 'shared_resource', 'disputed_territory', 'migration_route')),
    description TEXT,
    strength TEXT CHECK(strength IN ('weak', 'moderate', 'strong')),
    is_public_knowledge BOOLEAN DEFAULT TRUE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (location_id_1) REFERENCES locations(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id_2) REFERENCES locations(id) ON DELETE CASCADE,
    CHECK (location_id_1 < location_id_2)
);

CREATE TABLE character_culture_relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    culture_id INTEGER NOT NULL,
    relationship_type TEXT CHECK(relationship_type IN ('born_into', 'adopted_into', 'exiled_from', 'ambassador_to', 'student_of', 'married_into', 'enemy_of', 'protector_of')),
    integration_level TEXT CHECK(integration_level IN ('outsider', 'tolerated', 'accepted', 'integrated', 'exemplar')),
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (culture_id) REFERENCES cultures(id) ON DELETE CASCADE
);

CREATE TABLE character_deity_relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    deity_id INTEGER NOT NULL,
    relationship_type TEXT CHECK(relationship_type IN ('devout_follower', 'casual_worshipper', 'cleric', 'paladin', 'chosen', 'cursed_by', 'seeking_favor', 'apostate', 'enemy')),
    piety_level INTEGER CHECK(piety_level BETWEEN -100 AND 100),
    last_prayer_date TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (deity_id) REFERENCES deities(id) ON DELETE CASCADE
);

CREATE TABLE item_culture_significance (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL,
    culture_id INTEGER NOT NULL,
    significance_type TEXT CHECK(significance_type IN ('sacred', 'taboo', 'status_symbol', 'traditional_tool', 'ceremonial', 'forbidden')),
    description TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (culture_id) REFERENCES cultures(id) ON DELETE CASCADE
);

-- =====================================================
-- PC BACKSTORY INTEGRATION TABLES
-- =====================================================

CREATE TABLE pc_backstory_elements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pc_id INTEGER NOT NULL,
    element_type TEXT CHECK(element_type IN ('person', 'place', 'organization', 'item', 'event', 'mystery', 'debt', 'oath')),
    name TEXT NOT NULL,
    description TEXT,
    importance_level INTEGER CHECK(importance_level BETWEEN 1 AND 5),
    emotional_weight TEXT CHECK(emotional_weight IN ('positive', 'negative', 'neutral', 'complicated')),
    current_status TEXT,
    integrated_into_world BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (pc_id) REFERENCES player_characters(id) ON DELETE CASCADE
);

CREATE TABLE pc_npc_connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pc_id INTEGER NOT NULL,
    npc_id INTEGER NOT NULL,
    connection_type TEXT CHECK(connection_type IN ('family', 'mentor', 'rival', 'friend', 'enemy', 'lover', 'employer', 'employee', 'ally', 'contact', 'debt_owed_to', 'debt_owed_by')),
    backstory_reference TEXT,
    connection_strength INTEGER CHECK(connection_strength BETWEEN 1 AND 5),
    discovered_in_play BOOLEAN DEFAULT FALSE,
    is_alive BOOLEAN DEFAULT TRUE,
    last_known_location_id INTEGER,
    deleted_at TIMESTAMP,
    FOREIGN KEY (pc_id) REFERENCES player_characters(id) ON DELETE CASCADE,
    FOREIGN KEY (npc_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (last_known_location_id) REFERENCES locations(id) ON DELETE SET NULL
);

CREATE TABLE pc_location_connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pc_id INTEGER NOT NULL,
    location_id INTEGER NOT NULL,
    connection_type TEXT CHECK(connection_type IN ('birthplace', 'childhood_home', 'trained_at', 'worked_at', 'imprisoned_at', 'hiding_place', 'sacred_place', 'traumatic_event_site')),
    time_period TEXT,
    significance TEXT,
    last_visited TEXT,
    can_return BOOLEAN DEFAULT TRUE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (pc_id) REFERENCES player_characters(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

CREATE TABLE pc_organization_connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pc_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    connection_type TEXT CHECK(connection_type IN ('member', 'former_member', 'ally', 'enemy', 'owes_debt', 'owed_debt', 'hunted_by', 'protected_by')),
    rank_achieved TEXT,
    standing TEXT CHECK(standing IN ('excellent', 'good', 'neutral', 'poor', 'hostile')),
    years_associated INTEGER,
    left_on_good_terms BOOLEAN,
    deleted_at TIMESTAMP,
    FOREIGN KEY (pc_id) REFERENCES player_characters(id) ON DELETE CASCADE,
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE CASCADE
);

-- =====================================================
-- HISTORY & TIMELINE
-- =====================================================

CREATE TABLE historical_eras (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    start_year INTEGER,
    end_year INTEGER,
    description TEXT,
    defining_characteristics JSON DEFAULT '[]' CHECK(json_valid(defining_characteristics)),
    technology_changes JSON DEFAULT '[]' CHECK(json_valid(technology_changes)),
    major_conflicts JSON DEFAULT '[]' CHECK(json_valid(major_conflicts)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE historical_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    era_id INTEGER,
    event_year INTEGER,
    event_month INTEGER,
    event_day INTEGER,
    name TEXT NOT NULL,
    event_type TEXT,
    description TEXT,
    location_id INTEGER,
    significance TEXT,
    still_celebrated BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (era_id) REFERENCES historical_eras(id) ON DELETE SET NULL,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
);

CREATE TABLE event_participants (
    event_id INTEGER NOT NULL,
    participant_type TEXT CHECK(participant_type IN ('character', 'organization', 'deity', 'culture')),
    participant_id INTEGER NOT NULL,
    role TEXT,
    deleted_at TIMESTAMP,
    PRIMARY KEY (event_id, participant_type, participant_id),
    FOREIGN KEY (event_id) REFERENCES historical_events(id) ON DELETE CASCADE
);

-- =====================================================
-- MAGIC SYSTEM
-- =====================================================

CREATE TABLE magic_schools (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    typical_effects JSON DEFAULT '[]' CHECK(json_valid(typical_effects)),
    societal_view TEXT,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE spells (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    school_id INTEGER,
    level INTEGER,
    casting_time TEXT,
    range TEXT,
    components TEXT,
    duration TEXT,
    description TEXT,
    higher_levels TEXT,
    is_homebrew BOOLEAN DEFAULT FALSE,
    regional_variations JSON DEFAULT '{}' CHECK(json_valid(regional_variations)),
    forbidden_in_locations JSON DEFAULT '[]' CHECK(json_valid(forbidden_in_locations)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (school_id) REFERENCES magic_schools(id) ON DELETE SET NULL
);

CREATE TABLE character_spells (
    character_id INTEGER NOT NULL,
    spell_id INTEGER NOT NULL,
    source TEXT,
    prepared BOOLEAN DEFAULT FALSE,
    uses_remaining INTEGER,
    deleted_at TIMESTAMP,
    PRIMARY KEY (character_id, spell_id),
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (spell_id) REFERENCES spells(id) ON DELETE CASCADE
);

CREATE TABLE magical_phenomena (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    location_id INTEGER,
    name TEXT NOT NULL,
    phenomenon_type TEXT,
    description TEXT,
    effects JSON DEFAULT '[]' CHECK(json_valid(effects)),
    is_permanent BOOLEAN DEFAULT TRUE,
    discovery_dc INTEGER,
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
);

CREATE TABLE magic_items_sentience (
    item_id INTEGER PRIMARY KEY,
    intelligence INTEGER,
    wisdom INTEGER,
    charisma INTEGER,
    alignment TEXT,
    communication_method TEXT,
    languages JSON DEFAULT '[]' CHECK(json_valid(languages)),
    personality TEXT,
    goals JSON DEFAULT '[]' CHECK(json_valid(goals)),
    quirks JSON DEFAULT '[]' CHECK(json_valid(quirks)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
);

-- =====================================================
-- PROPHECIES & MYSTERIES
-- =====================================================

CREATE TABLE prophecies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    prophecy_text TEXT,
    source TEXT,
    known_interpretations JSON DEFAULT '[]' CHECK(json_valid(known_interpretations)),
    true_meaning TEXT,
    is_fulfilled BOOLEAN DEFAULT FALSE,
    fulfillment_conditions JSON DEFAULT '[]' CHECK(json_valid(fulfillment_conditions)),
    red_herrings JSON DEFAULT '[]' CHECK(json_valid(red_herrings)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE mysteries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    known_facts JSON DEFAULT '[]' CHECK(json_valid(known_facts)),
    false_leads JSON DEFAULT '[]' CHECK(json_valid(false_leads)),
    true_solution TEXT,
    is_solved BOOLEAN DEFAULT FALSE,
    consequences TEXT,
    discovery_methods JSON DEFAULT '[]' CHECK(json_valid(discovery_methods)),
    deleted_at TIMESTAMP,
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
);

CREATE TABLE mystery_clue_locations (
    mystery_id INTEGER NOT NULL,
    location_id INTEGER NOT NULL,
    clue_description TEXT,
    discovery_requirements TEXT,
    is_red_herring BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMP,
    PRIMARY KEY (mystery_id, location_id),
    FOREIGN KEY (mystery_id) REFERENCES mysteries(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE CASCADE
);

-- =====================================================
-- FULL-TEXT SEARCH VIRTUAL TABLES
-- =====================================================

-- Characters and PCs full-text search
CREATE VIRTUAL TABLE character_search USING fts5(
    name, 
    backstory, 
    personality_traits,
    ideals,
    bonds,
    flaws,
    secrets,
    core_identity,
    primary_motivation,
    content=characters,
    content_rowid=id
);

-- Trigger to keep FTS index updated
CREATE TRIGGER characters_ai AFTER INSERT ON characters BEGIN
    INSERT INTO character_search(rowid, name, backstory, personality_traits, ideals, bonds, flaws, secrets, core_identity, primary_motivation)
    VALUES (new.id, new.name, new.backstory, new.personality_traits, new.ideals, new.bonds, new.flaws, new.secrets, new.core_identity, new.primary_motivation);
END;

CREATE TRIGGER characters_ad AFTER DELETE ON characters BEGIN
    DELETE FROM character_search WHERE rowid = old.id;
END;

CREATE TRIGGER characters_au AFTER UPDATE ON characters BEGIN
    DELETE FROM character_search WHERE rowid = old.id;
    INSERT INTO character_search(rowid, name, backstory, personality_traits, ideals, bonds, flaws, secrets, core_identity, primary_motivation)
    VALUES (new.id, new.name, new.backstory, new.personality_traits, new.ideals, new.bonds, new.flaws, new.secrets, new.core_identity, new.primary_motivation);
END;

-- Locations full-text search
CREATE VIRTUAL TABLE location_search USING fts5(
    name,
    description,
    notable_features,
    secrets,
    content=locations,
    content_rowid=id
);

-- Trigger to keep FTS index updated
CREATE TRIGGER locations_ai AFTER INSERT ON locations BEGIN
    INSERT INTO location_search(rowid, name, description, notable_features, secrets)
    VALUES (new.id, new.name, new.description, new.notable_features, new.secrets);
END;

CREATE TRIGGER locations_ad AFTER DELETE ON locations BEGIN
    DELETE FROM location_search WHERE rowid = old.id;
END;

CREATE TRIGGER locations_au AFTER UPDATE ON locations BEGIN
    DELETE FROM location_search WHERE rowid = old.id;
    INSERT INTO location_search(rowid, name, description, notable_features, secrets)
    VALUES (new.id, new.name, new.description, new.notable_features, new.secrets);
END;

-- Items full-text search
CREATE VIRTUAL TABLE item_search USING fts5(
    name,
    description,
    history,
    content=items,
    content_rowid=id
);

-- Organizations full-text search
CREATE VIRTUAL TABLE organization_search USING fts5(
    name,
    description,
    goals,
    motto,
    content=organizations,
    content_rowid=id
);

-- =====================================================
-- INDEXES FOR PERFORMANCE
-- =====================================================

-- Original indexes
CREATE INDEX idx_locations_campaign ON locations(campaign_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_locations_parent ON locations(parent_location_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_characters_campaign ON characters(campaign_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_characters_location ON characters(current_location_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_item_ownership_character ON item_ownership(owner_character_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_item_ownership_location ON item_ownership(location_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_character_relationships_char1 ON character_relationships(character_id_1) WHERE deleted_at IS NULL;
CREATE INDEX idx_character_relationships_char2 ON character_relationships(character_id_2) WHERE deleted_at IS NULL;
CREATE INDEX idx_travel_routes_from ON travel_routes(from_location_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_travel_routes_to ON travel_routes(to_location_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_historical_events_year ON historical_events(event_year) WHERE deleted_at IS NULL;
CREATE INDEX idx_prophecies_unfulfilled ON prophecies(campaign_id) WHERE deleted_at IS NULL AND is_fulfilled = FALSE;
CREATE INDEX idx_mysteries_unsolved ON mysteries(campaign_id) WHERE deleted_at IS NULL AND is_solved = FALSE;
CREATE INDEX idx_pc_connections ON pc_npc_connections(pc_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_pc_backstory_elements ON pc_backstory_elements(pc_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_pc_location_connections ON pc_location_connections(pc_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_pc_organization_connections ON pc_organization_connections(pc_id) WHERE deleted_at IS NULL;

-- Additional partial indexes for active records
CREATE INDEX idx_active_characters ON characters(campaign_id, status) WHERE deleted_at IS NULL AND status = 'active';

-- JSON extraction indexes for frequently queried JSON fields
CREATE INDEX idx_character_alignment ON characters(alignment) WHERE deleted_at IS NULL;
CREATE INDEX idx_location_danger ON locations(danger_level) WHERE deleted_at IS NULL;
CREATE INDEX idx_item_rarity ON items(rarity) WHERE deleted_at IS NULL;

-- =====================================================
-- TRIGGERS FOR UPDATED_AT TIMESTAMPS
-- =====================================================

CREATE TRIGGER update_campaign_timestamp 
AFTER UPDATE ON campaigns
BEGIN
    UPDATE campaigns SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- =====================================================
-- VIEWS FOR COMMON QUERIES
-- =====================================================

-- View for active characters with full details
CREATE VIEW active_character_details AS
WITH character_base AS (
    SELECT 
        c.*,
        r.name as race_name,
        cult.name as culture_name,
        l.name as current_location_name,
        cs.strength, cs.dexterity, cs.constitution, 
        cs.intelligence, cs.wisdom, cs.charisma,
        cs.hit_points_max, cs.hit_points_current,
        cs.armor_class, cs.speed
    FROM characters c
    LEFT JOIN races r ON c.race_id = r.id
    LEFT JOIN cultures cult ON c.culture_id = cult.id
    LEFT JOIN locations l ON c.current_location_id = l.id
    LEFT JOIN character_stats cs ON c.id = cs.character_id
    WHERE c.deleted_at IS NULL AND c.status = 'active'
)
SELECT * FROM character_base;

-- View for location hierarchy using CTE
CREATE VIEW location_hierarchy AS
WITH RECURSIVE location_tree AS (
    SELECT 
        l.*,
        l.name as full_path,
        0 as depth
    FROM locations l
    WHERE l.parent_location_id IS NULL AND l.deleted_at IS NULL
    
    UNION ALL
    
    SELECT 
        l.*,
        lt.full_path || ' > ' || l.name as full_path,
        lt.depth + 1 as depth
    FROM locations l
    JOIN location_tree lt ON l.parent_location_id = lt.id
    WHERE l.deleted_at IS NULL
)
SELECT * FROM location_tree;

-- =====================================================
-- SAMPLE QUERIES DEMONSTRATING MODERN FEATURES
-- =====================================================

/* Example: Find all characters in a location and its sub-locations
WITH RECURSIVE location_tree AS (
    SELECT id FROM locations WHERE id = ? AND deleted_at IS NULL
    UNION ALL
    SELECT l.id FROM locations l
    JOIN location_tree lt ON l.parent_location_id = lt.id
    WHERE l.deleted_at IS NULL
)
SELECT c.* FROM characters c
WHERE c.current_location_id IN (SELECT id FROM location_tree)
AND c.deleted_at IS NULL;

-- Example: Search for characters mentioning "dragon"
SELECT c.*, snippet(character_search, -1, '<b>', '</b>', '...', 32) as excerpt
FROM characters c
JOIN character_search cs ON c.id = cs.rowid
WHERE character_search MATCH 'dragon'
AND c.deleted_at IS NULL
ORDER BY rank;

-- Example: Get campaign pillars as separate columns
SELECT 
    name,
    json_extract(campaign_pillar_balance, '$.combat') as combat_focus,
    json_extract(campaign_pillar_balance, '$.social') as social_focus,
    json_extract(campaign_pillar_balance, '$.exploration') as exploration_focus
FROM campaigns
WHERE deleted_at IS NULL;

-- Example: Insert with RETURNING
INSERT INTO characters (campaign_id, name, character_type, level)
VALUES (1, 'New NPC', 'npc', 5)
RETURNING id, name, created_at;

-- Example: UPSERT for session attendance
INSERT INTO session_participants (session_id, character_id, attendance_status)
VALUES (?, ?, 'present')
ON CONFLICT(session_id, character_id) 
DO UPDATE SET attendance_status = excluded.attendance_status,
              deleted_at = NULL;
*/