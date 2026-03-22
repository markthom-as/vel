use serde::{Deserialize, Serialize};

use crate::{CapabilityRequest, RegistryObject, RegistryStatus};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleLifecycleState {
    Registered,
    Reconciled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleEnablementState {
    Eligible,
    Activated,
    DisabledFeature,
    PolicyDenied,
    ReadOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestedCapability {
    pub capability: String,
    pub feature_gate: Option<String>,
    pub read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleCapabilityProfile {
    pub module_id: String,
    pub requested_capabilities: Vec<RequestedCapability>,
    pub lifecycle_state: ModuleLifecycleState,
    pub enablement_state: ModuleEnablementState,
    pub eligible: bool,
    pub activated: bool,
    pub invokable: bool,
    pub read_only: bool,
}

impl ModuleCapabilityProfile {
    pub fn from_registry_object(object: &RegistryObject) -> Self {
        let requested_capabilities = object
            .capability_requests
            .iter()
            .map(requested_capability_from)
            .collect::<Vec<_>>();
        let overlay_enabled = object.persisted_overlay.enabled.unwrap_or(true);
        let read_only = requested_capabilities.iter().all(|capability| capability.read_only);
        let lifecycle_state = ModuleLifecycleState::Reconciled;
        let eligible = object.status == RegistryStatus::Active && overlay_enabled;
        let activated = eligible;

        Self {
            module_id: object.id.clone(),
            requested_capabilities,
            lifecycle_state,
            enablement_state: if activated {
                ModuleEnablementState::Activated
            } else {
                ModuleEnablementState::Eligible
            },
            eligible,
            activated,
            invokable: activated,
            read_only,
        }
    }

    pub fn registered(module_id: impl Into<String>, capability_requests: &[CapabilityRequest]) -> Self {
        let requested_capabilities = capability_requests
            .iter()
            .map(requested_capability_from)
            .collect::<Vec<_>>();
        let read_only = requested_capabilities.iter().all(|capability| capability.read_only);

        Self {
            module_id: module_id.into(),
            requested_capabilities,
            lifecycle_state: ModuleLifecycleState::Registered,
            enablement_state: ModuleEnablementState::Eligible,
            eligible: false,
            activated: false,
            invokable: false,
            read_only,
        }
    }
}

fn requested_capability_from(request: &CapabilityRequest) -> RequestedCapability {
    RequestedCapability {
        capability: request.capability.clone(),
        feature_gate: request.feature_gate.clone(),
        read_only: is_read_only_capability(&request.capability),
    }
}

fn is_read_only_capability(capability: &str) -> bool {
    capability.ends_with(".read") || capability == "object.read"
}

#[cfg(test)]
mod tests {
    use super::{
        ModuleCapabilityProfile, ModuleEnablementState, ModuleLifecycleState, RequestedCapability,
    };
    use crate::{
        CapabilityRequest, PersistedOverlay, RegistryKind, RegistryObject, RegistryStatus,
    };

    #[test]
    fn requested_capabilities_capture_feature_gate_and_read_only_posture() {
        let profile = ModuleCapabilityProfile::registered(
            "module.integration.todoist",
            &[
                CapabilityRequest {
                    capability: "todoist.read".to_string(),
                    feature_gate: Some("todoist".to_string()),
                },
                CapabilityRequest {
                    capability: "todoist.write".to_string(),
                    feature_gate: Some("todoist".to_string()),
                },
            ],
        );

        assert_eq!(profile.lifecycle_state, ModuleLifecycleState::Registered);
        assert_eq!(profile.requested_capabilities.len(), 2);
        assert_eq!(
            profile.requested_capabilities[0],
            RequestedCapability {
                capability: "todoist.read".to_string(),
                feature_gate: Some("todoist".to_string()),
                read_only: true,
            }
        );
        assert!(!profile.read_only);
    }

    #[test]
    fn reconciled_profiles_expose_eligible_activated_and_invokable_states() {
        let profile = ModuleCapabilityProfile::from_registry_object(&RegistryObject {
            id: "module.core.orientation".to_string(),
            registry_kind: RegistryKind::Module,
            namespace: "core".to_string(),
            slug: "orientation".to_string(),
            display_name: "Orientation".to_string(),
            version: "0.5".to_string(),
            status: RegistryStatus::Active,
            manifest_ref: "modules/core/orientation/module.yaml".to_string(),
            capability_requests: vec![CapabilityRequest {
                capability: "object.read".to_string(),
                feature_gate: None,
            }],
            persisted_overlay: PersistedOverlay {
                enabled: Some(true),
                notes: None,
                metadata: serde_json::json!({"activation":"policy-mediated"}),
            },
        });

        assert_eq!(profile.lifecycle_state, ModuleLifecycleState::Reconciled);
        assert_eq!(profile.enablement_state, ModuleEnablementState::Activated);
        assert!(profile.eligible);
        assert!(profile.activated);
        assert!(profile.invokable);
        assert!(profile.read_only);
    }
}
