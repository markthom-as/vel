use axum::{extract::State, Json};
use vel_api_types::{AgentInspectData, ApiResponse};

use crate::{errors::AppError, routes::response, services::agent_grounding, state::AppState};

pub async fn get_agent_inspect(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AgentInspectData>>, AppError> {
    let inspect = agent_grounding::build_agent_inspect(&state).await?;
    Ok(response::success(inspect))
}
