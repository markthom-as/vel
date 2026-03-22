use vel_core::{
    ConfirmationMode, PolicyDecision, PolicyDecisionKind, PolicyEvaluationInput,
    PolicyLayerDecision, PolicyLayerKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEvaluatorError {
    PolicyDenied(String),
    ConfirmationRequired(String),
    ReadOnlyViolation(String),
}

#[derive(Debug, Default)]
pub struct PolicyEvaluator;

impl PolicyEvaluator {
    pub fn evaluate(&self, input: &PolicyEvaluationInput) -> Result<PolicyDecision, PolicyEvaluatorError> {
        let layers = [
            &input.workspace,
            &input.module,
            &input.integration_account,
            &input.object,
            &input.action,
            &input.execution,
        ];
        let reasons = layers.iter().map(|layer| layer.reason.clone()).collect::<Vec<_>>();

        if input.allows_external_write && layers.iter().any(|layer| layer.read_only) {
            return Err(PolicyEvaluatorError::ReadOnlyViolation(format!(
                "workspace/module/integration account/object/action/execution precedence denied external write for {}",
                input.action_name
            )));
        }

        let effective_confirmation = layers.iter().fold(
            ConfirmationMode::Auto,
            |current, layer| more_restrictive_confirmation(current, layer.confirmation.clone()),
        );

        match effective_confirmation {
            ConfirmationMode::Deny => Err(PolicyEvaluatorError::PolicyDenied(format!(
                "policy denied {} after precedence evaluation",
                input.action_name
            ))),
            ConfirmationMode::Ask
            | ConfirmationMode::AskIfCrossSource
            | ConfirmationMode::AskIfDestructive
            | ConfirmationMode::AskIfExternalWrite => Err(
                PolicyEvaluatorError::ConfirmationRequired(format!(
                    "confirmation required for {} with mode {:?}",
                    input.action_name, effective_confirmation
                )),
            ),
            ConfirmationMode::Auto => Ok(PolicyDecision {
                kind: PolicyDecisionKind::Allowed,
                confirmation: ConfirmationMode::Auto,
                read_only: false,
                reasons,
            }),
        }
    }
}

pub fn default_layer(layer: PolicyLayerKind) -> PolicyLayerDecision {
    PolicyLayerDecision {
        layer,
        read_only: false,
        confirmation: ConfirmationMode::Auto,
        reason: "default allow".to_string(),
    }
}

fn more_restrictive_confirmation(
    left: ConfirmationMode,
    right: ConfirmationMode,
) -> ConfirmationMode {
    use ConfirmationMode::{
        Ask, AskIfCrossSource, AskIfDestructive, AskIfExternalWrite, Auto, Deny,
    };

    let rank = |mode: &ConfirmationMode| match mode {
        Auto => 0,
        Ask => 1,
        AskIfDestructive => 2,
        AskIfCrossSource => 3,
        AskIfExternalWrite => 4,
        Deny => 5,
    };

    if rank(&right) > rank(&left) {
        right
    } else {
        left
    }
}

#[cfg(test)]
mod tests {
    use super::{default_layer, PolicyEvaluator, PolicyEvaluatorError};
    use vel_core::{ConfirmationMode, PolicyEvaluationInput, PolicyLayerKind};

    #[test]
    fn policy_evaluator_applies_workspace_module_integration_account_object_action_execution_precedence() {
        let evaluator = PolicyEvaluator;
        let mut module = default_layer(PolicyLayerKind::Module);
        module.confirmation = ConfirmationMode::AskIfExternalWrite;
        module.reason = "module asks before external writes".to_string();

        let decision = evaluator.evaluate(&PolicyEvaluationInput {
            action_name: "object.update".to_string(),
            allows_external_write: false,
            is_destructive: false,
            is_cross_source: false,
            workspace: default_layer(PolicyLayerKind::Workspace),
            module,
            integration_account: default_layer(PolicyLayerKind::IntegrationAccount),
            object: default_layer(PolicyLayerKind::Object),
            action: default_layer(PolicyLayerKind::Action),
            execution: default_layer(PolicyLayerKind::Execution),
        });

        assert!(matches!(
            decision,
            Err(PolicyEvaluatorError::ConfirmationRequired(_))
        ));
    }

    #[test]
    fn policy_evaluator_enforces_read_only_violation_before_allowing_external_write() {
        let evaluator = PolicyEvaluator;
        let mut workspace = default_layer(PolicyLayerKind::Workspace);
        workspace.read_only = true;
        workspace.reason = "workspace read-only".to_string();

        let decision = evaluator.evaluate(&PolicyEvaluationInput {
            action_name: "object.update".to_string(),
            allows_external_write: true,
            is_destructive: false,
            is_cross_source: false,
            workspace,
            module: default_layer(PolicyLayerKind::Module),
            integration_account: default_layer(PolicyLayerKind::IntegrationAccount),
            object: default_layer(PolicyLayerKind::Object),
            action: default_layer(PolicyLayerKind::Action),
            execution: default_layer(PolicyLayerKind::Execution),
        });

        assert!(matches!(
            decision,
            Err(PolicyEvaluatorError::ReadOnlyViolation(_))
        ));
    }
}

