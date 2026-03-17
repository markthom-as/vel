use serde_json::json;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;

use crate::db::{AssistantTranscriptInsert, AssistantTranscriptRecord, StorageError};

pub(crate) async fn insert_assistant_transcript(
    pool: &SqlitePool,
    input: AssistantTranscriptInsert,
) -> Result<bool, StorageError> {
    let mut tx = pool.begin().await?;
    let inserted = insert_assistant_transcript_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(inserted)
}

pub(crate) async fn insert_assistant_transcript_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &AssistantTranscriptInsert,
) -> Result<bool, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let metadata_str = serde_json::to_string(&input.metadata_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let result = sqlx::query(
        r#"INSERT OR IGNORE INTO assistant_transcripts
               (id, source, conversation_id, timestamp, role, content, metadata_json, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&input.id)
    .bind(&input.source)
    .bind(&input.conversation_id)
    .bind(input.timestamp)
    .bind(&input.role)
    .bind(&input.content)
    .bind(&metadata_str)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub(crate) async fn list_assistant_transcripts_by_conversation(
    pool: &SqlitePool,
    conversation_id: &str,
) -> Result<Vec<AssistantTranscriptRecord>, StorageError> {
    let rows = sqlx::query(
        r#"SELECT id, source, conversation_id, timestamp, role, content, metadata_json, created_at
               FROM assistant_transcripts
               WHERE conversation_id = ?
               ORDER BY timestamp ASC, created_at ASC"#,
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            let metadata_str: String = row.try_get("metadata_json")?;
            Ok(AssistantTranscriptRecord {
                id: row.try_get("id")?,
                source: row.try_get("source")?,
                conversation_id: row.try_get("conversation_id")?,
                timestamp: row.try_get("timestamp")?,
                role: row.try_get("role")?,
                content: row.try_get("content")?,
                metadata_json: serde_json::from_str(&metadata_str).unwrap_or_else(|_| json!({})),
                created_at: row.try_get("created_at")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()
        .map_err(StorageError::from)
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
    async fn insert_and_list_assistant_transcripts_orders_by_timestamp() {
        let pool = test_pool().await;

        insert_assistant_transcript(
            &pool,
            AssistantTranscriptInsert {
                id: "at_1".to_string(),
                source: "assistant".to_string(),
                conversation_id: "conv_1".to_string(),
                timestamp: 1_700_000_100,
                role: "assistant".to_string(),
                content: "second".to_string(),
                metadata_json: json!({"idx":2}),
            },
        )
        .await
        .unwrap();

        insert_assistant_transcript(
            &pool,
            AssistantTranscriptInsert {
                id: "at_2".to_string(),
                source: "assistant".to_string(),
                conversation_id: "conv_1".to_string(),
                timestamp: 1_700_000_050,
                role: "user".to_string(),
                content: "first".to_string(),
                metadata_json: json!({"idx":1}),
            },
        )
        .await
        .unwrap();

        let rows = list_assistant_transcripts_by_conversation(&pool, "conv_1")
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, "at_2");
        assert_eq!(rows[1].id, "at_1");
    }

    #[tokio::test]
    async fn insert_assistant_transcript_deduplicates_by_id() {
        let pool = test_pool().await;

        let first = insert_assistant_transcript(
            &pool,
            AssistantTranscriptInsert {
                id: "at_dup".to_string(),
                source: "assistant".to_string(),
                conversation_id: "conv_dup".to_string(),
                timestamp: 1_700_000_200,
                role: "assistant".to_string(),
                content: "hello".to_string(),
                metadata_json: json!({"v":1}),
            },
        )
        .await
        .unwrap();

        let second = insert_assistant_transcript(
            &pool,
            AssistantTranscriptInsert {
                id: "at_dup".to_string(),
                source: "assistant".to_string(),
                conversation_id: "conv_dup".to_string(),
                timestamp: 1_700_000_201,
                role: "assistant".to_string(),
                content: "ignored".to_string(),
                metadata_json: json!({"v":2}),
            },
        )
        .await
        .unwrap();

        assert!(first);
        assert!(!second);

        let rows = list_assistant_transcripts_by_conversation(&pool, "conv_dup")
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].content, "hello");
    }

    #[tokio::test]
    async fn insert_assistant_transcript_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;

        {
            let mut tx = pool.begin().await.unwrap();
            let inserted = insert_assistant_transcript_in_tx(
                &mut tx,
                &AssistantTranscriptInsert {
                    id: "at_tx".to_string(),
                    source: "assistant".to_string(),
                    conversation_id: "conv_tx".to_string(),
                    timestamp: 1_700_000_999,
                    role: "assistant".to_string(),
                    content: "transient".to_string(),
                    metadata_json: json!({"tx":true}),
                },
            )
            .await
            .unwrap();
            assert!(inserted);
            tx.rollback().await.unwrap();
        }

        let rows = list_assistant_transcripts_by_conversation(&pool, "conv_tx")
            .await
            .unwrap();
        assert!(rows.is_empty());
    }
}
