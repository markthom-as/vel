//! Thread graph: list, inspect, create, update status, link entities.
//! See docs/api/runtime.md.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ThreadCreateRequest, ThreadData, ThreadLinkData, ThreadLinkRequest,
    ThreadUpdateRequest,
};
use vel_core::ProjectId;

use crate::{
    errors::AppError,
    services::chat::thread_continuation::{
        parse_thread_metadata, proposal_thread_lifecycle_stage, thread_category,
        thread_continuation_data,
    },
    state::AppState,
};

#[derive(Debug, serde::Deserialize)]
pub struct ListThreadsQuery {
    pub status: Option<String>,
    pub limit: Option<u32>,
    pub thread_type: Option<String>,
    pub project_id: Option<String>,
    pub continuation_category: Option<String>,
}

fn planning_thread_fields(thread_type: &str, status: &str) -> (Option<String>, Option<String>) {
    let planning_kind = match thread_type {
        "planning_spec" => Some("spec".to_string()),
        "planning_execution" => Some("execution_plan".to_string()),
        "planning_delegation" => Some("delegation_plan".to_string()),
        _ => None,
    };
    let lifecycle_stage = planning_kind.as_ref().map(|_| status.to_string());
    (planning_kind, lifecycle_stage)
}

fn thread_project_details(
    metadata: Option<&serde_json::Value>,
) -> (Option<ProjectId>, Option<String>) {
    let Some(metadata) = metadata else {
        return (None, None);
    };
    let project_id = metadata
        .get("project_id")
        .and_then(serde_json::Value::as_str)
        .map(|value| ProjectId::from(value.to_string()));
    let project_label = metadata
        .get("project_label")
        .or_else(|| metadata.get("project"))
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string);
    (project_id, project_label)
}

fn thread_data_from_summary(
    id: String,
    thread_type: String,
    title: String,
    status: String,
    metadata_json: String,
    created_at: i64,
    updated_at: i64,
) -> ThreadData {
    let metadata = parse_thread_metadata(&metadata_json);
    let (planning_kind, lifecycle_stage) = planning_thread_fields(&thread_type, &status);
    let (project_id, project_label) = thread_project_details(metadata.as_ref());
    ThreadData {
        id,
        thread_type,
        title,
        status,
        planning_kind,
        lifecycle_stage,
        created_at,
        updated_at,
        continuation: None,
        metadata,
        links: None,
        project_id,
        project_label,
    }
}

fn thread_data_from_row(
    row: (String, String, String, String, String, i64, i64),
    links: Option<Vec<ThreadLinkData>>,
) -> ThreadData {
    let (id, thread_type, title, status, metadata_json, created_at, updated_at) = row;
    let metadata = parse_thread_metadata(&metadata_json);
    let continuation = thread_continuation_data(&thread_type, metadata.as_ref());
    let (project_id, project_label) = thread_project_details(metadata.as_ref());
    let (planning_kind, lifecycle_stage) = if thread_type == "assistant_proposal"
        || thread_type == "planning_profile_edit"
        || thread_type == "reflow_edit"
        || thread_type == "day_plan_apply"
    {
        (
            None,
            proposal_thread_lifecycle_stage(&thread_type, metadata.as_ref()),
        )
    } else {
        planning_thread_fields(&thread_type, &status)
    };
    ThreadData {
        id,
        thread_type,
        title,
        status,
        planning_kind,
        lifecycle_stage,
        created_at,
        updated_at,
        continuation,
        metadata,
        links,
        project_id,
        project_label,
    }
}

pub async fn list_threads(
    State(state): State<AppState>,
    Query(q): Query<ListThreadsQuery>,
) -> Result<Json<ApiResponse<Vec<ThreadData>>>, AppError> {
    let limit = q.limit.unwrap_or(50);
    let rows = state
        .storage
        .list_threads(q.status.as_deref(), limit)
        .await?;
    let data: Vec<ThreadData> = if q.thread_type.is_none()
        && q.project_id.is_none()
        && q.continuation_category.is_none()
    {
        rows.into_iter()
            .map(
                |(id, thread_type, title, status, metadata_json, created_at, updated_at)| {
                    thread_data_from_summary(
                        id,
                        thread_type,
                        title,
                        status,
                        metadata_json,
                        created_at,
                        updated_at,
                    )
                },
            )
            .collect()
    } else {
        let mut data = Vec::new();
        for (id, thread_type, title, status, metadata_json, created_at, updated_at) in rows {
            if !matches_thread_filters(&state, &id, &thread_type, q.project_id.as_deref()).await? {
                continue;
            }
            if let Some(expected_thread_type) = q.thread_type.as_deref() {
                if thread_type != expected_thread_type {
                    continue;
                }
            }
            if let Some(expected_category) = q.continuation_category.as_deref() {
                if continuation_category_key(thread_category(&thread_type)) != expected_category {
                    continue;
                }
            }
            data.push(thread_data_from_summary(
                id,
                thread_type,
                title,
                status,
                metadata_json,
                created_at,
                updated_at,
            ));
        }
        data
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

async fn matches_thread_filters(
    state: &AppState,
    thread_id: &str,
    thread_type: &str,
    project_id: Option<&str>,
) -> Result<bool, AppError> {
    let Some(project_id) = project_id.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(true);
    };

    if thread_type.starts_with("project_") {
        let row = state.storage.get_thread_by_id(thread_id).await?;
        if let Some((_, _, _, _, metadata_json, _, _)) = row {
            if serde_json::from_str::<serde_json::Value>(&metadata_json)
                .ok()
                .and_then(|value| {
                    value
                        .get("project_id")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string)
                })
                .as_deref()
                == Some(project_id)
            {
                return Ok(true);
            }
        }
    }

    let links = state.storage.list_thread_links(thread_id).await?;
    Ok(links
        .iter()
        .any(|(_, entity_type, entity_id, _)| entity_type == "project" && entity_id == project_id))
}

fn continuation_category_key(category: vel_api_types::NowHeaderBucketKindData) -> &'static str {
    match category {
        vel_api_types::NowHeaderBucketKindData::ThreadsByType => "threads_by_type",
        vel_api_types::NowHeaderBucketKindData::NeedsInput => "needs_input",
        vel_api_types::NowHeaderBucketKindData::NewNudges => "new_nudges",
        vel_api_types::NowHeaderBucketKindData::SearchFilter => "search_filter",
        vel_api_types::NowHeaderBucketKindData::Snoozed => "snoozed",
        vel_api_types::NowHeaderBucketKindData::ReviewApply => "review_apply",
        vel_api_types::NowHeaderBucketKindData::Reflow => "reflow",
        vel_api_types::NowHeaderBucketKindData::FollowUp => "follow_up",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;

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
    async fn list_threads_filters_by_project_id_and_thread_type() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_thread(
                "thr_action_resolution_1",
                "action_resolution",
                "Vel review",
                "open",
                r#"{"project_id":"proj_vel","project_label":"Vel"}"#,
            )
            .await
            .unwrap();
        storage
            .insert_thread_link("thr_action_resolution_1", "project", "proj_vel", "about")
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_action_resolution_2",
                "action_resolution",
                "Other review",
                "open",
                r#"{"project_id":"proj_other"}"#,
            )
            .await
            .unwrap();
        storage
            .insert_thread_link("thr_action_resolution_2", "project", "proj_other", "about")
            .await
            .unwrap();

        let state = test_state(storage);
        let Json(response) = list_threads(
            State(state),
            Query(ListThreadsQuery {
                status: Some("open".to_string()),
                limit: Some(20),
                thread_type: Some("action_resolution".to_string()),
                project_id: Some("proj_vel".to_string()),
                continuation_category: None,
            }),
        )
        .await
        .unwrap();

        let data = response.data.expect("thread data");
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, "thr_action_resolution_1");
        assert_eq!(
            data[0].project_id.as_ref().map(|id| id.as_ref()),
            Some("proj_vel")
        );
        assert_eq!(data[0].project_label.as_deref(), Some("Vel"));
    }

    #[test]
    fn thread_row_preserves_resolution_metadata_on_detail_reads() {
        let data = thread_data_from_row(
            (
                "thr_action_1".to_string(),
                "action_resolution".to_string(),
                "Follow up".to_string(),
                "deferred".to_string(),
                json!({
                    "source": "check_in",
                    "resolution_state": "deferred",
                    "prompt_id": "standup_prompt_1"
                })
                .to_string(),
                1,
                2,
            ),
            None,
        );

        assert_eq!(
            data.metadata.as_ref().unwrap()["resolution_state"],
            "deferred"
        );
        assert_eq!(data.metadata.as_ref().unwrap()["source"], "check_in");
    }

    #[test]
    fn assistant_proposal_thread_row_uses_proposal_state_for_lifecycle_stage() {
        let data = thread_data_from_row(
            (
                "thr_assistant_1".to_string(),
                "assistant_proposal".to_string(),
                "Send reply".to_string(),
                "resolved".to_string(),
                json!({
                    "source_message_id": "msg_1",
                    "conversation_id": "conv_1",
                    "proposal_state": "applied",
                    "applied_via": "intervention_resolve",
                    "lineage": {
                        "source_message_id": "msg_1",
                        "conversation_id": "conv_1",
                        "action_item_id": "act_1"
                    }
                })
                .to_string(),
                1,
                2,
            ),
            None,
        );

        assert_eq!(data.lifecycle_stage.as_deref(), Some("applied"));
        assert_eq!(
            data.metadata.as_ref().unwrap()["applied_via"],
            "intervention_resolve"
        );
        assert_eq!(
            data.continuation
                .as_ref()
                .expect("continuation")
                .bounded_capability_state,
            "proposal_review_gated"
        );
        assert_eq!(
            data.continuation
                .as_ref()
                .and_then(|value| value.continuation_context.get("source_message_id"))
                .and_then(serde_json::Value::as_str),
            Some("msg_1")
        );
        assert_eq!(
            data.continuation
                .as_ref()
                .map(|value| value.continuation_category),
            Some(vel_api_types::NowHeaderBucketKindData::ReviewApply)
        );
    }

    #[tokio::test]
    async fn list_threads_filters_by_continuation_category() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_thread(
                "thr_reflow_1",
                "reflow_edit",
                "Fix the day",
                "open",
                r#"{"proposal_state":"staged","summary":"Needs repair"}"#,
            )
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_capture_1",
                "raw_capture",
                "Captured note",
                "open",
                r#"{}"#,
            )
            .await
            .unwrap();

        let state = test_state(storage);
        let Json(response) = list_threads(
            State(state),
            Query(ListThreadsQuery {
                status: Some("open".to_string()),
                limit: Some(20),
                thread_type: None,
                project_id: None,
                continuation_category: Some("reflow".to_string()),
            }),
        )
        .await
        .unwrap();

        let data = response.data.expect("thread data");
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, "thr_reflow_1");
    }

    #[test]
    fn planning_profile_edit_thread_row_uses_proposal_state_for_lifecycle_stage() {
        let data = thread_data_from_row(
            (
                "thr_planning_profile_1".to_string(),
                "planning_profile_edit".to_string(),
                "Add shutdown block".to_string(),
                "resolved".to_string(),
                json!({
                "proposal_state": "applied",
                "applied_via": "planning_profile_apply",
                "requires_confirmation": true,
                "lineage": {
                    "source_message_id": "msg_2",
                    "source_surface": "assistant"
                }
                })
                .to_string(),
                1,
                2,
            ),
            None,
        );

        assert_eq!(data.lifecycle_stage.as_deref(), Some("applied"));
        assert_eq!(
            data.metadata.as_ref().unwrap()["applied_via"],
            "planning_profile_apply"
        );
        assert_eq!(
            data.continuation
                .as_ref()
                .expect("continuation")
                .review_requirements[0],
            "Planning-profile edits require explicit approval before the backend saves them."
        );
    }

    #[test]
    fn reflow_edit_thread_row_uses_proposal_state_for_lifecycle_stage() {
        let data = thread_data_from_row(
            (
                "thr_reflow_1".to_string(),
                "reflow_edit".to_string(),
                "Reflow edit".to_string(),
                "resolved".to_string(),
                json!({
                    "source": "reflow",
                    "trigger": "missed_event",
                    "severity": "critical",
                    "summary": "A scheduled event appears to have slipped past without the plan being updated.",
                    "context_computed_at": 123,
                    "preview_lines": ["Next scheduled event started 20 minutes ago."],
                    "proposal_state": "applied",
                    "applied_via": "commitment_scheduling_apply"
                })
                .to_string(),
                1,
                2,
            ),
            None,
        );

        assert_eq!(data.lifecycle_stage.as_deref(), Some("applied"));
        assert_eq!(
            data.metadata.as_ref().unwrap()["applied_via"],
            "commitment_scheduling_apply"
        );
        assert_eq!(
            data.continuation
                .as_ref()
                .expect("continuation")
                .escalation_reason,
            "This reflow needs bounded manual shaping or explicit review in Threads."
        );
        assert_eq!(
            data.continuation
                .as_ref()
                .and_then(|value| value.continuation_context.get("trigger"))
                .and_then(serde_json::Value::as_str),
            Some("missed_event")
        );
    }
}

pub async fn get_thread(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ThreadData>>, AppError> {
    let row = state.storage.get_thread_by_id(id.trim()).await?;
    let (id, thread_type, title, status, metadata_json, created_at, updated_at) =
        row.ok_or_else(|| AppError::not_found("thread not found"))?;
    let links_rows = state.storage.list_thread_links(&id).await?;
    let links: Vec<ThreadLinkData> = links_rows
        .into_iter()
        .map(
            |(link_id, entity_type, entity_id, relation_type)| ThreadLinkData {
                id: link_id,
                entity_type,
                entity_id,
                relation_type,
            },
        )
        .collect();
    let data = thread_data_from_row(
        (
            id.clone(),
            thread_type,
            title,
            status,
            metadata_json,
            created_at,
            updated_at,
        ),
        Some(links),
    );
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn create_thread(
    State(state): State<AppState>,
    Json(payload): Json<ThreadCreateRequest>,
) -> Result<Json<ApiResponse<ThreadData>>, AppError> {
    let id = format!("thr_{}", Uuid::new_v4().simple());
    let metadata = payload
        .metadata_json
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "{}".to_string());
    state
        .storage
        .insert_thread(&id, &payload.thread_type, &payload.title, "open", &metadata)
        .await?;
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let data = thread_data_from_summary(
        id.clone(),
        payload.thread_type,
        payload.title,
        "open".to_string(),
        metadata.clone(),
        now,
        now,
    );
    let data = ThreadData {
        continuation: payload
            .metadata_json
            .as_ref()
            .and_then(|metadata| thread_continuation_data(&data.thread_type, Some(metadata))),
        metadata: payload.metadata_json,
        ..data
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn update_thread(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ThreadUpdateRequest>,
) -> Result<Json<ApiResponse<ThreadData>>, AppError> {
    let id = id.trim();
    if let Some(status) = &payload.status {
        state.storage.update_thread_status(id, status).await?;
    }
    let row = state
        .storage
        .get_thread_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("thread not found"))?;
    let (id, thread_type, title, status, metadata_json, created_at, updated_at) = row;
    let links_rows = state.storage.list_thread_links(&id).await?;
    let links: Vec<ThreadLinkData> = links_rows
        .into_iter()
        .map(
            |(link_id, entity_type, entity_id, relation_type)| ThreadLinkData {
                id: link_id,
                entity_type,
                entity_id,
                relation_type,
            },
        )
        .collect();
    let data = thread_data_from_row(
        (
            id.clone(),
            thread_type,
            title,
            status,
            metadata_json,
            created_at,
            updated_at,
        ),
        Some(links),
    );
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn add_thread_link(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ThreadLinkRequest>,
) -> Result<Json<ApiResponse<ThreadLinkData>>, AppError> {
    let thread_id = id.trim();
    let _ = state
        .storage
        .get_thread_by_id(thread_id)
        .await?
        .ok_or_else(|| AppError::not_found("thread not found"))?;
    let link_id = state
        .storage
        .insert_thread_link(
            thread_id,
            &payload.entity_type,
            &payload.entity_id,
            &payload.relation_type,
        )
        .await?;
    let data = ThreadLinkData {
        id: link_id,
        entity_type: payload.entity_type,
        entity_id: payload.entity_id,
        relation_type: payload.relation_type,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}
