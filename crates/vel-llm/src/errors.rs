//! Structured provider and transport errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("no provider registered for profile '{0}'")]
    NoProviderForProfile(String),

    #[error("router/config: {0}")]
    Config(String),
}

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("transport: {0}")]
    Transport(String),

    #[error("protocol: {0}")]
    Protocol(String),

    #[error("capability unsupported: {0}")]
    Capability(String),

    #[error("auth: {0}")]
    Auth(String),

    #[error("rate limit: {0}")]
    RateLimit(String),

    #[error("backend: {0}")]
    Backend(String),
}
