use axum::{
    extract::{Path, State},
    Json,
};
use vel_api_types::{
    ApiResponse, PlanningProfileMutationRequestData, PlanningProfileProposalApplyResponseData,
    PlanningProfileProposalSummaryData, PlanningProfileProposalSummaryItemData,
    PlanningProfileResponseData,
};

use crate::{errors::AppError, routes::response, services::planning_profile, state::AppState};

pub async fn get_planning_profile(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<PlanningProfileResponseData>>, AppError> {
    let profile = planning_profile::load_routine_planning_profile(&state.storage).await?;
    let proposal_summary =
        planning_profile::load_planning_profile_proposal_summary(&state.storage).await?;
    Ok(response::success(PlanningProfileResponseData {
        profile: profile.into(),
        proposal_summary: (!proposal_summary.is_empty())
            .then_some(planning_profile_summary_data(proposal_summary)),
    }))
}

pub async fn patch_planning_profile(
    State(state): State<AppState>,
    Json(payload): Json<PlanningProfileMutationRequestData>,
) -> Result<Json<ApiResponse<PlanningProfileResponseData>>, AppError> {
    let profile =
        planning_profile::apply_planning_profile_mutation(&state.storage, &payload.mutation.into())
            .await?;
    let proposal_summary =
        planning_profile::load_planning_profile_proposal_summary(&state.storage).await?;
    Ok(response::success(PlanningProfileResponseData {
        profile: profile.into(),
        proposal_summary: (!proposal_summary.is_empty())
            .then_some(planning_profile_summary_data(proposal_summary)),
    }))
}

pub async fn apply_planning_profile_proposal(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<PlanningProfileProposalApplyResponseData>>, AppError> {
    let (profile, proposal) =
        planning_profile::apply_staged_planning_profile_proposal(&state.storage, id.trim()).await?;
    Ok(response::success(
        PlanningProfileProposalApplyResponseData {
            profile: profile.into(),
            proposal: proposal.into(),
        },
    ))
}

fn planning_profile_summary_data(
    value: planning_profile::PlanningProfileProposalSummary,
) -> PlanningProfileProposalSummaryData {
    PlanningProfileProposalSummaryData {
        pending_count: value.pending_count,
        latest_pending: value.latest_pending.map(planning_profile_summary_item_data),
        latest_applied: value.latest_applied.map(planning_profile_summary_item_data),
        latest_failed: value.latest_failed.map(planning_profile_summary_item_data),
    }
}

fn planning_profile_summary_item_data(
    value: planning_profile::PlanningProfileProposalSummaryItem,
) -> PlanningProfileProposalSummaryItemData {
    PlanningProfileProposalSummaryItemData {
        thread_id: value.thread_id,
        state: value.state.into(),
        title: value.title,
        summary: value.summary,
        outcome_summary: value.outcome_summary,
        updated_at: value.updated_at,
    }
}
