//! `vel review` — daily and weekly review views.

use crate::client::ApiClient;
use crate::commands::doctor::backup_summary_lines;
use vel_api_types::{
    ActionItemData, BackupTrustData, CommitmentSchedulingProposalSummaryData, NowData, PersonRecordData,
    ReviewSnapshotData,
};

const TRUNCATE: usize = 50;
const TOP_ACTION_TITLES_LIMIT: usize = 3;

pub async fn run_today(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let captures_resp = client.list_captures_recent(20, true).await?;
    let captures = captures_resp
        .data
        .expect("list_captures_recent missing data");
    let now = client.get_now().await?.data.expect("get_now missing data");
    let doctor = client.doctor().await?.data.expect("doctor missing data");
    let latest_ctx = client
        .get_artifact_latest("context_brief")
        .await
        .ok()
        .and_then(|r| r.data);

    if json {
        let review_counts = summarize_review_snapshot(&now.review_snapshot);
        let out = serde_json::json!({
            "captures_today": captures.len(),
            "captures": captures,
            "latest_context_artifact": latest_ctx,
            "open_action_count": review_counts.0,
            "triage_count": review_counts.1,
            "commitment_scheduling_summary": now.commitment_scheduling_summary,
            "pending_writebacks": now.pending_writebacks.len(),
            "open_conflicts": now.conflicts.len(),
            "people_needing_review": people_needing_review(&now).len(),
            "top_action_titles": top_action_titles(&now.action_items),
            "backup": doctor.backup,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("=== Review: today ===\n");
    println!("Captures today: {}", captures.len());
    if !captures.is_empty() {
        for c in &captures {
            let content = if c.content_text.len() > TRUNCATE {
                format!("{}...", &c.content_text[..TRUNCATE])
            } else {
                c.content_text.clone()
            };
            println!("  {}  {}  {}", c.capture_id, c.occurred_at, content);
        }
    }
    println!();
    if let Some(Some(ref a)) = latest_ctx {
        println!(
            "Latest context artifact: {}  ({})",
            a.artifact_id, a.storage_uri
        );
    } else {
        println!("Latest context artifact: (none)");
    }
    print_commitment_scheduling_summary(now.commitment_scheduling_summary.as_ref());
    println!("Pending writebacks: {}", now.pending_writebacks.len());
    println!("Open conflicts: {}", now.conflicts.len());
    println!(
        "People needing review: {}",
        people_needing_review(&now).len()
    );
    print_backup_summary(&doctor.backup);
    Ok(())
}

pub async fn run_week(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let captures_resp = client.list_captures_recent(50, false).await?;
    let captures = captures_resp
        .data
        .expect("list_captures_recent missing data");
    let latest_ctx = client
        .get_artifact_latest("context_brief")
        .await
        .ok()
        .and_then(|r| r.data);
    let now = client.get_now().await?.data.expect("get_now missing data");
    let doctor = client.doctor().await?.data.expect("doctor missing data");

    if json {
        let out = serde_json::json!({
            "captures_recent": captures.len(),
            "captures": captures,
            "latest_context_artifact": latest_ctx,
            "commitment_scheduling_summary": now.commitment_scheduling_summary,
            "pending_writebacks": now.pending_writebacks.len(),
            "open_conflicts": now.conflicts.len(),
            "people_needing_review": people_needing_review(&now).len(),
            "backup": doctor.backup,
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("=== Review: week (recent {}) ===\n", captures.len());
    println!("Recent captures: {}", captures.len());
    if !captures.is_empty() {
        for c in &captures {
            let content = if c.content_text.len() > TRUNCATE {
                format!("{}...", &c.content_text[..TRUNCATE])
            } else {
                c.content_text.clone()
            };
            println!("  {}  {}  {}", c.capture_id, c.occurred_at, content);
        }
    }
    println!();
    if let Some(Some(ref a)) = latest_ctx {
        println!(
            "Latest context artifact: {}  ({})",
            a.artifact_id, a.storage_uri
        );
    } else {
        println!("Latest context artifact: (none)");
    }
    print_commitment_scheduling_summary(now.commitment_scheduling_summary.as_ref());
    println!("Pending writebacks: {}", now.pending_writebacks.len());
    println!("Open conflicts: {}", now.conflicts.len());
    println!(
        "People needing review: {}",
        people_needing_review(&now).len()
    );
    print_backup_summary(&doctor.backup);
    Ok(())
}

fn summarize_review_snapshot(snapshot: &ReviewSnapshotData) -> (u32, u32) {
    (snapshot.open_action_count, snapshot.triage_count)
}

fn top_action_titles(action_items: &[ActionItemData]) -> Vec<String> {
    action_items
        .iter()
        .take(TOP_ACTION_TITLES_LIMIT)
        .map(|item| item.title.clone())
        .collect()
}

fn print_backup_summary(backup: &BackupTrustData) {
    println!();
    for line in backup_summary_lines(backup) {
        println!("{line}");
    }
}

fn commitment_scheduling_summary_lines(
    summary: Option<&CommitmentSchedulingProposalSummaryData>,
) -> Vec<String> {
    let Some(summary) = summary else {
        return Vec::new();
    };

    let mut lines = vec![format!(
        "Schedule continuity: {} pending",
        summary.pending_count
    )];
    if let Some(item) = &summary.latest_pending {
        lines.push(format!("  Pending: {}", item.title));
    }
    if let Some(item) = &summary.latest_applied {
        lines.push(format!(
            "  Last applied: {}{}",
            item.title,
            item.outcome_summary
                .as_deref()
                .map(|summary| format!(" ({summary})"))
                .unwrap_or_default()
        ));
    } else if let Some(item) = &summary.latest_failed {
        lines.push(format!(
            "  Last failed: {}{}",
            item.title,
            item.outcome_summary
                .as_deref()
                .map(|summary| format!(" ({summary})"))
                .unwrap_or_default()
        ));
    }
    lines
}

fn print_commitment_scheduling_summary(summary: Option<&CommitmentSchedulingProposalSummaryData>) {
    for line in commitment_scheduling_summary_lines(summary) {
        println!("{line}");
    }
}

fn people_needing_review(now: &NowData) -> Vec<PersonRecordData> {
    let person_ids: std::collections::HashSet<&str> = now
        .action_items
        .iter()
        .flat_map(|item| item.evidence.iter())
        .filter(|evidence| evidence.source_kind == "person")
        .map(|evidence| evidence.source_id.as_str())
        .collect();

    now.people
        .iter()
        .filter(|person| person_ids.contains(person.id.as_ref()))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        commitment_scheduling_summary_lines,
        people_needing_review, summarize_review_snapshot, top_action_titles,
    };
    use serde_json::json;
    use time::OffsetDateTime;
    use vel_api_types::{
        ActionEvidenceRefData, ActionItemData, ActionKindData, ActionPermissionModeData,
        ActionScopeAffinityData, ActionStateData, ActionSurfaceData, NowAttentionData, NowData,
        NowDebugData, NowFreshnessData, NowLabelData, NowRiskSummaryData, NowScheduleData,
        NowSourcesData, NowSummaryData, NowTasksData, PersonRecordData,
        ReviewSnapshotData, TrustReadinessData, TrustReadinessFacetData,
        TrustReadinessReviewData,
    };
    use vel_core::{ActionItemId, PersonId};

    #[test]
    fn review_today_helpers_surface_phase5_counts() {
        let counts = summarize_review_snapshot(&ReviewSnapshotData {
            open_action_count: 4,
            triage_count: 2,
            projects_needing_review: 1,
            pending_execution_reviews: 0,
        });
        let titles = top_action_titles(&[
            ActionItemData {
                id: ActionItemId::from("act_1".to_string()),
                surface: ActionSurfaceData::Now,
                kind: ActionKindData::NextStep,
                permission_mode: ActionPermissionModeData::UserConfirm,
                scope_affinity: ActionScopeAffinityData::Global,
                title: "Ship Phase 05".to_string(),
                summary: "Finish the slice".to_string(),
                project_id: None,
                project_label: None,
                project_family: None,
                state: ActionStateData::Active,
                rank: 90,
                surfaced_at: OffsetDateTime::UNIX_EPOCH,
                snoozed_until: None,
                evidence: vec![],
                thread_route: None,
            },
            ActionItemData {
                id: ActionItemId::from("act_2".to_string()),
                surface: ActionSurfaceData::Inbox,
                kind: ActionKindData::Review,
                permission_mode: ActionPermissionModeData::UserConfirm,
                scope_affinity: ActionScopeAffinityData::Global,
                title: "Triage stale thread".to_string(),
                summary: "Open the thread".to_string(),
                project_id: None,
                project_label: None,
                project_family: None,
                state: ActionStateData::Active,
                rank: 80,
                surfaced_at: OffsetDateTime::UNIX_EPOCH,
                snoozed_until: None,
                evidence: vec![],
                thread_route: None,
            },
        ]);

        assert_eq!(counts, (4, 2));
        assert_eq!(titles, vec!["Ship Phase 05", "Triage stale thread"]);
    }

    #[test]
    fn review_people_needing_review_filters_people_from_action_evidence() {
        let now = NowData {
            computed_at: 0,
            timezone: "America/Denver".to_string(),
            header: None,
            mesh_summary: None,
            status_row: None,
            context_line: None,
            nudge_bars: Vec::new(),
            task_lane: None,
            next_up_items: Vec::new(),
            progress: vel_api_types::NowProgressData {
                base_count: 0,
                completed_count: 0,
                backlog_count: 0,
                completed_ratio: 0.0,
                backlog_ratio: 1.0,
            },
            docked_input: None,
            overview: vel_api_types::NowOverviewData {
                dominant_action: None,
                today_timeline: vec![],
                visible_nudge: None,
                why_state: vec![],
                suggestions: vec![],
                decision_options: vec![
                    "accept".to_string(),
                    "choose".to_string(),
                    "thread".to_string(),
                    "close".to_string(),
                ],
            },
            summary: NowSummaryData {
                mode: NowLabelData {
                    key: "focus".to_string(),
                    label: "Focus".to_string(),
                },
                phase: NowLabelData {
                    key: "engaged".to_string(),
                    label: "Engaged".to_string(),
                },
                meds: NowLabelData {
                    key: "ok".to_string(),
                    label: "OK".to_string(),
                },
                risk: NowRiskSummaryData {
                    level: "low".to_string(),
                    score: Some(0.2),
                    label: "low".to_string(),
                },
            },
            schedule: NowScheduleData {
                empty_message: None,
                next_event: None,
                upcoming_events: vec![],
                following_day_events: Vec::new(),
            },
            check_in: None,
            day_plan: None,
            reflow: None,
            reflow_status: None,
            tasks: NowTasksData {
                todoist: vec![],
                other_open: vec![],
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
                    key: "none".to_string(),
                    label: "None".to_string(),
                },
                confidence: Some(0.8),
                reasons: vec![],
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
                sources: vec![],
            },
            planning_profile_summary: None,
            commitment_scheduling_summary: None,
            trust_readiness: Some(TrustReadinessData {
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
                    open_action_count: 1,
                    pending_execution_reviews: 0,
                    pending_writeback_count: 0,
                    conflict_count: 0,
                },
                guidance: vec![],
                follow_through: vec![],
            }),
            action_items: vec![ActionItemData {
                id: ActionItemId::from("act_person".to_string()),
                surface: ActionSurfaceData::Now,
                kind: ActionKindData::NextStep,
                permission_mode: ActionPermissionModeData::UserConfirm,
                scope_affinity: ActionScopeAffinityData::Global,
                title: "Reply to Annie".to_string(),
                summary: "Draft reply pending".to_string(),
                project_id: None,
                project_label: None,
                project_family: None,
                state: ActionStateData::Active,
                rank: 72,
                surfaced_at: OffsetDateTime::UNIX_EPOCH,
                snoozed_until: None,
                evidence: vec![ActionEvidenceRefData {
                    source_kind: "person".to_string(),
                    source_id: "per_annie".to_string(),
                    label: "Annie Case".to_string(),
                    detail: None,
                }],
                thread_route: None,
            }],
            review_snapshot: ReviewSnapshotData {
                open_action_count: 1,
                triage_count: 0,
                projects_needing_review: 0,
                pending_execution_reviews: 0,
            },
            pending_writebacks: vec![],
            conflicts: vec![],
            people: vec![
                PersonRecordData {
                    id: PersonId::from("per_annie".to_string()),
                    display_name: "Annie Case".to_string(),
                    given_name: Some("Annie".to_string()),
                    family_name: Some("Case".to_string()),
                    relationship_context: None,
                    birthday: None,
                    last_contacted_at: None,
                    aliases: vec![],
                    links: vec![],
                },
                PersonRecordData {
                    id: PersonId::from("per_other".to_string()),
                    display_name: "Other Person".to_string(),
                    given_name: None,
                    family_name: None,
                    relationship_context: None,
                    birthday: None,
                    last_contacted_at: None,
                    aliases: vec![],
                    links: vec![],
                },
            ],
            reasons: vec![],
            debug: NowDebugData {
                raw_context: json!({}),
                signals_used: vec![],
                commitments_used: vec![],
                risk_used: vec![],
            },
        };

        let people = people_needing_review(&now);

        assert_eq!(people.len(), 1);
        assert_eq!(people[0].display_name, "Annie Case");
    }

    #[test]
    fn commitment_scheduling_summary_lines_render_pending_and_last_applied() {
        let lines = commitment_scheduling_summary_lines(Some(
            &vel_api_types::CommitmentSchedulingProposalSummaryData {
                pending_count: 1,
                latest_pending: Some(vel_api_types::CommitmentSchedulingProposalSummaryItemData {
                    thread_id: "thr_day_plan_apply_1".to_string(),
                    state: vel_api_types::AssistantProposalStateData::Staged,
                    title: "Apply focus block shift".to_string(),
                    summary: "Move the focus block after the calendar anchor.".to_string(),
                    outcome_summary: None,
                    updated_at: 1_710_000_000,
                }),
                latest_applied: Some(vel_api_types::CommitmentSchedulingProposalSummaryItemData {
                    thread_id: "thr_reflow_edit_0".to_string(),
                    state: vel_api_types::AssistantProposalStateData::Applied,
                    title: "Clear stale due time".to_string(),
                    summary: "Remove the stale due time from one commitment.".to_string(),
                    outcome_summary: Some(
                        "Commitment scheduling proposal applied through canonical mutation seam."
                            .to_string(),
                    ),
                    updated_at: 1_709_999_900,
                }),
                latest_failed: None,
            },
        ));

        assert_eq!(lines[0], "Schedule continuity: 1 pending");
        assert_eq!(lines[1], "  Pending: Apply focus block shift");
        assert!(lines[2].contains("Last applied: Clear stale due time"));
    }
}
