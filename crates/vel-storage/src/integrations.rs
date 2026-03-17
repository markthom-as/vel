use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
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
    .execute(pool)
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
    let current = get_integration_connection(pool, id)
        .await?
        .ok_or_else(|| StorageError::Validation("integration connection not found".to_string()))?;
    let next_status = status.unwrap_or(current.status);
    let next_display_name = display_name.unwrap_or(current.display_name.as_str());
    let next_account_ref = account_ref
        .map(|value| value.map(ToOwned::to_owned))
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
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn upsert_integration_connection_setting_ref(
    pool: &SqlitePool,
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
    .execute(pool)
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
    .execute(pool)
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
