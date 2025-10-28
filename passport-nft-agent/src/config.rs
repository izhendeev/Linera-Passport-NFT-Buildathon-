use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub linera_rpc_endpoint: String,
    pub rpc_endpoint: Option<String>,
    pub graphql_endpoint: String,
    #[serde(default = "default_indexer_endpoint")]
    pub indexer_endpoint: String,
    pub wallet_path: String,
    #[serde(default = "default_storage_path")]
    pub storage_path: String,
    pub application_id: String,
    pub operation_chain_id: String,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
    #[serde(default = "default_rules_path")]
    pub rules_path: String,
    #[serde(default)]
    pub openai: Option<OpenAiConfig>,
    /// CROSS-CHAIN FEATURE: Additional chains to scan for cross-chain reputation
    #[serde(default)]
    pub cross_chain_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAiConfig {
    pub api_key: String,
    #[serde(default = "default_openai_model")]
    pub model: String,
    #[serde(default)]
    pub base_url: Option<String>,
}

impl AppConfig {
    pub fn from_sources() -> Result<Self, anyhow::Error> {
        let mut settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("PASSPORT_AGENT").separator("__"));

        if let Ok(path) = std::env::var("PASSPORT_AGENT_CONFIG") {
            settings = settings.add_source(config::File::with_name(&path));
        }

        settings
            .build()?
            .try_deserialize()
            .map_err(anyhow::Error::from)
    }
}

fn default_poll_interval() -> u64 {
    30
}

fn default_rules_path() -> String {
    "config/achievements.json".to_string()
}

fn default_storage_path() -> String {
    "storage/passport-agent".to_string()
}

fn default_openai_model() -> String {
    "gpt-4.1-mini".to_string()
}

fn default_indexer_endpoint() -> String {
    "http://127.0.0.1:8000/operations".to_string()
}
