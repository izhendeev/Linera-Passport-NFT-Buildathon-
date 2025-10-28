use graphql_client::{GraphQLQuery, Response};
use hex;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::{str::FromStr, time::Duration};

use linera_base::identifiers::{AccountOwner, ChainId};
use linera_execution::{Operation, SystemOperation};
use linera_indexer_graphql_client::operations::{
    self,
    operations::{OperationKeyKind, Variables},
};

use crate::scoring::OwnerActivityEvent;

#[derive(Debug, Clone)]
pub struct ChainClient {
    http: Client,
    graphql_endpoint: String,
    indexer_endpoint: Url,
}

impl ChainClient {
    pub fn new(graphql_endpoint: impl Into<String>, indexer_endpoint: impl Into<String>) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            graphql_endpoint: graphql_endpoint.into(),
            indexer_endpoint: Url::parse(&indexer_endpoint.into())
                .expect("invalid indexer operations endpoint"),
        }
    }

    pub async fn all_passports(&self) -> Result<Vec<PassportInfo>, reqwest::Error> {
        let query = serde_json::json!({
            "query": r#"
            {
                allPassports {
                    tokenId { id }
                    owner
                    ownerChain
                    achievements
                    score
                }
            }
            "#
        });
        let resp = self
            .http
            .post(&self.graphql_endpoint)
            .json(&query)
            .send()
            .await?;
        let data: GraphQlResponse<AllPassportsData> = resp.json().await?;
        Ok(data.data.all_passports)
    }

    pub async fn passport_by_bytes(
        &self,
        token_id: &[u8],
    ) -> Result<Option<PassportInfo>, anyhow::Error> {
        let all = self.all_passports().await?;
        let needle = hex::encode(token_id);
        Ok(all.into_iter().find(|passport| {
            passport
                .token_id
                .as_bytes()
                .ok()
                .flatten()
                .map(|bytes| hex::encode(bytes) == needle)
                .unwrap_or(false)
        }))
    }

    pub async fn owner_activity(
        &self,
        owner: &AccountOwner,
        chain_id: &ChainId,
    ) -> Result<Vec<OwnerActivityEvent>, anyhow::Error> {
        let variables = Variables {
            from: OperationKeyKind::Last(*chain_id),
            limit: Some(1000),
        };

        let request_body = operations::Operations::build_query(variables);

        let response = self
            .http
            .post(self.indexer_endpoint.clone())
            .json(&request_body)
            .send()
            .await?;

        let response_body: Response<<operations::Operations as GraphQLQuery>::ResponseData> =
            response.json().await?;

        if let Some(errors) = response_body.errors {
            let messages = errors
                .into_iter()
                .map(|err| err.message)
                .collect::<Vec<_>>()
                .join(", ");
            return Err(anyhow::anyhow!("indexer query failed: {messages}"));
        }

        let data = response_body
            .data
            .ok_or_else(|| anyhow::anyhow!("missing operations data"))?;

        let mut events = Vec::new();

        for entry in data.operations {
            match entry.content {
                Operation::System(op) => match *op {
                    SystemOperation::Transfer {
                        owner: op_owner,
                        amount,
                        recipient,
                    } => {
                        if op_owner == *owner {
                            events.push(OwnerActivityEvent::system_transfer(
                                *owner,
                                entry.key.chain_id,
                                entry.key.height,
                                entry.key.index as u64,
                                amount,
                                recipient.owner,
                            ));
                        }
                    }
                    SystemOperation::CreateApplication { module_id, .. } => {
                        // SECURITY FIX: Track app creation for APP_CREATOR achievement
                        events.push(OwnerActivityEvent::create_application(
                            *owner,
                            entry.key.chain_id,
                            entry.key.height,
                            entry.key.index as u64,
                            module_id.to_string(),
                        ));
                    }
                    _ => {}
                },
                Operation::User {
                    application_id,
                    bytes,
                } => {
                    events.push(OwnerActivityEvent::user_operation(
                        *owner,
                        entry.key.chain_id,
                        entry.key.height,
                        entry.key.index as u64,
                        application_id,
                        bytes,
                    ));
                }
            }
        }

        Ok(events)
    }

    /// CROSS-CHAIN FEATURE: Query activity across multiple chains for one owner
    /// This aggregates reputation from all user's microchains
    pub async fn owner_activity_cross_chain(
        &self,
        owner: &AccountOwner,
        chain_ids: &[ChainId],
    ) -> Result<Vec<OwnerActivityEvent>, anyhow::Error> {
        let mut all_events = Vec::new();

        tracing::info!(
            owner = %owner,
            chain_count = chain_ids.len(),
            "Fetching cross-chain activity for owner"
        );

        for chain_id in chain_ids {
            match self.owner_activity(owner, chain_id).await {
                Ok(events) => {
                    tracing::debug!(
                        chain_id = %chain_id,
                        event_count = events.len(),
                        "Fetched activity from chain"
                    );
                    all_events.extend(events);
                }
                Err(err) => {
                    tracing::warn!(
                        chain_id = %chain_id,
                        error = %err,
                        "Failed to fetch activity from chain, continuing with other chains"
                    );
                    // Continue with other chains even if one fails
                }
            }
        }

        tracing::info!(
            owner = %owner,
            total_events = all_events.len(),
            "Cross-chain activity aggregated"
        );

        Ok(all_events)
    }

    /// Get all chain IDs where this owner has activity
    /// For demo, we'll query a configurable list of chains
    pub async fn get_owner_chains(
        &self,
        _owner: &AccountOwner,
        configured_chains: &[ChainId],
    ) -> Result<Vec<ChainId>, anyhow::Error> {
        // For hackathon demo: return configured chains to scan
        // In production, this could query indexer for chains where owner has activity
        Ok(configured_chains.to_vec())
    }
}

#[derive(Debug, Deserialize)]
struct GraphQlResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct AllPassportsData {
    #[serde(rename = "allPassports")]
    all_passports: Vec<PassportInfo>,
}

#[derive(Debug, Deserialize)]
struct PassportsData {
    passport: PassportRoot,
}

#[derive(Debug, Deserialize)]
struct PassportRoot {
    #[serde(rename = "allPassports")]
    all_passports: Vec<PassportInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PassportInfo {
    #[serde(rename = "tokenId")]
    pub token_id: PassportToken,
    pub owner: String,
    #[serde(rename = "ownerChain")]
    pub owner_chain: String,
    pub achievements: Option<Vec<String>>,
    pub score: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct PassportToken {
    pub id: serde_json::Value,
}

impl PassportInfo {
    pub fn owner_account(&self) -> anyhow::Result<AccountOwner> {
        AccountOwner::from_str(&self.owner).map_err(anyhow::Error::from)
    }

    pub fn owner_chain_id(&self) -> anyhow::Result<ChainId> {
        ChainId::from_str(&self.owner_chain).map_err(anyhow::Error::from)
    }

    pub fn token_id_bytes(&self) -> anyhow::Result<Option<Vec<u8>>> {
        self.token_id.as_bytes()
    }
}

impl PassportToken {
    pub fn as_bytes(&self) -> anyhow::Result<Option<Vec<u8>>> {
        match &self.id {
            serde_json::Value::Null => Ok(None),
            serde_json::Value::Array(values) => {
                let mut bytes = Vec::with_capacity(values.len());
                for value in values {
                    let num = value
                        .as_u64()
                        .ok_or_else(|| anyhow::anyhow!("expected u64 token byte, got {value}"))?;
                    if num > u8::MAX as u64 {
                        anyhow::bail!("token byte out of range: {num}");
                    }
                    bytes.push(num as u8);
                }
                Ok(Some(bytes))
            }
            serde_json::Value::String(s) => {
                if s.is_empty() {
                    return Ok(None);
                }
                let s = s.strip_prefix("0x").unwrap_or(s);
                let bytes = hex::decode(s)?;
                Ok(Some(bytes))
            }
            other => Err(anyhow::anyhow!(
                "unexpected token id representation: {other}"
            )),
        }
    }
}
