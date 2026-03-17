use crate::{
    db::StorageError,
    mapping::{parse_json_value, timestamp_to_datetime},
};
use sqlx::Row;
use vel_core::{Ref, Run, RunEvent, RunId};

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
