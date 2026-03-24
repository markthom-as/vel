//! Provider implementations. Each adapter talks to a concrete backend (llama-server, OpenAI proxy, etc.).

pub mod llama_cpp;
pub mod openai_api;
pub mod openai_oauth;

pub use llama_cpp::{LlamaCppConfig, LlamaCppProvider};
pub use openai_api::{OpenAiApiConfig, OpenAiApiProvider};
pub use openai_oauth::{OpenAiOauthConfig, OpenAiOauthProvider};
