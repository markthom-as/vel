use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::ConfigError;

const REPO_CONTRACTS_MANIFEST_JSON: &str = include_str!("../../../config/contracts-manifest.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractManifestEntry {
    pub id: String,
    pub path: String,
    pub kind: String,
    pub schema: String,
    #[serde(default)]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractsManifest {
    pub version: u32,
    pub generated_by: String,
    pub live_configs: Vec<ContractManifestEntry>,
    pub templates: Vec<ContractManifestEntry>,
    pub contract_examples: Vec<ContractManifestEntry>,
    pub authority_docs: Vec<String>,
}

impl ContractsManifest {
    pub fn from_json_str(content: &str) -> Result<Self, ConfigError> {
        serde_json::from_str(content).map_err(ConfigError::ContractManifestParse)
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        Self::from_json_str(&content)
    }

    pub fn load_repo() -> Result<Self, ConfigError> {
        Self::from_json_str(REPO_CONTRACTS_MANIFEST_JSON)
    }

    pub fn schema_count(&self) -> usize {
        self.live_configs
            .iter()
            .chain(self.templates.iter())
            .chain(self.contract_examples.iter())
            .map(|entry| entry.schema.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len()
    }
}
