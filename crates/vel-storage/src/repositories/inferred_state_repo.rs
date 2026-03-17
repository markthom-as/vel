use serde_json::json;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::{InferredStateInsert, InferredStateRecord, StorageError};

pub(crate) async fn insert_inferred_state(
    pool: &SqlitePool,
    input: InferredStateInsert,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let state_id = insert_inferred_state_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(state_id)
}

pub(crate) async fn insert_inferred_state_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &InferredStateInsert,
) -> Result<String, StorageError> {
    let state_id = format!("ist_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let context_str = serde_json::to_string(input.context_json.as_ref().unwrap_or(&json!({})))
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    sqlx::query(
        r#"INSERT INTO inferred_state (state_id, state_name, confidence, timestamp, context_json, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&state_id)
    .bind(&input.state_name)
    .bind(&input.confidence)
    .bind(input.timestamp)
    .bind(&context_str)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(state_id)
}

pub(crate) async fn list_inferred_state_recent(
    pool: &SqlitePool,
    state_name: Option<&str>,
    limit: u32,
) -> Result<Vec<InferredStateRecord>, StorageError> {
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
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(|row| map_inferred_state_row(&row)).collect()
}

fn map_inferred_state_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<InferredStateRecord, StorageError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn insert_and_list_inferred_state_recent_filters_and_orders() {
        let pool = test_pool().await;

        insert_inferred_state(
            &pool,
            InferredStateInsert {
                state_name: "morning".to_string(),
                confidence: Some("low".to_string()),
                timestamp: 1_700_000_100,
                context_json: Some(json!({"mode":"morning","score":1})),
            },
        )
        .await
        .unwrap();
        insert_inferred_state(
            &pool,
            InferredStateInsert {
                state_name: "today".to_string(),
                confidence: Some("high".to_string()),
                timestamp: 1_700_000_300,
                context_json: Some(json!({"mode":"today","score":3})),
            },
        )
        .await
        .unwrap();
        insert_inferred_state(
            &pool,
            InferredStateInsert {
                state_name: "morning".to_string(),
                confidence: Some("medium".to_string()),
                timestamp: 1_700_000_200,
                context_json: Some(json!({"mode":"morning","score":2})),
            },
        )
        .await
        .unwrap();

        let all = list_inferred_state_recent(&pool, None, 10).await.unwrap();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].timestamp, 1_700_000_300);
        assert_eq!(all[1].timestamp, 1_700_000_200);
        assert_eq!(all[2].timestamp, 1_700_000_100);

        let filtered = list_inferred_state_recent(&pool, Some("morning"), 10)
            .await
            .unwrap();
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|row| row.state_name == "morning"));
        assert_eq!(filtered[0].timestamp, 1_700_000_200);
    }

    #[tokio::test]
    async fn insert_inferred_state_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;

        {
            let mut tx = pool.begin().await.unwrap();
            let _id = insert_inferred_state_in_tx(
                &mut tx,
                &InferredStateInsert {
                    state_name: "rollback".to_string(),
                    confidence: Some("low".to_string()),
                    timestamp: 1_700_000_777,
                    context_json: Some(json!({"mode":"rollback"})),
                },
            )
            .await
            .unwrap();
            tx.rollback().await.unwrap();
        }

        let rows = list_inferred_state_recent(&pool, Some("rollback"), 10)
            .await
            .unwrap();
        assert!(rows.is_empty());
    }
}
