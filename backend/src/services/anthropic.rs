use crate::error::{ApiError, ApiResult};
use crate::models::{Campaign, GeneratedCampaignContent};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicResponse {
    pub content: Vec<Content>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
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
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to build HTTP client");

        Self { client, api_key }
    }

    pub async fn generate_campaign_content(&self, campaign: &Campaign) -> ApiResult<GeneratedCampaignContent> {
        info!("Generating content for campaign: {}", campaign.name);
        
        let prompt = self.build_campaign_prompt(campaign);
        let request = AnthropicRequest {
            model: MODEL.to_string(),
            max_tokens: MAX_TOKENS,
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt,
                }
            ],
            temperature: 0.8,
        };

        let response = self.send_request(request).await?;
        
        // Extract JSON from response
        let content = response.content.first()
            .ok_or_else(|| ApiError::BadRequest("Empty response from AI".to_string()))?;
        
        let json_content = self.extract_json(&content.text)?;
        
        // Parse JSON into our structure
        let generated_content: GeneratedCampaignContent = serde_json::from_str(&json_content)
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

    async fn send_request(&self, request: AnthropicRequest) -> ApiResult<AnthropicResponse> {
        debug!("Sending request to Anthropic API");
        
        let response = self.client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Network error calling Anthropic API: {}", e);
                ApiError::Internal(anyhow::anyhow!("Network error: {}", e))
            })?;

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
            
            match status {
                StatusCode::UNAUTHORIZED => {
                    error!("Invalid API key");
                    Err(ApiError::BadRequest("Invalid API key".to_string()))
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    error!("Rate limit exceeded");
                    Err(ApiError::BadRequest("Rate limit exceeded. Please try again later.".to_string()))
                }
                StatusCode::BAD_REQUEST => {
                    if let Ok(error_detail) = serde_json::from_str::<AnthropicError>(&error_text) {
                        error!("Anthropic API error: {}", error_detail.error.message);
                        Err(ApiError::BadRequest(error_detail.error.message))
                    } else {
                        Err(ApiError::BadRequest("Invalid request to AI service".to_string()))
                    }
                }
                _ => {
                    error!("Anthropic API error ({}): {}", status, error_text);
                    Err(ApiError::Internal(anyhow::anyhow!("AI service error: {}", status)))
                }
            }
        }
    }

    fn build_campaign_prompt(&self, campaign: &Campaign) -> String {
        let themes_str = campaign.themes.join(", ");
        let setting = campaign.setting.as_deref().unwrap_or("a fantasy world");
        
        format!(r#"You are a creative D&D 5e campaign designer. Generate detailed content for a campaign with the following parameters:

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

Respond with ONLY a JSON object in the following format:
{{
  "plot_summary": "A 2-3 sentence overview of the main campaign story",
  "central_conflict": "The main conflict driving the campaign",
  "npcs": [
    {{
      "name": "Character Name",
      "role": "Their role (e.g., Quest Giver, Villain, Ally, Merchant)",
      "description": "Physical appearance and general demeanor",
      "personality": {{
        "traits": ["trait1", "trait2", "trait3"],
        "motivation": "What drives this character",
        "fears": ["fear1", "fear2"],
        "connections": ["relationship to other NPCs or locations"]
      }},
      "stats": {{
        "race": "Race",
        "class": "Class if applicable",
        "level": 5,
        "abilities": {{
          "strength": 10,
          "dexterity": 10,
          "constitution": 10,
          "intelligence": 10,
          "wisdom": 10,
          "charisma": 10
        }}
      }},
      "secret_info": "Hidden information about this NPC (optional)"
    }}
  ],
  "locations": [
    {{
      "name": "Location Name",
      "type": "Type (e.g., Town, Dungeon, Wilderness)",
      "description": "Detailed description of the location",
      "properties": {{
        "atmosphere": "The mood and feel of the place",
        "notable_features": ["feature1", "feature2"],
        "hidden_elements": ["secret1", "secret2"],
        "danger_level": "Safe/Moderate/Dangerous/Deadly"
      }},
      "connections": ["Connected Location Name 1", "Connected Location Name 2"],
      "resident_npcs": ["NPC Name who lives/works here"]
    }}
  ],
  "quest_hooks": [
    {{
      "title": "Quest Title",
      "description": "Detailed quest description",
      "quest_giver": "NPC Name who gives this quest",
      "objectives": ["objective1", "objective2"],
      "reward": "What the party gains",
      "difficulty": "Easy/Medium/Hard/Deadly",
      "related_locations": ["Location Name 1", "Location Name 2"],
      "related_npcs": ["NPC Name 1", "NPC Name 2"]
    }}
  ]
}}

Make sure all NPCs, locations, and quests are interconnected. Characters should have relationships with each other, locations should be connected logically, and quests should involve multiple elements from the campaign."#,
            campaign.name, setting, themes_str
        )
    }

    fn extract_json(&self, text: &str) -> ApiResult<String> {
        // Try to find JSON between curly braces
        if let Some(start) = text.find('{') {
            if let Some(end) = text.rfind('}') {
                let json_str = &text[start..=end];
                return Ok(json_str.to_string());
            }
        }
        
        // If no JSON found, try the whole text
        if text.trim().starts_with('{') && text.trim().ends_with('}') {
            return Ok(text.trim().to_string());
        }
        
        Err(ApiError::BadRequest("No valid JSON found in AI response".to_string()))
    }
}