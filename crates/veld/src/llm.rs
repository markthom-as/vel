//! Build vel_llm Router from configs/models. Used for chat assistant replies.

use std::path::Path;
use std::sync::Arc;
use vel_config::{load_model_profiles, load_routing, RoutingConfig};
use vel_llm::{
    LlamaCppConfig, LlamaCppProvider, OpenAiOauthConfig, OpenAiOauthProvider, ProviderRegistry,
    Router,
};

const DEFAULT_MODELS_DIR: &str = "configs/models";
const OPENAI_OAUTH_ENV: &str = "VEL_ENABLE_OPENAI_OAUTH";

fn is_local_host(base_url: &str) -> bool {
    let host = match reqwest::Url::parse(base_url) {
        Ok(url) => url.host_str().map(ToString::to_string),
        Err(_) => None,
    };
    matches!(host.as_deref(), Some("localhost") | Some("127.0.0.1"))
}

fn openai_oauth_enabled() -> bool {
    matches!(
        std::env::var(OPENAI_OAUTH_ENV)
            .ok()
            .as_deref()
            .map(|value| value.trim()),
        Some("1") | Some("true") | Some("True") | Some("TRUE") | Some("yes") | Some("on")
    )
}

/// Load routing config from configs/models/routing.toml.
pub fn load_chat_routing() -> RoutingConfig {
    let models_dir =
        std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string());
    let path = Path::new(&models_dir).join("routing.toml");
    load_routing(&path).unwrap_or_default()
}

/// Build a Router from configs/models and return (router, chat_profile_id).
/// If no profiles or no "chat" task in routing, returns (None, None).
pub fn build_chat_router() -> (Option<Router>, Option<String>, Option<String>) {
    let models_dir =
        std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string());
    let profiles = match load_model_profiles(&models_dir) {
        Ok(p) => p,
        Err(e) => {
            tracing::debug!(error = %e, "no model profiles loaded");
            return (None, None, None);
        }
    };
    let routing = load_chat_routing();
    let chat_profile_id = routing
        .profile_for_task("chat")
        .map(String::from)
        .filter(|id| !id.is_empty());
    let fallback_profile_id = routing
        .fallback_remote_profile()
        .map(String::from)
        .filter(|id| !id.is_empty())
        .filter(|id| chat_profile_id.as_ref() != Some(id));

    let chat_profile_id = chat_profile_id.or_else(|| fallback_profile_id.clone());
    if chat_profile_id.is_none() {
        tracing::debug!("no chat profile in routing");
        return (None, None, None);
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
            continue;
        }
        if profile.provider == "openai_oauth" {
            if !openai_oauth_enabled() {
                tracing::debug!(
                    profile_id = %profile.id,
                    "openai_oauth profile skipped: VEL_ENABLE_OPENAI_OAUTH not set"
                );
                continue;
            }
            if !is_local_host(&profile.base_url) {
                tracing::warn!(
                    profile_id = %profile.id,
                    base_url = %profile.base_url,
                    "openai_oauth profile skipped: base_url must point to localhost"
                );
                continue;
            }
            let config = OpenAiOauthConfig {
                base_url: profile.base_url.clone(),
                model_id: profile.model.clone(),
                context_window: profile.context_window,
                supports_tools: profile.supports_tools,
                supports_json: profile.supports_json,
            };
            let provider = OpenAiOauthProvider::new(config);
            registry.register(profile.id.clone(), Arc::new(provider));
            continue;
        }
        tracing::debug!(
            profile_id = %profile.id,
            provider = %profile.provider,
            "skipping unsupported LLM provider"
        );
    }
    if registry.profile_ids().is_empty() {
        tracing::debug!("no LLM providers registered");
        return (None, None, None);
    }
    let router = Router::new(registry);
    tracing::info!(
        chat_profile_id = %chat_profile_id.as_deref().unwrap_or(""),
        fallback_profile_id = %fallback_profile_id.as_deref().unwrap_or(""),
        "chat LLM router ready"
    );
    (Some(router), chat_profile_id, fallback_profile_id)
}
