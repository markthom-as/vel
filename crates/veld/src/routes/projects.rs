use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ProjectCreateRequestData, ProjectCreateResponseData, ProjectFamilyData,
    ProjectListResponseData, ProjectRecordData,
};

use crate::{errors::AppError, services, state::AppState};

pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ProjectListResponseData>>, AppError> {
    let projects = services::projects::list_projects(&state).await?;
    Ok(Json(ApiResponse::success(
        ProjectListResponseData {
            projects: projects.into_iter().map(ProjectRecordData::from).collect(),
        },
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ProjectRecordData>>, AppError> {
    let project = services::projects::get_project(&state, id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("project not found"))?;
    Ok(Json(ApiResponse::success(
        ProjectRecordData::from(project),
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<ProjectCreateRequestData>,
) -> Result<Json<ApiResponse<ProjectCreateResponseData>>, AppError> {
    // Project creation stays local-first in this phase; pending_provision is persisted without side effects.
    let project = services::projects::create_project(&state, payload).await?;
    Ok(Json(ApiResponse::success(
        ProjectCreateResponseData {
            project: ProjectRecordData::from(project),
        },
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}

pub async fn list_project_families(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ProjectFamilyData>>>, AppError> {
    let families = services::projects::list_project_families(&state).await?;
    Ok(Json(ApiResponse::success(
        families.into_iter().map(ProjectFamilyData::from).collect(),
        format!("req_{}", Uuid::new_v4().simple()),
    )))
}
