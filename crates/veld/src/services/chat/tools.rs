use serde_json::{json, Value};
use time::OffsetDateTime;
use vel_api_types::{CommitmentData, NowData, PersonRecordData, ProjectRecordData};
use vel_core::{
    CommitmentStatus, DailyLoopPhase, RetrievalStrategy, SemanticQuery, SemanticQueryFilters,
};
use vel_llm::ToolSpec;

use crate::{
    errors::AppError,
    services::{agent_grounding, daily_loop, people, projects, retrieval, timezone},
    state::AppState,
};

const TOOL_GET_NOW: &str = "vel_get_now";
const TOOL_SEARCH_MEMORY: &str = "vel_search_memory";
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
            let session_date = timezone::local_date_string(&timezone, OffsetDateTime::now_utc());
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
            let threads = state
                .storage
                .list_threads(status, limit as u32)
                .await?
                .into_iter()
                .filter(|(_, current_thread_type, _, _, _, _)| {
                    thread_type.is_none_or(|expected| current_thread_type == expected)
                })
                .map(|(id, thread_type, title, status, created_at, updated_at)| {
                    json!({
                        "id": id,
                        "thread_type": thread_type,
                        "title": title,
                        "status": status,
                        "created_at": created_at,
                        "updated_at": updated_at,
                    })
                })
                .collect::<Vec<_>>();
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
         Daily-loop state, standup continuity, closeout briefs, and thread-based resolution should stay aligned with Vel's existing product lanes rather than invented ad hoc in chat.\n\
         Never invent access you do not have. If a write, mutation, or review-gated action is requested, explain the limit instead of pretending it happened.\n\
         Prefer direct, practical answers grounded in persisted Vel data.\n\n\
         Grounding snapshot (summary-first; call tools for fresher or deeper detail):\n\n{}",
        grounding
    ))
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
        let session_date = crate::services::timezone::local_date_string(
            &crate::services::timezone::ResolvedTimeZone::parse("America/Denver").unwrap(),
            OffsetDateTime::now_utc(),
        );
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
