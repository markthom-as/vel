use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::db::StorageError;

#[derive(Debug, Clone)]
pub struct ConnectRunRecord {
    pub id: String,
    pub agent_id: String,
    pub node_id: String,
    pub status: String,
    pub capabilities_json: String,
    pub lease_expires_at: i64,
    pub started_at: i64,
    pub terminated_at: Option<i64>,
    pub terminal_reason: Option<String>,
}

pub(crate) async fn insert_connect_run(
    pool: &SqlitePool,
    id: &str,
    agent_id: &str,
    node_id: &str,
    capabilities_json: &str,
    lease_expires_at: i64,
    started_at: i64,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO connect_runs (id, agent_id, node_id, status, capabilities_json, lease_expires_at, started_at)
        VALUES (?, ?, ?, 'running', ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(agent_id)
    .bind(node_id)
    .bind(capabilities_json)
    .bind(lease_expires_at)
    .bind(started_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn update_heartbeat(
    pool: &SqlitePool,
    id: &str,
    new_lease_expires_at: i64,
) -> Result<(), StorageError> {
    let result = sqlx::query(
        r#"
        UPDATE connect_runs
        SET lease_expires_at = ?
        WHERE id = ? AND status = 'running'
        "#,
    )
    .bind(new_lease_expires_at)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        // Check if the run exists at all
        let exists: Option<i64> = sqlx::query_scalar("SELECT 1 FROM connect_runs WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        if exists.is_none() {
            return Err(StorageError::NotFound(format!(
                "connect run not found: {}",
                id
            )));
        }
        return Err(StorageError::Validation(format!(
            "connect run {} is not in 'running' state",
            id
        )));
    }
    Ok(())
}

pub(crate) async fn expire_stale_runs(pool: &SqlitePool, now_ts: i64) -> Result<u64, StorageError> {
    let result = sqlx::query(
        r#"
        UPDATE connect_runs
        SET status = 'expired', terminal_reason = 'lease_expired', terminated_at = ?
        WHERE lease_expires_at < ? AND status = 'running'
        "#,
    )
    .bind(now_ts)
    .bind(now_ts)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn terminate_run(
    pool: &SqlitePool,
    id: &str,
    now_ts: i64,
    reason: &str,
) -> Result<(), StorageError> {
    let result = sqlx::query(
        r#"
        UPDATE connect_runs
        SET status = 'terminated', terminal_reason = ?, terminated_at = ?
        WHERE id = ? AND status IN ('running', 'degraded')
        "#,
    )
    .bind(reason)
    .bind(now_ts)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        // Check if the run exists at all
        let exists: Option<i64> = sqlx::query_scalar("SELECT 1 FROM connect_runs WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        if exists.is_none() {
            return Err(StorageError::NotFound(format!(
                "connect run not found: {}",
                id
            )));
        }
        return Err(StorageError::Validation(format!(
            "connect run {} is not in a terminable state (running or degraded)",
            id
        )));
    }
    Ok(())
}

pub(crate) async fn get_connect_run(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<ConnectRunRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT id, agent_id, node_id, status, capabilities_json,
               lease_expires_at, started_at, terminated_at, terminal_reason
        FROM connect_runs
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(map_connect_run_row(&row)?)),
        None => Ok(None),
    }
}

pub(crate) async fn list_connect_runs(
    pool: &SqlitePool,
    status_filter: Option<&str>,
) -> Result<Vec<ConnectRunRecord>, StorageError> {
    let rows = if let Some(status) = status_filter {
        sqlx::query(
            r#"
            SELECT id, agent_id, node_id, status, capabilities_json,
                   lease_expires_at, started_at, terminated_at, terminal_reason
            FROM connect_runs
            WHERE status = ?
            ORDER BY started_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, agent_id, node_id, status, capabilities_json,
                   lease_expires_at, started_at, terminated_at, terminal_reason
            FROM connect_runs
            ORDER BY started_at DESC
            "#,
        )
        .fetch_all(pool)
        .await?
    };

    rows.into_iter()
        .map(|row| map_connect_run_row(&row))
        .collect()
}

fn map_connect_run_row(row: &sqlx::sqlite::SqliteRow) -> Result<ConnectRunRecord, StorageError> {
    Ok(ConnectRunRecord {
        id: row.try_get("id")?,
        agent_id: row.try_get("agent_id")?,
        node_id: row.try_get("node_id")?,
        status: row.try_get("status")?,
        capabilities_json: row.try_get("capabilities_json")?,
        lease_expires_at: row.try_get("lease_expires_at")?,
        started_at: row.try_get("started_at")?,
        terminated_at: row.try_get("terminated_at")?,
        terminal_reason: row.try_get("terminal_reason")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");
    static TEST_DB_COUNTER: AtomicU64 = AtomicU64::new(0);

    /// Create an isolated in-memory database with a unique name per call.
    /// SQLite named in-memory databases (file:name?mode=memory&cache=shared) are
    /// process-scoped, so each test gets its own logical database.
    async fn make_pool() -> SqlitePool {
        let id = TEST_DB_COUNTER.fetch_add(1, Ordering::Relaxed);
        let url = format!(
            "file:connect_runs_{}_{}?mode=memory&cache=shared",
            std::process::id(),
            id
        );
        let pool = SqlitePool::connect(&url).await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_insert_and_get_connect_run() {
        let pool = make_pool().await;
        let now = OffsetDateTime::now_utc().unix_timestamp();

        insert_connect_run(
            &pool,
            "run-001",
            "agent-a",
            "node-1",
            r#"["read:context"]"#,
            now + 60,
            now,
        )
        .await
        .expect("insert");

        let record = get_connect_run(&pool, "run-001")
            .await
            .expect("get")
            .expect("should exist");

        assert_eq!(record.id, "run-001");
        assert_eq!(record.agent_id, "agent-a");
        assert_eq!(record.node_id, "node-1");
        assert_eq!(record.status, "running");
        assert_eq!(record.capabilities_json, r#"["read:context"]"#);
        assert_eq!(record.lease_expires_at, now + 60);
        assert_eq!(record.started_at, now);
        assert!(record.terminated_at.is_none());
        assert!(record.terminal_reason.is_none());
    }

    #[tokio::test]
    async fn test_update_heartbeat() {
        let pool = make_pool().await;
        let now = OffsetDateTime::now_utc().unix_timestamp();

        insert_connect_run(&pool, "run-002", "agent-b", "node-1", "[]", now + 60, now)
            .await
            .expect("insert");

        let new_expiry = now + 120;
        update_heartbeat(&pool, "run-002", new_expiry)
            .await
            .expect("heartbeat");

        let record = get_connect_run(&pool, "run-002")
            .await
            .expect("get")
            .expect("exists");
        assert_eq!(record.lease_expires_at, new_expiry);
    }

    #[tokio::test]
    async fn test_expire_stale_runs_leaves_fresh_untouched() {
        let pool = make_pool().await;
        let now = OffsetDateTime::now_utc().unix_timestamp();

        // Stale run: already expired
        insert_connect_run(
            &pool,
            "run-stale",
            "agent-c",
            "node-1",
            "[]",
            now - 10,
            now - 100,
        )
        .await
        .expect("insert stale");

        // Fresh run: not yet expired
        insert_connect_run(&pool, "run-fresh", "agent-d", "node-1", "[]", now + 60, now)
            .await
            .expect("insert fresh");

        let count = expire_stale_runs(&pool, now).await.expect("expire");
        assert_eq!(count, 1);

        let stale = get_connect_run(&pool, "run-stale")
            .await
            .expect("get stale")
            .expect("exists");
        assert_eq!(stale.status, "expired");
        assert_eq!(stale.terminal_reason.as_deref(), Some("lease_expired"));

        let fresh = get_connect_run(&pool, "run-fresh")
            .await
            .expect("get fresh")
            .expect("exists");
        assert_eq!(fresh.status, "running");
    }

    #[tokio::test]
    async fn test_terminate_run() {
        let pool = make_pool().await;
        let now = OffsetDateTime::now_utc().unix_timestamp();

        insert_connect_run(&pool, "run-term", "agent-e", "node-1", "[]", now + 60, now)
            .await
            .expect("insert");

        terminate_run(&pool, "run-term", now, "operator_request")
            .await
            .expect("terminate");

        let record = get_connect_run(&pool, "run-term")
            .await
            .expect("get")
            .expect("exists");
        assert_eq!(record.status, "terminated");
        assert_eq!(record.terminal_reason.as_deref(), Some("operator_request"));
        assert!(record.terminated_at.is_some());
    }

    #[tokio::test]
    async fn test_terminate_already_expired_run_fails() {
        let pool = make_pool().await;
        let now = OffsetDateTime::now_utc().unix_timestamp();

        // Insert a run that's already expired
        insert_connect_run(
            &pool,
            "run-expired",
            "agent-f",
            "node-1",
            "[]",
            now - 10,
            now - 100,
        )
        .await
        .expect("insert");

        // Expire it
        expire_stale_runs(&pool, now).await.expect("expire");

        // Now try to terminate it — should fail
        let result = terminate_run(&pool, "run-expired", now, "test").await;
        assert!(
            result.is_err(),
            "terminating an already-expired run should fail"
        );
    }
}
