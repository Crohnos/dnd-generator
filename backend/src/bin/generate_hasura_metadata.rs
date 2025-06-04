use std::fs;
use std::path::Path;

#[derive(Clone)]
struct TableRelationship {
    name: &'static str,
    column: &'static str,
    target_table: Option<&'static str>,
}

struct TableConfig {
    name: &'static str,
    object_relationships: Vec<TableRelationship>,
    array_relationships: Vec<TableRelationship>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tables = vec![
        // Phase 1A: Core World Systems
        TableConfig {
            name: "calendar_systems",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "planes",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "geography_regions", column: "plane_id", target_table: Some("geography_regions") },
            ],
        },
        TableConfig {
            name: "geography_regions",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "parent_region", column: "parent_region_id", target_table: None },
                TableRelationship { name: "plane", column: "plane_id", target_table: Some("planes") },
            ],
            array_relationships: vec![
                TableRelationship { name: "child_regions", column: "parent_region_id", target_table: Some("geography_regions") },
                TableRelationship { name: "locations", column: "geography_region_id", target_table: Some("locations") },
                TableRelationship { name: "economic_systems", column: "region_id", target_table: Some("economic_systems") },
                TableRelationship { name: "legal_systems", column: "region_id", target_table: Some("legal_systems") },
                TableRelationship { name: "cultures", column: "geography_region_id", target_table: Some("cultures") },
            ],
        },
        TableConfig {
            name: "historical_periods",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "economic_systems",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "region", column: "region_id", target_table: Some("geography_regions") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "legal_systems",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "region", column: "region_id", target_table: Some("geography_regions") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "celestial_bodies",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![],
        },
        
        // Phase 1B: Character Building
        TableConfig {
            name: "races",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "parent_race", column: "parent_race_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "subraces", column: "parent_race_id", target_table: Some("races") },
                TableRelationship { name: "entities", column: "race_id", target_table: Some("entities") },
                TableRelationship { name: "race_cultures", column: "race_id", target_table: Some("race_cultures") },
            ],
        },
        TableConfig {
            name: "character_classes",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "parent_class", column: "parent_class_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "subclasses", column: "parent_class_id", target_table: Some("character_classes") },
                TableRelationship { name: "entities", column: "class_id", target_table: Some("entities") },
            ],
        },
        TableConfig {
            name: "feats",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "backgrounds",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "entities", column: "background_id", target_table: Some("entities") },
            ],
        },
        
        // Phase 1C: Social Framework
        TableConfig {
            name: "languages",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "cultures",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "primary_race", column: "primary_race_id", target_table: Some("races") },
                TableRelationship { name: "geography_region", column: "geography_region_id", target_table: Some("geography_regions") },
            ],
            array_relationships: vec![
                TableRelationship { name: "race_cultures", column: "culture_id", target_table: Some("race_cultures") },
            ],
        },
        TableConfig {
            name: "factions",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "entity_factions", column: "faction_id", target_table: Some("entity_factions") },
                TableRelationship { name: "faction_relationships_as_faction1", column: "faction1_id", target_table: Some("faction_relationships") },
                TableRelationship { name: "faction_relationships_as_faction2", column: "faction2_id", target_table: Some("faction_relationships") },
            ],
        },
        TableConfig {
            name: "pantheons",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "deities", column: "pantheon_id", target_table: Some("deities") },
            ],
        },
        TableConfig {
            name: "deities",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "pantheon", column: "pantheon_id", target_table: Some("pantheons") },
            ],
            array_relationships: vec![
                TableRelationship { name: "temples", column: "deity_id", target_table: Some("temples") },
            ],
        },
        
        // Phase 2A: Entities
        TableConfig {
            name: "entities",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "race", column: "race_id", target_table: Some("races") },
                TableRelationship { name: "class", column: "class_id", target_table: Some("character_classes") },
                TableRelationship { name: "background", column: "background_id", target_table: Some("backgrounds") },
            ],
            array_relationships: vec![
                TableRelationship { name: "entity_relationships_as_entity1", column: "entity1_id", target_table: Some("entity_relationships") },
                TableRelationship { name: "entity_relationships_as_entity2", column: "entity2_id", target_table: Some("entity_relationships") },
                TableRelationship { name: "entity_locations", column: "entity_id", target_table: Some("entity_locations") },
                TableRelationship { name: "entity_factions", column: "entity_id", target_table: Some("entity_factions") },
                TableRelationship { name: "entity_items", column: "entity_id", target_table: Some("entity_items") },
                TableRelationship { name: "owned_shops", column: "owner_entity_id", target_table: Some("shops") },
                TableRelationship { name: "owned_taverns", column: "owner_entity_id", target_table: Some("taverns") },
                TableRelationship { name: "temples_as_high_priest", column: "high_priest_entity_id", target_table: Some("temples") },
                TableRelationship { name: "quest_entities", column: "entity_id", target_table: Some("quest_entities") },
            ],
        },
        
        // Phase 2B: Locations
        TableConfig {
            name: "dungeons",
            object_relationships: vec![
                TableRelationship { name: "location", column: "location_id", target_table: Some("locations") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "buildings",
            object_relationships: vec![
                TableRelationship { name: "location", column: "location_id", target_table: Some("locations") },
            ],
            array_relationships: vec![
                TableRelationship { name: "shops", column: "building_id", target_table: Some("shops") },
                TableRelationship { name: "taverns", column: "building_id", target_table: Some("taverns") },
                TableRelationship { name: "temples", column: "building_id", target_table: Some("temples") },
            ],
        },
        TableConfig {
            name: "shops",
            object_relationships: vec![
                TableRelationship { name: "building", column: "building_id", target_table: Some("buildings") },
                TableRelationship { name: "owner", column: "owner_entity_id", target_table: Some("entities") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "taverns",
            object_relationships: vec![
                TableRelationship { name: "building", column: "building_id", target_table: Some("buildings") },
                TableRelationship { name: "owner", column: "owner_entity_id", target_table: Some("entities") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "temples",
            object_relationships: vec![
                TableRelationship { name: "building", column: "building_id", target_table: Some("buildings") },
                TableRelationship { name: "deity", column: "deity_id", target_table: Some("deities") },
                TableRelationship { name: "high_priest", column: "high_priest_entity_id", target_table: Some("entities") },
            ],
            array_relationships: vec![],
        },
        
        // Phase 2C: Items
        TableConfig {
            name: "items",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "item_effects", column: "item_id", target_table: Some("item_effects") },
                TableRelationship { name: "sentient_properties", column: "item_id", target_table: Some("sentient_item_properties") },
                TableRelationship { name: "entity_items", column: "item_id", target_table: Some("entity_items") },
                TableRelationship { name: "location_items", column: "item_id", target_table: Some("location_items") },
            ],
        },
        TableConfig {
            name: "item_effects",
            object_relationships: vec![
                TableRelationship { name: "item", column: "item_id", target_table: Some("items") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "sentient_item_properties",
            object_relationships: vec![
                TableRelationship { name: "item", column: "item_id", target_table: Some("items") },
            ],
            array_relationships: vec![],
        },
        
        // Relationship Tables
        TableConfig {
            name: "entity_relationships",
            object_relationships: vec![
                TableRelationship { name: "entity1", column: "entity1_id", target_table: Some("entities") },
                TableRelationship { name: "entity2", column: "entity2_id", target_table: Some("entities") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "entity_locations",
            object_relationships: vec![
                TableRelationship { name: "entity", column: "entity_id", target_table: Some("entities") },
                TableRelationship { name: "location", column: "location_id", target_table: Some("locations") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "entity_factions",
            object_relationships: vec![
                TableRelationship { name: "entity", column: "entity_id", target_table: Some("entities") },
                TableRelationship { name: "faction", column: "faction_id", target_table: Some("factions") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "entity_items",
            object_relationships: vec![
                TableRelationship { name: "entity", column: "entity_id", target_table: Some("entities") },
                TableRelationship { name: "item", column: "item_id", target_table: Some("items") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "location_items",
            object_relationships: vec![
                TableRelationship { name: "location", column: "location_id", target_table: Some("locations") },
                TableRelationship { name: "item", column: "item_id", target_table: Some("items") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "quest_entities",
            object_relationships: vec![
                TableRelationship { name: "quest_hook", column: "quest_hook_id", target_table: Some("quest_hooks") },
                TableRelationship { name: "entity", column: "entity_id", target_table: Some("entities") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "quest_locations",
            object_relationships: vec![
                TableRelationship { name: "quest_hook", column: "quest_hook_id", target_table: Some("quest_hooks") },
                TableRelationship { name: "location", column: "location_id", target_table: Some("locations") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "faction_relationships",
            object_relationships: vec![
                TableRelationship { name: "faction1", column: "faction1_id", target_table: Some("factions") },
                TableRelationship { name: "faction2", column: "faction2_id", target_table: Some("factions") },
            ],
            array_relationships: vec![],
        },
        TableConfig {
            name: "race_cultures",
            object_relationships: vec![
                TableRelationship { name: "race", column: "race_id", target_table: Some("races") },
                TableRelationship { name: "culture", column: "culture_id", target_table: Some("cultures") },
            ],
            array_relationships: vec![],
        },
    ];

    // Create output directory
    let output_dir = Path::new("hasura/metadata/databases/default/tables");
    fs::create_dir_all(output_dir)?;

    // Generate YAML files for each table
    for table in &tables {
        let yaml_content = generate_yaml(table);
        let filename = format!("public_{}.yaml", table.name);
        let filepath = output_dir.join(&filename);
        
        fs::write(&filepath, yaml_content)?;
        println!("Generated {}", filename);
    }

    // Update tables.yaml
    let mut tables_yaml_content = String::from("- \"!include public_campaigns.yaml\"\n");
    
    // Add all new tables
    for table in &tables {
        tables_yaml_content.push_str(&format!("- \"!include public_{}.yaml\"\n", table.name));
    }
    
    // Also update the existing tables that need relationship updates
    let updated_tables = vec![
        TableConfig {
            name: "locations",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "parent_location", column: "parent_location_id", target_table: None },
                TableRelationship { name: "geography_region", column: "geography_region_id", target_table: Some("geography_regions") },
            ],
            array_relationships: vec![
                TableRelationship { name: "child_locations", column: "parent_location_id", target_table: Some("locations") },
                TableRelationship { name: "entity_locations", column: "location_id", target_table: Some("entity_locations") },
                TableRelationship { name: "location_items", column: "location_id", target_table: Some("location_items") },
                TableRelationship { name: "encounters", column: "location_id", target_table: Some("encounters") },
                TableRelationship { name: "quest_locations", column: "location_id", target_table: Some("quest_locations") },
                TableRelationship { name: "dungeons", column: "location_id", target_table: Some("dungeons") },
                TableRelationship { name: "buildings", column: "location_id", target_table: Some("buildings") },
            ],
        },
        TableConfig {
            name: "quest_hooks",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
            ],
            array_relationships: vec![
                TableRelationship { name: "quest_entities", column: "quest_hook_id", target_table: Some("quest_entities") },
                TableRelationship { name: "quest_locations", column: "quest_hook_id", target_table: Some("quest_locations") },
            ],
        },
        TableConfig {
            name: "encounters",
            object_relationships: vec![
                TableRelationship { name: "campaign", column: "campaign_id", target_table: None },
                TableRelationship { name: "location", column: "location_id", target_table: Some("locations") },
            ],
            array_relationships: vec![],
        },
    ];
    
    // Generate updated versions
    for table in &updated_tables {
        let yaml_content = generate_yaml(table);
        let filename = format!("public_{}.yaml", table.name);
        let filepath = output_dir.join(&filename);
        
        fs::write(&filepath, yaml_content)?;
        println!("Updated {}", filename);
        
        // Only add to tables.yaml if not already in the main list
        if !tables.iter().any(|t| t.name == table.name) {
            tables_yaml_content.push_str(&format!("- \"!include public_{}.yaml\"\n", table.name));
        }
    }
    
    // Remove old npcs and location_npcs tables from tables.yaml
    // (they're replaced by entities and entity_locations)
    
    let tables_yaml_path = output_dir.join("tables.yaml");
    fs::write(&tables_yaml_path, tables_yaml_content)?;
    println!("\nUpdated tables.yaml with {} total tables", tables.len() + updated_tables.len());

    Ok(())
}

fn generate_yaml(table: &TableConfig) -> String {
    let mut yaml = format!("table:\n  name: {}\n  schema: public", table.name);
    
    // Add object relationships
    if !table.object_relationships.is_empty() {
        yaml.push_str("\nobject_relationships:");
        for rel in &table.object_relationships {
            yaml.push_str(&format!("\n  - name: {}\n    using:\n      foreign_key_constraint_on: {}", 
                rel.name, rel.column));
        }
    }
    
    // Add array relationships
    if !table.array_relationships.is_empty() {
        yaml.push_str("\narray_relationships:");
        for rel in &table.array_relationships {
            yaml.push_str(&format!(
                "\n  - name: {}\n    using:\n      foreign_key_constraint_on:\n        column: {}\n        table:\n          name: {}\n          schema: public",
                rel.name, rel.column, rel.target_table.unwrap_or(table.name)
            ));
        }
    }
    
    // Add permissions
    yaml.push_str(r#"
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
      check: {}"#);
    
    yaml
}