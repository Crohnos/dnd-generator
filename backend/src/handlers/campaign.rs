use crate::error::ApiResult;
use crate::models::{Campaign, CampaignDetail, CreateCampaignRequest, UpdateCampaignRequest};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn create_campaign(
    State(state): State<AppState>,
    Json(req): Json<CreateCampaignRequest>,
) -> ApiResult<Json<Campaign>> {
    let campaign = state.campaign_service.create_campaign(req).await?;
    Ok(Json(campaign))
}

pub async fn get_campaign(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Campaign>> {
    let campaign = state.campaign_service.get_campaign(id).await?;
    Ok(Json(campaign))
}

pub async fn list_campaigns(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<Campaign>>> {
    let campaigns = state.campaign_service.list_campaigns().await?;
    Ok(Json(campaigns))
}

pub async fn update_campaign(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCampaignRequest>,
) -> ApiResult<Json<Campaign>> {
    let campaign = state.campaign_service.update_campaign(id, req).await?;
    Ok(Json(campaign))
}

pub async fn delete_campaign(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<()> {
    state.campaign_service.delete_campaign(id).await?;
    Ok(())
}

pub async fn get_campaign_detail(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<CampaignDetail>> {
    let detail = state.campaign_service.get_campaign_detail(id).await?;
    Ok(Json(detail))
}

pub async fn generate_campaign_content(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    // Spawn generation task in background
    let generation_service = state.generation_service.clone();
    tokio::spawn(async move {
        if let Err(e) = generation_service.generate_campaign_content(id).await {
            tracing::error!("Failed to generate content for campaign {}: {}", id, e);
        }
    });
    
    Ok(Json(serde_json::json!({
        "campaign_id": id,
        "status": "generating",
        "message": "Content generation started. Check campaign status for updates."
    })))
}

pub async fn generate_encounters(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    // Placeholder for encounter generation - will be implemented in Phase 4
    Ok(Json(serde_json::json!({
        "campaign_id": id,
        "status": "pending",
        "message": "Encounter generation will be implemented in Phase 4"
    })))
}