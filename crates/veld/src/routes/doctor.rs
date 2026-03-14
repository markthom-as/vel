//! Diagnostic endpoint for `vel doctor`. Thin handler that calls the doctor service.

use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, DoctorData};

use crate::{errors::AppError, state::AppState};
use crate::services::doctor;

pub async fn doctor(State(state): State<AppState>) -> Result<Json<ApiResponse<DoctorData>>, AppError> {
    let data = doctor::run_diagnostics(&state).await;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
