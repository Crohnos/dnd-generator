-- 005_sample_data.sql: Sample data for testing

-- Only insert if no campaigns exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM campaigns WHERE name = 'The Lost Mines of Phandelver') THEN
        -- Insert test campaign
        INSERT INTO campaigns (name, setting, themes, status, metadata)
        VALUES (
            'The Lost Mines of Phandelver',
            'The Forgotten Realms - Sword Coast',
            ARRAY['Adventure', 'Mystery', 'Dungeon Crawling'],
            'ready',
            '{"description": "A classic starter adventure where heroes uncover a lost mine filled with magical treasures."}'
        );
    END IF;
END $$;

-- Get the campaign ID for foreign key references
DO $$
DECLARE
    campaign_id INTEGER;
BEGIN
    SELECT id INTO campaign_id FROM campaigns WHERE name = 'The Lost Mines of Phandelver' LIMIT 1;
    
    -- Insert sample NPCs
    INSERT INTO npcs (campaign_id, name, role, description, personality, stats, secret_info)
    VALUES
    (
        campaign_id,
        'Gundren Rockseeker',
        'Quest Giver',
        'A dwarf entrepreneur seeking to reopen the lost Wave Echo Cave mine',
        '{"traits": ["Determined", "Entrepreneurial", "Secretive about his plans"], "race": "Dwarf", "class": "Commoner"}',
        '{"strength": 13, "dexterity": 10, "constitution": 14, "intelligence": 12, "wisdom": 11, "charisma": 10, "level": 2}',
        'One of three Rockseeker brothers who discovered the location of the lost mine'
    ),
    (
        campaign_id,
        'Sildar Hallwinter',
        'Ally',
        'A veteran warrior and agent of the Lords Alliance',
        '{"traits": ["Loyal", "Honorable", "Dedicated to law and order"], "race": "Human", "class": "Fighter"}',
        '{"strength": 16, "dexterity": 13, "constitution": 14, "intelligence": 12, "wisdom": 11, "charisma": 13, "level": 5}',
        'Searching for his missing friend Iarno Albrek'
    ),
    (
        campaign_id,
        'Klarg',
        'Villain',
        'A bugbear leader of the Cragmaw goblins',
        '{"traits": ["Brutal", "Cunning", "Greedy"], "race": "Bugbear", "class": "Warrior"}',
        '{"strength": 17, "dexterity": 14, "constitution": 13, "intelligence": 8, "wisdom": 11, "charisma": 9, "level": 3}',
        'Has a pet wolf named Ripper'
    );

    -- Insert sample locations
    INSERT INTO locations (campaign_id, name, type, description, properties)
    VALUES
    (
        campaign_id,
        'Phandalin',
        'Town',
        'A small frontier town built on the ruins of an older settlement',
        '{"population": 200, "notable_buildings": ["Stonehill Inn", "Barthens Provisions", "Shrine of Luck"], "atmosphere": "Frontier town with hidden tensions"}'
    ),
    (
        campaign_id,
        'Cragmaw Hideout',
        'Dungeon',
        'A cave complex used by goblins to ambush travelers',
        '{"rooms": 6, "inhabitants": "Cragmaw Goblins", "treasure": "Stolen goods and supplies", "traps": ["Flood trap", "Guard posts"]}'
    ),
    (
        campaign_id,
        'Wave Echo Cave',
        'Dungeon',
        'The lost mine of the Phandelver Pact, filled with magical energy',
        '{"rooms": 20, "magical_forge": true, "inhabitants": "Undead and monsters", "treasure": "Spell Forge and magical items"}'
    );

    -- Insert sample quest hooks  
    INSERT INTO quest_hooks (campaign_id, title, description, difficulty, reward, status)
    VALUES
    (
        campaign_id,
        'Escort to Phandalin',
        'Escort a wagon of supplies from Neverwinter to Phandalin for Gundren Rockseeker',
        'easy',
        '10 gold per person',
        'available'
    ),
    (
        campaign_id,
        'Rescue Gundren',
        'Find and rescue Gundren Rockseeker who has been captured by goblins',
        'medium',
        '50 gold and information about Wave Echo Cave',
        'available'
    ),
    (
        campaign_id,
        'Clear Wave Echo Cave',
        'Explore and secure the lost mine of Wave Echo Cave',
        'hard',
        '500 gold, magic items, and 10% share of mine profits',
        'available'
    );

    -- Insert sample encounters
    INSERT INTO encounters (campaign_id, title, description, difficulty, creatures, environmental_factors)
    VALUES
    (
        campaign_id,
        'Goblin Ambush',
        'Goblins attack the party on the Triboar Trail',
        'easy',
        '[{"name": "Goblin", "count": 4, "hp": 7, "ac": 15}]',
        'Dense forest provides cover for ambushers'
    ),
    (
        campaign_id,
        'Klargs Chamber',
        'Face off against Klarg and his pet wolf in the depths of Cragmaw Hideout',
        'medium',
        '[{"name": "Klarg (Bugbear)", "count": 1, "hp": 27, "ac": 16}, {"name": "Ripper (Wolf)", "count": 1, "hp": 11, "ac": 13}, {"name": "Goblin", "count": 2, "hp": 7, "ac": 15}]',
        'Elevated platform gives Klarg tactical advantage'
    );

    -- Link NPCs to locations using location_npcs junction table
    -- First get NPC and location IDs
    INSERT INTO location_npcs (location_id, npc_id, relationship_type)
    SELECT 
        l.id,
        n.id,
        CASE 
            WHEN n.name = 'Klarg' THEN 'resident'
            WHEN n.name = 'Sildar Hallwinter' THEN 'visitor'
        END
    FROM locations l
    CROSS JOIN npcs n
    WHERE l.campaign_id = n.campaign_id
    AND (
        (l.name = 'Cragmaw Hideout' AND n.name = 'Klarg') OR
        (l.name = 'Phandalin' AND n.name = 'Sildar Hallwinter')
    );

    -- Update location connections
    UPDATE locations SET connections = ARRAY[
        (SELECT id FROM locations l2 WHERE l2.campaign_id = locations.campaign_id AND l2.name = 'Cragmaw Hideout')
    ] WHERE name = 'Phandalin';

END $$;