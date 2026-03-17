use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, SyncResultData};

use crate::{adapters, errors::AppError, services, state::AppState};

pub async fn sync_calendar(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SyncResultData>>, AppError> {
    let count =
        match services::integrations::sync_google_calendar(&state.storage, &state.config).await {
            Ok(Some(count)) => count,
            Ok(None) => adapters::calendar::ingest(&state.storage, &state.config).await?,
            Err(error) => {
                let _ = services::integrations::record_sync_error(
                    &state.storage,
                    "google_calendar",
                    &error.to_string(),
                )
                .await;
                return Err(error);
            }
        };
    let _ = services::evaluate::run(&state.storage, &state.policy_config).await;
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
    let count = match services::integrations::sync_todoist(&state.storage).await {
        Ok(Some(count)) => count,
        Ok(None) => adapters::todoist::ingest(&state.storage, &state.config).await?,
        Err(error) => {
            let _ = services::integrations::record_sync_error(
                &state.storage,
                "todoist",
                &error.to_string(),
            )
            .await;
            return Err(error);
        }
    };
    let _ = services::evaluate::run(&state.storage, &state.policy_config).await;
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
    let count = match adapters::activity::ingest(&state.storage, &state.config).await {
        Ok(count) => count,
        Err(error) => {
            let _ = services::integrations::record_sync_error(
                &state.storage,
                "activity",
                &error.to_string(),
            )
            .await;
            return Err(error);
        }
    };
    let _ = services::integrations::record_sync_success(&state.storage, "activity", count).await;
    let _ = services::evaluate::run(&state.storage, &state.policy_config).await;
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
    let _ = services::evaluate::run(&state.storage, &state.policy_config).await;
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
    let count = match adapters::messaging::ingest(&state.storage, &state.config).await {
        Ok(count) => count,
        Err(error) => {
            let _ = services::integrations::record_sync_error(
                &state.storage,
                "messaging",
                &error.to_string(),
            )
            .await;
            return Err(error);
        }
    };
    let _ = services::integrations::record_sync_success(&state.storage, "messaging", count).await;
    let _ = services::evaluate::run(&state.storage, &state.policy_config).await;
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
    let _ = services::evaluate::run(&state.storage, &state.policy_config).await;
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
    let _ = services::evaluate::run(&state.storage, &state.policy_config).await;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SyncResultData {
            source: "transcripts".to_string(),
            signals_ingested: count,
        },
        request_id,
    )))
}
