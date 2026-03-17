//! Context routes: thin handlers that call the run-backed context generation service and expose current context.

use axum::{
    extract::{Query, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ContextTimelineEntry, CurrentContextData, EndOfDayData, MorningData, TodayData,
};
use vel_core::ContextMigrator;

use crate::services::context_runs;
use crate::{errors::AppError, state::AppState};

#[derive(Debug, serde::Deserialize)]
pub struct ContextTimelineQuery {
    pub limit: Option<u32>,
}

pub async fn today(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<TodayData>>, AppError> {
    let output = context_runs::generate_today(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(output.data, request_id)))
}

pub async fn morning(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MorningData>>, AppError> {
    let output = context_runs::generate_morning(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(output.data, request_id)))
}

pub async fn end_of_day(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<EndOfDayData>>, AppError> {
    let output = context_runs::generate_end_of_day(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(output.data, request_id)))
}

/// GET /v1/context/current — persistent current context (singleton) written by inference engine.
pub async fn current(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Option<CurrentContextData>>>, AppError> {
    let row = state.storage.get_current_context().await?;
    let data = row.map(|(computed_at, context_str)| {
        let context = serde_json::from_str(&context_str).unwrap_or(serde_json::json!({}));
        let _ = ContextMigrator::from_json_value(context.clone());
        CurrentContextData {
            computed_at,
            context,
        }
    });
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

/// GET /v1/context/timeline — recent material context transitions (append-only timeline).
pub async fn timeline(
    State(state): State<AppState>,
    Query(q): Query<ContextTimelineQuery>,
) -> Result<Json<ApiResponse<Vec<ContextTimelineEntry>>>, AppError> {
    let limit = q.limit.unwrap_or(20);
    let rows = state.storage.list_context_timeline(limit).await?;
    let entries: Vec<ContextTimelineEntry> = rows
        .into_iter()
        .filter_map(|(id, timestamp, context_json)| {
            let context = serde_json::from_str(&context_json).ok()?;
            Some(ContextTimelineEntry {
                id,
                timestamp,
                context,
            })
        })
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(entries, request_id)))
}
