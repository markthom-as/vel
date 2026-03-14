use serde_json::json;
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, QueryBuilder, Row, Sqlite, SqlitePool};
use std::{fs, path::Path};
use time::OffsetDateTime;
use vel_api_types::{ContextCapture, SearchResult};
use vel_core::{CaptureId, JobId, JobStatus, PrivacyClass};

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

#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub capture_type: Option<String>,
    pub source_device: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct OrientationSnapshot {
    pub recent_today: Vec<ContextCapture>,
    pub recent_week: Vec<ContextCapture>,
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
            .connect(&sqlite_url(db_path))
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), StorageError> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }

    pub async fn healthcheck(&self) -> Result<(), StorageError> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
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

fn timestamp_to_datetime(timestamp: i64) -> Result<OffsetDateTime, StorageError> {
    OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|error| StorageError::InvalidTimestamp(error.to_string()))
}

fn sqlite_url(db_path: &str) -> String {
    if db_path == ":memory:" {
        "sqlite::memory:".to_string()
    } else if db_path.starts_with("sqlite:") {
        db_path.to_string()
    } else {
        format!("sqlite://{db_path}")
    }
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
}
