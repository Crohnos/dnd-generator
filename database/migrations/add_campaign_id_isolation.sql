-- Add campaign_id to all tables for full campaign isolation
-- This ensures every piece of data is tied to a specific campaign

-- Add campaign_id to entity_factions
ALTER TABLE entity_factions ADD COLUMN campaign_id INTEGER;
ALTER TABLE entity_factions ADD CONSTRAINT entity_factions_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to entity_items  
ALTER TABLE entity_items ADD COLUMN campaign_id INTEGER;
ALTER TABLE entity_items ADD CONSTRAINT entity_items_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to entity_locations
ALTER TABLE entity_locations ADD COLUMN campaign_id INTEGER;
ALTER TABLE entity_locations ADD CONSTRAINT entity_locations_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to entity_relationships
ALTER TABLE entity_relationships ADD COLUMN campaign_id INTEGER;
ALTER TABLE entity_relationships ADD CONSTRAINT entity_relationships_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to faction_relationships
ALTER TABLE faction_relationships ADD COLUMN campaign_id INTEGER;
ALTER TABLE faction_relationships ADD CONSTRAINT faction_relationships_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to item_effects
ALTER TABLE item_effects ADD COLUMN campaign_id INTEGER;
ALTER TABLE item_effects ADD CONSTRAINT item_effects_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to location_items
ALTER TABLE location_items ADD COLUMN campaign_id INTEGER;
ALTER TABLE location_items ADD CONSTRAINT location_items_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to quest_entities
ALTER TABLE quest_entities ADD COLUMN campaign_id INTEGER;
ALTER TABLE quest_entities ADD CONSTRAINT quest_entities_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to quest_locations
ALTER TABLE quest_locations ADD COLUMN campaign_id INTEGER;
ALTER TABLE quest_locations ADD CONSTRAINT quest_locations_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to race_cultures
ALTER TABLE race_cultures ADD COLUMN campaign_id INTEGER;
ALTER TABLE race_cultures ADD CONSTRAINT race_cultures_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Add campaign_id to sentient_item_properties
ALTER TABLE sentient_item_properties ADD COLUMN campaign_id INTEGER;
ALTER TABLE sentient_item_properties ADD CONSTRAINT sentient_item_properties_campaign_id_fkey 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Create indexes for performance
CREATE INDEX idx_entity_factions_campaign_id ON entity_factions(campaign_id);
CREATE INDEX idx_entity_items_campaign_id ON entity_items(campaign_id);
CREATE INDEX idx_entity_locations_campaign_id ON entity_locations(campaign_id);
CREATE INDEX idx_entity_relationships_campaign_id ON entity_relationships(campaign_id);
CREATE INDEX idx_faction_relationships_campaign_id ON faction_relationships(campaign_id);
CREATE INDEX idx_item_effects_campaign_id ON item_effects(campaign_id);
CREATE INDEX idx_location_items_campaign_id ON location_items(campaign_id);
CREATE INDEX idx_quest_entities_campaign_id ON quest_entities(campaign_id);
CREATE INDEX idx_quest_locations_campaign_id ON quest_locations(campaign_id);
CREATE INDEX idx_race_cultures_campaign_id ON race_cultures(campaign_id);
CREATE INDEX idx_sentient_item_properties_campaign_id ON sentient_item_properties(campaign_id);