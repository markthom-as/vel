//! Provider implementations. Each adapter talks to a concrete backend (llama-server, OpenAI proxy, etc.).

pub mod llama_cpp;

pub use llama_cpp::{LlamaCppConfig, LlamaCppProvider};
