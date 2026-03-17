use std::collections::HashMap;

use serde_json::Value as JsonValue;
use sqlx::{QueryBuilder, Row, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::{
    ClusterWorkerRecord, ClusterWorkerUpsert, RuntimeLoopRecord, StorageError,
    WorkAssignmentInsert, WorkAssignmentRecord, WorkAssignmentUpdate,
};

pub(crate) async fn get_all_settings(
    pool: &SqlitePool,
) -> Result<HashMap<String, JsonValue>, StorageError> {
    let rows = sqlx::query_as::<_, (String, String)>("SELECT key, value_json FROM settings")
        .fetch_all(pool)
        .await?;
    let mut out = HashMap::new();
    for (k, v) in rows {
        let val: JsonValue = serde_json::from_str(&v).unwrap_or(JsonValue::Null);
        out.insert(k, val);
    }
    Ok(out)
}

pub(crate) async fn set_setting(
    pool: &SqlitePool,
    key: &str,
    value: &JsonValue,
) -> Result<(), StorageError> {
    let json = serde_json::to_string(value).map_err(|e| StorageError::Validation(e.to_string()))?;
    sqlx::query("INSERT INTO settings (key, value_json) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json")
        .bind(key)
        .bind(&json)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn insert_work_assignment(
    pool: &SqlitePool,
    assignment: WorkAssignmentInsert,
) -> Result<String, StorageError> {
    let receipt_id = assignment
        .receipt_id
        .unwrap_or_else(|| Uuid::new_v4().simple().to_string());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        INSERT INTO work_assignment_receipts (
            receipt_id,
            work_request_id,
            worker_id,
            worker_class,
            capability,
            status,
            assigned_at,
            last_updated
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&receipt_id)
    .bind(&assignment.work_request_id)
    .bind(&assignment.worker_id)
    .bind(&assignment.worker_class)
    .bind(&assignment.capability)
    .bind(assignment.status.to_string())
    .bind(assignment.assigned_at)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(receipt_id)
}

pub(crate) async fn update_work_assignment(
    pool: &SqlitePool,
    update: WorkAssignmentUpdate,
) -> Result<WorkAssignmentRecord, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        UPDATE work_assignment_receipts
        SET status = ?,
            started_at = ?,
            completed_at = ?,
            result = ?,
            error_message = ?,
            last_updated = ?
        WHERE receipt_id = ?
        "#,
    )
    .bind(update.status.to_string())
    .bind(update.started_at)
    .bind(update.completed_at)
    .bind(update.result)
    .bind(update.error_message)
    .bind(now)
    .bind(&update.receipt_id)
    .execute(pool)
    .await?;

    let row = sqlx::query(
        r#"
        SELECT
            receipt_id,
            work_request_id,
            worker_id,
            worker_class,
            capability,
            status,
            assigned_at,
            started_at,
            completed_at,
            result,
            error_message,
            last_updated
        FROM work_assignment_receipts
        WHERE receipt_id = ?
        "#,
    )
    .bind(&update.receipt_id)
    .fetch_one(pool)
    .await?;

    map_work_assignment_row(&row)
}

pub(crate) async fn set_work_assignment_last_updated(
    pool: &SqlitePool,
    receipt_id: &str,
    last_updated: i64,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        UPDATE work_assignment_receipts
        SET last_updated = ?
        WHERE receipt_id = ?
        "#,
    )
    .bind(last_updated)
    .bind(receipt_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn list_work_assignments(
    pool: &SqlitePool,
    work_request_id: Option<&str>,
    worker_id: Option<&str>,
) -> Result<Vec<WorkAssignmentRecord>, StorageError> {
    let mut query = QueryBuilder::new(
        r#"
        SELECT
            receipt_id,
            work_request_id,
            worker_id,
            worker_class,
            capability,
            status,
            assigned_at,
            started_at,
            completed_at,
            result,
            error_message,
            last_updated
        FROM work_assignment_receipts
        "#,
    );
    query.push("WHERE 1=1");
    if work_request_id.is_some() {
        query.push(" AND work_request_id = ");
        query.push_bind(work_request_id);
    }
    if worker_id.is_some() {
        query.push(" AND worker_id = ");
        query.push_bind(worker_id);
    }
    query.push(" ORDER BY last_updated DESC");

    let rows = query.build().fetch_all(pool).await?;
    rows.into_iter()
        .map(|row| map_work_assignment_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn upsert_cluster_worker(
    pool: &SqlitePool,
    worker: ClusterWorkerUpsert,
) -> Result<(), StorageError> {
    let worker_classes_json = serde_json::to_string(&worker.worker_classes)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let capabilities_json = serde_json::to_string(&worker.capabilities)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let now = OffsetDateTime::now_utc().unix_timestamp();

    sqlx::query(
        r#"
        INSERT INTO cluster_workers (
            worker_id,
            node_id,
            node_display_name,
            worker_class,
            worker_classes_json,
            capabilities_json,
            status,
            max_concurrency,
            current_load,
            queue_depth,
            reachability,
            latency_class,
            compute_class,
            power_class,
            recent_failure_rate,
            tailscale_preferred,
            sync_base_url,
            sync_transport,
            tailscale_base_url,
            preferred_tailnet_endpoint,
            tailscale_reachable,
            lan_base_url,
            localhost_base_url,
            last_heartbeat_at,
            started_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(worker_id) DO UPDATE SET
            node_id = excluded.node_id,
            node_display_name = excluded.node_display_name,
            worker_class = excluded.worker_class,
            worker_classes_json = excluded.worker_classes_json,
            capabilities_json = excluded.capabilities_json,
            status = excluded.status,
            max_concurrency = excluded.max_concurrency,
            current_load = excluded.current_load,
            queue_depth = excluded.queue_depth,
            reachability = excluded.reachability,
            latency_class = excluded.latency_class,
            compute_class = excluded.compute_class,
            power_class = excluded.power_class,
            recent_failure_rate = excluded.recent_failure_rate,
            tailscale_preferred = excluded.tailscale_preferred,
            sync_base_url = excluded.sync_base_url,
            sync_transport = excluded.sync_transport,
            tailscale_base_url = excluded.tailscale_base_url,
            preferred_tailnet_endpoint = excluded.preferred_tailnet_endpoint,
            tailscale_reachable = excluded.tailscale_reachable,
            lan_base_url = excluded.lan_base_url,
            localhost_base_url = excluded.localhost_base_url,
            last_heartbeat_at = excluded.last_heartbeat_at,
            started_at = excluded.started_at,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&worker.worker_id)
    .bind(&worker.node_id)
    .bind(&worker.node_display_name)
    .bind(&worker.worker_class)
    .bind(worker_classes_json)
    .bind(capabilities_json)
    .bind(&worker.status)
    .bind(worker.max_concurrency.map(i64::from))
    .bind(worker.current_load.map(i64::from))
    .bind(worker.queue_depth.map(i64::from))
    .bind(&worker.reachability)
    .bind(&worker.latency_class)
    .bind(&worker.compute_class)
    .bind(&worker.power_class)
    .bind(worker.recent_failure_rate)
    .bind(if worker.tailscale_preferred {
        1_i64
    } else {
        0_i64
    })
    .bind(&worker.sync_base_url)
    .bind(&worker.sync_transport)
    .bind(&worker.tailscale_base_url)
    .bind(&worker.preferred_tailnet_endpoint)
    .bind(if worker.tailscale_reachable {
        1_i64
    } else {
        0_i64
    })
    .bind(&worker.lan_base_url)
    .bind(&worker.localhost_base_url)
    .bind(worker.last_heartbeat_at)
    .bind(worker.started_at)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub(crate) async fn expire_cluster_workers(
    pool: &SqlitePool,
    stale_before: i64,
) -> Result<u64, StorageError> {
    let result = sqlx::query("DELETE FROM cluster_workers WHERE last_heartbeat_at < ?")
        .bind(stale_before)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn list_cluster_workers(
    pool: &SqlitePool,
) -> Result<Vec<ClusterWorkerRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            worker_id,
            node_id,
            node_display_name,
            client_kind,
            client_version,
            protocol_version,
            build_id,
            worker_class,
            worker_classes_json,
            capabilities_json,
            status,
            max_concurrency,
            current_load,
            queue_depth,
            reachability,
            latency_class,
            compute_class,
            power_class,
            recent_failure_rate,
            tailscale_preferred,
            sync_base_url,
            sync_transport,
            tailscale_base_url,
            preferred_tailnet_endpoint,
            tailscale_reachable,
            lan_base_url,
            localhost_base_url,
            ping_ms,
            sync_status,
            last_upstream_sync_at,
            last_downstream_sync_at,
            last_sync_error,
            last_heartbeat_at,
            started_at,
            updated_at
        FROM cluster_workers
        ORDER BY node_id ASC, worker_id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_cluster_worker_row(&row))
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) async fn get_runtime_loop(
    pool: &SqlitePool,
    loop_kind: &str,
) -> Result<Option<RuntimeLoopRecord>, StorageError> {
    crate::runtime_loops::get_runtime_loop(pool, loop_kind).await
}

pub(crate) async fn update_runtime_loop_config(
    pool: &SqlitePool,
    loop_kind: &str,
    enabled: Option<bool>,
    interval_seconds: Option<i64>,
) -> Result<Option<RuntimeLoopRecord>, StorageError> {
    crate::runtime_loops::update_runtime_loop_config(pool, loop_kind, enabled, interval_seconds)
        .await
}

fn map_cluster_worker_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<ClusterWorkerRecord, StorageError> {
    let worker_classes_json: String = row.try_get("worker_classes_json")?;
    let capabilities_json: String = row.try_get("capabilities_json")?;
    let tailscale_preferred: i64 = row.try_get("tailscale_preferred")?;
    let tailscale_reachable: i64 = row.try_get("tailscale_reachable")?;

    Ok(ClusterWorkerRecord {
        worker_id: row.try_get("worker_id")?,
        node_id: row.try_get("node_id")?,
        node_display_name: row.try_get("node_display_name")?,
        client_kind: row.try_get("client_kind")?,
        client_version: row.try_get("client_version")?,
        protocol_version: row.try_get("protocol_version")?,
        build_id: row.try_get("build_id")?,
        worker_class: row.try_get("worker_class")?,
        worker_classes: serde_json::from_str(&worker_classes_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?,
        capabilities: serde_json::from_str(&capabilities_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?,
        status: row.try_get("status")?,
        max_concurrency: row
            .try_get::<Option<i64>, _>("max_concurrency")?
            .map(|value| value.max(0) as u32),
        current_load: row
            .try_get::<Option<i64>, _>("current_load")?
            .map(|value| value.max(0) as u32),
        queue_depth: row
            .try_get::<Option<i64>, _>("queue_depth")?
            .map(|value| value.max(0) as u32),
        reachability: row.try_get("reachability")?,
        latency_class: row.try_get("latency_class")?,
        compute_class: row.try_get("compute_class")?,
        power_class: row.try_get("power_class")?,
        recent_failure_rate: row.try_get("recent_failure_rate")?,
        tailscale_preferred: tailscale_preferred != 0,
        sync_base_url: row.try_get("sync_base_url")?,
        sync_transport: row.try_get("sync_transport")?,
        tailscale_base_url: row.try_get("tailscale_base_url")?,
        preferred_tailnet_endpoint: row.try_get("preferred_tailnet_endpoint")?,
        tailscale_reachable: tailscale_reachable != 0,
        lan_base_url: row.try_get("lan_base_url")?,
        localhost_base_url: row.try_get("localhost_base_url")?,
        ping_ms: row
            .try_get::<Option<i64>, _>("ping_ms")?
            .map(|value| value.max(0) as u32),
        sync_status: row.try_get("sync_status")?,
        last_upstream_sync_at: row.try_get("last_upstream_sync_at")?,
        last_downstream_sync_at: row.try_get("last_downstream_sync_at")?,
        last_sync_error: row.try_get("last_sync_error")?,
        last_heartbeat_at: row.try_get("last_heartbeat_at")?,
        started_at: row.try_get("started_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_work_assignment_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<WorkAssignmentRecord, StorageError> {
    let status_str: String = row.try_get("status")?;
    let status = status_str.parse().map_err(|e: vel_core::VelCoreError| {
        StorageError::Validation(format!("invalid work assignment status: {e}"))
    })?;

    Ok(WorkAssignmentRecord {
        receipt_id: row.try_get("receipt_id")?,
        work_request_id: row.try_get("work_request_id")?,
        worker_id: row.try_get("worker_id")?,
        worker_class: row.try_get("worker_class")?,
        capability: row.try_get("capability")?,
        status,
        assigned_at: row.try_get("assigned_at")?,
        started_at: row.try_get("started_at")?,
        completed_at: row.try_get("completed_at")?,
        result: row.try_get("result")?,
        error_message: row.try_get("error_message")?,
        last_updated: row.try_get("last_updated")?,
    })
}
