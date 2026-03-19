use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use vel_core::{OrderingStamp, WritebackOperationRecord, WritebackStatus, WritebackTargetRef};

use crate::{
    db::StorageError,
    mapping::{parse_json_value, timestamp_to_datetime},
};

pub(crate) async fn insert_writeback_operation(
    pool: &SqlitePool,
    record: &WritebackOperationRecord,
    ordering_stamp: &OrderingStamp,
) -> Result<WritebackOperationRecord, StorageError> {
    sqlx::query(
        r#"
        INSERT INTO writeback_operations (
            id,
            kind,
            risk,
            status,
            family,
            provider_key,
            project_id,
            connection_id,
            external_id,
            requested_payload_json,
            result_payload_json,
            provenance_json,
            conflict_case_id,
            requested_by_node_id,
            ordering_stamp_json,
            requested_at,
            applied_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(record.id.as_ref())
    .bind(record.kind.to_string())
    .bind(record.risk.to_string())
    .bind(record.status.to_string())
    .bind(record.target.family.to_string())
    .bind(&record.target.provider_key)
    .bind(
        record
            .target
            .project_id
            .as_ref()
            .map(|value| value.as_ref()),
    )
    .bind(
        record
            .target
            .connection_id
            .as_ref()
            .map(|value| value.as_ref()),
    )
    .bind(&record.target.external_id)
    .bind(serde_json::to_string(&record.requested_payload)?)
    .bind(json_string_or_default(record.result_payload.as_ref())?)
    .bind(provenance_json(&record.provenance)?)
    .bind(&record.conflict_case_id)
    .bind(&record.requested_by_node_id)
    .bind(serde_json::to_string(ordering_stamp)?)
    .bind(record.requested_at.unix_timestamp())
    .bind(record.applied_at.map(|value| value.unix_timestamp()))
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(record.clone())
}

pub(crate) async fn get_writeback_operation(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<WritebackOperationRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT id, kind, risk, status, family, provider_key, project_id, connection_id, external_id,
               requested_payload_json, result_payload_json, provenance_json, conflict_case_id,
               requested_by_node_id, requested_at, applied_at, updated_at
        FROM writeback_operations
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_writeback_operation_row).transpose()
}

pub(crate) async fn list_writeback_operations(
    pool: &SqlitePool,
    status_filter: Option<WritebackStatus>,
    limit: u32,
) -> Result<Vec<WritebackOperationRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT id, kind, risk, status, family, provider_key, project_id, connection_id, external_id,
               requested_payload_json, result_payload_json, provenance_json, conflict_case_id,
               requested_by_node_id, requested_at, applied_at, updated_at
        FROM writeback_operations
        WHERE (? IS NULL OR status = ?)
        ORDER BY updated_at DESC, requested_at DESC, id ASC
        LIMIT ?
        "#,
    )
    .bind(status_filter.map(|value| value.to_string()))
    .bind(status_filter.map(|value| value.to_string()))
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_writeback_operation_row).collect()
}

pub(crate) async fn update_writeback_operation(
    pool: &SqlitePool,
    record: &WritebackOperationRecord,
) -> Result<WritebackOperationRecord, StorageError> {
    sqlx::query(
        r#"
        UPDATE writeback_operations
        SET status = ?,
            result_payload_json = ?,
            conflict_case_id = ?,
            applied_at = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(record.status.to_string())
    .bind(json_string_or_default(record.result_payload.as_ref())?)
    .bind(&record.conflict_case_id)
    .bind(record.applied_at.map(|value| value.unix_timestamp()))
    .bind(record.updated_at.unix_timestamp())
    .bind(record.id.as_ref())
    .execute(pool)
    .await?;

    Ok(record.clone())
}

fn provenance_json(values: &[vel_core::IntegrationSourceRef]) -> Result<String, StorageError> {
    if values.is_empty() {
        return Ok("[]".to_string());
    }
    serde_json::to_string(values).map_err(Into::into)
}

fn json_string_or_default(value: Option<&JsonValue>) -> Result<String, StorageError> {
    match value {
        Some(value) => serde_json::to_string(value).map_err(Into::into),
        None => Ok("{}".to_string()),
    }
}

fn optional_json_from_default(raw: String) -> Result<Option<JsonValue>, StorageError> {
    let value = parse_json_value(&raw)?;
    Ok(match value {
        JsonValue::Object(ref object) if object.is_empty() => None,
        JsonValue::Null => None,
        other => Some(other),
    })
}

fn map_writeback_operation_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<WritebackOperationRecord, StorageError> {
    let target = WritebackTargetRef {
        family: row
            .try_get::<String, _>("family")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        provider_key: row.try_get("provider_key")?,
        project_id: row
            .try_get::<Option<String>, _>("project_id")?
            .map(Into::into),
        connection_id: row
            .try_get::<Option<String>, _>("connection_id")?
            .map(Into::into),
        external_id: row.try_get("external_id")?,
    };
    let provenance_json: String = row.try_get("provenance_json")?;
    let provenance = match parse_json_value(&provenance_json)? {
        JsonValue::Object(ref object) if object.is_empty() => Vec::new(),
        other => serde_json::from_value(other)?,
    };

    Ok(WritebackOperationRecord {
        id: row.try_get::<String, _>("id")?.into(),
        kind: row
            .try_get::<String, _>("kind")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        risk: row
            .try_get::<String, _>("risk")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        status: row
            .try_get::<String, _>("status")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        target,
        requested_payload: parse_json_value(&row.try_get::<String, _>("requested_payload_json")?)?,
        result_payload: optional_json_from_default(row.try_get("result_payload_json")?)?,
        provenance,
        conflict_case_id: row.try_get("conflict_case_id")?,
        requested_by_node_id: row.try_get("requested_by_node_id")?,
        requested_at: timestamp_to_datetime(row.try_get("requested_at")?)?,
        applied_at: row
            .try_get::<Option<i64>, _>("applied_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        updated_at: timestamp_to_datetime(row.try_get("updated_at")?)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{migrate::Migrator, SqlitePool};
    use time::OffsetDateTime;
    use vel_core::{
        IntegrationFamily, NodeIdentity, OrderingStamp, WritebackOperationKind, WritebackRisk,
    };

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_record() -> (WritebackOperationRecord, OrderingStamp) {
        let requested_at = OffsetDateTime::now_utc();
        (
            WritebackOperationRecord {
                id: "wb_repo_01".to_string().into(),
                kind: WritebackOperationKind::TodoistCreateTask,
                risk: WritebackRisk::Safe,
                status: WritebackStatus::Queued,
                target: WritebackTargetRef {
                    family: IntegrationFamily::Tasks,
                    provider_key: "todoist".to_string(),
                    project_id: Some("proj_repo".to_string().into()),
                    connection_id: Some("icn_repo".to_string().into()),
                    external_id: Some("todo_1".to_string()),
                },
                requested_payload: serde_json::json!({"content": "ship slice"}),
                result_payload: None,
                provenance: vec![],
                conflict_case_id: None,
                requested_by_node_id: "node-alpha".to_string(),
                requested_at,
                applied_at: None,
                updated_at: requested_at,
            },
            OrderingStamp::new(
                requested_at.unix_timestamp(),
                1,
                NodeIdentity::from("123e4567-e89b-12d3-a456-426614174000".to_string()),
            ),
        )
    }

    #[tokio::test]
    async fn writeback_operations_repo_inserts_lists_and_updates() {
        let pool = test_pool().await;
        let (record, ordering) = sample_record();
        insert_writeback_operation(&pool, &record, &ordering)
            .await
            .unwrap();

        let listed = list_writeback_operations(&pool, Some(WritebackStatus::Queued), 10)
            .await
            .unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, record.id);

        let mut applied = listed[0].clone();
        applied.status = WritebackStatus::Applied;
        applied.result_payload = Some(serde_json::json!({"external_id": "todo_1"}));
        applied.applied_at = Some(applied.updated_at);
        update_writeback_operation(&pool, &applied).await.unwrap();

        let stored = get_writeback_operation(&pool, record.id.as_ref())
            .await
            .unwrap()
            .expect("updated record should exist");
        assert_eq!(stored.status, WritebackStatus::Applied);
        assert!(stored.result_payload.is_some());
    }
}
