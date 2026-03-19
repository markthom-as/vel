use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(pub(crate) String);

impl ProjectId {
    pub fn new() -> Self {
        Self(format!("proj_{}", Uuid::new_v4().simple()))
    }
}

impl Default for ProjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ProjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ProjectId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for ProjectId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectFamily {
    Personal,
    Creative,
    Work,
}

impl Display for ProjectFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Personal => "personal",
            Self::Creative => "creative",
            Self::Work => "work",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for ProjectFamily {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "personal" => Ok(Self::Personal),
            "creative" => Ok(Self::Creative),
            "work" => Ok(Self::Work),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown project family: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    #[default]
    Active,
    Paused,
    Archived,
}

impl Display for ProjectStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Archived => "archived",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for ProjectStatus {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "paused" => Ok(Self::Paused),
            "archived" => Ok(Self::Archived),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown project status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectRootRef {
    pub path: String,
    pub label: String,
    pub kind: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectProvisionRequest {
    #[serde(default)]
    pub create_repo: bool,
    #[serde(default)]
    pub create_notes_root: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectRecord {
    pub id: ProjectId,
    pub slug: String,
    pub name: String,
    pub family: ProjectFamily,
    pub status: ProjectStatus,
    pub primary_repo: ProjectRootRef,
    pub primary_notes_root: ProjectRootRef,
    #[serde(default)]
    pub secondary_repos: Vec<ProjectRootRef>,
    #[serde(default)]
    pub secondary_notes_roots: Vec<ProjectRootRef>,
    #[serde(default)]
    pub upstream_ids: BTreeMap<String, String>,
    #[serde(default)]
    pub pending_provision: ProjectProvisionRequest,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub archived_at: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use super::{ProjectFamily, ProjectRecord};

    #[test]
    fn project_workspace_example_parses() {
        let record: ProjectRecord = serde_json::from_str(include_str!(
            "../../../config/examples/project-workspace.example.json"
        ))
        .expect("project workspace example should parse");

        assert_eq!(record.family, ProjectFamily::Work);
        assert_eq!(record.primary_repo.kind, "repo");
        assert_eq!(record.primary_notes_root.kind, "notes_root");
    }
}
