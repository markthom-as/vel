use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{ApiResponse, RunDetailData, RunEventData, RunSummaryData};

use crate::{errors::AppError, state::AppState};

pub async fn list_runs(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<RunSummaryData>>>, AppError> {
    let runs = state.storage.list_runs(20).await?;
    let data = runs
        .into_iter()
        .map(|r| RunSummaryData {
            id: r.id,
            kind: r.kind.to_string(),
            status: r.status.to_string(),
            created_at: r.created_at,
            started_at: r.started_at,
            finished_at: r.finished_at,
        })
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_run(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<RunDetailData>>, AppError> {
    let run = state
        .storage
        .get_run_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("run not found"))?;
    let events = state.storage.list_run_events(run.id.as_ref()).await?;
    let event_data = events
        .into_iter()
        .map(|e| RunEventData {
            seq: e.seq,
            event_type: e.event_type.to_string(),
            payload: e.payload_json,
            created_at: e.created_at,
        })
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        RunDetailData {
            id: run.id,
            kind: run.kind.to_string(),
            status: run.status.to_string(),
            input: run.input_json,
            output: run.output_json,
            error: run.error_json,
            created_at: run.created_at,
            started_at: run.started_at,
            finished_at: run.finished_at,
            events: event_data,
        },
        request_id,
    )))
}
