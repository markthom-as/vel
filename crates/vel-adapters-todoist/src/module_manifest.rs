use vel_core::{
    CapabilityRequest, RegistryKind, RegistryManifest, RegistryStatus, SemanticRegistryId,
};

pub fn requested_capabilities() -> Vec<CapabilityRequest> {
    vec![
        CapabilityRequest {
            capability: "todoist.read".to_string(),
            feature_gate: Some("todoist".to_string()),
        },
        CapabilityRequest {
            capability: "todoist.write".to_string(),
            feature_gate: Some("todoist".to_string()),
        },
    ]
}

pub fn todoist_module_manifest() -> RegistryManifest {
    RegistryManifest {
        registry_id: SemanticRegistryId::new(RegistryKind::Module, "integration", "todoist"),
        display_name: "Todoist".to_string(),
        version: "0.5".to_string(),
        status: RegistryStatus::Active,
        manifest_ref: "modules/integration/todoist/module.yaml".to_string(),
        capability_requests: requested_capabilities(),
    }
}

#[cfg(test)]
mod tests {
    use super::todoist_module_manifest;

    #[test]
    fn todoist_manifest_uses_canonical_registry_identity() {
        let manifest = todoist_module_manifest();

        assert_eq!(manifest.registry_id.as_string(), "module.integration.todoist");
        assert_eq!(manifest.version, "0.5");
        assert_eq!(manifest.capability_requests.len(), 2);
    }
}
