use axum::{
    extract::{Path, State},
    Json,
};
use vel_api_types::{ApiResponse, CommitmentSchedulingProposalApplyResponseData};

use crate::{errors::AppError, routes::response, services::commitment_scheduling, state::AppState};

pub async fn apply_commitment_scheduling_proposal(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<CommitmentSchedulingProposalApplyResponseData>>, AppError> {
    let proposal = commitment_scheduling::apply_staged_commitment_scheduling_proposal(
        &state.storage,
        id.trim(),
    )
    .await?;
    Ok(response::success(
        CommitmentSchedulingProposalApplyResponseData {
            proposal: proposal.into(),
        },
    ))
}
