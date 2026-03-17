use vel_config::AppConfig;

use crate::errors::AppError;

pub(crate) const NODE_DISPLAY_NAME_SETTINGS_KEY: &str = "node_display_name";
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
    if let Some(value) = string_setting(&settings, TAILSCALE_BASE_URL_SETTINGS_KEY)? {
        runtime.tailscale_base_url = value;
    }
    if let Some(value) = string_setting(&settings, LAN_BASE_URL_SETTINGS_KEY)? {
        runtime.lan_base_url = value;
    }

    Ok(runtime)
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
