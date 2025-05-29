-- 003_locations.sql
-- Locations table and location_npcs junction table

-- Create locations table
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT,
    description TEXT,
    connections INTEGER[] DEFAULT '{}',
    properties JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for updated_at
CREATE TRIGGER update_locations_updated_at BEFORE UPDATE
    ON locations FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Create index for campaign_id for performance
CREATE INDEX idx_locations_campaign_id ON locations(campaign_id);

-- Create index on name for searching
CREATE INDEX idx_locations_name ON locations(name);

-- Create many-to-many junction table for NPCs and locations
CREATE TABLE location_npcs (
    id SERIAL PRIMARY KEY,
    location_id INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    npc_id INTEGER NOT NULL REFERENCES npcs(id) ON DELETE CASCADE,
    relationship_type TEXT DEFAULT 'resident',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(location_id, npc_id)
);

-- Create indexes for junction table
CREATE INDEX idx_location_npcs_location_id ON location_npcs(location_id);
CREATE INDEX idx_location_npcs_npc_id ON location_npcs(npc_id);