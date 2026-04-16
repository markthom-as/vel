use serde_json::Value as JsonValue;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;

use crate::{
    db::{BackupJobEventRecord, BackupJobRecord, StorageError},
    mapping::timestamp_to_datetime,
};

const BACKUP_EXPORT_TARGET_KIND: &str = "local_filesystem";
const BACKUP_EXPORT_TARGET_ROLE: &str = "knowledge_export";
const BACKUP_EXPORT_TARGET_LABEL: &str = "Scheduled Backup Export Target";
const SCHEDULED_TRIGGER_TYPE: &str = "scheduled";
const COLD_STORAGE_SCOPE: &str = "cold_storage";
const DEFAULT_SAFETY_MODE: &str = "default_local_only";
const REQUESTED_BY_SCHEDULER: &str = "scheduler";

pub(crate) async fn queue_scheduled_backup_export_job(
    pool: &SqlitePool,
    target_root: &str,
    payload_json: &JsonValue,
    now: OffsetDateTime,
) -> Result<BackupJobRecord, StorageError> {
    let mut tx = pool.begin().await?;
    let storage_target_id = ensure_backup_export_job_target(&mut tx, target_root, now).await?;

    if let Some(existing) = fetch_active_scheduled_export_job(&mut tx, &storage_target_id).await? {
        tx.commit().await?;
        return Ok(existing);
    }

    let job_id = format!("bjob_{}", uuid::Uuid::new_v4().simple());
    let now_ts = now.unix_timestamp();
    sqlx::query(
        r#"
        INSERT INTO v0_backup_jobs (
            backup_job_id,
            storage_target_id,
            trigger_type,
            scope,
            status,
            safety_mode,
            requested_by,
            urgency,
            attempt,
            max_attempts,
            base_backoff_minutes,
            created_at,
            next_attempt_at,
            payload_json
        ) VALUES (?, ?, ?, ?, 'queued', ?, ?, 0, 0, 6, 2, ?, ?, ?)
        "#,
    )
    .bind(&job_id)
    .bind(&storage_target_id)
    .bind(SCHEDULED_TRIGGER_TYPE)
    .bind(COLD_STORAGE_SCOPE)
    .bind(DEFAULT_SAFETY_MODE)
    .bind(REQUESTED_BY_SCHEDULER)
    .bind(now_ts)
    .bind(now_ts)
    .bind(serde_json::to_string(payload_json)?)
    .execute(&mut *tx)
    .await?;

    insert_backup_job_event(
        &mut tx,
        &job_id,
        "backup_job_queued",
        None,
        Some("queued"),
        None,
        None,
        now,
        &serde_json::json!({
            "storage_target_id": storage_target_id,
            "scope": COLD_STORAGE_SCOPE
        }),
    )
    .await?;

    let record = get_backup_job_in_tx(&mut tx, &job_id)
        .await?
        .ok_or_else(|| StorageError::DataCorrupted("queued backup job missing".to_string()))?;
    tx.commit().await?;
    Ok(record)
}

pub(crate) async fn claim_next_due_scheduled_backup_export_job(
    pool: &SqlitePool,
    now: OffsetDateTime,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let mut tx = pool.begin().await?;
    let now_ts = now.unix_timestamp();
    let Some(job) = fetch_next_due_scheduled_export_job(&mut tx, now_ts).await? else {
        tx.commit().await?;
        return Ok(None);
    };

    let next_attempt = job.attempt + 1;
    sqlx::query(
        r#"
        UPDATE v0_backup_jobs
        SET status = 'running',
            attempt = ?,
            started_at = ?,
            last_error_code = NULL,
            last_error_message = NULL,
            last_error_transient = 1
        WHERE backup_job_id = ?
          AND status IN ('queued', 'failed')
        "#,
    )
    .bind(next_attempt)
    .bind(now_ts)
    .bind(&job.backup_job_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO v0_backup_job_attempts (
            attempt_id,
            backup_job_id,
            attempt_no,
            state,
            scheduled_at,
            started_at,
            metadata_json
        ) VALUES (?, ?, ?, 'running', ?, ?, '{}')
        "#,
    )
    .bind(format!("bjat_{}", uuid::Uuid::new_v4().simple()))
    .bind(&job.backup_job_id)
    .bind(next_attempt)
    .bind(
        job.next_attempt_at
            .unwrap_or(job.created_at)
            .unix_timestamp(),
    )
    .bind(now_ts)
    .execute(&mut *tx)
    .await?;

    insert_backup_job_event(
        &mut tx,
        &job.backup_job_id,
        "backup_job_started",
        Some(job.status.as_str()),
        Some("running"),
        None,
        None,
        now,
        &serde_json::json!({ "attempt": next_attempt }),
    )
    .await?;

    let record = get_backup_job_in_tx(&mut tx, &job.backup_job_id)
        .await?
        .ok_or_else(|| StorageError::DataCorrupted("claimed backup job missing".to_string()))?;
    tx.commit().await?;
    Ok(Some(record))
}

pub(crate) async fn complete_backup_export_job_success(
    pool: &SqlitePool,
    backup_job_id: &str,
    manifest_id: Option<&str>,
    now: OffsetDateTime,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let mut tx = pool.begin().await?;
    let Some(job) = get_backup_job_in_tx(&mut tx, backup_job_id).await? else {
        tx.commit().await?;
        return Ok(None);
    };
    let now_ts = now.unix_timestamp();

    sqlx::query(
        r#"
        UPDATE v0_backup_jobs
        SET status = 'succeeded',
            manifest_id = ?,
            finished_at = ?,
            completed_at = ?,
            last_error_code = NULL,
            last_error_message = NULL,
            last_error_transient = 0
        WHERE backup_job_id = ?
        "#,
    )
    .bind(manifest_id)
    .bind(now_ts)
    .bind(now_ts)
    .bind(backup_job_id)
    .execute(&mut *tx)
    .await?;

    update_latest_attempt(
        &mut tx,
        backup_job_id,
        job.attempt,
        "succeeded",
        now,
        None,
        None,
        None,
        None,
    )
    .await?;
    insert_backup_job_event(
        &mut tx,
        backup_job_id,
        "backup_job_succeeded",
        Some(job.status.as_str()),
        Some("succeeded"),
        None,
        None,
        now,
        &serde_json::json!({ "manifest_id": manifest_id }),
    )
    .await?;

    let record = get_backup_job_in_tx(&mut tx, backup_job_id).await?;
    tx.commit().await?;
    Ok(record)
}

pub(crate) async fn complete_backup_export_job_failure(
    pool: &SqlitePool,
    backup_job_id: &str,
    error_code: &str,
    error_message: &str,
    transient: bool,
    now: OffsetDateTime,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let mut tx = pool.begin().await?;
    let Some(job) = get_backup_job_in_tx(&mut tx, backup_job_id).await? else {
        tx.commit().await?;
        return Ok(None);
    };
    let now_ts = now.unix_timestamp();
    let retry_wait_seconds = if transient && job.attempt < job.max_attempts {
        Some(job.base_backoff_minutes.saturating_mul(60) * job.attempt.max(1))
    } else {
        None
    };

    sqlx::query(
        r#"
        UPDATE v0_backup_jobs
        SET status = 'failed',
            finished_at = ?,
            next_attempt_at = ?,
            last_error_code = ?,
            last_error_message = ?,
            last_error_transient = ?
        WHERE backup_job_id = ?
        "#,
    )
    .bind(now_ts)
    .bind(retry_wait_seconds.map(|seconds| now_ts + seconds))
    .bind(error_code)
    .bind(error_message)
    .bind(if transient { 1 } else { 0 })
    .bind(backup_job_id)
    .execute(&mut *tx)
    .await?;

    update_latest_attempt(
        &mut tx,
        backup_job_id,
        job.attempt,
        "failed",
        now,
        Some(error_code),
        Some(error_message),
        Some(transient),
        retry_wait_seconds,
    )
    .await?;
    insert_backup_job_event(
        &mut tx,
        backup_job_id,
        "backup_job_failed",
        Some(job.status.as_str()),
        Some("failed"),
        Some(error_code),
        Some(error_message),
        now,
        &serde_json::json!({
            "transient": transient,
            "retry_wait_seconds": retry_wait_seconds
        }),
    )
    .await?;

    let record = get_backup_job_in_tx(&mut tx, backup_job_id).await?;
    tx.commit().await?;
    Ok(record)
}

pub(crate) async fn get_backup_job(
    pool: &SqlitePool,
    backup_job_id: &str,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let row = backup_job_query()
        .push(" WHERE j.backup_job_id = ")
        .push_bind(backup_job_id)
        .build()
        .fetch_optional(pool)
        .await?;

    row.as_ref().map(map_backup_job_row).transpose()
}

pub(crate) async fn get_latest_finished_scheduled_backup_export_job(
    pool: &SqlitePool,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let row = backup_job_query()
        .push(
            r#"
            WHERE j.trigger_type = "#,
        )
        .push_bind(SCHEDULED_TRIGGER_TYPE)
        .push(
            r#"
              AND j.scope = "#,
        )
        .push_bind(COLD_STORAGE_SCOPE)
        .push(
            r#"
              AND j.status IN ('succeeded', 'failed', 'blocked', 'expired', 'cancelled')
            ORDER BY COALESCE(j.finished_at, j.completed_at, j.started_at, j.created_at) DESC,
                     j.created_at DESC
            LIMIT 1
            "#,
        )
        .build()
        .fetch_optional(pool)
        .await?;

    row.as_ref().map(map_backup_job_row).transpose()
}

pub(crate) async fn list_backup_job_events(
    pool: &SqlitePool,
    backup_job_id: &str,
) -> Result<Vec<BackupJobEventRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            event_id,
            backup_job_id,
            event_type,
            state_before,
            state_after,
            reason_code,
            reason_text,
            created_at,
            metadata_json
        FROM v0_backup_job_events
        WHERE backup_job_id = ?
        ORDER BY created_at ASC, event_id ASC
        "#,
    )
    .bind(backup_job_id)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_backup_job_event_row).collect()
}

async fn ensure_backup_export_job_target(
    tx: &mut Transaction<'_, Sqlite>,
    target_root: &str,
    now: OffsetDateTime,
) -> Result<String, StorageError> {
    if let Some(row) = sqlx::query(
        r#"
        SELECT storage_target_id
        FROM storage_targets
        WHERE kind = ?
          AND role = ?
          AND root_uri = ?
        ORDER BY created_at ASC
        LIMIT 1
        "#,
    )
    .bind(BACKUP_EXPORT_TARGET_KIND)
    .bind(BACKUP_EXPORT_TARGET_ROLE)
    .bind(target_root)
    .fetch_optional(&mut **tx)
    .await?
    {
        return row.try_get("storage_target_id").map_err(Into::into);
    }

    let storage_target_id = format!("stgt_backup_export_{}", uuid::Uuid::new_v4().simple());
    sqlx::query(
        r#"
        INSERT INTO storage_targets (
            storage_target_id,
            kind,
            role,
            label,
            root_uri,
            metadata_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&storage_target_id)
    .bind(BACKUP_EXPORT_TARGET_KIND)
    .bind(BACKUP_EXPORT_TARGET_ROLE)
    .bind(BACKUP_EXPORT_TARGET_LABEL)
    .bind(target_root)
    .bind(r#"{"managed_by":"scheduled_backup_export_jobs"}"#)
    .bind(now.unix_timestamp())
    .bind(now.unix_timestamp())
    .execute(&mut **tx)
    .await?;

    Ok(storage_target_id)
}

async fn fetch_active_scheduled_export_job(
    tx: &mut Transaction<'_, Sqlite>,
    storage_target_id: &str,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let row = backup_job_query()
        .push(
            r#"
            WHERE j.storage_target_id = "#,
        )
        .push_bind(storage_target_id)
        .push(
            r#"
              AND j.trigger_type = "#,
        )
        .push_bind(SCHEDULED_TRIGGER_TYPE)
        .push(
            r#"
              AND j.scope = "#,
        )
        .push_bind(COLD_STORAGE_SCOPE)
        .push(
            r#"
              AND j.status IN ('queued', 'running')
            ORDER BY j.created_at DESC
            LIMIT 1
            "#,
        )
        .build()
        .fetch_optional(&mut **tx)
        .await?;

    row.as_ref().map(map_backup_job_row).transpose()
}

async fn fetch_next_due_scheduled_export_job(
    tx: &mut Transaction<'_, Sqlite>,
    now_ts: i64,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let row = backup_job_query()
        .push(
            r#"
            WHERE j.trigger_type = "#,
        )
        .push_bind(SCHEDULED_TRIGGER_TYPE)
        .push(
            r#"
              AND j.scope = "#,
        )
        .push_bind(COLD_STORAGE_SCOPE)
        .push(
            r#"
              AND (
                j.status = 'queued'
                OR (j.status = 'failed' AND j.next_attempt_at IS NOT NULL)
              )
              AND COALESCE(j.next_attempt_at, j.created_at) <= "#,
        )
        .push_bind(now_ts)
        .push(
            r#"
            ORDER BY j.urgency DESC, COALESCE(j.next_attempt_at, j.created_at) ASC, j.created_at ASC
            LIMIT 1
            "#,
        )
        .build()
        .fetch_optional(&mut **tx)
        .await?;

    row.as_ref().map(map_backup_job_row).transpose()
}

async fn get_backup_job_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    backup_job_id: &str,
) -> Result<Option<BackupJobRecord>, StorageError> {
    let row = backup_job_query()
        .push(" WHERE j.backup_job_id = ")
        .push_bind(backup_job_id)
        .build()
        .fetch_optional(&mut **tx)
        .await?;

    row.as_ref().map(map_backup_job_row).transpose()
}

fn backup_job_query<'args>() -> sqlx::QueryBuilder<'args, Sqlite> {
    sqlx::QueryBuilder::new(
        r#"
        SELECT
            j.backup_job_id,
            j.storage_target_id,
            st.root_uri AS storage_target_root,
            j.trigger_type,
            j.scope,
            j.status,
            j.safety_mode,
            j.requested_by,
            j.requested_by_ref,
            j.manifest_id,
            j.urgency,
            j.attempt,
            j.max_attempts,
            j.base_backoff_minutes,
            j.queue_confidence,
            j.created_at,
            j.next_attempt_at,
            j.started_at,
            j.finished_at,
            j.completed_at,
            j.last_error_code,
            j.last_error_message,
            j.last_error_transient,
            j.policy_json,
            j.payload_json
        FROM v0_backup_jobs j
        JOIN storage_targets st ON st.storage_target_id = j.storage_target_id
        "#,
    )
}

async fn update_latest_attempt(
    tx: &mut Transaction<'_, Sqlite>,
    backup_job_id: &str,
    attempt_no: i64,
    state: &str,
    finished_at: OffsetDateTime,
    error_code: Option<&str>,
    error_message: Option<&str>,
    error_is_transient: Option<bool>,
    retry_wait_seconds: Option<i64>,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        UPDATE v0_backup_job_attempts
        SET state = ?,
            finished_at = ?,
            error_code = ?,
            error_message = ?,
            error_is_transient = ?,
            retry_wait_seconds = ?
        WHERE backup_job_id = ?
          AND attempt_no = ?
        "#,
    )
    .bind(state)
    .bind(finished_at.unix_timestamp())
    .bind(error_code)
    .bind(error_message)
    .bind(error_is_transient.map(|value| if value { 1 } else { 0 }))
    .bind(retry_wait_seconds)
    .bind(backup_job_id)
    .bind(attempt_no)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_backup_job_event(
    tx: &mut Transaction<'_, Sqlite>,
    backup_job_id: &str,
    event_type: &str,
    state_before: Option<&str>,
    state_after: Option<&str>,
    reason_code: Option<&str>,
    reason_text: Option<&str>,
    created_at: OffsetDateTime,
    metadata_json: &JsonValue,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO v0_backup_job_events (
            event_id,
            backup_job_id,
            event_type,
            state_before,
            state_after,
            reason_code,
            reason_text,
            created_at,
            metadata_json
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(format!("bje_{}", uuid::Uuid::new_v4().simple()))
    .bind(backup_job_id)
    .bind(event_type)
    .bind(state_before)
    .bind(state_after)
    .bind(reason_code)
    .bind(reason_text)
    .bind(created_at.unix_timestamp())
    .bind(serde_json::to_string(metadata_json)?)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

fn map_backup_job_row(row: &sqlx::sqlite::SqliteRow) -> Result<BackupJobRecord, StorageError> {
    Ok(BackupJobRecord {
        backup_job_id: row.try_get("backup_job_id")?,
        storage_target_id: row.try_get("storage_target_id")?,
        storage_target_root: row.try_get("storage_target_root")?,
        trigger_type: row.try_get("trigger_type")?,
        scope: row.try_get("scope")?,
        status: row.try_get("status")?,
        safety_mode: row.try_get("safety_mode")?,
        requested_by: row.try_get("requested_by")?,
        requested_by_ref: row.try_get("requested_by_ref")?,
        manifest_id: row.try_get("manifest_id")?,
        urgency: row.try_get("urgency")?,
        attempt: row.try_get("attempt")?,
        max_attempts: row.try_get("max_attempts")?,
        base_backoff_minutes: row.try_get("base_backoff_minutes")?,
        queue_confidence: row.try_get("queue_confidence")?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        next_attempt_at: row
            .try_get::<Option<i64>, _>("next_attempt_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        started_at: row
            .try_get::<Option<i64>, _>("started_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        finished_at: row
            .try_get::<Option<i64>, _>("finished_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        completed_at: row
            .try_get::<Option<i64>, _>("completed_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        last_error_code: row.try_get("last_error_code")?,
        last_error_message: row.try_get("last_error_message")?,
        last_error_transient: row.try_get::<i64, _>("last_error_transient")? != 0,
        policy_json: serde_json::from_str(&row.try_get::<String, _>("policy_json")?)?,
        payload_json: serde_json::from_str(&row.try_get::<String, _>("payload_json")?)?,
    })
}

fn map_backup_job_event_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<BackupJobEventRecord, StorageError> {
    Ok(BackupJobEventRecord {
        event_id: row.try_get("event_id")?,
        backup_job_id: row.try_get("backup_job_id")?,
        event_type: row.try_get("event_type")?,
        state_before: row.try_get("state_before")?,
        state_after: row.try_get("state_after")?,
        reason_code: row.try_get("reason_code")?,
        reason_text: row.try_get("reason_text")?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        metadata_json: serde_json::from_str(&row.try_get::<String, _>("metadata_json")?)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn scheduled_backup_export_jobs_dedupe_per_target_and_claim_due() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();

        let first = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["tasks"] }),
            now,
        )
        .await
        .unwrap();
        let duplicate = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["calendar"] }),
            now,
        )
        .await
        .unwrap();
        let other_target = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/activity",
            &serde_json::json!({ "domains": ["activity"] }),
            now,
        )
        .await
        .unwrap();

        assert_eq!(first.backup_job_id, duplicate.backup_job_id);
        assert_ne!(first.backup_job_id, other_target.backup_job_id);

        let claimed = claim_next_due_scheduled_backup_export_job(&pool, now)
            .await
            .unwrap()
            .expect("due job");
        assert_eq!(claimed.status, "running");
        assert_eq!(claimed.attempt, 1);

        let duplicate_running = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["tasks"] }),
            now,
        )
        .await
        .unwrap();
        if claimed.storage_target_id == first.storage_target_id {
            assert_eq!(duplicate_running.backup_job_id, claimed.backup_job_id);
        }
    }

    #[tokio::test]
    async fn scheduled_backup_export_job_lifecycle_events_are_persisted() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();
        let job = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["tasks"] }),
            now,
        )
        .await
        .unwrap();

        let queued_events = list_backup_job_events(&pool, &job.backup_job_id)
            .await
            .unwrap();
        assert_eq!(queued_events.len(), 1);
        assert_eq!(queued_events[0].event_type, "backup_job_queued");

        let claimed = claim_next_due_scheduled_backup_export_job(&pool, now)
            .await
            .unwrap()
            .expect("due job");
        let succeeded =
            complete_backup_export_job_success(&pool, &claimed.backup_job_id, None, now)
                .await
                .unwrap()
                .expect("completed job");
        assert_eq!(succeeded.status, "succeeded");
        assert!(succeeded.manifest_id.is_none());

        let events = list_backup_job_events(&pool, &job.backup_job_id)
            .await
            .unwrap();
        let event_types: Vec<_> = events
            .iter()
            .map(|event| event.event_type.as_str())
            .collect();
        assert_eq!(event_types.len(), 3);
        assert!(event_types.contains(&"backup_job_queued"));
        assert!(event_types.contains(&"backup_job_started"));
        assert!(event_types.contains(&"backup_job_succeeded"));

        let next_job = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["tasks"] }),
            now,
        )
        .await
        .unwrap();
        assert_ne!(next_job.backup_job_id, job.backup_job_id);
    }

    #[tokio::test]
    async fn scheduled_backup_export_job_failure_records_retry_state() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();
        let job = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["tasks"] }),
            now,
        )
        .await
        .unwrap();
        let claimed = claim_next_due_scheduled_backup_export_job(&pool, now)
            .await
            .unwrap()
            .expect("due job");

        let failed = complete_backup_export_job_failure(
            &pool,
            &claimed.backup_job_id,
            "export_io",
            "target unavailable",
            true,
            now,
        )
        .await
        .unwrap()
        .expect("failed job");

        assert_eq!(failed.status, "failed");
        assert_eq!(failed.last_error_code.as_deref(), Some("export_io"));
        assert_eq!(
            failed.last_error_message.as_deref(),
            Some("target unavailable")
        );
        assert!(failed.next_attempt_at.is_some());
        assert!(
            claim_next_due_scheduled_backup_export_job(&pool, now)
                .await
                .unwrap()
                .is_none(),
            "failed jobs should not be claimable before their retry time"
        );

        let retried =
            claim_next_due_scheduled_backup_export_job(&pool, failed.next_attempt_at.unwrap())
                .await
                .unwrap()
                .expect("retry due");
        assert_eq!(retried.backup_job_id, job.backup_job_id);
        assert_eq!(retried.status, "running");
        assert_eq!(retried.attempt, 2);

        let events = list_backup_job_events(&pool, &job.backup_job_id)
            .await
            .unwrap();
        let event_types: Vec<_> = events
            .iter()
            .map(|event| event.event_type.as_str())
            .collect();
        assert!(event_types.contains(&"backup_job_failed"));
        assert_eq!(events.len(), 4);
    }

    #[tokio::test]
    async fn latest_finished_scheduled_backup_export_job_returns_latest_terminal_job() {
        let pool = test_pool().await;
        let t1 = OffsetDateTime::now_utc();
        let first = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["tasks"] }),
            t1,
        )
        .await
        .unwrap();
        let first_claim = claim_next_due_scheduled_backup_export_job(&pool, t1)
            .await
            .unwrap()
            .expect("first due job");
        complete_backup_export_job_failure(
            &pool,
            &first_claim.backup_job_id,
            "export_io",
            "target unavailable",
            false,
            t1,
        )
        .await
        .unwrap();

        let t2 = t1 + time::Duration::minutes(5);
        let second = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/activity",
            &serde_json::json!({ "domains": ["activity"] }),
            t2,
        )
        .await
        .unwrap();
        let second_claim = claim_next_due_scheduled_backup_export_job(&pool, t2)
            .await
            .unwrap()
            .expect("second due job");
        complete_backup_export_job_success(&pool, &second_claim.backup_job_id, None, t2)
            .await
            .unwrap();

        let latest = get_latest_finished_scheduled_backup_export_job(&pool)
            .await
            .unwrap()
            .expect("latest terminal job");
        assert_eq!(latest.backup_job_id, second.backup_job_id);
        assert_eq!(latest.status, "succeeded");
        assert_eq!(latest.storage_target_root, "/tmp/nas/activity");
        assert_ne!(latest.backup_job_id, first.backup_job_id);
    }

    #[tokio::test]
    async fn latest_finished_scheduled_backup_export_job_surfaces_failed_target() {
        let pool = test_pool().await;
        let now = OffsetDateTime::now_utc();
        let job = queue_scheduled_backup_export_job(
            &pool,
            "/tmp/nas/google",
            &serde_json::json!({ "domains": ["tasks"] }),
            now,
        )
        .await
        .unwrap();
        let claimed = claim_next_due_scheduled_backup_export_job(&pool, now)
            .await
            .unwrap()
            .expect("due job");
        complete_backup_export_job_failure(
            &pool,
            &claimed.backup_job_id,
            "export_io",
            "target unavailable",
            true,
            now,
        )
        .await
        .unwrap();

        let latest = get_latest_finished_scheduled_backup_export_job(&pool)
            .await
            .unwrap()
            .expect("latest terminal job");
        assert_eq!(latest.backup_job_id, job.backup_job_id);
        assert_eq!(latest.status, "failed");
        assert_eq!(latest.last_error_code.as_deref(), Some("export_io"));
        assert_eq!(
            latest.last_error_message.as_deref(),
            Some("target unavailable")
        );
        assert_eq!(latest.storage_target_root, "/tmp/nas/google");
    }
}
