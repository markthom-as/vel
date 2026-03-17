use serde_json::{json, Value as JsonValue};
use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{Run, RunEvent, RunEventType, RunId, RunKind, RunStatus};

use crate::{
    db::{RetryReadyRun, StorageError},
    repositories::run_refs_repo::{map_run_event_row, map_run_row},
};

pub(crate) async fn create_run(
    pool: &SqlitePool,
    id: &RunId,
    kind: RunKind,
    input_json: &JsonValue,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let input_str =
        serde_json::to_string(input_json).map_err(|e| StorageError::Validation(e.to_string()))?;
    let run_created_payload = json!({ "kind": kind.to_string() });
    let payload_str = serde_json::to_string(&run_created_payload)
        .map_err(|e| StorageError::Validation(e.to_string()))?;
    let event_id = format!("evt_{}", Uuid::new_v4().simple());
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"
        INSERT INTO runs (run_id, run_kind, status, created_at, input_json)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(id.as_ref())
    .bind(kind.to_string())
    .bind(RunStatus::Queued.to_string())
    .bind(now)
    .bind(&input_str)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO run_events (event_id, run_id, seq, event_type, payload_json, created_at)
        VALUES (?, ?, 1, ?, ?, ?)
        "#,
    )
    .bind(&event_id)
    .bind(id.as_ref())
    .bind(RunEventType::RunCreated.to_string())
    .bind(&payload_str)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn get_run_by_id(
    pool: &SqlitePool,
    run_id: &str,
) -> Result<Option<Run>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT run_id, run_kind, status, input_json, output_json, error_json,
               created_at, started_at, finished_at
        FROM runs WHERE run_id = ?
        "#,
    )
    .bind(run_id)
    .fetch_optional(pool)
    .await?;
    let Some(row) = row else {
        return Ok(None);
    };
    Ok(Some(map_run_row(&row)?))
}

pub(crate) async fn list_runs(
    pool: &SqlitePool,
    limit: u32,
    kind_filter: Option<&str>,
    status_filter: Option<&str>,
) -> Result<Vec<Run>, StorageError> {
    let limit = limit.clamp(1, 100) as i64;
    let mut sql = r#"
        SELECT run_id, run_kind, status, input_json, output_json, error_json,
               created_at, started_at, finished_at
        FROM runs
    "#
    .to_string();
    let mut conditions = Vec::new();
    if kind_filter.map(|s| !s.is_empty()).unwrap_or(false) {
        conditions.push("run_kind = ?");
    }
    if status_filter.map(|s| !s.is_empty()).unwrap_or(false) {
        conditions.push("status = ?");
    }
    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }
    sql.push_str(" ORDER BY created_at DESC LIMIT ?");

    let mut q = sqlx::query(&sql);
    if let Some(k) = kind_filter.filter(|s| !s.is_empty()) {
        q = q.bind(k);
    }
    if let Some(status) = status_filter.filter(|s| !s.is_empty()) {
        q = q.bind(status);
    }
    q = q.bind(limit);

    let rows = q.fetch_all(pool).await?;
    rows.into_iter()
        .map(|row| map_run_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn update_run_status(
    pool: &SqlitePool,
    run_id: &str,
    status: RunStatus,
    started_at: Option<i64>,
    finished_at: Option<i64>,
    output_json: Option<&JsonValue>,
    error_json: Option<&JsonValue>,
) -> Result<(), StorageError> {
    let output_str = output_json
        .map(|v| serde_json::to_string(v).map_err(|e| StorageError::Validation(e.to_string())))
        .transpose()?;
    let error_str = error_json
        .map(|v| serde_json::to_string(v).map_err(|e| StorageError::Validation(e.to_string())))
        .transpose()?;
    sqlx::query(
        r#"
        UPDATE runs SET status = ?,
            started_at = COALESCE(?, started_at),
            finished_at = COALESCE(?, finished_at),
            output_json = ?, error_json = ?
        WHERE run_id = ?
        "#,
    )
    .bind(status.to_string())
    .bind(started_at)
    .bind(finished_at)
    .bind(output_str)
    .bind(error_str)
    .bind(run_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn reset_run_for_retry(
    pool: &SqlitePool,
    run_id: &str,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        UPDATE runs
        SET status = ?,
            started_at = NULL,
            finished_at = NULL,
            output_json = NULL,
            error_json = NULL
        WHERE run_id = ?
        "#,
    )
    .bind(RunStatus::Queued.to_string())
    .bind(run_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn append_run_event(
    pool: &SqlitePool,
    run_id: &str,
    seq: u32,
    event_type: RunEventType,
    payload_json: &JsonValue,
) -> Result<String, StorageError> {
    let event_id = format!("evt_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let payload_str =
        serde_json::to_string(payload_json).map_err(|e| StorageError::Validation(e.to_string()))?;
    sqlx::query(
        r#"
        INSERT INTO run_events (event_id, run_id, seq, event_type, payload_json, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&event_id)
    .bind(run_id)
    .bind(seq as i64)
    .bind(event_type.to_string())
    .bind(&payload_str)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(event_id)
}

pub(crate) async fn next_run_event_seq(
    pool: &SqlitePool,
    run_id: &str,
) -> Result<u32, StorageError> {
    let (next_seq,): (i64,) =
        sqlx::query_as(r#"SELECT COALESCE(MAX(seq), 0) + 1 FROM run_events WHERE run_id = ?"#)
            .bind(run_id)
            .fetch_one(pool)
            .await?;
    Ok(next_seq as u32)
}

pub(crate) async fn append_run_event_auto(
    pool: &SqlitePool,
    run_id: &str,
    event_type: RunEventType,
    payload_json: &JsonValue,
) -> Result<String, StorageError> {
    let seq = next_run_event_seq(pool, run_id).await?;
    append_run_event(pool, run_id, seq, event_type, payload_json).await
}

pub(crate) async fn list_retry_ready_runs(
    pool: &SqlitePool,
    now_ts: i64,
    max_retries: i64,
    limit: u32,
) -> Result<Vec<RetryReadyRun>, StorageError> {
    let limit = limit.clamp(1, 100) as i64;
    let rows = sqlx::query(
        r#"
        SELECT
            r.run_id,
            r.run_kind,
            r.status,
            r.input_json,
            r.output_json,
            r.error_json,
            r.created_at,
            r.started_at,
            r.finished_at,
            CAST(json_extract(e.payload_json, '$.retry_at') AS INTEGER) AS retry_at,
            json_extract(e.payload_json, '$.reason') AS retry_reason
        FROM runs r
        JOIN (
            SELECT run_id, MAX(seq) AS max_seq
            FROM run_events
            WHERE event_type = ?
            GROUP BY run_id
        ) latest
          ON latest.run_id = r.run_id
        JOIN run_events e
          ON e.run_id = latest.run_id AND e.seq = latest.max_seq
        WHERE r.status = ?
          AND CAST(json_extract(e.payload_json, '$.retry_at') AS INTEGER) <= ?
          AND CAST(json_extract(e.payload_json, '$.attempt_count') AS INTEGER) <= ?
        ORDER BY retry_at ASC, r.created_at ASC
        LIMIT ?
        "#,
    )
    .bind(RunEventType::RunRetryScheduled.to_string())
    .bind(RunStatus::RetryScheduled.to_string())
    .bind(now_ts)
    .bind(max_retries)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            let retry_at = row.try_get::<i64, _>("retry_at")?;
            let retry_reason = row.try_get::<Option<String>, _>("retry_reason")?;
            let run = map_run_row(&row)?;
            Ok(RetryReadyRun {
                run,
                retry_at,
                retry_reason,
            })
        })
        .collect()
}

pub(crate) async fn list_run_events(
    pool: &SqlitePool,
    run_id: &str,
) -> Result<Vec<RunEvent>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT event_id, run_id, seq, event_type, payload_json, created_at
        FROM run_events WHERE run_id = ? ORDER BY seq ASC
        "#,
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(map_run_event_row)
        .collect::<Result<Vec<_>, _>>()
}
