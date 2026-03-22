use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

#[derive(Debug, Clone, PartialEq)]
pub struct SyncLinkRecord {
    pub id: String,
    pub provider: String,
    pub integration_account_id: String,
    pub object_id: String,
    pub remote_id: String,
    pub remote_type: String,
    pub state: String,
    pub authority_mode: String,
    pub remote_version: Option<String>,
    pub metadata_json: JsonValue,
    pub linked_at: OffsetDateTime,
    pub last_seen_at: OffsetDateTime,
}

pub async fn upsert_sync_link(
    pool: &SqlitePool,
    record: &SyncLinkRecord,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO sync_links (
            id,
            provider,
            integration_account_id,
            object_id,
            remote_id,
            remote_type,
            state,
            authority_mode,
            remote_version,
            metadata_json,
            linked_at,
            last_seen_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(provider, integration_account_id, remote_id, remote_type) DO UPDATE SET
            object_id = excluded.object_id,
            state = excluded.state,
            authority_mode = excluded.authority_mode,
            remote_version = excluded.remote_version,
            metadata_json = excluded.metadata_json,
            last_seen_at = excluded.last_seen_at
        "#,
    )
    .bind(&record.id)
    .bind(&record.provider)
    .bind(&record.integration_account_id)
    .bind(&record.object_id)
    .bind(&record.remote_id)
    .bind(&record.remote_type)
    .bind(&record.state)
    .bind(&record.authority_mode)
    .bind(&record.remote_version)
    .bind(serde_json::to_string(&record.metadata_json)?)
    .bind(record.linked_at.unix_timestamp())
    .bind(record.last_seen_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_sync_links_for_object(
    pool: &SqlitePool,
    object_id: &str,
) -> Result<Vec<SyncLinkRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            provider,
            integration_account_id,
            object_id,
            remote_id,
            remote_type,
            state,
            authority_mode,
            remote_version,
            metadata_json,
            linked_at,
            last_seen_at
        FROM sync_links
        WHERE object_id = ?
        ORDER BY last_seen_at DESC, linked_at DESC
        "#,
    )
    .bind(object_id)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_sync_link_row).collect()
}

pub async fn update_sync_link_state(
    pool: &SqlitePool,
    id: &str,
    state: &str,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        UPDATE sync_links
        SET state = ?, last_seen_at = ?
        WHERE id = ?
        "#,
    )
    .bind(state)
    .bind(OffsetDateTime::now_utc().unix_timestamp())
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

fn map_sync_link_row(row: &sqlx::sqlite::SqliteRow) -> Result<SyncLinkRecord, StorageError> {
    Ok(SyncLinkRecord {
        id: row.try_get("id")?,
        provider: row.try_get("provider")?,
        integration_account_id: row.try_get("integration_account_id")?,
        object_id: row.try_get("object_id")?,
        remote_id: row.try_get("remote_id")?,
        remote_type: row.try_get("remote_type")?,
        state: row.try_get("state")?,
        authority_mode: row.try_get("authority_mode")?,
        remote_version: row.try_get("remote_version")?,
        metadata_json: serde_json::from_str(&row.try_get::<String, _>("metadata_json")?)?,
        linked_at: timestamp_to_datetime(row.try_get("linked_at")?)?,
        last_seen_at: timestamp_to_datetime(row.try_get("last_seen_at")?)?,
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

    async fn seed_account(pool: &SqlitePool) {
        sqlx::query(
            r#"
            INSERT INTO integration_accounts (
                id, provider, display_name, external_account_ref, auth_state, policy_profile,
                activation_state, sync_posture, metadata_json, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind("integration_account_01gcal")
        .bind("google-calendar")
        .bind("Primary Calendar")
        .bind("acct_gcal")
        .bind("authorized")
        .bind("conservative_sync")
        .bind("active")
        .bind("bounded_window")
        .bind("{}")
        .bind(OffsetDateTime::now_utc().unix_timestamp())
        .bind(OffsetDateTime::now_utc().unix_timestamp())
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn sync_links_repo_persists_lifecycle_states() {
        let pool = test_pool().await;
        seed_account(&pool).await;
        let now = OffsetDateTime::now_utc();
        let record = SyncLinkRecord {
            id: "sync_link_01gcal".to_string(),
            provider: "google-calendar".to_string(),
            integration_account_id: "integration_account_01gcal".to_string(),
            object_id: "event_01gcal".to_string(),
            remote_id: "evt_remote_1".to_string(),
            remote_type: "event".to_string(),
            state: "deleted_upstream".to_string(),
            authority_mode: "source_preferred".to_string(),
            remote_version: Some("etag_1".to_string()),
            metadata_json: json!({"window":"past 90 days / future 365 days"}),
            linked_at: now,
            last_seen_at: now,
        };

        upsert_sync_link(&pool, &record).await.unwrap();
        update_sync_link_state(&pool, &record.id, "reconciled")
            .await
            .unwrap();

        let mut listed = list_sync_links_for_object(&pool, "event_01gcal")
            .await
            .unwrap();
        let stored = listed.pop().expect("sync link should exist");
        assert_eq!(stored.provider, "google-calendar");
        assert_eq!(stored.state, "reconciled");

        update_sync_link_state(&pool, &record.id, "conflicted")
            .await
            .unwrap();
        let stored = list_sync_links_for_object(&pool, "event_01gcal")
            .await
            .unwrap()
            .pop()
            .expect("sync link should still exist");
        assert_eq!(stored.state, "conflicted");
    }

    #[tokio::test]
    async fn sync_links_repo_allows_historical_states() {
        let pool = test_pool().await;
        seed_account(&pool).await;
        let now = OffsetDateTime::now_utc();

        let record = SyncLinkRecord {
            id: "sync_link_01todo".to_string(),
            provider: "todoist".to_string(),
            integration_account_id: "integration_account_01gcal".to_string(),
            object_id: "task_01todo".to_string(),
            remote_id: "todo_remote_1".to_string(),
            remote_type: "task".to_string(),
            state: "superseded".to_string(),
            authority_mode: "shared".to_string(),
            remote_version: None,
            metadata_json: json!({"reason":"merge"}),
            linked_at: now,
            last_seen_at: now,
        };

        upsert_sync_link(&pool, &record).await.unwrap();
        let stored = list_sync_links_for_object(&pool, "task_01todo")
            .await
            .unwrap()
            .pop()
            .expect("historical sync link should exist");
        assert_eq!(stored.state, "superseded");
    }
}
