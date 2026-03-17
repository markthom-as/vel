use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;

use crate::db::{ClusterWorkerRecord, ClusterWorkerUpsert, StorageError};

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
