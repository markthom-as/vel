use anyhow::anyhow;
use vel_api_types::{
    AgentBlockerData, AgentCapabilityEntryData, AgentCapabilityGroupData, AgentInspectData,
};

use crate::client::ApiClient;

pub async fn run_inspect(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.get_agent_inspect().await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let inspect = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("agent inspect response missing data"))?;
    println!("{}", format_agent_inspect(inspect));
    Ok(())
}

fn format_agent_inspect(inspect: &AgentInspectData) -> String {
    let grounding = &inspect.grounding;
    let mut lines = vec![
        "=== Agent grounding ===".to_string(),
        String::new(),
        format!("generated_at: {}", grounding.generated_at),
        format!("timezone: {}", grounding.now.timezone),
        format!(
            "now: {} / {} / {}",
            grounding.now.summary.mode.label,
            grounding.now.summary.phase.label,
            grounding.now.attention.state.label
        ),
    ];

    if let Some(current_context) = &grounding.current_context {
        lines.push(format!(
            "current_context: {} / {}",
            current_context.mode.as_deref().unwrap_or("unknown"),
            current_context
                .morning_state
                .as_deref()
                .unwrap_or("unknown"),
        ));
    }

    lines.push(String::new());
    lines.push("grounding scope:".to_string());
    lines.push(format!("- projects: {}", grounding.projects.len()));
    lines.push(format!("- people: {}", grounding.people.len()));
    lines.push(format!("- commitments: {}", grounding.commitments.len()));
    lines.push(format!(
        "- pending writebacks: {}",
        grounding.review.pending_writebacks.len()
    ));
    lines.push(format!(
        "- open conflicts: {}",
        grounding.review.conflicts.len()
    ));
    lines.push(format!(
        "- pending execution handoffs: {}",
        grounding.review.pending_execution_handoffs.len()
    ));
    lines.push(
        "- assistant proposals: staged only; supervised writes stay review-gated".to_string(),
    );

    lines.push(String::new());
    lines.push("review obligations:".to_string());
    lines.push(format!(
        "- open action count: {}",
        grounding.review.review_snapshot.open_action_count
    ));
    lines.push(format!(
        "- triage count: {}",
        grounding.review.review_snapshot.triage_count
    ));
    lines.push(format!(
        "- projects needing review: {}",
        grounding.review.review_snapshot.projects_needing_review
    ));

    for group in &inspect.capabilities.groups {
        lines.push(String::new());
        lines.push(format!("{}:", group.label));
        lines.extend(format_capability_group(group));
    }

    if !inspect.blockers.is_empty() {
        lines.push(String::new());
        lines.push("blockers:".to_string());
        for blocker in &inspect.blockers {
            lines.push(format!("- {}", format_blocker(blocker)));
        }
    }

    if !inspect.explainability.supporting_paths.is_empty() {
        lines.push(String::new());
        lines.push("supporting paths:".to_string());
        lines.extend(
            inspect
                .explainability
                .supporting_paths
                .iter()
                .map(|path| format!("- {}", path)),
        );
    }

    lines.join("\n")
}

fn format_capability_group(group: &AgentCapabilityGroupData) -> Vec<String> {
    if group.entries.is_empty() {
        return vec!["- none".to_string()];
    }

    group
        .entries
        .iter()
        .flat_map(|entry| format_capability_entry(entry))
        .collect()
}

fn format_capability_entry(entry: &AgentCapabilityEntryData) -> Vec<String> {
    let mut lines = vec![format!(
        "- {}: {} [{}]",
        entry.label,
        entry.summary,
        if entry.available {
            "available"
        } else {
            "blocked"
        }
    )];
    if let Some(review_gate) = entry.requires_review_gate {
        lines.push(format!("  review gate: {}", review_gate_label(review_gate)));
    }
    if entry.requires_writeback_enabled {
        lines.push("  requires: writeback enabled".to_string());
    }
    if let Some(blocker) = &entry.blocked_reason {
        lines.push(format!("  blocker: {}", format_blocker(blocker)));
    }
    lines
}

fn format_blocker(blocker: &AgentBlockerData) -> String {
    match blocker.escalation_hint.as_deref() {
        Some(hint) if !hint.trim().is_empty() => {
            format!("{} ({}) -> {}", blocker.message, blocker.code, hint)
        }
        _ => format!("{} ({})", blocker.message, blocker.code),
    }
}

fn review_gate_label(review_gate: vel_api_types::ExecutionReviewGateData) -> &'static str {
    match review_gate {
        vel_api_types::ExecutionReviewGateData::None => "none",
        vel_api_types::ExecutionReviewGateData::OperatorApproval => "operator approval",
        vel_api_types::ExecutionReviewGateData::OperatorPreview => "operator preview",
        vel_api_types::ExecutionReviewGateData::PostRunReview => "post-run review",
    }
}

#[cfg(test)]
mod tests {
    use super::format_agent_inspect;
    use time::OffsetDateTime;
    use vel_api_types::{
        AgentBlockerData, AgentCapabilityEntryData, AgentCapabilityGroupData,
        AgentCapabilityGroupKindData, AgentCapabilitySummaryData, AgentContextRefData,
        AgentGroundingPackData, AgentInspectData, AgentInspectExplainabilityData,
        AgentReviewObligationsData, CommitmentData, ExecutionReviewGateData, NowAttentionData,
        NowData, NowDebugData, NowFreshnessData, NowLabelData, NowRiskSummaryData, NowScheduleData,
        NowSourcesData, NowSummaryData, NowTasksData, ReviewSnapshotData, TrustReadinessData,
        TrustReadinessFacetData, TrustReadinessReviewData,
    };

    fn sample_inspect() -> AgentInspectData {
        AgentInspectData {
            grounding: AgentGroundingPackData {
                generated_at: 1_763_661_000,
                now: NowData {
                    computed_at: 1_763_661_000,
                    timezone: "America/Denver".to_string(),
                    summary: NowSummaryData {
                        mode: NowLabelData {
                            key: "focused".to_string(),
                            label: "Focused".to_string(),
                        },
                        phase: NowLabelData {
                            key: "morning_overview".to_string(),
                            label: "Morning overview".to_string(),
                        },
                        meds: NowLabelData {
                            key: "done".to_string(),
                            label: "Done".to_string(),
                        },
                        risk: NowRiskSummaryData {
                            level: "low".to_string(),
                            score: Some(0.18),
                            label: "Low risk".to_string(),
                        },
                    },
                    schedule: NowScheduleData {
                        empty_message: None,
                        next_event: None,
                        upcoming_events: Vec::new(),
                    },
                    tasks: NowTasksData {
                        todoist: Vec::new(),
                        other_open: Vec::new(),
                        next_commitment: None,
                    },
                    attention: NowAttentionData {
                        state: NowLabelData {
                            key: "on_task".to_string(),
                            label: "On task".to_string(),
                        },
                        drift: NowLabelData {
                            key: "none".to_string(),
                            label: "None".to_string(),
                        },
                        severity: NowLabelData {
                            key: "low".to_string(),
                            label: "Low".to_string(),
                        },
                        confidence: Some(0.9),
                        reasons: Vec::new(),
                    },
                    sources: NowSourcesData {
                        git_activity: None,
                        health: None,
                        mood: None,
                        pain: None,
                        note_document: None,
                        assistant_message: None,
                    },
                    freshness: NowFreshnessData {
                        overall_status: "fresh".to_string(),
                        sources: Vec::new(),
                    },
                    trust_readiness: TrustReadinessData {
                        level: "ok".to_string(),
                        headline: "Trust looks good".to_string(),
                        summary: "No trust blockers are active.".to_string(),
                        backup: TrustReadinessFacetData {
                            level: "ok".to_string(),
                            label: "Backup".to_string(),
                            detail: "Recent backup available".to_string(),
                        },
                        freshness: TrustReadinessFacetData {
                            level: "ok".to_string(),
                            label: "Freshness".to_string(),
                            detail: "Context is fresh".to_string(),
                        },
                        review: TrustReadinessReviewData {
                            open_action_count: 3,
                            pending_execution_reviews: 0,
                            pending_writeback_count: 0,
                            conflict_count: 0,
                        },
                        guidance: Vec::new(),
                        follow_through: Vec::new(),
                    },
                    check_in: None,
                    reflow: None,
                    reflow_status: None,
                    action_items: Vec::new(),
                    review_snapshot: ReviewSnapshotData {
                        open_action_count: 3,
                        triage_count: 1,
                        projects_needing_review: 1,
                        pending_execution_reviews: 0,
                    },
                    pending_writebacks: Vec::new(),
                    conflicts: Vec::new(),
                    people: Vec::new(),
                    reasons: Vec::new(),
                    debug: NowDebugData {
                        raw_context: serde_json::json!({}),
                        signals_used: Vec::new(),
                        commitments_used: Vec::new(),
                        risk_used: Vec::new(),
                    },
                },
                current_context: Some(AgentContextRefData {
                    computed_at: 1_763_661_000,
                    mode: Some("focused".to_string()),
                    morning_state: Some("engaged".to_string()),
                    current_context_path: "/v1/context/current".to_string(),
                    explain_context_path: "/v1/explain/context".to_string(),
                    explain_drift_path: "/v1/explain/drift".to_string(),
                }),
                projects: Vec::new(),
                people: Vec::new(),
                commitments: vec![CommitmentData {
                    id: "com_1".to_string().into(),
                    text: "Ship Phase 11".to_string(),
                    source_type: "todoist".to_string(),
                    source_id: Some("todo_1".to_string()),
                    status: "open".to_string(),
                    due_at: Some(OffsetDateTime::now_utc()),
                    project: Some("proj_vel".to_string()),
                    commitment_kind: Some("must".to_string()),
                    created_at: OffsetDateTime::now_utc(),
                    resolved_at: None,
                    metadata: serde_json::json!({ "priority": 1 }),
                }],
                review: AgentReviewObligationsData {
                    review_snapshot: ReviewSnapshotData {
                        open_action_count: 3,
                        triage_count: 1,
                        projects_needing_review: 1,
                        pending_execution_reviews: 0,
                    },
                    pending_writebacks: Vec::new(),
                    conflicts: Vec::new(),
                    pending_execution_handoffs: Vec::new(),
                },
            },
            capabilities: AgentCapabilitySummaryData {
                groups: vec![AgentCapabilityGroupData {
                    kind: AgentCapabilityGroupKindData::MutationActions,
                    label: "Bounded mutation affordances".to_string(),
                    entries: vec![AgentCapabilityEntryData {
                        key: "integration_writeback".to_string(),
                        label: "Request integration writeback".to_string(),
                        summary: "Bounded upstream mutations remain subject to SAFE MODE and review gates.".to_string(),
                        available: false,
                        blocked_reason: Some(AgentBlockerData {
                            code: "safe_mode_enabled".to_string(),
                            message: "SAFE MODE keeps writeback disabled.".to_string(),
                            escalation_hint: Some("Enable writeback in Settings before retrying.".to_string()),
                        }),
                        requires_review_gate: Some(ExecutionReviewGateData::OperatorPreview),
                        requires_writeback_enabled: true,
                    }],
                }],
            },
            blockers: vec![AgentBlockerData {
                code: "writeback_disabled".to_string(),
                message: "Writeback-dependent mutation requests are unavailable while SAFE MODE is enabled.".to_string(),
                escalation_hint: Some("Enable writeback or stay within read/review lanes.".to_string()),
            }],
            explainability: AgentInspectExplainabilityData {
                persisted_record_kinds: vec!["now".to_string()],
                supporting_paths: vec!["/v1/agent/inspect".to_string()],
                raw_context_json_supporting_only: true,
            },
        }
    }

    #[test]
    fn agent_inspect_format_surfaces_grounding_and_blocker_language() {
        let text = format_agent_inspect(&sample_inspect());

        assert!(text.contains("=== Agent grounding ==="));
        assert!(text.contains("current_context: focused / engaged"));
        assert!(text.contains("review obligations:"));
        assert!(text.contains("Request integration writeback"));
        assert!(text.contains("operator preview"));
        assert!(text.contains("Enable writeback in Settings before retrying."));
        assert!(text.contains("writeback_disabled"));
    }
}
