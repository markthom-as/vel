use sqlx::{Row, SqlitePool};

use crate::db::{BrokerEventRecord, StorageError};

pub(crate) async fn insert_broker_event(
    pool: &SqlitePool,
    id: &str,
    event_type: &str,
    run_id: &str,
    scope: &str,
    resource: Option<&str>,
    action: &str,
    reason: Option<&str>,
    occurred_at: i64,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO broker_events (id, event_type, run_id, scope, resource, action, reason, occurred_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(event_type)
    .bind(run_id)
    .bind(scope)
    .bind(resource)
    .bind(action)
    .bind(reason)
    .bind(occurred_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn list_broker_events(
    pool: &SqlitePool,
    run_id: &str,
) -> Result<Vec<BrokerEventRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT id, event_type, run_id, scope, resource, action, reason, occurred_at
        FROM broker_events
        WHERE run_id = ?
        ORDER BY occurred_at ASC
        "#,
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_broker_event_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

fn map_broker_event_row(row: &sqlx::sqlite::SqliteRow) -> Result<BrokerEventRecord, StorageError> {
    Ok(BrokerEventRecord {
        id: row.try_get("id")?,
        event_type: row.try_get("event_type")?,
        run_id: row.try_get("run_id")?,
        scope: row.try_get("scope")?,
        resource: row.try_get("resource")?,
        action: row.try_get("action")?,
        reason: row.try_get("reason")?,
        occurred_at: row.try_get("occurred_at")?,
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
    async fn insert_broker_event_persists_grant_event() {
        let pool = test_pool().await;
        insert_broker_event(
            &pool,
            "evt_grant_1",
            "grant",
            "run_abc",
            "read:context",
            None,
            "read",
            None,
            1_700_000_000,
        )
        .await
        .unwrap();
        let events = list_broker_events(&pool, "run_abc").await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "grant");
        assert_eq!(events[0].scope, "read:context");
    }

    #[tokio::test]
    async fn list_broker_events_filters_by_run_id() {
        let pool = test_pool().await;
        insert_broker_event(
            &pool,
            "evt_1",
            "deny",
            "run_one",
            "write:captures",
            None,
            "write",
            Some("scope not in allowlist"),
            1_700_000_001,
        )
        .await
        .unwrap();
        insert_broker_event(
            &pool,
            "evt_2",
            "grant",
            "run_two",
            "read:context",
            None,
            "read",
            None,
            1_700_000_002,
        )
        .await
        .unwrap();
        let events = list_broker_events(&pool, "run_one").await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].run_id, "run_one");
    }
}
