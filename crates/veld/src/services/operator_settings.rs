use vel_config::AppConfig;

use crate::errors::AppError;

pub(crate) const CORE_SETTINGS_KEY: &str = "core_settings";
pub(crate) const NODE_DISPLAY_NAME_SETTINGS_KEY: &str = "node_display_name";
pub(crate) const WRITEBACK_ENABLED_SETTINGS_KEY: &str = "writeback_enabled";
pub(crate) const TAILSCALE_PREFERRED_SETTINGS_KEY: &str = "tailscale_preferred";
pub(crate) const TAILSCALE_BASE_URL_SETTINGS_KEY: &str = "tailscale_base_url";
pub(crate) const LAN_BASE_URL_SETTINGS_KEY: &str = "lan_base_url";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub(crate) struct AgentProfileSettings {
    pub role: Option<String>,
    pub preferences: Option<String>,
    pub constraints: Option<String>,
    pub freeform: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub(crate) struct CoreSettings {
    pub user_display_name: Option<String>,
    pub client_location_label: Option<String>,
    #[serde(default)]
    pub developer_mode: bool,
    #[serde(default)]
    pub bypass_setup_gate: bool,
    #[serde(default)]
    pub agent_profile: AgentProfileSettings,
}

impl Default for CoreSettings {
    fn default() -> Self {
        Self {
            user_display_name: None,
            client_location_label: None,
            developer_mode: false,
            bypass_setup_gate: false,
            agent_profile: AgentProfileSettings::default(),
        }
    }
}

pub(crate) fn load_core_settings(
    settings: &std::collections::HashMap<String, serde_json::Value>,
) -> Result<CoreSettings, AppError> {
    match settings.get(CORE_SETTINGS_KEY) {
        Some(value) => serde_json::from_value::<CoreSettings>(value.clone())
            .map_err(|error| AppError::internal(format!("parse core_settings: {error}"))),
        None => Ok(CoreSettings::default()),
    }
}

pub(crate) fn normalize_optional_string(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

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
    let discovered_tailscale_base_url =
        crate::services::tailscale::discover_base_url(&runtime).await;
    let discovered_lan_base_url = crate::services::local_network::discover_lan_base_url(&runtime);
    apply_discovered_sync_urls(
        &mut runtime,
        discovered_tailscale_base_url,
        discovered_lan_base_url,
    );

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

pub(crate) fn sync_url_auto_discovered(
    runtime_value: Option<&str>,
    discovered_value: Option<&str>,
) -> bool {
    match (
        runtime_value.map(str::trim),
        discovered_value.map(str::trim),
    ) {
        (Some(runtime), Some(discovered)) => {
            !runtime.is_empty() && !discovered.is_empty() && runtime == discovered
        }
        _ => false,
    }
}

fn apply_discovered_sync_urls(
    runtime: &mut AppConfig,
    discovered_tailscale_base_url: Option<String>,
    discovered_lan_base_url: Option<String>,
) {
    if let Some(url) = discovered_tailscale_base_url {
        runtime.tailscale_base_url = Some(url);
    }
    if let Some(url) = discovered_lan_base_url {
        runtime.lan_base_url = Some(url);
    }
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
    use super::*;

    #[test]
    fn tailscale_auto_discovery_flag_is_true_for_discovered_runtime_value() {
        assert!(sync_url_auto_discovered(
            Some("http://vel-desktop.tailnet.ts.net:4130"),
            Some("http://vel-desktop.tailnet.ts.net:4130"),
        ));
    }

    #[test]
    fn tailscale_auto_discovery_flag_is_false_when_operator_setting_exists() {
        assert!(!sync_url_auto_discovered(
            Some("http://vel-override.tailnet.ts.net:4130"),
            Some("http://vel-auto.tailnet.ts.net:4130"),
        ));
    }

    #[test]
    fn tailscale_auto_discovery_flag_is_false_when_config_already_sets_value() {
        assert!(!sync_url_auto_discovered(
            Some("http://vel-configured.tailnet.ts.net:4130"),
            None,
        ));
    }

    #[test]
    fn apply_discovered_sync_urls_prefers_discovery_over_saved_values() {
        let mut runtime = AppConfig::default();
        runtime.tailscale_base_url = Some("http://manual.tailnet.ts.net:4130".to_string());
        runtime.lan_base_url = Some("http://192.168.1.99:4130".to_string());

        apply_discovered_sync_urls(
            &mut runtime,
            Some("http://auto.tailnet.ts.net:4130".to_string()),
            Some("http://192.168.1.22:4130".to_string()),
        );

        assert_eq!(
            runtime.tailscale_base_url.as_deref(),
            Some("http://auto.tailnet.ts.net:4130")
        );
        assert_eq!(
            runtime.lan_base_url.as_deref(),
            Some("http://192.168.1.22:4130")
        );
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
