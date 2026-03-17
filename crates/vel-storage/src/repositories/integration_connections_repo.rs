use serde_json::Value as JsonValue;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{
    IntegrationConnection, IntegrationConnectionEvent, IntegrationConnectionEventType,
    IntegrationConnectionId, IntegrationConnectionSettingRef, IntegrationConnectionStatus,
    IntegrationProvider,
};

use crate::{
    db::{IntegrationConnectionFilters, IntegrationConnectionInsert, StorageError},
    mapping::{parse_json_value, timestamp_to_datetime},
};

pub(crate) async fn insert_integration_connection(
    pool: &SqlitePool,
    input: IntegrationConnectionInsert,
) -> Result<IntegrationConnectionId, StorageError> {
    let mut tx = pool.begin().await?;
    let connection_id = insert_integration_connection_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(connection_id)
}

pub(crate) async fn insert_integration_connection_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &IntegrationConnectionInsert,
) -> Result<IntegrationConnectionId, StorageError> {
    if input.provider.family != input.family {
        return Err(StorageError::Validation(
            "integration provider family does not match connection family".to_string(),
        ));
    }

    let id = IntegrationConnectionId::new();
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let metadata_json = serde_json::to_string(&input.metadata_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;

    sqlx::query(
        r#"
            INSERT INTO integration_connections (
                id,
                family,
                provider_key,
                status,
                display_name,
                account_ref,
                metadata_json,
                created_at,
                updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
    )
    .bind(id.as_ref())
    .bind(input.family.to_string())
    .bind(&input.provider.key)
    .bind(input.status.to_string())
    .bind(&input.display_name)
    .bind(&input.account_ref)
    .bind(metadata_json)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(id)
}

pub(crate) async fn get_integration_connection(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<IntegrationConnection>, StorageError> {
    let row = sqlx::query(
        r#"
            SELECT id, family, provider_key, status, display_name, account_ref, metadata_json, created_at, updated_at
            FROM integration_connections
            WHERE id = ?
            "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.map(|row| map_integration_connection_row(&row))
        .transpose()
}

pub(crate) async fn get_integration_connection_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<Option<IntegrationConnection>, StorageError> {
    let row = sqlx::query(
        r#"
            SELECT id, family, provider_key, status, display_name, account_ref, metadata_json, created_at, updated_at
            FROM integration_connections
            WHERE id = ?
            "#,
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .await?;

    row.map(|row| map_integration_connection_row(&row))
        .transpose()
}

pub(crate) async fn list_integration_connections(
    pool: &SqlitePool,
    filters: IntegrationConnectionFilters,
) -> Result<Vec<IntegrationConnection>, StorageError> {
    let rows = sqlx::query(
        r#"
            SELECT id, family, provider_key, status, display_name, account_ref, metadata_json, created_at, updated_at
            FROM integration_connections
            WHERE (? IS NULL OR family = ?)
              AND (? IS NULL OR provider_key = ?)
              AND (? = 1 OR status != 'disabled')
            ORDER BY family ASC, provider_key ASC, created_at ASC
            "#,
    )
    .bind(filters.family.map(|family| family.to_string()))
    .bind(filters.family.map(|family| family.to_string()))
    .bind(filters.provider_key.as_deref())
    .bind(filters.provider_key.as_deref())
    .bind(if filters.include_disabled { 1_i64 } else { 0_i64 })
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_integration_connection_row(&row))
        .collect()
}

pub(crate) async fn update_integration_connection(
    pool: &SqlitePool,
    id: &str,
    status: Option<IntegrationConnectionStatus>,
    display_name: Option<&str>,
    account_ref: Option<Option<&str>>,
    metadata_json: Option<&JsonValue>,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    update_integration_connection_in_tx(
        &mut tx,
        id,
        status,
        display_name,
        account_ref,
        metadata_json,
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn update_integration_connection_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    status: Option<IntegrationConnectionStatus>,
    display_name: Option<&str>,
    account_ref: Option<Option<&str>>,
    metadata_json: Option<&JsonValue>,
) -> Result<(), StorageError> {
    let current = get_integration_connection_in_tx(tx, id)
        .await?
        .ok_or_else(|| StorageError::Validation("integration connection not found".to_string()))?;

    let next_status = status.unwrap_or(current.status);
    let next_display_name = display_name.unwrap_or(current.display_name.as_str());
    let next_account_ref = account_ref
        .map(|value| value.map(|value| value.to_owned()))
        .unwrap_or(current.account_ref);
    let next_metadata_json = metadata_json.cloned().unwrap_or(current.metadata_json);
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let metadata_json = serde_json::to_string(&next_metadata_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;

    sqlx::query(
        r#"
            UPDATE integration_connections
            SET status = ?, display_name = ?, account_ref = ?, metadata_json = ?, updated_at = ?
            WHERE id = ?
            "#,
    )
    .bind(next_status.to_string())
    .bind(next_display_name)
    .bind(next_account_ref)
    .bind(metadata_json)
    .bind(now)
    .bind(id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub(crate) async fn upsert_integration_connection_setting_ref(
    pool: &SqlitePool,
    connection_id: &str,
    setting_key: &str,
    setting_value: &str,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    upsert_integration_connection_setting_ref_in_tx(
        &mut tx,
        connection_id,
        setting_key,
        setting_value,
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn upsert_integration_connection_setting_ref_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    connection_id: &str,
    setting_key: &str,
    setting_value: &str,
) -> Result<(), StorageError> {
    let id = format!("icsr_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();

    sqlx::query(
        r#"
            INSERT INTO integration_connection_setting_refs (
                id,
                connection_id,
                setting_key,
                setting_value,
                created_at
            )
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(connection_id, setting_key) DO UPDATE SET setting_value = excluded.setting_value
            "#,
    )
    .bind(id)
    .bind(connection_id)
    .bind(setting_key)
    .bind(setting_value)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub(crate) async fn list_integration_connection_setting_refs(
    pool: &SqlitePool,
    connection_id: &str,
) -> Result<Vec<IntegrationConnectionSettingRef>, StorageError> {
    let rows = sqlx::query(
        r#"
            SELECT connection_id, setting_key, setting_value, created_at
            FROM integration_connection_setting_refs
            WHERE connection_id = ?
            ORDER BY setting_key ASC
            "#,
    )
    .bind(connection_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_integration_connection_setting_ref_row(&row))
        .collect()
}

pub(crate) async fn insert_integration_connection_event(
    pool: &SqlitePool,
    connection_id: &str,
    event_type: IntegrationConnectionEventType,
    payload_json: &JsonValue,
    timestamp: i64,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let event_id = insert_integration_connection_event_in_tx(
        &mut tx,
        connection_id,
        event_type,
        payload_json,
        timestamp,
    )
    .await?;
    tx.commit().await?;
    Ok(event_id)
}

pub(crate) async fn insert_integration_connection_event_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    connection_id: &str,
    event_type: IntegrationConnectionEventType,
    payload_json: &JsonValue,
    timestamp: i64,
) -> Result<String, StorageError> {
    let id = format!("icev_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let payload_json = serde_json::to_string(payload_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;

    sqlx::query(
        r#"
            INSERT INTO integration_connection_events (
                id,
                connection_id,
                event_type,
                payload_json,
                timestamp,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
    )
    .bind(&id)
    .bind(connection_id)
    .bind(event_type.to_string())
    .bind(payload_json)
    .bind(timestamp)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(id)
}

pub(crate) async fn list_integration_connection_events(
    pool: &SqlitePool,
    connection_id: &str,
    limit: u32,
) -> Result<Vec<IntegrationConnectionEvent>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = sqlx::query(
        r#"
            SELECT id, connection_id, event_type, payload_json, timestamp, created_at
            FROM integration_connection_events
            WHERE connection_id = ?
            ORDER BY timestamp DESC, created_at DESC
            LIMIT ?
            "#,
    )
    .bind(connection_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_integration_connection_event_row(&row))
        .collect()
}

fn map_integration_connection_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<IntegrationConnection, StorageError> {
    let family = row.try_get::<String, _>("family")?;
    let provider_key = row.try_get::<String, _>("provider_key")?;
    let status = row.try_get::<String, _>("status")?;
    let metadata_json = row.try_get::<String, _>("metadata_json")?;

    Ok(IntegrationConnection {
        id: IntegrationConnectionId::from(row.try_get::<String, _>("id")?),
        provider: IntegrationProvider::new(
            family.parse().map_err(|error: vel_core::VelCoreError| {
                StorageError::Validation(error.to_string())
            })?,
            provider_key,
        )
        .map_err(|error| StorageError::Validation(error.to_string()))?,
        status: status
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        display_name: row.try_get("display_name")?,
        account_ref: row.try_get("account_ref")?,
        metadata_json: parse_json_value(&metadata_json)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        updated_at: timestamp_to_datetime(row.try_get("updated_at")?)?,
    })
}

fn map_integration_connection_setting_ref_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<IntegrationConnectionSettingRef, StorageError> {
    Ok(IntegrationConnectionSettingRef {
        connection_id: IntegrationConnectionId::from(row.try_get::<String, _>("connection_id")?),
        setting_key: row.try_get("setting_key")?,
        setting_value: row.try_get("setting_value")?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

fn map_integration_connection_event_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<IntegrationConnectionEvent, StorageError> {
    let event_type = row.try_get::<String, _>("event_type")?;
    let payload_json = row.try_get::<String, _>("payload_json")?;

    Ok(IntegrationConnectionEvent {
        id: row.try_get("id")?,
        connection_id: IntegrationConnectionId::from(row.try_get::<String, _>("connection_id")?),
        event_type: event_type
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        payload_json: parse_json_value(&payload_json)?,
        timestamp: timestamp_to_datetime(row.try_get("timestamp")?)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use vel_core::{IntegrationFamily, IntegrationProvider};

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_connection_input() -> IntegrationConnectionInsert {
        IntegrationConnectionInsert {
            family: IntegrationFamily::Messaging,
            provider: IntegrationProvider::new(IntegrationFamily::Messaging, "signal").unwrap(),
            status: IntegrationConnectionStatus::Pending,
            display_name: "Signal personal".to_string(),
            account_ref: Some("+15555550123".to_string()),
            metadata_json: json!({ "scope": "personal" }),
        }
    }

    #[tokio::test]
    async fn integration_connection_write_helpers_roll_back_inside_transaction() {
        let pool = test_pool().await;
        let mut tx = pool.begin().await.unwrap();

        let connection_id =
            insert_integration_connection_in_tx(&mut tx, &sample_connection_input())
                .await
                .unwrap();

        let fetched = get_integration_connection_in_tx(&mut tx, connection_id.as_ref())
            .await
            .unwrap()
            .expect("connection should be visible inside transaction");
        assert_eq!(fetched.display_name, "Signal personal");

        update_integration_connection_in_tx(
            &mut tx,
            connection_id.as_ref(),
            Some(IntegrationConnectionStatus::Connected),
            Some("Signal primary"),
            Some(Some("signal:primary")),
            Some(&json!({ "scope": "primary" })),
        )
        .await
        .unwrap();

        let updated = get_integration_connection_in_tx(&mut tx, connection_id.as_ref())
            .await
            .unwrap()
            .expect("connection should still be visible inside transaction");
        assert_eq!(updated.status, IntegrationConnectionStatus::Connected);
        assert_eq!(updated.display_name, "Signal primary");
        assert_eq!(updated.account_ref.as_deref(), Some("signal:primary"));
        assert_eq!(updated.metadata_json["scope"], "primary");

        upsert_integration_connection_setting_ref_in_tx(
            &mut tx,
            connection_id.as_ref(),
            "messaging_snapshot_path",
            "/tmp/signal.json",
        )
        .await
        .unwrap();

        let event_id = insert_integration_connection_event_in_tx(
            &mut tx,
            connection_id.as_ref(),
            IntegrationConnectionEventType::SyncStarted,
            &json!({ "job": "manual" }),
            1_700_000_100,
        )
        .await
        .unwrap();
        assert!(event_id.starts_with("icev_"));

        tx.rollback().await.unwrap();

        assert!(get_integration_connection(&pool, connection_id.as_ref())
            .await
            .unwrap()
            .is_none());
        assert!(
            list_integration_connection_setting_refs(&pool, connection_id.as_ref())
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            list_integration_connection_events(&pool, connection_id.as_ref(), 10)
                .await
                .unwrap()
                .is_empty()
        );
    }
}
