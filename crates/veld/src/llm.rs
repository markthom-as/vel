//! Build vel_llm Router from configs/models. Used for chat assistant replies.

use std::path::Path;
use std::sync::Arc;
use vel_config::{load_model_profiles, load_routing, RoutingConfig};
use vel_llm::{LlamaCppConfig, LlamaCppProvider, ProviderRegistry, Router};

const DEFAULT_MODELS_DIR: &str = "configs/models";

/// Load routing config from configs/models/routing.toml.
pub fn load_chat_routing() -> RoutingConfig {
    let models_dir = std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string());
    let path = Path::new(&models_dir).join("routing.toml");
    load_routing(&path).unwrap_or_default()
}

/// Build a Router from configs/models and return (router, chat_profile_id).
/// If no profiles or no "chat" task in routing, returns (None, None).
pub fn build_chat_router() -> (Option<Router>, Option<String>) {
    let models_dir = std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string());
    let profiles = match load_model_profiles(&models_dir) {
        Ok(p) => p,
        Err(e) => {
            tracing::debug!(error = %e, "no model profiles loaded");
            return (None, None);
        }
    };
    let routing = load_chat_routing();
    let chat_profile_id = routing
        .profile_for_task("chat")
        .map(String::from)
        .filter(|id| !id.is_empty());
    if chat_profile_id.is_none() {
        tracing::debug!("no chat profile in routing");
        return (None, None);
    }
    let mut registry = ProviderRegistry::default();
    for profile in profiles {
        if !profile.enabled {
            continue;
        }
        if profile.provider == "llama_cpp" {
            let config = LlamaCppConfig {
                base_url: profile.base_url.clone(),
                model_id: profile.model.clone(),
                context_window: profile.context_window,
                supports_tools: profile.supports_tools,
                supports_json: profile.supports_json,
            };
            let provider = LlamaCppProvider::new(config);
            registry.register(profile.id.clone(), Arc::new(provider));
        }
        // TODO: openai_oauth and other providers
    }
    if registry.profile_ids().is_empty() {
        tracing::debug!("no LLM providers registered");
        return (None, None);
    }
    let router = Router::new(registry);
    tracing::info!(
        chat_profile_id = %chat_profile_id.as_deref().unwrap_or(""),
        "chat LLM router ready"
    );
    (Some(router), chat_profile_id)
}
