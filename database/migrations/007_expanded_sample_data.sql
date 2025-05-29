-- 007_expanded_sample_data.sql
-- Comprehensive sample data for the expanded schema

-- ===================================================================
-- CHARACTER RACES AND BACKGROUNDS
-- ===================================================================

-- Insert common D&D races
INSERT INTO character_races (name, size_category, average_lifespan, common_alignments, physical_traits, cultural_traits, languages) VALUES
('Human', 'Medium', 80, ARRAY['any'], ARRAY['diverse appearance', 'adaptable'], ARRAY['ambitious', 'innovative', 'diverse cultures'], ARRAY['Common']),
('Elf', 'Medium', 750, ARRAY['chaotic good', 'chaotic neutral'], ARRAY['pointed ears', 'graceful', 'lithe'], ARRAY['long-lived perspective', 'magical affinity', 'artistic'], ARRAY['Common', 'Elvish']),
('Dwarf', 'Medium', 350, ARRAY['lawful good', 'lawful neutral'], ARRAY['short and stout', 'bearded', 'resilient'], ARRAY['craftsmanship', 'clan loyalty', 'mountain dwelling'], ARRAY['Common', 'Dwarvish']),
('Halfling', 'Small', 150, ARRAY['lawful good', 'neutral good'], ARRAY['small stature', 'curly hair', 'large feet'], ARRAY['hospitality', 'comfort-loving', 'brave when needed'], ARRAY['Common', 'Halfling']),
('Dragonborn', 'Medium', 80, ARRAY['any'], ARRAY['draconic heritage', 'breath weapon', 'scales'], ARRAY['honor-bound', 'clan-focused', 'proud heritage'], ARRAY['Common', 'Draconic']),
('Gnome', 'Small', 400, ARRAY['neutral good', 'chaotic good'], ARRAY['small and energetic', 'colorful hair', 'expressive'], ARRAY['curiosity', 'invention', 'forest connection'], ARRAY['Common', 'Gnomish']),
('Half-Elf', 'Medium', 180, ARRAY['chaotic good', 'chaotic neutral'], ARRAY['mixed heritage', 'human-elf features'], ARRAY['between two worlds', 'adaptable', 'diplomatic'], ARRAY['Common', 'Elvish']),
('Half-Orc', 'Medium', 75, ARRAY['chaotic neutral', 'chaotic good'], ARRAY['orcish features', 'large and strong'], ARRAY['struggle for acceptance', 'fierce emotions'], ARRAY['Common', 'Orcish']),
('Tiefling', 'Medium', 100, ARRAY['chaotic neutral', 'chaotic evil'], ARRAY['infernal heritage', 'horns', 'tail'], ARRAY['outsider status', 'self-reliant', 'mysterious'], ARRAY['Common', 'Infernal']);

-- Insert common backgrounds
INSERT INTO character_backgrounds (name, description, skill_proficiencies, equipment, feature_name, feature_description) VALUES
('Acolyte', 'Sheltered life in religious service', ARRAY['Insight', 'Religion'], '{"holy_symbol": 1, "prayer_book": 1, "incense": 5}', 'Shelter of the Faithful', 'Free lodging and support at temples'),
('Criminal', 'Experience in breaking the law', ARRAY['Deception', 'Stealth'], '{"crowbar": 1, "dark_clothes": 1, "belt_pouch": 1}', 'Criminal Contact', 'Connection to criminal underworld'),
('Folk Hero', 'Humble origins but destined for greatness', ARRAY['Animal Handling', 'Survival'], '{"artisan_tools": 1, "shovel": 1, "work_clothes": 1}', 'Rustic Hospitality', 'Common folk provide shelter and aid'),
('Noble', 'Wealthy and privileged upbringing', ARRAY['History', 'Persuasion'], '{"signet_ring": 1, "fine_clothes": 1, "scroll_of_pedigree": 1}', 'Position of Privilege', 'Access to high society and political influence'),
('Sage', 'Life spent in academic pursuit', ARRAY['Arcana', 'History'], '{"ink_and_quill": 1, "spellbook": 1, "scholarly_robes": 1}', 'Researcher', 'Know how to obtain information and access libraries'),
('Soldier', 'Military training and experience', ARRAY['Athletics', 'Intimidation'], '{"rank_insignia": 1, "trophy": 1, "deck_of_cards": 1}', 'Military Rank', 'Authority over enlisted soldiers and access to military facilities'),
('Guild Artisan', 'Member of a professional guild', ARRAY['Insight', 'Persuasion'], '{"artisan_tools": 1, "letter_of_introduction": 1, "traveler_clothes": 1}', 'Guild Membership', 'Support and privileges within your guild network'),
('Hermit', 'Secluded life of contemplation', ARRAY['Medicine', 'Religion'], '{"herbalism_kit": 1, "scroll_case": 1, "winter_blanket": 1}', 'Discovery', 'Unique and powerful discovery that others would want to learn'),
('Entertainer', 'Life performing for others', ARRAY['Acrobatics', 'Performance'], '{"musical_instrument": 1, "costume": 1, "love_letter": 1}', 'By Popular Demand', 'Free lodging and food at inns by performing'),
('Charlatan', 'Living by your wits and deception', ARRAY['Deception', 'Sleight of Hand'], '{"forgery_kit": 1, "disguise_kit": 1, "signet_ring_fake": 1}', 'False Identity', 'Created second identity with documentation and acquaintances');

-- ===================================================================
-- TRADE GOODS AND ECONOMY
-- ===================================================================

-- Insert common trade goods
INSERT INTO trade_goods (name, category, base_value, weight_pounds, rarity, seasonal_availability) VALUES
-- Food and Agricultural Products
('Wheat', 'food', 2.0, 1.0, 'common', ARRAY['summer', 'autumn']),
('Barley', 'food', 2.0, 1.0, 'common', ARRAY['summer', 'autumn']),
('Ale (barrel)', 'food', 20.0, 50.0, 'common', ARRAY['all']),
('Wine (bottle)', 'food', 10.0, 1.5, 'uncommon', ARRAY['autumn', 'winter']),
('Cheese (wheel)', 'food', 5.0, 4.0, 'common', ARRAY['all']),
('Salted Fish', 'food', 3.0, 2.0, 'common', ARRAY['all']),
('Spices (pound)', 'food', 50.0, 1.0, 'rare', ARRAY['all']),
('Honey (jar)', 'food', 8.0, 2.0, 'uncommon', ARRAY['summer', 'autumn']),

-- Raw Materials
('Iron Ore', 'raw_materials', 1.0, 10.0, 'common', ARRAY['all']),
('Copper Ore', 'raw_materials', 2.0, 10.0, 'common', ARRAY['all']),
('Silver Ore', 'raw_materials', 50.0, 10.0, 'uncommon', ARRAY['all']),
('Gold Ore', 'raw_materials', 500.0, 10.0, 'rare', ARRAY['all']),
('Lumber (board)', 'raw_materials', 0.2, 2.0, 'common', ARRAY['spring', 'summer', 'autumn']),
('Stone (block)', 'raw_materials', 1.0, 50.0, 'common', ARRAY['all']),
('Clay', 'raw_materials', 0.1, 5.0, 'common', ARRAY['all']),
('Wool (bundle)', 'raw_materials', 5.0, 3.0, 'common', ARRAY['spring']),

-- Crafted Goods
('Sword (iron)', 'crafted_goods', 20.0, 3.0, 'common', ARRAY['all']),
('Sword (steel)', 'crafted_goods', 50.0, 3.0, 'uncommon', ARRAY['all']),
('Chainmail', 'crafted_goods', 75.0, 20.0, 'uncommon', ARRAY['all']),
('Leather Armor', 'crafted_goods', 10.0, 10.0, 'common', ARRAY['all']),
('Pottery (set)', 'crafted_goods', 3.0, 5.0, 'common', ARRAY['all']),
('Rope (50 feet)', 'crafted_goods', 1.0, 10.0, 'common', ARRAY['all']),
('Canvas (bolt)', 'crafted_goods', 4.0, 8.0, 'common', ARRAY['all']),
('Books (common)', 'crafted_goods', 25.0, 5.0, 'uncommon', ARRAY['all']),

-- Luxuries
('Silk (bolt)', 'luxuries', 100.0, 2.0, 'rare', ARRAY['all']),
('Jewelry (fine)', 'luxuries', 500.0, 0.1, 'very_rare', ARRAY['all']),
('Perfume (bottle)', 'luxuries', 25.0, 0.5, 'rare', ARRAY['all']),
('Musical Instrument', 'luxuries', 75.0, 3.0, 'uncommon', ARRAY['all']),
('Art Object', 'luxuries', 200.0, 1.0, 'rare', ARRAY['all']),
('Exotic Furs', 'luxuries', 150.0, 5.0, 'rare', ARRAY['winter']);

-- ===================================================================
-- SAMPLE CAMPAIGN WITH COMPREHENSIVE DATA
-- ===================================================================

-- Insert a sample campaign to demonstrate the system
INSERT INTO campaigns (name, setting, themes, status) VALUES 
('The Shattered Crown', 'Medieval Fantasy', ARRAY['political intrigue', 'ancient magic', 'war'], 'ready');

-- Get the campaign ID for references
DO $$
DECLARE
    campaign_id INTEGER;
BEGIN
    SELECT id INTO campaign_id FROM campaigns WHERE name = 'The Shattered Crown';

    -- Calendar System
    INSERT INTO calendar_systems (campaign_id, name, months, weekdays, current_year, current_month, current_day) VALUES
    (campaign_id, 'Imperial Calendar', 
     '[
        {"name": "Frostmorn", "days": 31, "season": "winter"},
        {"name": "Thawing", "days": 28, "season": "winter"},
        {"name": "Bloomtide", "days": 31, "season": "spring"},
        {"name": "Verdant", "days": 30, "season": "spring"},
        {"name": "Sunturn", "days": 31, "season": "summer"},
        {"name": "Harvest", "days": 30, "season": "summer"},
        {"name": "Goldleaf", "days": 31, "season": "autumn"},
        {"name": "Cooling", "days": 30, "season": "autumn"},
        {"name": "Firstfrost", "days": 31, "season": "winter"},
        {"name": "Darkmoon", "days": 30, "season": "winter"}
     ]'::jsonb,
     '["Moonsday", "Sunsday", "Earthday", "Airday", "Fireday", "Waterday", "Starday"]'::jsonb,
     1247, 6, 15);

    -- Sample Locations with Hierarchical Structure
    INSERT INTO locations (campaign_id, name, type, description, parent_location_id, population, government_type, climate, terrain) VALUES
    (campaign_id, 'Kingdom of Aethermoor', 'kingdom', 'A prosperous kingdom known for its magical academies and trade routes', NULL, 500000, 'constitutional_monarchy', 'temperate', 'varied');
    
    -- Get the kingdom location ID for references
    DECLARE kingdom_id INTEGER;
    SELECT id INTO kingdom_id FROM locations WHERE campaign_id = campaign_id AND name = 'Kingdom of Aethermoor';
    
    INSERT INTO locations (campaign_id, name, type, description, parent_location_id, population, government_type, climate, terrain) VALUES
    (campaign_id, 'Goldenheart', 'capital_city', 'The magnificent capital city, seat of the Shattered Crown', kingdom_id, 85000, 'royal_council', 'temperate', 'hills');
    
    -- Get the capital location ID for references
    DECLARE capital_id INTEGER;
    SELECT id INTO capital_id FROM locations WHERE campaign_id = campaign_id AND name = 'Goldenheart';
    
    INSERT INTO locations (campaign_id, name, type, description, parent_location_id, population, government_type, climate, terrain) VALUES
    (campaign_id, 'Merchant Quarter', 'district', 'Bustling commercial center of the capital', capital_id, 25000, 'guild_council', 'temperate', 'urban'),
    (campaign_id, 'Noble Quarter', 'district', 'Where the wealthy and powerful reside', capital_id, 8000, 'noble_houses', 'temperate', 'urban'),
    (campaign_id, 'Silverbrook', 'town', 'A prosperous trading town along the Silver River', kingdom_id, 3500, 'elected_mayor', 'temperate', 'river_valley'),
    (campaign_id, 'Ironhold', 'fortress', 'Ancient dwarven fortress guarding the mountain passes', kingdom_id, 1200, 'military', 'cold', 'mountain'),
    (campaign_id, 'Whisperwood', 'forest', 'Ancient forest said to hold elven secrets', kingdom_id, 200, 'druidic_circle', 'temperate', 'forest'),
    (campaign_id, 'The Sunken Crypts', 'dungeon', 'Mysterious underground complex beneath Goldenheart', capital_id, 0, 'none', 'underground', 'constructed');

    -- Get location IDs for NPC references
    DECLARE silverbrook_id INTEGER;
    DECLARE merchant_quarter_id INTEGER;
    DECLARE crypts_id INTEGER;
    
    SELECT id INTO silverbrook_id FROM locations WHERE campaign_id = campaign_id AND name = 'Silverbrook';
    SELECT id INTO merchant_quarter_id FROM locations WHERE campaign_id = campaign_id AND name = 'Merchant Quarter';
    SELECT id INTO crypts_id FROM locations WHERE campaign_id = campaign_id AND name = 'The Sunken Crypts';

    -- Sample NPCs with Rich Detail
    INSERT INTO npcs (campaign_id, name, role, description, age, physical_description, alignment, occupation, current_location_id, home_location_id, goals, secrets) VALUES
    (campaign_id, 'Queen Lyralei Goldmane', 'monarch', 'The rightful queen whose crown was shattered in a magical catastrophe', 34, 'Tall, regal woman with golden hair and piercing blue eyes', 'lawful_good', 'Queen', capital_id, capital_id, ARRAY['reclaim full royal power', 'unite the kingdom', 'discover who shattered the crown'], ARRAY['the crown breaking was not an accident', 'she can still hear the crown calling to her']),
    (campaign_id, 'Lord Commander Marcus Ironwall', 'military_leader', 'Loyal commander of the royal guard, sworn to protect the queen', 45, 'Grizzled veteran with multiple scars and steel-gray hair', 'lawful_good', 'Military Commander', capital_id, capital_id, ARRAY['protect the queen', 'maintain order in the kingdom', 'train the next generation of guards'], ARRAY['secretly in love with the queen', 'knows more about the crown incident than he admits']),
    (campaign_id, 'Thessa Nightwhisper', 'spymaster', 'The queen mysterious advisor who deals in secrets and shadows', 28, 'Petite woman with dark hair and eyes that seem to see everything', 'chaotic_neutral', 'Spymaster', capital_id, capital_id, ARRAY['gather intelligence for the queen', 'eliminate threats to the kingdom', 'expand her network of informants'], ARRAY['she has agents in enemy kingdoms', 'her real name is not Thessa', 'she possesses a fragment of the shattered crown']),
    (campaign_id, 'Gareth Brewmaster', 'tavern_owner', 'Jovial halfling who runs the most popular tavern in Silverbrook', 52, 'Rotund halfling with a perpetual smile and ale-stained apron', 'neutral_good', 'Tavern Owner', silverbrook_id, silverbrook_id, ARRAY['brew the perfect ale', 'provide a safe haven for travelers', 'expand his tavern business'], ARRAY['he used to be an adventurer', 'hides treasure beneath his tavern', 'is secretly funding the local resistance']),
    (campaign_id, 'Sage Aldric Scrollseeker', 'scholar', 'Elderly human wizard studying the nature of the shattered crown', 67, 'Thin, elderly man with wild white hair and ink-stained fingers', 'lawful_neutral', 'Court Wizard', capital_id, capital_id, ARRAY['understand the crown magic', 'restore magical balance', 'preserve knowledge for future generations'], ARRAY['he knows how to reforge the crown', 'he was present when the crown shattered', 'has been having prophetic dreams']);

    -- Get organization type IDs
    DECLARE noble_quarter_id INTEGER;
    SELECT id INTO noble_quarter_id FROM locations WHERE campaign_id = campaign_id AND name = 'Noble Quarter';

    -- Sample Organizations
    INSERT INTO organizations (campaign_id, organization_type_id, name, description, headquarters_location_id, size_category, influence_level, primary_goals) VALUES
    (campaign_id, 2, 'Order of the Golden Sun', 'Religious order dedicated to bringing light and justice to the kingdom', capital_id, 'medium', 7, ARRAY['spread divine light', 'fight undead and demons', 'support the rightful monarchy']),
    (campaign_id, 1, 'Merchants Guild of Aethermoor', 'Powerful trade organization controlling most commerce in the kingdom', merchant_quarter_id, 'large', 8, ARRAY['increase trade profits', 'maintain trade route security', 'influence economic policy']),
    (campaign_id, 4, 'The Shadowhand Thieves', 'Criminal organization operating in the shadows of major cities', merchant_quarter_id, 'medium', 5, ARRAY['control black market trade', 'gather valuable information', 'avoid detection by authorities']),
    (campaign_id, 3, 'House Ravencrest', 'Ancient noble house with claims to the throne', noble_quarter_id, 'small', 6, ARRAY['claim the throne', 'restore ancient bloodlines', 'accumulate political power']);

    -- Get NPC IDs for quest references
    DECLARE sage_id INTEGER;
    DECLARE gareth_id INTEGER;
    DECLARE marcus_id INTEGER;
    DECLARE whisperwood_id INTEGER;
    
    SELECT id INTO sage_id FROM npcs WHERE campaign_id = campaign_id AND name = 'Sage Aldric Scrollseeker';
    SELECT id INTO gareth_id FROM npcs WHERE campaign_id = campaign_id AND name = 'Gareth Brewmaster';
    SELECT id INTO marcus_id FROM npcs WHERE campaign_id = campaign_id AND name = 'Lord Commander Marcus Ironwall';
    SELECT id INTO whisperwood_id FROM locations WHERE campaign_id = campaign_id AND name = 'Whisperwood';

    -- Sample Quest Hooks Connected to NPCs and Locations
    INSERT INTO quest_hooks (campaign_id, title, description, difficulty, patron_npc_id, target_location_id, urgency_level, complications) VALUES
    (campaign_id, 'The Missing Crown Fragment', 'Sage Aldric believes a fragment of the shattered crown lies hidden in the Sunken Crypts beneath the capital', 'hard', sage_id, crypts_id, 4, ARRAY['undead guardians protect the crypts', 'rival treasure hunters are also searching', 'the fragment may be cursed']),
    (campaign_id, 'Merchants Under Siege', 'Bandits have been attacking trade caravans between Goldenheart and Silverbrook, threatening the kingdom economy', 'medium', gareth_id, silverbrook_id, 3, ARRAY['bandits have inside information', 'some attacks may be politically motivated', 'local guards may be compromised']),
    (campaign_id, 'The Whisperwood Envoy', 'The elves of Whisperwood have sent an urgent message requesting aid against an ancient evil awakening in their forest', 'hard', NULL, whisperwood_id, 5, ARRAY['the evil predates recorded history', 'elves are traditionally isolationist', 'time is running out before the evil spreads']),
    (campaign_id, 'Noble Conspiracy', 'Lord Commander Marcus suspects that House Ravencrest is plotting against the queen', 'medium', marcus_id, noble_quarter_id, 4, ARRAY['evidence must be gathered carefully', 'accusations without proof could start a civil war', 'there may be other conspirators']);

    -- Get location IDs for backstory elements
    DECLARE ironhold_id INTEGER;
    SELECT id INTO ironhold_id FROM locations WHERE campaign_id = campaign_id AND name = 'Ironhold';

    -- Sample Backstory Elements for Player Characters
    INSERT INTO backstory_elements (campaign_id, element_type_id, player_character_name, element_name, description, importance_level, connected_npc_id, connected_location_id) VALUES
    (campaign_id, 1, 'Kael Brightblade', 'Sister Elena', 'Kael younger sister who joined the Order of the Golden Sun and has been missing for three months', 8, NULL, capital_id),
    (campaign_id, 4, 'Kael Brightblade', 'The Blackened Blade', 'A mysterious assassin who killed Kael mentor and bears a weapon of shadow magic', 9, NULL, NULL),
    (campaign_id, 2, 'Lyra Moonsong', 'Master Silvianus', 'The elven mage who taught Lyra the fundamentals of magic before disappearing into Whisperwood', 7, NULL, whisperwood_id),
    (campaign_id, 7, 'Thorin Ironforge', 'The Fall of Ironhold', 'Thorin was present when his ancestral home was overrun by shadow creatures, forcing the evacuation', 10, NULL, ironhold_id),
    (campaign_id, 6, 'Zara Swiftarrow', 'The Old Shrine', 'A ruined shrine in Whisperwood where Zara found her first magical arrow and met a mysterious figure', 6, NULL, whisperwood_id);

    -- Historical Events
    INSERT INTO historical_events (campaign_id, title, description, event_type, year, importance_level, consequences) VALUES
    (campaign_id, 'The Shattering of the Crown', 'The ancient crown of Aethermoor was mysteriously shattered during Queen Lyralei coronation ceremony', 'disaster', 1247, 10, 'The kingdom magical defenses weakened, political instability increased'),
    (campaign_id, 'The Great Trade Accord', 'Establishment of the major trade routes connecting all settlements in the kingdom', 'founding', 1203, 7, 'Economic prosperity increased, cultural exchange flourished'),
    (campaign_id, 'The Shadow War', 'A brief but devastating conflict against creatures of shadow that emerged from unknown portals', 'war', 1239, 8, 'Military reformed, magical research intensified, several locations still bear scars');

    -- Sample Magic Items
    INSERT INTO magic_items (campaign_id, name, item_type, rarity, description, current_location_id, historical_significance) VALUES
    (campaign_id, 'Shard of the Sundered Crown', 'artifact', 'legendary', 'A fragment of the original crown that still pulses with royal magic', crypts_id, 'Part of the original crown of Aethermoor, shattered three months ago'),
    (campaign_id, 'Ironwall Shield', 'shield', 'rare', 'The enchanted shield carried by the Ironwall family for generations', capital_id, 'Forged by dwarven masters, passed down through military leaders'),
    (campaign_id, 'Whisperwind Cloak', 'wondrous', 'uncommon', 'A cloak that seems to move with a breeze that only the wearer can feel', whisperwood_id, 'Woven by elven crafters using threads from the heart of Whisperwood');

    -- Sample Rumors
    INSERT INTO rumors (campaign_id, content, rumor_type, accuracy_level, origin_location_id, target_audience) VALUES
    (campaign_id, 'The queen crown was not destroyed by accident - it was sabotaged by someone close to her', 'secret', 8, capital_id, ARRAY['nobles', 'guards', 'spies']),
    (campaign_id, 'Strange lights have been seen dancing in Whisperwood at night, and the elves have stopped coming to market', 'news', 7, silverbrook_id, ARRAY['traders', 'travelers']),
    (campaign_id, 'The Merchants Guild is secretly funding both the bandits and the guards to inflate protection prices', 'gossip', 6, merchant_quarter_id, ARRAY['merchants', 'commoners']),
    (campaign_id, 'There treasure buried beneath every major building in Goldenheart, left by the previous dynasty', 'gossip', 2, capital_id, ARRAY['treasure hunters', 'criminals']);

END $$;