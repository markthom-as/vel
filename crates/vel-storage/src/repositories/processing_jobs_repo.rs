use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use vel_core::{JobId, JobStatus};

use crate::db::{PendingJob, StorageError};

/// Claims the next pending job of the given type.
/// Returns `None` if no pending job exists.
pub(crate) async fn claim_next_pending_job(
    pool: &SqlitePool,
    job_type: &str,
) -> Result<Option<PendingJob>, StorageError> {
    let mut tx = pool.begin().await?;
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

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn claim_next_pending_job_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    job_type: &str,
) -> Result<Option<PendingJob>, StorageError> {
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
    .fetch_optional(&mut **tx)
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
    .execute(&mut **tx)
    .await?;

    if updated.rows_affected() == 0 {
        return Ok(None);
    }

    Ok(Some(PendingJob {
        job_id: JobId::from(job_id),
        job_type: job_type.to_string(),
        payload_json,
    }))
}

pub(crate) async fn mark_job_succeeded(
    pool: &SqlitePool,
    job_id: &str,
) -> Result<(), StorageError> {
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
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn mark_job_succeeded_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    job_id: &str,
) -> Result<(), StorageError> {
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
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub(crate) async fn mark_job_failed(
    pool: &SqlitePool,
    job_id: &str,
    error: &str,
) -> Result<(), StorageError> {
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
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn mark_job_failed_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    job_id: &str,
    error: &str,
) -> Result<(), StorageError> {
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
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    async fn seed_pending_job(pool: &SqlitePool, job_type: &str) -> String {
        let job_id = format!("job_{}", uuid::Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO processing_jobs (
                job_id, job_type, status, payload_json, created_at
            ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&job_id)
        .bind(job_type)
        .bind(JobStatus::Pending.to_string())
        .bind(json!({"capture_id":"cap_test"}).to_string())
        .bind(now)
        .execute(pool)
        .await
        .unwrap();
        job_id
    }

    #[tokio::test]
    async fn claim_next_pending_job_returns_oldest_pending_for_type() {
        let pool = test_pool().await;
        let _job_a = seed_pending_job(&pool, "capture_ingest").await;
        let _job_b = seed_pending_job(&pool, "capture_ingest").await;

        let claimed = claim_next_pending_job(&pool, "capture_ingest")
            .await
            .unwrap()
            .expect("claim one");

        let status: String =
            sqlx::query_scalar("SELECT status FROM processing_jobs WHERE job_id = ?")
                .bind(claimed.job_id.to_string())
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(status, JobStatus::Running.to_string());
    }

    #[tokio::test]
    async fn mark_job_succeeded_and_failed_in_tx_rollback() {
        let pool = test_pool().await;
        let job_id = seed_pending_job(&pool, "capture_ingest").await;

        {
            let mut tx = pool.begin().await.unwrap();
            mark_job_succeeded_in_tx(&mut tx, &job_id).await.unwrap();
            tx.rollback().await.unwrap();
        }
        let status_after_rollback: String =
            sqlx::query_scalar("SELECT status FROM processing_jobs WHERE job_id = ?")
                .bind(&job_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(status_after_rollback, JobStatus::Pending.to_string());

        {
            let mut tx = pool.begin().await.unwrap();
            mark_job_failed_in_tx(&mut tx, &job_id, "boom")
                .await
                .unwrap();
            tx.commit().await.unwrap();
        }
        let status_after_commit: String =
            sqlx::query_scalar("SELECT status FROM processing_jobs WHERE job_id = ?")
                .bind(&job_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(status_after_commit, JobStatus::Failed.to_string());
    }

    #[tokio::test]
    async fn claim_next_pending_job_in_tx_rolls_back() {
        let pool = test_pool().await;
        let job_id = seed_pending_job(&pool, "capture_ingest").await;

        {
            let mut tx = pool.begin().await.unwrap();
            let claimed = claim_next_pending_job_in_tx(&mut tx, "capture_ingest")
                .await
                .unwrap();
            assert!(claimed.is_some());
            tx.rollback().await.unwrap();
        }

        let status: String =
            sqlx::query_scalar("SELECT status FROM processing_jobs WHERE job_id = ?")
                .bind(&job_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(status, JobStatus::Pending.to_string());
    }
}
