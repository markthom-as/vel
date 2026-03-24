use sqlx::SqlitePool;
use vel_core::{Grant, ReconciliationResult, RegistryManifest};
use vel_storage::SqliteModuleRegistryStore;

use super::{
    module_activation::{ModuleActivation, ModuleActivationRequest, ModuleActivationService},
    registry_loader::RegistryLoader,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ProviderModuleRegistration {
    pub reconciliation: ReconciliationResult,
    pub activation: ModuleActivation,
    pub runtime_behavior_implemented: bool,
}

pub struct ProviderModuleRegistrationService {
    pool: SqlitePool,
}

impl ProviderModuleRegistrationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn register_provider_module(
        &self,
        manifest: RegistryManifest,
        grant: Grant,
        enabled_feature_gates: Vec<String>,
    ) -> Result<ProviderModuleRegistration, String> {
        let store = SqliteModuleRegistryStore::new(self.pool.clone());
        let loader = RegistryLoader::new(SingleManifestSource { manifest }, store);
        let reconciliation = loader
            .load_all()
            .await
            .map_err(|error| error.to_string())?
            .into_iter()
            .next()
            .ok_or_else(|| "provider module registration requires one manifest".to_string())?;

        let activation = ModuleActivationService::default()
            .activate(&ModuleActivationRequest {
                registry_object: reconciliation.object.clone(),
                enabled_feature_gates,
                grant,
                read_only: false,
            })
            .map_err(|error| format!("{error:?}"))?;

        Ok(ProviderModuleRegistration {
            reconciliation,
            activation,
            runtime_behavior_implemented: false,
        })
    }
}

struct SingleManifestSource {
    manifest: RegistryManifest,
}

impl vel_core::ManifestSource for SingleManifestSource {
    fn load_manifests(&self) -> Result<Vec<RegistryManifest>, String> {
        Ok(vec![self.manifest.clone()])
    }
}
