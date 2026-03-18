use axum::extract::State;
use axum::Json;
use vel_api_types::{ApiResponse, DiagnosticsData, FreshnessEntryData};

use crate::{errors::AppError, routes::response, services, state::AppState};

/// GET /api/diagnostics — operator-authenticated
///
/// Returns a diagnostics snapshot: node identity, overall sync status, active worker count,
/// aggregated capability summary, and per-worker freshness entries.
///
/// This endpoint surfaces already-available data from the worker registry; it does not
/// introduce new backend behavior. It is the visibility closure for SP1 Lane B (ticket 019).
pub async fn get_diagnostics(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DiagnosticsData>>, AppError> {
    let workers_data = services::client_sync::cluster_workers_data(&state).await?;
    let bootstrap = services::client_sync::effective_cluster_bootstrap_data(&state).await?;

    let now = workers_data.generated_at;

    // Derive overall sync status from worker statuses.
    let sync_status = if workers_data.workers.is_empty() {
        "unknown".to_string()
    } else {
        let any_ready = workers_data.workers.iter().any(|w| w.status == "ready");
        let all_offline = workers_data
            .workers
            .iter()
            .all(|w| w.status == "offline" || w.status == "unknown");
        if any_ready {
            "ready".to_string()
        } else if all_offline {
            "offline".to_string()
        } else {
            "degraded".to_string()
        }
    };

    // Collect unique capability strings across all active workers.
    let mut capability_set: Vec<String> = Vec::new();
    for worker in &workers_data.workers {
        for cap in &worker.capabilities {
            if !capability_set.contains(cap) {
                capability_set.push(cap.clone());
            }
        }
    }
    capability_set.sort();

    // Build per-worker freshness entries.
    // A worker is considered "fresh" if it has heartbeated within the last 5 minutes,
    // "stale" if it has heartbeated more than 5 minutes ago, or "missing" if no timestamp.
    const FRESH_THRESHOLD_SECONDS: i64 = 5 * 60;
    let freshness_entries: Vec<FreshnessEntryData> = workers_data
        .workers
        .iter()
        .map(|worker| {
            let last_seen_at = Some(worker.last_heartbeat_at);
            let status = if now - worker.last_heartbeat_at <= FRESH_THRESHOLD_SECONDS {
                "fresh".to_string()
            } else {
                "stale".to_string()
            };
            FreshnessEntryData {
                source: worker.worker_id.clone(),
                last_seen_at,
                status,
            }
        })
        .collect();

    let diagnostics = DiagnosticsData {
        node_id: bootstrap.node_id,
        node_display_name: bootstrap.node_display_name,
        generated_at: now,
        sync_status,
        active_workers: workers_data.workers.len() as u32,
        capability_summary: capability_set,
        freshness_entries,
    };

    Ok(response::success(diagnostics))
}
