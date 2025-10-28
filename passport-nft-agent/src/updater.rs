use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use hex::FromHex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use bcs;
use hex;
use linera_base::{
    crypto::{AccountSecretKey, AccountSignature, CryptoHash},
    identifiers::{AccountOwner, ApplicationId, ChainId},
};
use linera_client::wallet::Wallet;
use linera_persistent::{File as PersistentFile, Persist};
use passport_nft::{MintArgs, PassportOperation, UpdateArgs};

use crate::chain_client::ChainClient;

/// Abstraction over the signing mechanism so we can plug in linera-client or mocks later on.
#[async_trait]
pub trait PayloadSigner: Send + Sync {
    async fn sign(
        &self,
        request_id: Uuid,
        chain_id: ChainId,
        hash: CryptoHash,
    ) -> Result<AccountSignature>;
}

pub struct PassportUpdater {
    client: ChainClient,
    signer: Arc<dyn PayloadSigner>,
    application_id: ApplicationId,
    linera_rpc_endpoint: String,
    http: Client,
}

impl PassportUpdater {
    pub fn new(
        client: ChainClient,
        signer: Arc<dyn PayloadSigner>,
        application_id: ApplicationId,
        linera_rpc_endpoint: String,
    ) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");
        Self {
            client,
            signer,
            application_id,
            linera_rpc_endpoint,
            http,
        }
    }

    pub async fn submit_update(
        &self,
        chain_id: ChainId,
        args: UpdateArgs,
        _wallet_path: &PathBuf,
    ) -> Result<()> {
        let request_id = Uuid::new_v4();

        tracing::info!(
            request_id = %request_id,
            chain_id = %chain_id,
            application_id = %self.application_id,
            token_id = ?args.token_id,
            score_increase = args.score_increase,
            achievement_count = args.new_achievements.len(),
            "Submitting update operation to blockchain"
        );

        // Use GraphQL mutation to submit operation
        let graphql_endpoint = format!(
            "{}/chains/{}/applications/{}",
            self.linera_rpc_endpoint, chain_id, self.application_id
        );

        // Convert token_id bytes to array format for GraphQL
        let token_id_array = args.token_id.id.clone();

        // Build GraphQL mutation
        let mutation = format!(
            r#"
            mutation {{
                updateAchievements(
                    tokenId: {{ id: {:?} }}
                    newAchievements: {:?}
                    scoreIncrease: {}
                )
            }}
            "#,
            token_id_array,
            args.new_achievements,
            args.score_increase
        );

        let request = serde_json::json!({
            "query": mutation
        });

        let response = self
            .http
            .post(&graphql_endpoint)
            .json(&request)
            .send()
            .await
            .context("failed to send GraphQL request")?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("GraphQL request failed with status {}: {}", status, body);
        }

        #[derive(Deserialize)]
        struct GraphqlResponse {
            data: Option<Value>,
            errors: Option<Vec<GraphqlError>>,
        }

        #[derive(Deserialize)]
        struct GraphqlError {
            message: String,
        }

        let result: GraphqlResponse = serde_json::from_str(&body)
            .context("failed to parse GraphQL response")?;

        if let Some(errors) = result.errors {
            let messages: Vec<_> = errors.iter().map(|e| e.message.as_str()).collect();
            anyhow::bail!("GraphQL errors: {}", messages.join(", "));
        }

        tracing::info!(
            request_id = %request_id,
            "Operation submitted successfully"
        );

        Ok(())
    }

    pub async fn submit_mint(&self, chain_id: ChainId, args: MintArgs) -> Result<()> {
        let request_id = Uuid::new_v4();
        tracing::info!(request_id = %request_id, chain_id = %chain_id, application_id = %self.application_id, token_id = ?args.token_id, metadata_uri = %args.metadata_uri, "Submitting mint operation to blockchain");
        let graphql_endpoint = format!("{}/chains/{}/applications/{}", self.linera_rpc_endpoint, chain_id, self.application_id);
        let token_id_array = args.token_id.id.clone();
        let mutation = format!(r#"mutation {{ mint(tokenId: {{ id: {:?} }} metadataUri: "{}" imageUri: "{}" contentHash: "{}") }}"#, token_id_array, args.metadata_uri, args.image_uri, args.content_hash);
        let request = serde_json::json!({"query": mutation});
        let response = self.http.post(&graphql_endpoint).json(&request).send().await.context("failed to send GraphQL mint request")?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() { anyhow::bail!("GraphQL mint request failed with status {}: {}", status, body); }
        #[derive(Deserialize)]
        struct GraphqlResponse { data: Option<Value>, errors: Option<Vec<GraphqlError>> }
        #[derive(Deserialize)]
        struct GraphqlError { message: String }
        let result: GraphqlResponse = serde_json::from_str(&body).context("failed to parse GraphQL response")?;
        if let Some(errors) = result.errors { let messages: Vec<_> = errors.iter().map(|e| e.message.as_str()).collect(); anyhow::bail!("GraphQL mint errors: {}", messages.join(", ")); }
        tracing::info!(request_id = %request_id, "Mint operation submitted successfully");
        Ok(())
    }
}

pub struct WalletSigner {
    wallet_path: PathBuf,
    owner_keys: BTreeMap<AccountOwner, AccountSecretKey>,
}

#[async_trait]
impl PayloadSigner for WalletSigner {
    async fn sign(
        &self,
        _request_id: Uuid,
        chain_id: ChainId,
        hash: CryptoHash,
    ) -> Result<AccountSignature> {
        let secret = self.key_for_chain(chain_id)?;
        Ok(secret.sign_prehash(hash))
    }
}

impl WalletSigner {
    pub async fn from_config(config: &crate::config::AppConfig) -> Result<Arc<dyn PayloadSigner>> {
        let wallet_path = Self::resolve_wallet_path(config)?;
        let keystore_path = Self::derive_keystore_path(&wallet_path)?;
        let owner_keys = Self::load_keystore(&keystore_path).await?;

        Ok(Arc::new(WalletSigner {
            wallet_path,
            owner_keys,
        }))
    }

    fn resolve_wallet_path(config: &crate::config::AppConfig) -> Result<PathBuf> {
        if config.wallet_path.trim().is_empty() {
            anyhow::bail!("wallet_path in configuration cannot be empty");
        }
        Ok(PathBuf::from(&config.wallet_path))
    }

    fn derive_keystore_path(wallet_path: &PathBuf) -> Result<PathBuf> {
        let mut keystore_path = wallet_path.clone();
        keystore_path.set_file_name("keystore.json");
        Ok(keystore_path)
    }

    async fn load_keystore(path: &PathBuf) -> Result<BTreeMap<AccountOwner, AccountSecretKey>> {
        let bytes = tokio::fs::read(path)
            .await
            .with_context(|| format!("failed to read keystore at {}", path.display()))?;
        let json: Value = serde_json::from_slice(&bytes)
            .with_context(|| format!("failed to parse keystore {}", path.display()))?;
        let entries = json
            .get("keys")
            .and_then(|value| value.as_array())
            .ok_or_else(|| anyhow!("keystore file missing keys field"))?;

        let mut owner_keys = BTreeMap::new();
        for entry in entries {
            let Some(owner_hex) = entry.get(0).and_then(|v| v.as_str()) else {
                anyhow::bail!("malformed keystore entry (no owner)");
            };
            let owner: AccountOwner = owner_hex.parse().context("invalid owner in keystore")?;

            let Some(secret_hex) = entry.get(1).and_then(|v| v.as_str()) else {
                anyhow::bail!("malformed keystore entry (no secret)");
            };
            let secret_bytes = Vec::from_hex(secret_hex)
                .with_context(|| format!("invalid secret hex for owner {owner_hex}"))?;
            let secret: AccountSecretKey = serde_json::from_slice(&secret_bytes)
                .with_context(|| format!("failed to decode secret for owner {owner_hex}"))?;
            owner_keys.insert(owner, secret);
        }

        Ok(owner_keys)
    }

    fn key_for_chain(&self, chain_id: ChainId) -> Result<AccountSecretKey> {
        let wallet = Self::load_wallet(&self.wallet_path)?;
        let chain = wallet
            .chains
            .get(&chain_id)
            .ok_or_else(|| anyhow!("wallet missing chain {chain_id}"))?;
        let owner = chain
            .owner
            .ok_or_else(|| anyhow!("chain {chain_id} has no owner recorded in wallet"))?;
        let secret = self
            .owner_keys
            .get(&owner)
            .ok_or_else(|| anyhow!("keystore missing key for owner {owner}"))?;
        Ok(secret.copy())
    }

    fn load_wallet(path: &PathBuf) -> Result<Wallet> {
        let file =
            PersistentFile::<Wallet>::read(path).context("failed to open wallet file for signing")?;
        Ok(file.into_value())
    }
}
