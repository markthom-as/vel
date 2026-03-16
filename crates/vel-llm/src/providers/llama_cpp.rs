//! OpenAI-compatible client for local llama-server (llama.cpp).
//!
//! Talks to a base URL (e.g. http://127.0.0.1:8013/v1). Uses POST /chat/completions and GET /models.
//! Normalizes usage and finish reasons; returns typed transport/protocol/capability errors.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::provider::LlmProvider;
use crate::types::{
    FinishReason, LlmRequest, LlmResponse, ModelInfo, ProviderHealth, ToolCall, Usage,
};
use crate::{LlmError, ProviderError};

/// Config for a single llama-server instance (one model or one port).
#[derive(Debug, Clone)]
pub struct LlamaCppConfig {
    /// Base URL including /v1 (e.g. http://127.0.0.1:8013/v1).
    pub base_url: String,
    /// Model ID to send in requests (e.g. "qwen2.5-coder-14b").
    pub model_id: String,
    /// Context window for ModelInfo (optional).
    pub context_window: Option<u32>,
    /// Whether the server supports tools.
    pub supports_tools: bool,
    /// Whether the server supports JSON mode.
    pub supports_json: bool,
}

impl Default for LlamaCppConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8080/v1".to_string(),
            model_id: "llama".to_string(),
            context_window: None,
            supports_tools: false,
            supports_json: true,
        }
    }
}

/// Provider that calls a local llama-server via OpenAI-compatible HTTP API.
pub struct LlamaCppProvider {
    client: reqwest::Client,
    config: LlamaCppConfig,
}

impl LlamaCppProvider {
    pub fn new(config: LlamaCppConfig) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("reqwest client"),
            config,
        }
    }

    fn chat_completions_url(&self) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        format!("{}/chat/completions", base)
    }

    fn models_url(&self) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        format!("{}/models", base)
    }

    fn map_finish_reason(s: Option<&str>) -> FinishReason {
        match s {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCall,
            Some("content_filter") => FinishReason::ContentFilter,
            _ => FinishReason::Unknown,
        }
    }
}

// --- OpenAI-compatible request/response DTOs (internal) ---

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<OpenAIResponseFormat>,
}

#[derive(Debug, Serialize)]
struct OpenAITool {
    r#type: String,
    function: OpenAIFunction,
}

#[derive(Debug, Serialize)]
struct OpenAIFunction {
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct OpenAIResponseFormat {
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Option<Vec<OpenAIChoice>>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: Option<OpenAIMessageResponse>,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessageResponse {
    content: Option<String>,
    #[allow(dead_code)]
    role: Option<String>,
    tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIToolCall {
    id: Option<String>,
    #[allow(dead_code)]
    r#type: Option<String>,
    function: Option<OpenAIToolCallFunction>,
}

#[derive(Debug, Deserialize)]
struct OpenAIToolCallFunction {
    name: Option<String>,
    arguments: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct OpenAIModelsResponse {
    data: Option<Vec<OpenAIModel>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIModel {
    id: Option<String>,
}

#[async_trait]
impl LlmProvider for LlamaCppProvider {
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
        // Build OpenAI-style messages, optionally prepending a system prompt.
        let mut messages: Vec<OpenAIMessage> = Vec::new();
        if !req.system.trim().is_empty() {
            messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: req.system.clone(),
            });
        }
        messages.extend(req.messages.iter().map(|m| OpenAIMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        }));

        let tools = if req.tools.is_empty() {
            None
        } else if !self.config.supports_tools {
            return Err(
                ProviderError::Capability("tools not supported by this profile".into()).into(),
            );
        } else {
            Some(
                req.tools
                    .iter()
                    .map(|t| OpenAITool {
                        r#type: "function".to_string(),
                        function: OpenAIFunction {
                            name: t.name.clone(),
                            description: t.description.clone(),
                            parameters: if t.schema.is_null() {
                                None
                            } else {
                                Some(t.schema.clone())
                            },
                        },
                    })
                    .collect::<Vec<_>>(),
            )
        };

        let response_format = match req.response_format {
            crate::types::ResponseFormat::Text => None,
            crate::types::ResponseFormat::JsonObject => {
                if !self.config.supports_json {
                    return Err(ProviderError::Capability(
                        "json_object not supported by this profile".into(),
                    )
                    .into());
                }
                Some(OpenAIResponseFormat {
                    r#type: "json_object".to_string(),
                })
            }
        };

        let body = OpenAIRequest {
            model: self.config.model_id.clone(),
            messages,
            temperature: Some(req.temperature),
            max_tokens: Some(req.max_output_tokens),
            tools,
            response_format,
        };

        let url = self.chat_completions_url();
        let res = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Provider(ProviderError::Transport(e.to_string())))?;

        let status = res.status();
        let raw: serde_json::Value = res
            .json()
            .await
            .map_err(|e| LlmError::Provider(ProviderError::Protocol(e.to_string())))?;

        if !status.is_success() {
            let msg = raw
                .get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .unwrap_or("unknown error");
            return Err(LlmError::Provider(ProviderError::Backend(format!(
                "{}: {}",
                status, msg
            ))));
        }

        let parsed: OpenAIResponse = serde_json::from_value(raw.clone())
            .map_err(|e| LlmError::Provider(ProviderError::Protocol(e.to_string())))?;

        let choice = parsed
            .choices
            .as_ref()
            .and_then(|c| c.first())
            .ok_or_else(|| {
                LlmError::Provider(ProviderError::Protocol("no choices in response".into()))
            })?;

        let message = choice.message.as_ref().ok_or_else(|| {
            LlmError::Provider(ProviderError::Protocol("no message in choice".into()))
        })?;

        let text = message.content.clone();

        let tool_calls: Vec<ToolCall> = message
            .tool_calls
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|tc| {
                let id = tc.id.clone().unwrap_or_default();
                let name = tc.function.as_ref()?.name.clone().unwrap_or_default();
                let args_str = tc.function.as_ref()?.arguments.clone().unwrap_or_default();
                let arguments = serde_json::from_str(&args_str).unwrap_or(serde_json::Value::Null);
                Some(ToolCall {
                    id,
                    name,
                    arguments,
                })
            })
            .collect();

        let usage = parsed
            .usage
            .as_ref()
            .map(|u| Usage {
                input_tokens: u.prompt_tokens,
                output_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            })
            .unwrap_or_default();

        let finish_reason = Self::map_finish_reason(choice.finish_reason.as_deref());

        Ok(LlmResponse {
            text,
            tool_calls,
            usage,
            finish_reason,
            raw,
        })
    }

    async fn health(&self) -> Result<ProviderHealth, LlmError> {
        let url = self.models_url();
        let res = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| LlmError::Provider(ProviderError::Transport(e.to_string())))?;

        let healthy = res.status().is_success();
        let details: serde_json::Value = if healthy {
            res.json()
                .await
                .unwrap_or(serde_json::json!({ "note": "models response ok" }))
        } else {
            let status = res.status();
            serde_json::json!({ "status": status.as_u16(), "error": "models endpoint failed" })
        };

        Ok(ProviderHealth { healthy, details })
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let url = self.models_url();
        let res = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| LlmError::Provider(ProviderError::Transport(e.to_string())))?;

        if !res.status().is_success() {
            return Err(LlmError::Provider(ProviderError::Transport(format!(
                "models endpoint returned {}",
                res.status()
            ))));
        }

        let parsed: OpenAIModelsResponse = res
            .json()
            .await
            .map_err(|e| LlmError::Provider(ProviderError::Protocol(e.to_string())))?;

        let list = parsed.data.unwrap_or_default();
        if list.is_empty() {
            return Ok(vec![ModelInfo {
                id: self.config.model_id.clone(),
                context_window: self.config.context_window,
                supports_tools: self.config.supports_tools,
                supports_json: self.config.supports_json,
            }]);
        }

        let models: Vec<ModelInfo> = list
            .into_iter()
            .map(|m| ModelInfo {
                id: m.id.unwrap_or_else(|| self.config.model_id.clone()),
                context_window: self.config.context_window,
                supports_tools: self.config.supports_tools,
                supports_json: self.config.supports_json,
            })
            .collect();

        Ok(models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LlmError;
    use crate::ProviderError;

    #[tokio::test]
    async fn health_returns_transport_error_when_server_unreachable() {
        let config = LlamaCppConfig {
            base_url: "http://127.0.0.1:31999/v1".to_string(),
            model_id: "test".to_string(),
            ..Default::default()
        };
        let provider = LlamaCppProvider::new(config);
        let result = provider.health().await;
        let err = result.unwrap_err();
        match &err {
            LlmError::Provider(ProviderError::Transport(_)) => {}
            _ => panic!("expected Transport error, got {:?}", err),
        }
    }
}
