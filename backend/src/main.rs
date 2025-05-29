use axum::{
    routing::{get, post},
    Router,
};
use dnd_campaign_generator::{
    config::Config,
    db::create_pool,
    handlers::{
        create_campaign, delete_campaign, get_campaign_detail, 
        generate_campaign_content, generate_encounters, health_check,
        list_campaigns, update_campaign,
    },
    state::AppState,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dnd_campaign_generator=debug,tower_http=debug,sqlx=info".into()),
        )
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database pool
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Create app state
    let state = AppState::new(pool, &config);

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/campaigns", get(list_campaigns).post(create_campaign))
        .route(
            "/api/campaigns/:id",
            get(get_campaign_detail)
                .put(update_campaign)
                .delete(delete_campaign),
        )
        .route("/api/campaigns/:id/generate", post(generate_campaign_content))
        .route("/api/campaigns/:id/encounters", post(generate_encounters))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("D&D Campaign Generator Backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}