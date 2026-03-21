//! Build vel_llm Router from configs/models. Used for chat assistant replies.

use std::path::Path;
use std::sync::Arc;
use vel_config::{load_model_profiles, load_routing};
use vel_llm::{
    LlamaCppConfig, LlamaCppProvider, OpenAiOauthConfig, OpenAiOauthProvider, ProviderRegistry,
    Router,
};

const DEFAULT_MODELS_DIR: &str = "configs/models";

fn is_local_host(base_url: &str) -> bool {
    let host = match reqwest::Url::parse(base_url) {
        Ok(url) => url.host_str().map(ToString::to_string),
        Err(_) => None,
    };
    matches!(host.as_deref(), Some("localhost") | Some("127.0.0.1"))
}

/// Build a Router from configs/models and return (router, chat_profile_id).
/// If no profiles or no "chat" task in routing, returns (None, None).
pub fn build_chat_router() -> (Option<Router>, Option<String>, Option<String>) {
    let models_dir =
        std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string());
    build_chat_router_from_models_dir(Path::new(&models_dir))
}

fn build_chat_router_from_models_dir(
    models_dir: &Path,
) -> (Option<Router>, Option<String>, Option<String>) {
    let models_dir = models_dir.to_string_lossy().to_string();
    let profiles = match load_model_profiles(&models_dir) {
        Ok(p) => p,
        Err(e) => {
            tracing::debug!(error = %e, "no model profiles loaded");
            return (None, None, None);
        }
    };
    let routing = load_routing(Path::new(&models_dir).join("routing.toml")).unwrap_or_default();
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

#[cfg(test)]
mod tests {
    use super::build_chat_router_from_models_dir;
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_models_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("vel_llm_router_{label}_{unique}"));
        fs::create_dir_all(&dir).expect("temp dir should be created");
        dir
    }

    fn write_models_fixture(dir: &Path, routing: &str, profile: &str) {
        fs::write(dir.join("routing.toml"), routing).expect("routing fixture");
        fs::write(dir.join("oauth-openai.toml"), profile).expect("profile fixture");
    }

    #[test]
    fn build_chat_router_registers_local_openai_oauth_without_env_gate() {
        let dir = temp_models_dir("oauth_localhost");
        write_models_fixture(
            &dir,
            "[default]\nchat = \"oauth-openai\"\n",
            "id = \"oauth-openai\"\nprovider = \"openai_oauth\"\nbase_url = \"http://127.0.0.1:8014/v1\"\nmodel = \"gpt-5.4\"\ncontext_window = 32768\nsupports_tools = true\nsupports_json = true\nenabled = true\n",
        );

        let (router, chat_profile_id, fallback_profile_id) =
            build_chat_router_from_models_dir(&dir);
        let profile_ids = router
            .expect("router should be built")
            .registry()
            .profile_ids();

        assert_eq!(chat_profile_id.as_deref(), Some("oauth-openai"));
        assert_eq!(fallback_profile_id, None);
        assert_eq!(profile_ids, vec!["oauth-openai".to_string()]);
    }

    #[test]
    fn build_chat_router_still_rejects_non_local_openai_oauth_profiles() {
        let dir = temp_models_dir("oauth_remote");
        write_models_fixture(
            &dir,
            "[default]\nchat = \"oauth-openai\"\n",
            "id = \"oauth-openai\"\nprovider = \"openai_oauth\"\nbase_url = \"https://api.openai.com/v1\"\nmodel = \"gpt-5.4\"\ncontext_window = 32768\nsupports_tools = true\nsupports_json = true\nenabled = true\n",
        );

        let (router, chat_profile_id, fallback_profile_id) =
            build_chat_router_from_models_dir(&dir);

        assert!(router.is_none());
        assert_eq!(chat_profile_id, None);
        assert_eq!(fallback_profile_id, None);
    }
}
