use serde_json::json;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;

use crate::db::{AssistantTranscriptInsert, AssistantTranscriptRecord, StorageError};
use crate::repositories::semantic_memory_repo;

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
               (id, source, conversation_id, message_id, timestamp, role, content, metadata_json, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&input.id)
    .bind(&input.source)
    .bind(&input.conversation_id)
    .bind(&input.message_id)
    .bind(input.timestamp)
    .bind(&input.role)
    .bind(&input.content)
    .bind(&metadata_str)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    if result.rows_affected() > 0 {
        semantic_memory_repo::upsert_transcript_note_record_in_tx(tx, input).await?;
    }
    Ok(result.rows_affected() > 0)
}

pub(crate) async fn list_assistant_transcripts_by_conversation(
    pool: &SqlitePool,
    conversation_id: &str,
) -> Result<Vec<AssistantTranscriptRecord>, StorageError> {
    let rows = sqlx::query(
        r#"SELECT id, source, conversation_id, message_id, timestamp, role, content, metadata_json, created_at
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
                message_id: row.try_get("message_id")?,
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
