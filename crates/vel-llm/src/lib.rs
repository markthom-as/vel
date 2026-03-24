//! Provider-agnostic LLM layer for Vel.
//!
//! Agent and tool code must depend only on `vel-llm`; no direct HTTP calls to
//! model backends. Provider selection is driven by config profile IDs.
//!
//! See docs/llm-backend-plan/ for architecture and implementation tickets.

pub mod errors;
pub mod provider;
pub mod providers;
pub mod registry;
pub mod types;

pub use errors::{LlmError, ProviderError};
pub use provider::LlmProvider;
pub use providers::{
    LlamaCppConfig, LlamaCppProvider, OpenAiApiConfig, OpenAiApiProvider, OpenAiOauthConfig,
    OpenAiOauthProvider,
};
pub use registry::{ProviderRegistry, Router};
pub use types::{
    FinishReason, LlmRequest, LlmResponse, Message, ModelInfo, ProviderHealth, ResponseFormat,
    ToolCall, ToolSpec, Usage,
};
