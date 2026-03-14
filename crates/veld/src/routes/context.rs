//! Context routes: thin handlers that call the context generation service.

use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, EndOfDayData, MorningData, TodayData};

use crate::{errors::AppError, state::AppState};
use crate::services::context_generation;

pub async fn today(State(state): State<AppState>) -> Result<Json<ApiResponse<TodayData>>, AppError> {
    let snapshot = state.storage.orientation_snapshot().await?;
    let data = context_generation::build_today(&snapshot);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn morning(State(state): State<AppState>) -> Result<Json<ApiResponse<MorningData>>, AppError> {
    let snapshot = state.storage.orientation_snapshot().await?;
    let data = context_generation::build_morning(&snapshot);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn end_of_day(State(state): State<AppState>) -> Result<Json<ApiResponse<EndOfDayData>>, AppError> {
    let snapshot = state.storage.orientation_snapshot().await?;
    let data = context_generation::build_end_of_day(&snapshot);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
