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
    services::{
        agent_grounding, daily_loop, integrations_todoist, operator_settings, people, projects,
        retrieval, timezone, writeback,
    },
    state::AppState,
};

const TOOL_GET_NOW: &str = "vel_get_now";
const TOOL_LIST_CALENDAR_EVENTS: &str = "vel_list_calendar_events";
const TOOL_GET_CALENDAR_EVENT: &str = "vel_get_calendar_event";
const TOOL_LIST_TASKS: &str = "vel_list_tasks";
const TOOL_GET_TASK: &str = "vel_get_task";
const TOOL_CREATE_TODOIST_TASK: &str = "vel_create_todoist_task";
const TOOL_UPDATE_TODOIST_TASK: &str = "vel_update_todoist_task";
const TOOL_COMPLETE_TODOIST_TASK: &str = "vel_complete_todoist_task";
const TOOL_REOPEN_TODOIST_TASK: &str = "vel_reopen_todoist_task";
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
            name: TOOL_LIST_CALENDAR_EVENTS.to_string(),
            description:
                "List the calendar events currently surfaced by Vel's Now schedule, with optional inclusion of following-day events."
                    .to_string(),
            schema: json!({
                "type": "object",
                "properties": {
                    "include_following_day": {
                        "type": "boolean",
                        "description": "Whether to include following-day events from the current Now schedule."
                    },
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 20,
                        "description": "Maximum number of events to return."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_GET_CALENDAR_EVENT.to_string(),
            description:
                "Read one surfaced calendar event from Vel's current Now schedule by event id."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["event_id"],
                "properties": {
                    "event_id": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Calendar event id from Vel's surfaced Now schedule."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_LIST_TASKS.to_string(),
            description:
                "List Vel tasks and commitments with optional status, source, and project filtering."
                    .to_string(),
            schema: json!({
                "type": "object",
                "properties": {
                    "status": {
                        "type": "string",
                        "enum": ["open", "done", "all"],
                        "description": "Task status filter. Defaults to open."
                    },
                    "source_type": {
                        "type": "string",
                        "description": "Optional source filter such as todoist."
                    },
                    "project": {
                        "type": "string",
                        "description": "Optional project filter."
                    },
                    "limit": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 20,
                        "description": "Maximum number of tasks to return."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_GET_TASK.to_string(),
            description:
                "Read one Vel task or commitment by its canonical task id."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["task_id"],
                "properties": {
                    "task_id": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Canonical Vel task/commitment id."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_CREATE_TODOIST_TASK.to_string(),
            description:
                "Create a Todoist-backed task through Vel's bounded writeback lane when SAFE MODE is disabled."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["content"],
                "properties": {
                    "content": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Todoist task content."
                    },
                    "project_id": {
                        "type": "string",
                        "description": "Optional canonical Vel project id."
                    },
                    "scheduled_for": {
                        "type": "string",
                        "description": "Optional due date or datetime string."
                    },
                    "priority": {
                        "description": "Optional Todoist priority, as 1-4 or strings like p1/high/low.",
                        "oneOf": [
                            { "type": "integer", "minimum": 1, "maximum": 4 },
                            { "type": "string", "minLength": 1 }
                        ]
                    },
                    "waiting_on": {
                        "type": "string",
                        "description": "Optional waiting_on label value."
                    },
                    "review_state": {
                        "type": "string",
                        "description": "Optional review_state label value."
                    },
                    "tags": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Optional Todoist tags. An empty array clears tags if tag writeback is enabled."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_UPDATE_TODOIST_TASK.to_string(),
            description:
                "Update one Todoist-backed Vel task by canonical task id through Vel's bounded writeback lane."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["task_id"],
                "properties": {
                    "task_id": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Canonical Vel task id for a Todoist-backed commitment."
                    },
                    "content": {
                        "type": "string",
                        "description": "Updated Todoist task content."
                    },
                    "project_id": {
                        "type": "string",
                        "description": "Optional canonical Vel project id."
                    },
                    "scheduled_for": {
                        "type": "string",
                        "description": "Optional due date or datetime string."
                    },
                    "priority": {
                        "description": "Optional Todoist priority, as 1-4 or strings like p1/high/low.",
                        "oneOf": [
                            { "type": "integer", "minimum": 1, "maximum": 4 },
                            { "type": "string", "minLength": 1 }
                        ]
                    },
                    "waiting_on": {
                        "type": "string",
                        "description": "Optional waiting_on label value."
                    },
                    "review_state": {
                        "type": "string",
                        "description": "Optional review_state label value."
                    },
                    "tags": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Optional Todoist tags. An empty array clears tags if tag writeback is enabled."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_COMPLETE_TODOIST_TASK.to_string(),
            description:
                "Complete one Todoist-backed Vel task by canonical task id through Vel's bounded writeback lane."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["task_id"],
                "properties": {
                    "task_id": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Canonical Vel task id for a Todoist-backed commitment."
                    }
                },
                "additionalProperties": false,
            }),
        },
        ToolSpec {
            name: TOOL_REOPEN_TODOIST_TASK.to_string(),
            description:
                "Reopen one Todoist-backed Vel task by canonical task id through Vel's bounded writeback lane."
                    .to_string(),
            schema: json!({
                "type": "object",
                "required": ["task_id"],
                "properties": {
                    "task_id": {
                        "type": "string",
                        "minLength": 1,
                        "description": "Canonical Vel task id for a Todoist-backed commitment."
                    }
                },
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
                        "description": "Optional thread type filter such as reflow_edit or planning_execution."
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
        TOOL_LIST_CALENDAR_EVENTS => {
            let now: NowData = crate::services::now::get_now(&state.storage, &state.config)
                .await?
                .into();
            let limit = parse_limit(arguments, 8, 20)?;
            let include_following_day =
                parse_bool_argument(arguments, "include_following_day")?.unwrap_or(true);
            let mut events = now.schedule.upcoming_events.clone();
            if include_following_day {
                events.extend(now.schedule.following_day_events.clone());
            }
            events.truncate(limit);
            Ok(json!({
                "computed_at": now.computed_at,
                "timezone": now.timezone,
                "events": events,
                "include_following_day": include_following_day,
            }))
        }
        TOOL_GET_CALENDAR_EVENT => {
            let event_id =
                required_trimmed_argument(arguments, "event_id", TOOL_GET_CALENDAR_EVENT)?;
            let now: NowData = crate::services::now::get_now(&state.storage, &state.config)
                .await?
                .into();
            let upcoming = now
                .schedule
                .upcoming_events
                .iter()
                .find(|event| event.event_id.as_deref() == Some(event_id.as_str()))
                .cloned();
            let (event, surfaced_in) = if let Some(event) = upcoming {
                (event, "upcoming_events")
            } else {
                let following_day = now
                    .schedule
                    .following_day_events
                    .iter()
                    .find(|event| event.event_id.as_deref() == Some(event_id.as_str()))
                    .cloned();
                match following_day {
                    Some(event) => (event, "following_day_events"),
                    None => {
                        return Err(AppError::not_found(
                            "calendar event is not surfaced in the current Now schedule",
                        ));
                    }
                }
            };
            Ok(json!({
                "computed_at": now.computed_at,
                "timezone": now.timezone,
                "event": event,
                "surfaced_in": surfaced_in,
            }))
        }
        TOOL_LIST_TASKS => {
            let limit = parse_limit(arguments, 8, 20)?;
            let status = parse_task_status(arguments)?;
            let source_type = optional_trimmed_argument(arguments, "source_type")?;
            let project = optional_trimmed_argument(arguments, "project")?;
            let commitments = state
                .storage
                .list_commitments(status, None, None, 256)
                .await?
                .into_iter()
                .filter(|commitment| {
                    source_type.as_deref().is_none_or(|expected| {
                        commitment.source_type.trim().eq_ignore_ascii_case(expected)
                    })
                })
                .filter(|commitment| {
                    project.as_deref().is_none_or(|expected| {
                        commitment
                            .project
                            .as_deref()
                            .map(|value| value.trim().eq_ignore_ascii_case(expected))
                            .unwrap_or(false)
                    })
                })
                .take(limit)
                .map(CommitmentData::from)
                .collect::<Vec<_>>();
            Ok(json!({
                "tasks": commitments,
                "filters": {
                    "status": task_status_label(status),
                    "source_type": source_type,
                    "project": project,
                }
            }))
        }
        TOOL_GET_TASK => {
            let task_id = required_trimmed_argument(arguments, "task_id", TOOL_GET_TASK)?;
            let task = state
                .storage
                .get_commitment_by_id(&task_id)
                .await?
                .ok_or_else(|| AppError::not_found("task not found"))?;
            let now: NowData = crate::services::now::get_now(&state.storage, &state.config)
                .await?
                .into();
            let surfaced_in_now = now
                .tasks
                .next_commitment
                .as_ref()
                .is_some_and(|next| next.id == task_id)
                || now.tasks.todoist.iter().any(|item| item.id == task_id)
                || now.tasks.other_open.iter().any(|item| item.id == task_id);
            Ok(json!({
                "task": CommitmentData::from(task),
                "surfaced_in_now": surfaced_in_now,
            }))
        }
        TOOL_CREATE_TODOIST_TASK => {
            let mutation = parse_todoist_tool_mutation(
                arguments,
                TOOL_CREATE_TODOIST_TASK,
                TodoistContentRequirement::Required,
            )?;
            if let Some(blocked) =
                todoist_writeback_block_response(state, "todoist_create_task").await?
            {
                return Ok(blocked);
            }
            let operation =
                writeback::todoist_create_task(&state.storage, &state.config, "vel-chat", mutation)
                    .await?;
            Ok(todoist_writeback_result("todoist_create_task", operation))
        }
        TOOL_UPDATE_TODOIST_TASK => {
            let task_id =
                required_trimmed_argument(arguments, "task_id", TOOL_UPDATE_TODOIST_TASK)?;
            let mutation = parse_todoist_tool_mutation(
                arguments,
                TOOL_UPDATE_TODOIST_TASK,
                TodoistContentRequirement::Optional,
            )?;
            if let Some(blocked) =
                todoist_writeback_block_response(state, "todoist_update_task").await?
            {
                return Ok(blocked);
            }
            let operation = writeback::todoist_update_task(
                &state.storage,
                &state.config,
                "vel-chat",
                &task_id,
                mutation,
            )
            .await?;
            Ok(todoist_writeback_result("todoist_update_task", operation))
        }
        TOOL_COMPLETE_TODOIST_TASK => {
            let task_id =
                required_trimmed_argument(arguments, "task_id", TOOL_COMPLETE_TODOIST_TASK)?;
            if let Some(blocked) =
                todoist_writeback_block_response(state, "todoist_complete_task").await?
            {
                return Ok(blocked);
            }
            let operation = writeback::todoist_complete_task(
                &state.storage,
                &state.config,
                "vel-chat",
                &task_id,
            )
            .await?;
            Ok(todoist_writeback_result("todoist_complete_task", operation))
        }
        TOOL_REOPEN_TODOIST_TASK => {
            let task_id =
                required_trimmed_argument(arguments, "task_id", TOOL_REOPEN_TODOIST_TASK)?;
            if let Some(blocked) =
                todoist_writeback_block_response(state, "todoist_reopen_task").await?
            {
                return Ok(blocked);
            }
            let operation =
                writeback::todoist_reopen_task(&state.storage, &state.config, "vel-chat", &task_id)
                    .await?;
            Ok(todoist_writeback_result("todoist_reopen_task", operation))
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
            for (id, thread_type, title, status, metadata_json, created_at, updated_at) in state
                .storage
                .list_threads(status, limit as u32)
                .await?
                .into_iter()
                .filter(|(_, current_thread_type, _, _, _, _, _)| {
                    thread_type.is_none_or(|expected| current_thread_type == expected)
                })
            {
                let metadata = parse_thread_metadata(&metadata_json);
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
         Use the dedicated task and calendar tools for detailed inspection when the user needs concrete commitments or surfaced schedule events instead of only the compact Now summary.\n\
         Todoist task mutation tools are bounded by SAFE MODE and Todoist connectivity; use them only when the user explicitly asks to create, update, complete, or reopen a Todoist-backed task.\n\
         Calendar mutation is not currently available through chat tools; do not pretend a calendar write happened.\n\
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

fn parse_bool_argument(arguments: &Value, key: &str) -> Result<Option<bool>, AppError> {
    match arguments.get(key) {
        Some(value) => value
            .as_bool()
            .map(Some)
            .ok_or_else(|| AppError::bad_request(format!("{key} must be a boolean"))),
        None => Ok(None),
    }
}

fn optional_trimmed_argument(arguments: &Value, key: &str) -> Result<Option<String>, AppError> {
    match arguments.get(key) {
        Some(value) => value
            .as_str()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| Some(value.to_string()))
            .ok_or_else(|| AppError::bad_request(format!("{key} must be a non-empty string"))),
        None => Ok(None),
    }
}

fn required_trimmed_argument(
    arguments: &Value,
    key: &str,
    tool_name: &str,
) -> Result<String, AppError> {
    optional_trimmed_argument(arguments, key)?
        .ok_or_else(|| AppError::bad_request(format!("{tool_name} requires non-empty {key}")))
}

fn parse_task_status(arguments: &Value) -> Result<Option<CommitmentStatus>, AppError> {
    let status = optional_trimmed_argument(arguments, "status")?;
    match status.as_deref() {
        None | Some("open") => Ok(Some(CommitmentStatus::Open)),
        Some("done") => Ok(Some(CommitmentStatus::Done)),
        Some("all") => Ok(None),
        Some(_) => Err(AppError::bad_request(
            "task status must be open, done, or all",
        )),
    }
}

fn task_status_label(status: Option<CommitmentStatus>) -> &'static str {
    match status {
        Some(CommitmentStatus::Open) => "open",
        Some(CommitmentStatus::Done) => "done",
        None => "all",
        Some(_) => "open",
    }
}

#[derive(Clone, Copy)]
enum TodoistContentRequirement {
    Required,
    Optional,
}

async fn todoist_writeback_block_response(
    state: &AppState,
    action: &str,
) -> Result<Option<Value>, AppError> {
    if !operator_settings::runtime_writeback_enabled(&state.storage, &state.config).await? {
        return Ok(Some(todoist_blocked_response(
            action,
            "safe_mode_enabled",
            writeback::SAFE_MODE_MESSAGE,
            true,
        )));
    }

    let settings = integrations_todoist::load_todoist_settings(&state.storage).await?;
    let status = integrations_todoist::todoist_status(&settings);
    if !status.has_api_token {
        return Ok(Some(todoist_blocked_response(
            action,
            "todoist_not_configured",
            "Todoist task mutation requires a configured Todoist API token.",
            false,
        )));
    }

    Ok(None)
}

fn todoist_blocked_response(
    action: &str,
    code: &str,
    message: &str,
    requires_writeback_enabled: bool,
) -> Value {
    json!({
        "provider": "todoist",
        "action": action,
        "outcome": "blocked",
        "requires_writeback_enabled": requires_writeback_enabled,
        "blocked_reason": {
            "code": code,
            "message": message,
        }
    })
}

fn todoist_writeback_result(action: &str, operation: vel_core::WritebackOperationRecord) -> Value {
    json!({
        "provider": "todoist",
        "action": action,
        "outcome": operation.status.to_string(),
        "writeback": operation,
    })
}

fn parse_todoist_tool_mutation(
    arguments: &Value,
    tool_name: &str,
    content_requirement: TodoistContentRequirement,
) -> Result<integrations_todoist::TodoistTaskMutation, AppError> {
    let content = match content_requirement {
        TodoistContentRequirement::Required => {
            Some(required_trimmed_argument(arguments, "content", tool_name)?)
        }
        TodoistContentRequirement::Optional => optional_trimmed_argument(arguments, "content")?,
    };

    Ok(integrations_todoist::TodoistTaskMutation {
        content,
        project_id: optional_trimmed_argument(arguments, "project_id")?,
        scheduled_for: optional_trimmed_argument(arguments, "scheduled_for")?,
        priority: parse_todoist_priority_argument(arguments.get("priority"))?,
        waiting_on: optional_trimmed_argument(arguments, "waiting_on")?,
        review_state: optional_trimmed_argument(arguments, "review_state")?,
        tags: parse_string_list_argument(arguments.get("tags"), "tags")?,
    })
}

fn parse_string_list_argument(
    value: Option<&Value>,
    key: &str,
) -> Result<Option<Vec<String>>, AppError> {
    match value {
        None => Ok(None),
        Some(Value::Null) => Ok(Some(Vec::new())),
        Some(Value::Array(values)) => values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(|value| value.to_string())
                    .ok_or_else(|| {
                        AppError::bad_request(format!(
                            "{key} must be an array of non-empty strings"
                        ))
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Some),
        Some(_) => Err(AppError::bad_request(format!(
            "{key} must be an array of non-empty strings"
        ))),
    }
}

fn parse_todoist_priority_argument(value: Option<&Value>) -> Result<Option<u8>, AppError> {
    let Some(value) = value else {
        return Ok(None);
    };

    match value {
        Value::Number(number) => {
            if let Some(raw) = number.as_u64() {
                return parse_todoist_priority_value(raw as i64);
            }
            if let Some(raw) = number.as_i64() {
                return parse_todoist_priority_value(raw);
            }
            Err(AppError::bad_request(
                "todoist priority must be between 1 and 4",
            ))
        }
        Value::String(raw) => {
            let trimmed = raw.trim().to_ascii_lowercase();
            if trimmed.is_empty() {
                return Ok(None);
            }
            match trimmed.as_str() {
                "critical" | "urgent" | "high" => Ok(Some(1)),
                "medium" => Ok(Some(2)),
                "low" => Ok(Some(3)),
                "lowest" => Ok(Some(4)),
                _ => {
                    let numeric = trimmed.strip_prefix('p').unwrap_or(trimmed.as_str());
                    let parsed = numeric.parse::<i64>().map_err(|_| {
                        AppError::bad_request(format!(
                            "todoist priority string {trimmed} is not recognized"
                        ))
                    })?;
                    parse_todoist_priority_value(parsed)
                }
            }
        }
        other => Err(AppError::bad_request(format!(
            "todoist priority must be a number or string, got {other}"
        ))),
    }
}

fn parse_todoist_priority_value(value: i64) -> Result<Option<u8>, AppError> {
    if (1..=4).contains(&value) {
        Ok(Some(value as u8))
    } else {
        Err(AppError::bad_request(
            "todoist priority must be between 1 and 4",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{
        CurrentContextV1, DailyLoopStartMetadata, DailyLoopStartRequest, DailyLoopStartSource,
        DailyLoopSurface,
    };

    fn test_state(storage: vel_storage::Storage) -> AppState {
        test_state_with_config(storage, AppConfig::default())
    }

    fn test_state_with_config(storage: vel_storage::Storage, config: AppConfig) -> AppState {
        let (broadcast_tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            config,
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
            .insert_thread("thr_reflow", "reflow_edit", "Reflow edit", "open", "{}")
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_resolution",
                "action_resolution",
                "Resolution thread",
                "open",
                "{}",
            )
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
        let reflow = execute_chat_tool(
            &state,
            TOOL_LIST_THREADS,
            &json!({
                "status": "open",
                "thread_type": "action_resolution",
                "limit": 10,
            }),
        )
        .await
        .unwrap();

        assert_eq!(reflow["threads"].as_array().unwrap().len(), 1);
        assert_eq!(reflow["threads"][0]["id"], "thr_resolution");
        assert_eq!(reflow["threads"][0]["thread_type"], "action_resolution");
        assert_eq!(reflow["threads"][0]["title"], "Resolution thread");

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

    #[tokio::test]
    async fn list_tasks_tool_filters_by_source_and_project() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Todoist task".to_string(),
                source_type: "todoist".to_string(),
                source_id: "todo_1".to_string(),
                status: CommitmentStatus::Open,
                due_at: None,
                project: Some("alpha".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(json!({})),
            })
            .await
            .unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Local task".to_string(),
                source_type: "manual".to_string(),
                source_id: "local_1".to_string(),
                status: CommitmentStatus::Open,
                due_at: None,
                project: Some("beta".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(json!({})),
            })
            .await
            .unwrap();

        let state = test_state(storage);
        let value = execute_chat_tool(
            &state,
            TOOL_LIST_TASKS,
            &json!({
                "status": "open",
                "source_type": "todoist",
                "project": "alpha",
                "limit": 10,
            }),
        )
        .await
        .unwrap();

        let tasks = value["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0]["text"], "Todoist task");
        assert_eq!(value["filters"]["status"], "open");
        assert_eq!(value["filters"]["source_type"], "todoist");
        assert_eq!(value["filters"]["project"], "alpha");
    }

    #[tokio::test]
    async fn calendar_tools_return_surfaced_now_schedule_events() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        storage
            .set_current_context(
                now,
                &serde_json::to_string(&CurrentContextV1 {
                    computed_at: now,
                    ..CurrentContextV1::default()
                })
                .unwrap(),
            )
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "calendar".to_string(),
                source_ref: Some("calendar:event_today".to_string()),
                timestamp: now + 1800,
                payload_json: Some(json!({
                    "event_id": "event_today",
                    "calendar_id": "cal_a",
                    "calendar_name": "Primary",
                    "title": "Design review",
                    "start": now + 1800,
                    "end": now + 3600,
                    "location": "Studio",
                    "description": "Review current slice",
                    "attendees": ["annie@example.com"],
                    "prep_minutes": 10,
                    "travel_minutes": 0
                })),
            })
            .await
            .unwrap();

        let state = test_state(storage);
        let listed = execute_chat_tool(
            &state,
            TOOL_LIST_CALENDAR_EVENTS,
            &json!({
                "limit": 10,
                "include_following_day": false,
            }),
        )
        .await
        .unwrap();

        let events = listed["events"].as_array().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0]["title"], "Design review");
        assert_eq!(events[0]["event_id"], "event_today");

        let fetched = execute_chat_tool(
            &state,
            TOOL_GET_CALENDAR_EVENT,
            &json!({ "event_id": "event_today" }),
        )
        .await
        .unwrap();

        assert_eq!(fetched["event"]["title"], "Design review");
        assert_eq!(fetched["surfaced_in"], "upcoming_events");
    }

    #[tokio::test]
    async fn todoist_create_tool_returns_blocked_response_in_safe_mode() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let state = test_state(storage);
        let value = execute_chat_tool(
            &state,
            TOOL_CREATE_TODOIST_TASK,
            &json!({
                "content": "Follow up on agenda",
            }),
        )
        .await
        .unwrap();

        assert_eq!(value["outcome"], "blocked");
        assert_eq!(value["blocked_reason"]["code"], "safe_mode_enabled");
        assert_eq!(value["action"], "todoist_create_task");
    }

    #[tokio::test]
    async fn todoist_create_tool_returns_blocked_response_when_not_configured() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state_with_config(
            storage,
            AppConfig {
                writeback_enabled: true,
                ..AppConfig::default()
            },
        );

        let value = execute_chat_tool(
            &state,
            TOOL_CREATE_TODOIST_TASK,
            &json!({
                "content": "Follow up on agenda",
            }),
        )
        .await
        .unwrap();

        assert_eq!(value["outcome"], "blocked");
        assert_eq!(value["blocked_reason"]["code"], "todoist_not_configured");
        assert_eq!(value["provider"], "todoist");
    }

    #[tokio::test]
    async fn todoist_update_tool_rejects_empty_change_set_before_network() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(
                integrations_todoist::TODOIST_SETTINGS_KEY,
                &json!({
                    "write_capabilities": {
                        "completion_status": true,
                        "due_date": true,
                        "tags": false
                    }
                }),
            )
            .await
            .unwrap();
        storage
            .set_setting(
                integrations_todoist::TODOIST_SECRETS_KEY,
                &json!({
                    "api_token": "test-token"
                }),
            )
            .await
            .unwrap();

        let state = test_state_with_config(
            storage,
            AppConfig {
                writeback_enabled: true,
                ..AppConfig::default()
            },
        );

        let error = execute_chat_tool(
            &state,
            TOOL_UPDATE_TODOIST_TASK,
            &json!({
                "task_id": "commitment_missing_changes",
            }),
        )
        .await
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "todoist_update_task requires at least one changed field"
        );
    }
}
