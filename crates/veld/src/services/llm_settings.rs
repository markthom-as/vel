use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use vel_config::{load_routing, ModelProfile};

use crate::errors::AppError;

const DEFAULT_MODELS_DIR: &str = "configs/models";
pub(crate) const OPENAI_API_SECRET_SETTINGS_PREFIX: &str = "llm_openai_api_secret_";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LlmProfileSettingsData {
    pub id: String,
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub context_window: Option<u32>,
    pub enabled: bool,
    pub editable: bool,
    #[serde(default)]
    pub has_api_key: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LlmSettingsData {
    pub models_dir: String,
    pub default_chat_profile_id: Option<String>,
    pub fallback_chat_profile_id: Option<String>,
    pub profiles: Vec<LlmProfileSettingsData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiCompatProfileUpdateRequest {
    pub id: String,
    pub base_url: String,
    pub model: String,
    pub context_window: Option<u32>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiApiProfileUpdateRequest {
    pub id: String,
    pub base_url: String,
    pub model: String,
    pub context_window: Option<u32>,
    pub enabled: bool,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LlmSettingsUpdateRequest {
    pub default_chat_profile_id: Option<String>,
    pub fallback_chat_profile_id: Option<String>,
    pub openai_compat_profiles: Option<Vec<OpenAiCompatProfileUpdateRequest>>,
    pub openai_api_profiles: Option<Vec<OpenAiApiProfileUpdateRequest>>,
}

#[derive(Debug, Clone)]
struct ProfileSource {
    path: PathBuf,
    profile: ModelProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct OpenAiApiSecrets {
    pub api_key: Option<String>,
}

fn models_dir() -> PathBuf {
    PathBuf::from(
        std::env::var("VEL_MODELS_DIR").unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string()),
    )
}

fn routing_path(models_dir: &Path) -> PathBuf {
    models_dir.join("routing.toml")
}

fn load_profile_sources(models_dir: &Path) -> Result<Vec<ProfileSource>, AppError> {
    if !models_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut sources = Vec::new();
    for entry in fs::read_dir(models_dir).map_err(|error| {
        AppError::internal(format!("read models dir {}: {error}", models_dir.display()))
    })? {
        let entry =
            entry.map_err(|error| AppError::internal(format!("read models dir entry: {error}")))?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("toml") {
            continue;
        }
        if path.file_name().and_then(|value| value.to_str()) == Some("routing.toml") {
            continue;
        }

        let content = fs::read_to_string(&path).map_err(|error| {
            AppError::internal(format!("read model profile {}: {error}", path.display()))
        })?;
        let profile = ModelProfile::parse_toml(&content).map_err(|error| {
            AppError::internal(format!("parse model profile {}: {error}", path.display()))
        })?;
        sources.push(ProfileSource { path, profile });
    }

    sources.sort_by(|left, right| left.profile.id.cmp(&right.profile.id));
    Ok(sources)
}

fn sanitize_profile_id(value: &str) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::bad_request("LLM profile id must not be empty"));
    }
    if !trimmed
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    {
        return Err(AppError::bad_request(
            "LLM profile id must use only ASCII letters, digits, '-' or '_'",
        ));
    }
    Ok(trimmed.to_string())
}

fn validate_local_base_url(base_url: &str) -> Result<String, AppError> {
    let trimmed = base_url.trim();
    let host = reqwest::Url::parse(trimmed)
        .ok()
        .and_then(|url| url.host_str().map(ToString::to_string));
    if matches!(host.as_deref(), Some("localhost") | Some("127.0.0.1")) {
        return Ok(trimmed.to_string());
    }
    Err(AppError::bad_request(
        "OpenAI-compatible base_url must point to localhost or 127.0.0.1",
    ))
}

fn validate_remote_base_url(base_url: &str) -> Result<String, AppError> {
    let trimmed = base_url.trim();
    let url = reqwest::Url::parse(trimmed)
        .map_err(|error| AppError::bad_request(format!("Invalid LLM base_url: {error}")))?;
    if matches!(url.scheme(), "http" | "https") {
        return Ok(trimmed.to_string());
    }
    Err(AppError::bad_request(
        "LLM base_url must use http or https",
    ))
}

fn profile_file_path(models_dir: &Path, profile_id: &str) -> PathBuf {
    models_dir.join(format!("{profile_id}.toml"))
}

fn openai_api_secret_settings_key(profile_id: &str) -> String {
    format!("{OPENAI_API_SECRET_SETTINGS_PREFIX}{profile_id}")
}

fn load_openai_api_secrets_from_map(
    settings: &HashMap<String, serde_json::Value>,
    profile_id: &str,
) -> Result<OpenAiApiSecrets, AppError> {
    match settings.get(&openai_api_secret_settings_key(profile_id)) {
        Some(value) => serde_json::from_value::<OpenAiApiSecrets>(value.clone())
            .map_err(|error| AppError::internal(format!("parse openai api secrets: {error}"))),
        None => Ok(OpenAiApiSecrets::default()),
    }
}

async fn save_openai_api_secrets(
    storage: &vel_storage::Storage,
    profile_id: &str,
    api_key: Option<String>,
) -> Result<(), AppError> {
    let normalized = api_key.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });
    let serialized = serde_json::to_value(OpenAiApiSecrets { api_key: normalized })
        .map_err(|error| AppError::internal(format!("serialize openai api secrets: {error}")))?;
    storage
        .set_setting(&openai_api_secret_settings_key(profile_id), &serialized)
        .await?;
    Ok(())
}

fn write_openai_compat_profile_file(
    path: &Path,
    profile: &OpenAiCompatProfileUpdateRequest,
) -> Result<(), AppError> {
    let profile_id = sanitize_profile_id(&profile.id)?;
    let base_url = validate_local_base_url(&profile.base_url)?;
    let model = profile.model.trim();
    if model.is_empty() {
        return Err(AppError::bad_request("LLM model must not be empty"));
    }

    let mut document = String::new();
    document.push_str(&format!("id = \"{}\"\n", profile_id));
    document.push_str("provider = \"openai_oauth\"\n");
    document.push_str(&format!("base_url = \"{}\"\n", base_url));
    document.push_str(&format!("model = \"{}\"\n", model));
    if let Some(context_window) = profile.context_window {
        document.push_str(&format!("context_window = {}\n", context_window));
    }
    document.push_str("supports_tools = true\n");
    document.push_str("supports_json = true\n");
    document.push_str(&format!(
        "enabled = {}\n",
        if profile.enabled { "true" } else { "false" }
    ));

    fs::write(path, document).map_err(|error| {
        AppError::internal(format!("write model profile {}: {error}", path.display()))
    })
}

fn write_openai_api_profile_file(
    path: &Path,
    profile: &OpenAiApiProfileUpdateRequest,
) -> Result<(), AppError> {
    let profile_id = sanitize_profile_id(&profile.id)?;
    let base_url = validate_remote_base_url(&profile.base_url)?;
    let model = profile.model.trim();
    if model.is_empty() {
        return Err(AppError::bad_request("LLM model must not be empty"));
    }

    let mut document = String::new();
    document.push_str(&format!("id = \"{}\"\n", profile_id));
    document.push_str("provider = \"openai_api\"\n");
    document.push_str(&format!("base_url = \"{}\"\n", base_url));
    document.push_str(&format!("model = \"{}\"\n", model));
    if let Some(context_window) = profile.context_window {
        document.push_str(&format!("context_window = {}\n", context_window));
    }
    document.push_str("supports_tools = true\n");
    document.push_str("supports_json = true\n");
    document.push_str(&format!(
        "enabled = {}\n",
        if profile.enabled { "true" } else { "false" }
    ));

    fs::write(path, document).map_err(|error| {
        AppError::internal(format!("write model profile {}: {error}", path.display()))
    })
}

fn write_routing_file(
    path: &Path,
    default_chat_profile_id: Option<&str>,
    fallback_chat_profile_id: Option<&str>,
) -> Result<(), AppError> {
    let mut document = if path.exists() {
        fs::read_to_string(path)
            .map_err(|error| {
                AppError::internal(format!("read routing config {}: {error}", path.display()))
            })?
            .parse::<toml::Table>()
            .map_err(|error| {
                AppError::internal(format!("parse routing config {}: {error}", path.display()))
            })?
    } else {
        toml::Table::new()
    };

    let default_table = document
        .entry("default")
        .or_insert_with(|| toml::Value::Table(toml::Table::new()))
        .as_table_mut()
        .ok_or_else(|| AppError::internal("routing.toml [default] should be a table"))?;

    match default_chat_profile_id
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(value) => {
            default_table.insert("chat".to_string(), toml::Value::String(value.to_string()));
        }
        None => {
            default_table.remove("chat");
        }
    }

    match fallback_chat_profile_id
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .filter(|value| default_chat_profile_id != Some(*value))
    {
        Some(value) => {
            default_table.insert(
                "fallback_remote".to_string(),
                toml::Value::String(value.to_string()),
            );
        }
        None => {
            default_table.remove("fallback_remote");
        }
    }

    let serialized = toml::to_string_pretty(&document).map_err(|error| {
        AppError::internal(format!(
            "serialize routing config {}: {error}",
            path.display()
        ))
    })?;
    fs::write(path, serialized).map_err(|error| {
        AppError::internal(format!("write routing config {}: {error}", path.display()))
    })
}

fn known_profile_ids(
    existing_sources: &[ProfileSource],
    submitted_openai_compat_profiles: Option<&[OpenAiCompatProfileUpdateRequest]>,
    submitted_openai_api_profiles: Option<&[OpenAiApiProfileUpdateRequest]>,
) -> Result<HashSet<String>, AppError> {
    let mut ids = existing_sources
        .iter()
        .filter(|source| {
            source.profile.provider != "openai_oauth" && source.profile.provider != "openai_api"
        })
        .map(|source| source.profile.id.clone())
        .collect::<HashSet<_>>();

    if let Some(profiles) = submitted_openai_compat_profiles {
        for profile in profiles {
            ids.insert(sanitize_profile_id(&profile.id)?);
        }
    } else {
        for source in existing_sources
            .iter()
            .filter(|source| source.profile.provider == "openai_oauth")
        {
            ids.insert(source.profile.id.clone());
        }
    }

    if let Some(profiles) = submitted_openai_api_profiles {
        for profile in profiles {
            ids.insert(sanitize_profile_id(&profile.id)?);
        }
    } else {
        for source in existing_sources
            .iter()
            .filter(|source| source.profile.provider == "openai_api")
        {
            ids.insert(source.profile.id.clone());
        }
    }

    Ok(ids)
}

fn load_llm_settings_from_map(
    settings: Option<&HashMap<String, serde_json::Value>>,
) -> Result<LlmSettingsData, AppError> {
    let models_dir = models_dir();
    let profile_sources = load_profile_sources(&models_dir)?;
    let routing = load_routing(routing_path(&models_dir)).unwrap_or_default();

    let mut profiles = profile_sources
        .into_iter()
        .map(|source| {
            let has_api_key = if source.profile.provider == "openai_api" {
                settings
                    .map(|value| load_openai_api_secrets_from_map(value, &source.profile.id))
                    .transpose()?
                    .and_then(|secrets| secrets.api_key)
                    .is_some()
            } else {
                false
            };
            Ok(LlmProfileSettingsData {
                id: source.profile.id,
                provider: source.profile.provider.clone(),
                base_url: source.profile.base_url,
                model: source.profile.model,
                context_window: source.profile.context_window,
                enabled: source.profile.enabled,
                editable: source.profile.provider == "openai_oauth"
                    || source.profile.provider == "openai_api",
                has_api_key,
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;
    profiles.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(LlmSettingsData {
        models_dir: models_dir.to_string_lossy().to_string(),
        default_chat_profile_id: routing.profile_for_task("chat").map(ToString::to_string),
        fallback_chat_profile_id: routing.fallback_remote_profile().map(ToString::to_string),
        profiles,
    })
}

pub async fn load_llm_settings(storage: &vel_storage::Storage) -> Result<LlmSettingsData, AppError> {
    let settings = storage.get_all_settings().await?;
    load_llm_settings_from_map(Some(&settings))
}

pub fn load_llm_settings_from_disk() -> Result<LlmSettingsData, AppError> {
    load_llm_settings_from_map(None)
}

pub async fn apply_llm_settings_update(
    storage: &vel_storage::Storage,
    request: &LlmSettingsUpdateRequest,
) -> Result<LlmSettingsData, AppError> {
    let models_dir = models_dir();
    fs::create_dir_all(&models_dir).map_err(|error| {
        AppError::internal(format!(
            "create models dir {}: {error}",
            models_dir.display()
        ))
    })?;

    let existing_sources = load_profile_sources(&models_dir)?;
    let existing_by_id = existing_sources
        .iter()
        .map(|source| (source.profile.id.clone(), source.path.clone()))
        .collect::<HashMap<_, _>>();

    if let Some(openai_profiles) = request.openai_compat_profiles.as_ref() {
        let mut seen_ids = HashSet::new();
        for profile in openai_profiles {
            let profile_id = sanitize_profile_id(&profile.id)?;
            if !seen_ids.insert(profile_id.clone()) {
                return Err(AppError::bad_request(format!(
                    "Duplicate LLM profile id: {profile_id}"
                )));
            }

            let target_path = profile_file_path(&models_dir, &profile_id);
            write_openai_compat_profile_file(&target_path, profile)?;
            if let Some(previous_path) = existing_by_id.get(&profile_id) {
                if previous_path != &target_path && previous_path.exists() {
                    fs::remove_file(previous_path).map_err(|error| {
                        AppError::internal(format!(
                            "remove replaced model profile {}: {error}",
                            previous_path.display()
                        ))
                    })?;
                }
            }
        }

        for source in existing_sources
            .iter()
            .filter(|source| source.profile.provider == "openai_oauth")
        {
            if !seen_ids.contains(&source.profile.id) && source.path.exists() {
                fs::remove_file(&source.path).map_err(|error| {
                    AppError::internal(format!(
                        "remove model profile {}: {error}",
                        source.path.display()
                    ))
                })?;
            }
        }
    }

    if let Some(openai_api_profiles) = request.openai_api_profiles.as_ref() {
        let mut seen_ids = HashSet::new();
        for profile in openai_api_profiles {
            let profile_id = sanitize_profile_id(&profile.id)?;
            if !seen_ids.insert(profile_id.clone()) {
                return Err(AppError::bad_request(format!(
                    "Duplicate LLM profile id: {profile_id}"
                )));
            }

            let target_path = profile_file_path(&models_dir, &profile_id);
            write_openai_api_profile_file(&target_path, profile)?;
            if profile.api_key.is_some() {
                save_openai_api_secrets(storage, &profile_id, profile.api_key.clone()).await?;
            }
            if let Some(previous_path) = existing_by_id.get(&profile_id) {
                if previous_path != &target_path && previous_path.exists() {
                    fs::remove_file(previous_path).map_err(|error| {
                        AppError::internal(format!(
                            "remove replaced model profile {}: {error}",
                            previous_path.display()
                        ))
                    })?;
                }
            }
        }

        for source in existing_sources
            .iter()
            .filter(|source| source.profile.provider == "openai_api")
        {
            if !seen_ids.contains(&source.profile.id) {
                if source.path.exists() {
                    fs::remove_file(&source.path).map_err(|error| {
                        AppError::internal(format!(
                            "remove model profile {}: {error}",
                            source.path.display()
                        ))
                    })?;
                }
                save_openai_api_secrets(storage, &source.profile.id, None).await?;
            }
        }
    }

    let known_ids = known_profile_ids(
        &existing_sources,
        request.openai_compat_profiles.as_deref(),
        request.openai_api_profiles.as_deref(),
    )?;
    let default_chat_profile_id = request
        .default_chat_profile_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let fallback_chat_profile_id = request
        .fallback_chat_profile_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .filter(|value| default_chat_profile_id.as_ref() != Some(value));

    if let Some(profile_id) = default_chat_profile_id.as_ref() {
        if !known_ids.contains(profile_id) {
            return Err(AppError::bad_request(format!(
                "Unknown default chat profile: {profile_id}"
            )));
        }
    }
    if let Some(profile_id) = fallback_chat_profile_id.as_ref() {
        if !known_ids.contains(profile_id) {
            return Err(AppError::bad_request(format!(
                "Unknown fallback chat profile: {profile_id}"
            )));
        }
    }

    write_routing_file(
        &routing_path(&models_dir),
        default_chat_profile_id.as_deref(),
        fallback_chat_profile_id.as_deref(),
    )?;

    load_llm_settings(storage).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_models_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("vel_llm_settings_{label}_{unique}"));
        fs::create_dir_all(&dir).expect("temp dir should be created");
        dir
    }

    #[test]
    fn load_llm_settings_reads_profiles_and_routing() {
        let dir = temp_models_dir("load");
        fs::write(
            dir.join("routing.toml"),
            "[default]\nchat = \"oauth-openai\"\nfallback_remote = \"local-qwen3-coder\"\n",
        )
        .unwrap();
        fs::write(
            dir.join("local-qwen3-coder.toml"),
            "id = \"local-qwen3-coder\"\nprovider = \"llama_cpp\"\nbase_url = \"http://127.0.0.1:8012/v1\"\nmodel = \"qwen3\"\nenabled = true\nsupports_tools = true\nsupports_json = true\n",
        )
        .unwrap();
        fs::write(
            dir.join("oauth-openai.toml"),
            "id = \"oauth-openai\"\nprovider = \"openai_oauth\"\nbase_url = \"http://127.0.0.1:8014/v1\"\nmodel = \"gpt-5.4\"\nenabled = true\nsupports_tools = true\nsupports_json = true\n",
        )
        .unwrap();

        std::env::set_var("VEL_MODELS_DIR", &dir);
        let settings = load_llm_settings_from_disk().unwrap();

        assert_eq!(
            settings.default_chat_profile_id.as_deref(),
            Some("oauth-openai")
        );
        assert_eq!(
            settings.fallback_chat_profile_id.as_deref(),
            Some("local-qwen3-coder")
        );
        assert_eq!(settings.profiles.len(), 2);
        assert!(settings.profiles.iter().any(|profile| profile.editable));
    }

    #[tokio::test]
    async fn apply_llm_settings_update_writes_profiles_and_routing() {
        let dir = temp_models_dir("update");
        fs::write(
            dir.join("routing.toml"),
            "[default]\nchat = \"local-qwen3-coder\"\n",
        )
        .unwrap();
        fs::write(
            dir.join("local-qwen3-coder.toml"),
            "id = \"local-qwen3-coder\"\nprovider = \"llama_cpp\"\nbase_url = \"http://127.0.0.1:8012/v1\"\nmodel = \"qwen3\"\nenabled = true\nsupports_tools = true\nsupports_json = true\n",
        )
        .unwrap();

        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        std::env::set_var("VEL_MODELS_DIR", &dir);
        let settings = apply_llm_settings_update(
            &storage,
            &LlmSettingsUpdateRequest {
                default_chat_profile_id: Some("oauth-openai".to_string()),
                fallback_chat_profile_id: Some("local-qwen3-coder".to_string()),
                openai_compat_profiles: Some(vec![OpenAiCompatProfileUpdateRequest {
                    id: "oauth-openai".to_string(),
                    base_url: "http://127.0.0.1:8014/v1".to_string(),
                    model: "gpt-5.4".to_string(),
                    context_window: Some(32768),
                    enabled: true,
                }]),
                openai_api_profiles: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(
            settings.default_chat_profile_id.as_deref(),
            Some("oauth-openai")
        );
        assert_eq!(
            settings.fallback_chat_profile_id.as_deref(),
            Some("local-qwen3-coder")
        );
        let profile_body = fs::read_to_string(dir.join("oauth-openai.toml")).unwrap();
        assert!(profile_body.contains("provider = \"openai_oauth\""));
        assert!(profile_body.contains("model = \"gpt-5.4\""));
        let routing_body = fs::read_to_string(dir.join("routing.toml")).unwrap();
        assert!(routing_body.contains("chat = \"oauth-openai\""));
        assert!(routing_body.contains("fallback_remote = \"local-qwen3-coder\""));
    }

    #[tokio::test]
    async fn apply_llm_settings_update_persists_openai_api_secrets_without_exposing_them() {
        let dir = temp_models_dir("openai_api");
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        std::env::set_var("VEL_MODELS_DIR", &dir);
        let settings = apply_llm_settings_update(
            &storage,
            &LlmSettingsUpdateRequest {
                default_chat_profile_id: Some("openai-api".to_string()),
                fallback_chat_profile_id: None,
                openai_compat_profiles: None,
                openai_api_profiles: Some(vec![OpenAiApiProfileUpdateRequest {
                    id: "openai-api".to_string(),
                    base_url: "https://api.openai.com/v1".to_string(),
                    model: "gpt-5.4".to_string(),
                    context_window: Some(32768),
                    enabled: true,
                    api_key: Some("sk-test-secret".to_string()),
                }]),
            },
        )
        .await
        .unwrap();

        let saved = storage.get_all_settings().await.unwrap();
        assert_eq!(
            saved[&openai_api_secret_settings_key("openai-api")]["api_key"],
            "sk-test-secret"
        );
        let profile = settings
            .profiles
            .iter()
            .find(|profile| profile.id == "openai-api")
            .expect("openai api profile");
        assert!(profile.has_api_key);
        let profile_body = fs::read_to_string(dir.join("openai-api.toml")).unwrap();
        assert!(profile_body.contains("provider = \"openai_api\""));
        assert!(!profile_body.contains("sk-test-secret"));
    }
}
