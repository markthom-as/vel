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

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn insert_suggestion_feedback_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &SuggestionFeedbackInsert,
) -> Result<String, StorageError> {
    let id = format!("sugfb_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let payload_json = input
        .payload_json
        .as_ref()
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| StorageError::Validation(error.to_string()))?;
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
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.suggestion_id)
    .bind(&input.outcome_type)
    .bind(&input.notes)
    .bind(input.observed_at)
    .bind(payload_json)
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
        ORDER BY observed_at DESC, created_at DESC
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
            COALESCE(SUM(CASE WHEN sf.outcome_type = 'accepted_and_policy_changed' THEN 1 ELSE 0 END), 0)
                AS accepted_and_policy_changed,
            COALESCE(SUM(CASE WHEN sf.outcome_type = 'rejected_not_useful' THEN 1 ELSE 0 END), 0)
                AS rejected_not_useful,
            COALESCE(SUM(CASE WHEN sf.outcome_type = 'rejected_incorrect' THEN 1 ELSE 0 END), 0)
                AS rejected_incorrect
        FROM suggestion_feedback sf
        INNER JOIN suggestions s ON s.id = sf.suggestion_id
        WHERE s.suggestion_type = ?
        "#,
    )
    .bind(suggestion_type)
    .fetch_one(pool)
    .await?;
    Ok(SuggestionFeedbackSummary {
        accepted_and_policy_changed: row.try_get::<i64, _>("accepted_and_policy_changed")?.max(0)
            as u32,
        rejected_not_useful: row.try_get::<i64, _>("rejected_not_useful")?.max(0) as u32,
        rejected_incorrect: row.try_get::<i64, _>("rejected_incorrect")?.max(0) as u32,
    })
}

fn map_suggestion_feedback_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SuggestionFeedbackRecord, StorageError> {
    let payload_json = row.try_get::<Option<String>, _>("payload_json")?;
    Ok(SuggestionFeedbackRecord {
        id: row.try_get("id")?,
        suggestion_id: row.try_get("suggestion_id")?,
        outcome_type: row.try_get("outcome_type")?,
        notes: row.try_get("notes")?,
        observed_at: row.try_get("observed_at")?,
        payload_json: payload_json.as_deref().map(parse_json_value).transpose()?,
        created_at: row.try_get("created_at")?,
    })
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

    async fn seed_suggestion(pool: &SqlitePool, suggestion_id: &str, suggestion_type: &str) {
        sqlx::query(
            r#"
            INSERT INTO suggestions (
                id,
                suggestion_type,
                state,
                payload_json,
                created_at,
                priority
            )
            VALUES (?, ?, 'open', '{}', ?, 50)
            "#,
        )
        .bind(suggestion_id)
        .bind(suggestion_type)
        .bind(OffsetDateTime::now_utc().unix_timestamp())
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn insert_list_and_summarize_feedback() {
        let pool = test_pool().await;
        let suggestion_id = "sug_repo_feedback";
        seed_suggestion(&pool, suggestion_id, "alignment").await;

        let inserted_id = insert_suggestion_feedback(
            &pool,
            SuggestionFeedbackInsert {
                suggestion_id: suggestion_id.to_string(),
                outcome_type: "accepted_and_policy_changed".to_string(),
                notes: Some("works well".to_string()),
                observed_at: 1_700_000_000,
                payload_json: Some(json!({"source":"repo-test"})),
            },
        )
        .await
        .unwrap();

        insert_suggestion_feedback(
            &pool,
            SuggestionFeedbackInsert {
                suggestion_id: suggestion_id.to_string(),
                outcome_type: "rejected_not_useful".to_string(),
                notes: None,
                observed_at: 1_700_000_100,
                payload_json: None,
            },
        )
        .await
        .unwrap();

        let listed = list_suggestion_feedback(&pool, suggestion_id)
            .await
            .unwrap();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].outcome_type, "rejected_not_useful");
        assert_eq!(listed[1].id, inserted_id);
        assert_eq!(listed[1].payload_json, Some(json!({"source":"repo-test"})));

        let summary = summarize_suggestion_feedback(&pool, "alignment")
            .await
            .unwrap();
        assert_eq!(summary.accepted_and_policy_changed, 1);
        assert_eq!(summary.rejected_not_useful, 1);
        assert_eq!(summary.rejected_incorrect, 0);
    }

    #[tokio::test]
    async fn insert_suggestion_feedback_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;
        let suggestion_id = "sug_repo_feedback_tx";
        seed_suggestion(&pool, suggestion_id, "focus").await;

        {
            let mut tx = pool.begin().await.unwrap();
            insert_suggestion_feedback_in_tx(
                &mut tx,
                &SuggestionFeedbackInsert {
                    suggestion_id: suggestion_id.to_string(),
                    outcome_type: "rejected_incorrect".to_string(),
                    notes: Some("needs more context".to_string()),
                    observed_at: 1_700_000_200,
                    payload_json: None,
                },
            )
            .await
            .unwrap();
            tx.rollback().await.unwrap();
        }

        let listed = list_suggestion_feedback(&pool, suggestion_id)
            .await
            .unwrap();
        assert!(listed.is_empty());
    }
}
