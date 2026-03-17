use sqlx::{Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::StorageError;

pub(crate) async fn insert_context_timeline(
    pool: &SqlitePool,
    timestamp: i64,
    context_json: &str,
    trigger_signal_id: Option<&str>,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    insert_context_timeline_in_tx(&mut tx, timestamp, context_json, trigger_signal_id).await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn insert_context_timeline_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    timestamp: i64,
    context_json: &str,
    trigger_signal_id: Option<&str>,
) -> Result<(), StorageError> {
    let id = format!("ctl_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"INSERT INTO context_timeline (id, timestamp, context_json, trigger_signal_id, created_at) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(timestamp)
    .bind(context_json)
    .bind(trigger_signal_id)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub(crate) async fn list_context_timeline(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<(String, i64, String)>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = sqlx::query_as::<_, (String, i64, String)>(
        r#"SELECT id, timestamp, context_json FROM context_timeline ORDER BY timestamp DESC LIMIT ?"#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows)
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
    async fn insert_and_list_context_timeline_order_and_limit() {
        let pool = test_pool().await;

        insert_context_timeline(&pool, 1_700_000_001, r#"{"state":"one"}"#, None)
            .await
            .unwrap();
        insert_context_timeline(&pool, 1_700_000_003, r#"{"state":"three"}"#, Some("sig_3"))
            .await
            .unwrap();
        insert_context_timeline(&pool, 1_700_000_002, r#"{"state":"two"}"#, Some("sig_2"))
            .await
            .unwrap();

        let rows = list_context_timeline(&pool, 2).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].1, 1_700_000_003);
        assert_eq!(rows[1].1, 1_700_000_002);
    }

    #[tokio::test]
    async fn insert_context_timeline_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;
        let before = list_context_timeline(&pool, 10).await.unwrap();

        {
            let mut tx = pool.begin().await.unwrap();
            insert_context_timeline_in_tx(
                &mut tx,
                1_700_000_888,
                r#"{"state":"transient"}"#,
                Some("sig_tx"),
            )
            .await
            .unwrap();
            tx.rollback().await.unwrap();
        }

        let after = list_context_timeline(&pool, 10).await.unwrap();
        assert_eq!(before, after);
    }
}
