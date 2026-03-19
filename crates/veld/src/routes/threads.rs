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

use crate::{errors::AppError, state::AppState};

#[derive(Debug, serde::Deserialize)]
pub struct ListThreadsQuery {
    pub status: Option<String>,
    pub limit: Option<u32>,
    pub thread_type: Option<String>,
    pub project_id: Option<String>,
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

fn thread_data_from_summary(
    id: String,
    thread_type: String,
    title: String,
    status: String,
    created_at: i64,
    updated_at: i64,
) -> ThreadData {
    let (planning_kind, lifecycle_stage) = planning_thread_fields(&thread_type, &status);
    ThreadData {
        id,
        thread_type,
        title,
        status,
        planning_kind,
        lifecycle_stage,
        created_at,
        updated_at,
        links: None,
    }
}

fn thread_data_from_row(
    row: (String, String, String, String, String, i64, i64),
    links: Option<Vec<ThreadLinkData>>,
) -> ThreadData {
    let (id, thread_type, title, status, _metadata_json, created_at, updated_at) = row;
    let (planning_kind, lifecycle_stage) = planning_thread_fields(&thread_type, &status);
    ThreadData {
        id,
        thread_type,
        title,
        status,
        planning_kind,
        lifecycle_stage,
        created_at,
        updated_at,
        links,
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
    let data: Vec<ThreadData> = if q.thread_type.is_none() && q.project_id.is_none() {
        rows.into_iter()
            .map(|(id, thread_type, title, status, created_at, updated_at)| {
                thread_data_from_summary(id, thread_type, title, status, created_at, updated_at)
            })
            .collect()
    } else {
        let mut data = Vec::new();
        for (id, thread_type, title, status, created_at, updated_at) in rows {
            if !matches_thread_filters(&state, &id, &thread_type, q.project_id.as_deref()).await? {
                continue;
            }
            if let Some(expected_thread_type) = q.thread_type.as_deref() {
                if thread_type != expected_thread_type {
                    continue;
                }
            }
            data.push(thread_data_from_summary(
                id,
                thread_type,
                title,
                status,
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

#[cfg(test)]
mod tests {
    use super::*;
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
                "thr_project_review_1",
                "project_review",
                "Vel review",
                "open",
                r#"{"project_id":"proj_vel"}"#,
            )
            .await
            .unwrap();
        storage
            .insert_thread_link("thr_project_review_1", "project", "proj_vel", "about")
            .await
            .unwrap();
        storage
            .insert_thread(
                "thr_project_review_2",
                "project_review",
                "Other review",
                "open",
                r#"{"project_id":"proj_other"}"#,
            )
            .await
            .unwrap();
        storage
            .insert_thread_link("thr_project_review_2", "project", "proj_other", "about")
            .await
            .unwrap();

        let state = test_state(storage);
        let Json(response) = list_threads(
            State(state),
            Query(ListThreadsQuery {
                status: Some("open".to_string()),
                limit: Some(20),
                thread_type: Some("project_review".to_string()),
                project_id: Some("proj_vel".to_string()),
            }),
        )
        .await
        .unwrap();

        let data = response.data.expect("thread data");
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].id, "thr_project_review_1");
    }
}

pub async fn get_thread(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ThreadData>>, AppError> {
    let row = state.storage.get_thread_by_id(id.trim()).await?;
    let (id, thread_type, title, status, _metadata_json, created_at, updated_at) =
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
            String::new(),
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
        now,
        now,
    );
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
    let (id, thread_type, title, status, _metadata_json, created_at, updated_at) = row;
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
            String::new(),
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
