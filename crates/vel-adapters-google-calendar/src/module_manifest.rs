use vel_core::{
    CapabilityRequest, RegistryKind, RegistryManifest, RegistryStatus, SemanticRegistryId,
};

pub fn requested_capabilities() -> Vec<CapabilityRequest> {
    vec![
        CapabilityRequest {
            capability: "google.calendar.read".to_string(),
            feature_gate: Some("google-calendar".to_string()),
        },
        CapabilityRequest {
            capability: "google.calendar.write".to_string(),
            feature_gate: Some("google-calendar".to_string()),
        },
    ]
}

pub fn google_calendar_module_manifest() -> RegistryManifest {
    RegistryManifest {
        registry_id: SemanticRegistryId::new(
            RegistryKind::Module,
            "integration",
            "google-calendar",
        ),
        display_name: "Google Calendar".to_string(),
        version: "0.5".to_string(),
        status: RegistryStatus::Active,
        manifest_ref: "modules/integration/google-calendar/module.yaml".to_string(),
        capability_requests: requested_capabilities(),
    }
}

#[cfg(test)]
mod tests {
    use super::google_calendar_module_manifest;

    #[test]
    fn google_calendar_manifest_uses_canonical_registry_identity() {
        let manifest = google_calendar_module_manifest();

        assert_eq!(
            manifest.registry_id.as_string(),
            "module.integration.google-calendar"
        );
        assert_eq!(manifest.version, "0.5");
        assert_eq!(manifest.capability_requests.len(), 2);
    }
}
