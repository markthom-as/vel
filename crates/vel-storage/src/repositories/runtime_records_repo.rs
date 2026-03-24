use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeRecord {
    pub id: String,
    pub record_type: String,
    pub object_ref: Option<String>,
    pub status: String,
    pub payload_json: JsonValue,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn insert_runtime_record(
    pool: &SqlitePool,
    record: &RuntimeRecord,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO runtime_records (
            id,
            record_type,
            object_ref,
            status,
            payload_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&record.id)
    .bind(&record.record_type)
    .bind(&record.object_ref)
    .bind(&record.status)
    .bind(serde_json::to_string(&record.payload_json)?)
    .bind(record.created_at.unix_timestamp())
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_runtime_records(
    pool: &SqlitePool,
    record_type: &str,
) -> Result<Vec<RuntimeRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            record_type,
            object_ref,
            status,
            payload_json,
            created_at,
            updated_at
        FROM runtime_records
        WHERE record_type = ?
        ORDER BY created_at ASC, id ASC
        "#,
    )
    .bind(record_type)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_runtime_record_row).collect()
}

fn map_runtime_record_row(row: &sqlx::sqlite::SqliteRow) -> Result<RuntimeRecord, StorageError> {
    Ok(RuntimeRecord {
        id: row.try_get("id")?,
        record_type: row.try_get("record_type")?,
        object_ref: row.try_get("object_ref")?,
        status: row.try_get("status")?,
        payload_json: serde_json::from_str(&row.try_get::<String, _>("payload_json")?)?,
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
    async fn runtime_records_repo_persists_write_intent_approval_and_run_records() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();

        for (id, record_type, status) in [
            ("runtime_01_write", "write_intent", "proposed"),
            ("runtime_01_approval", "approval", "pending"),
            ("runtime_01_run", "run", "running"),
        ] {
            insert_runtime_record(
                &pool,
                &RuntimeRecord {
                    id: id.to_string(),
                    record_type: record_type.to_string(),
                    object_ref: Some("task_01phase58".to_string()),
                    status: status.to_string(),
                    payload_json: json!({"kind": record_type}),
                    created_at: now,
                    updated_at: now,
                },
            )
            .await
            .unwrap();
        }

        let write_intents = list_runtime_records(&pool, "write_intent").await.unwrap();
        assert_eq!(write_intents.len(), 1);
        assert_eq!(write_intents[0].status, "proposed");
    }
}
