use serde_json::json;
use vel_core::{
    CapabilityRequest, Grant, GrantScope, ModuleEnablementState, PersistedOverlay, RegistryKind,
    RegistryObject, RegistryStatus,
};
use veld::services::module_activation::{ModuleActivationRequest, ModuleActivationService};
use veld::services::module_policy_bridge::{ModulePolicyBridge, ModulePolicyBridgeError};

fn grant_for(capabilities: &[&str], read_only: bool) -> Grant {
    Grant {
        id: "grant_01moduleactivation".to_string(),
        scope: vec![
            GrantScope::Workspace,
            GrantScope::Module("module.integration.todoist".to_string()),
        ],
        capabilities: capabilities.iter().map(|value| value.to_string()).collect(),
        durable: false,
        run_scoped: true,
        read_only,
    }
}

fn module(
    id: &str,
    capability_requests: Vec<CapabilityRequest>,
    enabled: Option<bool>,
) -> RegistryObject {
    RegistryObject {
        id: id.to_string(),
        registry_kind: RegistryKind::Module,
        namespace: if id.contains(".core.") {
            "core".to_string()
        } else {
            "integration".to_string()
        },
        slug: id.rsplit('.').next().unwrap().to_string(),
        display_name: id.to_string(),
        version: "0.5".to_string(),
        status: RegistryStatus::Active,
        manifest_ref: format!("modules/{id}/module.yaml"),
        capability_requests,
        persisted_overlay: PersistedOverlay {
            enabled,
            notes: Some("feature gate".to_string()),
            metadata: json!({"read_only":false}),
        },
    }
}

#[test]
fn allowed_activation_marks_module_eligible_activated_and_invokable() {
    let service = ModuleActivationService::default();
    let result = service
        .activate(&ModuleActivationRequest {
            registry_object: module(
                "module.integration.todoist",
                vec![CapabilityRequest {
                    capability: "todoist.read".to_string(),
                    feature_gate: Some("todoist".to_string()),
                }],
                Some(true),
            ),
            enabled_feature_gates: vec!["todoist".to_string()],
            grant: grant_for(&["todoist.read"], false),
            read_only: false,
        })
        .unwrap();

    assert!(result.requested_capabilities.eligible);
    assert_eq!(result.enablement_state, ModuleEnablementState::Activated);
    assert!(result.eligible);
    assert!(result.activated);
    assert!(result.invokable);
}

#[test]
fn disabled_feature_gate_returns_unsupported_capability_without_breaking_activation_model() {
    let bridge = ModulePolicyBridge::default();
    let profile = vel_core::ModuleCapabilityProfile::from_registry_object(&module(
        "module.integration.google-calendar",
        vec![CapabilityRequest {
            capability: "google.calendar.read".to_string(),
            feature_gate: Some("google-calendar".to_string()),
        }],
        Some(true),
    ));

    let error = bridge
        .evaluate(&veld::services::module_policy_bridge::ModulePolicyBridgeInput {
            module_id: "module.integration.google-calendar".to_string(),
            requested_capabilities: profile,
            enabled_feature_gates: vec![],
            grant: grant_for(&["google.calendar.read"], false),
            read_only: false,
        })
        .unwrap_err();

    assert!(matches!(
        error,
        ModulePolicyBridgeError::UnsupportedCapability(_)
    ));
}

#[test]
fn read_only_activation_refuses_write_capabilities() {
    let service = ModuleActivationService::default();
    let error = service
        .activate(&ModuleActivationRequest {
            registry_object: module(
                "module.integration.todoist",
                vec![CapabilityRequest {
                    capability: "todoist.write".to_string(),
                    feature_gate: Some("todoist".to_string()),
                }],
                Some(true),
            ),
            enabled_feature_gates: vec!["todoist".to_string()],
            grant: grant_for(&["todoist.write"], true),
            read_only: false,
        })
        .unwrap_err();

    assert!(matches!(
        error,
        ModulePolicyBridgeError::ReadOnlyViolation(_)
    ));
}

#[test]
fn core_modules_still_pass_through_policy_mediation() {
    let bridge = ModulePolicyBridge::default();
    let mut profile = vel_core::ModuleCapabilityProfile::from_registry_object(&module(
        "module.core.orientation",
        vec![CapabilityRequest {
            capability: "object.read".to_string(),
            feature_gate: None,
        }],
        Some(true),
    ));
    profile.requested_capabilities.push(vel_core::RequestedCapability {
        capability: "object.write".to_string(),
        feature_gate: None,
        read_only: false,
    });

    let error = bridge
        .evaluate(&veld::services::module_policy_bridge::ModulePolicyBridgeInput {
            module_id: "module.core.orientation".to_string(),
            requested_capabilities: profile,
            enabled_feature_gates: vec![],
            grant: grant_for(&["object.read", "object.write"], false),
            read_only: true,
        })
        .unwrap_err();

    assert!(matches!(error, ModulePolicyBridgeError::ReadOnlyViolation(_)));
}
