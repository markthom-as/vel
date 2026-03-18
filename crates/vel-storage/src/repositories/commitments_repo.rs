use serde_json::Value as JsonValue;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{Commitment, CommitmentId, CommitmentStatus};

use crate::db::{CommitmentInsert, StorageError};
use crate::mapping::timestamp_to_datetime;

pub(crate) async fn insert_commitment(
    pool: &SqlitePool,
    input: CommitmentInsert,
) -> Result<CommitmentId, StorageError> {
    let mut tx = pool.begin().await?;
    let id = insert_commitment_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(id)
}

pub(crate) async fn insert_commitment_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &CommitmentInsert,
) -> Result<CommitmentId, StorageError> {
    let id = CommitmentId::new();
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let metadata_str = serde_json::to_string(
        input
            .metadata_json
            .as_ref()
            .unwrap_or(&serde_json::json!({})),
    )
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
    .execute(&mut **tx)
    .await?;

    Ok(id)
}

pub(crate) async fn get_commitment_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<Commitment>, StorageError> {
    let row = sqlx::query(
        r#"SELECT id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json FROM commitments WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(map_commitment_row(&row)?)),
        None => Ok(None),
    }
}

pub(crate) async fn list_commitments(
    pool: &SqlitePool,
    status_filter: Option<CommitmentStatus>,
    project: Option<&str>,
    kind: Option<&str>,
    limit: u32,
) -> Result<Vec<Commitment>, StorageError> {
    let limit = limit.min(200) as i64;
    let status_str = status_filter.map(|s| s.to_string());

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
    .bind(&status_str)
    .bind(&status_str)
    .bind(project)
    .bind(project)
    .bind(kind)
    .bind(kind)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_commitment_row(&row))
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn update_commitment(
    pool: &SqlitePool,
    id: &str,
    text: Option<&str>,
    status: Option<CommitmentStatus>,
    due_at: Option<Option<OffsetDateTime>>,
    project: Option<&str>,
    commitment_kind: Option<&str>,
    metadata_json: Option<&JsonValue>,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let resolved = status.and_then(|s| {
        (s == CommitmentStatus::Done || s == CommitmentStatus::Cancelled).then_some(now)
    });
    let current = get_commitment_by_id(pool, id).await?;
    let Some(c) = current else {
        return Err(StorageError::Validation("commitment not found".to_string()));
    };

    let new_text = text.map(String::from).unwrap_or(c.text);
    let new_status = status.unwrap_or(c.status);
    let new_due = due_at.unwrap_or(c.due_at).map(|t| t.unix_timestamp());
    let new_project = project.map(String::from).or(c.project);
    let new_kind = commitment_kind.map(String::from).or(c.commitment_kind);
    let new_resolved = match status {
        Some(CommitmentStatus::Open) => None,
        Some(_) => resolved,
        None => c.resolved_at.map(|t| t.unix_timestamp()),
    };
    let meta = metadata_json
        .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()))
        .unwrap_or_else(|| c.metadata_json.to_string());

    sqlx::query(
        r#"
        UPDATE commitments SET text = ?, status = ?, due_at = ?, project = ?, commitment_kind = ?, resolved_at = ?, metadata_json = ?
        WHERE id = ?
        "#,
    )
    .bind(&new_text)
    .bind(new_status.to_string())
    .bind(new_due)
    .bind(&new_project)
    .bind(&new_kind)
    .bind(new_resolved)
    .bind(&meta)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub(crate) async fn insert_commitment_dependency(
    pool: &SqlitePool,
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
    .fetch_optional(pool)
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
    .execute(pool)
    .await?;

    Ok(id)
}

pub(crate) async fn list_commitment_dependencies_by_parent(
    pool: &SqlitePool,
    parent_commitment_id: &str,
) -> Result<Vec<(String, String, String, i64)>, StorageError> {
    let rows = sqlx::query_as::<_, (String, String, String, i64)>(
        r#"SELECT id, child_commitment_id, dependency_type, created_at FROM commitment_dependencies WHERE parent_commitment_id = ? ORDER BY created_at ASC"#,
    )
    .bind(parent_commitment_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub(crate) async fn list_commitment_dependencies_by_child(
    pool: &SqlitePool,
    child_commitment_id: &str,
) -> Result<Vec<(String, String, String, i64)>, StorageError> {
    let rows = sqlx::query_as::<_, (String, String, String, i64)>(
        r#"SELECT id, parent_commitment_id, dependency_type, created_at FROM commitment_dependencies WHERE child_commitment_id = ? ORDER BY created_at ASC"#,
    )
    .bind(child_commitment_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

fn map_commitment_row(row: &sqlx::sqlite::SqliteRow) -> Result<Commitment, StorageError> {
    let status: String = row.try_get("status")?;
    let created_at: i64 = row.try_get("created_at")?;
    let metadata_str: String = row.try_get("metadata_json")?;
    let metadata_json =
        serde_json::from_str(&metadata_str).unwrap_or_else(|_| serde_json::json!({}));
    Ok(Commitment {
        id: CommitmentId::from(row.try_get::<String, _>("id")?),
        text: row.try_get("text")?,
        source_type: row.try_get("source_type")?,
        source_id: row.try_get("source_id")?,
        status: status
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        due_at: row
            .try_get::<Option<i64>, _>("due_at")?
            .and_then(|t| timestamp_to_datetime(t).ok()),
        project: row.try_get("project")?,
        commitment_kind: row.try_get("commitment_kind")?,
        created_at: timestamp_to_datetime(created_at)?,
        resolved_at: row
            .try_get::<Option<i64>, _>("resolved_at")?
            .and_then(|t| timestamp_to_datetime(t).ok()),
        metadata_json,
    })
}
