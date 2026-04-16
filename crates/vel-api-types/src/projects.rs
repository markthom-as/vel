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
