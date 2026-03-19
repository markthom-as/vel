use serde_json::Value as JsonValue;
use sqlx::{Row, SqlitePool};
use vel_core::{ConflictCaseRecord, WritebackTargetRef};

use crate::{
    db::StorageError,
    mapping::{parse_json_value, timestamp_to_datetime},
};

pub(crate) async fn insert_conflict_case(
    pool: &SqlitePool,
    record: &ConflictCaseRecord,
) -> Result<ConflictCaseRecord, StorageError> {
    sqlx::query(
        r#"
        INSERT INTO conflict_cases (
            id,
            kind,
            status,
            family,
            provider_key,
            project_id,
            connection_id,
            external_id,
            summary,
            local_payload_json,
            upstream_payload_json,
            resolution_payload_json,
            opened_at,
            resolved_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(record.id.as_ref())
    .bind(record.kind.to_string())
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
    .bind(&record.summary)
    .bind(serde_json::to_string(&record.local_payload)?)
    .bind(json_string_or_default(record.upstream_payload.as_ref())?)
    .bind(json_string_or_default(record.resolution_payload.as_ref())?)
    .bind(record.opened_at.unix_timestamp())
    .bind(record.resolved_at.map(|value| value.unix_timestamp()))
    .bind(record.updated_at.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(record.clone())
}

pub(crate) async fn get_conflict_case(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<ConflictCaseRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT id, kind, status, family, provider_key, project_id, connection_id, external_id,
               summary, local_payload_json, upstream_payload_json, resolution_payload_json,
               opened_at, resolved_at, updated_at
        FROM conflict_cases
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_conflict_case_row).transpose()
}

pub(crate) async fn list_open_conflict_cases(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<ConflictCaseRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT id, kind, status, family, provider_key, project_id, connection_id, external_id,
               summary, local_payload_json, upstream_payload_json, resolution_payload_json,
               opened_at, resolved_at, updated_at
        FROM conflict_cases
        WHERE status IN ('open', 'acknowledged')
        ORDER BY updated_at DESC, opened_at DESC, id ASC
        LIMIT ?
        "#,
    )
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_conflict_case_row).collect()
}

pub(crate) async fn update_conflict_case(
    pool: &SqlitePool,
    record: &ConflictCaseRecord,
) -> Result<ConflictCaseRecord, StorageError> {
    sqlx::query(
        r#"
        UPDATE conflict_cases
        SET status = ?,
            resolution_payload_json = ?,
            resolved_at = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(record.status.to_string())
    .bind(json_string_or_default(record.resolution_payload.as_ref())?)
    .bind(record.resolved_at.map(|value| value.unix_timestamp()))
    .bind(record.updated_at.unix_timestamp())
    .bind(record.id.as_ref())
    .execute(pool)
    .await?;

    Ok(record.clone())
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

fn map_conflict_case_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<ConflictCaseRecord, StorageError> {
    Ok(ConflictCaseRecord {
        id: row.try_get::<String, _>("id")?.into(),
        kind: row
            .try_get::<String, _>("kind")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        status: row
            .try_get::<String, _>("status")?
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        target: WritebackTargetRef {
            family: row.try_get::<String, _>("family")?.parse().map_err(
                |error: vel_core::VelCoreError| StorageError::Validation(error.to_string()),
            )?,
            provider_key: row.try_get("provider_key")?,
            project_id: row
                .try_get::<Option<String>, _>("project_id")?
                .map(Into::into),
            connection_id: row
                .try_get::<Option<String>, _>("connection_id")?
                .map(Into::into),
            external_id: row.try_get("external_id")?,
        },
        summary: row.try_get("summary")?,
        local_payload: parse_json_value(&row.try_get::<String, _>("local_payload_json")?)?,
        upstream_payload: optional_json_from_default(row.try_get("upstream_payload_json")?)?,
        resolution_payload: optional_json_from_default(row.try_get("resolution_payload_json")?)?,
        opened_at: timestamp_to_datetime(row.try_get("opened_at")?)?,
        resolved_at: row
            .try_get::<Option<i64>, _>("resolved_at")?
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
    use vel_core::{ConflictCaseKind, ConflictCaseStatus, IntegrationFamily};

    static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_record() -> ConflictCaseRecord {
        let now = OffsetDateTime::now_utc();
        ConflictCaseRecord {
            id: "conf_repo_01".to_string().into(),
            kind: ConflictCaseKind::UpstreamVsLocal,
            status: ConflictCaseStatus::Open,
            target: WritebackTargetRef {
                family: IntegrationFamily::Tasks,
                provider_key: "todoist".to_string(),
                project_id: Some("proj_repo".to_string().into()),
                connection_id: Some("icn_repo".to_string().into()),
                external_id: Some("todo_1".to_string()),
            },
            summary: "upstream differs".to_string(),
            local_payload: serde_json::json!({"content": "local"}),
            upstream_payload: Some(serde_json::json!({"content": "upstream"})),
            resolution_payload: None,
            opened_at: now,
            resolved_at: None,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn conflict_cases_repo_inserts_lists_and_updates() {
        let pool = test_pool().await;
        let mut record = sample_record();
        insert_conflict_case(&pool, &record).await.unwrap();

        let listed = list_open_conflict_cases(&pool, 10).await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, record.id);

        record.status = ConflictCaseStatus::Resolved;
        record.resolution_payload = Some(serde_json::json!({"choice": "local"}));
        record.resolved_at = Some(record.updated_at);
        update_conflict_case(&pool, &record).await.unwrap();

        let stored = get_conflict_case(&pool, record.id.as_ref())
            .await
            .unwrap()
            .expect("conflict should exist");
        assert_eq!(stored.status, ConflictCaseStatus::Resolved);
        assert!(stored.resolution_payload.is_some());
    }
}
