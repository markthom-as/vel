use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use vel_core::RunStatus;
use vel_api_types::{ApiResponse, ArtifactSummaryData, RunDetailData, RunEventData, RunSummaryData, RunUpdateRequest};

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ListRunsQuery {
    pub limit: Option<u32>,
    pub kind: Option<String>,
    pub today: Option<bool>,
}

fn duration_ms(
    started_at: Option<time::OffsetDateTime>,
    finished_at: Option<time::OffsetDateTime>,
) -> Option<i64> {
    started_at.and_then(|s| finished_at.map(|f| (f - s).whole_milliseconds()))
}

fn start_of_today_utc() -> i64 {
    let now = time::OffsetDateTime::now_utc();
    let date = now.date();
    date.midnight().assume_utc().unix_timestamp()
}

pub async fn list_runs(
    State(state): State<AppState>,
    Query(q): Query<ListRunsQuery>,
) -> Result<Json<ApiResponse<Vec<RunSummaryData>>>, AppError> {
    let limit = q.limit.unwrap_or(20).clamp(1, 100);
    let kind_filter = q.kind.as_deref().filter(|s| !s.is_empty());
    let since_ts = q.today.unwrap_or(false).then(start_of_today_utc);
    let runs = state.storage.list_runs(limit, kind_filter, since_ts).await?;
    let data = runs
        .into_iter()
        .map(|r| RunSummaryData {
            id: r.id,
            kind: r.kind.to_string(),
            status: r.status.to_string(),
            created_at: r.created_at,
            started_at: r.started_at,
            finished_at: r.finished_at,
            duration_ms: duration_ms(r.started_at, r.finished_at),
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

    let refs_from_run = state.storage.list_refs_from("run", run.id.as_ref()).await?;
    let mut artifacts = Vec::new();
    for ref_ in refs_from_run {
        if ref_.to_type == "artifact" {
            if let Some(record) = state.storage.get_artifact_by_id(&ref_.to_id).await? {
                artifacts.push(ArtifactSummaryData {
                    artifact_id: record.artifact_id,
                    artifact_type: record.artifact_type,
                    title: record.title,
                    storage_uri: record.storage_uri,
                    storage_kind: record.storage_kind.to_string(),
                    size_bytes: record.size_bytes,
                });
            }
        }
    }

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
            duration_ms: duration_ms(run.started_at, run.finished_at),
            events: event_data,
            artifacts,
        },
        request_id,
    )))
}

pub async fn update_run(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<RunUpdateRequest>,
) -> Result<Json<ApiResponse<RunDetailData>>, AppError> {
    let status: RunStatus = body
        .status
        .trim()
        .parse()
        .map_err(|e: vel_core::VelCoreError| AppError::bad_request(e.to_string()))?;
    let id = id.trim();
    let _existing = state
        .storage
        .get_run_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("run not found"))?;
    state
        .storage
        .update_run_status(id, status, None, None, None, None)
        .await?;
    get_run(State(state), Path(id.to_string())).await
}
