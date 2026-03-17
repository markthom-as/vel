use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use vel_api_types::{ApiResponse, ComponentData, ComponentLogEventData, WsEventType};

use crate::{
    broadcast::WsEnvelope,
    errors::AppError,
    services::{self, components},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ComponentLogsQuery {
    pub limit: Option<u32>,
}

fn map_component(data: components::ComponentData) -> ComponentData {
    ComponentData {
        id: data.id,
        name: data.name,
        description: data.description,
        status: data.status,
        last_restarted_at: data.last_restarted_at,
        last_error: data.last_error,
        restart_count: data.restart_count,
    }
}

fn map_log_event(data: components::ComponentLogEventData) -> ComponentLogEventData {
    ComponentLogEventData {
        id: data.id,
        component_id: data.component_id,
        event_name: data.event_name,
        status: data.status,
        message: data.message,
        payload: data.payload,
        created_at: data.created_at,
    }
}

pub async fn list_components(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ComponentData>>>, AppError> {
    let components = services::components::list_components(&state.storage)
        .await?
        .into_iter()
        .map(map_component)
        .collect();
    let request_id = format!("req_{}", uuid::Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(components, request_id)))
}

pub async fn list_component_logs(
    Path(component_id): Path<String>,
    State(state): State<AppState>,
    Query(query): Query<ComponentLogsQuery>,
) -> Result<Json<ApiResponse<Vec<ComponentLogEventData>>>, AppError> {
    let logs =
        services::components::list_component_logs(&state.storage, component_id.trim(), query.limit)
            .await?
            .into_iter()
            .map(map_log_event)
            .collect();
    let request_id = format!("req_{}", uuid::Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(logs, request_id)))
}

pub async fn restart_component(
    Path(component_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ComponentData>>, AppError> {
    let component_id = component_id.trim().to_string();
    let component = services::components::restart_component(
        &state.storage,
        &state.config,
        &state.policy_config,
        &component_id,
    )
    .await?;

    let api_component = map_component(component.clone());
    let payload = serde_json::to_value(&api_component).map_err(|error| {
        AppError::internal(format!("serialize component for websocket: {error}"))
    })?;

    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(WsEventType::ComponentsUpdated, payload));

    if component_id == "evaluate" {
        let _ = crate::routes::evaluate::broadcast_context_updated(&state).await;
    }

    let request_id = format!("req_{}", uuid::Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(api_component, request_id)))
}
