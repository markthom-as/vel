use serde_json::{json, Value};
use time::OffsetDateTime;
use vel_core::{
    AssistantProposalState, CommitmentSchedulingContinuity, CommitmentSchedulingMutation,
    CommitmentSchedulingMutationKind, CommitmentSchedulingProposal, CommitmentSchedulingSourceKind,
    ReflowProposal,
};
use vel_storage::{Storage, StorageError};

use crate::errors::AppError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitmentSchedulingProposalSummaryItem {
    pub thread_id: String,
    pub state: AssistantProposalState,
    pub title: String,
    pub summary: String,
    pub outcome_summary: Option<String>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommitmentSchedulingProposalSummary {
    pub pending_count: u32,
    pub latest_pending: Option<CommitmentSchedulingProposalSummaryItem>,
    pub latest_applied: Option<CommitmentSchedulingProposalSummaryItem>,
    pub latest_failed: Option<CommitmentSchedulingProposalSummaryItem>,
}

impl CommitmentSchedulingProposalSummary {
    pub fn is_empty(&self) -> bool {
        self.pending_count == 0
            && self.latest_pending.is_none()
            && self.latest_applied.is_none()
            && self.latest_failed.is_none()
    }
}

pub async fn load_commitment_scheduling_proposal_summary(
    storage: &Storage,
) -> Result<CommitmentSchedulingProposalSummary, AppError> {
    let rows = storage.list_threads(None, 100).await?;
    let mut summary = CommitmentSchedulingProposalSummary::default();

    for (thread_id, thread_type, title, _, _metadata_json, _, updated_at) in rows {
        if !matches!(thread_type.as_str(), "reflow_edit" | "day_plan_apply") {
            continue;
        }

        let Some((_, _, _, _, metadata_json, _, _)) = storage.get_thread_by_id(&thread_id).await?
        else {
            continue;
        };
        let metadata = parse_proposal_metadata(&metadata_json)?;
        let proposal = proposal_from_thread_metadata(&thread_id, &thread_type, &metadata)?;
        let item = CommitmentSchedulingProposalSummaryItem {
            thread_id: thread_id.clone(),
            state: proposal.state,
            title: title.clone(),
            summary: proposal.summary,
            outcome_summary: proposal.outcome_summary,
            updated_at,
        };

        match item.state {
            AssistantProposalState::Staged | AssistantProposalState::Approved => {
                summary.pending_count += 1;
                if summary.latest_pending.is_none() {
                    summary.latest_pending = Some(item);
                }
            }
            AssistantProposalState::Applied | AssistantProposalState::Reversed => {
                if summary.latest_applied.is_none() {
                    summary.latest_applied = Some(item);
                }
            }
            AssistantProposalState::Failed => {
                if summary.latest_failed.is_none() {
                    summary.latest_failed = Some(item);
                }
            }
        }
    }

    Ok(summary)
}

pub async fn apply_staged_commitment_scheduling_proposal(
    storage: &Storage,
    thread_id: &str,
) -> Result<CommitmentSchedulingProposal, AppError> {
    let Some((_, thread_type, _, _, metadata_json, _, _)) =
        storage.get_thread_by_id(thread_id).await?
    else {
        return Err(AppError::not_found(
            "commitment scheduling proposal thread not found",
        ));
    };

    if !matches!(thread_type.as_str(), "reflow_edit" | "day_plan_apply") {
        return Err(AppError::bad_request(
            "thread is not a commitment scheduling proposal",
        ));
    }

    let mut metadata = parse_proposal_metadata(&metadata_json)?;
    let mut proposal = proposal_from_thread_metadata(thread_id, &thread_type, &metadata)?;
    if proposal.state != AssistantProposalState::Staged
        && proposal.state != AssistantProposalState::Approved
    {
        return Err(AppError::bad_request(
            "commitment scheduling proposal is not pending application",
        ));
    }
    if proposal.mutations.is_empty() {
        return Err(AppError::bad_request(
            "commitment scheduling proposal has no applicable mutations",
        ));
    }

    let now = OffsetDateTime::now_utc().unix_timestamp();
    let transition_via = "commitment_scheduling_apply";
    update_proposal_metadata_transition(
        &mut metadata,
        AssistantProposalState::Approved,
        Some("Commitment scheduling proposal approved for canonical application.".to_string()),
        now,
        transition_via,
    );

    match apply_commitment_scheduling_mutations(storage, &proposal.mutations).await {
        Ok(()) => {
            let outcome = "Commitment scheduling proposal applied through canonical mutation seam."
                .to_string();
            update_proposal_metadata_transition(
                &mut metadata,
                AssistantProposalState::Applied,
                Some(outcome.clone()),
                now,
                transition_via,
            );
            storage
                .update_thread_metadata(thread_id, &metadata.to_string())
                .await?;
            storage.update_thread_status(thread_id, "resolved").await?;
            proposal.state = AssistantProposalState::Applied;
            proposal.outcome_summary = Some(outcome);
            proposal.thread_id = Some(thread_id.to_string());
            proposal.thread_type = Some(thread_type);
            Ok(proposal)
        }
        Err(error) => {
            update_proposal_metadata_transition(
                &mut metadata,
                AssistantProposalState::Failed,
                Some(error.to_string()),
                now,
                transition_via,
            );
            storage
                .update_thread_metadata(thread_id, &metadata.to_string())
                .await?;
            storage.update_thread_status(thread_id, "open").await?;
            Err(error)
        }
    }
}

pub(crate) async fn staged_commitment_scheduling_proposal_from_reflow(
    storage: &Storage,
    proposal: &ReflowProposal,
) -> Result<Option<CommitmentSchedulingProposal>, AppError> {
    let mut mutations = Vec::new();

    for change in &proposal.changes {
        let Some(commitment_id) = change.commitment_id.as_ref() else {
            continue;
        };
        let current = storage
            .get_commitment_by_id(commitment_id)
            .await?
            .ok_or_else(|| AppError::not_found("commitment not found"))?;
        let previous_due_at_ts = current.due_at.map(|value| value.unix_timestamp());

        match change.kind {
            vel_core::ReflowChangeKind::Moved => {
                let next_due_at_ts = change.scheduled_start_ts.ok_or_else(|| {
                    AppError::bad_request("moved reflow change must include scheduled_start_ts")
                })?;
                mutations.push(CommitmentSchedulingMutation {
                    commitment_id: commitment_id.clone(),
                    kind: CommitmentSchedulingMutationKind::SetDueAt,
                    title: change.title.clone(),
                    summary: change.detail.clone(),
                    project_label: change.project_label.clone(),
                    previous_due_at_ts,
                    next_due_at_ts: Some(next_due_at_ts),
                });
            }
            vel_core::ReflowChangeKind::Unscheduled => {
                mutations.push(CommitmentSchedulingMutation {
                    commitment_id: commitment_id.clone(),
                    kind: CommitmentSchedulingMutationKind::ClearDueAt,
                    title: change.title.clone(),
                    summary: change.detail.clone(),
                    project_label: change.project_label.clone(),
                    previous_due_at_ts,
                    next_due_at_ts: None,
                });
            }
            vel_core::ReflowChangeKind::NeedsJudgment => {}
        }
    }

    if mutations.is_empty() {
        return Ok(None);
    }

    Ok(Some(CommitmentSchedulingProposal {
        source_kind: CommitmentSchedulingSourceKind::Reflow,
        state: AssistantProposalState::Staged,
        summary: proposal.summary.clone(),
        requires_confirmation: true,
        continuity: CommitmentSchedulingContinuity::Thread,
        mutations,
        outcome_summary: None,
        thread_id: None,
        thread_type: None,
    }))
}

pub(crate) fn applyable_proposal_metadata(proposal: &CommitmentSchedulingProposal) -> Value {
    json!({
        "source_kind": proposal.source_kind,
        "proposal_state": proposal.state.to_string(),
        "summary": proposal.summary,
        "requires_confirmation": proposal.requires_confirmation,
        "continuity": proposal.continuity,
        "mutations": proposal.mutations,
        "outcome_summary": proposal.outcome_summary,
    })
}

async fn apply_commitment_scheduling_mutations(
    storage: &Storage,
    mutations: &[CommitmentSchedulingMutation],
) -> Result<(), AppError> {
    for mutation in mutations {
        validate_mutation_shape(mutation)?;
        storage
            .get_commitment_by_id(&mutation.commitment_id)
            .await?
            .ok_or_else(|| AppError::not_found("commitment not found"))?;
    }

    for mutation in mutations {
        apply_commitment_scheduling_mutation(storage, mutation).await?;
    }

    Ok(())
}

async fn apply_commitment_scheduling_mutation(
    storage: &Storage,
    mutation: &CommitmentSchedulingMutation,
) -> Result<(), AppError> {
    let due_at = match mutation.kind {
        CommitmentSchedulingMutationKind::SetDueAt => {
            let next_due_at_ts = mutation.next_due_at_ts.ok_or_else(|| {
                AppError::bad_request("set_due_at mutation requires next_due_at_ts")
            })?;
            Some(Some(
                OffsetDateTime::from_unix_timestamp(next_due_at_ts)
                    .map_err(|_| AppError::bad_request("invalid next_due_at_ts"))?,
            ))
        }
        CommitmentSchedulingMutationKind::ClearDueAt => Some(None),
    };

    storage
        .update_commitment(
            &mutation.commitment_id,
            None,
            None,
            due_at,
            None,
            None,
            None,
        )
        .await
        .map_err(map_storage_error)
}

fn validate_mutation_shape(mutation: &CommitmentSchedulingMutation) -> Result<(), AppError> {
    if mutation.commitment_id.trim().is_empty() {
        return Err(AppError::bad_request(
            "commitment scheduling mutation must target a commitment id",
        ));
    }
    match mutation.kind {
        CommitmentSchedulingMutationKind::SetDueAt => {
            if mutation.next_due_at_ts.is_none() {
                return Err(AppError::bad_request(
                    "set_due_at mutation requires next_due_at_ts",
                ));
            }
        }
        CommitmentSchedulingMutationKind::ClearDueAt => {
            if mutation.next_due_at_ts.is_some() {
                return Err(AppError::bad_request(
                    "clear_due_at mutation must not include next_due_at_ts",
                ));
            }
        }
    }
    Ok(())
}

fn parse_proposal_metadata(metadata_json: &str) -> Result<Value, AppError> {
    let value = serde_json::from_str::<Value>(metadata_json)
        .map_err(|_| AppError::bad_request("thread metadata must be valid json"))?;
    if !value.is_object() {
        return Err(AppError::bad_request(
            "thread metadata must be a json object",
        ));
    }
    Ok(value)
}

fn proposal_from_thread_metadata(
    thread_id: &str,
    thread_type: &str,
    metadata: &Value,
) -> Result<CommitmentSchedulingProposal, AppError> {
    let object = metadata
        .as_object()
        .ok_or_else(|| AppError::bad_request("thread metadata must be an object"))?;
    let source_kind = match object
        .get("source_kind")
        .and_then(Value::as_str)
        .unwrap_or("reflow")
    {
        "day_plan" => CommitmentSchedulingSourceKind::DayPlan,
        "reflow" => CommitmentSchedulingSourceKind::Reflow,
        other => {
            return Err(AppError::bad_request(format!(
                "unsupported commitment scheduling proposal source_kind: {other}"
            )))
        }
    };
    let continuity = match object
        .get("continuity")
        .and_then(Value::as_str)
        .unwrap_or("inline")
    {
        "inline" => CommitmentSchedulingContinuity::Inline,
        "thread" => CommitmentSchedulingContinuity::Thread,
        other => {
            return Err(AppError::bad_request(format!(
                "unsupported commitment scheduling continuity: {other}"
            )))
        }
    };

    let mutations_value = object
        .get("mutations")
        .cloned()
        .unwrap_or_else(|| Value::Array(Vec::new()));
    let mutations = serde_json::from_value::<Vec<CommitmentSchedulingMutation>>(mutations_value)
        .map_err(|_| AppError::bad_request("invalid commitment scheduling mutations payload"))?;

    Ok(CommitmentSchedulingProposal {
        source_kind,
        state: object
            .get("proposal_state")
            .and_then(Value::as_str)
            .map(parse_assistant_proposal_state)
            .transpose()?
            .unwrap_or(AssistantProposalState::Staged),
        summary: object
            .get("summary")
            .and_then(Value::as_str)
            .unwrap_or("Commitment scheduling proposal")
            .to_string(),
        requires_confirmation: object
            .get("requires_confirmation")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        continuity,
        mutations,
        outcome_summary: object
            .get("outcome_summary")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        thread_id: Some(thread_id.to_string()),
        thread_type: Some(thread_type.to_string()),
    })
}

fn parse_assistant_proposal_state(value: &str) -> Result<AssistantProposalState, AppError> {
    match value {
        "staged" => Ok(AssistantProposalState::Staged),
        "approved" => Ok(AssistantProposalState::Approved),
        "applied" => Ok(AssistantProposalState::Applied),
        "failed" => Ok(AssistantProposalState::Failed),
        "reversed" => Ok(AssistantProposalState::Reversed),
        other => Err(AppError::bad_request(format!(
            "unsupported proposal state: {other}"
        ))),
    }
}

fn update_proposal_metadata_transition(
    metadata: &mut Value,
    state: AssistantProposalState,
    outcome_summary: Option<String>,
    now_ts: i64,
    transition_via: &str,
) {
    let object = metadata
        .as_object_mut()
        .expect("proposal metadata should be object");
    object.insert("proposal_state".to_string(), json!(state.to_string()));
    object.insert(
        "outcome_summary".to_string(),
        outcome_summary
            .as_ref()
            .map(|value| Value::String(value.clone()))
            .unwrap_or(Value::Null),
    );
    object.insert("proposal_updated_at".to_string(), json!(now_ts));

    match state {
        AssistantProposalState::Approved => {
            object.insert("approved_via".to_string(), json!(transition_via));
        }
        AssistantProposalState::Applied => {
            object.insert("applied_via".to_string(), json!(transition_via));
        }
        AssistantProposalState::Failed => {
            object.insert("failed_via".to_string(), json!(transition_via));
        }
        AssistantProposalState::Reversed => {
            object.insert("reversed_via".to_string(), json!(transition_via));
        }
        AssistantProposalState::Staged => {}
    }
}

fn map_storage_error(error: StorageError) -> AppError {
    match error {
        StorageError::Validation(message) => AppError::bad_request(message),
        StorageError::NotFound(message) => AppError::not_found(message),
        other => AppError::internal(other.to_string()),
    }
}
