use crate::error::{ApiError, ApiResult};
use crate::services::GraphQLClient;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use tracing::{debug, info, warn};

const INTROSPECTION_QUERY: &str = r#"
query IntrospectionQuery {
  __schema {
    types {
      name
      kind
      inputFields {
        name
        type {
          name
          kind
          ofType {
            name
            kind
            ofType {
              name
              kind
            }
          }
        }
      }
    }
  }
}
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLSchema {
    pub types: Vec<GraphQLType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLType {
    pub name: String,
    pub kind: String,
    #[serde(rename = "inputFields")]
    pub input_fields: Option<Vec<GraphQLInputField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLInputField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: GraphQLTypeRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLTypeRef {
    pub name: Option<String>,
    pub kind: String,
    #[serde(rename = "ofType")]
    pub of_type: Option<Box<GraphQLTypeRef>>,
}

#[derive(Debug, Clone)]
pub struct HasuraSchemaGenerator {
    graphql_client: GraphQLClient,
    schema_cache: HashMap<String, Value>,
    table_schemas: HashMap<String, Value>,
}

impl HasuraSchemaGenerator {
    pub fn new(graphql_client: GraphQLClient) -> Self {
        Self {
            graphql_client,
            schema_cache: HashMap::new(),
            table_schemas: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self) -> ApiResult<()> {
        info!("Initializing Hasura schema generator...");
        
        // Execute introspection query
        let response = self.graphql_client.execute(INTROSPECTION_QUERY, None).await?;
        
        // Parse the schema
        let schema_data = response
            .get("__schema")
            .ok_or_else(|| ApiError::BadRequest("No __schema in introspection response".to_string()))?;
            
        let types_array = schema_data
            .get("types")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ApiError::BadRequest("No types array in schema".to_string()))?;

        let schema: GraphQLSchema = serde_json::from_value(json!({
            "types": types_array
        }))
        .map_err(|e| ApiError::BadRequest(format!("Failed to parse GraphQL schema: {}", e)))?;

        // Process insert input types and generate JSON schemas
        let mut processed_count = 0;
        for graphql_type in &schema.types {
            if graphql_type.name.ends_with("_insert_input") && graphql_type.kind == "INPUT_OBJECT" {
                let table_name = graphql_type.name.replace("_insert_input", "");
                
                if let Some(json_schema) = self.convert_to_json_schema(graphql_type) {
                    debug!("Generated schema for table: {}", table_name);
                    self.schema_cache.insert(graphql_type.name.clone(), json_schema.clone());
                    self.table_schemas.insert(table_name, json_schema);
                    processed_count += 1;
                } else {
                    warn!("Failed to convert schema for table: {}", table_name);
                }
            }
        }

        info!("Successfully generated {} JSON schemas from Hasura introspection", processed_count);
        
        // Log available tables for debugging
        let table_names: Vec<String> = self.table_schemas.keys().cloned().collect();
        debug!("Available table schemas: {:?}", table_names);
        
        Ok(())
    }

    fn convert_to_json_schema(&self, graphql_type: &GraphQLType) -> Option<Value> {
        let input_fields = graphql_type.input_fields.as_ref()?;
        
        let mut properties = Map::new();
        let mut required = Vec::new();

        for field in input_fields {
            // Skip internal Hasura fields
            if field.name.starts_with("_") {
                continue;
            }

            let (json_type, is_required) = self.convert_graphql_type_to_json(&field.field_type);
            properties.insert(field.name.clone(), json_type);
            
            if is_required {
                required.push(field.name.clone());
            }
        }

        Some(json!({
            "type": "object",
            "properties": properties,
            "required": required
        }))
    }

    fn convert_graphql_type_to_json(&self, type_ref: &GraphQLTypeRef) -> (Value, bool) {
        match type_ref.kind.as_str() {
            "NON_NULL" => {
                if let Some(of_type) = &type_ref.of_type {
                    let (inner_type, _) = self.convert_graphql_type_to_json(of_type);
                    (inner_type, true) // NON_NULL means required
                } else {
                    (json!({"type": "string"}), true)
                }
            },
            "LIST" => {
                if let Some(of_type) = &type_ref.of_type {
                    let (item_type, _) = self.convert_graphql_type_to_json(of_type);
                    (json!({
                        "type": "array",
                        "items": item_type
                    }), false)
                } else {
                    (json!({
                        "type": "array",
                        "items": {"type": "string"}
                    }), false)
                }
            },
            "SCALAR" => {
                let json_type = match type_ref.name.as_deref() {
                    Some("String") => json!({"type": "string"}),
                    Some("Int") => json!({"type": "integer"}),
                    Some("Float") => json!({"type": "number"}),
                    Some("Boolean") => json!({"type": "boolean"}),
                    Some("jsonb") | Some("json") => json!({"type": "object"}),
                    Some("uuid") => json!({
                        "type": "string",
                        "format": "uuid"
                    }),
                    Some("timestamptz") | Some("timestamp") => json!({
                        "type": "string",
                        "format": "date-time"
                    }),
                    Some("date") => json!({
                        "type": "string",
                        "format": "date"
                    }),
                    Some("numeric") => json!({"type": "number"}),
                    _ => {
                        debug!("Unknown scalar type: {:?}, defaulting to string", type_ref.name);
                        json!({"type": "string"})
                    }
                };
                (json_type, false)
            },
            "INPUT_OBJECT" => {
                // For nested input objects, we could recursively resolve them
                // For now, treat as generic object
                (json!({"type": "object"}), false)
            },
            "ENUM" => {
                // For enums, we could extract the enum values
                // For now, treat as string with validation note
                (json!({
                    "type": "string",
                    "description": format!("Enum type: {}", type_ref.name.as_deref().unwrap_or("unknown"))
                }), false)
            },
            _ => {
                debug!("Unknown GraphQL type kind: {}, defaulting to string", type_ref.kind);
                (json!({"type": "string"}), false)
            }
        }
    }

    pub fn get_table_schema(&self, table_name: &str) -> Option<&Value> {
        self.table_schemas.get(table_name)
    }

    pub fn get_insert_input_schema(&self, table_name: &str) -> Option<&Value> {
        let insert_input_name = format!("{}_insert_input", table_name);
        self.schema_cache.get(&insert_input_name)
    }

    pub fn get_insert_tool(&self, table_name: &str) -> Option<crate::services::Tool> {
        let schema = self.get_insert_input_schema(table_name)?.clone();
        
        Some(crate::services::Tool {
            name: format!("insert_{}", table_name),
            description: format!("Insert a new {} record into the database", table_name),
            input_schema: schema,
        })
    }

    pub fn create_tool_for_table(&self, table_name: &str, tool_name: &str, description: &str) -> Option<crate::services::Tool> {
        let schema = self.get_table_schema(table_name)?.clone();
        
        Some(crate::services::Tool {
            name: tool_name.to_string(),
            description: description.to_string(),
            input_schema: schema,
        })
    }

    pub fn get_available_tables(&self) -> Vec<String> {
        self.table_schemas.keys().cloned().collect()
    }

    pub fn create_campaign_content_tool(&self) -> Option<crate::services::Tool> {
        // Create a comprehensive tool schema that includes multiple tables
        let mut properties = Map::new();
        
        // Add arrays for each table type we want to generate
        let table_configs = vec![
            ("npcs", "Array of NPCs to create"),
            ("locations", "Array of locations to create"),  
            ("quest_hooks", "Array of quest hooks to create"),
            ("magic_items", "Array of magic items to create"),
            ("shops", "Array of shops to create"),
            ("taverns", "Array of taverns to create"),
            ("temples", "Array of temples to create"),
            ("encounters", "Array of encounters to create"),
        ];

        for (table_name, description) in table_configs {
            if let Some(table_schema) = self.get_table_schema(table_name) {
                properties.insert(table_name.to_string(), json!({
                    "type": "array",
                    "description": description,
                    "items": table_schema
                }));
            }
        }

        if properties.is_empty() {
            warn!("No table schemas available for campaign content tool");
            return None;
        }

        Some(crate::services::Tool {
            name: "generate_campaign_content".to_string(),
            description: "Generate comprehensive D&D campaign content including NPCs, locations, quests, and items".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": properties,
                "required": ["npcs", "locations", "quest_hooks"]
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphql_type_conversion() {
        let generator = HasuraSchemaGenerator::new(
            crate::services::GraphQLClient::new("test".to_string())
        );

        // Test string type
        let string_type = GraphQLTypeRef {
            name: Some("String".to_string()),
            kind: "SCALAR".to_string(),
            of_type: None,
        };
        let (json_type, required) = generator.convert_graphql_type_to_json(&string_type);
        assert_eq!(json_type, json!({"type": "string"}));
        assert!(!required);

        // Test non-null string
        let non_null_string = GraphQLTypeRef {
            name: None,
            kind: "NON_NULL".to_string(),
            of_type: Some(Box::new(string_type)),
        };
        let (json_type, required) = generator.convert_graphql_type_to_json(&non_null_string);
        assert_eq!(json_type, json!({"type": "string"}));
        assert!(required);
    }
}