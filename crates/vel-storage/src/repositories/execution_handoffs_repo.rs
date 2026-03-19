use serde_json::Value as JsonValue;
use sqlx::{QueryBuilder, Row, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::ProjectId;

use crate::{db::StorageError, mapping::timestamp_to_datetime};

type ExecutionHandoffRow = (
    String,
    String,
    JsonValue,
    JsonValue,
    String,
    String,
    Option<String>,
    String,
    Option<String>,
    Option<String>,
    Option<OffsetDateTime>,
    Option<OffsetDateTime>,
    OffsetDateTime,
    OffsetDateTime,
);

pub(crate) async fn create_execution_handoff(
    pool: &SqlitePool,
    project_id: &ProjectId,
    handoff_json: &JsonValue,
    task_kind: &str,
    agent_profile: &str,
    token_budget: &str,
    review_gate: &str,
    origin_kind: &str,
    review_state: &str,
    routing_json: &JsonValue,
    manifest_id: Option<&str>,
    requested_by: &str,
    now: OffsetDateTime,
) -> Result<String, StorageError> {
    let id = format!("xho_{}", Uuid::new_v4().simple());
    sqlx::query(
        r#"
        INSERT INTO execution_handoffs (
            id,
            project_id,
            handoff_json,
            task_kind,
            agent_profile,
            token_budget,
            review_gate,
            origin_kind,
            review_state,
            routing_json,
            manifest_id,
            requested_by,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(project_id.as_ref())
    .bind(serde_json::to_string(handoff_json)?)
    .bind(task_kind)
    .bind(agent_profile)
    .bind(token_budget)
    .bind(review_gate)
    .bind(origin_kind)
    .bind(review_state)
    .bind(serde_json::to_string(routing_json)?)
    .bind(manifest_id)
    .bind(requested_by)
    .bind(now.unix_timestamp())
    .bind(now.unix_timestamp())
    .execute(pool)
    .await?;

    Ok(id)
}

pub(crate) async fn list_execution_handoffs(
    pool: &SqlitePool,
    project_id: Option<&str>,
    review_state: Option<&str>,
) -> Result<Vec<ExecutionHandoffRow>, StorageError> {
    let mut builder = QueryBuilder::new(
        r#"
        SELECT
            id,
            project_id,
            handoff_json,
            routing_json,
            origin_kind,
            review_state,
            manifest_id,
            requested_by,
            reviewed_by,
            decision_reason,
            reviewed_at,
            launched_at,
            created_at,
            updated_at
        FROM execution_handoffs
        "#,
    );

    let mut has_where = false;
    if project_id.is_some() || review_state.is_some() {
        builder.push(" WHERE ");
        has_where = true;
    }
    if let Some(project_id) = project_id {
        builder.push("project_id = ");
        builder.push_bind(project_id);
    }
    if let Some(review_state) = review_state {
        if project_id.is_some() && has_where {
            builder.push(" AND ");
        }
        builder.push("review_state = ");
        builder.push_bind(review_state);
    }
    builder.push(" ORDER BY updated_at DESC, created_at DESC");

    let rows = builder.build().fetch_all(pool).await?;
    rows.iter().map(map_execution_handoff_row).collect()
}

pub(crate) async fn get_execution_handoff(
    pool: &SqlitePool,
    handoff_id: &str,
) -> Result<Option<ExecutionHandoffRow>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            project_id,
            handoff_json,
            routing_json,
            origin_kind,
            review_state,
            manifest_id,
            requested_by,
            reviewed_by,
            decision_reason,
            reviewed_at,
            launched_at,
            created_at,
            updated_at
        FROM execution_handoffs
        WHERE id = ?
        "#,
    )
    .bind(handoff_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_execution_handoff_row).transpose()
}

pub(crate) async fn update_execution_handoff_review(
    pool: &SqlitePool,
    handoff_id: &str,
    review_state: &str,
    reviewed_by: Option<&str>,
    decision_reason: Option<&str>,
    reviewed_at: Option<OffsetDateTime>,
    launched_at: Option<OffsetDateTime>,
    now: OffsetDateTime,
) -> Result<Option<ExecutionHandoffRow>, StorageError> {
    let result = sqlx::query(
        r#"
        UPDATE execution_handoffs
        SET review_state = ?,
            reviewed_by = ?,
            decision_reason = ?,
            reviewed_at = ?,
            launched_at = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(review_state)
    .bind(reviewed_by)
    .bind(decision_reason)
    .bind(reviewed_at.map(|value| value.unix_timestamp()))
    .bind(launched_at.map(|value| value.unix_timestamp()))
    .bind(now.unix_timestamp())
    .bind(handoff_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    get_execution_handoff(pool, handoff_id).await
}

fn map_execution_handoff_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<ExecutionHandoffRow, StorageError> {
    Ok((
        row.try_get("id")?,
        row.try_get("project_id")?,
        serde_json::from_str(&row.try_get::<String, _>("handoff_json")?)?,
        serde_json::from_str(&row.try_get::<String, _>("routing_json")?)?,
        row.try_get("origin_kind")?,
        row.try_get("review_state")?,
        row.try_get("manifest_id")?,
        row.try_get("requested_by")?,
        row.try_get("reviewed_by")?,
        row.try_get("decision_reason")?,
        row.try_get::<Option<i64>, _>("reviewed_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        row.try_get::<Option<i64>, _>("launched_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        timestamp_to_datetime(row.try_get("created_at")?)?,
        timestamp_to_datetime(row.try_get("updated_at")?)?,
    ))
}
