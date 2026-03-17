use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ClientActionBatchRequest, ClientActionBatchResultData, SyncBootstrapData,
    SyncClusterStateData, SyncResultData,
};

use crate::{errors::AppError, services, state::AppState};

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
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "calendar".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_bootstrap(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncBootstrapData>>, AppError> {
    let data = services::client_sync::build_sync_bootstrap(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn sync_cluster(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncClusterStateData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = services::client_sync::build_sync_cluster_state(&state);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
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
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn sync_todoist(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_todoist_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "todoist".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_activity(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_activity_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "activity".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_health(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_health_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "health".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_git(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_git_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "git".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_messaging(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_messaging_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "messaging".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_notes(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_notes_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "notes".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}

pub async fn sync_transcripts(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = services::integrations::run_transcripts_sync(&state.storage, &state.config).await?;
    if count > 0 {
        evaluate_and_broadcast_context(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "transcripts".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}
