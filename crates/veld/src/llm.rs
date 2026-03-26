//! Build vel_llm Router from configs/models. Used for chat assistant replies.

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use vel_api_types::{LlmOpenAiOauthLaunchRequestData, LlmProfileHandshakeRequestData};
use vel_config::{load_model_profiles, load_routing, ModelProfile};
use vel_llm::{
    LlamaCppConfig, LlamaCppProvider, LlmError, LlmProvider, OpenAiApiConfig, OpenAiApiProvider,
    OpenAiOauthConfig, OpenAiOauthProvider, ProviderRegistry, Router,
};
use vel_storage::Storage;

use crate::errors::AppError;

const DEFAULT_MODELS_DIR: &str = "configs/models";

fn is_local_host(base_url: &str) -> bool {
    let host = match reqwest::Url::parse(base_url) {
        Ok(url) => url.host_str().map(ToString::to_string),
        Err(_) => None,
    };
    matches!(host.as_deref(), Some("localhost") | Some("127.0.0.1"))
}

fn openai_api_secret_key(profile_id: &str) -> String {
    format!(
        "{}{}",
        crate::services::llm_settings::OPENAI_API_SECRET_SETTINGS_PREFIX,
        profile_id
    )
}

fn openai_api_key_from_settings(
    settings: &std::collections::HashMap<String, serde_json::Value>,
    profile_id: &str,
) -> Option<String> {
    settings
        .get(&openai_api_secret_key(profile_id))
        .and_then(|value| value.get("api_key"))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn validate_handshake_base_url(base_url: &str) -> Result<String, AppError> {
    let trimmed = base_url.trim();
    let url = reqwest::Url::parse(trimmed)
        .map_err(|error| AppError::bad_request(format!("Invalid LLM base_url: {error}")))?;
    if matches!(url.scheme(), "http" | "https") {
        return Ok(trimmed.to_string());
    }
    Err(AppError::bad_request("LLM base_url must use http or https"))
}

fn openai_oauth_start_command(base_url: &str) -> String {
    let parsed = reqwest::Url::parse(base_url).ok();
    let host = parsed
        .as_ref()
        .and_then(|url| url.host_str())
        .filter(|host| !host.is_empty())
        .unwrap_or("127.0.0.1");
    let port = parsed
        .as_ref()
        .and_then(|url| url.port_or_known_default())
        .unwrap_or(8014);
    format!("npx --yes openai-oauth@latest --host {host} --port {port}")
}

fn map_openai_oauth_error(base_url: &str, error: LlmError, context: &str) -> AppError {
    let message = format!(
        "{context}: openai-oauth proxy at {base_url} is unavailable ({error}). Start it with `{}`. If the local auth file is missing, run `npx @openai/codex login` first.",
        openai_oauth_start_command(base_url)
    );
    AppError::internal(message)
}

fn sanitize_log_slug(value: &str) -> String {
    let slug = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        "openai-oauth".to_string()
    } else {
        trimmed.to_string()
    }
}

fn repo_root() -> Result<PathBuf, AppError> {
    std::env::current_dir()
        .map_err(|error| AppError::internal(format!("resolve current working directory: {error}")))
}

fn openai_oauth_script_path(root: &Path) -> PathBuf {
    root.join("scripts").join("openai-oauth.sh")
}

fn openai_oauth_log_path(root: &Path, profile_id: Option<&str>) -> PathBuf {
    let slug = sanitize_log_slug(profile_id.unwrap_or("openai-oauth"));
    root.join("var")
        .join("logs")
        .join(format!("openai-oauth-{slug}.log"))
}

fn openai_oauth_provider(base_url: &str) -> OpenAiOauthProvider {
    OpenAiOauthProvider::new(OpenAiOauthConfig {
        base_url: base_url.to_string(),
        model_id: "gpt-5.4".to_string(),
        context_window: Some(32768),
        supports_tools: true,
        supports_json: true,
    })
}

fn tail_log(path: &Path, max_lines: usize) -> String {
    let Ok(contents) = fs::read_to_string(path) else {
        return String::new();
    };
    let lines = contents.lines().collect::<Vec<_>>();
    let start = lines.len().saturating_sub(max_lines);
    lines[start..].join("\n")
}

fn provider_from_profile(
    settings: Option<&std::collections::HashMap<String, serde_json::Value>>,
    profile: &ModelProfile,
) -> Result<Option<Arc<dyn LlmProvider>>, AppError> {
    if !profile.enabled {
        return Ok(None);
    }

    if profile.provider == "llama_cpp" {
        let config = LlamaCppConfig {
            base_url: profile.base_url.clone(),
            model_id: profile.model.clone(),
            context_window: profile.context_window,
            supports_tools: profile.supports_tools,
            supports_json: profile.supports_json,
        };
        return Ok(Some(Arc::new(LlamaCppProvider::new(config))));
    }

    if profile.provider == "openai_oauth" {
        if !is_local_host(&profile.base_url) {
            tracing::warn!(
                profile_id = %profile.id,
                base_url = %profile.base_url,
                "openai_oauth profile skipped: base_url must point to localhost"
            );
            return Ok(None);
        }
        let config = OpenAiOauthConfig {
            base_url: profile.base_url.clone(),
            model_id: profile.model.clone(),
            context_window: profile.context_window,
            supports_tools: profile.supports_tools,
            supports_json: profile.supports_json,
        };
        return Ok(Some(Arc::new(OpenAiOauthProvider::new(config))));
    }

    if profile.provider == "openai_api" {
        let api_key = settings
            .and_then(|value| openai_api_key_from_settings(value, &profile.id))
            .ok_or_else(|| {
                AppError::bad_request(format!(
                    "OpenAI API key is missing for profile {}",
                    profile.id
                ))
            })?;
        let config = OpenAiApiConfig {
            base_url: profile.base_url.clone(),
            api_key,
            model_id: profile.model.clone(),
            context_window: profile.context_window,
            supports_tools: profile.supports_tools,
            supports_json: profile.supports_json,
        };
        return Ok(Some(Arc::new(OpenAiApiProvider::new(config))));
    }

    tracing::debug!(
        profile_id = %profile.id,
        provider = %profile.provider,
        "skipping unsupported LLM provider"
    );
    Ok(None)
}

/// Build a Router from configs/models and return (router, chat_profile_id, fallback_profile_id).
pub async fn build_chat_router(
    storage: &Storage,
) -> (Option<Router>, Option<String>, Option<String>) {
    let models_dir =
        std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string());
    match storage.get_all_settings().await {
        Ok(settings) => build_chat_router_from_models_dir(Path::new(&models_dir), Some(&settings)),
        Err(error) => {
            tracing::warn!(error = %error, "failed to load settings for llm router");
            (None, None, None)
        }
    }
}

fn build_chat_router_from_models_dir(
    models_dir: &Path,
    settings: Option<&std::collections::HashMap<String, serde_json::Value>>,
) -> (Option<Router>, Option<String>, Option<String>) {
    let models_dir_str = models_dir.to_string_lossy().to_string();
    let profiles = match load_model_profiles(&models_dir_str) {
        Ok(p) => p,
        Err(e) => {
            tracing::debug!(error = %e, "no model profiles loaded");
            return (None, None, None);
        }
    };
    let routing = load_routing(Path::new(&models_dir_str).join("routing.toml")).unwrap_or_default();
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
        match provider_from_profile(settings, &profile) {
            Ok(Some(provider)) => registry.register(profile.id.clone(), provider),
            Ok(None) => {}
            Err(error) => {
                tracing::warn!(
                    profile_id = %profile.id,
                    error = %error,
                    "skipping llm profile because provider setup failed"
                );
            }
        }
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

pub async fn profile_health(
    storage: &Storage,
    profile_id: &str,
) -> Result<vel_llm::ProviderHealth, AppError> {
    let models_dir =
        std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string());
    let profiles = load_model_profiles(&models_dir)
        .map_err(|error| AppError::internal(format!("load llm profiles: {error}")))?;
    let settings = storage.get_all_settings().await?;
    let profile = profiles
        .into_iter()
        .find(|candidate| candidate.id == profile_id)
        .ok_or_else(|| AppError::not_found("llm profile not found"))?;
    let provider = provider_from_profile(Some(&settings), &profile)?
        .ok_or_else(|| AppError::bad_request("llm profile is disabled or unsupported"))?;
    provider.health().await.map_err(|error| {
        if profile.provider == "openai_oauth" {
            map_openai_oauth_error(&profile.base_url, error, "llm health failed")
        } else {
            AppError::internal(format!("llm health failed: {error}"))
        }
    })
}

pub async fn handshake_profile(
    request: &LlmProfileHandshakeRequestData,
) -> Result<vel_llm::ProviderHealth, AppError> {
    let provider_name = request.provider.trim().to_string();
    let base_url = request.base_url.trim().to_string();
    let provider: Arc<dyn LlmProvider> = match request.provider.trim() {
        "llama_cpp" => Arc::new(LlamaCppProvider::new(LlamaCppConfig {
            base_url: validate_handshake_base_url(&request.base_url)?,
            model_id: request.model.trim().to_string(),
            context_window: request.context_window,
            supports_tools: true,
            supports_json: true,
        })),
        "openai_oauth" => {
            if !is_local_host(&request.base_url) {
                return Err(AppError::bad_request(
                    "OpenAI-compatible base_url must point to localhost or 127.0.0.1",
                ));
            }
            Arc::new(OpenAiOauthProvider::new(OpenAiOauthConfig {
                base_url: request.base_url.trim().to_string(),
                model_id: request.model.trim().to_string(),
                context_window: request.context_window,
                supports_tools: true,
                supports_json: true,
            }))
        }
        "openai_api" => {
            let api_key = request
                .api_key
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| AppError::bad_request("OpenAI API key is required for handshake"))?;
            Arc::new(OpenAiApiProvider::new(OpenAiApiConfig {
                base_url: validate_handshake_base_url(&request.base_url)?,
                api_key: api_key.to_string(),
                model_id: request.model.trim().to_string(),
                context_window: request.context_window,
                supports_tools: true,
                supports_json: true,
            }))
        }
        _ => {
            return Err(AppError::bad_request(format!(
                "Unsupported LLM provider for handshake: {}",
                request.provider
            )))
        }
    };

    provider.health().await.map_err(|error| {
        if provider_name == "openai_oauth" {
            map_openai_oauth_error(&base_url, error, "llm handshake failed")
        } else {
            AppError::internal(format!("llm handshake failed: {error}"))
        }
    })
}

pub async fn launch_openai_oauth_proxy(
    request: &LlmOpenAiOauthLaunchRequestData,
) -> Result<vel_llm::ProviderHealth, AppError> {
    let base_url = validate_handshake_base_url(&request.base_url)?;
    if !is_local_host(&base_url) {
        return Err(AppError::bad_request(
            "OpenAI-compatible base_url must point to localhost or 127.0.0.1",
        ));
    }

    let provider = openai_oauth_provider(&base_url);
    if let Ok(health) = provider.health().await {
        return Ok(health);
    }

    let root = repo_root()?;
    let script_path = openai_oauth_script_path(&root);
    if !script_path.is_file() {
        return Err(AppError::internal(format!(
            "OpenAI OAuth launcher script is missing at {}",
            script_path.display()
        )));
    }

    let log_path = openai_oauth_log_path(&root, request.profile_id.as_deref());
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            AppError::internal(format!(
                "create OpenAI OAuth log dir {}: {error}",
                parent.display()
            ))
        })?;
    }
    let stdout = File::create(&log_path).map_err(|error| {
        AppError::internal(format!(
            "create OpenAI OAuth log file {}: {error}",
            log_path.display()
        ))
    })?;
    let stderr = stdout.try_clone().map_err(|error| {
        AppError::internal(format!("clone OpenAI OAuth log file handle: {error}"))
    })?;

    let mut child = Command::new(&script_path)
        .arg("run")
        .arg("--base-url")
        .arg(&base_url)
        .current_dir(&root)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .map_err(|error| {
            AppError::internal(format!(
                "launch OpenAI OAuth proxy with {}: {error}",
                script_path.display()
            ))
        })?;

    for _ in 0..20 {
        if let Ok(health) = provider.health().await {
            return Ok(health);
        }
        if let Some(status) = child
            .try_wait()
            .map_err(|error| AppError::internal(format!("poll OpenAI OAuth launcher: {error}")))?
        {
            let log_excerpt = tail_log(&log_path, 20);
            let detail = if log_excerpt.is_empty() {
                format!("OpenAI OAuth proxy exited early with status {status}.")
            } else {
                format!(
                    "OpenAI OAuth proxy exited early with status {status}. Recent log output:\n{}",
                    log_excerpt
                )
            };
            return Err(AppError::internal(detail));
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    let _ = child.kill();
    let _ = child.wait();
    let log_excerpt = tail_log(&log_path, 20);
    let detail = if log_excerpt.is_empty() {
        format!(
            "OpenAI OAuth proxy did not become ready at {base_url}. Check {}.",
            log_path.display()
        )
    } else {
        format!(
            "OpenAI OAuth proxy did not become ready at {base_url}. Recent log output:\n{}",
            log_excerpt
        )
    };
    Err(AppError::internal(detail))
}

#[cfg(test)]
mod tests {
    use super::{build_chat_router_from_models_dir, handshake_profile};
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };
    use vel_api_types::LlmProfileHandshakeRequestData;

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
            build_chat_router_from_models_dir(&dir, None);
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
            build_chat_router_from_models_dir(&dir, None);

        assert!(router.is_none());
        assert_eq!(chat_profile_id, None);
        assert_eq!(fallback_profile_id, None);
    }

    #[tokio::test]
    async fn profile_health_uses_stored_openai_api_key() {
        let dir = temp_models_dir("openai_api_health");
        fs::write(
            dir.join("routing.toml"),
            "[default]\nchat = \"openai-api\"\n",
        )
        .unwrap();
        fs::write(
            dir.join("openai-api.toml"),
            "id = \"openai-api\"\nprovider = \"openai_api\"\nbase_url = \"http://127.0.0.1:8014/v1\"\nmodel = \"gpt-5.4\"\nenabled = true\nsupports_tools = true\nsupports_json = true\n",
        )
        .unwrap();

        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(
                &super::openai_api_secret_key("openai-api"),
                &serde_json::json!({ "api_key": "sk-test" }),
            )
            .await
            .unwrap();

        let profiles = vel_config::load_model_profiles(&dir).unwrap();
        let settings = storage.get_all_settings().await.unwrap();
        let profile = profiles
            .into_iter()
            .find(|candidate| candidate.id == "openai-api")
            .unwrap();
        let provider = super::provider_from_profile(Some(&settings), &profile).unwrap();
        assert!(provider.is_some());
    }

    #[tokio::test]
    async fn handshake_profile_returns_actionable_openai_oauth_error_when_proxy_is_missing() {
        let error = handshake_profile(&LlmProfileHandshakeRequestData {
            profile_id: Some("oauth-openai".to_string()),
            provider: "openai_oauth".to_string(),
            base_url: "http://127.0.0.1:9/v1".to_string(),
            model: "gpt-5.4".to_string(),
            context_window: Some(32768),
            api_key: None,
        })
        .await
        .unwrap_err();

        let rendered = error.to_string();
        assert!(rendered.contains("openai-oauth proxy at http://127.0.0.1:9/v1 is unavailable"));
        assert!(rendered.contains("npx --yes openai-oauth@latest --host 127.0.0.1 --port 9"));
        assert!(rendered.contains("npx @openai/codex login"));
    }
}
