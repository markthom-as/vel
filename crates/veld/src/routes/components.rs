use axum::{extract::{Path, Query, State}, Json};
use serde::Deserialize;
use uuid::Uuid;
use vel_api_types::{ApiResponse, ComponentData, ComponentLogEventData};

use crate::{
    errors::AppError,
    services,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ComponentLogsQuery {
    pub limit: Option<u32>,
}

pub async fn list_components(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ComponentData>>>, AppError> {
    let components = services::components::list_components(&state.storage).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(components, request_id)))
}

pub async fn list_component_logs(
    Path(component_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<ComponentLogsQuery>,
) -> Result<Json<ApiResponse<Vec<ComponentLogEventData>>>, AppError> {
    let logs = services::components::list_component_logs(
        &state.storage,
        component_id.trim(),
        query.limit,
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(logs, request_id)))
}

pub async fn restart_component(
    Path(component_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ComponentData>>, AppError> {
    let component = services::components::restart_component(
        &state.storage,
        &state.config,
        &state.policy_config,
        component_id.trim(),
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(component, request_id)))
}

