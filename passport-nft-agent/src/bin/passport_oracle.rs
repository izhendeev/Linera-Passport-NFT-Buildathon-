use anyhow::Result;
use clap::Parser;
use linera_base::identifiers::ApplicationId;
use passport_nft::{TokenId, UpdateArgs};
use passport_nft_agent::chain_client::ChainClient;
use passport_nft_agent::config::AppConfig;
use passport_nft_agent::scoring::{self, ObservationContext};
use passport_nft_agent::updater::{PassportUpdater, WalletSigner};
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser, Debug)]
struct PassportOracleCli {
    #[arg(long, default_value = "info")]
    log_level: String,
    #[arg(long, help = "Dry run - don't submit to blockchain")]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = PassportOracleCli::parse();
    passport_nft_agent::init_logging(&cli.log_level)?;

    let config = AppConfig::from_sources()?;
    tracing::info!(?config, "Passport oracle configuration loaded");

    let client = ChainClient::new(
        config.graphql_endpoint.clone(),
        config.indexer_endpoint.clone(),
    );

    // Setup updater if not dry run
    let updater = if !cli.dry_run {
        let signer = WalletSigner::from_config(&config).await?;
        let application_id = ApplicationId::from_str(&config.application_id)?;
        Some(PassportUpdater::new(client.clone(), signer, application_id, config.linera_rpc_endpoint.clone()))
    } else {
        None
    };

    let passports = client.all_passports().await?;
    tracing::info!(count = passports.len(), "Fetched passports");

    for passport in passports {
        let owner = match passport.owner_account() {
            Ok(owner) => owner,
            Err(err) => {
                tracing::warn!(error = %err, "Failed to parse owner");
                continue;
            }
        };
        let owner_chain = match passport.owner_chain_id() {
            Ok(chain) => chain,
            Err(err) => {
                tracing::warn!(error = %err, "Failed to parse owner chain id");
                continue;
            }
        };

        let token_id_bytes = match passport.token_id_bytes() {
            Ok(Some(bytes)) => bytes,
            Ok(None) => {
                tracing::warn!("Token ID is empty");
                continue;
            }
            Err(err) => {
                tracing::warn!(error = %err, "Failed to parse token ID");
                continue;
            }
        };

        // CROSS-CHAIN FEATURE: Query activity from multiple chains if configured
        let chains_to_query: Vec<linera_base::identifiers::ChainId> = if config.cross_chain_ids.is_empty() {
            // Default: only query owner_chain
            vec![owner_chain]
        } else {
            // Parse configured chains and include owner_chain
            let mut chains = vec![owner_chain];
            for chain_str in &config.cross_chain_ids {
                match std::str::FromStr::from_str(chain_str) {
                    Ok(chain_id) => chains.push(chain_id),
                    Err(err) => tracing::warn!(
                        chain = %chain_str,
                        error = ?err,
                        "Failed to parse cross-chain ID, skipping"
                    ),
                }
            }
            chains
        };

        let activity = match client.owner_activity_cross_chain(&owner, &chains_to_query).await {
            Ok(events) => events,
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "Failed to fetch cross-chain activity from indexer, using empty activity list"
                );
                Vec::new()
            }
        };

        // Get existing achievements from passport
        let existing_achievements: HashSet<String> = passport
            .achievements
            .as_ref()
            .map(|achs| achs.iter().cloned().collect())
            .unwrap_or_default();
        
        let existing_score = passport.score.unwrap_or(0);

        let context = ObservationContext::from_passport(passport, activity);
        match scoring::evaluate_rules_with_llm(&config, &context).await {
            Ok(result) => {
                // Calculate only NEW achievements
                let computed_achievements: Vec<String> = result
                    .achievements
                    .iter()
                    .map(|a| format!("{}: {}", a.code, a.explanation))
                    .collect();
                
                let new_achievements: Vec<String> = computed_achievements
                    .into_iter()
                    .filter(|ach| !existing_achievements.contains(ach))
                    .collect();
                
                // Calculate score delta
                let score_delta = if result.score > existing_score {
                    result.score - existing_score
                } else {
                    0
                };

                tracing::info!(
                    passport_id = %context.passport_id,
                    total_score = result.score,
                    existing_score = existing_score,
                    score_delta = score_delta,
                    new_achievement_count = new_achievements.len(),
                    "Passport evaluated"
                );

                // Only submit if there are updates
                if new_achievements.is_empty() && score_delta == 0 {
                    tracing::info!(
                        passport_id = %context.passport_id,
                        "No updates needed - passport is up to date"
                    );
                    continue;
                }

                // Submit update to blockchain
                if let Some(ref updater) = updater {
                    let update_args = UpdateArgs {
                        token_id: TokenId {
                            id: token_id_bytes.clone(),
                        },
                        new_achievements,
                        score_increase: score_delta,
                    };

                    let wallet_path = PathBuf::from(&config.wallet_path);
                    match updater
                        .submit_update(owner_chain, update_args, &wallet_path)
                        .await
                    {
                        Ok(_) => tracing::info!(
                            passport_id = %context.passport_id,
                            "Update submitted to blockchain"
                        ),
                        Err(err) => tracing::error!(
                            passport_id = %context.passport_id,
                            error = %err,
                            "Failed to submit update"
                        ),
                    }
                } else {
                    tracing::info!("Dry run mode - skipping blockchain submission");
                }
            }
            Err(err) => tracing::warn!(
                passport_id = %context.passport_id,
                error = %err,
                "Failed to evaluate passport"
            ),
        }
    }

    Ok(())
}
