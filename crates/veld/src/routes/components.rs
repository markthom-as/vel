use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use serde_json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, ComponentData, ComponentLogEventData, WsEventType};

use crate::{
    broadcast::WsEnvelope,
    errors::AppError,
    services::{
        self,
        components::{ComponentListItem, ComponentLogEvent, ComponentRestartResult},
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct ComponentLogsQuery {
    pub limit: Option<u32>,
}

fn map_component(item: ComponentListItem) -> ComponentData {
    ComponentData {
        id: item.id,
        name: item.name,
        description: item.description,
        status: item.status,
        last_restarted_at: item.last_restarted_at,
        last_error: item.last_error,
        restart_count: item.restart_count,
    }
}

fn map_component_log(event: ComponentLogEvent) -> ComponentLogEventData {
    ComponentLogEventData {
        id: event.id,
        component_id: event.component_id,
        event_name: event.event_name,
        status: event.status,
        message: event.message,
        payload: event.payload,
        created_at: event.created_at,
    }
}

fn map_restart_component(result: ComponentRestartResult) -> ComponentData {
    ComponentData {
        id: result.id,
        name: result.name,
        description: result.description,
        status: result.status,
        last_restarted_at: result.last_restarted_at,
        last_error: result.last_error,
        restart_count: result.restart_count,
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
    let request_id = format!("req_{}", Uuid::new_v4().simple());
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
            .map(map_component_log)
            .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
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
    let component = map_restart_component(component);
    let payload = serde_json::to_value(&component).map_err(|error| {
        AppError::internal(format!("serialize component for websocket: {error}"))
    })?;
    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(WsEventType::ComponentsUpdated, payload));
    if component_id == "evaluate" {
        let _ = crate::routes::evaluate::broadcast_context_updated(&state).await;
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(component, request_id)))
}
