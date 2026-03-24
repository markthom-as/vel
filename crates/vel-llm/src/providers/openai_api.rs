//! Direct OpenAI API provider wrapper.
//!
//! This adapter speaks the same OpenAI-compatible surface as other providers in this crate,
//! but authenticates with a server-held bearer token instead of a localhost proxy.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::provider::LlmProvider;
use crate::types::{
    FinishReason, LlmRequest, LlmResponse, ModelInfo, ProviderHealth, ToolCall, Usage,
};
use crate::{LlmError, ProviderError};

#[derive(Debug, Clone)]
pub struct OpenAiApiConfig {
    pub base_url: String,
    pub api_key: String,
    pub model_id: String,
    pub context_window: Option<u32>,
    pub supports_tools: bool,
    pub supports_json: bool,
}

pub struct OpenAiApiProvider {
    client: reqwest::Client,
    config: OpenAiApiConfig,
}

impl OpenAiApiProvider {
    pub fn new(config: OpenAiApiConfig) -> Self {
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

    fn authed_get(&self, url: &str) -> reqwest::RequestBuilder {
        self.client.get(url).bearer_auth(&self.config.api_key)
    }

    fn authed_post(&self, url: &str) -> reqwest::RequestBuilder {
        self.client.post(url).bearer_auth(&self.config.api_key)
    }
}

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
impl LlmProvider for OpenAiApiProvider {
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
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

        let res = self
            .authed_post(&self.chat_completions_url())
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
                status.as_u16(),
                msg
            ))));
        }

        let resp: OpenAIResponse = serde_json::from_value(raw.clone())
            .map_err(|e| LlmError::Provider(ProviderError::Protocol(e.to_string())))?;
        let first = resp
            .choices
            .as_ref()
            .and_then(|v| v.first())
            .ok_or_else(|| LlmError::Provider(ProviderError::Protocol("no choices".into())))?;
        let message = first.message.as_ref().ok_or_else(|| {
            LlmError::Provider(ProviderError::Protocol("missing message".into()))
        })?;

        let tool_calls = message
            .tool_calls
            .as_ref()
            .map(|calls| {
                calls.iter()
                    .map(|call| ToolCall {
                        id: call.id.clone().unwrap_or_default(),
                        name: call
                            .function
                            .as_ref()
                            .and_then(|value| value.name.clone())
                            .unwrap_or_default(),
                        arguments: call
                            .function
                            .as_ref()
                            .and_then(|value| value.arguments.as_deref())
                            .map(|value| {
                                serde_json::from_str(value).unwrap_or_else(
                                    |_| serde_json::Value::String(value.to_string()),
                                )
                            })
                            .unwrap_or(serde_json::Value::Null),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(LlmResponse {
            text: message.content.clone(),
            tool_calls,
            usage: Usage {
                input_tokens: resp.usage.as_ref().and_then(|u| u.prompt_tokens),
                output_tokens: resp.usage.as_ref().and_then(|u| u.completion_tokens),
                total_tokens: resp.usage.as_ref().and_then(|u| u.total_tokens),
            },
            finish_reason: Self::map_finish_reason(first.finish_reason.as_deref()),
            raw,
        })
    }

    async fn health(&self) -> Result<ProviderHealth, LlmError> {
        let res = self
            .authed_get(&self.models_url())
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
                status.as_u16(),
                msg
            ))));
        }
        Ok(ProviderHealth {
            healthy: true,
            details: raw,
        })
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        let res = self
            .authed_get(&self.models_url())
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
                status.as_u16(),
                msg
            ))));
        }

        let parsed: OpenAIModelsResponse = serde_json::from_value(raw)
            .map_err(|e| LlmError::Provider(ProviderError::Protocol(e.to_string())))?;
        Ok(parsed
            .data
            .unwrap_or_default()
            .into_iter()
            .filter_map(|m| m.id)
            .map(|id| ModelInfo {
                id,
                context_window: self.config.context_window,
                supports_tools: self.config.supports_tools,
                supports_json: self.config.supports_json,
            })
            .collect())
    }
}
