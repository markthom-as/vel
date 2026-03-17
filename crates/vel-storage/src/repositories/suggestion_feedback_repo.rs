use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    db::{
        StorageError, SuggestionFeedbackInsert, SuggestionFeedbackRecord, SuggestionFeedbackSummary,
    },
    mapping::parse_json_value,
};

pub(crate) async fn insert_suggestion_feedback(
    pool: &SqlitePool,
    input: SuggestionFeedbackInsert,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let id = insert_suggestion_feedback_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(id)
}

pub(crate) async fn insert_suggestion_feedback_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &SuggestionFeedbackInsert,
) -> Result<String, StorageError> {
    let id = format!("sf_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let payload_str = input
        .payload_json
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "{}".to_string());

    sqlx::query(
        r#"
        INSERT INTO suggestion_feedback (
            id,
            suggestion_id,
            outcome_type,
            notes,
            observed_at,
            payload_json,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.suggestion_id)
    .bind(&input.outcome_type)
    .bind(&input.notes)
    .bind(input.observed_at)
    .bind(payload_str)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(id)
}

pub(crate) async fn list_suggestion_feedback(
    pool: &SqlitePool,
    suggestion_id: &str,
) -> Result<Vec<SuggestionFeedbackRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT id, suggestion_id, outcome_type, notes, observed_at, payload_json, created_at
        FROM suggestion_feedback
        WHERE suggestion_id = ?
        ORDER BY created_at ASC
        "#,
    )
    .bind(suggestion_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_suggestion_feedback_row(&row))
        .collect()
}

pub(crate) async fn summarize_suggestion_feedback(
    pool: &SqlitePool,
    suggestion_type: &str,
) -> Result<SuggestionFeedbackSummary, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            COUNT(CASE WHEN outcome_type = 'accepted_policy_changed' THEN 1 END) as accepted,
            COUNT(CASE WHEN outcome_type = 'rejected_not_useful' THEN 1 END) as rejected_not_useful,
            COUNT(CASE WHEN outcome_type = 'rejected_incorrect' THEN 1 END) as rejected_incorrect
        FROM suggestion_feedback sf
        JOIN suggestions s ON s.id = sf.suggestion_id
        WHERE s.suggestion_type = ?
        "#,
    )
    .bind(suggestion_type)
    .fetch_one(pool)
    .await?;

    Ok(SuggestionFeedbackSummary {
        accepted_and_policy_changed: row.try_get::<i64, _>("accepted")? as u32,
        rejected_not_useful: row.try_get::<i64, _>("rejected_not_useful")? as u32,
        rejected_incorrect: row.try_get::<i64, _>("rejected_incorrect")? as u32,
    })
}

fn map_suggestion_feedback_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SuggestionFeedbackRecord, StorageError> {
    let payload_json: String = row.try_get("payload_json")?;
    Ok(SuggestionFeedbackRecord {
        id: row.try_get("id")?,
        suggestion_id: row.try_get("suggestion_id")?,
        outcome_type: row.try_get("outcome_type")?,
        notes: row.try_get("notes")?,
        observed_at: row.try_get("observed_at")?,
        payload_json: Some(parse_json_value(&payload_json)?),
        created_at: row.try_get("created_at")?,
    })
}
