use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{ConversationId, EventId, InterventionId, MessageId};

use crate::db::{
    ConversationInsert, ConversationRecord, EventLogInsert, EventLogRecord, InterventionInsert,
    InterventionRecord, MessageInsert, MessageRecord, StorageError,
};

pub(crate) async fn create_conversation(
    pool: &SqlitePool,
    input: ConversationInsert,
) -> Result<ConversationRecord, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"INSERT INTO conversations (id, title, kind, pinned, archived, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&input.id)
    .bind(input.title.as_deref())
    .bind(&input.kind)
    .bind(if input.pinned { 1i32 } else { 0 })
    .bind(if input.archived { 1i32 } else { 0 })
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(ConversationRecord {
        id: ConversationId::from(input.id),
        title: input.title,
        kind: input.kind,
        pinned: input.pinned,
        archived: input.archived,
        created_at: now,
        updated_at: now,
        message_count: 0,
        last_message_at: None,
        project_label: None,
    })
}

pub(crate) async fn list_conversations(
    pool: &SqlitePool,
    archived: Option<bool>,
    limit: u32,
) -> Result<Vec<ConversationRecord>, StorageError> {
    let limit = limit.min(500) as i64;
    let rows = if let Some(arch) = archived {
        sqlx::query(
            r#"SELECT
                   c.id,
                   c.title,
                   c.kind,
                   c.pinned,
                   c.archived,
                   c.created_at,
                   c.updated_at,
                   COALESCE((SELECT COUNT(*) FROM messages m WHERE m.conversation_id = c.id), 0) AS message_count,
                   (SELECT MAX(m.created_at) FROM messages m WHERE m.conversation_id = c.id) AS last_message_at,
                   (
                     SELECT COALESCE(
                       json_extract(m.content_json, '$.project_label'),
                       json_extract(m.content_json, '$.project')
                     )
                     FROM messages m
                     WHERE m.conversation_id = c.id
                       AND COALESCE(
                         json_extract(m.content_json, '$.project_label'),
                         json_extract(m.content_json, '$.project')
                       ) IS NOT NULL
                     ORDER BY m.created_at DESC
                     LIMIT 1
                   ) AS project_label
               FROM conversations c WHERE c.archived = ? ORDER BY c.updated_at DESC LIMIT ?"#,
        )
        .bind(if arch { 1i32 } else { 0 })
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"SELECT
                   c.id,
                   c.title,
                   c.kind,
                   c.pinned,
                   c.archived,
                   c.created_at,
                   c.updated_at,
                   COALESCE((SELECT COUNT(*) FROM messages m WHERE m.conversation_id = c.id), 0) AS message_count,
                   (SELECT MAX(m.created_at) FROM messages m WHERE m.conversation_id = c.id) AS last_message_at,
                   (
                     SELECT COALESCE(
                       json_extract(m.content_json, '$.project_label'),
                       json_extract(m.content_json, '$.project')
                     )
                     FROM messages m
                     WHERE m.conversation_id = c.id
                       AND COALESCE(
                         json_extract(m.content_json, '$.project_label'),
                         json_extract(m.content_json, '$.project')
                       ) IS NOT NULL
                     ORDER BY m.created_at DESC
                     LIMIT 1
                   ) AS project_label
               FROM conversations c ORDER BY c.updated_at DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };
    rows.into_iter()
        .map(|row| map_conversation_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn get_conversation(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<ConversationRecord>, StorageError> {
    let row = sqlx::query(
        r#"SELECT
               c.id,
               c.title,
               c.kind,
               c.pinned,
               c.archived,
               c.created_at,
               c.updated_at,
               COALESCE((SELECT COUNT(*) FROM messages m WHERE m.conversation_id = c.id), 0) AS message_count,
               (SELECT MAX(m.created_at) FROM messages m WHERE m.conversation_id = c.id) AS last_message_at,
               (
                 SELECT COALESCE(
                   json_extract(m.content_json, '$.project_label'),
                   json_extract(m.content_json, '$.project')
                 )
                 FROM messages m
                 WHERE m.conversation_id = c.id
                   AND COALESCE(
                     json_extract(m.content_json, '$.project_label'),
                     json_extract(m.content_json, '$.project')
                   ) IS NOT NULL
                 ORDER BY m.created_at DESC
                 LIMIT 1
               ) AS project_label
           FROM conversations c WHERE c.id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    row.map(|r| map_conversation_row(&r)).transpose()
}

pub(crate) async fn rename_conversation(
    pool: &SqlitePool,
    id: &str,
    title: &str,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(r#"UPDATE conversations SET title = ?, updated_at = ? WHERE id = ?"#)
        .bind(title)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn pin_conversation(
    pool: &SqlitePool,
    id: &str,
    pinned: bool,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(r#"UPDATE conversations SET pinned = ?, updated_at = ? WHERE id = ?"#)
        .bind(if pinned { 1i32 } else { 0 })
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn archive_conversation(
    pool: &SqlitePool,
    id: &str,
    archived: bool,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(r#"UPDATE conversations SET archived = ?, updated_at = ? WHERE id = ?"#)
        .bind(if archived { 1i32 } else { 0 })
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn create_message(
    pool: &SqlitePool,
    input: MessageInsert,
) -> Result<MessageId, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"INSERT INTO messages (id, conversation_id, role, kind, content_json, status, importance, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&input.id)
    .bind(&input.conversation_id)
    .bind(&input.role)
    .bind(&input.kind)
    .bind(&input.content_json)
    .bind(input.status.as_deref())
    .bind(input.importance.as_deref())
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(MessageId::from(input.id))
}

pub(crate) async fn list_messages_by_conversation(
    pool: &SqlitePool,
    conversation_id: &str,
    limit: u32,
) -> Result<Vec<MessageRecord>, StorageError> {
    let limit = limit.min(2000) as i64;
    let rows = sqlx::query(
        r#"SELECT id, conversation_id, role, kind, content_json, status, importance, created_at, updated_at
           FROM messages WHERE conversation_id = ? ORDER BY created_at ASC LIMIT ?"#,
    )
    .bind(conversation_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_message_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn get_message(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<MessageRecord>, StorageError> {
    let row = sqlx::query(
        r#"SELECT id, conversation_id, role, kind, content_json, status, importance, created_at, updated_at
           FROM messages WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    row.map(|r| map_message_row(&r)).transpose()
}

pub(crate) async fn update_message_status(
    pool: &SqlitePool,
    id: &str,
    status: &str,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(r#"UPDATE messages SET status = ?, updated_at = ? WHERE id = ?"#)
        .bind(status)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn create_intervention(
    pool: &SqlitePool,
    input: InterventionInsert,
) -> Result<InterventionId, StorageError> {
    sqlx::query(
        r#"INSERT INTO interventions (id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&input.id)
    .bind(&input.message_id)
    .bind(&input.kind)
    .bind(&input.state)
    .bind(input.surfaced_at)
    .bind(input.resolved_at)
    .bind(input.snoozed_until)
    .bind(input.confidence)
    .bind(input.source_json.as_deref())
    .bind(input.provenance_json.as_deref())
    .execute(pool)
    .await?;
    Ok(InterventionId::from(input.id))
}

pub(crate) async fn list_interventions_active(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<InterventionRecord>, StorageError> {
    let limit = limit.min(500) as i64;
    let now_ts = OffsetDateTime::now_utc().unix_timestamp();
    let rows = sqlx::query(
        r#"SELECT id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json
           FROM interventions WHERE state = 'active' OR (state = 'snoozed' AND (snoozed_until IS NULL OR snoozed_until > ?))
           ORDER BY surfaced_at DESC LIMIT ?"#,
    )
    .bind(now_ts)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_intervention_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn list_interventions_archived(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<InterventionRecord>, StorageError> {
    let limit = limit.min(500) as i64;
    let rows = sqlx::query(
        r#"SELECT id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json
           FROM interventions
           WHERE state IN ('resolved', 'dismissed')
           ORDER BY COALESCE(resolved_at, surfaced_at) DESC
           LIMIT ?"#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_intervention_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn get_interventions_by_message(
    pool: &SqlitePool,
    message_id: &str,
) -> Result<Vec<InterventionRecord>, StorageError> {
    let rows = sqlx::query(
        r#"SELECT id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json
           FROM interventions WHERE message_id = ? ORDER BY surfaced_at DESC"#,
    )
    .bind(message_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_intervention_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn get_interventions_by_conversation(
    pool: &SqlitePool,
    conversation_id: &str,
) -> Result<Vec<InterventionRecord>, StorageError> {
    let rows = sqlx::query(
        r#"SELECT i.id, i.message_id, i.kind, i.state, i.surfaced_at, i.resolved_at, i.snoozed_until, i.confidence, i.source_json, i.provenance_json
           FROM interventions i
           JOIN messages m ON m.id = i.message_id
           WHERE m.conversation_id = ?
           ORDER BY i.surfaced_at DESC"#,
    )
    .bind(conversation_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_intervention_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn get_intervention(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<InterventionRecord>, StorageError> {
    let row = sqlx::query(
        r#"SELECT id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json
           FROM interventions WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    row.map(|r| map_intervention_row(&r)).transpose()
}

pub(crate) async fn snooze_intervention(
    pool: &SqlitePool,
    id: &str,
    snoozed_until_ts: i64,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"UPDATE interventions SET state = 'snoozed', snoozed_until = ?, resolved_at = NULL WHERE id = ?"#,
    )
    .bind(snoozed_until_ts)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn acknowledge_intervention(
    pool: &SqlitePool,
    id: &str,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"UPDATE interventions SET state = 'acknowledged', resolved_at = NULL, snoozed_until = NULL WHERE id = ?"#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn resolve_intervention(pool: &SqlitePool, id: &str) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"UPDATE interventions SET state = 'resolved', resolved_at = ?, snoozed_until = NULL WHERE id = ?"#,
    )
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn dismiss_intervention(pool: &SqlitePool, id: &str) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"UPDATE interventions SET state = 'dismissed', resolved_at = ?, snoozed_until = NULL WHERE id = ?"#,
    )
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Return intervention to the active queue (operator "mark unread").
pub(crate) async fn reactivate_intervention(pool: &SqlitePool, id: &str) -> Result<(), StorageError> {
    sqlx::query(
        r#"UPDATE interventions SET state = 'active', resolved_at = NULL, snoozed_until = NULL WHERE id = ?"#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn append_event(
    pool: &SqlitePool,
    input: EventLogInsert,
) -> Result<EventId, StorageError> {
    let id = input
        .id
        .unwrap_or_else(|| format!("evt_{}", Uuid::new_v4().simple()));
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"INSERT INTO event_log (id, event_name, aggregate_type, aggregate_id, payload_json, created_at)
           VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(&input.event_name)
    .bind(input.aggregate_type.as_deref())
    .bind(input.aggregate_id.as_deref())
    .bind(&input.payload_json)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(EventId::from(id))
}

pub(crate) async fn list_events_recent(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<EventLogRecord>, StorageError> {
    let limit = limit.min(1000) as i64;
    let rows = sqlx::query(
        r#"SELECT id, event_name, aggregate_type, aggregate_id, payload_json, created_at
           FROM event_log ORDER BY created_at DESC LIMIT ?"#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_event_log_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn list_events_by_aggregate(
    pool: &SqlitePool,
    aggregate_type: &str,
    aggregate_id: &str,
    limit: u32,
) -> Result<Vec<EventLogRecord>, StorageError> {
    let limit = limit.min(500) as i64;
    let rows = sqlx::query(
        r#"SELECT id, event_name, aggregate_type, aggregate_id, payload_json, created_at
           FROM event_log WHERE aggregate_type = ? AND aggregate_id = ? ORDER BY created_at DESC LIMIT ?"#,
    )
    .bind(aggregate_type)
    .bind(aggregate_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_event_log_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

fn map_conversation_row(row: &sqlx::sqlite::SqliteRow) -> Result<ConversationRecord, StorageError> {
    let pinned: i64 = row.try_get("pinned")?;
    let archived: i64 = row.try_get("archived")?;
    Ok(ConversationRecord {
        id: ConversationId::from(row.try_get::<String, _>("id")?),
        title: row.try_get("title")?,
        kind: row.try_get("kind")?,
        pinned: pinned != 0,
        archived: archived != 0,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        message_count: row.try_get("message_count")?,
        last_message_at: row.try_get("last_message_at")?,
        project_label: row.try_get("project_label")?,
    })
}

fn map_message_row(row: &sqlx::sqlite::SqliteRow) -> Result<MessageRecord, StorageError> {
    Ok(MessageRecord {
        id: MessageId::from(row.try_get::<String, _>("id")?),
        conversation_id: ConversationId::from(row.try_get::<String, _>("conversation_id")?),
        role: row.try_get("role")?,
        kind: row.try_get("kind")?,
        content_json: row.try_get("content_json")?,
        status: row.try_get("status")?,
        importance: row.try_get("importance")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_intervention_row(row: &sqlx::sqlite::SqliteRow) -> Result<InterventionRecord, StorageError> {
    Ok(InterventionRecord {
        id: InterventionId::from(row.try_get::<String, _>("id")?),
        message_id: MessageId::from(row.try_get::<String, _>("message_id")?),
        kind: row.try_get("kind")?,
        state: row.try_get("state")?,
        surfaced_at: row.try_get("surfaced_at")?,
        resolved_at: row.try_get("resolved_at")?,
        snoozed_until: row.try_get("snoozed_until")?,
        confidence: row.try_get("confidence")?,
        source_json: row.try_get("source_json")?,
        provenance_json: row.try_get("provenance_json")?,
        created_at: row.try_get("surfaced_at")?,
    })
}

fn map_event_log_row(row: &sqlx::sqlite::SqliteRow) -> Result<EventLogRecord, StorageError> {
    Ok(EventLogRecord {
        id: EventId::from(row.try_get::<String, _>("id")?),
        event_name: row.try_get("event_name")?,
        aggregate_type: row.try_get("aggregate_type")?,
        aggregate_id: row.try_get("aggregate_id")?,
        payload_json: row.try_get("payload_json")?,
        created_at: row.try_get("created_at")?,
    })
}
