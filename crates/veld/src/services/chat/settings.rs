use crate::{errors::AppError, services::adaptive_policies};

pub(crate) async fn settings_payload(
    storage: &vel_storage::Storage,
) -> Result<serde_json::Value, AppError> {
    let mut map = storage.get_all_settings().await?;
    let adaptive_overrides = adaptive_policies::load(storage).await?;
    map.insert(
        "adaptive_policy_overrides".to_string(),
        serde_json::to_value(adaptive_overrides)
            .map_err(|error| AppError::internal(error.to_string()))?,
    );
    Ok(serde_json::to_value(map).unwrap_or_else(|_| serde_json::json!({})))
}
