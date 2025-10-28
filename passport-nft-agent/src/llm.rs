use anyhow::{anyhow, Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::scoring::ObservationContext;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmRequest {
    pub context: ObservationContext,
    pub schema: serde_json::Value,
    pub instructions: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmResponse<T> {
    pub raw: serde_json::Value,
    pub parsed: T,
}

#[cfg(feature = "openai")]
pub struct OpenAiClient {
    inner: async_openai::Client<async_openai::config::OpenAIConfig>,
    model: String,
}

#[cfg(feature = "openai")]
impl OpenAiClient {
    pub fn new(api_key: &str, model: impl Into<String>, base_url: Option<String>) -> Self {
        use async_openai::config::{OpenAIConfig, OPENAI_API_BASE};

        let cfg = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url.unwrap_or_else(|| OPENAI_API_BASE.to_string()));
        Self {
            inner: async_openai::Client::with_config(cfg),
            model: model.into(),
        }
    }

    pub async fn request<T>(&self, payload: &LlmRequest) -> Result<LlmResponse<T>>
    where
        T: DeserializeOwned,
    {
        use async_openai::types::{
            ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs,
            ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
        };

        let prompt = serde_json::json!({
            "context": payload.context,
            "instructions": payload.instructions,
            "schema": payload.schema,
        })
        .to_string();

        let message = ChatCompletionRequestUserMessageArgs::default()
            .content(ChatCompletionRequestUserMessageContent::Text(prompt))
            .build()?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(vec![ChatCompletionRequestMessage::User(message)])
            .build()?;

        let response = self
            .inner
            .chat()
            .create(request)
            .await
            .context("failed to call OpenAI API")?;

        let choice = response
            .choices
            .into_iter()
            .next()
            .context("missing choice")?;

        let content = choice
            .message
            .content
            .ok_or_else(|| anyhow!("missing message content"))?;

        let raw_json: serde_json::Value =
            serde_json::from_str(&content).context("failed to parse LLM JSON output")?;
        let parsed: T =
            serde_json::from_value(raw_json.clone()).context("failed to deserialize LLM output")?;

        Ok(LlmResponse {
            raw: raw_json,
            parsed,
        })
    }
}
