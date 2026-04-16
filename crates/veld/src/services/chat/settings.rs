use vel_api_types::{BackupSettingsData, WebSettingsData};

use crate::{errors::AppError, services::adaptive_policies, state::AppState};

pub(crate) async fn settings_payload(state: &AppState) -> Result<serde_json::Value, AppError> {
    let mut map = state.storage.get_all_settings().await?;
    let adaptive_overrides = adaptive_policies::load(&state.storage).await?;
    let runtime_config =
        crate::services::operator_settings::runtime_sync_config(&state.storage, &state.config)
            .await?;
    let core_setup_suggestions =
        crate::services::operator_settings::core_setup_suggestions(&runtime_config);
    let discovered_tailscale_base_url =
        crate::services::tailscale::discover_base_url(&runtime_config).await;
    let discovered_lan_base_url =
        crate::services::local_network::discover_lan_base_url(&runtime_config);
    let tailscale_base_url_auto_discovered =
        crate::services::operator_settings::sync_url_auto_discovered(
            runtime_config.tailscale_base_url.as_deref(),
            discovered_tailscale_base_url.as_deref(),
        );
    let lan_base_url_auto_discovered = crate::services::operator_settings::sync_url_auto_discovered(
        runtime_config.lan_base_url.as_deref(),
        discovered_lan_base_url.as_deref(),
    );
    map.insert(
        "node_display_name".to_string(),
        serde_json::to_value(runtime_config.node_display_name)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    map.insert(
        "writeback_enabled".to_string(),
        serde_json::json!(runtime_config.writeback_enabled),
    );
    map.insert(
        "tailscale_preferred".to_string(),
        serde_json::json!(runtime_config.tailscale_preferred),
    );
    map.insert(
        "tailscale_base_url".to_string(),
        serde_json::to_value(runtime_config.tailscale_base_url)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    map.insert(
        "tailscale_base_url_auto_discovered".to_string(),
        serde_json::json!(tailscale_base_url_auto_discovered),
    );
    map.insert(
        "lan_base_url".to_string(),
        serde_json::to_value(runtime_config.lan_base_url)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    map.insert(
        "lan_base_url_auto_discovered".to_string(),
        serde_json::json!(lan_base_url_auto_discovered),
    );
    map.insert(
        "adaptive_policy_overrides".to_string(),
        serde_json::to_value(adaptive_overrides)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    let backup = crate::services::doctor::backup_trust(state).await?;
    map.insert(
        "backup".to_string(),
        serde_json::to_value(BackupSettingsData {
            default_output_root: crate::services::backup::DEFAULT_BACKUP_ROOT.to_string(),
            trust: backup,
        })
        .map_err(|error| AppError::internal(error.to_string()))?,
    );
    map.insert(
        "llm".to_string(),
        serde_json::to_value(
            crate::services::llm_settings::load_llm_settings(&state.storage).await?,
        )
        .map_err(|error| AppError::internal(error.to_string()))?,
    );
    map.insert(
        "web_settings".to_string(),
        serde_json::to_value(load_web_settings_from_map(&map)?)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    map.insert(
        "core_settings".to_string(),
        serde_json::to_value(crate::services::operator_settings::load_core_settings(
            &map,
        )?)
        .map_err(|error| AppError::internal(error.to_string()))?,
    );
    map.insert(
        "core_setup_suggestions".to_string(),
        serde_json::to_value(core_setup_suggestions)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    Ok(serde_json::to_value(map).unwrap_or_else(|_| serde_json::json!({})))
}

pub(crate) fn load_web_settings_from_map(
    map: &std::collections::HashMap<String, serde_json::Value>,
) -> Result<WebSettingsData, AppError> {
    match map.get("web_settings") {
        Some(value) => serde_json::from_value::<WebSettingsData>(value.clone())
            .map_err(|error| AppError::internal(format!("parse web_settings: {error}"))),
        None => Ok(WebSettingsData::default()),
    }
}
