use serde_json::{json, Value as JsonValue};
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    QueryBuilder, Row, Sqlite, SqlitePool,
};
use std::{fs, path::Path};
use std::str::FromStr;
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{
    ArtifactId, ArtifactStorageKind, CaptureId, Commitment, CommitmentId, CommitmentStatus, ContextCapture,
    JobId, JobStatus, OrientationSnapshot, PrivacyClass, Ref, RefRelationType, Run, RunEvent, RunEventType,
    RunId, RunKind, RunStatus, SearchResult, SyncClass,
};

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

#[derive(Debug, Clone)]
pub struct Storage {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct CaptureInsert {
    pub content_text: String,
    pub capture_type: String,
    pub source_device: Option<String>,
    pub privacy_class: PrivacyClass,
}

#[derive(Debug, Clone)]
pub struct SignalInsert {
    pub signal_type: String,
    pub source: String,
    pub timestamp: i64,
    pub payload_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct SignalRecord {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub timestamp: i64,
    pub payload_json: JsonValue,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct InferredStateInsert {
    pub state_name: String,
    pub confidence: Option<String>,
    pub timestamp: i64,
    pub context_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct InferredStateRecord {
    pub state_id: String,
    pub state_name: String,
    pub confidence: Option<String>,
    pub timestamp: i64,
    pub context_json: JsonValue,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct NudgeInsert {
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub related_commitment_id: Option<String>,
    pub message: String,
    pub snoozed_until: Option<i64>,
    pub resolved_at: Option<i64>,
    pub signals_snapshot_json: Option<String>,
    pub inference_snapshot_json: Option<String>,
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct NudgeRecord {
    pub nudge_id: String,
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub related_commitment_id: Option<String>,
    pub message: String,
    pub created_at: i64,
    pub snoozed_until: Option<i64>,
    pub resolved_at: Option<i64>,
    pub signals_snapshot_json: Option<String>,
    pub inference_snapshot_json: Option<String>,
    pub metadata_json: JsonValue,
}

#[derive(Debug, Clone)]
pub struct CommitmentInsert {
    pub text: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: CommitmentStatus,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub capture_type: Option<String>,
    pub source_device: Option<String>,
    pub limit: Option<u32>,
}

/// A job claimed for processing. Caller must eventually call `mark_job_succeeded` or `mark_job_failed`.
#[derive(Debug, Clone)]
pub struct PendingJob {
    pub job_id: JobId,
    pub job_type: String,
    pub payload_json: String,
}

#[derive(Debug, Clone)]
pub struct ArtifactInsert {
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    pub storage_kind: ArtifactStorageKind,
    pub privacy_class: PrivacyClass,
    pub sync_class: SyncClass,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    /// Optional JSON metadata (e.g. generator, context_kind). Stored as TEXT.
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct ArtifactRecord {
    pub artifact_id: ArtifactId,
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    pub storage_kind: ArtifactStorageKind,
    pub privacy_class: String,
    pub sync_class: String,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("invalid timestamp in storage: {0}")]
    InvalidTimestamp(String),
    #[error("validation error: {0}")]
    Validation(String),
}

impl Storage {
    pub async fn connect(db_path: &str) -> Result<Self, StorageError> {
        if db_path != ":memory:" {
            if let Some(parent) = Path::new(db_path).parent() {
                fs::create_dir_all(parent)?;
            }
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(sqlite_connect_options(db_path)?)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), StorageError> {
        MIGRATOR.run(&self.pool).await?;
        let version = self.schema_version().await?;
        let payload = serde_json::json!({ "schema_version": version }).to_string();
        if let Err(e) = self
            .emit_event("SCHEMA_MIGRATION_COMPLETE", "schema", None, &payload)
            .await
        {
            tracing::warn!(error = %e, "failed to emit SCHEMA_MIGRATION_COMPLETE event");
        }
        Ok(())
    }

    pub async fn healthcheck(&self) -> Result<(), StorageError> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    /// Returns the number of applied migrations (schema version). Used by doctor/diagnostics.
    pub async fn schema_version(&self) -> Result<u32, StorageError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0 as u32)
    }

    /// Appends a runtime event to the events table. Idempotent callers should generate event_id themselves if needed.
    pub async fn emit_event(
        &self,
        event_type: &str,
        subject_type: &str,
        subject_id: Option<&str>,
        payload_json: &str,
    ) -> Result<(), StorageError> {
        let event_id = format!("evt_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO events (event_id, event_type, subject_type, subject_id, payload_json, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&event_id)
        .bind(event_type)
        .bind(subject_type)
        .bind(subject_id)
        .bind(payload_json)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_capture(&self, input: CaptureInsert) -> Result<CaptureId, StorageError> {
        let capture_id = CaptureId::new();
        let job_id = JobId::new();
        let now = OffsetDateTime::now_utc();
        let metadata = json!({});

        sqlx::query(
            r#"
            INSERT INTO captures (
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
        .bind(now.unix_timestamp())
        .bind(now.unix_timestamp())
        .bind(input.source_device)
        .bind(input.privacy_class.to_string())
        .bind(metadata.to_string())
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO processing_jobs (
                job_id,
                job_type,
                status,
                created_at,
                started_at,
                finished_at,
                payload_json,
                error_text
            ) VALUES (?, ?, ?, ?, NULL, NULL, ?, NULL)
            "#,
        )
        .bind(job_id.to_string())
        .bind("capture_ingest")
        .bind(JobStatus::Pending.to_string())
        .bind(now.unix_timestamp())
        .bind(json!({ "capture_id": capture_id.to_string() }).to_string())
        .execute(&self.pool)
        .await?;

        Ok(capture_id)
    }

    pub async fn capture_count(&self) -> Result<i64, StorageError> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM captures")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    pub async fn get_capture_by_id(
        &self,
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
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(map_context_capture_row(row)?))
    }

    /// List captures most recent first. If today_only, restrict to since start of current day (UTC).
    pub async fn list_captures_recent(
        &self,
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
            .fetch_all(&self.pool)
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
            .fetch_all(&self.pool)
            .await?
        };
        rows.into_iter()
            .map(|row| map_context_capture_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn insert_commitment(&self, input: CommitmentInsert) -> Result<CommitmentId, StorageError> {
        let id = CommitmentId::new();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let metadata_str = serde_json::to_string(input.metadata_json.as_ref().unwrap_or(&json!({})))
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        let due_ts = input.due_at.map(|t| t.unix_timestamp());
        sqlx::query(
            r#"
            INSERT INTO commitments (id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, ?)
            "#,
        )
        .bind(id.as_ref())
        .bind(&input.text)
        .bind(&input.source_type)
        .bind(&input.source_id)
        .bind(input.status.to_string())
        .bind(due_ts)
        .bind(&input.project)
        .bind(&input.commitment_kind)
        .bind(now)
        .bind(&metadata_str)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn get_commitment_by_id(&self, id: &str) -> Result<Option<Commitment>, StorageError> {
        let row = sqlx::query(
            r#"SELECT id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json FROM commitments WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(map_commitment_row(&row)?))
    }

    pub async fn list_commitments(
        &self,
        status_filter: Option<CommitmentStatus>,
        project: Option<&str>,
        kind: Option<&str>,
        limit: u32,
    ) -> Result<Vec<Commitment>, StorageError> {
        let limit = limit.min(200) as i64;
        let rows = sqlx::query(
            r#"
            SELECT id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json
            FROM commitments
            WHERE (? IS NULL OR status = ?)
              AND (? IS NULL OR project = ?)
              AND (? IS NULL OR commitment_kind = ?)
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(status_filter.as_ref().map(|s| s.to_string()))
        .bind(status_filter.as_ref().map(|s| s.to_string()))
        .bind(project)
        .bind(project)
        .bind(kind)
        .bind(kind)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_commitment_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn update_commitment(
        &self,
        id: &str,
        status: Option<CommitmentStatus>,
        due_at: Option<Option<OffsetDateTime>>,
        project: Option<&str>,
        commitment_kind: Option<&str>,
        metadata_json: Option<&JsonValue>,
    ) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let resolved = status.and_then(|s| (s == CommitmentStatus::Done || s == CommitmentStatus::Cancelled).then_some(now));
        let current: Option<Commitment> = self.get_commitment_by_id(id).await?;
        let Some(c) = current else {
            return Err(StorageError::Validation("commitment not found".to_string()));
        };
        let new_status = status.unwrap_or(c.status);
        let new_due = due_at.unwrap_or(c.due_at).map(|t| t.unix_timestamp());
        let new_project = project.map(String::from).or(c.project);
        let new_kind = commitment_kind.map(String::from).or(c.commitment_kind);
        let new_resolved = resolved.or(c.resolved_at.map(|t| t.unix_timestamp()));
        let meta = metadata_json
            .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| c.metadata_json.to_string());
        sqlx::query(
            r#"
            UPDATE commitments SET status = ?, due_at = ?, project = ?, commitment_kind = ?, resolved_at = ?, metadata_json = ?
            WHERE id = ?
            "#,
        )
        .bind(new_status.to_string())
        .bind(new_due)
        .bind(&new_project)
        .bind(&new_kind)
        .bind(new_resolved)
        .bind(&meta)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Commitment dependencies ---

    pub async fn insert_commitment_dependency(
        &self,
        parent_commitment_id: &str,
        child_commitment_id: &str,
        dependency_type: &str,
    ) -> Result<String, StorageError> {
        let existing = sqlx::query_as::<_, (String,)>(
            r#"SELECT id FROM commitment_dependencies WHERE parent_commitment_id = ? AND child_commitment_id = ? AND dependency_type = ?"#,
        )
        .bind(parent_commitment_id)
        .bind(child_commitment_id)
        .bind(dependency_type)
        .fetch_optional(&self.pool)
        .await?;
        if let Some((id,)) = existing {
            return Ok(id);
        }
        let id = format!("cdep_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO commitment_dependencies (id, parent_commitment_id, child_commitment_id, dependency_type, created_at) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(parent_commitment_id)
        .bind(child_commitment_id)
        .bind(dependency_type)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_commitment_dependencies_by_parent(
        &self,
        parent_commitment_id: &str,
    ) -> Result<Vec<(String, String, String, i64)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, String, i64)>(
            r#"SELECT id, child_commitment_id, dependency_type, created_at FROM commitment_dependencies WHERE parent_commitment_id = ? ORDER BY created_at ASC"#,
        )
        .bind(parent_commitment_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn list_commitment_dependencies_by_child(
        &self,
        child_commitment_id: &str,
    ) -> Result<Vec<(String, String, String, i64)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, String, i64)>(
            r#"SELECT id, parent_commitment_id, dependency_type, created_at FROM commitment_dependencies WHERE child_commitment_id = ? ORDER BY created_at ASC"#,
        )
        .bind(child_commitment_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- Signals (Phase B) ---

    pub async fn insert_signal(&self, input: SignalInsert) -> Result<String, StorageError> {
        let signal_id = format!("sig_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let payload_str = serde_json::to_string(input.payload_json.as_ref().unwrap_or(&json!({})))
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"INSERT INTO signals (signal_id, signal_type, source, timestamp, payload_json, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&signal_id)
        .bind(&input.signal_type)
        .bind(&input.source)
        .bind(input.timestamp)
        .bind(&payload_str)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(signal_id)
    }

    pub async fn list_signals(
        &self,
        signal_type: Option<&str>,
        since_ts: Option<i64>,
        limit: u32,
    ) -> Result<Vec<SignalRecord>, StorageError> {
        let limit = limit.min(500) as i64;
        let rows = sqlx::query(
            r#"
            SELECT signal_id, signal_type, source, timestamp, payload_json, created_at
            FROM signals
            WHERE (? IS NULL OR signal_type = ?) AND (? IS NULL OR timestamp >= ?)
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(signal_type)
        .bind(signal_type)
        .bind(since_ts)
        .bind(since_ts)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(map_signal_row).collect()
    }

    // --- Inferred state (Phase C) ---

    pub async fn insert_inferred_state(&self, input: InferredStateInsert) -> Result<String, StorageError> {
        let state_id = format!("ist_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let context_str = serde_json::to_string(input.context_json.as_ref().unwrap_or(&json!({})))
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"INSERT INTO inferred_state (state_id, state_name, confidence, timestamp, context_json, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&state_id)
        .bind(&input.state_name)
        .bind(&input.confidence)
        .bind(input.timestamp)
        .bind(&context_str)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(state_id)
    }

    pub async fn list_inferred_state_recent(&self, state_name: Option<&str>, limit: u32) -> Result<Vec<InferredStateRecord>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT state_id, state_name, confidence, timestamp, context_json, created_at
            FROM inferred_state
            WHERE (? IS NULL OR state_name = ?)
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(state_name)
        .bind(state_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(map_inferred_state_row).collect()
    }

    // --- Nudges (Phase D) ---

    pub async fn insert_nudge(&self, input: NudgeInsert) -> Result<String, StorageError> {
        let nudge_id = format!("nud_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let meta_str = serde_json::to_string(input.metadata_json.as_ref().unwrap_or(&json!({})))
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO nudges (nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&nudge_id)
        .bind(&input.nudge_type)
        .bind(&input.level)
        .bind(&input.state)
        .bind(&input.related_commitment_id)
        .bind(&input.message)
        .bind(now)
        .bind(input.snoozed_until)
        .bind(input.resolved_at)
        .bind(&input.signals_snapshot_json)
        .bind(&input.inference_snapshot_json)
        .bind(&meta_str)
        .execute(&self.pool)
        .await?;
        Ok(nudge_id)
    }

    pub async fn get_nudge(&self, id: &str) -> Result<Option<NudgeRecord>, StorageError> {
        let row = sqlx::query(
            r#"SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json FROM nudges WHERE nudge_id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| map_nudge_row(&r)).transpose()
    }

    pub async fn list_nudges(&self, state_filter: Option<&str>, limit: u32) -> Result<Vec<NudgeRecord>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json
            FROM nudges
            WHERE (? IS NULL OR state = ?)
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(state_filter)
        .bind(state_filter)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(|r| map_nudge_row(&r)).collect()
    }

    pub async fn update_nudge_state(
        &self,
        nudge_id: &str,
        state: &str,
        snoozed_until: Option<i64>,
        resolved_at: Option<i64>,
    ) -> Result<(), StorageError> {
        sqlx::query(
            r#"UPDATE nudges SET state = ?, snoozed_until = ?, resolved_at = ? WHERE nudge_id = ?"#,
        )
        .bind(state)
        .bind(snoozed_until)
        .bind(resolved_at)
        .bind(nudge_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Current context (singleton) ---

    pub async fn get_current_context(&self) -> Result<Option<(i64, String)>, StorageError> {
        let row = sqlx::query_as::<_, (i64, String)>(
            r#"SELECT computed_at, context_json FROM current_context WHERE id = 1"#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn set_current_context(&self, computed_at: i64, context_json: &str) -> Result<(), StorageError> {
        sqlx::query(
            r#"INSERT OR REPLACE INTO current_context (id, computed_at, context_json) VALUES (1, ?, ?)"#,
        )
        .bind(computed_at)
        .bind(context_json)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_context_timeline(
        &self,
        timestamp: i64,
        context_json: &str,
        trigger_signal_id: Option<&str>,
    ) -> Result<(), StorageError> {
        let id = format!("ctl_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO context_timeline (id, timestamp, context_json, trigger_signal_id, created_at) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(timestamp)
        .bind(context_json)
        .bind(trigger_signal_id)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_context_timeline(&self, limit: u32) -> Result<Vec<(String, i64, String)>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query_as::<_, (String, i64, String)>(
            r#"SELECT id, timestamp, context_json FROM context_timeline ORDER BY timestamp DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- Threads (thread graph) ---

    pub async fn insert_thread(
        &self,
        id: &str,
        thread_type: &str,
        title: &str,
        status: &str,
        metadata_json: &str,
    ) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO threads (id, thread_type, title, status, metadata_json, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(id)
        .bind(thread_type)
        .bind(title)
        .bind(status)
        .bind(metadata_json)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_thread_by_id(&self, id: &str) -> Result<Option<(String, String, String, String, String, i64, i64)>, StorageError> {
        let row = sqlx::query_as::<_, (String, String, String, String, String, i64, i64)>(
            r#"SELECT id, thread_type, title, status, metadata_json, created_at, updated_at FROM threads WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_threads(&self, status_filter: Option<&str>, limit: u32) -> Result<Vec<(String, String, String, String, i64, i64)>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = if let Some(s) = status_filter {
            sqlx::query_as::<_, (String, String, String, String, i64, i64)>(
                r#"SELECT id, thread_type, title, status, created_at, updated_at FROM threads WHERE status = ? ORDER BY updated_at DESC LIMIT ?"#,
            )
            .bind(s)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, (String, String, String, String, i64, i64)>(
                r#"SELECT id, thread_type, title, status, created_at, updated_at FROM threads ORDER BY updated_at DESC LIMIT ?"#,
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };
        Ok(rows)
    }

    pub async fn update_thread_status(&self, id: &str, status: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(r#"UPDATE threads SET status = ?, updated_at = ? WHERE id = ?"#)
            .bind(status)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_thread_link(
        &self,
        thread_id: &str,
        entity_type: &str,
        entity_id: &str,
        relation_type: &str,
    ) -> Result<String, StorageError> {
        let id = format!("tl_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT OR IGNORE INTO thread_links (id, thread_id, entity_type, entity_id, relation_type, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(thread_id)
        .bind(entity_type)
        .bind(entity_id)
        .bind(relation_type)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_thread_links(&self, thread_id: &str) -> Result<Vec<(String, String, String, String)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, String, String)>(
            r#"SELECT id, entity_type, entity_id, relation_type FROM thread_links WHERE thread_id = ? ORDER BY created_at ASC"#,
        )
        .bind(thread_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- Suggestions (steering loop) ---

    pub async fn insert_suggestion(
        &self,
        suggestion_type: &str,
        state: &str,
        payload_json: &str,
    ) -> Result<String, StorageError> {
        let id = format!("sug_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO suggestions (id, suggestion_type, state, payload_json, created_at) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(suggestion_type)
        .bind(state)
        .bind(payload_json)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_suggestions(
        &self,
        state_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<(String, String, String, String, i64, Option<i64>)>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = if let Some(s) = state_filter {
            sqlx::query_as::<_, (String, String, String, String, i64, Option<i64>)>(
                r#"SELECT id, suggestion_type, state, payload_json, created_at, resolved_at FROM suggestions WHERE state = ? ORDER BY created_at DESC LIMIT ?"#,
            )
            .bind(s)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, (String, String, String, String, i64, Option<i64>)>(
                r#"SELECT id, suggestion_type, state, payload_json, created_at, resolved_at FROM suggestions ORDER BY created_at DESC LIMIT ?"#,
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };
        Ok(rows)
    }

    pub async fn get_suggestion_by_id(&self, id: &str) -> Result<Option<(String, String, String, String, i64, Option<i64>)>, StorageError> {
        let row = sqlx::query_as::<_, (String, String, String, String, i64, Option<i64>)>(
            r#"SELECT id, suggestion_type, state, payload_json, created_at, resolved_at FROM suggestions WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update_suggestion_state(
        &self,
        id: &str,
        state: &str,
        resolved_at: Option<i64>,
        payload_json: Option<&str>,
    ) -> Result<(), StorageError> {
        if let Some(payload) = payload_json {
            sqlx::query(
                r#"UPDATE suggestions SET state = ?, resolved_at = ?, payload_json = ? WHERE id = ?"#,
            )
            .bind(state)
            .bind(resolved_at)
            .bind(payload)
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(
                r#"UPDATE suggestions SET state = ?, resolved_at = ? WHERE id = ?"#,
            )
            .bind(state)
            .bind(resolved_at)
            .bind(id)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    // --- Commitment risk (risk engine) ---

    pub async fn insert_commitment_risk(
        &self,
        commitment_id: &str,
        risk_score: f64,
        risk_level: &str,
        factors_json: &str,
        computed_at: i64,
    ) -> Result<String, StorageError> {
        let id = format!("risk_{}", Uuid::new_v4().simple());
        sqlx::query(
            r#"INSERT INTO commitment_risk (id, commitment_id, risk_score, risk_level, factors_json, computed_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(commitment_id)
        .bind(risk_score)
        .bind(risk_level)
        .bind(factors_json)
        .bind(computed_at)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_commitment_risk_recent(&self, commitment_id: &str, limit: u32) -> Result<Vec<(String, f64, String, String, i64)>, StorageError> {
        let limit = limit.min(50) as i64;
        let rows = sqlx::query_as::<_, (String, f64, String, String, i64)>(
            r#"SELECT id, risk_score, risk_level, factors_json, computed_at FROM commitment_risk WHERE commitment_id = ? ORDER BY computed_at DESC LIMIT ?"#,
        )
        .bind(commitment_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Latest risk snapshot per commitment (for listing current risk).
    pub async fn list_commitment_risk_latest_all(&self) -> Result<Vec<(String, String, f64, String, String, i64)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, f64, String, String, i64)>(
            r#"SELECT cr.id, cr.commitment_id, cr.risk_score, cr.risk_level, cr.factors_json, cr.computed_at
               FROM commitment_risk cr
               INNER JOIN (
                 SELECT commitment_id, MAX(computed_at) AS max_at FROM commitment_risk GROUP BY commitment_id
               ) latest ON cr.commitment_id = latest.commitment_id AND cr.computed_at = latest.max_at
               ORDER BY cr.risk_score DESC"#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- Nudge events (append-only log) ---

    pub async fn insert_nudge_event(
        &self,
        nudge_id: &str,
        event_type: &str,
        payload_json: &str,
        timestamp: i64,
    ) -> Result<(), StorageError> {
        let id = format!("nve_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO nudge_events (id, nudge_id, event_type, payload_json, timestamp, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(nudge_id)
        .bind(event_type)
        .bind(payload_json)
        .bind(timestamp)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn search_captures(
        &self,
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

        let rows = builder.build().fetch_all(&self.pool).await?;
        rows.into_iter().map(map_search_row).collect()
    }

    /// Claims the next pending job of the given type. Returns `None` if no pending job exists.
    /// The job is marked `running` and `started_at` is set. Caller must call `mark_job_succeeded` or `mark_job_failed`.
    pub async fn claim_next_pending_job(
        &self,
        job_type: &str,
    ) -> Result<Option<PendingJob>, StorageError> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query(
            r#"
            SELECT job_id, payload_json
            FROM processing_jobs
            WHERE job_type = ? AND status = ?
            ORDER BY created_at ASC
            LIMIT 1
            "#,
        )
        .bind(job_type)
        .bind(JobStatus::Pending.to_string())
        .fetch_optional(&mut *tx)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let job_id: String = row.try_get("job_id")?;
        let payload_json: String = row.try_get("payload_json")?;

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let updated = sqlx::query(
            r#"
            UPDATE processing_jobs
            SET status = ?, started_at = ?
            WHERE job_id = ? AND status = ?
            "#,
        )
        .bind(JobStatus::Running.to_string())
        .bind(now)
        .bind(&job_id)
        .bind(JobStatus::Pending.to_string())
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        if updated.rows_affected() == 0 {
            return Ok(None);
        }

        Ok(Some(PendingJob {
            job_id: JobId::from(job_id),
            job_type: job_type.to_string(),
            payload_json,
        }))
    }

    pub async fn mark_job_succeeded(&self, job_id: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            UPDATE processing_jobs
            SET status = ?, finished_at = ?, error_text = NULL
            WHERE job_id = ?
            "#,
        )
        .bind(JobStatus::Succeeded.to_string())
        .bind(now)
        .bind(job_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn mark_job_failed(&self, job_id: &str, error: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            UPDATE processing_jobs
            SET status = ?, finished_at = ?, error_text = ?
            WHERE job_id = ?
            "#,
        )
        .bind(JobStatus::Failed.to_string())
        .bind(now)
        .bind(error)
        .bind(job_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_artifact(&self, input: ArtifactInsert) -> Result<ArtifactId, StorageError> {
        let artifact_id = ArtifactId::new();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO artifacts (
                artifact_id,
                artifact_type,
                title,
                mime_type,
                storage_uri,
                storage_kind,
                privacy_class,
                sync_class,
                content_hash,
                size_bytes,
                created_at,
                updated_at,
                metadata_json
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(artifact_id.to_string())
        .bind(&input.artifact_type)
        .bind(&input.title)
        .bind(&input.mime_type)
        .bind(&input.storage_uri)
        .bind(input.storage_kind.to_string())
        .bind(input.privacy_class.to_string())
        .bind(input.sync_class.to_string())
        .bind(&input.content_hash)
        .bind(input.size_bytes)
        .bind(now)
        .bind(now)
        .bind(
            input
                .metadata_json
                .as_ref()
                .and_then(|v| serde_json::to_string(v).ok())
                .as_deref()
                .unwrap_or("{}"),
        )
        .execute(&self.pool)
        .await?;
        Ok(artifact_id)
    }

    pub async fn get_artifact_by_id(
        &self,
        artifact_id: &str,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
                   privacy_class, sync_class, content_hash, size_bytes, created_at, updated_at
            FROM artifacts
            WHERE artifact_id = ?
            "#,
        )
        .bind(artifact_id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let storage_kind_str: String = row.try_get("storage_kind")?;
        let storage_kind = storage_kind_str
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?;

        Ok(Some(ArtifactRecord {
            artifact_id: ArtifactId::from(row.try_get::<String, _>("artifact_id")?),
            artifact_type: row.try_get("artifact_type")?,
            title: row.try_get("title")?,
            mime_type: row.try_get("mime_type")?,
            storage_uri: row.try_get("storage_uri")?,
            storage_kind,
            privacy_class: row.try_get("privacy_class")?,
            sync_class: row.try_get("sync_class")?,
            content_hash: row.try_get("content_hash")?,
            size_bytes: row.try_get("size_bytes")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        }))
    }

    /// Returns the most recently created artifact of the given type, if any.
    pub async fn get_latest_artifact_by_type(
        &self,
        artifact_type: &str,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
                   privacy_class, sync_class, content_hash, size_bytes, created_at, updated_at
            FROM artifacts
            WHERE artifact_type = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(artifact_type)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let storage_kind_str: String = row.try_get("storage_kind")?;
        let storage_kind = storage_kind_str
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?;

        Ok(Some(ArtifactRecord {
            artifact_id: ArtifactId::from(row.try_get::<String, _>("artifact_id")?),
            artifact_type: row.try_get("artifact_type")?,
            title: row.try_get("title")?,
            mime_type: row.try_get("mime_type")?,
            storage_uri: row.try_get("storage_uri")?,
            storage_kind,
            privacy_class: row.try_get("privacy_class")?,
            sync_class: row.try_get("sync_class")?,
            content_hash: row.try_get("content_hash")?,
            size_bytes: row.try_get("size_bytes")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        }))
    }

    pub async fn create_run(
        &self,
        id: &RunId,
        kind: RunKind,
        input_json: &JsonValue,
    ) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let input_str = serde_json::to_string(input_json).map_err(|e| StorageError::Validation(e.to_string()))?;
        let run_created_payload = json!({ "kind": kind.to_string() });
        let payload_str = serde_json::to_string(&run_created_payload).map_err(|e| StorageError::Validation(e.to_string()))?;
        let event_id = format!("evt_{}", Uuid::new_v4().simple());
        let mut tx = self.pool.begin().await?;
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

    pub async fn get_run_by_id(&self, run_id: &str) -> Result<Option<Run>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT run_id, run_kind, status, input_json, output_json, error_json,
                   created_at, started_at, finished_at
            FROM runs WHERE run_id = ?
            "#,
        )
        .bind(run_id)
        .fetch_optional(&self.pool)
        .await?;
        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(map_run_row(&row)?))
    }

    pub async fn list_runs(&self, limit: u32) -> Result<Vec<Run>, StorageError> {
        let limit = limit.clamp(1, 100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT run_id, run_kind, status, input_json, output_json, error_json,
                   created_at, started_at, finished_at
            FROM runs ORDER BY created_at DESC LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(|row| map_run_row(&row)).collect::<Result<Vec<_>, _>>()
    }

    pub async fn update_run_status(
        &self,
        run_id: &str,
        status: RunStatus,
        started_at: Option<i64>,
        finished_at: Option<i64>,
        output_json: Option<&JsonValue>,
        error_json: Option<&JsonValue>,
    ) -> Result<(), StorageError> {
        let output_str = output_json.map(|v| serde_json::to_string(v).map_err(|e| StorageError::Validation(e.to_string()))).transpose()?;
        let error_str = error_json.map(|v| serde_json::to_string(v).map_err(|e| StorageError::Validation(e.to_string()))).transpose()?;
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
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn append_run_event(
        &self,
        run_id: &str,
        seq: u32,
        event_type: RunEventType,
        payload_json: &JsonValue,
    ) -> Result<(), StorageError> {
        let event_id = format!("evt_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let payload_str = serde_json::to_string(payload_json).map_err(|e| StorageError::Validation(e.to_string()))?;
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
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_run_events(&self, run_id: &str) -> Result<Vec<RunEvent>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT event_id, run_id, seq, event_type, payload_json, created_at
            FROM run_events WHERE run_id = ? ORDER BY seq ASC
            "#,
        )
        .bind(run_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(map_run_event_row).collect::<Result<Vec<_>, _>>()
    }

    pub async fn create_ref(&self, ref_: &Ref) -> Result<(), StorageError> {
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
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_refs_from(
        &self,
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
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(map_ref_row).collect::<Result<Vec<_>, _>>()
    }

    pub async fn list_refs_to(&self, to_type: &str, to_id: &str) -> Result<Vec<Ref>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT ref_id, from_type, from_id, to_type, to_id, relation_type, created_at
            FROM refs WHERE to_type = ? AND to_id = ? ORDER BY created_at ASC
            "#,
        )
        .bind(to_type)
        .bind(to_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(map_ref_row).collect::<Result<Vec<_>, _>>()
    }

    pub async fn orientation_snapshot(&self) -> Result<OrientationSnapshot, StorageError> {
        let now = OffsetDateTime::now_utc();
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
        .fetch_all(&self.pool)
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
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_context_capture_row)
        .collect::<Result<Vec<_>, _>>()?;

        Ok(OrientationSnapshot {
            recent_today,
            recent_week,
        })
    }
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

fn map_context_capture_row(row: sqlx::sqlite::SqliteRow) -> Result<ContextCapture, StorageError> {
    Ok(ContextCapture {
        capture_id: CaptureId::from(row.try_get::<String, _>("capture_id")?),
        capture_type: row.try_get("capture_type")?,
        content_text: row.try_get("content_text")?,
        occurred_at: timestamp_to_datetime(row.try_get("occurred_at")?)?,
        source_device: row.try_get("source_device")?,
    })
}

fn map_commitment_row(row: &sqlx::sqlite::SqliteRow) -> Result<Commitment, StorageError> {
    let status: String = row.try_get("status")?;
    let created_at: i64 = row.try_get("created_at")?;
    let metadata_str: String = row.try_get("metadata_json")?;
    let metadata_json = serde_json::from_str(&metadata_str).unwrap_or_else(|_| json!({}));
    Ok(Commitment {
        id: CommitmentId::from(row.try_get::<String, _>("id")?),
        text: row.try_get("text")?,
        source_type: row.try_get("source_type")?,
        source_id: row.try_get("source_id")?,
        status: status.parse().map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        due_at: row.try_get::<Option<i64>, _>("due_at")?.and_then(|t| timestamp_to_datetime(t).ok()),
        project: row.try_get("project")?,
        commitment_kind: row.try_get("commitment_kind")?,
        created_at: timestamp_to_datetime(created_at)?,
        resolved_at: row.try_get::<Option<i64>, _>("resolved_at")?.and_then(|t| timestamp_to_datetime(t).ok()),
        metadata_json,
    })
}

fn map_signal_row(row: &sqlx::sqlite::SqliteRow) -> Result<SignalRecord, StorageError> {
    let payload_str: String = row.try_get("payload_json")?;
    Ok(SignalRecord {
        signal_id: row.try_get("signal_id")?,
        signal_type: row.try_get("signal_type")?,
        source: row.try_get("source")?,
        timestamp: row.try_get("timestamp")?,
        payload_json: serde_json::from_str(&payload_str).unwrap_or_else(|_| json!({})),
        created_at: row.try_get("created_at")?,
    })
}

fn map_inferred_state_row(row: &sqlx::sqlite::SqliteRow) -> Result<InferredStateRecord, StorageError> {
    let context_str: String = row.try_get("context_json")?;
    Ok(InferredStateRecord {
        state_id: row.try_get("state_id")?,
        state_name: row.try_get("state_name")?,
        confidence: row.try_get("confidence")?,
        timestamp: row.try_get("timestamp")?,
        context_json: serde_json::from_str(&context_str).unwrap_or_else(|_| json!({})),
        created_at: row.try_get("created_at")?,
    })
}

fn map_nudge_row(row: &sqlx::sqlite::SqliteRow) -> Result<NudgeRecord, StorageError> {
    let meta_str: String = row.try_get("metadata_json")?;
    Ok(NudgeRecord {
        nudge_id: row.try_get("nudge_id")?,
        nudge_type: row.try_get("nudge_type")?,
        level: row.try_get("level")?,
        state: row.try_get("state")?,
        related_commitment_id: row.try_get("related_commitment_id")?,
        message: row.try_get("message")?,
        created_at: row.try_get("created_at")?,
        snoozed_until: row.try_get("snoozed_until")?,
        resolved_at: row.try_get("resolved_at")?,
        signals_snapshot_json: row.try_get("signals_snapshot_json")?,
        inference_snapshot_json: row.try_get("inference_snapshot_json")?,
        metadata_json: serde_json::from_str(&meta_str).unwrap_or_else(|_| json!({})),
    })
}

fn parse_json_value(s: &str) -> Result<JsonValue, StorageError> {
    serde_json::from_str(s).map_err(|e| StorageError::Validation(e.to_string()))
}

fn map_run_row(row: &sqlx::sqlite::SqliteRow) -> Result<Run, StorageError> {
    let kind: String = row.try_get("run_kind")?;
    let status: String = row.try_get("status")?;
    let input_str: String = row.try_get("input_json")?;
    let output_str: Option<String> = row.try_get("output_json")?;
    let error_str: Option<String> = row.try_get("error_json")?;
    Ok(Run {
        id: RunId::from(row.try_get::<String, _>("run_id")?),
        kind: kind.parse().map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        status: status.parse().map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        input_json: parse_json_value(&input_str)?,
        output_json: output_str.as_deref().map(parse_json_value).transpose()?,
        error_json: error_str.as_deref().map(parse_json_value).transpose()?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        started_at: row.try_get::<Option<i64>, _>("started_at")?.map(timestamp_to_datetime).transpose()?,
        finished_at: row.try_get::<Option<i64>, _>("finished_at")?.map(timestamp_to_datetime).transpose()?,
    })
}

fn map_run_event_row(row: sqlx::sqlite::SqliteRow) -> Result<RunEvent, StorageError> {
    let event_type: String = row.try_get("event_type")?;
    let payload_str: String = row.try_get("payload_json")?;
    Ok(RunEvent {
        id: row.try_get("event_id")?,
        run_id: RunId::from(row.try_get::<String, _>("run_id")?),
        seq: row.try_get::<i64, _>("seq")? as u32,
        event_type: event_type.parse().map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        payload_json: parse_json_value(&payload_str)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

fn map_ref_row(row: sqlx::sqlite::SqliteRow) -> Result<Ref, StorageError> {
    let relation_type: String = row.try_get("relation_type")?;
    Ok(Ref {
        id: row.try_get("ref_id")?,
        from_type: row.try_get("from_type")?,
        from_id: row.try_get("from_id")?,
        to_type: row.try_get("to_type")?,
        to_id: row.try_get("to_id")?,
        relation_type: relation_type.parse().map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

fn timestamp_to_datetime(timestamp: i64) -> Result<OffsetDateTime, StorageError> {
    OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|error| StorageError::InvalidTimestamp(error.to_string()))
}

fn sqlite_connect_options(db_path: &str) -> Result<SqliteConnectOptions, StorageError> {
    let url = if db_path == ":memory:" {
        "sqlite::memory:".to_string()
    } else if db_path.starts_with("sqlite:") {
        db_path.to_string()
    } else {
        format!("sqlite://{db_path}")
    };

    let options = SqliteConnectOptions::from_str(&url)?.create_if_missing(true);

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrations_apply_and_capture_inserts() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_capture(CaptureInsert {
                content_text: "remember lidar budget".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("test".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();

        assert_eq!(storage.capture_count().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn search_returns_matching_captures_in_relevance_then_recency_order() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        storage
            .insert_capture(CaptureInsert {
                content_text: "lidar budget estimate for q3".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("phone".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();
        storage
            .insert_capture(CaptureInsert {
                content_text: "budget notes that mention lidar twice lidar".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("phone".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();
        storage
            .insert_capture(CaptureInsert {
                content_text: "completely unrelated note".to_string(),
                capture_type: "journal".to_string(),
                source_device: Some("desktop".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();

        let results = storage
            .search_captures(
                "lidar",
                SearchFilters {
                    capture_type: Some("quick_note".to_string()),
                    source_device: Some("phone".to_string()),
                    limit: Some(10),
                },
            )
            .await
            .unwrap();

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|result| result.capture_type == "quick_note"));
        assert!(results.iter().all(|result| result.source_device.as_deref() == Some("phone")));
        assert!(results[0].snippet.to_lowercase().contains("lidar"));
        assert!(results[1].snippet.to_lowercase().contains("lidar"));
    }

    #[tokio::test]
    async fn claim_and_complete_capture_ingest_job() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let _capture_id = storage
            .insert_capture(CaptureInsert {
                content_text: "test note".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: None,
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();

        let job = storage.claim_next_pending_job("capture_ingest").await.unwrap();
        let job = job.expect("one pending job");
        assert_eq!(job.job_type, "capture_ingest");
        assert!(job.payload_json.contains("capture_id"));

        storage.mark_job_succeeded(&job.job_id.to_string()).await.unwrap();

        let again = storage.claim_next_pending_job("capture_ingest").await.unwrap();
        assert!(again.is_none());
    }

    #[tokio::test]
    async fn create_and_get_artifact() {
        use vel_core::SyncClass;

        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let id = storage
            .create_artifact(ArtifactInsert {
                artifact_type: "transcript".to_string(),
                title: Some("Meeting notes".to_string()),
                mime_type: Some("text/plain".to_string()),
                storage_uri: "file:///var/artifacts/transcripts/abc.txt".to_string(),
                storage_kind: ArtifactStorageKind::External,
                privacy_class: PrivacyClass::Private,
                sync_class: SyncClass::Warm,
                content_hash: Some("sha256:abc".to_string()),
                size_bytes: None,
                metadata_json: None,
            })
            .await
            .unwrap();

        let record = storage.get_artifact_by_id(&id.to_string()).await.unwrap();
        let record = record.expect("artifact should exist");
        assert_eq!(record.artifact_id.to_string(), id.to_string());
        assert_eq!(record.artifact_type, "transcript");
        assert_eq!(record.title.as_deref(), Some("Meeting notes"));
        assert_eq!(record.storage_uri, "file:///var/artifacts/transcripts/abc.txt");
        assert_eq!(record.content_hash.as_deref(), Some("sha256:abc"));

        let missing = storage.get_artifact_by_id("art_nonexistent").await.unwrap();
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn create_run_list_runs_get_run_and_events() {
        use vel_core::{RefRelationType, RunKind};

        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(&run_id, RunKind::ContextGeneration, &json!({"context_kind":"today"}))
            .await
            .unwrap();

        let runs = storage.list_runs(10).await.unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].id.to_string(), run_id.to_string());
        assert_eq!(runs[0].status, vel_core::RunStatus::Queued);

        let run = storage.get_run_by_id(run_id.as_ref()).await.unwrap().unwrap();
        assert_eq!(run.kind, RunKind::ContextGeneration);

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type.to_string(), "run_created");

        let ref_ = Ref::new("run", run_id.as_ref(), "artifact", "art_1", RefRelationType::AttachedTo);
        storage.create_ref(&ref_).await.unwrap();
        let from_refs = storage.list_refs_from("run", run_id.as_ref()).await.unwrap();
        assert_eq!(from_refs.len(), 1);
        assert_eq!(from_refs[0].to_id, "art_1");
    }
}
