//! Context routes: thin handlers that call the run-backed context generation service.

use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, EndOfDayData, MorningData, TodayData};

use crate::{errors::AppError, state::AppState};
use crate::services::context_runs;

pub async fn today(State(state): State<AppState>) -> Result<Json<ApiResponse<TodayData>>, AppError> {
    let output = context_runs::generate_today(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(output.data, request_id)))
}

pub async fn morning(State(state): State<AppState>) -> Result<Json<ApiResponse<MorningData>>, AppError> {
    let output = context_runs::generate_morning(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(output.data, request_id)))
}

pub async fn end_of_day(State(state): State<AppState>) -> Result<Json<ApiResponse<EndOfDayData>>, AppError> {
    let output = context_runs::generate_end_of_day(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(output.data, request_id)))
}
