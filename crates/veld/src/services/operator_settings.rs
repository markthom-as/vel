use vel_config::AppConfig;

use crate::errors::AppError;

pub(crate) const NODE_DISPLAY_NAME_SETTINGS_KEY: &str = "node_display_name";
pub(crate) const WRITEBACK_ENABLED_SETTINGS_KEY: &str = "writeback_enabled";
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
    if let Some(value) = bool_setting(&settings, WRITEBACK_ENABLED_SETTINGS_KEY)? {
        runtime.writeback_enabled = value;
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

pub(crate) async fn runtime_writeback_enabled(
    storage: &vel_storage::Storage,
    config: &AppConfig,
) -> Result<bool, AppError> {
    let settings = storage.get_all_settings().await?;
    Ok(
        bool_setting(&settings, WRITEBACK_ENABLED_SETTINGS_KEY)?
            .unwrap_or(config.writeback_enabled),
    )
}

pub(crate) fn tailscale_base_url_auto_discovered(
    settings: &std::collections::HashMap<String, serde_json::Value>,
    config: &AppConfig,
    runtime: &AppConfig,
) -> bool {
    !settings.contains_key(TAILSCALE_BASE_URL_SETTINGS_KEY)
        && config.tailscale_base_url.is_none()
        && runtime
            .tailscale_base_url
            .as_ref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn tailscale_auto_discovery_flag_is_true_for_discovered_runtime_value() {
        let settings = HashMap::new();
        let config = AppConfig::default();
        let mut runtime = AppConfig::default();
        runtime.tailscale_base_url = Some("http://vel-desktop.tailnet.ts.net:4130".to_string());

        assert!(tailscale_base_url_auto_discovered(
            &settings, &config, &runtime
        ));
    }

    #[test]
    fn tailscale_auto_discovery_flag_is_false_when_operator_setting_exists() {
        let settings = HashMap::from([(
            TAILSCALE_BASE_URL_SETTINGS_KEY.to_string(),
            serde_json::json!("http://vel-override.tailnet.ts.net:4130"),
        )]);
        let config = AppConfig::default();
        let mut runtime = AppConfig::default();
        runtime.tailscale_base_url = Some("http://vel-override.tailnet.ts.net:4130".to_string());

        assert!(!tailscale_base_url_auto_discovered(
            &settings, &config, &runtime
        ));
    }

    #[test]
    fn tailscale_auto_discovery_flag_is_false_when_config_already_sets_value() {
        let settings = HashMap::new();
        let mut config = AppConfig::default();
        config.tailscale_base_url = Some("http://vel-configured.tailnet.ts.net:4130".to_string());
        let runtime = config.clone();

        assert!(!tailscale_base_url_auto_discovered(
            &settings, &config, &runtime
        ));
    }

    #[tokio::test]
    async fn runtime_writeback_enabled_uses_setting_override() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(WRITEBACK_ENABLED_SETTINGS_KEY, &serde_json::json!(true))
            .await
            .unwrap();

        assert!(runtime_writeback_enabled(&storage, &AppConfig::default())
            .await
            .unwrap());
    }
}
