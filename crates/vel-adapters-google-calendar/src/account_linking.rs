use serde_json::{Value as JsonValue, json};
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_storage::{
    IntegrationAccountRecord, StorageError, get_integration_account, upsert_integration_account,
};

use crate::google_ids::{
    GOOGLE_CALENDAR_MODULE_ID, GOOGLE_CALENDAR_PROVIDER, google_calendar_integration_account_id,
};

#[derive(Debug, Clone)]
pub struct GoogleCalendarCheckpointState {
    pub sync_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GoogleCalendarAccountLinkRequest {
    pub external_account_ref: String,
    pub display_name: String,
    pub auth_state: String,
    pub policy_profile: String,
    pub activation_state: String,
    pub sync_posture: String,
    pub metadata_json: JsonValue,
    pub checkpoints: GoogleCalendarCheckpointState,
}

pub async fn link_google_calendar_account(
    pool: &SqlitePool,
    request: &GoogleCalendarAccountLinkRequest,
) -> Result<IntegrationAccountRecord, StorageError> {
    let account_id = google_calendar_integration_account_id(&request.external_account_ref);
    let now = OffsetDateTime::now_utc();
    let existing = get_integration_account(pool, account_id.as_ref()).await?;
    let created_at = existing
        .as_ref()
        .map(|record| record.created_at)
        .unwrap_or(now);
    let record = IntegrationAccountRecord {
        id: account_id.to_string(),
        provider: GOOGLE_CALENDAR_PROVIDER.to_string(),
        display_name: request.display_name.clone(),
        external_account_ref: Some(request.external_account_ref.clone()),
        auth_state: request.auth_state.clone(),
        policy_profile: request.policy_profile.clone(),
        activation_state: request.activation_state.clone(),
        sync_posture: request.sync_posture.clone(),
        metadata_json: merge_metadata(&request.metadata_json, &request.checkpoints),
        created_at,
        updated_at: now,
    };

    upsert_integration_account(pool, &record).await?;
    Ok(record)
}

fn merge_metadata(
    metadata_json: &JsonValue,
    checkpoints: &GoogleCalendarCheckpointState,
) -> JsonValue {
    let mut metadata = metadata_json.clone();
    if !metadata.is_object() {
        metadata = json!({});
    }

    let JsonValue::Object(ref mut map) = metadata else {
        return metadata;
    };

    map.insert(
        "provider".to_string(),
        JsonValue::String(GOOGLE_CALENDAR_PROVIDER.to_string()),
    );
    map.insert(
        "module_id".to_string(),
        JsonValue::String(GOOGLE_CALENDAR_MODULE_ID.to_string()),
    );
    map.insert(
        "checkpoints".to_string(),
        json!({
            "sync_cursor": checkpoints.sync_cursor,
        }),
    );
    map.insert(
        "history_layers".to_string(),
        json!({
            "current_state": true,
            "sync_linkage": true,
            "provider_activity": "bounded_window",
        }),
    );

    metadata
}

#[cfg(test)]
mod tests {
    use super::{
        GoogleCalendarAccountLinkRequest, GoogleCalendarCheckpointState,
        link_google_calendar_account,
    };
    use serde_json::{Value as JsonValue, json};
    use sqlx::{SqlitePool, migrate::Migrator};

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn google_calendar_account_linking_is_deterministic_and_checkpoint_aware() {
        let pool = test_pool().await;
        let record = link_google_calendar_account(
            &pool,
            &GoogleCalendarAccountLinkRequest {
                external_account_ref: "google_primary".to_string(),
                display_name: "Primary Google".to_string(),
                auth_state: "authorized".to_string(),
                policy_profile: "bounded".to_string(),
                activation_state: "active".to_string(),
                sync_posture: "bounded_window".to_string(),
                metadata_json: json!({"family":"calendar"}),
                checkpoints: GoogleCalendarCheckpointState {
                    sync_cursor: Some("sync_123".to_string()),
                },
            },
        )
        .await
        .unwrap();

        assert_eq!(record.provider, "google-calendar");
        assert_eq!(
            record.metadata_json["checkpoints"]["sync_cursor"],
            JsonValue::String("sync_123".to_string())
        );
        assert_eq!(
            record.metadata_json["module_id"],
            "module.integration.google-calendar"
        );
    }
}
