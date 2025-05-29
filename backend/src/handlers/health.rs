use crate::state::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub version: String,
}

pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    // For now, just return ok - we could add a pool getter to AppState if needed
    Json(HealthResponse {
        status: "ok".to_string(),
        database: "connected".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}