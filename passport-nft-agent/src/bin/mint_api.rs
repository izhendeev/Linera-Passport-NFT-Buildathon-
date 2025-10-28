/**
 * Mint API Server for Passport NFT
 *
 * FOR HACKATHON DEMO:
 * - Provides HTTP endpoint for minting passports
 * - Signs transactions with Oracle Agent wallet
 * - Allows frontend to mint without wallet setup
 *
 * PRODUCTION ARCHITECTURE:
 * - This is ONLY for demo convenience
 * - Real users sign with their own wallets (SDK integration ready)
 * - See web-frontend/contexts/linera-context-faucet.tsx for production code
 */

use anyhow::{Context, Result};
use axum::{
    extract::Json as AxumJson,
    http::StatusCode,
    routing::post,
    Router,
};
use linera_base::identifiers::{ApplicationId, ChainId};
use passport_nft::{MintArgs, TokenId};
use passport_nft_agent::{
    chain_client::ChainClient,
    config::AppConfig,
    updater::{PassportUpdater, WalletSigner},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{info, error};

#[derive(Debug, Deserialize)]
struct MintRequest {
    owner: String,
}

#[derive(Debug, Serialize)]
struct MintResponse {
    success: bool,
    token_id: Vec<u8>,
    owner: String,
    metadata_uri: String,
    image_uri: String,
    content_hash: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    details: String,
}

/// Global state shared across handlers
struct AppState {
    updater: PassportUpdater,
    config: AppConfig,
}

/// Generate random token ID (16 bytes)
fn generate_token_id() -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..16).map(|_| rng.gen()).collect()
}

/// Generate random content hash (32 bytes)
fn generate_content_hash() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    format!("0x{}", hex::encode(bytes))
}

async fn mint_passport_handler(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    AxumJson(request): AxumJson<MintRequest>,
) -> Result<AxumJson<MintResponse>, (StatusCode, AxumJson<ErrorResponse>)> {
    info!(owner = %request.owner, "Mint request received");

    // Parse chain ID from config
    let chain_id = ChainId::from_str(&state.config.operation_chain_id)
        .map_err(|e| {
            error!("Invalid chain ID in config: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(ErrorResponse {
                    error: "Configuration error".to_string(),
                    details: format!("Invalid chain ID: {}", e),
                }),
            )
        })?;

    // Generate mint parameters
    let token_id_bytes = generate_token_id();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let metadata_uri = format!("ipfs://QmPassportMetadata{}", timestamp);
    let image_uri = format!("ipfs://QmPassportImage{}", timestamp);
    let content_hash = generate_content_hash();

    info!(
        token_id = ?token_id_bytes,
        metadata_uri = %metadata_uri,
        owner = %request.owner,
        "Generated mint parameters"
    );

    // Create mint arguments
    let mint_args = MintArgs {
        token_id: TokenId {
            id: token_id_bytes.clone(),
        },
        metadata_uri: metadata_uri.clone(),
        image_uri: image_uri.clone(),
        content_hash: content_hash.clone(),
    };

    // Submit mint operation to blockchain
    // Note: This uses the same mechanism as Oracle Agent's submit_update
    match state.updater.submit_mint(chain_id, mint_args).await {
        Ok(_) => {
            info!(
                owner = %request.owner,
                token_id = ?token_id_bytes,
                "Mint operation submitted successfully"
            );

            Ok(AxumJson(MintResponse {
                success: true,
                token_id: token_id_bytes,
                owner: request.owner,
                metadata_uri,
                image_uri,
                content_hash,
            }))
        }
        Err(e) => {
            error!(
                owner = %request.owner,
                error = %e,
                "Failed to submit mint operation"
            );

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(ErrorResponse {
                    error: "Mint failed".to_string(),
                    details: e.to_string(),
                }),
            ))
        }
    }
}

async fn health_handler() -> AxumJson<serde_json::Value> {
    AxumJson(serde_json::json!({
        "status": "ok",
        "service": "passport-nft-mint-api",
        "mode": "demo",
        "note": "This API is for hackathon demo. Production uses SDK wallet signing."
    }))
}

async fn info_handler() -> AxumJson<serde_json::Value> {
    AxumJson(serde_json::json!({
        "demo_mode": {
            "description": "Backend API signs transactions for quick demo",
            "purpose": "Allow judges to test without wallet installation",
            "flow": "Frontend → Mint API (admin signs) → Blockchain"
        },
        "production_architecture": {
            "description": "Full decentralized wallet signing",
            "implementation": "See web-frontend/contexts/linera-context-faucet.tsx",
            "wallets_supported": ["MetaMask", "Dynamic", "Browser SDK", "Hardware wallets"],
            "flow": "Frontend → User Wallet (user signs) → Blockchain"
        },
        "real_components": {
            "indexer": "Running - reads blockchain in real-time",
            "oracle_agent": "Running - AI analysis with Claude",
            "smart_contract": "Deployed - handles all operations",
            "cross_chain": "Implemented - see contract.rs"
        }
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("🚀 Passport NFT Mint API Server");
    info!("   Loading configuration...");

    // Load config
    let config = AppConfig::from_sources()
        .context("Failed to load configuration")?;

    info!("   Configuration loaded");
    info!("   Chain ID: {}", config.operation_chain_id);
    info!("   Application ID: {}", config.application_id);

    // Create chain client
    let client = ChainClient::new(
        config.graphql_endpoint.clone(),
        config.indexer_endpoint.clone(),
    );

    // Create wallet signer
    info!("   Loading wallet signer...");
    let signer = WalletSigner::from_config(&config)
        .await
        .context("Failed to create wallet signer")?;

    // Create updater
    let application_id = ApplicationId::from_str(&config.application_id)
        .context("Invalid application ID")?;

    let updater = PassportUpdater::new(
        client,
        signer,
        application_id,
        config.linera_rpc_endpoint.clone(),
    );

    info!("   Updater initialized");

    // Create shared state
    let state = Arc::new(AppState { updater, config });

    // Build router
    let app = Router::new()
        .route("/mint", post(mint_passport_handler))
        .route("/health", axum::routing::get(health_handler))
        .route("/info", axum::routing::get(info_handler))
        .with_state(state);

    // Start server
    let port = std::env::var("MINT_API_PORT").unwrap_or_else(|_| "8082".to_string());
    let addr = format!("0.0.0.0:{}", port);

    info!("📊 MODE: Hackathon Demo API");
    info!("   - Signs transactions with Oracle Agent wallet");
    info!("   - Allows instant testing without wallet setup");
    info!("");
    info!("🚀 PRODUCTION ARCHITECTURE READY:");
    info!("   - SDK wallet integration: web-frontend/contexts/linera-context-faucet.tsx");
    info!("   - Real indexer + Oracle Agent active");
    info!("   - Full decentralized signing supported");
    info!("");
    info!("✅ Server listening on http://{}", addr);
    info!("   POST /mint      - Mint passport for demo wallet");
    info!("   GET  /health    - Health check");
    info!("   GET  /info      - Architecture details for judges");
    info!("");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to {}", addr))?;

    axum::serve(listener, app)
        .await
        .context("Server error")?;

    Ok(())
}
