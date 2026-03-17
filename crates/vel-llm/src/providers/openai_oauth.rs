//! OpenAI-oauth provider wrapper.
//!
//! This adapter intentionally mirrors the OpenAI-compatible API surface used by other providers
//! in this crate, while keeping an explicit localhost policy and explicit opt-in guard.

use async_trait::async_trait;

use super::llama_cpp::LlamaCppConfig;
use super::llama_cpp::LlamaCppProvider;
use crate::provider::LlmProvider;
use crate::types::{LlmRequest, LlmResponse, ModelInfo, ProviderHealth};
use crate::{LlmError, ProviderError};

#[derive(Debug, Clone)]
pub struct OpenAiOauthConfig {
    /// Base URL for openai-oauth proxy (for example `http://127.0.0.1:8014/v1`).
    pub base_url: String,
    /// Model identifier expected by the backend (for example `gpt-5.4`).
    pub model_id: String,
    /// Context window metadata exposed as provider capability.
    pub context_window: Option<u32>,
    /// Whether the backend supports tools.
    pub supports_tools: bool,
    /// Whether the backend supports JSON mode.
    pub supports_json: bool,
}

/// Provider adapter for local OpenAI-compatible OAuth-backed proxies.
///
/// The adapter intentionally reuses `LlamaCppProvider` transport behavior, with an
/// additional localhost policy check for safe defaults.
pub struct OpenAiOauthProvider {
    provider: LlamaCppProvider,
    base_url: String,
}

impl OpenAiOauthProvider {
    pub fn new(config: OpenAiOauthConfig) -> Self {
        let base_url = config.base_url.clone();
        let legacy_config = LlamaCppConfig {
            base_url,
            model_id: config.model_id,
            context_window: config.context_window,
            supports_tools: config.supports_tools,
            supports_json: config.supports_json,
        };
        Self {
            provider: LlamaCppProvider::new(legacy_config),
            base_url: config.base_url,
        }
    }

    fn assert_localhost(&self) -> Result<(), LlmError> {
        let host = reqwest::Url::parse(&self.base_url)
            .ok()
            .and_then(|url| url.host_str().map(ToString::to_string))
            .unwrap_or_default();
        if host == "localhost" || host == "127.0.0.1" {
            return Ok(());
        }
        Err(LlmError::Provider(ProviderError::Auth(
            "openai_oauth base_url must be localhost".to_string(),
        )))
    }
}

#[async_trait]
impl LlmProvider for OpenAiOauthProvider {
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
        self.assert_localhost()?;
        self.provider.generate(req).await
    }

    async fn health(&self) -> Result<ProviderHealth, LlmError> {
        self.assert_localhost()?;
        self.provider.health().await
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        self.assert_localhost()?;
        self.provider.models().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LlmError;
    use crate::ProviderError;

    #[test]
    fn openai_oauth_rejects_non_localhost() {
        let provider = OpenAiOauthProvider::new(OpenAiOauthConfig {
            base_url: "http://example.org:8014/v1".to_string(),
            model_id: "gpt-5.4".to_string(),
            context_window: Some(1024),
            supports_tools: true,
            supports_json: true,
        });
        let err = provider.assert_localhost().unwrap_err();
        match &err {
            LlmError::Provider(ProviderError::Auth(msg)) => {
                assert_eq!(msg, "openai_oauth base_url must be localhost")
            }
            _ => panic!("expected auth error, got {err:?}"),
        }
    }
}
