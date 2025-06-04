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
}