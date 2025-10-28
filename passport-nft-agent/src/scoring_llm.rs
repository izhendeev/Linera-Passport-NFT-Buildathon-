use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::scoring::{AchievementEntry, AchievementResult, ObservationContext};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[derive(Deserialize)]
struct LlmScoringResponse {
    score: f64,
    achievements: Vec<LlmAchievement>,
    reasoning: String,
}

#[derive(Deserialize)]
struct LlmAchievement {
    code: String,
    explanation: String,
    points: u64,
}

pub async fn llm_scoring_ollama(
    base_url: &str,
    model: &str,
    context: &ObservationContext,
) -> Result<AchievementResult> {
    let total_transactions: u64 = context.actions.iter().map(|e| e.count).sum();
    
    let prompt = format!(
        r#"Analyze blockchain wallet activity. Follow these EXACT scoring rules:

RULE 1: Base Score (mandatory)
- Formula: total_transactions / 10
- Example: 100 tx = 10 points, 500 tx = 50 points

RULE 2: One-time Achievements (award once only):
- CONWAY_PARTICIPANT: +100 points (if any user_operation exists)
- APP_CREATOR: +100 points (if CreateApplication detected)
- NFT_INTERACTION: +50 points (if NFT contract interaction detected)

RULE 3: Transaction Milestones (one-time each):
- MILESTONE_10: +10 points (if total_tx >= 10)
- MILESTONE_50: +25 points (if total_tx >= 50)
- MILESTONE_100: +50 points (if total_tx >= 100)
- MILESTONE_500: +100 points (if total_tx >= 500)

USER DATA:
Total transactions: {}
Actions: {}

IMPORTANT:
1. Base score MUST be exactly: total_tx / 10
2. Each achievement awarded ONCE only
3. Return ONLY JSON, no markdown
4. Explain your reasoning

Return this exact JSON structure:
{{
  "score": <base_score + achievement_points>,
  "achievements": [
    {{"code": "ACHIEVEMENT_CODE", "explanation": "why awarded", "points": <number>}}
  ],
  "reasoning": "Your analysis"
}}"#,
        total_transactions,
        serde_json::to_string_pretty(&context.actions)?
    );

    let client = reqwest::Client::new();
    let ollama_url = format!("{}/api/generate", base_url.trim_end_matches("/v1"));
    
    let request = OllamaRequest {
        model: model.to_string(),
        prompt,
        stream: false,
        format: "json".to_string(),
    };

    tracing::debug!("Calling Ollama at {}", ollama_url);

    let response = client
        .post(&ollama_url)
        .json(&request)
        .send()
        .await
        .context("Failed to call Ollama API")?;

    let ollama_resp: OllamaResponse = response
        .json()
        .await
        .context("Failed to parse Ollama response")?;

    tracing::debug!(llm_response = %ollama_resp.response, "Raw LLM response");

    // Parse the JSON response
    let parsed: LlmScoringResponse = serde_json::from_str(&ollama_resp.response)
        .context("Failed to parse LLM JSON output")?;

    tracing::info!(
        score = parsed.score,
        reasoning = %parsed.reasoning,
        "LLM analysis complete"
    );

    Ok(AchievementResult {
        score: parsed.score.round() as u64,
        achievements: parsed
            .achievements
            .into_iter()
            .map(|a| AchievementEntry {
                code: a.code,
                explanation: a.explanation,
                points: Some(a.points),
            })
            .collect(),
    })
}
