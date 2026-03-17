use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{ApiResponse, ConnectInstanceData};

use crate::{errors::AppError, state::AppState};

pub async fn list_instances(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ConnectInstanceData>>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::connect::list_connect_instances(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_instance(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
) -> Result<Json<ApiResponse<ConnectInstanceData>>, AppError> {
    state.storage.healthcheck().await?;
    let data = crate::services::connect::get_connect_instance(&state, instance_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("connect instance not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
