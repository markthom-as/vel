//! Provider registry and router. Selection by config profile ID.

use std::collections::HashMap;
use std::sync::Arc;

use crate::provider::LlmProvider;
use crate::types::LlmRequest;
use crate::{LlmError, LlmResponse};

/// Registry of providers by profile ID. Thread-safe; clone is cheap (Arc).
#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn LlmProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider for a profile ID (e.g. "local-qwen3-coder", "oauth-openai").
    pub fn register(&mut self, profile_id: impl Into<String>, provider: Arc<dyn LlmProvider>) {
        self.providers.insert(profile_id.into(), provider);
    }

    /// Get a provider by profile ID, if registered.
    pub fn get(&self, profile_id: &str) -> Option<Arc<dyn LlmProvider>> {
        self.providers.get(profile_id).cloned()
    }

    /// Return all registered profile IDs.
    pub fn profile_ids(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}

/// Router that selects provider by request's model_profile and dispatches.
pub struct Router {
    registry: Arc<ProviderRegistry>,
}

impl Router {
    pub fn new(registry: ProviderRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Generate using the provider registered for `req.model_profile`.
    pub async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
        let provider = self
            .registry
            .get(req.model_profile.as_str())
            .ok_or_else(|| LlmError::NoProviderForProfile(req.model_profile.clone()))?;
        provider.generate(req).await
    }

    /// Access the registry (e.g. for health checks across providers).
    pub fn registry(&self) -> &ProviderRegistry {
        &self.registry
    }
}
