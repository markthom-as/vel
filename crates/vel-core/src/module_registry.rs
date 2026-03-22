use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{registry_ids::SemanticRegistryId, RegistryKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryStatus {
    Active,
    Deprecated,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationState {
    New,
    Unchanged,
    Updated,
    Drifted,
    ForkedLocal,
    Superseded,
    Disabled,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub capability: String,
    pub feature_gate: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedOverlay {
    pub enabled: Option<bool>,
    pub notes: Option<String>,
    pub metadata: JsonValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryManifest {
    pub registry_id: SemanticRegistryId,
    pub display_name: String,
    pub version: String,
    pub status: RegistryStatus,
    pub manifest_ref: String,
    pub capability_requests: Vec<CapabilityRequest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryObject {
    pub id: String,
    pub registry_kind: RegistryKind,
    pub namespace: String,
    pub slug: String,
    pub display_name: String,
    pub version: String,
    pub status: RegistryStatus,
    pub manifest_ref: String,
    pub capability_requests: Vec<CapabilityRequest>,
    pub persisted_overlay: PersistedOverlay,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReconciliationResult {
    pub state: ReconciliationState,
    pub object: RegistryObject,
    pub reason: String,
}

pub trait ManifestSource {
    fn load_manifests(&self) -> Result<Vec<RegistryManifest>, String>;
}

pub trait RegistryReconciler {
    fn reconcile(
        &self,
        manifest: &RegistryManifest,
        persisted_overlay: Option<PersistedOverlay>,
        existing: Option<&RegistryObject>,
    ) -> ReconciliationResult;
}

pub trait RegistryLoader {
    fn registry_roles(&self) -> [&'static str; 3] {
        ["ManifestSource", "RegistryLoader", "RegistryReconciler"]
    }
}

pub fn materialize_registry_object(
    manifest: &RegistryManifest,
    persisted_overlay: Option<PersistedOverlay>,
) -> RegistryObject {
    RegistryObject {
        id: manifest.registry_id.as_string(),
        registry_kind: manifest.registry_id.kind.clone(),
        namespace: manifest.registry_id.namespace.clone(),
        slug: manifest.registry_id.slug.clone(),
        display_name: manifest.display_name.clone(),
        version: manifest.version.clone(),
        status: manifest.status.clone(),
        manifest_ref: manifest.manifest_ref.clone(),
        capability_requests: manifest.capability_requests.clone(),
        // The persisted overlay is canonical local state, not ambient manifest truth.
        persisted_overlay: persisted_overlay.unwrap_or(PersistedOverlay {
            enabled: None,
            notes: None,
            metadata: serde_json::json!({}),
        }),
    }
}

#[derive(Debug, Default)]
pub struct DefaultRegistryReconciler;

impl RegistryReconciler for DefaultRegistryReconciler {
    fn reconcile(
        &self,
        manifest: &RegistryManifest,
        persisted_overlay: Option<PersistedOverlay>,
        existing: Option<&RegistryObject>,
    ) -> ReconciliationResult {
        let object = materialize_registry_object(manifest, persisted_overlay);
        let state = match existing {
            None => ReconciliationState::New,
            Some(existing)
                if existing.version == object.version
                    && existing.status == object.status
                    && existing.display_name == object.display_name
                    && existing.manifest_ref == object.manifest_ref
                    && existing.capability_requests == object.capability_requests
                    && existing.persisted_overlay == object.persisted_overlay =>
            {
                ReconciliationState::Unchanged
            }
            Some(existing)
                if existing.manifest_ref == object.manifest_ref
                    && existing.persisted_overlay != object.persisted_overlay =>
            {
                ReconciliationState::Drifted
            }
            Some(existing) if existing.status == RegistryStatus::Disabled => {
                ReconciliationState::Disabled
            }
            Some(_) => ReconciliationState::Updated,
        };

        ReconciliationResult {
            state,
            object,
            reason: "reconcile manifest-backed registry object against persisted overlay".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        materialize_registry_object, CapabilityRequest, DefaultRegistryReconciler, ManifestSource,
        PersistedOverlay, ReconciliationState, RegistryLoader, RegistryManifest, RegistryObject,
        RegistryReconciler, RegistryStatus,
    };
    use crate::registry_ids::{
        RegistryKind, SemanticRegistryId, MODULE_INTEGRATION_GOOGLE_CALENDAR,
        MODULE_INTEGRATION_TODOIST, SKILL_CORE_DAILY_BRIEF, TOOL_OBJECT_GET,
    };

    struct StaticManifestSource;

    impl ManifestSource for StaticManifestSource {
        fn load_manifests(&self) -> Result<Vec<RegistryManifest>, String> {
            Ok(vec![
                RegistryManifest {
                    registry_id: SemanticRegistryId::new(
                        RegistryKind::Module,
                        "integration",
                        "todoist",
                    ),
                    display_name: "Todoist".to_string(),
                    version: "0.5".to_string(),
                    status: RegistryStatus::Active,
                    manifest_ref: "modules/integration/todoist/module.yaml".to_string(),
                    capability_requests: vec![CapabilityRequest {
                        capability: "todoist.read".to_string(),
                        feature_gate: Some("todoist".to_string()),
                    }],
                },
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
                    capability_requests: vec![CapabilityRequest {
                        capability: "google.calendar.read".to_string(),
                        feature_gate: Some("google-calendar".to_string()),
                    }],
                },
                RegistryManifest {
                    registry_id: SemanticRegistryId::new(RegistryKind::Skill, "core", "daily-brief"),
                    display_name: "Daily Brief".to_string(),
                    version: "0.5".to_string(),
                    status: RegistryStatus::Active,
                    manifest_ref: "modules/core/orientation/skills/daily-brief.yaml".to_string(),
                    capability_requests: vec![],
                },
                RegistryManifest {
                    registry_id: SemanticRegistryId::new(RegistryKind::Tool, "object", "get"),
                    display_name: "Object Get".to_string(),
                    version: "0.5".to_string(),
                    status: RegistryStatus::Active,
                    manifest_ref: "modules/core/object/tools/get.yaml".to_string(),
                    capability_requests: vec![],
                },
            ])
        }
    }

    struct DummyRegistryLoader;

    impl RegistryLoader for DummyRegistryLoader {}

    #[test]
    fn registry_contract_keeps_canonical_ids_and_loader_roles_visible() {
        let manifests = StaticManifestSource.load_manifests().unwrap();
        let ids = manifests
            .iter()
            .map(|manifest| manifest.registry_id.as_string())
            .collect::<Vec<_>>();

        assert!(ids.iter().any(|id| id == MODULE_INTEGRATION_TODOIST));
        assert!(ids.iter().any(|id| id == MODULE_INTEGRATION_GOOGLE_CALENDAR));
        assert!(ids.iter().any(|id| id == SKILL_CORE_DAILY_BRIEF));
        assert!(ids.iter().any(|id| id == TOOL_OBJECT_GET));
        assert_eq!(
            DummyRegistryLoader.registry_roles(),
            ["ManifestSource", "RegistryLoader", "RegistryReconciler"]
        );
    }

    #[test]
    fn reconciler_names_new_unchanged_updated_and_drifted_states() {
        let manifest = RegistryManifest {
            registry_id: SemanticRegistryId::new(RegistryKind::Module, "integration", "todoist"),
            display_name: "Todoist".to_string(),
            version: "0.5".to_string(),
            status: RegistryStatus::Active,
            manifest_ref: "modules/integration/todoist/module.yaml".to_string(),
            capability_requests: vec![CapabilityRequest {
                capability: "todoist.read".to_string(),
                feature_gate: Some("todoist".to_string()),
            }],
        };
        let overlay = PersistedOverlay {
            enabled: Some(true),
            notes: Some("persisted overlay".to_string()),
            metadata: serde_json::json!({"activation":"manual"}),
        };
        let reconciler = DefaultRegistryReconciler;
        let created = reconciler.reconcile(&manifest, Some(overlay.clone()), None);
        let unchanged = reconciler.reconcile(&manifest, Some(overlay.clone()), Some(&created.object));

        let updated_existing = RegistryObject {
            version: "0.4".to_string(),
            ..created.object.clone()
        };
        let updated = reconciler.reconcile(&manifest, Some(overlay.clone()), Some(&updated_existing));

        let drifted_overlay = PersistedOverlay {
            enabled: Some(false),
            notes: Some("persisted overlay drift".to_string()),
            metadata: serde_json::json!({"activation":"manual"}),
        };
        let drifted = reconciler.reconcile(
            &manifest,
            Some(drifted_overlay),
            Some(&created.object),
        );

        assert_eq!(created.state, ReconciliationState::New);
        assert_eq!(unchanged.state, ReconciliationState::Unchanged);
        assert_eq!(updated.state, ReconciliationState::Updated);
        assert_eq!(drifted.state, ReconciliationState::Drifted);
    }

    #[test]
    fn materialized_registry_object_preserves_persisted_overlay() {
        let object = materialize_registry_object(
            &RegistryManifest {
                registry_id: SemanticRegistryId::new(RegistryKind::Module, "integration", "todoist"),
                display_name: "Todoist".to_string(),
                version: "0.5".to_string(),
                status: RegistryStatus::Active,
                manifest_ref: "modules/integration/todoist/module.yaml".to_string(),
                capability_requests: vec![],
            },
            Some(PersistedOverlay {
                enabled: Some(true),
                notes: Some("persisted overlay".to_string()),
                metadata: serde_json::json!({"read_only":true}),
            }),
        );

        assert_eq!(object.id, MODULE_INTEGRATION_TODOIST);
        assert_eq!(object.persisted_overlay.notes.as_deref(), Some("persisted overlay"));
    }
}
