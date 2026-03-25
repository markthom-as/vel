use serde_json::json;
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::{SignalInsert, SignalRecord, StorageError};

pub(crate) async fn insert_signal(
    pool: &SqlitePool,
    input: SignalInsert,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let signal_id = insert_signal_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(signal_id)
}

pub(crate) async fn insert_signal_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &SignalInsert,
) -> Result<String, StorageError> {
    if let Some(source_ref) = input.source_ref.as_deref() {
        if let Some(existing_id) = sqlx::query_scalar::<_, String>(
            r#"SELECT signal_id FROM signals WHERE source = ? AND source_ref = ? LIMIT 1"#,
        )
        .bind(&input.source)
        .bind(source_ref)
        .fetch_optional(&mut **tx)
        .await?
        {
            return Ok(existing_id);
        }
    }

    let signal_id = format!("sig_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let payload_str = serde_json::to_string(input.payload_json.as_ref().unwrap_or(&json!({})))
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    sqlx::query(
        r#"INSERT INTO signals (signal_id, signal_type, source, source_ref, timestamp, payload_json, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&signal_id)
    .bind(&input.signal_type)
    .bind(&input.source)
    .bind(&input.source_ref)
    .bind(input.timestamp)
    .bind(&payload_str)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(signal_id)
}

pub(crate) async fn list_signals(
    pool: &SqlitePool,
    signal_type: Option<&str>,
    since_ts: Option<i64>,
    limit: u32,
) -> Result<Vec<SignalRecord>, StorageError> {
    let limit = limit.min(500) as i64;
    let rows = sqlx::query(
        r#"
        SELECT signal_id, signal_type, source, source_ref, timestamp, payload_json, created_at
        FROM signals
        WHERE (? IS NULL OR signal_type = ?) AND (? IS NULL OR timestamp >= ?)
        ORDER BY timestamp DESC
        LIMIT ?
        "#,
    )
    .bind(signal_type)
    .bind(signal_type)
    .bind(since_ts)
    .bind(since_ts)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(|row| map_signal_row(&row)).collect()
}

pub(crate) async fn list_signals_in_window(
    pool: &SqlitePool,
    signal_type: Option<&str>,
    start_ts: i64,
    end_ts: i64,
    limit: u32,
) -> Result<Vec<SignalRecord>, StorageError> {
    let limit = limit.min(5000) as i64;
    let rows = sqlx::query(
        r#"
        SELECT signal_id, signal_type, source, source_ref, timestamp, payload_json, created_at
        FROM signals
        WHERE (? IS NULL OR signal_type = ?)
          AND timestamp >= ?
          AND timestamp < ?
        ORDER BY timestamp DESC
        LIMIT ?
        "#,
    )
    .bind(signal_type)
    .bind(signal_type)
    .bind(start_ts)
    .bind(end_ts)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(|row| map_signal_row(&row)).collect()
}

pub(crate) async fn list_signals_by_ids(
    pool: &SqlitePool,
    signal_ids: &[String],
) -> Result<Vec<SignalRecord>, StorageError> {
    if signal_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut query = QueryBuilder::<Sqlite>::new(
        "SELECT signal_id, signal_type, source, source_ref, timestamp, payload_json, created_at FROM signals WHERE signal_id IN (",
    );
    let mut separated = query.separated(", ");
    for signal_id in signal_ids {
        separated.push_bind(signal_id);
    }
    query.push(") ORDER BY timestamp DESC");

    let rows = query.build().fetch_all(pool).await?;
    rows.into_iter().map(|row| map_signal_row(&row)).collect()
}

fn map_signal_row(row: &sqlx::sqlite::SqliteRow) -> Result<SignalRecord, StorageError> {
    let payload_str: String = row.try_get("payload_json")?;
    Ok(SignalRecord {
        signal_id: row.try_get("signal_id")?,
        signal_type: row.try_get("signal_type")?,
        source: row.try_get("source")?,
        source_ref: row.try_get("source_ref")?,
        timestamp: row.try_get("timestamp")?,
        payload_json: serde_json::from_str(&payload_str).unwrap_or_else(|_| json!({})),
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
    async fn insert_signal_deduplicates_by_source_and_source_ref() {
        let pool = test_pool().await;
        let first = insert_signal(
            &pool,
            SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "calendar".to_string(),
                source_ref: Some("event-1".to_string()),
                timestamp: 1_700_000_000,
                payload_json: Some(json!({"title":"first"})),
            },
        )
        .await
        .unwrap();
        let second = insert_signal(
            &pool,
            SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "calendar".to_string(),
                source_ref: Some("event-1".to_string()),
                timestamp: 1_700_000_010,
                payload_json: Some(json!({"title":"second"})),
            },
        )
        .await
        .unwrap();

        assert_eq!(first, second);
        let listed = list_signals(&pool, Some("calendar_event"), None, 10)
            .await
            .unwrap();
        assert_eq!(listed.len(), 1);
    }

    #[tokio::test]
    async fn insert_signal_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;

        {
            let mut tx = pool.begin().await.unwrap();
            let _id = insert_signal_in_tx(
                &mut tx,
                &SignalInsert {
                    signal_type: "chat".to_string(),
                    source: "assistant".to_string(),
                    source_ref: Some("msg-1".to_string()),
                    timestamp: 1_700_000_100,
                    payload_json: Some(json!({"content":"hello"})),
                },
            )
            .await
            .unwrap();
            tx.rollback().await.unwrap();
        }

        let listed = list_signals(&pool, None, None, 10).await.unwrap();
        assert!(listed.is_empty());
    }

    #[tokio::test]
    async fn list_signals_in_window_excludes_future_rows_even_with_desc_ordering() {
        let pool = test_pool().await;

        for timestamp in [1_700_000_100, 1_700_000_200, 1_800_000_000] {
            insert_signal(
                &pool,
                SignalInsert {
                    signal_type: "calendar_event".to_string(),
                    source: "calendar".to_string(),
                    source_ref: Some(format!("event-{timestamp}")),
                    timestamp,
                    payload_json: Some(json!({ "title": format!("event-{timestamp}") })),
                },
            )
            .await
            .unwrap();
        }

        let listed = list_signals_in_window(
            &pool,
            Some("calendar_event"),
            1_700_000_000,
            1_700_001_000,
            10,
        )
        .await
        .unwrap();

        assert_eq!(listed.len(), 2);
        assert!(listed.iter().all(|signal| signal.timestamp < 1_700_001_000));
        assert_eq!(listed[0].timestamp, 1_700_000_200);
        assert_eq!(listed[1].timestamp, 1_700_000_100);
    }
}
