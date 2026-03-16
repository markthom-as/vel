//! Thread graph: list, inspect, create, update status, link entities.
//! See docs/specs/vel-thread-graph-spec.md.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ThreadCreateRequest, ThreadData, ThreadLinkData, ThreadLinkRequest, ThreadUpdateRequest,
};

use crate::{errors::AppError, state::AppState};

#[derive(Debug, serde::Deserialize)]
pub struct ListThreadsQuery {
    pub status: Option<String>,
    pub limit: Option<u32>,
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
    let data: Vec<ThreadData> = rows
        .into_iter()
        .map(|(id, thread_type, title, status, created_at, updated_at)| ThreadData {
            id,
            thread_type,
            title,
            status,
            created_at,
            updated_at,
            links: None,
        })
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_thread(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ThreadData>>, AppError> {
    let row = state.storage.get_thread_by_id(id.trim()).await?;
    let (id, thread_type, title, status, _metadata_json, created_at, updated_at) = row
        .ok_or_else(|| AppError::not_found("thread not found"))?;
    let links_rows = state.storage.list_thread_links(&id).await?;
    let links: Vec<ThreadLinkData> = links_rows
        .into_iter()
        .map(|(link_id, entity_type, entity_id, relation_type)| ThreadLinkData {
            id: link_id,
            entity_type,
            entity_id,
            relation_type,
        })
        .collect();
    let data = ThreadData {
        id: id.clone(),
        thread_type,
        title,
        status,
        created_at,
        updated_at,
        links: Some(links),
    };
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
        .insert_thread(
            &id,
            &payload.thread_type,
            &payload.title,
            "open",
            &metadata,
        )
        .await?;
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let data = ThreadData {
        id: id.clone(),
        thread_type: payload.thread_type,
        title: payload.title,
        status: "open".to_string(),
        created_at: now,
        updated_at: now,
        links: Some(vec![]),
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
    let row = state.storage.get_thread_by_id(id).await?.ok_or_else(|| AppError::not_found("thread not found"))?;
    let (id, thread_type, title, status, _metadata_json, created_at, updated_at) = row;
    let links_rows = state.storage.list_thread_links(&id).await?;
    let links: Vec<ThreadLinkData> = links_rows
        .into_iter()
        .map(|(link_id, entity_type, entity_id, relation_type)| ThreadLinkData {
            id: link_id,
            entity_type,
            entity_id,
            relation_type,
        })
        .collect();
    let data = ThreadData {
        id: id.clone(),
        thread_type,
        title,
        status,
        created_at,
        updated_at,
        links: Some(links),
    };
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
        .insert_thread_link(thread_id, &payload.entity_type, &payload.entity_id, &payload.relation_type)
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
