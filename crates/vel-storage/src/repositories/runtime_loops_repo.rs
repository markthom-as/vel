use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::db::{RuntimeLoopRecord, StorageError};

pub(crate) async fn claim_due_loop(
    pool: &SqlitePool,
    loop_kind: &str,
    interval_seconds: i64,
    now_ts: i64,
) -> Result<bool, StorageError> {
    let result = sqlx::query(
        r#"
        UPDATE runtime_loops
        SET last_status = 'running',
            last_started_at = ?,
            last_finished_at = NULL,
            last_error = NULL
        WHERE loop_kind = ?
          AND enabled = 1
          AND (last_status IS NULL OR last_status != 'running' OR last_started_at IS NULL OR last_started_at < ? - 300)
          AND (next_due_at IS NULL OR next_due_at <= ?)
        "#,
    )
    .bind(now_ts)
    .bind(loop_kind)
    .bind(now_ts)
    .bind(now_ts)
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(true)
    } else {
        // If it doesn't exist yet, try to create it.
        // This is a bit racey but fine for loop registration.
        let _ = sqlx::query(
            r#"
            INSERT OR IGNORE INTO runtime_loops (loop_kind, enabled, interval_seconds, next_due_at)
            VALUES (?, 1, ?, ?)
            "#,
        )
        .bind(loop_kind)
        .bind(interval_seconds)
        .bind(now_ts)
        .execute(pool)
        .await;

        // Try one more time after potential insert
        let result = sqlx::query(
            r#"
            UPDATE runtime_loops
            SET last_status = 'running',
                last_started_at = ?,
                last_finished_at = NULL,
                last_error = NULL
            WHERE loop_kind = ?
              AND enabled = 1
              AND (last_status IS NULL OR last_status != 'running')
              AND (next_due_at IS NULL OR next_due_at <= ?)
            "#,
        )
        .bind(now_ts)
        .bind(loop_kind)
        .bind(now_ts)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
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
        INSERT INTO runtime_loops (loop_kind, enabled, interval_seconds, next_due_at)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(loop_kind) DO UPDATE SET
            enabled = excluded.enabled,
            interval_seconds = excluded.interval_seconds,
            next_due_at = COALESCE(runtime_loops.next_due_at, excluded.next_due_at)
        "#,
    )
    .bind(loop_kind)
    .bind(if enabled { 1 } else { 0 })
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
        SET last_status = ?,
            last_error = ?,
            last_finished_at = ?,
            next_due_at = ?
        WHERE loop_kind = ?
        "#,
    )
    .bind(status)
    .bind(error)
    .bind(now)
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
        SELECT loop_kind, enabled, interval_seconds, last_status, last_error, last_started_at, last_finished_at, next_due_at
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
        SELECT loop_kind, enabled, interval_seconds, last_status, last_error, last_started_at, last_finished_at, next_due_at
        FROM runtime_loops
        WHERE loop_kind = ?
        "#,
    )
    .bind(loop_kind)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(map_runtime_loop_row(&row)?)),
        None => Ok(None),
    }
}

pub(crate) async fn update_runtime_loop_config(
    pool: &SqlitePool,
    loop_kind: &str,
    enabled: Option<bool>,
    interval_seconds: Option<i64>,
) -> Result<Option<RuntimeLoopRecord>, StorageError> {
    if let Some(enabled) = enabled {
        sqlx::query("UPDATE runtime_loops SET enabled = ? WHERE loop_kind = ?")
            .bind(if enabled { 1 } else { 0 })
            .bind(loop_kind)
            .execute(pool)
            .await?;
    }
    if let Some(interval) = interval_seconds {
        sqlx::query("UPDATE runtime_loops SET interval_seconds = ? WHERE loop_kind = ?")
            .bind(interval)
            .bind(loop_kind)
            .execute(pool)
            .await?;
    }

    get_runtime_loop(pool, loop_kind).await
}

fn map_runtime_loop_row(row: &sqlx::sqlite::SqliteRow) -> Result<RuntimeLoopRecord, StorageError> {
    let enabled: i64 = row.try_get("enabled")?;
    Ok(RuntimeLoopRecord {
        loop_kind: row.try_get("loop_kind")?,
        enabled: enabled != 0,
        interval_seconds: row.try_get("interval_seconds")?,
        last_status: row.try_get("last_status")?,
        last_error: row.try_get("last_error")?,
        last_started_at: row.try_get("last_started_at")?,
        last_finished_at: row.try_get("last_finished_at")?,
        next_due_at: row.try_get("next_due_at")?,
    })
}
