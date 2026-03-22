use sqlx::SqlitePool;
use vel_core::{
    generic_object_action_contracts, ActionContract, AuditEventKind, AuditRecord,
    AuditRequirement, GrantEnvelope, GrantRequest, PolicyEvaluationInput, PolicyLayerKind,
    RegistryKind, RegistryObject, SkillInvocation, SkillInvocationOutcome,
};

use crate::errors::AppError;

use super::{
    audit_emitter::AuditEmitter,
    grant_resolver::{GrantResolver, GrantResolverError},
    module_activation::{ModuleActivationRequest, ModuleActivationService},
    module_policy_bridge::ModulePolicyBridgeError,
    policy_evaluator::{default_layer, PolicyEvaluator, PolicyEvaluatorError},
};

#[derive(Debug, Default)]
pub struct SkillInvocationService {
    activation_service: ModuleActivationService,
    grant_resolver: GrantResolver,
    policy_evaluator: PolicyEvaluator,
    audit_emitter: AuditEmitter,
}

impl SkillInvocationService {
    pub async fn invoke(
        &self,
        pool: &SqlitePool,
        module_registry_object: &RegistryObject,
        skill_registry_object: &RegistryObject,
        grant_envelope: &GrantEnvelope,
        invocation: &SkillInvocation,
        enabled_feature_gates: Vec<String>,
    ) -> Result<SkillInvocationOutcome, AppError> {
        grant_envelope.validate().map_err(AppError::bad_request)?;
        invocation.validate().map_err(AppError::bad_request)?;
        validate_registry_pair(module_registry_object, skill_registry_object, invocation)?;

        let effective_grant = grant_envelope.effective_grant();
        let activation = match self.activation_service.activate(&ModuleActivationRequest {
                registry_object: module_registry_object.clone(),
                enabled_feature_gates,
                grant: effective_grant.clone(),
                read_only: grant_envelope.read_only,
            }) {
            Ok(activation) => activation,
            Err(error) => {
                let app_error = map_module_activation_error(error);
                let audit = denied_audit(invocation, &app_error.to_string(), None);
                self.audit_emitter.emit(pool, &audit).await?;
                return Err(app_error);
            }
        };

        if !activation.invokable {
            let reason = format!(
                "PolicyDenied module {} is not invokable for mediated skill invocation",
                activation.module_id
            );
            let audit = denied_audit(invocation, &reason, None);
            self.audit_emitter.emit(pool, &audit).await?;
            return Err(AppError::forbidden(reason));
        }

        let action_contract = action_contract_for(&invocation.action_name).ok_or_else(|| {
            AppError::bad_request(format!(
                "unsupported mediated skill action {}",
                invocation.action_name
            ))
        })?;

        let narrowed_grant = match self.grant_resolver.resolve(
                &effective_grant,
                &GrantRequest {
                    action_name: invocation.action_name.clone(),
                    capability: action_contract.capability.capability.clone(),
                    object_ids: invocation.target_object_refs.clone(),
                    durable: false,
                    run_scoped: true,
                },
            ) {
            Ok(grant) => grant,
            Err(error) => {
                let app_error = map_grant_error(error);
                let audit = denied_audit(invocation, &app_error.to_string(), None);
                self.audit_emitter.emit(pool, &audit).await?;
                return Err(app_error);
            }
        };

        let action_policy = action_policy_layer(&action_contract);
        let mut module_policy = default_layer(PolicyLayerKind::Module);
        module_policy.reason = format!(
            "mediated skill invocation via {} in {}",
            invocation.skill_id, invocation.module_id
        );
        module_policy.read_only = activation.read_only;

        let mut execution_policy = default_layer(PolicyLayerKind::Execution);
        execution_policy.reason = format!(
            "mediated skill invocation run record placeholder for {}",
            invocation.skill_id
        );

        let decision = self.policy_evaluator.evaluate(&PolicyEvaluationInput {
            action_name: invocation.action_name.clone(),
            allows_external_write: action_contract.capability.allows_external_write,
            is_destructive: matches!(invocation.action_name.as_str(), "object.delete"),
            is_cross_source: false,
            workspace: {
                let mut layer = default_layer(PolicyLayerKind::Workspace);
                layer.read_only = grant_envelope.read_only || narrowed_grant.read_only;
                layer.reason = "workflow caller grant envelope baseline".to_string();
                layer
            },
            module: module_policy,
            integration_account: default_layer(PolicyLayerKind::IntegrationAccount),
            object: default_layer(PolicyLayerKind::Object),
            action: action_policy,
            execution: execution_policy,
        });

        match decision {
            Ok(policy_decision) => {
                let audit_required =
                    matches!(audit_requirement(&action_contract), AuditRequirement::Required);
                let audit = AuditRecord {
                    action_name: invocation.action_name.clone(),
                    target_object_refs: invocation.target_object_refs.clone(),
                    dry_run: invocation.dry_run,
                    approval_required: false,
                    outcome: if invocation.dry_run {
                        AuditEventKind::DryRun
                    } else {
                        AuditEventKind::Allowed
                    },
                    reason: "mediated skill invocation passed module activation and action membrane"
                        .to_string(),
                    field_captures: vec![],
                    write_intent_ref: None,
                    downstream_operation_ref: None,
                };
                let runtime_audit = self.audit_emitter.emit(pool, &audit).await?;

                Ok(SkillInvocationOutcome {
                    invocation: invocation.clone(),
                    grant_envelope: grant_envelope.clone(),
                    effective_grant: narrowed_grant,
                    action_contract,
                    confirmation: policy_decision.confirmation,
                    audit_required,
                    mediated: true,
                    audit_record_ref: Some(runtime_audit.id),
                    run_record_ref: None,
                })
            }
            Err(PolicyEvaluatorError::PolicyDenied(message)) => {
                let reason = format!("PolicyDenied {message}");
                let runtime_audit = self
                    .audit_emitter
                    .emit(pool, &denied_audit(invocation, &reason, None))
                    .await?;
                Err(AppError::forbidden(format!(
                    "{reason} (audit {})",
                    runtime_audit.id
                )))
            }
            Err(PolicyEvaluatorError::ConfirmationRequired(message)) => {
                let runtime_audit = self
                    .audit_emitter
                    .emit(
                        pool,
                        &AuditRecord {
                            action_name: invocation.action_name.clone(),
                            target_object_refs: invocation.target_object_refs.clone(),
                            dry_run: invocation.dry_run,
                            approval_required: true,
                            outcome: AuditEventKind::ApprovalRequired,
                            reason: format!("ConfirmationRequired {message}"),
                            field_captures: vec![],
                            write_intent_ref: None,
                            downstream_operation_ref: None,
                        },
                    )
                    .await?;
                Err(AppError::forbidden(format!(
                    "ConfirmationRequired {message} (audit {})",
                    runtime_audit.id
                )))
            }
            Err(PolicyEvaluatorError::ReadOnlyViolation(message)) => {
                let reason = format!("ReadOnlyViolation {message}");
                let runtime_audit = self
                    .audit_emitter
                    .emit(pool, &denied_audit(invocation, &reason, None))
                    .await?;
                Err(AppError::forbidden(format!(
                    "{reason} (audit {})",
                    runtime_audit.id
                )))
            }
        }
    }
}

fn validate_registry_pair(
    module_registry_object: &RegistryObject,
    skill_registry_object: &RegistryObject,
    invocation: &SkillInvocation,
) -> Result<(), AppError> {
    if module_registry_object.registry_kind != RegistryKind::Module {
        return Err(AppError::bad_request(format!(
            "{} is not a module registry object",
            module_registry_object.id
        )));
    }
    if skill_registry_object.registry_kind != RegistryKind::Skill {
        return Err(AppError::bad_request(format!(
            "{} is not a skill registry object",
            skill_registry_object.id
        )));
    }
    if module_registry_object.id != invocation.module_id {
        return Err(AppError::bad_request(format!(
            "skill invocation module {} did not match registry object {}",
            invocation.module_id, module_registry_object.id
        )));
    }
    if skill_registry_object.id != invocation.skill_id {
        return Err(AppError::bad_request(format!(
            "skill invocation skill {} did not match registry object {}",
            invocation.skill_id, skill_registry_object.id
        )));
    }

    Ok(())
}

fn action_contract_for(action_name: &str) -> Option<ActionContract> {
    generic_object_action_contracts()
        .into_iter()
        .find(|contract| contract.action_name == action_name)
}

fn action_policy_layer(action_contract: &ActionContract) -> vel_core::PolicyLayerDecision {
    let mut layer = default_layer(PolicyLayerKind::Action);
    layer.reason = format!(
        "action membrane mediated skill invocation for {}",
        action_contract.action_name
    );
    layer.confirmation = action_contract.confirmation.clone();
    layer
}

fn audit_requirement(action_contract: &ActionContract) -> AuditRequirement {
    action_contract.audit.clone()
}

fn denied_audit(
    invocation: &SkillInvocation,
    reason: &str,
    downstream_operation_ref: Option<String>,
) -> AuditRecord {
    AuditRecord {
        action_name: invocation.action_name.clone(),
        target_object_refs: invocation.target_object_refs.clone(),
        dry_run: invocation.dry_run,
        approval_required: false,
        outcome: AuditEventKind::Denied,
        reason: reason.to_string(),
        field_captures: vec![],
        write_intent_ref: None,
        downstream_operation_ref,
    }
}

fn map_module_activation_error(error: ModulePolicyBridgeError) -> AppError {
    match error {
        ModulePolicyBridgeError::PolicyDenied(message) => {
            AppError::forbidden(format!("PolicyDenied {message}"))
        }
        ModulePolicyBridgeError::ConfirmationRequired(message) => {
            AppError::forbidden(format!("ConfirmationRequired {message}"))
        }
        ModulePolicyBridgeError::ReadOnlyViolation(message) => {
            AppError::forbidden(format!("ReadOnlyViolation {message}"))
        }
        ModulePolicyBridgeError::UnsupportedCapability(message) => AppError::bad_request(message),
    }
}

fn map_grant_error(error: GrantResolverError) -> AppError {
    match error {
        GrantResolverError::GrantMissing(message) => AppError::forbidden(message),
    }
}

#[cfg(test)]
mod tests {
    use super::action_contract_for;

    #[test]
    fn skill_runtime_only_exposes_generic_object_actions_through_mediated_lookup() {
        assert!(action_contract_for("object.get").is_some());
        assert!(action_contract_for("tool.object.get").is_none());
    }
}
