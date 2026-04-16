use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use vel_core::ProjectId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectFamilyData {
    Personal,
    Creative,
    Work,
}

impl From<vel_core::ProjectFamily> for ProjectFamilyData {
    fn from(value: vel_core::ProjectFamily) -> Self {
        match value {
            vel_core::ProjectFamily::Personal => Self::Personal,
            vel_core::ProjectFamily::Creative => Self::Creative,
            vel_core::ProjectFamily::Work => Self::Work,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatusData {
    Active,
    Paused,
    Archived,
}

impl From<vel_core::ProjectStatus> for ProjectStatusData {
    fn from(value: vel_core::ProjectStatus) -> Self {
        match value {
            vel_core::ProjectStatus::Active => Self::Active,
            vel_core::ProjectStatus::Paused => Self::Paused,
            vel_core::ProjectStatus::Archived => Self::Archived,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRootRefData {
    pub path: String,
    pub label: String,
    pub kind: String,
}

impl From<vel_core::ProjectRootRef> for ProjectRootRefData {
    fn from(value: vel_core::ProjectRootRef) -> Self {
        Self {
            path: value.path,
            label: value.label,
            kind: value.kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectProvisionRequestData {
    #[serde(default)]
    pub create_repo: bool,
    #[serde(default)]
    pub create_notes_root: bool,
}

impl From<vel_core::ProjectProvisionRequest> for ProjectProvisionRequestData {
    fn from(value: vel_core::ProjectProvisionRequest) -> Self {
        Self {
            create_repo: value.create_repo,
            create_notes_root: value.create_notes_root,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRecordData {
    pub id: ProjectId,
    pub slug: String,
    pub name: String,
    pub family: ProjectFamilyData,
    pub status: ProjectStatusData,
    pub primary_repo: ProjectRootRefData,
    pub primary_notes_root: ProjectRootRefData,
    #[serde(default)]
    pub secondary_repos: Vec<ProjectRootRefData>,
    #[serde(default)]
    pub secondary_notes_roots: Vec<ProjectRootRefData>,
    #[serde(default)]
    pub upstream_ids: BTreeMap<String, String>,
    #[serde(default)]
    pub pending_provision: ProjectProvisionRequestData,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub archived_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreateRequestData {
    pub slug: String,
    pub name: String,
    pub family: ProjectFamilyData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProjectStatusData>,
    pub primary_repo: ProjectRootRefData,
    pub primary_notes_root: ProjectRootRefData,
    #[serde(default)]
    pub secondary_repos: Vec<ProjectRootRefData>,
    #[serde(default)]
    pub secondary_notes_roots: Vec<ProjectRootRefData>,
    #[serde(default)]
    pub upstream_ids: BTreeMap<String, String>,
    #[serde(default)]
    pub pending_provision: ProjectProvisionRequestData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreateResponseData {
    pub project: ProjectRecordData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectListResponseData {
    #[serde(default)]
    pub projects: Vec<ProjectRecordData>,
}

impl From<vel_core::ProjectRecord> for ProjectRecordData {
    fn from(value: vel_core::ProjectRecord) -> Self {
        Self {
            id: value.id,
            slug: value.slug,
            name: value.name,
            family: value.family.into(),
            status: value.status.into(),
            primary_repo: value.primary_repo.into(),
            primary_notes_root: value.primary_notes_root.into(),
            secondary_repos: value.secondary_repos.into_iter().map(Into::into).collect(),
            secondary_notes_roots: value
                .secondary_notes_roots
                .into_iter()
                .map(Into::into)
                .collect(),
            upstream_ids: value.upstream_ids,
            pending_provision: value.pending_provision.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            archived_at: value.archived_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_create_request_serializes_project_contract() {
        let mut upstream_ids = BTreeMap::new();
        upstream_ids.insert("github".to_string(), "vel/phase5".to_string());

        let request = ProjectCreateRequestData {
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
            secondary_repos: Vec::new(),
            secondary_notes_roots: Vec::new(),
            upstream_ids,
            pending_provision: ProjectProvisionRequestData {
                create_repo: true,
                create_notes_root: false,
            },
        };

        let value = serde_json::to_value(request).expect("project request should serialize");
        assert_eq!(value["family"], "work");
        assert!(value.get("status").is_none());
        assert_eq!(value["pending_provision"]["create_repo"], true);
        assert_eq!(value["pending_provision"]["create_notes_root"], false);
        assert_eq!(value["upstream_ids"]["github"], "vel/phase5");
    }
}
