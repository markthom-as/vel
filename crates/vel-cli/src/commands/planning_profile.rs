use crate::client::ApiClient;
use vel_api_types::{
    DurableRoutineBlockData, PlanningConstraintData, PlanningConstraintKindData,
    PlanningProfileProposalSummaryData, RoutinePlanningProfileData, ScheduleTimeWindowData,
};

fn format_days(days: &[u8]) -> String {
    if days.is_empty() {
        return "every day".to_string();
    }

    days.iter()
        .map(|day| match day {
            1 => "Mon",
            2 => "Tue",
            3 => "Wed",
            4 => "Thu",
            5 => "Fri",
            6 => "Sat",
            7 => "Sun",
            _ => "?",
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_block(block: &DurableRoutineBlockData) -> String {
    let mut suffix = vec![
        if block.active { "active" } else { "inactive" }.to_string(),
        format_days(&block.days_of_week),
    ];
    if block.protected {
        suffix.push("protected".to_string());
    }

    format!(
        "- {} [{}-{} {}] ({})",
        block.label,
        block.start_local_time,
        block.end_local_time,
        block.local_timezone,
        suffix.join(", ")
    )
}

fn constraint_kind_label(kind: PlanningConstraintKindData) -> &'static str {
    match kind {
        PlanningConstraintKindData::MaxScheduledItems => "max scheduled items",
        PlanningConstraintKindData::ReserveBufferBeforeCalendar => "buffer before calendar",
        PlanningConstraintKindData::ReserveBufferAfterCalendar => "buffer after calendar",
        PlanningConstraintKindData::DefaultTimeWindow => "default time window",
        PlanningConstraintKindData::RequireJudgmentForOverflow => "require judgment for overflow",
    }
}

fn time_window_label(window: ScheduleTimeWindowData) -> &'static str {
    match window {
        ScheduleTimeWindowData::Prenoon => "prenoon",
        ScheduleTimeWindowData::Afternoon => "afternoon",
        ScheduleTimeWindowData::Evening => "evening",
        ScheduleTimeWindowData::Night => "night",
        ScheduleTimeWindowData::Day => "day",
    }
}

fn format_constraint(constraint: &PlanningConstraintData) -> String {
    let mut details = vec![constraint_kind_label(constraint.kind).to_string()];
    if let Some(window) = constraint.time_window {
        details.push(format!("window {}", time_window_label(window)));
    }
    if let Some(minutes) = constraint.minutes {
        details.push(format!("{minutes}m"));
    }
    if let Some(max_items) = constraint.max_items {
        details.push(format!("limit {max_items}"));
    }
    if let Some(detail) = &constraint.detail {
        if !detail.is_empty() {
            details.push(detail.clone());
        }
    }

    format!(
        "- {} ({}; {})",
        constraint.label,
        if constraint.active {
            "active"
        } else {
            "inactive"
        },
        details.join(", ")
    )
}

fn render_profile_text(
    profile: &RoutinePlanningProfileData,
    proposal_summary: Option<&PlanningProfileProposalSummaryData>,
) -> String {
    let active_blocks = profile
        .routine_blocks
        .iter()
        .filter(|block| block.active)
        .count();
    let active_constraints = profile
        .planning_constraints
        .iter()
        .filter(|constraint| constraint.active)
        .count();

    let mut lines = vec![
        "planning profile:".to_string(),
        format!(
            "routine blocks: {} active / {} total",
            active_blocks,
            profile.routine_blocks.len()
        ),
    ];

    if profile.routine_blocks.is_empty() {
        lines.push("  none yet".to_string());
    } else {
        for block in &profile.routine_blocks {
            lines.push(format!("  {}", format_block(block)));
        }
    }

    lines.push(format!(
        "planning constraints: {} active / {} total",
        active_constraints,
        profile.planning_constraints.len()
    ));
    if profile.planning_constraints.is_empty() {
        lines.push("  none yet".to_string());
    } else {
        for constraint in &profile.planning_constraints {
            lines.push(format!("  {}", format_constraint(constraint)));
        }
    }

    if let Some(summary) = proposal_summary {
        lines.push(format!(
            "proposal continuity: {} pending",
            summary.pending_count
        ));
        if let Some(item) = &summary.latest_pending {
            lines.push(format!("  pending: {}", item.title));
        }
        if let Some(item) = &summary.latest_applied {
            lines.push(format!("  last applied: {}", item.title));
        } else if let Some(item) = &summary.latest_failed {
            lines.push(format!(
                "  last failed: {}{}",
                item.title,
                item.outcome_summary
                    .as_deref()
                    .map(|summary| format!(" ({summary})"))
                    .unwrap_or_default()
            ));
        }
    }

    lines.join("\n")
}

pub async fn run(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.planning_profile().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let data = response
        .data
        .expect("planning profile response missing data");
    println!(
        "{}",
        render_profile_text(&data.profile, data.proposal_summary.as_ref())
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::render_profile_text;
    use vel_api_types::{
        DurableRoutineBlockData, PlanningConstraintData, PlanningConstraintKindData,
        RoutineBlockSourceKindData, RoutinePlanningProfileData, ScheduleTimeWindowData,
    };

    #[test]
    fn planning_profile_text_uses_summary_first_framing() {
        let profile = RoutinePlanningProfileData {
            routine_blocks: vec![DurableRoutineBlockData {
                id: "routine_focus".to_string(),
                label: "Focus".to_string(),
                source: RoutineBlockSourceKindData::OperatorDeclared,
                local_timezone: "America/Denver".to_string(),
                start_local_time: "09:00".to_string(),
                end_local_time: "11:00".to_string(),
                days_of_week: vec![1, 2, 3, 4, 5],
                protected: true,
                active: true,
            }],
            planning_constraints: vec![PlanningConstraintData {
                id: "constraint_prenoon".to_string(),
                label: "Default prenoon".to_string(),
                kind: PlanningConstraintKindData::DefaultTimeWindow,
                detail: None,
                time_window: Some(ScheduleTimeWindowData::Prenoon),
                minutes: None,
                max_items: None,
                active: true,
            }],
        };

        let rendered = render_profile_text(&profile, None);
        assert!(rendered.contains("planning profile:"));
        assert!(rendered.contains("routine blocks: 1 active / 1 total"));
        assert!(rendered.contains("Focus [09:00-11:00 America/Denver]"));
        assert!(rendered.contains("planning constraints: 1 active / 1 total"));
        assert!(rendered.contains("Default prenoon"));
    }

    #[test]
    fn planning_profile_text_surfaces_proposal_continuity_summary() {
        let profile = RoutinePlanningProfileData {
            routine_blocks: vec![],
            planning_constraints: vec![],
        };
        let rendered = render_profile_text(
            &profile,
            Some(&vel_api_types::PlanningProfileProposalSummaryData {
                pending_count: 1,
                latest_pending: Some(vel_api_types::PlanningProfileProposalSummaryItemData {
                    thread_id: "thr_planning_profile_edit_1".to_string(),
                    state: vel_api_types::AssistantProposalStateData::Staged,
                    title: "Add shutdown block".to_string(),
                    summary: "Add a protected shutdown block.".to_string(),
                    outcome_summary: None,
                    updated_at: 1_710_000_000,
                }),
                latest_applied: None,
                latest_failed: None,
            }),
        );

        assert!(rendered.contains("proposal continuity: 1 pending"));
        assert!(rendered.contains("pending: Add shutdown block"));
    }
}
