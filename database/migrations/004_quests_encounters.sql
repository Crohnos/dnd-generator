-- 004_quests_encounters.sql
-- Quest hooks and encounters tables

-- Create quest_hooks table
CREATE TABLE quest_hooks (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    difficulty TEXT DEFAULT 'medium',
    reward TEXT,
    related_npc_ids INTEGER[] DEFAULT '{}',
    related_location_ids INTEGER[] DEFAULT '{}',
    status TEXT DEFAULT 'available' CHECK (status IN ('available', 'active', 'completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for updated_at
CREATE TRIGGER update_quest_hooks_updated_at BEFORE UPDATE
    ON quest_hooks FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Create index for campaign_id for performance
CREATE INDEX idx_quest_hooks_campaign_id ON quest_hooks(campaign_id);

-- Create index on status for filtering
CREATE INDEX idx_quest_hooks_status ON quest_hooks(status);

-- Create encounters table
CREATE TABLE encounters (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    location_id INTEGER REFERENCES locations(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    description TEXT,
    difficulty TEXT DEFAULT 'medium',
    creatures JSONB DEFAULT '[]',
    environmental_factors TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for updated_at
CREATE TRIGGER update_encounters_updated_at BEFORE UPDATE
    ON encounters FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Create index for campaign_id for performance
CREATE INDEX idx_encounters_campaign_id ON encounters(campaign_id);

-- Create index for location_id for filtering
CREATE INDEX idx_encounters_location_id ON encounters(location_id);