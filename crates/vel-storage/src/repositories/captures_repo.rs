use crate::db::{CaptureInsert, SearchFilters, SignalRecord, StorageError};
use crate::mapping::timestamp_to_datetime;
use crate::repositories::{processing_jobs_repo, semantic_memory_repo, signals_repo};
use serde_json::json;
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool};
use time::OffsetDateTime;
use vel_core::{CaptureId, ContextCapture, JobId, JobStatus, OrientationSnapshot, SearchResult};

pub(crate) async fn insert_capture(
    pool: &SqlitePool,
    input: CaptureInsert,
) -> Result<CaptureId, StorageError> {
    insert_capture_at(pool, input, OffsetDateTime::now_utc().unix_timestamp()).await
}

pub(crate) async fn insert_capture_at(
    pool: &SqlitePool,
    input: CaptureInsert,
    occurred_at: i64,
) -> Result<CaptureId, StorageError> {
    let capture_id = CaptureId::new();
    insert_capture_with_id_at(pool, capture_id.clone(), input, occurred_at).await?;
    Ok(capture_id)
}

pub(crate) async fn insert_capture_with_id(
    pool: &SqlitePool,
    capture_id: CaptureId,
    input: CaptureInsert,
) -> Result<bool, StorageError> {
    insert_capture_with_id_at(
        pool,
        capture_id,
        input,
        OffsetDateTime::now_utc().unix_timestamp(),
    )
    .await
}

pub(crate) async fn insert_capture_with_id_at(
    pool: &SqlitePool,
    capture_id: CaptureId,
    input: CaptureInsert,
    occurred_at: i64,
) -> Result<bool, StorageError> {
    let mut tx = pool.begin().await?;
    let result = insert_capture_with_id_in_tx(&mut tx, capture_id, input, occurred_at).await?;
    tx.commit().await?;
    Ok(result)
}

pub(crate) async fn insert_capture_with_id_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    capture_id: CaptureId,
    input: CaptureInsert,
    occurred_at: i64,
) -> Result<bool, StorageError> {
    let metadata = json!({});

    let result = sqlx::query(
        r#"
        INSERT OR IGNORE INTO captures (
            capture_id,
            capture_type,
            content_text,
            occurred_at,
            created_at,
            source_device,
            privacy_class,
            metadata_json
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(capture_id.to_string())
    .bind(&input.capture_type)
    .bind(&input.content_text)
    .bind(occurred_at)
    .bind(occurred_at)
    .bind(input.source_device)
    .bind(input.privacy_class.to_string())
    .bind(metadata.to_string())
    .execute(&mut **tx)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(false);
    }

    let job_id = JobId::new();
    let payload = json!({ "capture_id": capture_id.to_string() }).to_string();

    semantic_memory_repo::upsert_capture_record_in_tx(
        tx,
        capture_id.as_ref(),
        &input.content_text,
        occurred_at,
    )
    .await?;

    processing_jobs_repo::insert_processing_job_in_tx(
        tx,
        &job_id,
        "capture_ingest",
        JobStatus::Pending,
        &payload,
    )
    .await?;

    Ok(true)
}

pub(crate) async fn capture_count(pool: &SqlitePool) -> Result<i64, StorageError> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM captures")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

pub(crate) async fn get_capture_by_id(
    pool: &SqlitePool,
    capture_id: &str,
) -> Result<Option<ContextCapture>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT capture_id, capture_type, content_text, occurred_at, source_device
        FROM captures
        WHERE capture_id = ?
        "#,
    )
    .bind(capture_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };
    Ok(Some(map_context_capture_row(row)?))
}

pub(crate) async fn list_captures_recent(
    pool: &SqlitePool,
    limit: u32,
    today_only: bool,
) -> Result<Vec<ContextCapture>, StorageError> {
    let limit = limit.min(500) as i64;
    let rows = if today_only {
        let now = OffsetDateTime::now_utc();
        let start_of_day = now
            .date()
            .with_hms(0, 0, 0)
            .map_err(|e| StorageError::InvalidTimestamp(e.to_string()))?
            .assume_utc()
            .unix_timestamp();
        sqlx::query(
            r#"
            SELECT capture_id, capture_type, content_text, occurred_at, source_device
            FROM captures
            WHERE created_at >= ?
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(start_of_day)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT capture_id, capture_type, content_text, occurred_at, source_device
            FROM captures
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };
    rows.into_iter()
        .map(map_context_capture_row)
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn orientation_snapshot_at(
    pool: &SqlitePool,
    now: OffsetDateTime,
) -> Result<OrientationSnapshot, StorageError> {
    let start_of_day = now
        .date()
        .with_hms(0, 0, 0)
        .map_err(|error| StorageError::InvalidTimestamp(error.to_string()))?
        .assume_utc()
        .unix_timestamp();
    let seven_days_ago = now - time::Duration::days(7);

    let recent_today = sqlx::query(
        r#"
        SELECT capture_id, capture_type, content_text, occurred_at, source_device
        FROM captures
        WHERE occurred_at >= ?
        ORDER BY occurred_at DESC, created_at DESC
        LIMIT 10
        "#,
    )
    .bind(start_of_day)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(map_context_capture_row)
    .collect::<Result<Vec<_>, _>>()?;

    let recent_week = sqlx::query(
        r#"
        SELECT capture_id, capture_type, content_text, occurred_at, source_device
        FROM captures
        WHERE occurred_at >= ?
        ORDER BY occurred_at DESC, created_at DESC
        LIMIT 50
        "#,
    )
    .bind(seven_days_ago.unix_timestamp())
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(map_context_capture_row)
    .collect::<Result<Vec<_>, _>>()?;

    let recent_signal_summaries =
        signals_repo::list_signals(pool, None, Some(seven_days_ago.unix_timestamp()), 100)
            .await?
            .into_iter()
            .filter_map(|signal| summarize_signal_for_orientation(&signal))
            .take(25)
            .collect();

    Ok(OrientationSnapshot {
        recent_today,
        recent_week,
        recent_signal_summaries,
    })
}

pub(crate) async fn search_captures(
    pool: &SqlitePool,
    query: &str,
    filters: SearchFilters,
) -> Result<Vec<SearchResult>, StorageError> {
    let mut builder = QueryBuilder::<Sqlite>::new(
        r#"
        SELECT
            c.capture_id,
            c.capture_type,
            snippet(captures_fts, 1, '[', ']', '...', 12) AS snippet,
            c.occurred_at,
            c.created_at,
            c.source_device,
            bm25(captures_fts) AS rank
        FROM captures_fts
        JOIN captures c ON c.capture_id = captures_fts.capture_id
        WHERE captures_fts MATCH
        "#,
    );
    builder.push_bind(query);

    if let Some(capture_type) = filters.capture_type.as_deref() {
        builder.push(" AND c.capture_type = ");
        builder.push_bind(capture_type);
    }

    if let Some(source_device) = filters.source_device.as_deref() {
        builder.push(" AND c.source_device = ");
        builder.push_bind(source_device);
    }

    let limit = i64::from(filters.limit.unwrap_or(10).clamp(1, 50));
    builder.push(" ORDER BY rank ASC, c.occurred_at DESC, c.created_at DESC LIMIT ");
    builder.push_bind(limit);

    let rows = builder.build().fetch_all(pool).await?;
    rows.into_iter().map(map_search_row).collect()
}

fn summarize_signal_for_orientation(signal: &SignalRecord) -> Option<String> {
    let payload = &signal.payload_json;
    match signal.signal_type.as_str() {
        "calendar_event" => payload
            .get("title")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(|title| format!("event {}", title.trim())),
        "external_task" => payload
            .get("text")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(|text| format!("todo {}", text.trim())),
        "git_activity" => {
            let summary = payload
                .get("summary")
                .and_then(serde_json::Value::as_str)
                .or_else(|| payload.get("operation").and_then(serde_json::Value::as_str))
                .filter(|value| !value.trim().is_empty())?;
            let repo = payload
                .get("repo_name")
                .or_else(|| payload.get("repo"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("repo");
            Some(format!("git {} {}", repo, summary.trim()))
        }
        "message_thread" => {
            let summary = payload
                .get("summary")
                .or_else(|| payload.get("snippet"))
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.trim().is_empty())?;
            let waiting_state = payload
                .get("waiting_state")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("message");
            Some(format!(
                "{} {}",
                waiting_state.replace('_', " "),
                summary.trim()
            ))
        }
        "health_metric" => {
            let metric = payload
                .get("metric")
                .or_else(|| payload.get("summary"))
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.trim().is_empty())?;
            Some(format!("health {}", metric.trim()))
        }
        "mood_log" => payload
            .get("score")
            .and_then(serde_json::Value::as_u64)
            .map(|score| {
                let mut summary = format!("mood {score}/10");
                if let Some(label) = payload
                    .get("label")
                    .and_then(serde_json::Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                {
                    summary.push(' ');
                    summary.push_str(label);
                }
                summary
            }),
        "pain_log" => payload
            .get("severity")
            .and_then(serde_json::Value::as_u64)
            .map(|severity| {
                let mut summary = format!("pain {severity}/10");
                if let Some(location) = payload
                    .get("location")
                    .and_then(serde_json::Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                {
                    summary.push_str(" in ");
                    summary.push_str(location);
                }
                summary
            }),
        "assistant_message" => payload
            .get("content")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(|content| format!("assistant {}", truncate_summary(content.trim(), 96))),
        "note_document" => payload
            .get("title")
            .or_else(|| payload.get("path"))
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(|title| format!("note {}", title.trim())),
        _ => None,
    }
}

fn truncate_summary(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut truncated = value
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    truncated.push('…');
    truncated
}

fn map_context_capture_row(row: sqlx::sqlite::SqliteRow) -> Result<ContextCapture, StorageError> {
    Ok(ContextCapture {
        capture_id: CaptureId::from(row.try_get::<String, _>("capture_id")?),
        capture_type: row.try_get("capture_type")?,
        content_text: row.try_get("content_text")?,
        occurred_at: timestamp_to_datetime(row.try_get("occurred_at")?)?,
        source_device: row.try_get("source_device")?,
    })
}

fn map_search_row(row: sqlx::sqlite::SqliteRow) -> Result<SearchResult, StorageError> {
    let occurred_at = timestamp_to_datetime(row.try_get("occurred_at")?)?;
    let created_at = timestamp_to_datetime(row.try_get("created_at")?)?;

    Ok(SearchResult {
        capture_id: CaptureId::from(row.try_get::<String, _>("capture_id")?),
        capture_type: row.try_get("capture_type")?,
        snippet: row.try_get("snippet")?,
        occurred_at,
        created_at,
        source_device: row.try_get("source_device")?,
    })
}
