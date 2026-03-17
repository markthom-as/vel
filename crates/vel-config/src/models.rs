//! Model profiles and task-class routing. See docs/llm-backend-plan/.
//!
//! Load from configs/models/: one .toml per profile (id, provider, base_url, model, ...)
//! and routing.toml ([default] section: task_class -> profile_id). Invalid configs fail on load.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::ConfigError;

/// Single model profile (one backend or one port). Matches configs/models/*.toml.
#[derive(Debug, Clone)]
pub struct ModelProfile {
    pub id: String,
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub context_window: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub supports_tools: bool,
    pub supports_json: bool,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
struct ModelProfileFile {
    id: String,
    provider: String,
    base_url: String,
    model: String,
    #[serde(default)]
    context_window: Option<u32>,
    #[serde(default)]
    max_output_tokens: Option<u32>,
    #[serde(default)]
    temperature: Option<f32>,
    #[serde(default)]
    supports_tools: bool,
    #[serde(default)]
    supports_json: bool,
    #[serde(default = "default_enabled")]
    enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl ModelProfile {
    /// Parse from toml string. Fails if required fields missing or invalid.
    pub fn parse_toml(content: &str) -> Result<Self, ConfigError> {
        let file: ModelProfileFile = toml::from_str(content)?;
        if file.id.is_empty() {
            return Err(ConfigError::Validation(
                "model profile: id must be non-empty".into(),
            ));
        }
        if file.provider.is_empty() {
            return Err(ConfigError::Validation(
                "model profile: provider must be non-empty".into(),
            ));
        }
        if file.base_url.is_empty() {
            return Err(ConfigError::Validation(
                "model profile: base_url must be non-empty".into(),
            ));
        }
        if file.model.is_empty() {
            return Err(ConfigError::Validation(
                "model profile: model must be non-empty".into(),
            ));
        }
        let known = ["llama_cpp", "openai_oauth"];
        if !known.contains(&file.provider.as_str()) {
            return Err(ConfigError::Validation(format!(
                "model profile: unknown provider '{}' (allowed: {})",
                file.provider,
                known.join(", ")
            )));
        }
        Ok(Self {
            id: file.id,
            provider: file.provider,
            base_url: file.base_url,
            model: file.model,
            context_window: file.context_window,
            max_output_tokens: file.max_output_tokens,
            temperature: file.temperature,
            supports_tools: file.supports_tools,
            supports_json: file.supports_json,
            enabled: file.enabled,
        })
    }
}

/// Task class -> profile id. Load from configs/models/routing.toml [default] section.
#[derive(Debug, Clone, Default)]
pub struct RoutingConfig {
    /// task_class (e.g. "chat", "summarize") -> profile_id (e.g. "local-qwen3-coder").
    pub task_to_profile: HashMap<String, String>,
}

impl RoutingConfig {
    /// Resolve profile id for a task class. Returns None if not configured.
    pub fn profile_for_task(&self, task_class: &str) -> Option<&str> {
        self.task_to_profile.get(task_class).map(String::as_str)
    }

    /// Optional remote fallback profile id for task routing.
    pub fn fallback_remote_profile(&self) -> Option<&str> {
        self.task_to_profile
            .get("fallback_remote")
            .map(String::as_str)
    }
}

/// Load all model profiles from a directory. Reads every .toml except routing.toml.
/// Invalid or duplicate id fails on startup.
pub fn load_model_profiles(dir: impl AsRef<Path>) -> Result<Vec<ModelProfile>, ConfigError> {
    let dir = dir.as_ref();
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut profiles = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e != "toml").unwrap_or(true) {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "routing.toml" {
            continue;
        }
        let content = std::fs::read_to_string(&path)?;
        let profile = ModelProfile::parse_toml(&content)?;
        if !seen_ids.insert(profile.id.clone()) {
            return Err(ConfigError::Validation(format!(
                "duplicate model profile id: {}",
                profile.id
            )));
        }
        profiles.push(profile);
    }
    Ok(profiles)
}

/// Load routing from a single toml file. Expects a [default] section with task_class = profile_id.
pub fn load_routing(path: impl AsRef<Path>) -> Result<RoutingConfig, ConfigError> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(RoutingConfig::default());
    }
    let content = std::fs::read_to_string(path)?;
    let file: HashMap<String, HashMap<String, String>> = toml::from_str(&content)?;
    let task_to_profile = file.get("default").cloned().unwrap_or_default();
    Ok(RoutingConfig { task_to_profile })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_models_path(relative: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../configs/models")
            .join(relative)
    }

    #[test]
    fn parse_model_profile_toml() {
        let t = r#"
id = "local-qwen25-fast"
provider = "llama_cpp"
base_url = "http://127.0.0.1:8013/v1"
model = "qwen2.5-coder-14b"
context_window = 8192
supports_tools = true
supports_json = true
enabled = true
"#;
        let p = ModelProfile::parse_toml(t).unwrap();
        assert_eq!(p.id, "local-qwen25-fast");
        assert_eq!(p.provider, "llama_cpp");
        assert_eq!(p.base_url, "http://127.0.0.1:8013/v1");
        assert_eq!(p.context_window, Some(8192));
        assert!(p.supports_tools);
    }

    #[test]
    fn parse_model_profile_rejects_unknown_provider() {
        let t = r#"
id = "x"
provider = "unknown"
base_url = "http://x/v1"
model = "m"
"#;
        let r = ModelProfile::parse_toml(t);
        assert!(r.is_err());
    }

    #[test]
    fn routing_resolve() {
        let mut r = RoutingConfig::default();
        r.task_to_profile
            .insert("chat".into(), "local-qwen3-coder".into());
        r.task_to_profile
            .insert("summarize".into(), "local-qwen25-fast".into());
        r.task_to_profile
            .insert("fallback_remote".into(), "oauth-openai".into());
        assert_eq!(r.profile_for_task("chat"), Some("local-qwen3-coder"));
        assert_eq!(r.profile_for_task("summarize"), Some("local-qwen25-fast"));
        assert_eq!(r.fallback_remote_profile(), Some("oauth-openai"));
        assert_eq!(r.profile_for_task("other"), None);
    }

    #[test]
    fn load_repo_model_profiles_and_routing() {
        let repo_models =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/models");
        if !repo_models.exists() {
            return;
        }
        let profiles = load_model_profiles(&repo_models).unwrap();
        assert!(
            !profiles.is_empty(),
            "configs/models should have at least one profile"
        );
        let routing_path = repo_models.join("routing.toml");
        let routing = load_routing(&routing_path).unwrap();
        assert!(
            routing.profile_for_task("chat").is_some() || routing.task_to_profile.is_empty(),
            "routing should map chat or be empty"
        );
    }

    #[test]
    fn model_profile_template_parses() {
        let template =
            std::fs::read_to_string(repo_models_path("templates/profile.template.toml")).unwrap();
        let profile = ModelProfile::parse_toml(&template).unwrap();
        assert!(!profile.id.is_empty());
        assert!(!profile.provider.is_empty());
    }

    #[test]
    fn model_routing_template_parses() {
        let routing = load_routing(repo_models_path("templates/routing.template.toml")).unwrap();
        assert_eq!(
            routing.profile_for_task("chat"),
            Some("local-primary-profile")
        );
        assert_eq!(
            routing.fallback_remote_profile(),
            Some("remote-review-profile")
        );
    }
}
