use sqlx::{Row, SqlitePool};

use crate::db::StorageError;

#[derive(Debug, Clone)]
pub struct ConnectRunEventRecord {
    pub id: i64,
    pub run_id: String,
    pub stream: String,
    pub chunk: String,
    pub created_at: i64,
}

pub(crate) async fn insert_connect_run_event(
    pool: &SqlitePool,
    run_id: &str,
    stream: &str,
    chunk: &str,
    created_at: i64,
) -> Result<i64, StorageError> {
    let result = sqlx::query(
        r#"
        INSERT INTO connect_run_events (run_id, stream, chunk, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(run_id)
    .bind(stream)
    .bind(chunk)
    .bind(created_at)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub(crate) async fn list_connect_run_events(
    pool: &SqlitePool,
    run_id: &str,
    after_id: Option<i64>,
    limit: u32,
) -> Result<Vec<ConnectRunEventRecord>, StorageError> {
    let capped_limit = limit.clamp(1, 500);
    let rows = if let Some(after_id) = after_id {
        sqlx::query(
            r#"
            SELECT id, run_id, stream, chunk, created_at
            FROM connect_run_events
            WHERE run_id = ? AND id > ?
            ORDER BY id ASC
            LIMIT ?
            "#,
        )
        .bind(run_id)
        .bind(after_id)
        .bind(capped_limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, run_id, stream, chunk, created_at
            FROM connect_run_events
            WHERE run_id = ?
            ORDER BY id ASC
            LIMIT ?
            "#,
        )
        .bind(run_id)
        .bind(capped_limit)
        .fetch_all(pool)
        .await?
    };

    rows.into_iter().map(map_connect_run_event_row).collect()
}

pub(crate) async fn latest_connect_run_event_id(
    pool: &SqlitePool,
    run_id: &str,
) -> Result<Option<i64>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT MAX(id) AS latest_id
        FROM connect_run_events
        WHERE run_id = ?
        "#,
    )
    .bind(run_id)
    .fetch_one(pool)
    .await?;
    let latest_id: Option<i64> = row.try_get("latest_id")?;
    Ok(latest_id)
}

fn map_connect_run_event_row(
    row: sqlx::sqlite::SqliteRow,
) -> Result<ConnectRunEventRecord, StorageError> {
    Ok(ConnectRunEventRecord {
        id: row.try_get("id")?,
        run_id: row.try_get("run_id")?,
        stream: row.try_get("stream")?,
        chunk: row.try_get("chunk")?,
        created_at: row.try_get("created_at")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");
    static TEST_DB_COUNTER: AtomicU64 = AtomicU64::new(0);

    async fn make_pool() -> SqlitePool {
        let id = TEST_DB_COUNTER.fetch_add(1, Ordering::Relaxed);
        let url = format!(
            "file:connect_run_events_{}_{}?mode=memory&cache=shared",
            std::process::id(),
            id
        );
        let pool = SqlitePool::connect(&url).await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn insert_and_list_connect_run_events() {
        let pool = make_pool().await;
        let created_at = time::OffsetDateTime::now_utc().unix_timestamp();

        let first = insert_connect_run_event(&pool, "run_1", "stdout", "hello", created_at)
            .await
            .expect("insert first");
        let second = insert_connect_run_event(&pool, "run_1", "stderr", "warn", created_at + 1)
            .await
            .expect("insert second");

        let listed = list_connect_run_events(&pool, "run_1", None, 100)
            .await
            .expect("list should work");
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].id, first);
        assert_eq!(listed[1].id, second);
        assert_eq!(listed[0].stream, "stdout");
        assert_eq!(listed[1].chunk, "warn");

        let tail = list_connect_run_events(&pool, "run_1", Some(first), 100)
            .await
            .expect("tail should work");
        assert_eq!(tail.len(), 1);
        assert_eq!(tail[0].id, second);

        let latest = latest_connect_run_event_id(&pool, "run_1")
            .await
            .expect("latest should work");
        assert_eq!(latest, Some(second));

        let missing = latest_connect_run_event_id(&pool, "run_missing")
            .await
            .expect("missing should work");
        assert_eq!(missing, None);
    }
}
