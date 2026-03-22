use serde::{Deserialize, Serialize};

use crate::{ManifestSource, RegistryManifest};

use super::seeded_workflows::SeededWorkflowSpec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreBootstrapPolicy {
    pub deterministic: bool,
    pub idempotent: bool,
}

impl Default for CoreBootstrapPolicy {
    fn default() -> Self {
        Self {
            deterministic: true,
            idempotent: true,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreBootstrapReport {
    pub registry_registered: usize,
    pub registry_reconciled: usize,
    pub workflow_seeded: usize,
    pub workflow_unchanged: usize,
    pub workflow_updated: usize,
    pub workflow_drifted: usize,
    pub workflow_forked_local: usize,
}

pub trait CoreBootstrapSource: ManifestSource {
    fn seeded_workflows(&self) -> Result<Vec<SeededWorkflowSpec>, String>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreBootstrapBundle {
    pub registry_manifests: Vec<RegistryManifest>,
    pub seeded_workflows: Vec<SeededWorkflowSpec>,
}

impl ManifestSource for CoreBootstrapBundle {
    fn load_manifests(&self) -> Result<Vec<RegistryManifest>, String> {
        Ok(self.registry_manifests.clone())
    }
}

impl CoreBootstrapSource for CoreBootstrapBundle {
    fn seeded_workflows(&self) -> Result<Vec<SeededWorkflowSpec>, String> {
        Ok(self.seeded_workflows.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::{CoreBootstrapBundle, CoreBootstrapPolicy, CoreBootstrapSource};
    use crate::{
        registry_ids::{RegistryKind, SemanticRegistryId},
        CapabilityRequest, ManifestSource, RegistryManifest, RegistryStatus,
        SeededWorkflowMutability, SeededWorkflowSpec,
    };

    #[test]
    fn bootstrap_policy_defaults_to_deterministic_and_idempotent() {
        let policy = CoreBootstrapPolicy::default();
        assert!(policy.deterministic);
        assert!(policy.idempotent);
    }

    #[test]
    fn bootstrap_bundle_provides_registry_and_seeded_workflow_inputs() {
        let bundle = CoreBootstrapBundle {
            registry_manifests: vec![RegistryManifest {
                registry_id: SemanticRegistryId::new(RegistryKind::Module, "core", "orientation"),
                display_name: "Orientation".to_string(),
                version: "0.5".to_string(),
                status: RegistryStatus::Active,
                manifest_ref: "modules/core/orientation/module.yaml".to_string(),
                capability_requests: vec![CapabilityRequest {
                    capability: "workflow.invoke".to_string(),
                    feature_gate: None,
                }],
            }],
            seeded_workflows: vec![SeededWorkflowSpec {
                workflow_id: "workflow_01seededbrief".to_string(),
                source_module_id: "module.core.orientation".to_string(),
                manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
                display_name: "Daily Brief".to_string(),
                version: "1.0.0".to_string(),
                mutability: SeededWorkflowMutability::Forkable,
                definition_json: serde_json::json!({"step_types":["action","skill"]}),
                policy_ref: Some("policy.workflow.daily-brief".to_string()),
                seed_version: "2026.03.22".to_string(),
                status: "active".to_string(),
            }],
        };

        assert_eq!(bundle.load_manifests().unwrap().len(), 1);
        assert_eq!(bundle.seeded_workflows().unwrap().len(), 1);
    }
}
