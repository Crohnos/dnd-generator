use crate::error::{ApiError, ApiResult};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{debug, error, info};

const HASURA_ENDPOINT: &str = "http://localhost:8080/v1/graphql";

#[derive(Debug, Clone)]
pub struct GraphQLClient {
    client: Client,
    admin_secret: String,
}

#[derive(Debug, Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<Value>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub path: Option<Vec<Value>>,
}

impl GraphQLClient {
    pub fn new(admin_secret: String) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to build HTTP client");

        Self { client, admin_secret }
    }

    pub async fn execute(&self, query: &str, variables: Option<Value>) -> ApiResult<Value> {
        let request = GraphQLRequest {
            query: query.to_string(),
            variables: variables.clone(),
        };

        debug!("Executing GraphQL query: {}", query);
        if let Some(vars) = &variables {
            debug!("With variables: {}", vars);
        }

        let response = self.client
            .post(HASURA_ENDPOINT)
            .header("x-hasura-admin-secret", &self.admin_secret)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Network error calling Hasura: {}", e);
                ApiError::Internal(anyhow::anyhow!("Network error: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("Hasura returned error status: {}", status);
            error!("Error response: {}", error_text);
            return Err(ApiError::Internal(anyhow::anyhow!("Hasura error ({}): {}", status, error_text)));
        }

        let graphql_response: GraphQLResponse = response.json().await
            .map_err(|e| {
                error!("Failed to parse Hasura response: {}", e);
                ApiError::Internal(anyhow::anyhow!("Failed to parse response: {}", e))
            })?;

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            error!("GraphQL errors: {:?}", error_messages);
            return Err(ApiError::BadRequest(format!("GraphQL errors: {}", error_messages.join(", "))));
        }

        graphql_response.data
            .ok_or_else(|| ApiError::BadRequest("No data in GraphQL response".to_string()))
    }

    // Helper methods for common operations
    pub async fn insert_one(&self, table: &str, object: Value) -> ApiResult<Value> {
        let mutation = format!(
            "mutation InsertOne($object: {}_insert_input!) {{ insert_{}_one(object: $object) {{ id }} }}",
            table, table
        );
        
        let variables = json!({ "object": object });
        let response = self.execute(&mutation, Some(variables)).await?;
        
        response.get(&format!("insert_{}_one", table))
            .ok_or_else(|| ApiError::BadRequest("No insert result in response".to_string()))
            .map(|v| v.clone())
    }

    pub async fn insert_many(&self, table: &str, objects: Vec<Value>) -> ApiResult<Value> {
        let mutation = format!(
            "mutation InsertMany($objects: [{}!]!) {{ insert_{}(objects: $objects) {{ returning {{ id }} }} }}",
            format!("{}_insert_input", table), table
        );
        
        let variables = json!({ "objects": objects });
        let response = self.execute(&mutation, Some(variables)).await?;
        
        response.get(&format!("insert_{}", table))
            .and_then(|v| v.get("returning"))
            .ok_or_else(|| ApiError::BadRequest("No insert result in response".to_string()))
            .map(|v| v.clone())
    }

    pub async fn update_by_pk(&self, table: &str, pk_columns: Value, set: Value) -> ApiResult<Value> {
        let mutation = format!(
            "mutation UpdateByPk($pk_columns: {}_pk_columns_input!, $set: {}_set_input!) {{ update_{}_by_pk(pk_columns: $pk_columns, _set: $set) {{ id }} }}",
            table, table, table
        );
        
        let variables = json!({ "pk_columns": pk_columns, "set": set });
        let response = self.execute(&mutation, Some(variables)).await?;
        
        response.get(&format!("update_{}_by_pk", table))
            .ok_or_else(|| ApiError::BadRequest("No update result in response".to_string()))
            .map(|v| v.clone())
    }

    // Campaign-specific helper methods
    pub async fn save_npc(&self, campaign_id: i32, npc_data: &Value) -> ApiResult<i32> {
        let object = json!({
            "campaign_id": campaign_id,
            "name": npc_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown NPC"),
            "role": npc_data.get("role").and_then(|v| v.as_str()).unwrap_or("Unknown"),
            "description": npc_data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
            "personality": npc_data.get("personality").unwrap_or(&json!({})),
            "stats": npc_data.get("stats").unwrap_or(&json!({})),
            "secret_info": npc_data.get("secret_info").and_then(|v| v.as_str())
        });

        let result = self.insert_one("npcs", object).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest("No ID in NPC insert result".to_string()))
    }

    pub async fn save_location(&self, campaign_id: i32, location_data: &Value) -> ApiResult<i32> {
        let object = json!({
            "campaign_id": campaign_id,
            "name": location_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Location"),
            "location_type": location_data.get("type").and_then(|v| v.as_str()).unwrap_or("Unknown"),
            "description": location_data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
            "properties": location_data.get("properties").unwrap_or(&json!({}))
        });

        let result = self.insert_one("locations", object).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest("No ID in location insert result".to_string()))
    }

    pub async fn save_quest_hook(&self, campaign_id: i32, quest_data: &Value) -> ApiResult<i32> {
        let object = json!({
            "campaign_id": campaign_id,
            "title": quest_data.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown Quest"),
            "description": quest_data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
            "difficulty": quest_data.get("difficulty").and_then(|v| v.as_str()).unwrap_or("medium"),
            "reward": quest_data.get("reward").and_then(|v| v.as_str()).unwrap_or(""),
            "related_npc_ids": quest_data.get("related_npcs")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default(),
            "related_location_ids": quest_data.get("related_locations")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default()
        });

        let result = self.insert_one("quest_hooks", object).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest("No ID in quest hook insert result".to_string()))
    }

    pub async fn save_magic_item(&self, campaign_id: i32, item_data: &Value) -> ApiResult<i32> {
        // Determine if this is a sentient item
        let is_sentient = item_data.get("personality").is_some() || 
                         item_data.get("communication_method").is_some() ||
                         item_data.get("intelligence").is_some() ||
                         item_data.get("goals").is_some();

        let object = json!({
            "campaign_id": campaign_id,
            "name": item_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Item"),
            "item_type": item_data.get("type").and_then(|v| v.as_str()).unwrap_or("wondrous item"),
            "rarity": item_data.get("rarity").and_then(|v| v.as_str()).unwrap_or("common"),
            "description": item_data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
            "mechanical_effects": item_data.get("mechanical_effects").and_then(|v| v.as_str()),
            "attunement_required": item_data.get("attunement_required").and_then(|v| v.as_bool()).unwrap_or(false),
            "is_sentient": is_sentient,
            "intelligence": if is_sentient { item_data.get("intelligence").and_then(|v| v.as_i64()).map(|v| v as i32) } else { None },
            "personality": if is_sentient { item_data.get("personality").and_then(|v| v.as_str()) } else { None }
        });

        let result = self.insert_one("magic_items", object).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest("No ID in magic item insert result".to_string()))
    }

    pub async fn save_shop(&self, location_id: i32, shop_data: &Value) -> ApiResult<i32> {
        let object = json!({
            "location_id": location_id,
            "name": shop_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Shop"),
            "shop_type": shop_data.get("type").and_then(|v| v.as_str()).unwrap_or("general"),
            "owner": shop_data.get("owner").and_then(|v| v.as_str()).unwrap_or(""),
            "description": shop_data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
            "specialties": shop_data.get("specialties")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default(),
            "notable_items": shop_data.get("notable_items")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default()
        });

        let result = self.insert_one("shops", object).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest("No ID in shop insert result".to_string()))
    }

    pub async fn save_tavern(&self, location_id: i32, tavern_data: &Value) -> ApiResult<i32> {
        let object = json!({
            "location_id": location_id,
            "name": tavern_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Tavern"),
            "owner": tavern_data.get("owner").and_then(|v| v.as_str()).unwrap_or(""),
            "description": tavern_data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
            "atmosphere": tavern_data.get("atmosphere").and_then(|v| v.as_str()).unwrap_or(""),
            "specialties": tavern_data.get("specialties")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default(),
            "regular_patrons": tavern_data.get("regular_patrons")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default()
        });

        let result = self.insert_one("taverns", object).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest("No ID in tavern insert result".to_string()))
    }

    pub async fn save_temple(&self, location_id: i32, temple_data: &Value) -> ApiResult<i32> {
        let object = json!({
            "location_id": location_id,
            "name": temple_data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Temple"),
            "deity": temple_data.get("deity").and_then(|v| v.as_str()).unwrap_or(""),
            "high_priest": temple_data.get("high_priest").and_then(|v| v.as_str()).unwrap_or(""),
            "description": temple_data.get("description").and_then(|v| v.as_str()).unwrap_or(""),
            "services": temple_data.get("services")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                .unwrap_or_default()
        });

        let result = self.insert_one("temples", object).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest("No ID in temple insert result".to_string()))
    }

    // Generic save method for any table with campaign_id
    pub async fn save_campaign_entity(&self, table: &str, campaign_id: i32, entity_data: &Value) -> ApiResult<i32> {
        let mut object = entity_data.as_object()
            .ok_or_else(|| ApiError::BadRequest("Entity data must be an object".to_string()))?
            .clone();
        
        // Add campaign_id to the object
        object.insert("campaign_id".to_string(), json!(campaign_id));
        
        let result = self.insert_one(table, json!(object)).await?;
        result.get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| ApiError::BadRequest(format!("No ID in {} insert result", table)))
    }

    // Batch save methods for phase-specific operations
    pub async fn save_phase_1a_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Calendar systems
        if let Some(calendar_systems) = phase_data.get("calendar_systems").and_then(|v| v.as_array()) {
            for calendar in calendar_systems {
                let mut calendar_obj = calendar.as_object().unwrap_or(&serde_json::Map::new()).clone();
                calendar_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("calendar_systems", json!(calendar_obj)).await?;
                saved_entities.push("calendar_systems".to_string());
            }
        }

        // Planes
        if let Some(planes) = phase_data.get("planes").and_then(|v| v.as_array()) {
            for plane in planes {
                let mut plane_obj = plane.as_object().unwrap_or(&serde_json::Map::new()).clone();
                plane_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("planes", json!(plane_obj)).await?;
                saved_entities.push("planes".to_string());
            }
        }

        // Geography regions
        if let Some(geography) = phase_data.get("geography_regions").and_then(|v| v.as_array()) {
            for region in geography {
                let mut region_obj = region.as_object().unwrap_or(&serde_json::Map::new()).clone();
                region_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("geography_regions", json!(region_obj)).await?;
                saved_entities.push("geography_regions".to_string());
            }
        }

        // Historical periods
        if let Some(history) = phase_data.get("historical_periods").and_then(|v| v.as_array()) {
            for period in history {
                let mut period_obj = period.as_object().unwrap_or(&serde_json::Map::new()).clone();
                period_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("historical_periods", json!(period_obj)).await?;
                saved_entities.push("historical_periods".to_string());
            }
        }

        // Economic systems
        if let Some(economics) = phase_data.get("economic_systems").and_then(|v| v.as_array()) {
            for system in economics {
                let mut system_obj = system.as_object().unwrap_or(&serde_json::Map::new()).clone();
                system_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("economic_systems", json!(system_obj)).await?;
                saved_entities.push("economic_systems".to_string());
            }
        }

        // Legal systems
        if let Some(legal) = phase_data.get("legal_systems").and_then(|v| v.as_array()) {
            for system in legal {
                let mut system_obj = system.as_object().unwrap_or(&serde_json::Map::new()).clone();
                system_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("legal_systems", json!(system_obj)).await?;
                saved_entities.push("legal_systems".to_string());
            }
        }

        // Celestial bodies
        if let Some(celestial) = phase_data.get("celestial_bodies").and_then(|v| v.as_array()) {
            for body in celestial {
                let mut body_obj = body.as_object().unwrap_or(&serde_json::Map::new()).clone();
                body_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("celestial_bodies", json!(body_obj)).await?;
                saved_entities.push("celestial_bodies".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_1b_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Races
        if let Some(races) = phase_data.get("races").and_then(|v| v.as_array()) {
            for race in races {
                let mut race_obj = race.as_object().unwrap_or(&serde_json::Map::new()).clone();
                race_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("races", json!(race_obj)).await?;
                saved_entities.push("races".to_string());
            }
        }

        // Character classes
        if let Some(classes) = phase_data.get("character_classes").and_then(|v| v.as_array()) {
            for class in classes {
                let mut class_obj = class.as_object().unwrap_or(&serde_json::Map::new()).clone();
                class_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("character_classes", json!(class_obj)).await?;
                saved_entities.push("character_classes".to_string());
            }
        }

        // Feats
        if let Some(feats) = phase_data.get("feats").and_then(|v| v.as_array()) {
            for feat in feats {
                let mut feat_obj = feat.as_object().unwrap_or(&serde_json::Map::new()).clone();
                feat_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("feats", json!(feat_obj)).await?;
                saved_entities.push("feats".to_string());
            }
        }

        // Backgrounds
        if let Some(backgrounds) = phase_data.get("backgrounds").and_then(|v| v.as_array()) {
            for background in backgrounds {
                let mut bg_obj = background.as_object().unwrap_or(&serde_json::Map::new()).clone();
                bg_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("backgrounds", json!(bg_obj)).await?;
                saved_entities.push("backgrounds".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_1c_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Languages
        if let Some(languages) = phase_data.get("languages").and_then(|v| v.as_array()) {
            for language in languages {
                let mut lang_obj = language.as_object().unwrap_or(&serde_json::Map::new()).clone();
                lang_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("languages", json!(lang_obj)).await?;
                saved_entities.push("languages".to_string());
            }
        }

        // Cultures
        if let Some(cultures) = phase_data.get("cultures").and_then(|v| v.as_array()) {
            for culture in cultures {
                let mut culture_obj = culture.as_object().unwrap_or(&serde_json::Map::new()).clone();
                culture_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("cultures", json!(culture_obj)).await?;
                saved_entities.push("cultures".to_string());
            }
        }

        // Factions
        if let Some(factions) = phase_data.get("factions").and_then(|v| v.as_array()) {
            for faction in factions {
                let mut faction_obj = faction.as_object().unwrap_or(&serde_json::Map::new()).clone();
                faction_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("factions", json!(faction_obj)).await?;
                saved_entities.push("factions".to_string());
            }
        }

        // Pantheons
        if let Some(pantheons) = phase_data.get("pantheons").and_then(|v| v.as_array()) {
            for pantheon in pantheons {
                let mut pantheon_obj = pantheon.as_object().unwrap_or(&serde_json::Map::new()).clone();
                pantheon_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("pantheons", json!(pantheon_obj)).await?;
                saved_entities.push("pantheons".to_string());
            }
        }

        // Deities
        if let Some(deities) = phase_data.get("deities").and_then(|v| v.as_array()) {
            for deity in deities {
                let mut deity_obj = deity.as_object().unwrap_or(&serde_json::Map::new()).clone();
                deity_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("deities", json!(deity_obj)).await?;
                saved_entities.push("deities".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_2a_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Entities (PC-connected NPCs)
        if let Some(entities) = phase_data.get("entities").and_then(|v| v.as_array()) {
            for entity in entities {
                let mut entity_obj = entity.as_object().unwrap_or(&serde_json::Map::new()).clone();
                entity_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("entities", json!(entity_obj)).await?;
                saved_entities.push("entities".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_2b_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Hierarchical locations (cities, districts, buildings)
        if let Some(locations) = phase_data.get("locations").and_then(|v| v.as_array()) {
            for location in locations {
                let mut loc_obj = location.as_object().unwrap_or(&serde_json::Map::new()).clone();
                loc_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("locations", json!(loc_obj)).await?;
                saved_entities.push("locations".to_string());
            }
        }

        // Buildings
        if let Some(buildings) = phase_data.get("buildings").and_then(|v| v.as_array()) {
            for building in buildings {
                let mut building_obj = building.as_object().unwrap_or(&serde_json::Map::new()).clone();
                building_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("buildings", json!(building_obj)).await?;
                saved_entities.push("buildings".to_string());
            }
        }

        // Dungeons
        if let Some(dungeons) = phase_data.get("dungeons").and_then(|v| v.as_array()) {
            for dungeon in dungeons {
                let mut dungeon_obj = dungeon.as_object().unwrap_or(&serde_json::Map::new()).clone();
                dungeon_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("dungeons", json!(dungeon_obj)).await?;
                saved_entities.push("dungeons".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_2c_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Items
        if let Some(items) = phase_data.get("items").and_then(|v| v.as_array()) {
            for item in items {
                let mut item_obj = item.as_object().unwrap_or(&serde_json::Map::new()).clone();
                item_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("items", json!(item_obj)).await?;
                saved_entities.push("items".to_string());
            }
        }

        // Item effects
        if let Some(effects) = phase_data.get("item_effects").and_then(|v| v.as_array()) {
            for effect in effects {
                let mut effect_obj = effect.as_object().unwrap_or(&serde_json::Map::new()).clone();
                effect_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("item_effects", json!(effect_obj)).await?;
                saved_entities.push("item_effects".to_string());
            }
        }

        // Sentient item properties
        if let Some(properties) = phase_data.get("sentient_item_properties").and_then(|v| v.as_array()) {
            for property in properties {
                let mut prop_obj = property.as_object().unwrap_or(&serde_json::Map::new()).clone();
                prop_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("sentient_item_properties", json!(prop_obj)).await?;
                saved_entities.push("sentient_item_properties".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_3a_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Quest hooks
        if let Some(quest_hooks) = phase_data.get("quest_hooks").and_then(|v| v.as_array()) {
            for quest in quest_hooks {
                let mut quest_obj = quest.as_object().unwrap_or(&serde_json::Map::new()).clone();
                quest_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("quest_hooks", json!(quest_obj)).await?;
                saved_entities.push("quest_hooks".to_string());
            }
        }

        // Encounters
        if let Some(encounters) = phase_data.get("encounters").and_then(|v| v.as_array()) {
            for encounter in encounters {
                let mut enc_obj = encounter.as_object().unwrap_or(&serde_json::Map::new()).clone();
                enc_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("encounters", json!(enc_obj)).await?;
                saved_entities.push("encounters".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_3b_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Additional NPCs
        if let Some(npcs) = phase_data.get("npcs").and_then(|v| v.as_array()) {
            for npc in npcs {
                let mut npc_obj = npc.as_object().unwrap_or(&serde_json::Map::new()).clone();
                npc_obj.insert("campaign_id".to_string(), json!(campaign_id));
                self.insert_one("npcs", json!(npc_obj)).await?;
                saved_entities.push("npcs".to_string());
            }
        }

        // Shops
        if let Some(shops) = phase_data.get("shops").and_then(|v| v.as_array()) {
            for shop in shops {
                let shop_obj = shop.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("shops", json!(shop_obj)).await?;
                saved_entities.push("shops".to_string());
            }
        }

        // Taverns
        if let Some(taverns) = phase_data.get("taverns").and_then(|v| v.as_array()) {
            for tavern in taverns {
                let tavern_obj = tavern.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("taverns", json!(tavern_obj)).await?;
                saved_entities.push("taverns".to_string());
            }
        }

        // Temples
        if let Some(temples) = phase_data.get("temples").and_then(|v| v.as_array()) {
            for temple in temples {
                let temple_obj = temple.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("temples", json!(temple_obj)).await?;
                saved_entities.push("temples".to_string());
            }
        }

        Ok(saved_entities)
    }

    pub async fn save_phase_3c_data(&self, campaign_id: i32, phase_data: &Value) -> ApiResult<Vec<String>> {
        let mut saved_entities = Vec::new();

        // Entity relationships
        if let Some(entity_rels) = phase_data.get("entity_relationships").and_then(|v| v.as_array()) {
            for rel in entity_rels {
                let rel_obj = rel.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("entity_relationships", json!(rel_obj)).await?;
                saved_entities.push("entity_relationships".to_string());
            }
        }

        // Entity factions
        if let Some(entity_factions) = phase_data.get("entity_factions").and_then(|v| v.as_array()) {
            for ef in entity_factions {
                let ef_obj = ef.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("entity_factions", json!(ef_obj)).await?;
                saved_entities.push("entity_factions".to_string());
            }
        }

        // Entity locations
        if let Some(entity_locs) = phase_data.get("entity_locations").and_then(|v| v.as_array()) {
            for el in entity_locs {
                let el_obj = el.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("entity_locations", json!(el_obj)).await?;
                saved_entities.push("entity_locations".to_string());
            }
        }

        // Entity items
        if let Some(entity_items) = phase_data.get("entity_items").and_then(|v| v.as_array()) {
            for ei in entity_items {
                let ei_obj = ei.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("entity_items", json!(ei_obj)).await?;
                saved_entities.push("entity_items".to_string());
            }
        }

        // Location items
        if let Some(location_items) = phase_data.get("location_items").and_then(|v| v.as_array()) {
            for li in location_items {
                let li_obj = li.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("location_items", json!(li_obj)).await?;
                saved_entities.push("location_items".to_string());
            }
        }

        // Quest entities
        if let Some(quest_entities) = phase_data.get("quest_entities").and_then(|v| v.as_array()) {
            for qe in quest_entities {
                let qe_obj = qe.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("quest_entities", json!(qe_obj)).await?;
                saved_entities.push("quest_entities".to_string());
            }
        }

        // Quest locations
        if let Some(quest_locs) = phase_data.get("quest_locations").and_then(|v| v.as_array()) {
            for ql in quest_locs {
                let ql_obj = ql.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("quest_locations", json!(ql_obj)).await?;
                saved_entities.push("quest_locations".to_string());
            }
        }

        // Faction relationships
        if let Some(faction_rels) = phase_data.get("faction_relationships").and_then(|v| v.as_array()) {
            for fr in faction_rels {
                let fr_obj = fr.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("faction_relationships", json!(fr_obj)).await?;
                saved_entities.push("faction_relationships".to_string());
            }
        }

        // Race cultures
        if let Some(race_cultures) = phase_data.get("race_cultures").and_then(|v| v.as_array()) {
            for rc in race_cultures {
                let rc_obj = rc.as_object().unwrap_or(&serde_json::Map::new()).clone();
                self.insert_one("race_cultures", json!(rc_obj)).await?;
                saved_entities.push("race_cultures".to_string());
            }
        }

        Ok(saved_entities)
    }
}