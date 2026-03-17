use crate::{
    db::StorageError,
    mapping::{parse_json_value, timestamp_to_datetime},
};
use sqlx::{Row, SqlitePool};
use vel_core::{Ref, Run, RunEvent, RunId};

pub(crate) async fn create_ref(pool: &SqlitePool, ref_: &Ref) -> Result<(), StorageError> {
    let now = ref_.created_at.unix_timestamp();
    sqlx::query(
        r#"
        INSERT INTO refs (ref_id, from_type, from_id, to_type, to_id, relation_type, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&ref_.id)
    .bind(&ref_.from_type)
    .bind(&ref_.from_id)
    .bind(&ref_.to_type)
    .bind(&ref_.to_id)
    .bind(ref_.relation_type.to_string())
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn list_refs_from(
    pool: &SqlitePool,
    from_type: &str,
    from_id: &str,
) -> Result<Vec<Ref>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT ref_id, from_type, from_id, to_type, to_id, relation_type, created_at
        FROM refs WHERE from_type = ? AND from_id = ? ORDER BY created_at ASC
        "#,
    )
    .bind(from_type)
    .bind(from_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(map_ref_row)
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn list_refs_to(
    pool: &SqlitePool,
    to_type: &str,
    to_id: &str,
) -> Result<Vec<Ref>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT ref_id, from_type, from_id, to_type, to_id, relation_type, created_at
        FROM refs WHERE to_type = ? AND to_id = ? ORDER BY created_at ASC
        "#,
    )
    .bind(to_type)
    .bind(to_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(map_ref_row)
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) fn map_run_row(row: &sqlx::sqlite::SqliteRow) -> Result<Run, StorageError> {
    let kind: String = row.try_get("run_kind")?;
    let status: String = row.try_get("status")?;
    let input_str: String = row.try_get("input_json")?;
    let output_str: Option<String> = row.try_get("output_json")?;
    let error_str: Option<String> = row.try_get("error_json")?;
    Ok(Run {
        id: RunId::from(row.try_get::<String, _>("run_id")?),
        kind: kind
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        status: status
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        input_json: parse_json_value(&input_str)?,
        output_json: output_str.as_deref().map(parse_json_value).transpose()?,
        error_json: error_str.as_deref().map(parse_json_value).transpose()?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        started_at: row
            .try_get::<Option<i64>, _>("started_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        finished_at: row
            .try_get::<Option<i64>, _>("finished_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
    })
}

pub(crate) fn map_run_event_row(row: sqlx::sqlite::SqliteRow) -> Result<RunEvent, StorageError> {
    let event_type: String = row.try_get("event_type")?;
    let payload_str: String = row.try_get("payload_json")?;
    Ok(RunEvent {
        id: row.try_get("event_id")?,
        run_id: RunId::from(row.try_get::<String, _>("run_id")?),
        seq: row.try_get::<i64, _>("seq")? as u32,
        event_type: event_type
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        payload_json: parse_json_value(&payload_str)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

pub(crate) fn map_ref_row(row: sqlx::sqlite::SqliteRow) -> Result<Ref, StorageError> {
    let relation_type: String = row.try_get("relation_type")?;
    Ok(Ref {
        id: row.try_get("ref_id")?,
        from_type: row.try_get("from_type")?,
        from_id: row.try_get("from_id")?,
        to_type: row.try_get("to_type")?,
        to_id: row.try_get("to_id")?,
        relation_type: relation_type
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}
