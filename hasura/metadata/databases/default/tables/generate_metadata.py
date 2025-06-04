#!/usr/bin/env python3
"""Generate Hasura metadata YAML files for all new tables"""

import os

# Define all tables with their relationships
TABLES = {
    # Phase 1A: Core World Systems
    "historical_periods": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ]
    },
    "economic_systems": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("region", "region_id", "geography_regions")
        ]
    },
    "legal_systems": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("region", "region_id", "geography_regions")
        ]
    },
    "celestial_bodies": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ]
    },
    
    # Phase 1B: Character Building
    "races": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("parent_race", "parent_race_id")
        ],
        "array_relationships": [
            ("subraces", "parent_race_id", "races"),
            ("entities", "race_id", "entities"),
            ("race_cultures", "race_id", "race_cultures")
        ]
    },
    "character_classes": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("parent_class", "parent_class_id")
        ],
        "array_relationships": [
            ("subclasses", "parent_class_id", "character_classes"),
            ("entities", "class_id", "entities")
        ]
    },
    "feats": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ]
    },
    "backgrounds": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ],
        "array_relationships": [
            ("entities", "background_id", "entities")
        ]
    },
    
    # Phase 1C: Social Framework
    "languages": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ]
    },
    "cultures": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("primary_race", "primary_race_id", "races"),
            ("geography_region", "geography_region_id", "geography_regions")
        ],
        "array_relationships": [
            ("race_cultures", "culture_id", "race_cultures")
        ]
    },
    "factions": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ],
        "array_relationships": [
            ("entity_factions", "faction_id", "entity_factions"),
            ("faction_relationships_as_faction1", "faction1_id", "faction_relationships"),
            ("faction_relationships_as_faction2", "faction2_id", "faction_relationships")
        ]
    },
    "pantheons": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ],
        "array_relationships": [
            ("deities", "pantheon_id", "deities")
        ]
    },
    "deities": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("pantheon", "pantheon_id", "pantheons")
        ],
        "array_relationships": [
            ("temples", "deity_id", "temples")
        ]
    },
    
    # Phase 2A: Entities
    "entities": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("race", "race_id", "races"),
            ("class", "class_id", "character_classes"),
            ("background", "background_id", "backgrounds")
        ],
        "array_relationships": [
            ("entity_relationships_as_entity1", "entity1_id", "entity_relationships"),
            ("entity_relationships_as_entity2", "entity2_id", "entity_relationships"),
            ("entity_locations", "entity_id", "entity_locations"),
            ("entity_factions", "entity_id", "entity_factions"),
            ("entity_items", "entity_id", "entity_items"),
            ("owned_shops", "owner_entity_id", "shops"),
            ("owned_taverns", "owner_entity_id", "taverns"),
            ("temples_as_high_priest", "high_priest_entity_id", "temples"),
            ("quest_entities", "entity_id", "quest_entities")
        ]
    },
    
    # Phase 2B: Locations (updated)
    "locations": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("parent_location", "parent_location_id"),
            ("geography_region", "geography_region_id", "geography_regions")
        ],
        "array_relationships": [
            ("child_locations", "parent_location_id", "locations"),
            ("entity_locations", "location_id", "entity_locations"),
            ("location_items", "location_id", "location_items"),
            ("encounters", "location_id", "encounters"),
            ("quest_locations", "location_id", "quest_locations"),
            ("dungeons", "location_id", "dungeons"),
            ("buildings", "location_id", "buildings")
        ]
    },
    "dungeons": {
        "object_relationships": [
            ("location", "location_id", "locations")
        ]
    },
    "buildings": {
        "object_relationships": [
            ("location", "location_id", "locations")
        ],
        "array_relationships": [
            ("shops", "building_id", "shops"),
            ("taverns", "building_id", "taverns"),
            ("temples", "building_id", "temples")
        ]
    },
    "shops": {
        "object_relationships": [
            ("building", "building_id", "buildings"),
            ("owner", "owner_entity_id", "entities")
        ]
    },
    "taverns": {
        "object_relationships": [
            ("building", "building_id", "buildings"),
            ("owner", "owner_entity_id", "entities")
        ]
    },
    "temples": {
        "object_relationships": [
            ("building", "building_id", "buildings"),
            ("deity", "deity_id", "deities"),
            ("high_priest", "high_priest_entity_id", "entities")
        ]
    },
    
    # Phase 2C: Items
    "items": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ],
        "array_relationships": [
            ("item_effects", "item_id", "item_effects"),
            ("sentient_properties", "item_id", "sentient_item_properties"),
            ("entity_items", "item_id", "entity_items"),
            ("location_items", "item_id", "location_items")
        ]
    },
    "item_effects": {
        "object_relationships": [
            ("item", "item_id", "items")
        ]
    },
    "sentient_item_properties": {
        "object_relationships": [
            ("item", "item_id", "items")
        ]
    },
    
    # Phase 3A: Quests & Encounters
    "quest_hooks": {
        "object_relationships": [
            ("campaign", "campaign_id")
        ],
        "array_relationships": [
            ("quest_entities", "quest_hook_id", "quest_entities"),
            ("quest_locations", "quest_hook_id", "quest_locations")
        ]
    },
    "encounters": {
        "object_relationships": [
            ("campaign", "campaign_id"),
            ("location", "location_id", "locations")
        ]
    },
    
    # Relationship Tables
    "entity_relationships": {
        "object_relationships": [
            ("entity1", "entity1_id", "entities"),
            ("entity2", "entity2_id", "entities")
        ]
    },
    "entity_locations": {
        "object_relationships": [
            ("entity", "entity_id", "entities"),
            ("location", "location_id", "locations")
        ]
    },
    "entity_factions": {
        "object_relationships": [
            ("entity", "entity_id", "entities"),
            ("faction", "faction_id", "factions")
        ]
    },
    "entity_items": {
        "object_relationships": [
            ("entity", "entity_id", "entities"),
            ("item", "item_id", "items")
        ]
    },
    "location_items": {
        "object_relationships": [
            ("location", "location_id", "locations"),
            ("item", "item_id", "items")
        ]
    },
    "quest_entities": {
        "object_relationships": [
            ("quest_hook", "quest_hook_id", "quest_hooks"),
            ("entity", "entity_id", "entities")
        ]
    },
    "quest_locations": {
        "object_relationships": [
            ("quest_hook", "quest_hook_id", "quest_hooks"),
            ("location", "location_id", "locations")
        ]
    },
    "faction_relationships": {
        "object_relationships": [
            ("faction1", "faction1_id", "factions"),
            ("faction2", "faction2_id", "factions")
        ]
    },
    "race_cultures": {
        "object_relationships": [
            ("race", "race_id", "races"),
            ("culture", "culture_id", "cultures")
        ]
    }
}

def generate_yaml(table_name, config):
    """Generate YAML content for a table"""
    yaml_content = f"""table:
  name: {table_name}
  schema: public"""
    
    # Add object relationships
    if "object_relationships" in config and config["object_relationships"]:
        yaml_content += "\nobject_relationships:"
        for rel in config["object_relationships"]:
            if len(rel) == 2:
                name, column = rel
                yaml_content += f"""
  - name: {name}
    using:
      foreign_key_constraint_on: {column}"""
            else:
                name, column, table = rel
                yaml_content += f"""
  - name: {name}
    using:
      foreign_key_constraint_on: {column}"""
    
    # Add array relationships
    if "array_relationships" in config and config["array_relationships"]:
        yaml_content += "\narray_relationships:"
        for name, column, table in config["array_relationships"]:
            yaml_content += f"""
  - name: {name}
    using:
      foreign_key_constraint_on:
        column: {column}
        table:
          name: {table}
          schema: public"""
    
    # Add permissions
    yaml_content += """
select_permissions:
  - role: public
    permission:
      columns: '*'
      filter: {}
insert_permissions:
  - role: public
    permission:
      check: {}
      columns: '*'
update_permissions:
  - role: public
    permission:
      columns: '*'
      filter: {}
      check: {}"""
    
    return yaml_content

def main():
    # Generate YAML files for all tables
    for table_name, config in TABLES.items():
        filename = f"public_{table_name}.yaml"
        yaml_content = generate_yaml(table_name, config)
        
        with open(filename, 'w') as f:
            f.write(yaml_content)
        
        print(f"Generated {filename}")
    
    # Update tables.yaml
    tables_yaml_content = """- "!include public_campaigns.yaml"
"""
    
    # Add existing tables (keeping old ones for now)
    existing_tables = ["npcs", "locations", "location_npcs", "quest_hooks", "encounters"]
    for table in existing_tables:
        if table not in TABLES:  # Don't duplicate
            tables_yaml_content += f'- "!include public_{table}.yaml"\n'
    
    # Add all new tables
    for table_name in sorted(TABLES.keys()):
        tables_yaml_content += f'- "!include public_{table_name}.yaml"\n'
    
    with open('tables.yaml', 'w') as f:
        f.write(tables_yaml_content)
    
    print(f"\nUpdated tables.yaml with {len(TABLES)} new tables")

if __name__ == "__main__":
    main()