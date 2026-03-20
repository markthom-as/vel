use time::OffsetDateTime;
use vel_api_types::{
    AgentBlockerData, AgentCapabilityEntryData, AgentCapabilityGroupData,
    AgentCapabilityGroupKindData, AgentCapabilitySummaryData, AgentContextRefData,
    AgentGroundingPackData, AgentInspectData, AgentInspectExplainabilityData,
    AgentReviewObligationsData, CommitmentData, ExecutionHandoffOriginKindData,
    ExecutionHandoffRecordData, ExecutionHandoffReviewStateData, ExecutionReviewGateData,
    ExecutionRoutingDecisionData, ExecutionRoutingReasonData, NowData, PersonRecordData,
    ProjectRecordData,
};
use vel_core::CommitmentStatus;

use crate::{
    errors::AppError,
    services::{execution_routing, operator_settings, people, projects},
    state::AppState,
};

const CURRENT_CONTEXT_PATH: &str = "/v1/context/current";
const EXPLAIN_CONTEXT_PATH: &str = "/v1/explain/context";
const EXPLAIN_DRIFT_PATH: &str = "/v1/explain/drift";
const NOW_PATH: &str = "/v1/now";
const EXECUTION_HANDOFFS_PATH: &str = "/v1/execution/handoffs";

pub async fn build_agent_inspect(state: &AppState) -> Result<AgentInspectData, AppError> {
    let generated_at = OffsetDateTime::now_utc().unix_timestamp();
    let now: NowData = crate::services::now::get_now(&state.storage, &state.config)
        .await?
        .into();
    let projects = projects::list_projects(state).await?;
    let people = people::list_people(state).await?;
    let commitments = state
        .storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?;
    let pending_handoffs = execution_routing::list_execution_handoffs(
        state,
        None,
        Some(execution_routing::HandoffReviewState::PendingReview),
    )
    .await?;
    let approved_handoffs = execution_routing::list_execution_handoffs(
        state,
        None,
        Some(execution_routing::HandoffReviewState::Approved),
    )
    .await?;
    let writeback_enabled =
        operator_settings::runtime_writeback_enabled(&state.storage, &state.config).await?;
    let current_context =
        state
            .storage
            .get_current_context()
            .await?
            .map(|(computed_at, context)| AgentContextRefData {
                computed_at,
                mode: (!context.mode.trim().is_empty()).then_some(context.mode),
                morning_state: (!context.morning_state.trim().is_empty())
                    .then_some(context.morning_state),
                current_context_path: CURRENT_CONTEXT_PATH.to_string(),
                explain_context_path: EXPLAIN_CONTEXT_PATH.to_string(),
                explain_drift_path: EXPLAIN_DRIFT_PATH.to_string(),
            });

    let review = AgentReviewObligationsData {
        review_snapshot: now.review_snapshot.clone(),
        pending_writebacks: now.pending_writebacks.clone(),
        conflicts: now.conflicts.clone(),
        pending_execution_handoffs: pending_handoffs
            .iter()
            .cloned()
            .map(map_handoff_record)
            .collect(),
    };

    let grounding = AgentGroundingPackData {
        generated_at,
        now,
        current_context,
        projects: projects.into_iter().map(ProjectRecordData::from).collect(),
        people: people.into_iter().map(PersonRecordData::from).collect(),
        commitments: commitments.into_iter().map(CommitmentData::from).collect(),
        review,
    };

    let blockers = collect_blockers(
        writeback_enabled,
        pending_handoffs.as_slice(),
        approved_handoffs.as_slice(),
    );

    Ok(AgentInspectData {
        grounding,
        capabilities: AgentCapabilitySummaryData {
            groups: vec![
                AgentCapabilityGroupData {
                    kind: AgentCapabilityGroupKindData::ReadContext,
                    label: "Read current Vel state".to_string(),
                    entries: vec![
                        AgentCapabilityEntryData {
                            key: "read_now".to_string(),
                            label: "Read Now and current context".to_string(),
                            summary: "The agent can inspect current Now state, typed context labels, and explain references.".to_string(),
                            available: true,
                            blocked_reason: None,
                            requires_review_gate: None,
                            requires_writeback_enabled: false,
                        },
                        AgentCapabilityEntryData {
                            key: "read_projects_people_commitments".to_string(),
                            label: "Read projects, people, and commitments".to_string(),
                            summary: "The agent can inspect persisted project, people, and open commitment records.".to_string(),
                            available: true,
                            blocked_reason: None,
                            requires_review_gate: None,
                            requires_writeback_enabled: false,
                        },
                    ],
                },
                AgentCapabilityGroupData {
                    kind: AgentCapabilityGroupKindData::ReviewActions,
                    label: "Review queues and obligations".to_string(),
                    entries: vec![
                        AgentCapabilityEntryData {
                            key: "review_execution_handoffs".to_string(),
                            label: "Inspect pending execution handoffs".to_string(),
                            summary: "The agent can inspect handoff review metadata and routing reasons.".to_string(),
                            available: !pending_handoffs.is_empty(),
                            blocked_reason: pending_handoffs
                                .is_empty()
                                .then_some(no_pending_handoffs_blocker()),
                            requires_review_gate: Some(ExecutionReviewGateData::OperatorPreview),
                            requires_writeback_enabled: false,
                        },
                        AgentCapabilityEntryData {
                            key: "review_writeback_pressure".to_string(),
                            label: "Inspect writeback and conflict pressure".to_string(),
                            summary: "The agent can inspect pending writebacks, conflicts, and review counts.".to_string(),
                            available: true,
                            blocked_reason: None,
                            requires_review_gate: Some(ExecutionReviewGateData::OperatorPreview),
                            requires_writeback_enabled: false,
                        },
                    ],
                },
                AgentCapabilityGroupData {
                    kind: AgentCapabilityGroupKindData::MutationActions,
                    label: "Bounded mutation affordances".to_string(),
                    entries: vec![
                        assistant_staged_actions_capability(writeback_enabled),
                        integration_writeback_capability(writeback_enabled),
                        repo_handoff_capability(
                            pending_handoffs.as_slice(),
                            approved_handoffs.as_slice(),
                        ),
                    ],
                },
            ],
        },
        blockers,
        explainability: AgentInspectExplainabilityData {
            persisted_record_kinds: vec![
                "now".to_string(),
                "project_record".to_string(),
                "person_record".to_string(),
                "commitment".to_string(),
                "writeback_operation".to_string(),
                "conflict_case".to_string(),
                "execution_handoff".to_string(),
            ],
            supporting_paths: vec![
                NOW_PATH.to_string(),
                CURRENT_CONTEXT_PATH.to_string(),
                EXPLAIN_CONTEXT_PATH.to_string(),
                EXECUTION_HANDOFFS_PATH.to_string(),
            ],
            raw_context_json_supporting_only: true,
        },
    })
}

pub fn render_agent_grounding_markdown(inspect: &AgentInspectData) -> String {
    let grounding = &inspect.grounding;
    let mut lines = vec![
        "# Agent Grounding".to_string(),
        String::new(),
        format!("- generated_at: {}", grounding.generated_at),
        format!("- timezone: {}", grounding.now.timezone),
        format!(
            "- review_snapshot: actions={} triage={} projects={}",
            grounding.review.review_snapshot.open_action_count,
            grounding.review.review_snapshot.triage_count,
            grounding.review.review_snapshot.projects_needing_review
        ),
        format!("- projects: {}", grounding.projects.len()),
        format!("- people: {}", grounding.people.len()),
        format!("- commitments: {}", grounding.commitments.len()),
        format!(
            "- pending_execution_handoffs: {}",
            grounding.review.pending_execution_handoffs.len()
        ),
    ];

    if let Some(current_context) = &grounding.current_context {
        lines.push(format!(
            "- current_context: mode={} morning_state={}",
            current_context.mode.as_deref().unwrap_or(""),
            current_context.morning_state.as_deref().unwrap_or("")
        ));
    }

    lines.push(String::new());
    lines.push("## Capabilities".to_string());
    for group in &inspect.capabilities.groups {
        lines.push(String::new());
        lines.push(format!("### {}", group.label));
        for entry in &group.entries {
            let mut line = format!(
                "- {}: {} [{}]",
                entry.key,
                entry.summary,
                if entry.available {
                    "available"
                } else {
                    "blocked"
                }
            );
            if let Some(blocker) = &entry.blocked_reason {
                line.push_str(&format!(" blocker={} ({})", blocker.code, blocker.message));
            }
            lines.push(line);
        }
    }

    if !inspect.blockers.is_empty() {
        lines.push(String::new());
        lines.push("## Blockers".to_string());
        lines.extend(
            inspect
                .blockers
                .iter()
                .map(|blocker| format!("- {}: {}", blocker.code, blocker.message)),
        );
    }

    lines.push(String::new());
    lines.push("## Explainability".to_string());
    lines.extend(
        inspect
            .explainability
            .supporting_paths
            .iter()
            .map(|path| format!("- {}", path)),
    );
    lines.push(String::new());
    lines.join("\n")
}

pub fn assistant_grounding_hint(inspect: &AgentInspectData) -> String {
    format!(
        "Current grounding shows {} open actions, {} triage items, {} projects, {} people, and {} pending execution handoffs.",
        inspect.grounding.review.review_snapshot.open_action_count,
        inspect.grounding.review.review_snapshot.triage_count,
        inspect.grounding.projects.len(),
        inspect.grounding.people.len(),
        inspect.grounding.review.pending_execution_handoffs.len()
    )
}

fn integration_writeback_capability(writeback_enabled: bool) -> AgentCapabilityEntryData {
    AgentCapabilityEntryData {
        key: "integration_writeback".to_string(),
        label: "Request integration writeback".to_string(),
        summary: "Bounded upstream mutations remain subject to SAFE MODE and review gates."
            .to_string(),
        available: writeback_enabled,
        blocked_reason: (!writeback_enabled).then_some(writeback_disabled_blocker()),
        requires_review_gate: Some(ExecutionReviewGateData::OperatorPreview),
        requires_writeback_enabled: true,
    }
}

fn assistant_staged_actions_capability(writeback_enabled: bool) -> AgentCapabilityEntryData {
    AgentCapabilityEntryData {
        key: "assistant_staged_actions".to_string(),
        label: "Stage assistant-mediated actions".to_string(),
        summary:
            "Assistant proposals can be staged, but mutation-capable follow-through still depends on SAFE MODE and review gates."
                .to_string(),
        available: writeback_enabled,
        blocked_reason: (!writeback_enabled).then_some(writeback_disabled_blocker()),
        requires_review_gate: Some(ExecutionReviewGateData::OperatorPreview),
        requires_writeback_enabled: true,
    }
}

fn repo_handoff_capability(
    pending_handoffs: &[execution_routing::ExecutionHandoffRecordData],
    approved_handoffs: &[execution_routing::ExecutionHandoffRecordData],
) -> AgentCapabilityEntryData {
    let matching_grant = approved_handoffs
        .iter()
        .find(|handoff| !handoff.routing.write_scopes.is_empty());

    let blocked_reason = if matching_grant.is_some() {
        None
    } else if pending_handoffs
        .iter()
        .any(|handoff| !handoff.routing.write_scopes.is_empty())
    {
        Some(AgentBlockerData {
            code: "handoff_review_pending".to_string(),
            message: "A repo-local write grant exists but still needs operator review.".to_string(),
            escalation_hint: Some(
                "Approve the pending execution handoff before launching repo-local mutation work."
                    .to_string(),
            ),
        })
    } else {
        Some(AgentBlockerData {
            code: "no_matching_write_grant".to_string(),
            message:
                "No approved repo-local handoff currently grants write scope for mutation work."
                    .to_string(),
            escalation_hint: Some(
                "Create and approve a scoped execution handoff before widening into repo-local edits."
                    .to_string(),
            ),
        })
    };

    AgentCapabilityEntryData {
        key: "repo_local_write_scope".to_string(),
        label: "Use approved repo-local write scope".to_string(),
        summary:
            "Repo-local mutation remains bounded to explicitly approved execution handoff scopes."
                .to_string(),
        available: matching_grant.is_some(),
        blocked_reason,
        requires_review_gate: Some(ExecutionReviewGateData::OperatorApproval),
        requires_writeback_enabled: false,
    }
}

fn collect_blockers(
    writeback_enabled: bool,
    pending_handoffs: &[execution_routing::ExecutionHandoffRecordData],
    approved_handoffs: &[execution_routing::ExecutionHandoffRecordData],
) -> Vec<AgentBlockerData> {
    let mut blockers = Vec::new();
    if !writeback_enabled {
        blockers.push(AgentBlockerData {
            code: "writeback_disabled".to_string(),
            message:
                "Writeback-dependent mutation requests are unavailable while SAFE MODE is enabled."
                    .to_string(),
            escalation_hint: Some("Enable writeback or stay within read/review lanes.".to_string()),
        });
    }
    if pending_handoffs
        .iter()
        .any(|handoff| !handoff.routing.write_scopes.is_empty())
    {
        blockers.push(AgentBlockerData {
            code: "handoff_review_pending".to_string(),
            message: "At least one repo-local mutation grant is still waiting on operator review."
                .to_string(),
            escalation_hint: Some(
                "Review pending execution handoffs before launching repo-local mutation work."
                    .to_string(),
            ),
        });
    } else if !approved_handoffs
        .iter()
        .any(|handoff| !handoff.routing.write_scopes.is_empty())
    {
        blockers.push(AgentBlockerData {
            code: "no_matching_write_grant".to_string(),
            message: "No approved repo-local write grant is currently available.".to_string(),
            escalation_hint: Some(
                "Create and approve a scoped execution handoff to unlock repo-local mutation work."
                    .to_string(),
            ),
        });
    }
    blockers
}

fn map_handoff_record(
    record: execution_routing::ExecutionHandoffRecordData,
) -> ExecutionHandoffRecordData {
    ExecutionHandoffRecordData {
        id: record.id,
        project_id: record.project_id,
        origin_kind: match record.origin_kind {
            execution_routing::HandoffOriginKind::HumanToAgent => {
                ExecutionHandoffOriginKindData::HumanToAgent
            }
            execution_routing::HandoffOriginKind::AgentToAgent => {
                ExecutionHandoffOriginKindData::AgentToAgent
            }
        },
        review_state: match record.review_state {
            execution_routing::HandoffReviewState::PendingReview => {
                ExecutionHandoffReviewStateData::PendingReview
            }
            execution_routing::HandoffReviewState::Approved => {
                ExecutionHandoffReviewStateData::Approved
            }
            execution_routing::HandoffReviewState::Rejected => {
                ExecutionHandoffReviewStateData::Rejected
            }
        },
        handoff: record.handoff.into(),
        routing: ExecutionRoutingDecisionData {
            task_kind: record.routing.task_kind.into(),
            agent_profile: record.routing.agent_profile.into(),
            token_budget: record.routing.token_budget.into(),
            review_gate: record.routing.review_gate.into(),
            read_scopes: record.routing.read_scopes,
            write_scopes: record.routing.write_scopes,
            allowed_tools: record.routing.allowed_tools,
            reasons: record
                .routing
                .reasons
                .into_iter()
                .map(|reason| ExecutionRoutingReasonData {
                    code: reason.code,
                    message: reason.message,
                })
                .collect(),
        },
        manifest_id: record.manifest_id,
        requested_by: record.requested_by,
        reviewed_by: record.reviewed_by,
        decision_reason: record.decision_reason,
        reviewed_at: record.reviewed_at,
        launched_at: record.launched_at,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}

fn writeback_disabled_blocker() -> AgentBlockerData {
    AgentBlockerData {
        code: "safe_mode_enabled".to_string(),
        message: "SAFE MODE keeps writeback disabled.".to_string(),
        escalation_hint: Some("Enable writeback in Settings before retrying.".to_string()),
    }
}

fn no_pending_handoffs_blocker() -> AgentBlockerData {
    AgentBlockerData {
        code: "no_pending_execution_handoffs".to_string(),
        message: "No pending execution handoffs are waiting for review.".to_string(),
        escalation_hint: Some(
            "Create a new scoped handoff if you need to route repo-local work for review."
                .to_string(),
        ),
    }
}
