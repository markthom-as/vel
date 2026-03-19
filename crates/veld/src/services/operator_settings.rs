use vel_config::AppConfig;

use crate::errors::AppError;

pub(crate) const NODE_DISPLAY_NAME_SETTINGS_KEY: &str = "node_display_name";
pub(crate) const TAILSCALE_PREFERRED_SETTINGS_KEY: &str = "tailscale_preferred";
pub(crate) const TAILSCALE_BASE_URL_SETTINGS_KEY: &str = "tailscale_base_url";
pub(crate) const LAN_BASE_URL_SETTINGS_KEY: &str = "lan_base_url";

pub(crate) async fn runtime_sync_config(
    storage: &vel_storage::Storage,
    config: &AppConfig,
) -> Result<AppConfig, AppError> {
    let settings = storage.get_all_settings().await?;
    let mut runtime = config.clone();

    if let Some(value) = string_setting(&settings, NODE_DISPLAY_NAME_SETTINGS_KEY)? {
        runtime.node_display_name = value;
    }
    if let Some(value) = bool_setting(&settings, TAILSCALE_PREFERRED_SETTINGS_KEY)? {
        runtime.tailscale_preferred = value;
    }
    if let Some(value) = string_setting(&settings, TAILSCALE_BASE_URL_SETTINGS_KEY)? {
        runtime.tailscale_base_url = value;
    }
    if let Some(value) = string_setting(&settings, LAN_BASE_URL_SETTINGS_KEY)? {
        runtime.lan_base_url = value;
    }
    if runtime.tailscale_base_url.is_none() {
        runtime.tailscale_base_url = crate::services::tailscale::discover_base_url(&runtime).await;
    }

    Ok(runtime)
}

fn bool_setting(
    settings: &std::collections::HashMap<String, serde_json::Value>,
    key: &str,
) -> Result<Option<bool>, AppError> {
    match settings.get(key) {
        None => Ok(None),
        Some(serde_json::Value::Bool(value)) => Ok(Some(*value)),
        Some(other) => Err(AppError::internal(format!(
            "setting {} should be bool, got {}",
            key, other
        ))),
    }
}

fn string_setting(
    settings: &std::collections::HashMap<String, serde_json::Value>,
    key: &str,
) -> Result<Option<Option<String>>, AppError> {
    match settings.get(key) {
        None => Ok(None),
        Some(serde_json::Value::Null) => Ok(Some(None)),
        Some(serde_json::Value::String(value)) => Ok(Some(Some(value.trim().to_string()))),
        Some(other) => Err(AppError::internal(format!(
            "setting {} should be string or null, got {}",
            key, other
        ))),
    }
}
