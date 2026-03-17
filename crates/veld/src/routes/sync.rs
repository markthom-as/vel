use axum::extract::{Query, State};
use axum::Json;
use vel_api_types::{
    ApiResponse, BranchSyncRequestData, ClientActionBatchRequest, ClientActionBatchResultData,
    QueuedWorkItemData, QueuedWorkRoutingData, SyncBootstrapData, SyncClusterStateData,
    SyncHeartbeatRequestData, SyncHeartbeatResponseData, SyncResultData, ValidationRequestData,
    WorkAssignmentClaimNextRequestData, WorkAssignmentClaimNextResponseData,
    WorkAssignmentClaimRequestData, WorkAssignmentReceiptData, WorkAssignmentUpdateRequest,
};

use crate::{errors::AppError, routes::response, services, state::AppState};

#[derive(Debug, serde::Deserialize)]
pub struct WorkAssignmentListQuery {
    pub work_request_id: Option<String>,
    pub worker_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct WorkerQueueQuery {
    pub node_id: String,
    pub worker_class: Option<String>,
    pub capability: Option<String>,
}

async fn evaluate_and_broadcast_context(state: &AppState) {
    if services::evaluate::run_and_broadcast(state).await.is_err() {
        tracing::warn!("evaluate after sync failed");
    }
}

pub async fn sync_calendar(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_calendar_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "calendar".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncBootstrapData>>, AppError> {
    let data = services::client_sync::build_sync_bootstrap(&state).await?;
    Ok(response::success(data))
}

pub async fn sync_cluster(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncClusterStateData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = services::client_sync::build_sync_cluster_state(&state).await?;
    Ok(response::success(data))
}

pub async fn sync_heartbeat(
    State(state): State<AppState>,
    Json(payload): Json<SyncHeartbeatRequestData>,
) -> Result<Json<ApiResponse<SyncHeartbeatResponseData>>, AppError> {
    let data = services::client_sync::ingest_worker_heartbeat(&state, payload).await?;
    Ok(response::success(data))
}

pub async fn sync_branch_sync_request(
    State(state): State<AppState>,
    Json(payload): Json<BranchSyncRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    let data =
        services::client_sync::queue_branch_sync_request(&state, payload, "sync_route", None)
            .await?;
    Ok(response::success(data))
}

pub async fn sync_validation_request(
    State(state): State<AppState>,
    Json(payload): Json<ValidationRequestData>,
) -> Result<Json<ApiResponse<QueuedWorkRoutingData>>, AppError> {
    let data = services::client_sync::queue_validation_request(&state, payload, "sync_route", None)
        .await?;
    Ok(response::success(data))
}

pub async fn claim_work_assignment(
    State(state): State<AppState>,
    Json(payload): Json<WorkAssignmentClaimRequestData>,
) -> Result<Json<ApiResponse<WorkAssignmentReceiptData>>, AppError> {
    let data = services::client_sync::claim_work_assignment(&state, payload).await?;
    Ok(response::success(data))
}

pub async fn update_work_assignment(
    State(state): State<AppState>,
    Json(payload): Json<WorkAssignmentUpdateRequest>,
) -> Result<Json<ApiResponse<WorkAssignmentReceiptData>>, AppError> {
    let data = services::client_sync::update_work_assignment_receipt(&state, payload).await?;
    Ok(response::success(data))
}

pub async fn list_work_assignments(
    State(state): State<AppState>,
    Query(query): Query<WorkAssignmentListQuery>,
) -> Result<Json<ApiResponse<Vec<WorkAssignmentReceiptData>>>, AppError> {
    let data = services::client_sync::list_work_assignment_receipts(
        &state,
        query.work_request_id.as_deref(),
        query.worker_id.as_deref(),
    )
    .await?;
    Ok(response::success(data))
}

pub async fn list_worker_queue(
    State(state): State<AppState>,
    Query(query): Query<WorkerQueueQuery>,
) -> Result<Json<ApiResponse<Vec<QueuedWorkItemData>>>, AppError> {
    let data = services::client_sync::list_worker_queue(
        &state,
        &query.node_id,
        query.worker_class.as_deref(),
        query.capability.as_deref(),
    )
    .await?;
    Ok(response::success(data))
}

pub async fn claim_next_worker_queue_item(
    State(state): State<AppState>,
    Json(payload): Json<WorkAssignmentClaimNextRequestData>,
) -> Result<Json<ApiResponse<WorkAssignmentClaimNextResponseData>>, AppError> {
    let data = services::client_sync::claim_next_work_for_worker(&state, payload).await?;
    Ok(response::success(data))
}

pub async fn sync_actions(
    State(state): State<AppState>,
    Json(payload): Json<ClientActionBatchRequest>,
) -> Result<Json<ApiResponse<ClientActionBatchResultData>>, AppError> {
    if payload.actions.is_empty() {
        return Err(AppError::bad_request("actions must not be empty"));
    }
    if payload.actions.len() > 200 {
        return Err(AppError::bad_request("actions batch exceeds 200"));
    }

    let data = services::client_sync::apply_client_actions(&state, payload).await?;
    if data.applied > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(data))
}

pub async fn sync_todoist(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_todoist_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "todoist".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_activity(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_activity_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "activity".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_health(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_health_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "health".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_git(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_git_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "git".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_messaging(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_messaging_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "messaging".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_reminders(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_reminders_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "reminders".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_notes(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_notes_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "notes".to_string(),
        signals_ingested: count,
    }))
}

pub async fn sync_transcripts(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_transcripts_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    Ok(response::success(SyncResultData {
        source: "transcripts".to_string(),
        signals_ingested: count,
    }))
}
