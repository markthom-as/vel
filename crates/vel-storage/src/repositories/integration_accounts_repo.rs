use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationAccountRecord {
    pub id: String,
    pub provider: String,
    pub display_name: String,
    pub external_account_ref: Option<String>,
    pub auth_state: String,
    pub policy_profile: String,
    pub activation_state: String,
    pub sync_posture: String,
    pub metadata_json: JsonValue,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn upsert_integration_account(
    pool: &SqlitePool,
    record: &IntegrationAccountRecord,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO integration_accounts (
            id,
            provider,
            display_name,
            external_account_ref,
            auth_state,
            policy_profile,
            activation_state,
            sync_posture,
            metadata_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            provider = excluded.provider,
            display_name = excluded.display_name,
            external_account_ref = excluded.external_account_ref,
            auth_state = excluded.auth_state,
            policy_profile = excluded.policy_profile,
            activation_state = excluded.activation_state,
            sync_posture = excluded.sync_posture,
            metadata_json = excluded.metadata_json,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&record.id)
    .bind(&record.provider)
    .bind(&record.display_name)
    .bind(&record.external_account_ref)
    .bind(&record.auth_state)
    .bind(&record.policy_profile)
    .bind(&record.activation_state)
    .bind(&record.sync_posture)
    .bind(serde_json::to_string(&record.metadata_json)?)
    .bind(record.created_at.unix_timestamp())
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_integration_account(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<IntegrationAccountRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            provider,
            display_name,
            external_account_ref,
            auth_state,
            policy_profile,
            activation_state,
            sync_posture,
            metadata_json,
            created_at,
            updated_at
        FROM integration_accounts
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_integration_account_row).transpose()
}

fn map_integration_account_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<IntegrationAccountRecord, StorageError> {
    Ok(IntegrationAccountRecord {
        id: row.try_get("id")?,
        provider: row.try_get("provider")?,
        display_name: row.try_get("display_name")?,
        external_account_ref: row.try_get("external_account_ref")?,
        auth_state: row.try_get("auth_state")?,
        policy_profile: row.try_get("policy_profile")?,
        activation_state: row.try_get("activation_state")?,
        sync_posture: row.try_get("sync_posture")?,
        metadata_json: serde_json::from_str(&row.try_get::<String, _>("metadata_json")?)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        updated_at: timestamp_to_datetime(row.try_get("updated_at")?)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::migrate::Migrator;

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn integration_accounts_repo_persists_auth_state_without_secrets() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();
        let record = IntegrationAccountRecord {
            id: "integration_account_01todoist".to_string(),
            provider: "todoist".to_string(),
            display_name: "Primary Todoist".to_string(),
            external_account_ref: Some("acct_123".to_string()),
            auth_state: "authorized".to_string(),
            policy_profile: "read_only".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "full_backlog".to_string(),
            metadata_json: json!({"family":"tasks"}),
            created_at: now,
            updated_at: now,
        };

        upsert_integration_account(&pool, &record).await.unwrap();

        let stored = get_integration_account(&pool, &record.id)
            .await
            .unwrap()
            .expect("integration account should exist");
        assert_eq!(stored.provider, "todoist");
        assert_eq!(stored.auth_state, "authorized");
        assert_eq!(stored.policy_profile, "read_only");
    }
}
