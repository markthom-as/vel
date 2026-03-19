use crate::{errors::AppError, services::adaptive_policies, state::AppState};

pub(crate) async fn settings_payload(state: &AppState) -> Result<serde_json::Value, AppError> {
    let mut map = state.storage.get_all_settings().await?;
    let adaptive_overrides = adaptive_policies::load(&state.storage).await?;
    let runtime_config =
        crate::services::operator_settings::runtime_sync_config(&state.storage, &state.config)
            .await?;
    let tailscale_base_url_auto_discovered =
        crate::services::operator_settings::tailscale_base_url_auto_discovered(
            &map,
            &state.config,
            &runtime_config,
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
        "adaptive_policy_overrides".to_string(),
        serde_json::to_value(adaptive_overrides)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    Ok(serde_json::to_value(map).unwrap_or_else(|_| serde_json::json!({})))
}
