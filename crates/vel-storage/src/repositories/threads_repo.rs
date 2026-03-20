use sqlx::{Sqlite, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::StorageError;
use crate::repositories::semantic_memory_repo;

type ThreadRecord = (String, String, String, String, String, i64, i64);
type ThreadListRecord = (String, String, String, String, i64, i64);
type ThreadLinkRecord = (String, String, String, String);
type ThreadIdRecord = (String,);

pub(crate) async fn insert_thread(
    pool: &SqlitePool,
    id: &str,
    thread_type: &str,
    title: &str,
    status: &str,
    metadata_json: &str,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"INSERT INTO threads (id, thread_type, title, status, metadata_json, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(id)
    .bind(thread_type)
    .bind(title)
    .bind(status)
    .bind(metadata_json)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    semantic_memory_repo::upsert_thread_record(pool, id, thread_type, title, status, now).await?;
    Ok(())
}

pub(crate) async fn get_thread_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<ThreadRecord>, StorageError> {
    sqlx::query_as::<Sqlite, ThreadRecord>(
        r#"SELECT id, thread_type, title, status, metadata_json, created_at, updated_at FROM threads WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(StorageError::from)
}

pub(crate) async fn list_threads(
    pool: &SqlitePool,
    status_filter: Option<&str>,
    limit: u32,
) -> Result<Vec<ThreadListRecord>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = if let Some(status) = status_filter {
        sqlx::query_as::<Sqlite, ThreadListRecord>(
            r#"SELECT id, thread_type, title, status, created_at, updated_at FROM threads WHERE status = ? ORDER BY updated_at DESC LIMIT ?"#,
        )
        .bind(status)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<Sqlite, ThreadListRecord>(
            r#"SELECT id, thread_type, title, status, created_at, updated_at FROM threads ORDER BY updated_at DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };

    Ok(rows)
}

pub(crate) async fn update_thread_status(
    pool: &SqlitePool,
    id: &str,
    status: &str,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(r#"UPDATE threads SET status = ?, updated_at = ? WHERE id = ?"#)
        .bind(status)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    if let Some((thread_id, thread_type, title, status, _, _, updated_at)) =
        get_thread_by_id(pool, id).await?
    {
        semantic_memory_repo::upsert_thread_record(
            pool,
            &thread_id,
            &thread_type,
            &title,
            &status,
            updated_at,
        )
        .await?;
    }
    Ok(())
}

pub(crate) async fn update_thread_metadata(
    pool: &SqlitePool,
    id: &str,
    metadata_json: &str,
) -> Result<(), StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(r#"UPDATE threads SET metadata_json = ?, updated_at = ? WHERE id = ?"#)
        .bind(metadata_json)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    if let Some((thread_id, thread_type, title, status, _, _, updated_at)) =
        get_thread_by_id(pool, id).await?
    {
        semantic_memory_repo::upsert_thread_record(
            pool,
            &thread_id,
            &thread_type,
            &title,
            &status,
            updated_at,
        )
        .await?;
    }
    Ok(())
}

pub(crate) async fn insert_thread_link(
    pool: &SqlitePool,
    thread_id: &str,
    entity_type: &str,
    entity_id: &str,
    relation_type: &str,
) -> Result<String, StorageError> {
    let id = format!("tl_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"INSERT OR IGNORE INTO thread_links (id, thread_id, entity_type, entity_id, relation_type, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(thread_id)
    .bind(entity_type)
    .bind(entity_id)
    .bind(relation_type)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(id)
}

pub(crate) async fn list_thread_links(
    pool: &SqlitePool,
    thread_id: &str,
) -> Result<Vec<ThreadLinkRecord>, StorageError> {
    sqlx::query_as::<Sqlite, ThreadLinkRecord>(
        r#"SELECT id, entity_type, entity_id, relation_type FROM thread_links WHERE thread_id = ? ORDER BY created_at ASC"#,
    )
    .bind(thread_id)
    .fetch_all(pool)
    .await
    .map_err(StorageError::from)
}

pub(crate) async fn list_threads_linking_entity(
    pool: &SqlitePool,
    entity_type: &str,
    entity_id: &str,
    relation_type: &str,
) -> Result<Vec<String>, StorageError> {
    let rows = sqlx::query_as::<Sqlite, ThreadIdRecord>(
        r#"SELECT thread_id FROM thread_links WHERE entity_type = ? AND entity_id = ? AND relation_type = ? ORDER BY created_at ASC"#,
    )
    .bind(entity_type)
    .bind(entity_id)
    .bind(relation_type)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(thread_id,)| thread_id).collect())
}
