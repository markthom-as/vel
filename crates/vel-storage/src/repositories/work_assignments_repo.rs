use sqlx::{QueryBuilder, Row, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::{StorageError, WorkAssignmentInsert, WorkAssignmentRecord, WorkAssignmentUpdate};
use vel_core::WorkAssignmentStatus;

pub(crate) async fn insert_work_assignment(
    pool: &SqlitePool,
    assignment: WorkAssignmentInsert,
) -> Result<String, StorageError> {
    let receipt_id = assignment
        .receipt_id
        .unwrap_or_else(|| Uuid::new_v4().simple().to_string());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        INSERT INTO work_assignment_receipts (
            receipt_id,
            work_request_id,
            worker_id,
            worker_class,
            capability,
            status,
            assigned_at,
            last_updated
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&receipt_id)
    .bind(&assignment.work_request_id)
    .bind(&assignment.worker_id)
    .bind(&assignment.worker_class)
    .bind(&assignment.capability)
    .bind(assignment.status.to_string())
    .bind(assignment.assigned_at)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(receipt_id)
}

pub(crate) async fn update_work_assignment(
    pool: &SqlitePool,
    update: WorkAssignmentUpdate,
) -> Result<WorkAssignmentRecord, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        UPDATE work_assignment_receipts
        SET status = ?,
            started_at = ?,
            completed_at = ?,
            result = ?,
            error_message = ?,
            last_updated = ?
        WHERE receipt_id = ?
        "#,
    )
    .bind(update.status.to_string())
    .bind(update.started_at)
    .bind(update.completed_at)
    .bind(update.result)
    .bind(update.error_message)
    .bind(now)
    .bind(&update.receipt_id)
    .execute(pool)
    .await?;

    let row = sqlx::query(
        r#"
        SELECT
            receipt_id,
            work_request_id,
            worker_id,
            worker_class,
            capability,
            status,
            assigned_at,
            started_at,
            completed_at,
            result,
            error_message,
            last_updated
        FROM work_assignment_receipts
        WHERE receipt_id = ?
        "#,
    )
    .bind(&update.receipt_id)
    .fetch_one(pool)
    .await?;

    map_work_assignment_row(&row)
}

pub(crate) async fn set_work_assignment_last_updated(
    pool: &SqlitePool,
    receipt_id: &str,
    last_updated: i64,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        UPDATE work_assignment_receipts
        SET last_updated = ?
        WHERE receipt_id = ?
        "#,
    )
    .bind(last_updated)
    .bind(receipt_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn list_work_assignments(
    pool: &SqlitePool,
    work_request_id: Option<&str>,
    worker_id: Option<&str>,
) -> Result<Vec<WorkAssignmentRecord>, StorageError> {
    let mut query = QueryBuilder::new(
        r#"
        SELECT
            receipt_id,
            work_request_id,
            worker_id,
            worker_class,
            capability,
            status,
            assigned_at,
            started_at,
            completed_at,
            result,
            error_message,
            last_updated
        FROM work_assignment_receipts
        "#,
    );
    query.push(" WHERE 1=1");
    if work_request_id.is_some() {
        query.push(" AND work_request_id = ");
        query.push_bind(work_request_id);
    }
    if worker_id.is_some() {
        query.push(" AND worker_id = ");
        query.push_bind(worker_id);
    }
    query.push(" ORDER BY last_updated DESC");

    let rows = query.build().fetch_all(pool).await?;
    rows.into_iter()
        .map(|row| map_work_assignment_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

fn map_work_assignment_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<WorkAssignmentRecord, StorageError> {
    let status_str: String = row.try_get("status")?;
    let status = status_str
        .parse::<WorkAssignmentStatus>()
        .map_err(|e| StorageError::Validation(format!("invalid work assignment status: {e}")))?;

    Ok(WorkAssignmentRecord {
        receipt_id: row.try_get("receipt_id")?,
        work_request_id: row.try_get("work_request_id")?,
        worker_id: row.try_get("worker_id")?,
        worker_class: row.try_get("worker_class")?,
        capability: row.try_get("capability")?,
        status,
        assigned_at: row.try_get("assigned_at")?,
        started_at: row.try_get("started_at")?,
        completed_at: row.try_get("completed_at")?,
        result: row.try_get("result")?,
        error_message: row.try_get("error_message")?,
        last_updated: row.try_get("last_updated")?,
    })
}
