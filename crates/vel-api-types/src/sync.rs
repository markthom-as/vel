use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchSyncCapabilityData {
    pub repo_root: String,
    pub default_remote: String,
    pub supports_fetch: bool,
    pub supports_pull: bool,
    pub supports_push: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationProfileData {
    pub profile_id: String,
    pub label: String,
    pub command_hint: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchSyncRequestData {
    pub repo_root: String,
    pub branch: String,
    #[serde(default)]
    pub remote: Option<String>,
    #[serde(default)]
    pub base_branch: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequestData {
    pub repo_root: String,
    pub profile_id: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(default)]
    pub requested_by: Option<String>,
}
