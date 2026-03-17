use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, NowData};

use crate::{errors::AppError, services, state::AppState};

pub async fn get_now(State(state): State<AppState>) -> Result<Json<ApiResponse<NowData>>, AppError> {
    let data = services::now::get_now(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
