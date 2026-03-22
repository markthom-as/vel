use vel_core::{
    ActionCapability, ActionContract, ActionErrorKind, AuditRequirement, ConfirmationMode, Grant,
    ModuleCapabilityProfile, ModuleEnablementState, PolicyDecision, PolicyEvaluationInput,
    PolicyLayerKind,
};

use super::policy_evaluator::{default_layer, PolicyEvaluator, PolicyEvaluatorError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModulePolicyBridgeError {
    PolicyDenied(String),
    ConfirmationRequired(String),
    ReadOnlyViolation(String),
    UnsupportedCapability(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePolicyBridgeInput {
    pub module_id: String,
    pub requested_capabilities: ModuleCapabilityProfile,
    pub enabled_feature_gates: Vec<String>,
    pub grant: Grant,
    pub read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePolicyBridgeDecision {
    pub module_id: String,
    pub decision: PolicyDecision,
    pub enablement_state: ModuleEnablementState,
    pub eligible: bool,
    pub activated: bool,
    pub invokable: bool,
    pub read_only: bool,
}

#[derive(Debug, Default)]
pub struct ModulePolicyBridge {
    evaluator: PolicyEvaluator,
}

impl ModulePolicyBridge {
    pub fn evaluate(
        &self,
        input: &ModulePolicyBridgeInput,
    ) -> Result<ModulePolicyBridgeDecision, ModulePolicyBridgeError> {
        for capability in &input.requested_capabilities.requested_capabilities {
            if let Some(feature_gate) = capability.feature_gate.as_deref() {
                if !input
                    .enabled_feature_gates
                    .iter()
                    .any(|enabled| enabled == feature_gate)
                {
                    return Err(ModulePolicyBridgeError::UnsupportedCapability(format!(
                        "unsupported capability {} because feature gate {} is disabled for {}",
                        capability.capability, feature_gate, input.module_id
                    )));
                }
            }

            if !input
                .grant
                .capabilities
                .iter()
                .any(|granted| granted == &capability.capability)
            {
                return Err(ModulePolicyBridgeError::UnsupportedCapability(format!(
                    "unsupported capability {} because grant {} does not allow it for {}",
                    capability.capability, input.grant.id, input.module_id
                )));
            }
        }

        let allows_external_write = input
            .requested_capabilities
            .requested_capabilities
            .iter()
            .any(|capability| !capability.read_only);

        let mut workspace = default_layer(PolicyLayerKind::Workspace);
        workspace.reason = "workspace module activation baseline".to_string();
        workspace.read_only = input.read_only || input.grant.read_only;

        let mut module = default_layer(PolicyLayerKind::Module);
        module.reason = format!("module capability activation for {}", input.module_id);
        module.read_only = input.read_only;

        let decision = self.evaluator.evaluate(&PolicyEvaluationInput {
            action_name: format!("module.activate.{}", input.module_id),
            allows_external_write,
            is_destructive: false,
            is_cross_source: false,
            workspace,
            module,
            integration_account: default_layer(PolicyLayerKind::IntegrationAccount),
            object: default_layer(PolicyLayerKind::Object),
            action: default_layer(PolicyLayerKind::Action),
            execution: default_layer(PolicyLayerKind::Execution),
        });

        match decision {
            Ok(decision) => Ok(ModulePolicyBridgeDecision {
                module_id: input.module_id.clone(),
                decision,
                enablement_state: ModuleEnablementState::Activated,
                eligible: true,
                activated: true,
                invokable: true,
                read_only: input.read_only || input.grant.read_only,
            }),
            Err(PolicyEvaluatorError::PolicyDenied(message)) => {
                Err(ModulePolicyBridgeError::PolicyDenied(message))
            }
            Err(PolicyEvaluatorError::ConfirmationRequired(message)) => {
                Err(ModulePolicyBridgeError::ConfirmationRequired(message))
            }
            Err(PolicyEvaluatorError::ReadOnlyViolation(message)) => {
                Err(ModulePolicyBridgeError::ReadOnlyViolation(message))
            }
        }
    }

    pub fn activation_contract() -> ActionContract {
        ActionContract {
            action_name: "module.activate".to_string(),
            alias_of: None,
            input_schema: "module-capability-profile".to_string(),
            output_schema: "module-activation".to_string(),
            capability: ActionCapability {
                capability: "module.activate".to_string(),
                provider_family: None,
                allows_external_write: false,
            },
            confirmation: ConfirmationMode::Auto,
            audit: AuditRequirement::Required,
            errors: vec![
                ActionErrorKind::PolicyDenied,
                ActionErrorKind::ConfirmationRequired,
                ActionErrorKind::ReadOnlyViolation,
                ActionErrorKind::UnsupportedCapability,
            ],
        }
    }
}
