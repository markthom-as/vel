use serde_json::{json, Value};
use time::OffsetDateTime;
use vel_api_types::{
    AssistantContextData, CommitmentData, NowData, PersonRecordData, ProjectRecordData,
    RecallContextData,
};
use vel_core::{
    CommitmentStatus, DailyLoopPhase, RetrievalStrategy, SemanticQuery, SemanticQueryFilters,
};
use vel_llm::ToolSpec;

use crate::services::chat::thread_continuation::{
    parse_thread_metadata, proposal_thread_lifecycle_stage, thread_continuation_data,
};
use crate::{
    errors::AppError,
    services::{agent_grounding, daily_loop, people, projects, retrieval, timezone},
    state::AppState,
};

const TOOL_GET_NOW: &str = "vel_get_now";
const TOOL_SEARCH_MEMORY: &str = "vel_search_memory";
const TOOL_GET_RECALL_CONTEXT: &str = "vel_get_recall_context";
const TOOL_LIST_PROJECTS: &str = "vel_list_projects";
const TOOL_LIST_PEOPLE: &str = "vel_list_people";
const TOOL_LIST_OPEN_COMMITMENTS: &str = "vel_list_open_commitments";
const TOOL_GET_DAILY_LOOP_STATUS: &str = "vel_get_daily_loop_status";
const TOOL_LIST_THREADS: &str = "vel_list_threads";
const TOOL_GET_CONTEXT_BRIEF: &str = "vel_get_context_brief";

pub(crate) fn chat_tool_specs() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: TOOL_GET_NOW.to_string(),
            description:
                "Read Vel's current operator-facing Now summary, including context, action items, and trust/readiness."
                    .to_string(),
            schema: json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_SEARCH_MEMORY.to_string(),
            description:
                "Search Vel's local semantic memory across captures, notes, projects, people, threads, and transcripts."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["query"],
                "properties": {
                    "query": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Natural-language search query for Vel memory."
                    },
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 8,
                        "description": "Maximum number of hits to return."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_GET_RECALL_CONTEXT.to_string(),
            description:
                "Build a bounded assistant context pack from Vel's local semantic memory with summary, focus lines, hit counts, source breakdown, scores, and provenance."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["query"],
                "properties": {
                    "query": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Natural-language recall query for Vel context assembly."
                    },
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 8,
                        "description": "Maximum number of recall hits to return."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_LIST_PROJECTS.to_string(),
            description: "List Vel projects with names, ids, family, and status.".to_string(),
            schema: json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 20
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_LIST_PEOPLE.to_string(),
            description: "List known people records in Vel.".to_string(),
            schema: json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 20
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_LIST_OPEN_COMMITMENTS.to_string(),
            description:
                "List open commitments and todos currently tracked by Vel.".to_string(),
            schema: json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 20
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_GET_DAILY_LOOP_STATUS.to_string(),
            description:
                "Read the active morning-overview and standup daily-loop state for the current local day."
                    .to_string(),
            schema: json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_LIST_THREADS.to_string(),
            description:
                "List Vel threads for continuity, search, and resolution context. Supports filtering by status and thread type."
                    .to_string(),
            schema: json!({
                "type": "object",
                "properties": {
                    "status": {
                        "type": "string",
                        "description": "Optional thread status filter such as open or planned."
                    },
                    "thread_type": {
                        "type": "string",
                        "description": "Optional thread type filter such as project_review, reflow_edit, or planning_execution."
                    },
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 20
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_GET_CONTEXT_BRIEF.to_string(),
            description:
                "Read one of Vel's compact context briefs for today, morning, or end_of_day."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["kind"],
                "properties": {
                    "kind": {
                        "type": "string",
                        "enum": ["today", "morning", "end_of_day"]
                    }
                },
                "additionalProperties": false,
            }),
        },
    ]
}

pub(crate) async fn execute_chat_tool(
    state: &AppState,
    name: &str,
    arguments: &Value,
) -> Result<Value, AppError> {
    match name {
        TOOL_GET_NOW => {
            let now: NowData = crate::services::now::get_now(&state.storage, &state.config)
                .await?
                .into();
            Ok(json!({ "now": now }))
        }
        TOOL_SEARCH_MEMORY => {
            let query = arguments
                .get("query")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    AppError::bad_request("vel_search_memory requires non-empty query")
                })?;
            let limit = parse_limit(arguments, 5, 8)?;
            let hits = retrieval::semantic_query(
                state,
                &SemanticQuery {
                    query_text: query.to_string(),
                    top_k: limit as u32,
                    strategy: RetrievalStrategy::Hybrid,
                    include_provenance: true,
                    filters: SemanticQueryFilters {
                        source_kinds: retrieval::context_source_kinds(),
                        ..Default::default()
                    },
                    policy: None,
                },
            )
            .await?;
            Ok(json!({
                "query": query,
                "hits": hits,
            }))
        }
        TOOL_GET_RECALL_CONTEXT => {
            let query = arguments
                .get("query")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    AppError::bad_request("vel_get_recall_context requires non-empty query")
                })?;
            let limit = parse_limit(arguments, 5, 8)?;
            let assistant_context = build_assistant_context(state, query, limit).await?;
            Ok(json!({
                "assistant_context": assistant_context,
                "recall": assistant_context.recall,
            }))
        }
        TOOL_LIST_PROJECTS => {
            let limit = parse_limit(arguments, 8, 20)?;
            let projects = projects::list_projects(state)
                .await?
                .into_iter()
                .take(limit)
                .map(ProjectRecordData::from)
                .collect::<Vec<_>>();
            Ok(json!({ "projects": projects }))
        }
        TOOL_LIST_PEOPLE => {
            let limit = parse_limit(arguments, 8, 20)?;
            let people = people::list_people(state)
                .await?
                .into_iter()
                .take(limit)
                .map(PersonRecordData::from)
                .collect::<Vec<_>>();
            Ok(json!({ "people": people }))
        }
        TOOL_LIST_OPEN_COMMITMENTS => {
            let limit = parse_limit(arguments, 8, 20)?;
            let commitments = state
                .storage
                .list_commitments(Some(CommitmentStatus::Open), None, None, limit as u32)
                .await?
                .into_iter()
                .map(CommitmentData::from)
                .collect::<Vec<_>>();
            Ok(json!({ "commitments": commitments }))
        }
        TOOL_GET_DAILY_LOOP_STATUS => {
            let timezone = timezone::resolve_timezone(&state.storage).await?;
            let session_date =
                timezone::current_day_date_string(&timezone, OffsetDateTime::now_utc())?;
            let morning = daily_loop::get_active_session(
                &state.storage,
                &session_date,
                DailyLoopPhase::MorningOverview,
            )
            .await?
            .map(vel_api_types::DailyLoopSessionData::from);
            let standup = daily_loop::get_active_session(
                &state.storage,
                &session_date,
                DailyLoopPhase::Standup,
            )
            .await?
            .map(vel_api_types::DailyLoopSessionData::from);
            let check_in =
                crate::services::check_in::get_current_check_in(&state.storage, &timezone)
                    .await?
                    .map(vel_api_types::CheckInCardData::from);
            Ok(json!({
                "session_date": session_date,
                "timezone": timezone.name,
                "morning_overview": morning,
                "standup": standup,
                "check_in": check_in,
            }))
        }
        TOOL_LIST_THREADS => {
            let limit = parse_limit(arguments, 8, 20)?;
            let status = arguments
                .get("status")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty());
            let thread_type = arguments
                .get("thread_type")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty());
            let mut threads = Vec::new();
            for (id, thread_type, title, status, created_at, updated_at) in state
                .storage
                .list_threads(status, limit as u32)
                .await?
                .into_iter()
                .filter(|(_, current_thread_type, _, _, _, _)| {
                    thread_type.is_none_or(|expected| current_thread_type == expected)
                })
            {
                let metadata = state.storage.get_thread_by_id(&id).await?.and_then(
                    |(_, _, _, _, metadata_json, _, _)| parse_thread_metadata(&metadata_json),
                );
                threads.push(json!({
                    "id": id,
                    "thread_type": thread_type,
                    "title": title,
                    "status": status,
                    "lifecycle_stage": proposal_thread_lifecycle_stage(&thread_type, metadata.as_ref()),
                    "continuation": thread_continuation_data(&thread_type, metadata.as_ref()),
                    "created_at": created_at,
                    "updated_at": updated_at,
                }));
            }
            Ok(json!({ "threads": threads }))
        }
        TOOL_GET_CONTEXT_BRIEF => {
            let kind = arguments
                .get("kind")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| AppError::bad_request("vel_get_context_brief requires kind"))?;
            match kind {
                "today" => {
                    let output = crate::services::context_runs::generate_today(state).await?;
                    Ok(json!({ "kind": "today", "brief": output.data }))
                }
                "morning" => {
                    let output = crate::services::context_runs::generate_morning(state).await?;
                    Ok(json!({ "kind": "morning", "brief": output.data }))
                }
                "end_of_day" => {
                    let output = crate::services::context_runs::generate_end_of_day(state).await?;
                    Ok(json!({ "kind": "end_of_day", "brief": output.data }))
                }
                _ => Err(AppError::bad_request(
                    "vel_get_context_brief kind must be today, morning, or end_of_day",
                )),
            }
        }
        _ => Err(AppError::bad_request("unsupported chat tool")),
    }
}

pub(crate) async fn build_chat_grounding_prompt(state: &AppState) -> Result<String, AppError> {
    let inspect = agent_grounding::build_agent_inspect(state).await?;
    let grounding = agent_grounding::render_agent_grounding_markdown(&inspect);
    Ok(format!(
        "You are Vel, a concise assistant for capture, recall, daily orientation, and supervised execution.\n\
         Use the provided Vel tool surface when the user asks about their current state, projects, people, commitments, or prior captured knowledge.\n\
         Prefer the recall-context tool for memory-backed questions when you need a bounded pack of relevant context instead of a raw search list.\n\
         Daily-loop state, standup continuity, closeout briefs, and thread-based resolution should stay aligned with Vel's existing product lanes rather than invented ad hoc in chat.\n\
         Never invent access you do not have. If a write, mutation, or review-gated action is requested, explain the limit instead of pretending it happened.\n\
         Prefer direct, practical answers grounded in persisted Vel data.\n\n\
         Grounding snapshot (summary-first; call tools for fresher or deeper detail):\n\n{}",
        grounding
    ))
}

pub(crate) async fn build_assistant_context(
    state: &AppState,
    query: &str,
    limit: usize,
) -> Result<AssistantContextData, AppError> {
    let hits = retrieval::semantic_query(
        state,
        &SemanticQuery {
            query_text: query.to_string(),
            top_k: limit as u32,
            strategy: RetrievalStrategy::Hybrid,
            include_provenance: true,
            filters: SemanticQueryFilters {
                source_kinds: retrieval::context_source_kinds(),
                ..Default::default()
            },
            policy: None,
        },
    )
    .await?;
    let recall = RecallContextData::from(retrieval::build_recall_context_pack(query, hits));
    let commitments = state
        .storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 5)
        .await?
        .into_iter()
        .map(CommitmentData::from)
        .collect::<Vec<_>>();
    let inspect = agent_grounding::build_agent_inspect(state).await?;
    let grounding_hint = agent_grounding::assistant_grounding_hint(&inspect);
    let summary = if recall.hit_count == 0 {
        format!(
            "No directly relevant recalled context was found. {} open commitment{} with canonical scheduler rules remain available. {}",
            commitments.len(),
            if commitments.len() == 1 { "" } else { "s" },
            grounding_hint
        )
    } else {
        format!(
            "Found {} relevant recalled item{} across {}. {} open commitment{} with canonical scheduler rules remain available. {}",
            recall.hit_count,
            if recall.hit_count == 1 { "" } else { "s" },
            describe_source_counts(&recall),
            commitments.len(),
            if commitments.len() == 1 { "" } else { "s" },
            grounding_hint
        )
    };

    Ok(AssistantContextData {
        query_text: query.to_string(),
        summary,
        focus_lines: assistant_focus_lines(&recall, &commitments),
        commitments,
        recall,
    })
}

fn describe_source_counts(recall: &RecallContextData) -> String {
    if recall.source_counts.is_empty() {
        return "no source groups".to_string();
    }

    recall
        .source_counts
        .iter()
        .map(|entry| format!("{:?} ({})", entry.source_kind, entry.count).to_lowercase())
        .collect::<Vec<_>>()
        .join(", ")
}

fn assistant_focus_lines(
    recall: &RecallContextData,
    commitments: &[CommitmentData],
) -> Vec<String> {
    let mut lines = recall
        .hits
        .iter()
        .take(3)
        .map(|hit| {
            let source_ref = hit
                .provenance
                .note_path
                .clone()
                .or_else(|| hit.provenance.project_id.clone())
                .or_else(|| hit.provenance.person_id.clone())
                .or_else(|| hit.provenance.thread_id.clone())
                .or_else(|| hit.provenance.capture_id.clone())
                .unwrap_or_else(|| hit.source_id.clone());
            format!("{:?} {}: {}", hit.source_kind, source_ref, hit.snippet).to_lowercase()
        })
        .collect::<Vec<_>>();

    lines.extend(commitments.iter().take(2).map(|commitment| {
        let mut parts = Vec::new();
        if let Some(block) = &commitment.scheduler_rules.block_target {
            parts.push(format!("block:{block}"));
        }
        if let Some(duration) = commitment.scheduler_rules.duration_minutes {
            parts.push(format!("{duration}m"));
        }
        if let Some(window) = &commitment.scheduler_rules.time_window {
            parts.push(format!("time:{window:?}").to_lowercase());
        }
        if commitment.scheduler_rules.calendar_free {
            parts.push("cal:free".to_string());
        }
        if commitment.scheduler_rules.local_urgency {
            parts.push("urgent".to_string());
        }
        if commitment.scheduler_rules.local_defer {
            parts.push("defer".to_string());
        }
        let suffix = if parts.is_empty() {
            "no explicit scheduler facets".to_string()
        } else {
            parts.join(", ")
        };
        format!("commitment {}: {}", commitment.text, suffix).to_lowercase()
    }));

    lines
}

fn parse_limit(
    arguments: &Value,
    default_limit: usize,
    max_limit: usize,
) -> Result<usize, AppError> {
    let Some(raw_limit) = arguments.get("limit") else {
        return Ok(default_limit);
    };
    let limit = raw_limit
        .as_u64()
        .ok_or_else(|| AppError::bad_request("tool limit must be an integer"))?
        as usize;
    if limit == 0 {
        return Err(AppError::bad_request(
            "tool limit must be greater than zero",
        ));
    }
    Ok(limit.min(max_limit))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{
        DailyLoopStartMetadata, DailyLoopStartRequest, DailyLoopStartSource, DailyLoopSurface,
    };

    fn test_state(storage: vel_storage::Storage) -> AppState {
        let (broadcast_tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    #[tokio::test]
    async fn daily_loop_status_tool_reports_active_standup_and_check_in() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting("timezone", &serde_json::json!("America/Denver"))
            .await
            .unwrap();

        let state = test_state(storage);
        let session_date = crate::services::timezone::current_day_date_string(
            &crate::services::timezone::ResolvedTimeZone::parse("America/Denver").unwrap(),
            OffsetDateTime::now_utc(),
        )
        .unwrap();
        crate::services::daily_loop::start_session(
            &state.storage,
            &state.config,
            DailyLoopStartRequest {
                session_date,
                phase: DailyLoopPhase::Standup,
                start: DailyLoopStartMetadata {
                    source: DailyLoopStartSource::Manual,
                    surface: DailyLoopSurface::Web,
                },
            },
        )
        .await
        .unwrap();

        let value = execute_chat_tool(&state, TOOL_GET_DAILY_LOOP_STATUS, &json!({}))
            .await
            .unwrap();

        assert_eq!(value["standup"]["phase"], "standup");
        assert_eq!(value["check_in"]["phase"], "standup");
        assert_eq!(value["check_in"]["escalation"]["target"], "threads");
    }

    #[tokio::test]
    async fn list_threads_tool_applies_thread_type_filter() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_thread(
                "thr_review",
                "project_review",
                "Review thread",
                "open",
                "{}",
            )
            .await
            .unwrap();
        storage
            .insert_thread("thr_reflow", "reflow_edit", "Reflow edit", "open", "{}")
            .await
            .unwrap();
        storage
            .update_thread_metadata(
                "thr_reflow",
                &json!({
                    "source": "reflow",
                    "trigger": "missed_event",
                    "severity": "critical",
                    "summary": "Missed event needs bounded manual shaping.",
                    "proposal_state": "staged",
                    "preview_lines": ["Missed: focus block at 10:00."]
                })
                .to_string(),
            )
            .await
            .unwrap();

        let state = test_state(storage);
        let value = execute_chat_tool(
            &state,
            TOOL_LIST_THREADS,
            &json!({
                "status": "open",
                "thread_type": "reflow_edit",
                "limit": 10,
            }),
        )
        .await
        .unwrap();

        assert_eq!(value["threads"].as_array().unwrap().len(), 1);
        assert_eq!(value["threads"][0]["id"], "thr_reflow");
        assert_eq!(value["threads"][0]["thread_type"], "reflow_edit");
        assert_eq!(value["threads"][0]["lifecycle_stage"], "staged");
        assert_eq!(
            value["threads"][0]["continuation"]["bounded_capability_state"],
            "schedule_review_gated"
        );
    }

    #[tokio::test]
    async fn recall_context_tool_returns_typed_pack() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        storage
            .upsert_note_semantic_record(
                "projects/tax/accountant.md",
                "Accountant follow up",
                "Need accountant follow up on quarterly estimate this week.",
                "cap_note_recall",
                now,
            )
            .await
            .unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Deep work @30m".to_string(),
                source_type: "todoist".to_string(),
                source_id: "task_1".to_string(),
                status: CommitmentStatus::Open,
                due_at: None,
                project: Some("tax".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(json!({
                    "labels": ["block:focus", "@cal:free", "time:prenoon", "@urgent"]
                })),
            })
            .await
            .unwrap();

        let state = test_state(storage);
        let value = execute_chat_tool(
            &state,
            TOOL_GET_RECALL_CONTEXT,
            &json!({
                "query": "accountant follow up",
                "limit": 5,
            }),
        )
        .await
        .unwrap();

        assert_eq!(
            value["assistant_context"]["query_text"],
            "accountant follow up"
        );
        assert!(value["assistant_context"]["summary"]
            .as_str()
            .unwrap()
            .contains("recalled"));
        assert_eq!(
            value["assistant_context"]["commitments"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            value["assistant_context"]["commitments"][0]["scheduler_rules"]["block_target"],
            "focus"
        );
        assert!(value["assistant_context"]["focus_lines"][0]
            .as_str()
            .unwrap()
            .contains("accountant"));
        assert!(value["assistant_context"]["focus_lines"]
            .as_array()
            .unwrap()
            .iter()
            .any(|line| line.as_str().unwrap().contains("block:focus")));
        assert_eq!(value["recall"]["query_text"], "accountant follow up");
        assert_eq!(value["recall"]["hit_count"], 1);
        assert_eq!(value["recall"]["source_counts"][0]["source_kind"], "note");
        assert!(
            value["recall"]["hits"][0]["combined_score"]
                .as_f64()
                .unwrap()
                > 0.0
        );
        assert_eq!(
            value["recall"]["hits"][0]["provenance"]["note_path"],
            "projects/tax/accountant.md"
        );
    }

    #[tokio::test]
    async fn context_brief_tool_returns_end_of_day_summary() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_capture(vel_storage::CaptureInsert {
                capture_type: "quick_note".to_string(),
                content_text: "remember to follow up with the accountant tomorrow".to_string(),
                source_device: Some("test-device".to_string()),
                privacy_class: vel_core::PrivacyClass::Private,
            })
            .await
            .unwrap();

        let state = test_state(storage);
        let value = execute_chat_tool(
            &state,
            TOOL_GET_CONTEXT_BRIEF,
            &json!({ "kind": "end_of_day" }),
        )
        .await
        .unwrap();

        assert_eq!(value["kind"], "end_of_day");
        assert!(value["brief"]["what_remains_open"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item.as_str().unwrap().contains("follow up")));
    }
}
