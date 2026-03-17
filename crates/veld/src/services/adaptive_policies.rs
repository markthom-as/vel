//! Adaptive policy overrides persisted from accepted suggestions.

use serde::{Deserialize, Serialize};
use vel_storage::Storage;

use crate::errors::AppError;

const ADAPTIVE_POLICY_OVERRIDES_KEY: &str = "adaptive_policy_overrides";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdaptivePolicyOverrides {
    pub default_prep_minutes: Option<u32>,
    pub commute_buffer_minutes: Option<u32>,
    #[serde(default)]
    pub default_prep_source_suggestion_id: Option<String>,
    #[serde(default)]
    pub default_prep_source_title: Option<String>,
    #[serde(default)]
    pub default_prep_source_accepted_at: Option<i64>,
    #[serde(default)]
    pub commute_buffer_source_suggestion_id: Option<String>,
    #[serde(default)]
    pub commute_buffer_source_title: Option<String>,
    #[serde(default)]
    pub commute_buffer_source_accepted_at: Option<i64>,
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
    suggestion: &vel_storage::SuggestionRecord,
    accepted_at: i64,
) -> Result<bool, AppError> {
    let suggested_minutes = suggestion
        .payload_json
        .get("suggested_minutes")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u32::try_from(value).ok());
    let Some(suggested_minutes) = suggested_minutes else {
        return Ok(false);
    };

    let mut overrides = load(storage).await?;
    let changed = match suggestion.suggestion_type.as_str() {
        "increase_commute_buffer" => {
            if overrides.commute_buffer_minutes == Some(suggested_minutes) {
                false
            } else {
                overrides.commute_buffer_minutes = Some(suggested_minutes);
                overrides.commute_buffer_source_suggestion_id = Some(suggestion.id.clone());
                overrides.commute_buffer_source_title = suggestion.title.clone();
                overrides.commute_buffer_source_accepted_at = Some(accepted_at);
                true
            }
        }
        "increase_prep_window" => {
            if overrides.default_prep_minutes == Some(suggested_minutes) {
                false
            } else {
                overrides.default_prep_minutes = Some(suggested_minutes);
                overrides.default_prep_source_suggestion_id = Some(suggestion.id.clone());
                overrides.default_prep_source_title = suggestion.title.clone();
                overrides.default_prep_source_accepted_at = Some(accepted_at);
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
                &vel_storage::SuggestionRecord {
                    id: "sug_commute".to_string(),
                    suggestion_type: "increase_commute_buffer".to_string(),
                    state: "pending".to_string(),
                    title: Some("Increase commute buffer".to_string()),
                    summary: None,
                    priority: 0,
                    confidence: None,
                    dedupe_key: None,
                    payload_json: serde_json::json!({ "suggested_minutes": 30 }),
                    decision_context_json: None,
                    evidence_count: 0,
                    created_at: 10,
                    resolved_at: None,
                },
                100,
            )
            .await
            .unwrap()
        );
        assert!(
            apply_suggestion_acceptance(
                &storage,
                &vel_storage::SuggestionRecord {
                    id: "sug_prep".to_string(),
                    suggestion_type: "increase_prep_window".to_string(),
                    state: "pending".to_string(),
                    title: Some("Increase prep window".to_string()),
                    summary: None,
                    priority: 0,
                    confidence: None,
                    dedupe_key: None,
                    payload_json: serde_json::json!({ "suggested_minutes": 45 }),
                    decision_context_json: None,
                    evidence_count: 0,
                    created_at: 20,
                    resolved_at: None,
                },
                200,
            )
            .await
            .unwrap()
        );

        let overrides = load(&storage).await.unwrap();
        assert_eq!(overrides.commute_buffer_minutes, Some(30));
        assert_eq!(overrides.default_prep_minutes, Some(45));
        assert_eq!(
            overrides.commute_buffer_source_suggestion_id.as_deref(),
            Some("sug_commute")
        );
        assert_eq!(
            overrides.default_prep_source_suggestion_id.as_deref(),
            Some("sug_prep")
        );
        assert_eq!(overrides.commute_buffer_source_accepted_at, Some(100));
        assert_eq!(overrides.default_prep_source_accepted_at, Some(200));
    }
}
