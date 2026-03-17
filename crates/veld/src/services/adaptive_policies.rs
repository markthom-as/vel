//! Adaptive policy overrides persisted from accepted suggestions.

use serde::{Deserialize, Serialize};
use vel_storage::Storage;

use crate::errors::AppError;

const ADAPTIVE_POLICY_OVERRIDES_KEY: &str = "adaptive_policy_overrides";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdaptivePolicyOverrides {
    pub default_prep_minutes: Option<u32>,
    pub commute_buffer_minutes: Option<u32>,
}

pub async fn load(storage: &Storage) -> Result<AdaptivePolicyOverrides, AppError> {
    let settings = storage.get_all_settings().await?;
    let Some(value) = settings.get(ADAPTIVE_POLICY_OVERRIDES_KEY).cloned() else {
        return Ok(AdaptivePolicyOverrides::default());
    };
    Ok(serde_json::from_value(value).unwrap_or_default())
}

pub async fn apply_suggestion_acceptance(
    storage: &Storage,
    suggestion_type: &str,
    payload: &serde_json::Value,
) -> Result<bool, AppError> {
    let suggested_minutes = payload
        .get("suggested_minutes")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u32::try_from(value).ok());
    let Some(suggested_minutes) = suggested_minutes else {
        return Ok(false);
    };

    let mut overrides = load(storage).await?;
    let changed = match suggestion_type {
        "increase_commute_buffer" => {
            if overrides.commute_buffer_minutes == Some(suggested_minutes) {
                false
            } else {
                overrides.commute_buffer_minutes = Some(suggested_minutes);
                true
            }
        }
        "increase_prep_window" => {
            if overrides.default_prep_minutes == Some(suggested_minutes) {
                false
            } else {
                overrides.default_prep_minutes = Some(suggested_minutes);
                true
            }
        }
        _ => false,
    };

    if changed {
        storage
            .set_setting(
                ADAPTIVE_POLICY_OVERRIDES_KEY,
                &serde_json::to_value(overrides)
                    .map_err(|error| AppError::internal(error.to_string()))?,
            )
            .await?;
    }

    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn apply_suggestion_acceptance_updates_both_supported_overrides() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        assert!(
            apply_suggestion_acceptance(
                &storage,
                "increase_commute_buffer",
                &serde_json::json!({ "suggested_minutes": 30 }),
            )
            .await
            .unwrap()
        );
        assert!(
            apply_suggestion_acceptance(
                &storage,
                "increase_prep_window",
                &serde_json::json!({ "suggested_minutes": 45 }),
            )
            .await
            .unwrap()
        );

        let overrides = load(&storage).await.unwrap();
        assert_eq!(overrides.commute_buffer_minutes, Some(30));
        assert_eq!(overrides.default_prep_minutes, Some(45));
    }
}
