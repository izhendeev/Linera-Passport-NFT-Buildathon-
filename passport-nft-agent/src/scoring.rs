use linera_base::{
    data_types::{Amount, BlockHeight, Timestamp},
    identifiers::{AccountOwner, ApplicationId, ChainId},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{chain_client::PassportInfo, config::AppConfig};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionEvent {
    pub action_type: String,
    pub count: u64,
    pub last_seen: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObservationContext {
    pub passport_id: String,
    pub owner: String,
    pub actions: Vec<ActionEvent>,
    pub aggregates: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AchievementEntry {
    pub code: String,
    pub points: Option<u64>,
    pub explanation: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AchievementResult {
    pub score: u64,
    pub achievements: Vec<AchievementEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnerActivityEvent {
    pub owner: AccountOwner,
    pub chain_id: ChainId,
    pub height: BlockHeight,
    pub operation_index: u64,
    pub timestamp: Option<Timestamp>,
    pub kind: ActivityKind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ActivityKind {
    SystemTransfer {
        amount: Amount,
        recipient: AccountOwner,
    },
    UserOperation {
        application_id: ApplicationId,
        payload: Vec<u8>,
    },
    // SECURITY FIX: Track app creation for APP_CREATOR achievement
    CreateApplication {
        module_id: String,
    },
}

impl OwnerActivityEvent {
    pub fn system_transfer(
        owner: AccountOwner,
        chain_id: ChainId,
        height: BlockHeight,
        operation_index: u64,
        amount: Amount,
        recipient: AccountOwner,
    ) -> Self {
        Self {
            owner,
            chain_id,
            height,
            operation_index,
            timestamp: None,
            kind: ActivityKind::SystemTransfer { amount, recipient },
        }
    }

    pub fn user_operation(
        owner: AccountOwner,
        chain_id: ChainId,
        height: BlockHeight,
        operation_index: u64,
        application_id: ApplicationId,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            owner,
            chain_id,
            height,
            operation_index,
            timestamp: None,
            kind: ActivityKind::UserOperation {
                application_id,
                payload,
            },
        }

    }
    pub fn create_application(
        owner: AccountOwner,
        chain_id: ChainId,
        height: BlockHeight,
        operation_index: u64,
        module_id: String,
    ) -> Self {
        Self {
            owner,
            chain_id,
            height,
            operation_index,
            timestamp: None,
            kind: ActivityKind::CreateApplication { module_id },
        }
    }
    }

impl ObservationContext {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("ObservationContext serializes")
    }

    pub fn from_passport(passport: PassportInfo, activity: Vec<OwnerActivityEvent>) -> Self {
        let passport_id = passport
            .token_id_bytes()
            .ok()
            .flatten()
            .map(hex::encode)
            .unwrap_or_else(|| "unknown".to_string());
        let owner = passport.owner.clone();

        // Calculate wallet age from first transaction
        let wallet_age_days = activity
            .iter()
            .filter_map(|event| event.timestamp)
            .min()
            .map(|first_ts| {
                let now = Timestamp::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros() as u64);
                let age_micros = now.micros().saturating_sub(first_ts.micros());
                let age_secs = age_micros / 1_000_000;
                age_secs / 86400 // convert to days
            })
            .unwrap_or(0);

        let (actions, unique_days) = categorize_activity(&activity);
        let aggregates = compute_aggregates(&actions, unique_days.len(), wallet_age_days);

        ObservationContext {
            passport_id,
            owner,
            actions,
            aggregates,
        }
    }
}

pub fn compute_base_score(total_transactions: u64, transactions_per_point: u64) -> u64 {
    total_transactions / transactions_per_point
}

pub async fn evaluate_rules(
    config: &AppConfig,
    context: &ObservationContext,
) -> anyhow::Result<AchievementResult> {
    let rules_content = tokio::fs::read_to_string(&config.rules_path).await?;
    let rules_json: serde_json::Value = serde_json::from_str(&rules_content)?;
    validate_rules(&rules_json)?;

    // Get total transaction count
    let total_transactions: u64 = context.actions.iter().map(|event| event.count).sum();

    // Get scoring rules
    let scoring_rules = rules_json.get("scoring_rules");
    let transactions_per_point = scoring_rules
        .and_then(|r| r.get("transactions_per_point"))
        .and_then(|v| v.as_u64())
        .unwrap_or(10);

    let daily_activity_points = scoring_rules
        .and_then(|r| r.get("daily_activity_points"))
        .and_then(|v| v.as_u64())
        .unwrap_or(10);

    let wallet_age_points_per_day = scoring_rules
        .and_then(|r| r.get("wallet_age_points_per_day"))
        .and_then(|v| v.as_u64())
        .unwrap_or(1);

    // Calculate base score from transactions
    let base_score = compute_base_score(total_transactions, transactions_per_point);

    // Get unique active days and wallet age from aggregates
    let unique_active_days = context
        .aggregates
        .get("unique_active_days")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let wallet_age_days = context
        .aggregates
        .get("wallet_age_days")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // Calculate daily activity bonus and wallet age bonus
    let daily_bonus = unique_active_days * daily_activity_points;
    let age_bonus = wallet_age_days * wallet_age_points_per_day;

    // Apply achievement rules
    let achievements = apply_rules(&rules_json, context, total_transactions);

    // Sum up achievement points
    let achievement_points: u64 = achievements
        .iter()
        .map(|entry| entry.points.unwrap_or(0))
        .sum();

    let total_score = base_score + daily_bonus + age_bonus + achievement_points;

    tracing::debug!(
        total_transactions = total_transactions,
        base_score = base_score,
        unique_active_days = unique_active_days,
        daily_bonus = daily_bonus,
        wallet_age_days = wallet_age_days,
        age_bonus = age_bonus,
        achievement_points = achievement_points,
        total_score = total_score,
        "Score calculation breakdown"
    );

    Ok(AchievementResult {
        score: total_score,
        achievements,
    })
}

fn validate_rules(rules: &serde_json::Value) -> anyhow::Result<()> {
    static SCHEMA: Lazy<jsonschema::JSONSchema> = Lazy::new(|| {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["achievements"],
            "properties": {
                "scoring_rules": {
                    "type": "object",
                    "properties": {
                        "transactions_per_point": {"type": "integer"},
                        "base_multiplier": {"type": "integer"}
                    }
                },
                "achievements": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["code", "explanation"],
                        "properties": {
                            "code": {"type": "string"},
                            "explanation": {"type": "string"},
                            "points": {"type": "integer", "minimum": 0},
                            "condition": {"type": "object"},
                        },
                    },
                },
            },
        });
        jsonschema::JSONSchema::compile(&schema).expect("static rules schema should compile")
    });

    if let Err(errors) = SCHEMA.validate(rules) {
        let messages = errors
            .map(|err| err.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        anyhow::bail!("rules validation error: {messages}");
    }
    Ok(())
}

fn apply_rules(
    rules: &serde_json::Value,
    context: &ObservationContext,
    total_transactions: u64,
) -> Vec<AchievementEntry> {
    let mut results = Vec::new();
    let Some(list) = rules.get("achievements").and_then(|value| value.as_array()) else {
        return results;
    };

    for entry in list {
        let code = entry
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let explanation = entry
            .get("explanation")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let points = entry.get("points").and_then(|v| v.as_u64());

        let condition = entry.get("condition");
        if condition_matches(condition, context, total_transactions) {
            results.push(AchievementEntry {
                code: code.to_string(),
                points,
                explanation: explanation.to_string(),
            });
        }
    }

    results
}

fn condition_matches(
    condition: Option<&serde_json::Value>,
    context: &ObservationContext,
    total_transactions: u64,
) -> bool {
    match condition {
        None => true,
        Some(serde_json::Value::Object(map)) => {
            for (condition_key, constraint) in map {
                match condition_key.as_str() {
                    // Check for specific action types
                    "system_transfer" | "user_operation" => {
                        let Some(event) = context
                            .actions
                            .iter()
                            .find(|action| action.action_type == *condition_key)
                        else {
                            return false;
                        };

                        if let Some(min_count) = constraint.get("min_count").and_then(|v| v.as_u64()) {
                            if event.count < min_count {
                                return false;
                            }
                        }
                    }
                    // Check total transactions milestone
                    "total_transactions" => {
                        if let Some(min_count) = constraint.get("min_count").and_then(|v| v.as_u64()) {
                            if total_transactions < min_count {
                                return false;
                            }
                        }
                    }
                    // Check for app creation (TODO: need to detect this from activity)
                    // SECURITY FIX: Check for actual app creation (not just any UserOperation)
                    "app_creation" => {
                        let has_created_app = context
                            .actions
                            .iter()
                            .any(|action| action.action_type == "create_application");

                        if !has_created_app {
                            return false;
                        }
                    }
                    "unique_active_days" => {
                        let unique_days = context
                            .aggregates
                            .get("unique_active_days")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);

                        if let Some(min_value) = constraint.get("min").and_then(|v| v.as_u64()) {
                            if unique_days < min_value {
                                return false;
                            }
                        }
                    }
                    // Check wallet age
                    "wallet_age_days" => {
                        let wallet_age = context
                            .aggregates
                            .get("wallet_age_days")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);

                        if let Some(min_value) = constraint.get("min").and_then(|v| v.as_u64()) {
                            if wallet_age < min_value {
                                return false;
                            }
                        }
                    }
                    _ => {
                        // Unknown condition key, skip
                        tracing::warn!("Unknown condition key: {}", condition_key);
                    }
                }
            }
            true
        }
        _ => false,
    }
}

fn categorize_activity(events: &[OwnerActivityEvent]) -> (Vec<ActionEvent>, HashSet<u64>) {
    let mut by_action: HashMap<String, ActionEvent> = HashMap::new();
    let mut unique_days: HashSet<u64> = HashSet::new();

    for event in events {
        // Track unique days (convert timestamp to day number)
        if let Some(ts) = event.timestamp {
            let micros = ts.micros();
            let secs = micros / 1_000_000;
            let days = secs / 86400; // seconds per day
            unique_days.insert(days);
        }

        match &event.kind {
            ActivityKind::SystemTransfer { .. } => {
                accumulate_action(&mut by_action, "system_transfer", event.timestamp);
            }
            ActivityKind::UserOperation { application_id, .. } => {
                let specific_key = format!("user_operation:{}", application_id);
                accumulate_action(&mut by_action, &specific_key, event.timestamp);
                accumulate_action(&mut by_action, "user_operation", event.timestamp);
            }
            ActivityKind::CreateApplication { module_id } => {
                accumulate_action(&mut by_action, "create_application", event.timestamp);
                let specific_key = format!("create_application:{}", module_id);
                accumulate_action(&mut by_action, &specific_key, event.timestamp);
            }
        }
    }

    (by_action.into_values().collect(), unique_days)
}

fn accumulate_action(
    actions: &mut HashMap<String, ActionEvent>,
    key: &str,
    timestamp: Option<Timestamp>,
) {
    let entry = actions.entry(key.to_string()).or_insert(ActionEvent {
        action_type: key.to_string(),
        count: 0,
        last_seen: None,
    });

    entry.count += 1;

    entry.last_seen = timestamp.map(|ts| ts.to_string());
}

fn compute_aggregates(
    actions: &[ActionEvent],
    unique_days: usize,
    wallet_age_days: u64,
) -> HashMap<String, serde_json::Value> {
    let mut aggregates = HashMap::new();
    let total_actions: u64 = actions.iter().map(|event| event.count).sum();
    aggregates.insert(
        "total_actions".to_string(),
        serde_json::json!(total_actions),
    );
    aggregates.insert(
        "unique_active_days".to_string(),
        serde_json::json!(unique_days),
    );
    aggregates.insert(
        "wallet_age_days".to_string(),
        serde_json::json!(wallet_age_days),
    );
    aggregates
}

/// LLM-based scoring with fallback to rule-based
pub async fn evaluate_rules_with_llm(
    config: &AppConfig,
    context: &ObservationContext,
) -> anyhow::Result<AchievementResult> {
    // Try LLM first if configured
    if let Some(ref openai_config) = config.openai {
        match try_llm_scoring(openai_config, context).await {
            Ok(result) => {
                tracing::info!(
                    score = result.score,
                    achievement_count = result.achievements.len(),
                    "LLM scoring successful"
                );
                return Ok(result);
            }
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "LLM scoring failed, falling back to rule-based"
                );
            }
        }
    }

    // Fallback to rule-based scoring
    tracing::info!("Using rule-based scoring");
    evaluate_rules(config, context).await
}

#[cfg(feature = "openai")]

#[cfg(not(feature = "openai"))]
async fn try_llm_scoring(
    _openai_config: &crate::config::OpenAiConfig,
    _context: &ObservationContext,
) -> anyhow::Result<AchievementResult> {
    anyhow::bail!("OpenAI feature not enabled")
}
async fn try_llm_scoring(
    openai_config: &crate::config::OpenAiConfig,
    context: &ObservationContext,
) -> anyhow::Result<AchievementResult> {
    // Use simplified Ollama client
    let base_url = openai_config
        .base_url
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("base_url required for Ollama"))?;

    crate::scoring_llm::llm_scoring_ollama(
        base_url,
        &openai_config.model,
        context,
    )
    .await
}
