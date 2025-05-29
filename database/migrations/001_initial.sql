-- 001_initial.sql
-- Core campaigns table with trigger functions

-- Create update_updated_at_column function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create campaigns table
CREATE TABLE campaigns (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    setting TEXT,
    themes TEXT[] DEFAULT '{}',
    player_characters JSONB DEFAULT '[]',
    status TEXT DEFAULT 'generating' CHECK (status IN ('generating', 'ready', 'error')),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for updated_at
CREATE TRIGGER update_campaigns_updated_at BEFORE UPDATE
    ON campaigns FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_at_column();

-- Create index on status for performance
CREATE INDEX idx_campaigns_status ON campaigns(status);

-- Create index on created_at for sorting
CREATE INDEX idx_campaigns_created_at ON campaigns(created_at DESC);