use std::collections::HashMap;

use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{
    ActionEvidenceRef, ActionItem, ActionItemId, ActionKind, ActionState, ActionSurface,
    Commitment, CommitmentStatus, ConflictCaseRecord, LinkStatus, LinkedNodeRecord, ProjectId,
    ProjectRecord, ProjectStatus, ReviewSnapshot, WritebackOperationRecord, WritebackStatus,
};
use vel_storage::{InterventionRecord, Storage};

use crate::{errors::AppError, services::integrations};

const FRESHNESS_ALERT_RANK: i64 = 90;
const LINKING_ALERT_RANK: i64 = 85;
const INTERVENTION_RANK: i64 = 80;
const EXECUTION_HANDOFF_RANK: i64 = 78;
const PROJECT_BLOCKED_RANK: i64 = 75;
const WRITEBACK_PENDING_RANK: i64 = 72;
const NEXT_COMMITMENT_RANK: i64 = 70;
const PROJECT_REVIEW_RANK: i64 = 60;
const SNOOZED_INTERVENTION_RANK: i64 = 40;

fn task_kind_label(value: vel_core::ExecutionTaskKind) -> &'static str {
    match value {
        vel_core::ExecutionTaskKind::Planning => "planning",
        vel_core::ExecutionTaskKind::Implementation => "implementation",
        vel_core::ExecutionTaskKind::Debugging => "debugging",
        vel_core::ExecutionTaskKind::Review => "review",
        vel_core::ExecutionTaskKind::Research => "research",
        vel_core::ExecutionTaskKind::Documentation => "documentation",
    }
}

fn agent_profile_label(value: vel_core::AgentProfile) -> &'static str {
    match value {
        vel_core::AgentProfile::Budget => "budget",
        vel_core::AgentProfile::Balanced => "balanced",
        vel_core::AgentProfile::Quality => "quality",
        vel_core::AgentProfile::Inherit => "inherit",
    }
}

fn token_budget_label(value: vel_core::TokenBudgetClass) -> &'static str {
    match value {
        vel_core::TokenBudgetClass::Small => "small",
        vel_core::TokenBudgetClass::Medium => "medium",
        vel_core::TokenBudgetClass::Large => "large",
        vel_core::TokenBudgetClass::Xlarge => "xlarge",
    }
}

fn review_gate_label(value: vel_core::ExecutionReviewGate) -> &'static str {
    match value {
        vel_core::ExecutionReviewGate::None => "none",
        vel_core::ExecutionReviewGate::OperatorApproval => "operator_approval",
        vel_core::ExecutionReviewGate::OperatorPreview => "operator_preview",
        vel_core::ExecutionReviewGate::PostRunReview => "post_run_review",
    }
}

#[derive(Debug, Clone)]
pub struct ActionQueueSnapshot {
    pub action_items: Vec<ActionItem>,
    pub review_snapshot: ReviewSnapshot,
    pub pending_writebacks: Vec<WritebackOperationRecord>,
    pub conflicts: Vec<ConflictCaseRecord>,
}

pub async fn build_action_items(
    storage: &Storage,
    config: &AppConfig,
) -> Result<ActionQueueSnapshot, AppError> {
    let now = OffsetDateTime::now_utc();
    let projects = storage.list_projects().await?;
    let project_lookup = build_project_lookup(&projects);
    let linked_nodes = storage.list_linked_nodes().await?;
    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?;
    let interventions = storage.list_interventions_active(64).await?;
    let pending_handoffs = storage
        .list_execution_handoffs(None, Some("pending_review"))
        .await?
        .into_iter()
        .map(crate::services::execution_routing::hydrate_record)
        .collect::<Result<Vec<_>, _>>()?;
    let pending_writebacks =
        crate::services::writeback::list_pending_writebacks(storage, 32).await?;
    let conflicts = crate::services::writeback::list_open_conflicts(storage, 32).await?;
    let integrations = integrations::get_integrations_with_config(storage, config).await?;
    let current_context = storage.get_current_context().await?;

    let mut items = Vec::new();
    items.extend(build_freshness_items(
        now,
        current_context.map(|(computed_at, _)| computed_at),
        &integrations,
    ));
    items.extend(build_linking_items(linked_nodes));
    items.extend(build_execution_handoff_items(&pending_handoffs));
    items.extend(build_writeback_items(&pending_writebacks));
    items.extend(build_conflict_items(&conflicts));
    items.extend(build_intervention_items(interventions));
    items.extend(build_project_items(now, &projects));
    items.extend(build_commitment_items(now, commitments, &project_lookup));

    items.sort_by(|left, right| {
        right
            .rank
            .cmp(&left.rank)
            .then_with(|| right.surfaced_at.cmp(&left.surfaced_at))
            .then_with(|| left.id.as_ref().cmp(right.id.as_ref()))
    });

    let review_snapshot = ReviewSnapshot {
        open_action_count: items
            .iter()
            .filter(|item| !matches!(item.state, ActionState::Snoozed))
            .count() as u32,
        triage_count: items
            .iter()
            .filter(|item| {
                item.surface == ActionSurface::Inbox && matches!(item.state, ActionState::Active)
            })
            .count() as u32,
        projects_needing_review: items
            .iter()
            .filter(|item| item.kind == ActionKind::Review && item.project_id.is_some())
            .count() as u32,
    };

    Ok(ActionQueueSnapshot {
        action_items: items,
        review_snapshot,
        pending_writebacks,
        conflicts,
    })
}

fn build_execution_handoff_items(
    records: &[crate::services::execution_routing::ExecutionHandoffRecordData],
) -> Vec<ActionItem> {
    records
        .iter()
        .map(|record| ActionItem {
            id: ActionItemId::from(format!("act_handoff_{}", record.id)),
            surface: ActionSurface::Now,
            kind: ActionKind::Review,
            title: format!(
                "Review execution handoff: {}",
                record.handoff.handoff.objective
            ),
            summary: format!(
                "{} -> {} | review gate {} | {}",
                record.handoff.handoff.from_agent,
                record.handoff.handoff.to_agent,
                review_gate_label(record.routing.review_gate),
                record
                    .routing
                    .reasons
                    .first()
                    .map(|reason| reason.message.as_str())
                    .unwrap_or("routing reasons available")
            ),
            project_id: Some(record.project_id.clone()),
            state: ActionState::Active,
            rank: EXECUTION_HANDOFF_RANK,
            surfaced_at: record.updated_at,
            snoozed_until: None,
            evidence: execution_handoff_evidence(record),
        })
        .collect()
}

fn execution_handoff_evidence(
    record: &crate::services::execution_routing::ExecutionHandoffRecordData,
) -> Vec<ActionEvidenceRef> {
    let mut evidence = vec![ActionEvidenceRef {
        source_kind: "execution_handoff".to_string(),
        source_id: record.id.clone(),
        label: format!(
            "{} · {} · {}",
            task_kind_label(record.routing.task_kind),
            agent_profile_label(record.routing.agent_profile),
            token_budget_label(record.routing.token_budget)
        ),
        detail: Some(format!(
            "{} | write scopes: {}",
            record
                .routing
                .reasons
                .iter()
                .map(|reason| reason.code.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            record.routing.write_scopes.join(", ")
        )),
    }];
    evidence.push(ActionEvidenceRef {
        source_kind: "trace".to_string(),
        source_id: record.handoff.handoff.trace_id.to_string(),
        label: "Trace".to_string(),
        detail: Some(record.handoff.handoff.trace_id.to_string()),
    });
    evidence
}

fn build_writeback_items(records: &[WritebackOperationRecord]) -> Vec<ActionItem> {
    records
        .iter()
        .filter(|record| !matches!(record.status, WritebackStatus::Conflicted))
        .map(|record| ActionItem {
            id: ActionItemId::from(format!("act_writeback_{}", record.id.as_ref())),
            surface: ActionSurface::Now,
            kind: ActionKind::NextStep,
            title: format!("Queued write: {}", record.kind),
            summary: format!(
                "Status is {} for {}:{}.",
                record.status, record.target.family, record.target.provider_key
            ),
            project_id: record.target.project_id.clone(),
            state: ActionState::Active,
            rank: WRITEBACK_PENDING_RANK,
            surfaced_at: record.updated_at,
            snoozed_until: None,
            evidence: writeback_evidence(record),
        })
        .collect()
}

fn build_conflict_items(records: &[ConflictCaseRecord]) -> Vec<ActionItem> {
    records
        .iter()
        .map(|record| ActionItem {
            id: ActionItemId::from(format!("act_conflict_{}", record.id.as_ref())),
            surface: ActionSurface::Now,
            kind: ActionKind::Conflict,
            title: format!("Conflict needs review: {}", record.summary),
            summary: format!(
                "{}:{} has an open {} case.",
                record.target.family, record.target.provider_key, record.kind
            ),
            project_id: record.target.project_id.clone(),
            state: ActionState::Active,
            rank: LINKING_ALERT_RANK + 1,
            surfaced_at: record.updated_at,
            snoozed_until: None,
            evidence: conflict_evidence(record),
        })
        .collect()
}

fn writeback_evidence(record: &WritebackOperationRecord) -> Vec<ActionEvidenceRef> {
    let mut evidence = vec![ActionEvidenceRef {
        source_kind: "writeback_operation".to_string(),
        source_id: record.id.to_string(),
        label: record.kind.to_string(),
        detail: Some(format!("risk={}, status={}", record.risk, record.status)),
    }];
    push_target_evidence(
        &mut evidence,
        record.target.connection_id.as_ref().map(|id| id.as_ref()),
        record.target.project_id.as_ref().map(|id| id.as_ref()),
        record.target.provider_key.as_str(),
    );
    push_people_evidence(&mut evidence, &record.requested_payload);
    if let Some(payload) = &record.result_payload {
        push_people_evidence(&mut evidence, payload);
    }
    evidence
}

fn conflict_evidence(record: &ConflictCaseRecord) -> Vec<ActionEvidenceRef> {
    let mut evidence = vec![ActionEvidenceRef {
        source_kind: "conflict_case".to_string(),
        source_id: record.id.to_string(),
        label: record.kind.to_string(),
        detail: Some(record.summary.clone()),
    }];
    push_target_evidence(
        &mut evidence,
        record.target.connection_id.as_ref().map(|id| id.as_ref()),
        record.target.project_id.as_ref().map(|id| id.as_ref()),
        record.target.provider_key.as_str(),
    );
    push_people_evidence(&mut evidence, &record.local_payload);
    if let Some(payload) = &record.upstream_payload {
        push_people_evidence(&mut evidence, payload);
    }
    evidence
}

fn push_target_evidence(
    evidence: &mut Vec<ActionEvidenceRef>,
    connection_id: Option<&str>,
    project_id: Option<&str>,
    provider_key: &str,
) {
    if let Some(connection_id) = connection_id {
        evidence.push(ActionEvidenceRef {
            source_kind: "integration_connection".to_string(),
            source_id: connection_id.to_string(),
            label: format!("{provider_key} connection"),
            detail: None,
        });
    }
    if let Some(project_id) = project_id {
        evidence.push(ActionEvidenceRef {
            source_kind: "project".to_string(),
            source_id: project_id.to_string(),
            label: format!("project {project_id}"),
            detail: None,
        });
    }
}

fn push_people_evidence(evidence: &mut Vec<ActionEvidenceRef>, payload: &serde_json::Value) {
    let Some(people) = payload.get("people").and_then(serde_json::Value::as_array) else {
        return;
    };

    for person in people {
        let Some(person_id) = person.get("person_id").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let label = person
            .get("display_name")
            .and_then(serde_json::Value::as_str)
            .or_else(|| person.get("handle").and_then(serde_json::Value::as_str))
            .unwrap_or(person_id);
        evidence.push(ActionEvidenceRef {
            source_kind: "person".to_string(),
            source_id: person_id.to_string(),
            label: label.to_string(),
            detail: person
                .get("platform")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string),
        });
    }
}

fn build_freshness_items(
    now: OffsetDateTime,
    context_computed_at: Option<i64>,
    integrations: &integrations::IntegrationsOutput,
) -> Vec<ActionItem> {
    let mut items = Vec::new();

    if context_computed_at.is_none() {
        items.push(ActionItem {
            id: ActionItemId::from("act_freshness_context".to_string()),
            surface: ActionSurface::Inbox,
            kind: ActionKind::Freshness,
            title: "Context is missing".to_string(),
            summary: "Run evaluate or sync the affected sources before relying on Now.".to_string(),
            project_id: None,
            state: ActionState::Active,
            rank: FRESHNESS_ALERT_RANK,
            surfaced_at: now,
            snoozed_until: None,
            evidence: vec![ActionEvidenceRef {
                source_kind: "integration_status".to_string(),
                source_id: "context".to_string(),
                label: "Context freshness".to_string(),
                detail: Some("No current context snapshot is persisted.".to_string()),
            }],
        });
    }

    push_integration_alert(
        &mut items,
        now,
        "calendar",
        "Calendar",
        integrations.google_calendar.last_sync_at,
        freshness_alert_status(
            integrations.google_calendar.last_sync_status.as_deref(),
            integrations.google_calendar.last_sync_at,
            !integrations.google_calendar.connected,
        ),
        integrations
            .google_calendar
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
    );
    push_integration_alert(
        &mut items,
        now,
        "todoist",
        "Todoist",
        integrations.todoist.last_sync_at,
        freshness_alert_status(
            integrations.todoist.last_sync_status.as_deref(),
            integrations.todoist.last_sync_at,
            !integrations.todoist.connected,
        ),
        integrations
            .todoist
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
    );
    push_integration_alert(
        &mut items,
        now,
        "activity",
        "Activity",
        integrations.activity.last_sync_at,
        freshness_alert_status(
            integrations.activity.last_sync_status.as_deref(),
            integrations.activity.last_sync_at,
            !integrations.activity.configured,
        ),
        integrations
            .activity
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
    );
    push_integration_alert(
        &mut items,
        now,
        "messaging",
        "Messaging",
        integrations.messaging.last_sync_at,
        freshness_alert_status(
            integrations.messaging.last_sync_status.as_deref(),
            integrations.messaging.last_sync_at,
            !integrations.messaging.configured,
        ),
        integrations
            .messaging
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
    );

    items
}

fn push_integration_alert(
    items: &mut Vec<ActionItem>,
    now: OffsetDateTime,
    key: &str,
    label: &str,
    last_sync_at: Option<i64>,
    status: Option<&'static str>,
    guidance: Option<String>,
) {
    let Some(status) = status else {
        return;
    };

    let detail = guidance.unwrap_or_else(|| match status {
        "error" => format!("{label} last sync failed."),
        "disconnected" => format!("{label} is disconnected from the runtime."),
        "missing" => format!("{label} has not synced into Vel yet."),
        other => format!("{label} needs attention ({other})."),
    });

    items.push(ActionItem {
        id: ActionItemId::from(format!("act_freshness_{key}")),
        surface: ActionSurface::Inbox,
        kind: ActionKind::Freshness,
        title: format!("{label} needs attention"),
        summary: detail.clone(),
        project_id: None,
        state: ActionState::Active,
        rank: FRESHNESS_ALERT_RANK,
        surfaced_at: timestamp_or_now(last_sync_at, now),
        snoozed_until: None,
        evidence: vec![ActionEvidenceRef {
            source_kind: "integration_status".to_string(),
            source_id: key.to_string(),
            label: format!("{label} freshness"),
            detail: Some(detail),
        }],
    });
}

fn freshness_alert_status(
    last_sync_status: Option<&str>,
    last_sync_at: Option<i64>,
    disconnected_by_config: bool,
) -> Option<&'static str> {
    if matches!(last_sync_status, Some("error")) {
        Some("error")
    } else if matches!(last_sync_status, Some("disconnected")) || disconnected_by_config {
        Some("disconnected")
    } else if last_sync_at.is_none() {
        Some("missing")
    } else {
        None
    }
}

fn build_linking_items(linked_nodes: Vec<LinkedNodeRecord>) -> Vec<ActionItem> {
    linked_nodes
        .into_iter()
        .filter_map(|node| match node.status {
            LinkStatus::Pending | LinkStatus::Expired | LinkStatus::Revoked => {
                let node_id = node.node_id.clone();
                Some(ActionItem {
                    id: ActionItemId::from(format!("act_linking_{}", node_id)),
                    surface: ActionSurface::Inbox,
                    kind: ActionKind::Linking,
                    title: format!("Linked node {} needs review", node.node_display_name),
                    summary: format!(
                        "Link state is {}. Inspect granted scopes before trusting cross-client continuity.",
                        node.status
                    ),
                    project_id: None,
                    state: ActionState::Active,
                    rank: LINKING_ALERT_RANK,
                    surfaced_at: node.last_seen_at.unwrap_or(node.linked_at),
                    snoozed_until: None,
                    evidence: vec![ActionEvidenceRef {
                        source_kind: "linked_node".to_string(),
                        source_id: node_id,
                        label: node.node_display_name,
                        detail: Some(format!("status={}, scopes={:?}", node.status, node.scopes)),
                    }],
                })
            }
            LinkStatus::Linked => None,
        })
        .collect()
}

fn build_intervention_items(interventions: Vec<InterventionRecord>) -> Vec<ActionItem> {
    interventions
        .into_iter()
        .map(|record| {
            let state = action_state_from_str(&record.state);
            let payload = record
                .source_json
                .as_deref()
                .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok());
            let title = payload
                .as_ref()
                .and_then(|value| value.get("title"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("Inbox intervention")
                .to_string();
            let summary = payload
                .as_ref()
                .and_then(|value| value.get("reason").or_else(|| value.get("summary")))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("Needs operator review from the intervention queue.")
                .to_string();

            ActionItem {
                id: ActionItemId::from(format!("act_intervention_{}", record.id.as_ref())),
                surface: ActionSurface::Inbox,
                kind: ActionKind::Intervention,
                title,
                summary,
                project_id: None,
                state,
                rank: if matches!(state, ActionState::Snoozed) {
                    SNOOZED_INTERVENTION_RANK
                } else {
                    INTERVENTION_RANK
                },
                surfaced_at: timestamp_or_now(Some(record.surfaced_at), OffsetDateTime::now_utc()),
                snoozed_until: record.snoozed_until.and_then(offset_datetime),
                evidence: vec![ActionEvidenceRef {
                    source_kind: "intervention".to_string(),
                    source_id: record.id.as_ref().to_string(),
                    label: record.kind,
                    detail: Some(format!("message_id={}", record.message_id)),
                }],
            }
        })
        .collect()
}

fn build_project_items(now: OffsetDateTime, projects: &[ProjectRecord]) -> Vec<ActionItem> {
    let mut items = Vec::new();

    for project in projects {
        if project.pending_provision.create_repo || project.pending_provision.create_notes_root {
            items.push(ActionItem {
                id: ActionItemId::from(format!("act_project_blocked_{}", project.id.as_ref())),
                surface: ActionSurface::Inbox,
                kind: ActionKind::Blocked,
                title: format!("Project {} needs provisioning review", project.name),
                summary: format!(
                    "Pending local-first provision flags: repo={}, notes_root={}.",
                    project.pending_provision.create_repo,
                    project.pending_provision.create_notes_root
                ),
                project_id: Some(project.id.clone()),
                state: ActionState::Active,
                rank: PROJECT_BLOCKED_RANK,
                surfaced_at: project.updated_at,
                snoozed_until: None,
                evidence: vec![ActionEvidenceRef {
                    source_kind: "project".to_string(),
                    source_id: project.id.as_ref().to_string(),
                    label: project.name.clone(),
                    detail: Some("pending_provision".to_string()),
                }],
            });
        }

        if matches!(project.status, ProjectStatus::Active) {
            items.push(ActionItem {
                id: ActionItemId::from(format!("act_project_review_{}", project.id.as_ref())),
                surface: ActionSurface::Now,
                kind: ActionKind::Review,
                title: format!("Review project {}", project.name),
                summary: "Weekly review keeps the project anchored in Now and Inbox.".to_string(),
                project_id: Some(project.id.clone()),
                state: ActionState::Active,
                rank: PROJECT_REVIEW_RANK,
                surfaced_at: if project.updated_at > project.created_at {
                    project.updated_at
                } else {
                    now
                },
                snoozed_until: None,
                evidence: vec![ActionEvidenceRef {
                    source_kind: "project".to_string(),
                    source_id: project.id.as_ref().to_string(),
                    label: project.name.clone(),
                    detail: Some(format!("family={}", project.family)),
                }],
            });
        }
    }

    items
}

fn build_commitment_items(
    now: OffsetDateTime,
    commitments: Vec<Commitment>,
    project_lookup: &HashMap<String, ProjectLookupEntry>,
) -> Vec<ActionItem> {
    let due_cutoff = now + time::Duration::hours(24);
    commitments
        .into_iter()
        .filter_map(|commitment| {
            let due_at = commitment.due_at?;
            if due_at > due_cutoff {
                return None;
            }

            let rank = if due_at < now {
                NEXT_COMMITMENT_RANK + 2
            } else {
                NEXT_COMMITMENT_RANK
            };
            let project_id = commitment
                .project
                .as_deref()
                .and_then(|value| project_lookup.get(&value.trim().to_lowercase()))
                .map(|entry| entry.id.clone());

            Some(ActionItem {
                id: ActionItemId::from(format!("act_commitment_{}", commitment.id.as_ref())),
                surface: ActionSurface::Now,
                kind: ActionKind::NextStep,
                title: commitment.text.clone(),
                summary: if due_at < now {
                    "This commitment is overdue and should be handled or rescheduled.".to_string()
                } else {
                    "This commitment is due within the next 24 hours.".to_string()
                },
                project_id,
                state: ActionState::Active,
                rank,
                surfaced_at: due_at,
                snoozed_until: None,
                evidence: vec![ActionEvidenceRef {
                    source_kind: "commitment".to_string(),
                    source_id: commitment.id.as_ref().to_string(),
                    label: commitment.text,
                    detail: commitment
                        .commitment_kind
                        .clone()
                        .or_else(|| commitment.project.clone()),
                }],
            })
        })
        .collect()
}

fn build_project_lookup(projects: &[ProjectRecord]) -> HashMap<String, ProjectLookupEntry> {
    let mut lookup = HashMap::new();
    for project in projects {
        let entry = ProjectLookupEntry {
            id: project.id.clone(),
        };
        lookup.insert(project.slug.to_lowercase(), entry.clone());
        lookup.insert(project.name.to_lowercase(), entry);
    }
    lookup
}

fn action_state_from_str(value: &str) -> ActionState {
    match value {
        "acknowledged" => ActionState::Acknowledged,
        "resolved" => ActionState::Resolved,
        "dismissed" => ActionState::Dismissed,
        "snoozed" => ActionState::Snoozed,
        _ => ActionState::Active,
    }
}

fn timestamp_or_now(timestamp: Option<i64>, now: OffsetDateTime) -> OffsetDateTime {
    timestamp.and_then(offset_datetime).unwrap_or(now)
}

fn offset_datetime(timestamp: i64) -> Option<OffsetDateTime> {
    OffsetDateTime::from_unix_timestamp(timestamp).ok()
}

#[derive(Debug, Clone)]
struct ProjectLookupEntry {
    id: ProjectId,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use time::Duration;
    use vel_core::{
        ConflictCaseKind, ConflictCaseStatus, IntegrationFamily, LinkScope, NodeIdentity,
        OrderingStamp, ProjectFamily, ProjectProvisionRequest, ProjectRootRef, ProjectStatus,
        WritebackOperationKind, WritebackRisk, WritebackStatus, WritebackTargetRef,
    };
    use vel_storage::{ConversationInsert, InterventionInsert, MessageInsert};

    #[tokio::test]
    async fn action_items_rank_freshness_linking_intervention_and_review_bands() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc();

        let conversation = storage
            .create_conversation(ConversationInsert {
                id: "conv_action_items".to_string(),
                title: Some("Inbox".to_string()),
                kind: "general".to_string(),
                pinned: false,
                archived: false,
            })
            .await
            .unwrap();
        let message_id = storage
            .create_message(MessageInsert {
                id: "msg_action_items".to_string(),
                conversation_id: conversation.id.as_ref().to_string(),
                role: "assistant".to_string(),
                kind: "risk_card".to_string(),
                content_json:
                    r#"{"title":"Link trust degraded","reason":"Review this queue item"}"#
                        .to_string(),
                status: None,
                importance: None,
            })
            .await
            .unwrap();
        storage
            .create_intervention(InterventionInsert {
                id: "intv_action_items".to_string(),
                message_id: message_id.as_ref().to_string(),
                kind: "risk".to_string(),
                state: "active".to_string(),
                surfaced_at: now.unix_timestamp(),
                resolved_at: None,
                snoozed_until: None,
                confidence: Some(0.9),
                source_json: Some(
                    serde_json::json!({
                        "title": "Inbox intervention",
                        "reason": "Operator review required"
                    })
                    .to_string(),
                ),
                provenance_json: None,
            })
            .await
            .unwrap();
        storage
            .upsert_linked_node(&LinkedNodeRecord {
                node_id: "node_beta".to_string(),
                node_display_name: "Beta".to_string(),
                status: LinkStatus::Revoked,
                scopes: LinkScope {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                linked_at: now,
                last_seen_at: Some(now),
                transport_hint: Some("tailscale".to_string()),
                sync_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
                tailscale_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
                lan_base_url: None,
                localhost_base_url: None,
                public_base_url: None,
            })
            .await
            .unwrap();
        storage
            .create_project(ProjectRecord {
                id: ProjectId::from("proj_action_items".to_string()),
                slug: "vel".to_string(),
                name: "Vel".to_string(),
                family: ProjectFamily::Work,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/vel".to_string(),
                    label: "vel".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/notes/vel".to_string(),
                    label: "vel".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: BTreeMap::new(),
                pending_provision: ProjectProvisionRequest {
                    create_repo: true,
                    create_notes_root: false,
                },
                created_at: now - Duration::days(2),
                updated_at: now,
                archived_at: None,
            })
            .await
            .unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Reply to project thread".to_string(),
                source_type: "todoist".to_string(),
                source_id: "todo_5".to_string(),
                status: CommitmentStatus::Open,
                due_at: Some(now + Duration::hours(4)),
                project: Some("vel".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({})),
            })
            .await
            .unwrap();
        storage
            .insert_writeback_operation(
                &WritebackOperationRecord {
                    id: "wb_action_items".to_string().into(),
                    kind: WritebackOperationKind::TodoistCreateTask,
                    risk: WritebackRisk::ConfirmRequired,
                    status: WritebackStatus::Queued,
                    target: WritebackTargetRef {
                        family: IntegrationFamily::Tasks,
                        provider_key: "todoist".to_string(),
                        project_id: Some("proj_action_items".to_string().into()),
                        connection_id: Some("icn_action_items".to_string().into()),
                        external_id: Some("todo_queued".to_string()),
                    },
                    requested_payload: serde_json::json!({"content": "queued"}),
                    result_payload: None,
                    provenance: vec![],
                    conflict_case_id: None,
                    requested_by_node_id: "node_alpha".to_string(),
                    requested_at: now,
                    applied_at: None,
                    updated_at: now,
                },
                &OrderingStamp::new(
                    now.unix_timestamp(),
                    1,
                    NodeIdentity::from("123e4567-e89b-12d3-a456-426614174000".to_string()),
                ),
            )
            .await
            .unwrap();
        storage
            .insert_conflict_case(&ConflictCaseRecord {
                id: "conf_action_items".to_string().into(),
                kind: ConflictCaseKind::UpstreamVsLocal,
                status: ConflictCaseStatus::Open,
                target: WritebackTargetRef {
                    family: IntegrationFamily::Tasks,
                    provider_key: "todoist".to_string(),
                    project_id: Some("proj_action_items".to_string().into()),
                    connection_id: Some("icn_action_items".to_string().into()),
                    external_id: Some("todo_conflict".to_string()),
                },
                summary: "Todoist differs upstream".to_string(),
                local_payload: serde_json::json!({"content": "local"}),
                upstream_payload: Some(serde_json::json!({"content": "remote"})),
                resolution_payload: None,
                opened_at: now,
                resolved_at: None,
                updated_at: now,
            })
            .await
            .unwrap();

        let snapshot = build_action_items(&storage, &AppConfig::default())
            .await
            .unwrap();

        assert!(snapshot.action_items.iter().any(|item| item.rank == 90));
        assert!(snapshot.action_items.iter().any(|item| item.rank == 85));
        assert!(snapshot.action_items.iter().any(|item| item.rank == 80));
        assert!(snapshot.action_items.iter().any(|item| item.rank == 75));
        assert!(snapshot.action_items.iter().any(|item| item.rank == 72));
        assert!(snapshot.action_items.iter().any(|item| item.rank == 70));
        assert!(snapshot.action_items.iter().any(|item| item.rank == 60));
        assert_eq!(snapshot.pending_writebacks.len(), 1);
        assert_eq!(snapshot.conflicts.len(), 1);
        assert!(snapshot
            .action_items
            .iter()
            .all(|item| !item.evidence.is_empty()));
        let writeback_item = snapshot
            .action_items
            .iter()
            .find(|item| item.id.as_ref() == "act_writeback_wb_action_items")
            .expect("writeback item should exist");
        assert_eq!(writeback_item.surface, ActionSurface::Now);
        assert!(writeback_item
            .evidence
            .iter()
            .any(|evidence| evidence.source_kind == "writeback_operation"));
        assert!(writeback_item
            .evidence
            .iter()
            .any(|evidence| evidence.source_kind == "integration_connection"));
        assert!(writeback_item
            .evidence
            .iter()
            .any(|evidence| evidence.source_kind == "project"));
        let conflict_item = snapshot
            .action_items
            .iter()
            .find(|item| item.id.as_ref() == "act_conflict_conf_action_items")
            .expect("conflict item should exist");
        assert_eq!(conflict_item.surface, ActionSurface::Now);
        assert!(conflict_item
            .evidence
            .iter()
            .any(|evidence| evidence.source_kind == "conflict_case"));
        assert!(conflict_item
            .evidence
            .iter()
            .any(|evidence| evidence.source_kind == "integration_connection"));
        assert!(conflict_item
            .evidence
            .iter()
            .any(|evidence| evidence.source_kind == "project"));
    }
}
