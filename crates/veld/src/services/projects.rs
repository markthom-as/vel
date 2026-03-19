use std::path::Path;

use time::OffsetDateTime;
use vel_api_types::{
    ProjectCreateRequestData, ProjectFamilyData, ProjectRootRefData, ProjectStatusData,
};
use vel_core::{
    ActionThreadRoute, ActionThreadRouteTarget, ProjectFamily, ProjectId, ProjectProvisionRequest,
    ProjectRecord, ProjectRootRef, ProjectStatus,
};

use crate::{errors::AppError, state::AppState};

pub async fn list_projects(state: &AppState) -> Result<Vec<ProjectRecord>, AppError> {
    Ok(state.storage.list_projects().await?)
}

pub async fn get_project(state: &AppState, id: &str) -> Result<Option<ProjectRecord>, AppError> {
    Ok(state.storage.get_project(id).await?)
}

pub async fn list_project_families(state: &AppState) -> Result<Vec<ProjectFamily>, AppError> {
    let mut families = vec![
        ProjectFamily::Personal,
        ProjectFamily::Creative,
        ProjectFamily::Work,
    ];
    for family in state.storage.list_project_families().await? {
        if !families.contains(&family) {
            families.push(family);
        }
    }
    Ok(families)
}

pub async fn create_project(
    state: &AppState,
    payload: ProjectCreateRequestData,
) -> Result<ProjectRecord, AppError> {
    let slug = payload.slug.trim();
    let name = payload.name.trim();
    if slug.is_empty() {
        return Err(AppError::bad_request("project slug must not be empty"));
    }
    if name.is_empty() {
        return Err(AppError::bad_request("project name must not be empty"));
    }

    let now = OffsetDateTime::now_utc();
    let project = ProjectRecord {
        id: ProjectId::new(),
        slug: slug.to_string(),
        name: name.to_string(),
        family: match payload.family {
            ProjectFamilyData::Personal => ProjectFamily::Personal,
            ProjectFamilyData::Creative => ProjectFamily::Creative,
            ProjectFamilyData::Work => ProjectFamily::Work,
        },
        status: match payload.status.unwrap_or(ProjectStatusData::Active) {
            ProjectStatusData::Active => ProjectStatus::Active,
            ProjectStatusData::Paused => ProjectStatus::Paused,
            ProjectStatusData::Archived => ProjectStatus::Archived,
        },
        primary_repo: root_from_data(payload.primary_repo, "repo")?,
        primary_notes_root: root_from_data(payload.primary_notes_root, "notes_root")?,
        secondary_repos: payload
            .secondary_repos
            .into_iter()
            .map(|root| root_from_data(root, "repo"))
            .collect::<Result<Vec<_>, _>>()?,
        secondary_notes_roots: payload
            .secondary_notes_roots
            .into_iter()
            .map(|root| root_from_data(root, "notes_root"))
            .collect::<Result<Vec<_>, _>>()?,
        upstream_ids: payload.upstream_ids,
        pending_provision: ProjectProvisionRequest {
            create_repo: payload.pending_provision.create_repo,
            create_notes_root: payload.pending_provision.create_notes_root,
        },
        created_at: now,
        updated_at: now,
        archived_at: None,
    };

    state
        .storage
        .create_project(project)
        .await
        .map_err(map_project_storage_error)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectThreadPurpose {
    Provisioning,
    Review,
}

pub fn project_thread_route(
    project: &ProjectRecord,
    purpose: ProjectThreadPurpose,
) -> ActionThreadRoute {
    let (thread_type, label) = match purpose {
        ProjectThreadPurpose::Provisioning => (
            "project_provisioning",
            format!("Open provisioning thread for {}", project.name),
        ),
        ProjectThreadPurpose::Review => (
            "project_review",
            format!("Open related threads for {}", project.name),
        ),
    };

    ActionThreadRoute {
        target: ActionThreadRouteTarget::FilteredThreads,
        label,
        thread_id: None,
        thread_type: Some(thread_type.to_string()),
        project_id: Some(project.id.clone()),
    }
}

fn root_from_data(
    data: ProjectRootRefData,
    default_kind: &str,
) -> Result<ProjectRootRef, AppError> {
    let path = data.path.trim();
    if path.is_empty() {
        return Err(AppError::bad_request("project root path must not be empty"));
    }

    let label = if data.label.trim().is_empty() {
        Path::new(path)
            .file_name()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .unwrap_or(path)
            .to_string()
    } else {
        data.label.trim().to_string()
    };

    let kind = if data.kind.trim().is_empty() {
        default_kind.to_string()
    } else {
        data.kind.trim().to_string()
    };

    Ok(ProjectRootRef {
        path: path.to_string(),
        label,
        kind,
    })
}

fn map_project_storage_error(error: vel_storage::StorageError) -> AppError {
    let message = error.to_string();
    match error {
        vel_storage::StorageError::Database(_)
            if message.contains("UNIQUE constraint failed: projects.slug") =>
        {
            AppError::bad_request("project slug already exists")
        }
        vel_storage::StorageError::Validation(message) => AppError::bad_request(message),
        other => other.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use tokio::sync::broadcast;
    use vel_api_types::ProjectProvisionRequestData;
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
    async fn project_service_create_is_local_first() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);

        let project = create_project(
            &state,
            ProjectCreateRequestData {
                slug: "vel-phase5".to_string(),
                name: "Vel Phase 5".to_string(),
                family: ProjectFamilyData::Work,
                status: None,
                primary_repo: ProjectRootRefData {
                    path: "/tmp/vel-phase5".to_string(),
                    label: "vel-phase5".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRefData {
                    path: "/tmp/notes/vel-phase5".to_string(),
                    label: "vel-phase5".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: BTreeMap::new(),
                pending_provision: ProjectProvisionRequestData {
                    create_repo: true,
                    create_notes_root: true,
                },
            },
        )
        .await
        .unwrap();

        assert_eq!(project.slug, "vel-phase5");
        assert_eq!(project.family, ProjectFamily::Work);
        assert!(project.pending_provision.create_repo);
        assert!(project.pending_provision.create_notes_root);
    }

    #[test]
    fn project_thread_route_preserves_project_scope() {
        let project = ProjectRecord {
            id: ProjectId::from("proj_vel".to_string()),
            slug: "vel".to_string(),
            name: "Vel".to_string(),
            family: ProjectFamily::Work,
            status: ProjectStatus::Active,
            primary_repo: ProjectRootRef {
                path: "/tmp/vel".to_string(),
                label: "vel".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRef {
                path: "/tmp/notes/vel".to_string(),
                label: "vel".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![],
            secondary_notes_roots: vec![],
            upstream_ids: BTreeMap::new(),
            pending_provision: ProjectProvisionRequest {
                create_repo: false,
                create_notes_root: false,
            },
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
            archived_at: None,
        };

        let route = project_thread_route(&project, ProjectThreadPurpose::Review);
        assert_eq!(route.target, ActionThreadRouteTarget::FilteredThreads);
        assert_eq!(route.thread_type.as_deref(), Some("project_review"));
        assert_eq!(route.project_id.as_ref(), Some(&project.id));
    }
}
