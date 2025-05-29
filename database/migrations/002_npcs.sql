-- 002_npcs.sql
-- NPCs table with relationships to campaigns

-- Create npcs table
CREATE TABLE npcs (
    id SERIAL PRIMARY KEY,
    campaign_id INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    role TEXT,
    description TEXT,
    personality JSONB DEFAULT '{}',
    stats JSONB DEFAULT '{}',
    secret_info TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for updated_at
CREATE TRIGGER update_npcs_updated_at BEFORE UPDATE
    ON npcs FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Create index for campaign_id for performance
CREATE INDEX idx_npcs_campaign_id ON npcs(campaign_id);

-- Create index on name for searching
CREATE INDEX idx_npcs_name ON npcs(name);