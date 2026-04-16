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
