use crate::error::{ApiError, ApiResult};
use crate::models::{Campaign, GeneratedCampaignContent};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use std::error::Error;
use tracing::{debug, error, info};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-20250514";
const MAX_TOKENS: u32 = 20000;

#[derive(Debug, Clone)]
pub struct AnthropicClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Serialize)]
pub struct ToolChoice {
    #[serde(rename = "type")]
    pub choice_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicResponse {
    pub content: Vec<Content>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse { 
        id: String,
        name: String,
        input: Value,
    },
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicError {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to build HTTP client");

        Self { client, api_key }
    }

    pub async fn generate_campaign_content(&self, campaign: &Campaign) -> ApiResult<GeneratedCampaignContent> {
        info!("Generating content for campaign: {}", campaign.name);
        
        let (prompt, tool) = self.build_campaign_tool(campaign);
        let response = self.generate_with_tool(&prompt, tool, MAX_TOKENS, 0.8).await?;
        
        // Parse JSON into our structure
        let generated_content: GeneratedCampaignContent = serde_json::from_value(response)
            .map_err(|e| {
                error!("Failed to parse AI response: {}", e);
                ApiError::BadRequest(format!("Invalid AI response format: {}", e))
            })?;

        info!("Successfully generated content with {} NPCs, {} locations, {} quest hooks",
            generated_content.npcs.len(),
            generated_content.locations.len(),
            generated_content.quest_hooks.len()
        );

        Ok(generated_content)
    }
    
    pub async fn generate_content(&self, prompt: &str, max_tokens: u32, temperature: f32) -> ApiResult<String> {
        info!("Generating content with custom parameters");
        
        let request = AnthropicRequest {
            model: MODEL.to_string(),
            max_tokens,
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            temperature,
            tools: None,
            tool_choice: None,
        };

        let response = self.send_request(request).await?;
        
        // Extract text content from response
        let content = response.content.first()
            .ok_or_else(|| ApiError::BadRequest("Empty response from AI".to_string()))?;
        
        match content {
            Content::Text { text } => Ok(text.clone()),
            Content::ToolUse { .. } => Err(ApiError::BadRequest("Unexpected tool use response".to_string())),
        }
    }
    
    pub async fn generate_with_tool(&self, prompt: &str, tool: Tool, max_tokens: u32, temperature: f32) -> ApiResult<Value> {
        info!("Generating content with tool: {}", tool.name);
        
        let request = AnthropicRequest {
            model: MODEL.to_string(),
            max_tokens,
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            temperature,
            tools: Some(vec![tool.clone()]),
            tool_choice: Some(ToolChoice {
                choice_type: "tool".to_string(),
                name: Some(tool.name.clone()),
            }),
        };

        let response = self.send_request(request).await?;
        
        // Find the tool use response
        for content in &response.content {
            match content {
                Content::ToolUse { name, input, .. } if name == &tool.name => {
                    return Ok(input.clone());
                }
                _ => continue,
            }
        }
        
        Err(ApiError::BadRequest("No tool use found in response".to_string()))
    }

    async fn send_request(&self, request: AnthropicRequest) -> ApiResult<AnthropicResponse> {
        info!("Sending request to Anthropic API");
        info!("API Key present: {}", !self.api_key.is_empty());
        info!("API Key first 10 chars: {}...", &self.api_key.chars().take(10).collect::<String>());
        debug!("Request model: {}", request.model);
        
        let response_result = self.client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await;
            
        let response = match response_result {
            Ok(resp) => resp,
            Err(e) => {
                error!("Network error calling Anthropic API: {}", e);
                error!("Error details: {:?}", e);
                error!("Error source: {:?}", e.source());
                if e.is_request() {
                    error!("Request construction error");
                } else if e.is_connect() {
                    error!("Connection error");
                } else if e.is_body() {
                    error!("Body error");
                } else if e.is_decode() {
                    error!("Decode error");
                }
                return Err(ApiError::Internal(anyhow::anyhow!("Network error: {}", e)));
            }
        };

        let status = response.status();
        
        if status.is_success() {
            let api_response = response.json::<AnthropicResponse>().await
                .map_err(|e| {
                    error!("Failed to parse Anthropic response: {}", e);
                    ApiError::Internal(anyhow::anyhow!("Failed to parse response: {}", e))
                })?;
            
            if let Some(usage) = &api_response.usage {
                debug!("Token usage - Input: {}, Output: {}", usage.input_tokens, usage.output_tokens);
            }
            
            Ok(api_response)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            error!("Anthropic API returned error status: {}", status);
            error!("Full error response: {}", error_text);
            
            match status {
                StatusCode::UNAUTHORIZED => {
                    error!("Invalid API key - check ANTHROPIC_API_KEY environment variable");
                    Err(ApiError::BadRequest("Invalid API key".to_string()))
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    error!("Rate limit exceeded");
                    if let Ok(error_detail) = serde_json::from_str::<AnthropicError>(&error_text) {
                        error!("Rate limit details: {}", error_detail.error.message);
                    }
                    Err(ApiError::BadRequest("Rate limit exceeded. Please try again later.".to_string()))
                }
                StatusCode::BAD_REQUEST => {
                    if let Ok(error_detail) = serde_json::from_str::<AnthropicError>(&error_text) {
                        error!("Anthropic API error: {}", error_detail.error.message);
                        error!("Error type: {:?}", error_detail.error.error_type);
                        Err(ApiError::BadRequest(error_detail.error.message))
                    } else {
                        error!("Could not parse error response: {}", error_text);
                        Err(ApiError::BadRequest(format!("Invalid request: {}", error_text)))
                    }
                }
                _ => {
                    error!("Unexpected status code from Anthropic: {}", status);
                    error!("Response body: {}", error_text);
                    Err(ApiError::Internal(anyhow::anyhow!("AI service error ({}): {}", status, error_text)))
                }
            }
        }
    }

    fn build_campaign_tool(&self, campaign: &Campaign) -> (String, Tool) {
        let themes_str = campaign.themes.join(", ");
        let setting = campaign.setting.as_deref().unwrap_or("a fantasy world");
        
        let prompt = format!(r#"You are a creative D&D 5e campaign designer. Generate detailed content for a campaign with the following parameters:

Campaign Name: {}
Setting: {}
Themes: {}

Generate a complete campaign with interconnected NPCs, locations, and quest hooks. The content should be rich, detailed, and suitable for a D&D 5e campaign.

Requirements:
- 8-12 unique NPCs with diverse roles (allies, villains, quest givers, merchants, etc.)
- 6-10 interesting locations that are interconnected
- 5-8 quest hooks that involve multiple NPCs and locations
- Each NPC should have personality traits, motivations, and some should have secrets
- Locations should have atmosphere and notable features
- Quest hooks should vary in difficulty and type

Make sure all NPCs, locations, and quests are interconnected. Characters should have relationships with each other, locations should be connected logically, and quests should involve multiple elements from the campaign."#,
            campaign.name, setting, themes_str
        );

        let tool = Tool {
            name: "generate_campaign_content".to_string(),
            description: "Generate complete D&D campaign content with interconnected NPCs, locations, and quest hooks".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "plot_summary": {
                        "type": "string",
                        "description": "A 2-3 sentence overview of the main campaign story"
                    },
                    "central_conflict": {
                        "type": "string",
                        "description": "The main conflict driving the campaign"
                    },
                    "npcs": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {"type": "string"},
                                "role": {"type": "string", "description": "e.g., Quest Giver, Villain, Ally, Merchant"},
                                "description": {"type": "string", "description": "Physical appearance and general demeanor"},
                                "personality": {
                                    "type": "object",
                                    "properties": {
                                        "traits": {"type": "array", "items": {"type": "string"}},
                                        "motivation": {"type": "string"},
                                        "fears": {"type": "array", "items": {"type": "string"}},
                                        "connections": {"type": "array", "items": {"type": "string"}}
                                    },
                                    "required": ["traits", "motivation", "fears", "connections"]
                                },
                                "stats": {
                                    "type": "object",
                                    "properties": {
                                        "race": {"type": "string"},
                                        "class": {"type": "string"},
                                        "level": {"type": "integer"},
                                        "abilities": {
                                            "type": "object",
                                            "properties": {
                                                "strength": {"type": "integer"},
                                                "dexterity": {"type": "integer"},
                                                "constitution": {"type": "integer"},
                                                "intelligence": {"type": "integer"},
                                                "wisdom": {"type": "integer"},
                                                "charisma": {"type": "integer"}
                                            },
                                            "required": ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"]
                                        }
                                    },
                                    "required": ["race", "class", "level", "abilities"]
                                },
                                "secret_info": {"type": "string"}
                            },
                            "required": ["name", "role", "description", "personality", "stats"]
                        }
                    },
                    "locations": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {"type": "string"},
                                "type": {"type": "string", "description": "e.g., Town, Dungeon, Wilderness"},
                                "description": {"type": "string"},
                                "properties": {
                                    "type": "object",
                                    "properties": {
                                        "atmosphere": {"type": "string"},
                                        "notable_features": {"type": "array", "items": {"type": "string"}},
                                        "hidden_elements": {"type": "array", "items": {"type": "string"}},
                                        "danger_level": {"type": "string", "enum": ["Safe", "Moderate", "Dangerous", "Deadly"]}
                                    },
                                    "required": ["atmosphere", "notable_features", "hidden_elements", "danger_level"]
                                },
                                "connections": {"type": "array", "items": {"type": "string"}},
                                "resident_npcs": {"type": "array", "items": {"type": "string"}}
                            },
                            "required": ["name", "type", "description", "properties", "connections", "resident_npcs"]
                        }
                    },
                    "quest_hooks": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "title": {"type": "string"},
                                "description": {"type": "string"},
                                "quest_giver": {"type": "string"},
                                "objectives": {"type": "array", "items": {"type": "string"}},
                                "reward": {"type": "string"},
                                "difficulty": {"type": "string", "enum": ["Easy", "Medium", "Hard", "Deadly"]},
                                "related_locations": {"type": "array", "items": {"type": "string"}},
                                "related_npcs": {"type": "array", "items": {"type": "string"}}
                            },
                            "required": ["title", "description", "quest_giver", "objectives", "reward", "difficulty", "related_locations", "related_npcs"]
                        }
                    }
                },
                "required": ["plot_summary", "central_conflict", "npcs", "locations", "quest_hooks"]
            })
        };

        (prompt, tool)
    }

}