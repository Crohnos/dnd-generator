use dnd_campaign_generator::models::{CreateCampaignRequest, UpdateCampaignRequest};
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn test_health_endpoint() {
    let response = reqwest::get("http://localhost:3001/health").await;
    
    // If server is not running, skip test
    if response.is_err() {
        println!("Server not running, skipping test");
        return;
    }
    
    let response = response.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "OK");
}

#[tokio::test]
async fn test_campaign_crud() {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:3001/api/campaigns";
    
    // Check if server is running
    if client.get("http://localhost:3001/health").send().await.is_err() {
        println!("Server not running, skipping test");
        return;
    }
    
    // Create a campaign
    let create_req = CreateCampaignRequest {
        name: "Test Campaign".to_string(),
        setting: Some("Test Setting".to_string()),
        themes: vec!["Adventure".to_string(), "Mystery".to_string()],
        player_characters: Some(json!([])),
        metadata: Some(json!({"test": true})),
    };
    
    let response = client
        .post(base_url)
        .json(&create_req)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    let campaign: serde_json::Value = response.json().await.unwrap();
    let campaign_id = campaign["id"].as_i64().unwrap();
    
    // Get the campaign
    let response = client
        .get(&format!("{}/{}", base_url, campaign_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Update the campaign
    let update_req = UpdateCampaignRequest {
        name: Some("Updated Campaign".to_string()),
        setting: None,
        themes: None,
        player_characters: None,
        status: Some("ready".to_string()),
        metadata: None,
    };
    
    let response = client
        .put(&format!("{}/{}", base_url, campaign_id))
        .json(&update_req)
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // List campaigns
    let response = client.get(base_url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    // Delete the campaign
    let response = client
        .delete(&format!("{}/{}", base_url, campaign_id))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}