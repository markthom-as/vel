use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, SyncResultData};

use crate::{adapters, errors::AppError, services, state::AppState};

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

pub async fn sync_git(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count = match adapters::git::ingest(&state.storage, &state.config).await {
        Ok(count) => count,
        Err(error) => {
            let _ = services::integrations::record_sync_error(
                &state.storage,
                "git",
                &error.to_string(),
            )
            .await;
            return Err(error);
        }
    };
    let _ = services::integrations::record_sync_success(&state.storage, "git", count).await;
    evaluate_and_broadcast_context(&state).await;
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
    let count = match adapters::notes::ingest(&state.storage, &state.config).await {
        Ok(count) => count,
        Err(error) => {
            let _ = services::integrations::record_sync_error(
                &state.storage,
                "notes",
                &error.to_string(),
            )
            .await;
            return Err(error);
        }
    };
    let _ = services::integrations::record_sync_success(&state.storage, "notes", count).await;
    evaluate_and_broadcast_context(&state).await;
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
    let count = match adapters::transcripts::ingest(&state.storage, &state.config).await {
        Ok(count) => count,
        Err(error) => {
            let _ = services::integrations::record_sync_error(
                &state.storage,
                "transcripts",
                &error.to_string(),
            )
            .await;
            return Err(error);
        }
    };
    let _ = services::integrations::record_sync_success(&state.storage, "transcripts", count).await;
    evaluate_and_broadcast_context(&state).await;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "transcripts".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}
