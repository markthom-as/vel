use sqlx::{Row, SqlitePool};

use crate::{
    db::{StorageError, UpstreamObjectRefRecord},
    mapping::{parse_json_value, timestamp_to_datetime},
};

pub(crate) async fn upsert_upstream_object_ref(
    pool: &SqlitePool,
    record: &UpstreamObjectRefRecord,
) -> Result<UpstreamObjectRefRecord, StorageError> {
    sqlx::query(
        r#"
        INSERT INTO upstream_object_refs (
            id,
            family,
            provider_key,
            project_id,
            local_object_kind,
            local_object_id,
            external_id,
            external_parent_id,
            ordering_stamp_json,
            last_seen_at,
            metadata_json
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(family, provider_key, local_object_kind, local_object_id) DO UPDATE SET
            external_id = excluded.external_id,
            external_parent_id = excluded.external_parent_id,
            ordering_stamp_json = excluded.ordering_stamp_json,
            last_seen_at = excluded.last_seen_at,
            metadata_json = excluded.metadata_json
        "#,
    )
    .bind(&record.id)
    .bind(record.family.to_string())
    .bind(&record.provider_key)
    .bind(record.project_id.as_ref().map(|value| value.as_ref()))
    .bind(&record.local_object_kind)
    .bind(&record.local_object_id)
    .bind(&record.external_id)
    .bind(&record.external_parent_id)
    .bind(serde_json::to_string(&record.ordering_stamp)?)
    .bind(record.last_seen_at.unix_timestamp())
    .bind(serde_json::to_string(&record.metadata_json)?)
    .execute(pool)
    .await?;

    Ok(record.clone())
}

pub(crate) async fn get_upstream_object_ref(
    pool: &SqlitePool,
    local_object_kind: &str,
    local_object_id: &str,
) -> Result<Option<UpstreamObjectRefRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT id, family, provider_key, project_id, local_object_kind, local_object_id,
               external_id, external_parent_id, ordering_stamp_json, last_seen_at, metadata_json
        FROM upstream_object_refs
        WHERE local_object_kind = ? AND local_object_id = ?
        "#,
    )
    .bind(local_object_kind)
    .bind(local_object_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_upstream_ref_row).transpose()
}

fn map_upstream_ref_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<UpstreamObjectRefRecord, StorageError> {
    Ok(UpstreamObjectRefRecord {
        id: row.try_get("id")?,
        family: row
            .try_get::<String, _>("family")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        provider_key: row.try_get("provider_key")?,
        project_id: row
            .try_get::<Option<String>, _>("project_id")?
            .map(Into::into),
        local_object_kind: row.try_get("local_object_kind")?,
        local_object_id: row.try_get("local_object_id")?,
        external_id: row.try_get("external_id")?,
        external_parent_id: row.try_get("external_parent_id")?,
        ordering_stamp: serde_json::from_str(&row.try_get::<String, _>("ordering_stamp_json")?)?,
        last_seen_at: timestamp_to_datetime(row.try_get("last_seen_at")?)?,
        metadata_json: parse_json_value(&row.try_get::<String, _>("metadata_json")?)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{migrate::Migrator, SqlitePool};
    use time::OffsetDateTime;
    use vel_core::{IntegrationFamily, NodeIdentity, OrderingStamp};

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn upstream_refs_repo_upserts_and_gets_records() {
        let pool = test_pool().await;
        let record = UpstreamObjectRefRecord {
            id: "uor_1".to_string(),
            family: IntegrationFamily::Tasks,
            provider_key: "todoist".to_string(),
            project_id: Some("proj_repo".to_string().into()),
            local_object_kind: "commitment".to_string(),
            local_object_id: "cmt_1".to_string(),
            external_id: "todo_1".to_string(),
            external_parent_id: Some("proj_todo".to_string()),
            ordering_stamp: OrderingStamp::new(
                1_710_000_000,
                2,
                NodeIdentity::from("123e4567-e89b-12d3-a456-426614174000".to_string()),
            ),
            last_seen_at: OffsetDateTime::now_utc(),
            metadata_json: serde_json::json!({"source": "todoist"}),
        };

        upsert_upstream_object_ref(&pool, &record).await.unwrap();

        let stored = get_upstream_object_ref(&pool, "commitment", "cmt_1")
            .await
            .unwrap()
            .expect("upstream ref should exist");
        assert_eq!(stored.external_id, "todo_1");
        assert_eq!(stored.provider_key, "todoist");
    }
}
