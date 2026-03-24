use vel_core::{ModuleCapabilityProfile, ModuleEnablementState, RegistryObject, RegistryStatus};

use super::module_policy_bridge::{
    ModulePolicyBridge, ModulePolicyBridgeDecision, ModulePolicyBridgeError,
    ModulePolicyBridgeInput,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleActivationRequest {
    pub registry_object: RegistryObject,
    pub enabled_feature_gates: Vec<String>,
    pub grant: vel_core::Grant,
    pub read_only: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleActivation {
    pub module_id: String,
    pub requested_capabilities: ModuleCapabilityProfile,
    pub enablement_state: ModuleEnablementState,
    pub eligible: bool,
    pub activated: bool,
    pub invokable: bool,
    pub read_only: bool,
}

#[derive(Debug, Default)]
pub struct ModuleActivationService {
    policy_bridge: ModulePolicyBridge,
}

impl ModuleActivationService {
    pub fn activate(
        &self,
        request: &ModuleActivationRequest,
    ) -> Result<ModuleActivation, ModulePolicyBridgeError> {
        let requested_capabilities =
            ModuleCapabilityProfile::from_registry_object(&request.registry_object);
        let overlay_enabled = request
            .registry_object
            .persisted_overlay
            .enabled
            .unwrap_or(true);
        let base_eligible =
            request.registry_object.status == RegistryStatus::Active && overlay_enabled;

        if !base_eligible {
            return Ok(ModuleActivation {
                module_id: request.registry_object.id.clone(),
                requested_capabilities,
                enablement_state: ModuleEnablementState::DisabledFeature,
                eligible: false,
                activated: false,
                invokable: false,
                read_only: request.read_only,
            });
        }

        let decision = self.policy_bridge.evaluate(&ModulePolicyBridgeInput {
            module_id: request.registry_object.id.clone(),
            requested_capabilities: requested_capabilities.clone(),
            enabled_feature_gates: request.enabled_feature_gates.clone(),
            grant: request.grant.clone(),
            read_only: request.read_only,
        })?;

        Ok(module_activation_from_decision(
            requested_capabilities,
            request.registry_object.id.clone(),
            request.read_only,
            decision,
        ))
    }
}

fn module_activation_from_decision(
    requested_capabilities: ModuleCapabilityProfile,
    module_id: String,
    read_only: bool,
    decision: ModulePolicyBridgeDecision,
) -> ModuleActivation {
    ModuleActivation {
        module_id,
        requested_capabilities,
        enablement_state: decision.enablement_state,
        eligible: decision.eligible,
        activated: decision.activated,
        invokable: decision.invokable,
        read_only: decision.read_only || read_only,
    }
}
