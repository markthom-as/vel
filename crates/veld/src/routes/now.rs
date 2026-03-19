use axum::extract::State;
use axum::Json;
use vel_api_types::{
    ActionItemData, ApiResponse, NowAttentionData, NowData, NowDebugData, NowEventData,
    NowFreshnessData, NowFreshnessEntryData, NowLabelData, NowRiskSummaryData, NowScheduleData,
    NowSourceActivityData, NowSourcesData, NowSummaryData, NowTaskData, NowTasksData,
};

use crate::{errors::AppError, routes::response, services, state::AppState};

pub async fn get_now(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<NowData>>, AppError> {
    let data = services::now::get_now(&state.storage, &state.config).await?;
    Ok(response::success(data.into()))
}

impl From<services::now::NowOutput> for NowData {
    fn from(value: services::now::NowOutput) -> Self {
        Self {
            computed_at: value.computed_at,
            timezone: value.timezone,
            summary: value.summary.into(),
            schedule: value.schedule.into(),
            tasks: value.tasks.into(),
            attention: value.attention.into(),
            sources: value.sources.into(),
            freshness: value.freshness.into(),
            action_items: value
                .action_items
                .into_iter()
                .map(ActionItemData::from)
                .collect(),
            review_snapshot: value.review_snapshot.into(),
            reasons: value.reasons,
            debug: value.debug.into(),
        }
    }
}

impl From<services::now::NowSummaryOutput> for NowSummaryData {
    fn from(value: services::now::NowSummaryOutput) -> Self {
        Self {
            mode: value.mode.into(),
            phase: value.phase.into(),
            meds: value.meds.into(),
            risk: value.risk.into(),
        }
    }
}

impl From<services::now::NowLabelOutput> for NowLabelData {
    fn from(value: services::now::NowLabelOutput) -> Self {
        Self {
            key: value.key,
            label: value.label,
        }
    }
}

impl From<services::now::NowRiskSummaryOutput> for NowRiskSummaryData {
    fn from(value: services::now::NowRiskSummaryOutput) -> Self {
        Self {
            level: value.level,
            score: value.score,
            label: value.label,
        }
    }
}

impl From<services::now::NowScheduleOutput> for NowScheduleData {
    fn from(value: services::now::NowScheduleOutput) -> Self {
        Self {
            empty_message: value.empty_message,
            next_event: value.next_event.map(Into::into),
            upcoming_events: value.upcoming_events.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<services::now::NowEventOutput> for NowEventData {
    fn from(value: services::now::NowEventOutput) -> Self {
        Self {
            title: value.title,
            start_ts: value.start_ts,
            end_ts: value.end_ts,
            location: value.location,
            prep_minutes: value.prep_minutes,
            travel_minutes: value.travel_minutes,
            leave_by_ts: value.leave_by_ts,
        }
    }
}

impl From<services::now::NowTasksOutput> for NowTasksData {
    fn from(value: services::now::NowTasksOutput) -> Self {
        Self {
            todoist: value.todoist.into_iter().map(Into::into).collect(),
            other_open: value.other_open.into_iter().map(Into::into).collect(),
            next_commitment: value.next_commitment.map(Into::into),
        }
    }
}

impl From<services::now::NowTaskOutput> for NowTaskData {
    fn from(value: services::now::NowTaskOutput) -> Self {
        Self {
            id: value.id,
            text: value.text,
            source_type: value.source_type,
            due_at: value.due_at,
            project: value.project,
            commitment_kind: value.commitment_kind,
        }
    }
}

impl From<services::now::NowAttentionOutput> for NowAttentionData {
    fn from(value: services::now::NowAttentionOutput) -> Self {
        Self {
            state: value.state.into(),
            drift: value.drift.into(),
            severity: value.severity.into(),
            confidence: value.confidence,
            reasons: value.reasons,
        }
    }
}

impl From<services::now::NowSourcesOutput> for NowSourcesData {
    fn from(value: services::now::NowSourcesOutput) -> Self {
        Self {
            git_activity: value.git_activity.map(Into::into),
            health: value.health.map(Into::into),
            mood: value.mood.map(Into::into),
            pain: value.pain.map(Into::into),
            note_document: value.note_document.map(Into::into),
            assistant_message: value.assistant_message.map(Into::into),
        }
    }
}

impl From<services::now::NowSourceActivityOutput> for NowSourceActivityData {
    fn from(value: services::now::NowSourceActivityOutput) -> Self {
        Self {
            label: value.label,
            timestamp: value.timestamp,
            summary: value.summary,
        }
    }
}

impl From<services::now::NowFreshnessOutput> for NowFreshnessData {
    fn from(value: services::now::NowFreshnessOutput) -> Self {
        Self {
            overall_status: value.overall_status,
            sources: value.sources.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<services::now::NowFreshnessEntryOutput> for NowFreshnessEntryData {
    fn from(value: services::now::NowFreshnessEntryOutput) -> Self {
        Self {
            key: value.key,
            label: value.label,
            status: value.status,
            last_sync_at: value.last_sync_at,
            age_seconds: value.age_seconds,
            guidance: value.guidance,
        }
    }
}

impl From<services::now::NowDebugOutput> for NowDebugData {
    fn from(value: services::now::NowDebugOutput) -> Self {
        Self {
            raw_context: value.raw_context,
            signals_used: value.signals_used,
            commitments_used: value.commitments_used,
            risk_used: value.risk_used,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;

    use crate::services;

    #[test]
    fn now_service_output_maps_to_existing_now_dto_shape() {
        let due_at = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let service_output = services::now::NowOutput {
            computed_at: 1_700_000_100,
            timezone: "America/Denver".to_string(),
            summary: services::now::NowSummaryOutput {
                mode: services::now::NowLabelOutput {
                    key: "day_mode".to_string(),
                    label: "Day".to_string(),
                },
                phase: services::now::NowLabelOutput {
                    key: "engaged".to_string(),
                    label: "Engaged".to_string(),
                },
                meds: services::now::NowLabelOutput {
                    key: "done".to_string(),
                    label: "Done".to_string(),
                },
                risk: services::now::NowRiskSummaryOutput {
                    level: "low".to_string(),
                    score: Some(0.2),
                    label: "low · 20%".to_string(),
                },
            },
            schedule: services::now::NowScheduleOutput {
                empty_message: None,
                next_event: Some(services::now::NowEventOutput {
                    title: "Standup".to_string(),
                    start_ts: 1_700_000_400,
                    end_ts: Some(1_700_000_700),
                    location: Some("Desk".to_string()),
                    prep_minutes: Some(10),
                    travel_minutes: Some(5),
                    leave_by_ts: Some(1_700_000_100),
                }),
                upcoming_events: vec![],
            },
            tasks: services::now::NowTasksOutput {
                todoist: vec![services::now::NowTaskOutput {
                    id: "com_1".to_string(),
                    text: "Ship patch".to_string(),
                    source_type: "todoist".to_string(),
                    due_at: Some(due_at),
                    project: Some("Vel".to_string()),
                    commitment_kind: Some("todo".to_string()),
                }],
                other_open: vec![],
                next_commitment: None,
            },
            attention: services::now::NowAttentionOutput {
                state: services::now::NowLabelOutput {
                    key: "on_task".to_string(),
                    label: "On task".to_string(),
                },
                drift: services::now::NowLabelOutput {
                    key: "none".to_string(),
                    label: "None".to_string(),
                },
                severity: services::now::NowLabelOutput {
                    key: "none".to_string(),
                    label: "None".to_string(),
                },
                confidence: Some(0.92),
                reasons: vec!["Mode: Day".to_string()],
            },
            sources: services::now::NowSourcesOutput {
                git_activity: Some(services::now::NowSourceActivityOutput {
                    label: "Git activity".to_string(),
                    timestamp: 1_700_000_050,
                    summary: json!({"label":"Recent commit"}),
                }),
                health: None,
                mood: None,
                pain: None,
                note_document: None,
                assistant_message: None,
            },
            freshness: services::now::NowFreshnessOutput {
                overall_status: "fresh".to_string(),
                sources: vec![services::now::NowFreshnessEntryOutput {
                    key: "context".to_string(),
                    label: "Context".to_string(),
                    status: "fresh".to_string(),
                    last_sync_at: Some(1_700_000_090),
                    age_seconds: Some(10),
                    guidance: None,
                }],
            },
            action_items: vec![vel_core::ActionItem {
                id: vel_core::ActionItemId::from("act_1".to_string()),
                surface: vel_core::ActionSurface::Now,
                kind: vel_core::ActionKind::NextStep,
                title: "Ship patch".to_string(),
                summary: "Due soon".to_string(),
                project_id: None,
                state: vel_core::ActionState::Active,
                rank: 70,
                surfaced_at: due_at,
                snoozed_until: None,
                evidence: vec![vel_core::ActionEvidenceRef {
                    source_kind: "commitment".to_string(),
                    source_id: "com_1".to_string(),
                    label: "Ship patch".to_string(),
                    detail: None,
                }],
            }],
            review_snapshot: vel_core::ReviewSnapshot {
                open_action_count: 1,
                triage_count: 0,
                projects_needing_review: 0,
            },
            reasons: vec!["Mode: Day".to_string()],
            debug: services::now::NowDebugOutput {
                raw_context: json!({"mode":"day_mode"}),
                signals_used: vec!["sig_1".to_string()],
                commitments_used: vec!["com_1".to_string()],
                risk_used: vec!["risk_1".to_string()],
            },
        };

        let dto: vel_api_types::NowData = service_output.into();
        let json = serde_json::to_value(dto).unwrap();

        assert_eq!(json["timezone"], "America/Denver");
        assert_eq!(json["summary"]["risk"]["label"], "low · 20%");
        assert_eq!(
            json["tasks"]["todoist"][0]["due_at"],
            "2023-11-14T22:13:20Z"
        );
        assert_eq!(
            json["sources"]["git_activity"]["summary"]["label"],
            "Recent commit"
        );
        assert_eq!(json["freshness"]["sources"][0]["key"], "context");
        assert_eq!(json["action_items"][0]["rank"], 70);
        assert_eq!(json["review_snapshot"]["open_action_count"], 1);
    }
}
