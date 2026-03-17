use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use uuid::Uuid;
use vel_api_types::{AgentRuntimeViewData, AgentSpecData, ApiResponse};
use vel_core::{AgentPriority, AgentSpawnRequest, RunId};

use crate::{errors::AppError, services::agents, state::AppState};

#[derive(Debug, Deserialize)]
pub struct SpawnAgentRequest {
    #[serde(alias = "agent_id", alias = "spec_id")]
    pub spec_id: String,
    #[serde(default, alias = "mission_input", alias = "input")]
    pub mission_input: JsonValue,
    #[serde(default)]
    pub parent_run_id: Option<String>,
    #[serde(default)]
    pub deadline: Option<OffsetDateTime>,
    #[serde(default)]
    pub deadline_ts: Option<i64>,
    #[serde(default)]
    pub priority: Option<String>,
}

pub async fn list_specs(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AgentSpecData>>>, AppError> {
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        agents::list_spec_data(&state.config)?,
        request_id,
    )))
}

pub async fn spawn_run(
    State(state): State<AppState>,
    Json(body): Json<SpawnAgentRequest>,
) -> Result<Json<ApiResponse<AgentRuntimeViewData>>, AppError> {
    let request = AgentSpawnRequest {
        agent_id: body.spec_id,
        mission_input: body.mission_input,
        parent_run_id: body.parent_run_id.map(RunId::from),
        deadline: match (body.deadline, body.deadline_ts) {
            (Some(deadline), _) => Some(deadline),
            (None, Some(ts)) => Some(OffsetDateTime::from_unix_timestamp(ts).map_err(|_| {
                AppError::bad_request("deadline_ts must be a valid unix timestamp")
            })?),
            (None, None) => None,
        },
        priority: body.priority.as_deref().map(parse_priority).transpose()?,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        agents::spawn_run(&state.storage, &state.config, request).await?,
        request_id,
    )))
}

pub async fn get_run(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<AgentRuntimeViewData>>, AppError> {
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        agents::get_run_view(&state.storage, &state.config, id.trim()).await?,
        request_id,
    )))
}

fn parse_priority(value: &str) -> Result<AgentPriority, AppError> {
    match value {
        "low" => Ok(AgentPriority::Low),
        "normal" => Ok(AgentPriority::Normal),
        "high" => Ok(AgentPriority::High),
        "urgent" => Ok(AgentPriority::Urgent),
        _ => Err(AppError::bad_request(
            "priority must be one of: low, normal, high, urgent",
        )),
    }
}
