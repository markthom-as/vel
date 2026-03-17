use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::db::{RuntimeLoopRecord, StorageError};

pub(crate) async fn claim_due_loop(
    pool: &SqlitePool,
    loop_kind: &str,
    interval_seconds: i64,
    now_ts: i64,
) -> Result<bool, StorageError> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO runtime_loops (
            loop_kind,
            enabled,
            interval_seconds,
            next_due_at
        ) VALUES (?, 1, ?, 0)
        "#,
    )
    .bind(loop_kind)
    .bind(interval_seconds)
    .execute(pool)
    .await?;

    let result = sqlx::query(
        r#"
        UPDATE runtime_loops
        SET interval_seconds = ?,
            last_started_at = ?,
            last_status = 'running',
            last_error = NULL
        WHERE loop_kind = ?
          AND enabled = 1
          AND COALESCE(last_status, '') != 'running'
          AND COALESCE(next_due_at, 0) <= ?
        "#,
    )
    .bind(interval_seconds)
    .bind(now_ts)
    .bind(loop_kind)
    .bind(now_ts)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

pub(crate) async fn ensure_runtime_loop(
    pool: &SqlitePool,
    loop_kind: &str,
    enabled: bool,
    interval_seconds: i64,
    next_due_at: Option<i64>,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO runtime_loops (
            loop_kind,
            enabled,
            interval_seconds,
            next_due_at
        ) VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(loop_kind)
    .bind(if enabled { 1_i64 } else { 0_i64 })
    .bind(interval_seconds)
    .bind(next_due_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn complete_loop(
    pool: &SqlitePool,
    loop_kind: &str,
    status: &str,
    error: Option<&str>,
    next_due_at: i64,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        UPDATE runtime_loops
        SET last_finished_at = ?,
            last_status = ?,
            last_error = ?,
            next_due_at = ?
        WHERE loop_kind = ?
        "#,
    )
    .bind(now)
    .bind(status)
    .bind(error)
    .bind(next_due_at)
    .bind(loop_kind)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn list_runtime_loops(
    pool: &SqlitePool,
) -> Result<Vec<RuntimeLoopRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            loop_kind,
            enabled,
            interval_seconds,
            last_started_at,
            last_finished_at,
            last_status,
            last_error,
            next_due_at
        FROM runtime_loops
        ORDER BY loop_kind ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_runtime_loop_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn get_runtime_loop(
    pool: &SqlitePool,
    loop_kind: &str,
) -> Result<Option<RuntimeLoopRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            loop_kind,
            enabled,
            interval_seconds,
            last_started_at,
            last_finished_at,
            last_status,
            last_error,
            next_due_at
        FROM runtime_loops
        WHERE loop_kind = ?
        "#,
    )
    .bind(loop_kind)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_runtime_loop_row).transpose()
}

pub(crate) async fn update_runtime_loop_config(
    pool: &SqlitePool,
    loop_kind: &str,
    enabled: Option<bool>,
    interval_seconds: Option<i64>,
) -> Result<Option<RuntimeLoopRecord>, StorageError> {
    let Some(existing) = get_runtime_loop(pool, loop_kind).await? else {
        return Ok(None);
    };

    let next_enabled = enabled.unwrap_or(existing.enabled);
    let next_interval_seconds = interval_seconds.unwrap_or(existing.interval_seconds);
    if next_interval_seconds <= 0 {
        return Err(StorageError::Validation(
            "interval_seconds must be > 0".to_string(),
        ));
    }

    let now = OffsetDateTime::now_utc().unix_timestamp();
    let next_due_at = if next_enabled {
        Some(now + next_interval_seconds)
    } else {
        existing.next_due_at
    };

    sqlx::query(
        r#"
        UPDATE runtime_loops
        SET enabled = ?,
            interval_seconds = ?,
            next_due_at = ?
        WHERE loop_kind = ?
        "#,
    )
    .bind(if next_enabled { 1_i64 } else { 0_i64 })
    .bind(next_interval_seconds)
    .bind(next_due_at)
    .bind(loop_kind)
    .execute(pool)
    .await?;

    get_runtime_loop(pool, loop_kind).await
}

fn map_runtime_loop_row(row: &sqlx::sqlite::SqliteRow) -> Result<RuntimeLoopRecord, StorageError> {
    let enabled: i64 = row.try_get("enabled")?;
    Ok(RuntimeLoopRecord {
        loop_kind: row.try_get("loop_kind")?,
        enabled: enabled != 0,
        interval_seconds: row.try_get("interval_seconds")?,
        last_started_at: row.try_get("last_started_at")?,
        last_finished_at: row.try_get("last_finished_at")?,
        last_status: row.try_get("last_status")?,
        last_error: row.try_get("last_error")?,
        next_due_at: row.try_get("next_due_at")?,
    })
}
