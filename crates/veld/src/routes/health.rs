use axum::{extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, HealthData};

use crate::{errors::AppError, state::AppState};

pub async fn health(State(state): State<AppState>) -> Result<Json<ApiResponse<HealthData>>, AppError> {
    state.storage.healthcheck().await?;

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        HealthData {
            status: "ok".to_string(),
            db: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        request_id,
    )))
}

