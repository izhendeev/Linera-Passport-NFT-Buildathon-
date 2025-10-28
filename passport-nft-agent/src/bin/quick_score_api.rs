use anyhow::{Context, Result};
use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use passport_nft_agent::{
    chain_client::{ChainClient, PassportInfo},
    config::AppConfig,
    scoring::{self, ObservationContext},
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{info, debug};
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct ScoreRequest {
    owner: String,
}

#[derive(Debug, Serialize)]
struct ScoreResponse {
    owner: String,
    score: u64,
    achievements: Vec<Achievement>,
    method: String,
    processing_time_ms: u128,
}

#[derive(Debug, Serialize)]
struct Achievement {
    code: String,
    explanation: String,
    points: Option<u64>,
}

async fn calculate_quick_score(
    Query(params): Query<ScoreRequest>,
) -> Result<Json<ScoreResponse>, StatusCode> {
    let start = Instant::now();

    let config = AppConfig::from_sources().map_err(|e| {
        tracing::error!("Config error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(owner = %params.owner, "Quick score request");

    // 1. Создать client
    let client = ChainClient::new(
        &config.graphql_endpoint,
        &config.indexer_endpoint,
    );

    // 2. Найти passport по owner
    let all_passports = client.all_passports().await.map_err(|e| {
        tracing::error!("Failed to fetch passports: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    debug!("Found {} passports total", all_passports.len());
    for (i, p) in all_passports.iter().enumerate() {
        debug!("Passport {}: owner={}, owner_chain={}", i, p.owner, p.owner_chain);
    }

    let passport_opt = all_passports.into_iter().find(|p| {
        let matches = p.owner.to_lowercase() == params.owner.to_lowercase();
        debug!("Comparing passport owner '{}' with request owner '{}': {}", p.owner, params.owner, matches);
        matches
    });

    let owner_parsed = params.owner.parse().map_err(|e| {
        tracing::error!("Invalid owner address: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    let owner_chain = if let Some(ref passport) = passport_opt {
        // Используем цепь из паспорта
        debug!("Using owner_chain from passport: {}", passport.owner_chain);
        passport.owner_chain.parse().map_err(|e| {
            tracing::error!("Invalid owner_chain from passport: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    } else {
        // Fallback: используем operation_chain_id из config
        tracing::warn!("No passport found for owner {}, using operation_chain_id", params.owner);
        config.operation_chain_id.parse().map_err(|e| {
            tracing::error!("Invalid chain ID: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    };

    // 3. Получить активность с цепи владельца
    let activity = client.owner_activity(&owner_parsed, &owner_chain)
        .await
        .unwrap_or_default();

    debug!("Retrieved {} activity events", activity.len());

    // 4. Создать контекст
    let fake_passport = passport_opt.unwrap_or_else(|| PassportInfo {
        token_id: passport_nft_agent::chain_client::PassportToken {
            id: serde_json::json!(vec![0u8; 16]),
        },
        owner: params.owner.clone(),
        owner_chain: owner_chain.to_string(),
        achievements: Some(vec![]),
        score: Some(0),
    });

    let context = ObservationContext::from_passport(fake_passport, activity);

    // 5. Rule-based скоринг
    let achievement_result = scoring::evaluate_rules(&config, &context)
        .await
        .map_err(|e| {
            tracing::error!("Scoring error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let processing_time = start.elapsed().as_millis();

    info!(
        owner = %params.owner,
        score = achievement_result.score,
        achievements = achievement_result.achievements.len(),
        processing_time_ms = processing_time,
        "Quick score calculated"
    );

    Ok(Json(ScoreResponse {
        owner: params.owner,
        score: achievement_result.score,
        achievements: achievement_result
            .achievements
            .into_iter()
            .map(|a| Achievement {
                code: a.code,
                explanation: a.explanation,
                points: a.points,
            })
            .collect(),
        method: "rule-based".to_string(),
        processing_time_ms: processing_time,
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();


    // Configure CORS to allow frontend requests
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/quick-score", get(calculate_quick_score))
        .layer(cors);

    let port = std::env::var("QUICK_SCORE_PORT").unwrap_or_else(|_| "8001".to_string());
    let addr = format!("127.0.0.1:{}", port);

    info!("Quick Score API listening on http://{}", addr);
    info!("Example: http://{}/quick-score?owner=0x...", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind")?;

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}
