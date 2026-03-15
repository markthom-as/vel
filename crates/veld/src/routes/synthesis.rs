use axum::{extract::Path, extract::State, Json};
use uuid::Uuid;
use vel_api_types::ApiResponse;

use crate::{errors::AppError, state::AppState};
use crate::services::synthesis;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SynthesisWeekResponse {
    pub run_id: String,
    pub artifact_id: String,
}

pub async fn synthesis_week(State(state): State<AppState>) -> Result<Json<ApiResponse<SynthesisWeekResponse>>, AppError> {
    let (run_id, artifact_id) = synthesis::run_week_synthesis(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SynthesisWeekResponse {
            run_id: run_id.to_string(),
            artifact_id: artifact_id.to_string(),
        },
        request_id,
    )))
}

pub async fn synthesis_project(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<SynthesisWeekResponse>>, AppError> {
    let (run_id, artifact_id) = synthesis::run_project_synthesis(&state, slug.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        SynthesisWeekResponse {
            run_id: run_id.to_string(),
            artifact_id: artifact_id.to_string(),
        },
        request_id,
    )))
}
