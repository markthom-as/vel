//! LLM provider trait. Implementations (llama_cpp, openai_oauth, etc.) live in provider adapters.

use async_trait::async_trait;

use crate::types::{LlmRequest, LlmResponse, ModelInfo, ProviderHealth};

/// Provider-agnostic LLM interface. All model backends are accessed through this trait.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a completion for the given request.
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, crate::LlmError>;

    /// Health check for this provider (e.g. /v1/models or connectivity).
    async fn health(&self) -> Result<ProviderHealth, crate::LlmError>;

    /// List models this provider can serve.
    async fn models(&self) -> Result<Vec<ModelInfo>, crate::LlmError>;
}
