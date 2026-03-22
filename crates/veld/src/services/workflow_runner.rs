use std::collections::BTreeMap;

use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{
    ApprovalRecord, ApprovalRequired, ApprovalStatus, GrantEnvelope, RegistryObject, RunId,
    RunRecord, SkillInvocation, SkillInvocationMode, WorkflowContext, WorkflowRunStatus,
    WorkflowStep,
};
use vel_storage::{insert_runtime_record, RuntimeRecord};

use crate::errors::AppError;

use super::{
    object_actions::{execute_object_explain, execute_object_get, ObjectGetInput},
    skill_invocation::SkillInvocationService,
    workflow_context_binding::{BoundWorkflowContext, ContextBinding},
};

#[derive(Debug, Clone)]
pub struct ManualWorkflowInvocationRequest {
    pub workflow_id: String,
    pub context: WorkflowContext,
    pub steps: Vec<WorkflowStep>,
    pub dry_run: bool,
    pub module_registry_objects: BTreeMap<String, RegistryObject>,
    pub skill_registry_objects: BTreeMap<String, RegistryObject>,
    pub grant_envelopes: BTreeMap<String, GrantEnvelope>,
    pub enabled_feature_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowRunOutcome {
    pub run_id: RunId,
    pub status: WorkflowRunStatus,
    pub approval_required: Option<ApprovalRequired>,
}

#[derive(Debug, Default)]
pub struct WorkflowRunner {
    skill_invocation_service: SkillInvocationService,
}

impl WorkflowRunner {
    pub async fn run_manual(
        &self,
        pool: &SqlitePool,
        request: &ManualWorkflowInvocationRequest,
    ) -> Result<WorkflowRunOutcome, AppError> {
        if request.workflow_id.trim().is_empty() {
            return Err(AppError::bad_request("manual workflow invocation missing workflow_id"));
        }
        if request.steps.is_empty() {
            return Err(AppError::bad_request("manual workflow invocation requires at least one step"));
        }

        let run_id = RunId::new();
        persist_run_record(
            pool,
            &RunRecord {
                run_id: run_id.clone(),
                workflow_id: request.workflow_id.clone(),
                status: WorkflowRunStatus::Created,
                dry_run: request.dry_run,
                current_step_id: None,
                reason: Some("manual invocation created".to_string()),
            },
        )
        .await?;

        let bound_context = ContextBinding::bind(pool, &request.context).await?;
        for step in &request.steps {
            step.validate().map_err(AppError::bad_request)?;
        }

        persist_run_record(
            pool,
            &RunRecord {
                run_id: run_id.clone(),
                workflow_id: request.workflow_id.clone(),
                status: WorkflowRunStatus::Ready,
                dry_run: request.dry_run,
                current_step_id: None,
                reason: Some("typed context binding and step validation passed".to_string()),
            },
        )
        .await?;

        for step in &request.steps {
            persist_run_record(
                pool,
                &RunRecord {
                    run_id: run_id.clone(),
                    workflow_id: request.workflow_id.clone(),
                    status: WorkflowRunStatus::Running,
                    dry_run: request.dry_run,
                    current_step_id: Some(step_id(step)?.to_string()),
                    reason: Some("manual workflow execution running".to_string()),
                },
            )
            .await?;

            match step {
                WorkflowStep::Action(action) => {
                    execute_action_step(pool, &bound_context, &action.action_name).await?;
                }
                WorkflowStep::Skill(skill) => {
                    let module_id = request
                        .grant_envelopes
                        .get(&skill.skill_id)
                        .map(|grant| grant.module_id.clone())
                        .ok_or_else(|| {
                            AppError::bad_request(format!(
                                "missing grant envelope for skill {}",
                                skill.skill_id
                            ))
                        })?;
                    let module_registry_object = request.module_registry_objects.get(&module_id).ok_or_else(
                        || AppError::bad_request(format!("missing module registry object {module_id}")),
                    )?;
                    let skill_registry_object = request.skill_registry_objects.get(&skill.skill_id).ok_or_else(
                        || AppError::bad_request(format!("missing skill registry object {}", skill.skill_id)),
                    )?;
                    let grant_envelope = request.grant_envelopes.get(&skill.skill_id).ok_or_else(|| {
                        AppError::bad_request(format!("missing grant envelope for skill {}", skill.skill_id))
                    })?;

                    let invocation = SkillInvocation {
                        workflow_id: request.workflow_id.clone(),
                        module_id,
                        skill_id: skill.skill_id.clone(),
                        action_name: skill_action_name(&skill.skill_id)?,
                        target_object_refs: primary_object_refs(&bound_context),
                        dry_run: request.dry_run,
                        input_json: json!({"run_record":"manual"}),
                        mode: SkillInvocationMode::Mediated,
                    };

                    match self
                        .skill_invocation_service
                        .invoke(
                            pool,
                            module_registry_object,
                            skill_registry_object,
                            grant_envelope,
                            &invocation,
                            request.enabled_feature_gates.clone(),
                        )
                        .await
                    {
                        Ok(_) => {}
                        Err(error) if request.dry_run && error.to_string().contains("ConfirmationRequired") => {
                            persist_run_record(
                                pool,
                                &RunRecord {
                                    run_id: run_id.clone(),
                                    workflow_id: request.workflow_id.clone(),
                                    status: WorkflowRunStatus::DryRunComplete,
                                    dry_run: true,
                                    current_step_id: Some(skill.step_id.clone()),
                                    reason: Some(
                                        "dry_run recorded mediated approval requirement without external mutation"
                                            .to_string(),
                                    ),
                                },
                            )
                            .await?;

                            return Ok(WorkflowRunOutcome {
                                run_id,
                                status: WorkflowRunStatus::DryRunComplete,
                                approval_required: None,
                            });
                        }
                        Err(error) => {
                            persist_run_record(
                                pool,
                                &RunRecord {
                                    run_id: run_id.clone(),
                                    workflow_id: request.workflow_id.clone(),
                                    status: WorkflowRunStatus::Refused,
                                    dry_run: request.dry_run,
                                    current_step_id: Some(skill.step_id.clone()),
                                    reason: Some(error.to_string()),
                                },
                            )
                            .await?;
                            return Err(error);
                        }
                    }
                }
                WorkflowStep::Approval(step) => {
                    let approval_id = format!("approval_{}", Uuid::new_v4().simple());
                    persist_approval_record(
                        pool,
                        &ApprovalRecord {
                            approval_id: approval_id.clone(),
                            run_id: run_id.clone(),
                            workflow_id: request.workflow_id.clone(),
                            step_id: step.step_id.clone(),
                            approval_key: step.approval_key.clone(),
                            status: ApprovalStatus::Pending,
                        },
                    )
                    .await?;
                    persist_run_record(
                        pool,
                        &RunRecord {
                            run_id: run_id.clone(),
                            workflow_id: request.workflow_id.clone(),
                            status: WorkflowRunStatus::AwaitingApproval,
                            dry_run: request.dry_run,
                            current_step_id: Some(step.step_id.clone()),
                            reason: Some("approval step paused manual workflow execution".to_string()),
                        },
                    )
                    .await?;

                    return Ok(WorkflowRunOutcome {
                        run_id,
                        status: WorkflowRunStatus::AwaitingApproval,
                        approval_required: Some(ApprovalRequired {
                            approval_id,
                            step_id: step.step_id.clone(),
                            approval_key: step.approval_key.clone(),
                        }),
                    });
                }
                WorkflowStep::Sync(step) => {
                    persist_run_record(
                        pool,
                        &RunRecord {
                            run_id: run_id.clone(),
                            workflow_id: request.workflow_id.clone(),
                            status: WorkflowRunStatus::Failed,
                            dry_run: request.dry_run,
                            current_step_id: Some(step.step_id.clone()),
                            reason: Some(
                                "sync step execution remains deferred beyond minimal manual runtime"
                                    .to_string(),
                            ),
                        },
                    )
                    .await?;
                    return Err(AppError::bad_request(
                        "sync step execution remains deferred in the minimal manual workflow runner",
                    ));
                }
                WorkflowStep::Condition(step) => {
                    let condition_true = evaluate_condition(&bound_context, &step.condition);
                    if !condition_true {
                        persist_run_record(
                            pool,
                            &RunRecord {
                                run_id: run_id.clone(),
                                workflow_id: request.workflow_id.clone(),
                                status: WorkflowRunStatus::Refused,
                                dry_run: request.dry_run,
                                current_step_id: Some(step.step_id.clone()),
                                reason: Some("condition step refused manual workflow progression".to_string()),
                            },
                        )
                        .await?;
                        return Ok(WorkflowRunOutcome {
                            run_id,
                            status: WorkflowRunStatus::Refused,
                            approval_required: None,
                        });
                    }
                }
            }
        }

        let terminal_status = if request.dry_run {
            WorkflowRunStatus::DryRunComplete
        } else {
            WorkflowRunStatus::Completed
        };
        persist_run_record(
            pool,
            &RunRecord {
                run_id: run_id.clone(),
                workflow_id: request.workflow_id.clone(),
                status: terminal_status.clone(),
                dry_run: request.dry_run,
                current_step_id: None,
                reason: Some(if request.dry_run {
                    "dry_run completed with no canonical mutation and no external mutation".to_string()
                } else {
                    "manual workflow completed".to_string()
                }),
            },
        )
        .await?;

        Ok(WorkflowRunOutcome {
            run_id,
            status: terminal_status,
            approval_required: None,
        })
    }
}

async fn execute_action_step(
    pool: &SqlitePool,
    bound_context: &BoundWorkflowContext,
    action_name: &str,
) -> Result<(), AppError> {
    let object_id = primary_object_ref(bound_context)?;
    match action_name {
        "object.get" => {
            execute_object_get(pool, &ObjectGetInput { object_id }).await?;
            Ok(())
        }
        "object.explain" => {
            execute_object_explain(pool, &ObjectGetInput { object_id }).await?;
            Ok(())
        }
        _ => Err(AppError::bad_request(format!(
            "manual workflow runner does not yet support action {}",
            action_name
        ))),
    }
}

fn skill_action_name(skill_id: &str) -> Result<String, AppError> {
    match skill_id {
        "skill.core.daily-brief" => Ok("object.explain".to_string()),
        _ => Err(AppError::bad_request(format!(
            "manual workflow runner has no mediated action mapping for skill {}",
            skill_id
        ))),
    }
}

fn primary_object_ref(bound_context: &BoundWorkflowContext) -> Result<String, AppError> {
    bound_context
        .canonical_objects
        .values()
        .next()
        .map(|record| record.id.clone())
        .ok_or_else(|| AppError::bad_request("manual workflow runner requires a canonical object binding"))
}

fn primary_object_refs(bound_context: &BoundWorkflowContext) -> Vec<String> {
    bound_context
        .canonical_objects
        .values()
        .next()
        .map(|record| vec![record.id.clone()])
        .unwrap_or_default()
}

fn evaluate_condition(bound_context: &BoundWorkflowContext, condition: &str) -> bool {
    if condition == "true" {
        return true;
    }

    if let Some((binding, value)) = condition.split_once(" == ") {
        if let Some((binding_name, field)) = binding.split_once('.') {
            if field == "status" {
                if let Some(record) = bound_context.canonical_objects.get(binding_name) {
                    return value == record.status;
                }
            }
        }
    }

    false
}

fn step_id(step: &WorkflowStep) -> Result<&str, AppError> {
    Ok(match step {
        WorkflowStep::Action(step) => &step.step_id,
        WorkflowStep::Skill(step) => &step.step_id,
        WorkflowStep::Approval(step) => &step.step_id,
        WorkflowStep::Sync(step) => &step.step_id,
        WorkflowStep::Condition(step) => &step.step_id,
    })
}

async fn persist_run_record(pool: &SqlitePool, run_record: &RunRecord) -> Result<(), AppError> {
    run_record.validate().map_err(AppError::bad_request)?;

    let now = OffsetDateTime::now_utc();
    insert_runtime_record(
        pool,
        &RuntimeRecord {
            id: format!(
                "{}__{}_{}",
                run_record.run_id,
                run_status_rank(&run_record.status),
                run_status_label(&run_record.status)
            ),
            record_type: "run".to_string(),
            object_ref: None,
            status: run_status_label(&run_record.status).to_string(),
            payload_json: serde_json::to_value(run_record)
                .map_err(|error| AppError::internal(error.to_string()))?,
            created_at: now,
            updated_at: now,
        },
    )
    .await?;

    Ok(())
}

async fn persist_approval_record(
    pool: &SqlitePool,
    approval_record: &ApprovalRecord,
) -> Result<(), AppError> {
    let now = OffsetDateTime::now_utc();
    insert_runtime_record(
        pool,
        &RuntimeRecord {
            id: approval_record.approval_id.clone(),
            record_type: "approval".to_string(),
            object_ref: None,
            status: approval_status_label(&approval_record.status).to_string(),
            payload_json: serde_json::to_value(approval_record)
                .map_err(|error| AppError::internal(error.to_string()))?,
            created_at: now,
            updated_at: now,
        },
    )
    .await?;

    Ok(())
}

fn run_status_label(status: &WorkflowRunStatus) -> &'static str {
    match status {
        WorkflowRunStatus::Created => "created",
        WorkflowRunStatus::Ready => "ready",
        WorkflowRunStatus::Running => "running",
        WorkflowRunStatus::AwaitingApproval => "awaiting_approval",
        WorkflowRunStatus::DryRunComplete => "dry_run_complete",
        WorkflowRunStatus::Completed => "completed",
        WorkflowRunStatus::Failed => "failed",
        WorkflowRunStatus::Refused => "refused",
        WorkflowRunStatus::Cancelled => "cancelled",
    }
}

fn run_status_rank(status: &WorkflowRunStatus) -> &'static str {
    match status {
        WorkflowRunStatus::Created => "01",
        WorkflowRunStatus::Ready => "02",
        WorkflowRunStatus::Running => "03",
        WorkflowRunStatus::AwaitingApproval => "04",
        WorkflowRunStatus::DryRunComplete => "05",
        WorkflowRunStatus::Completed => "06",
        WorkflowRunStatus::Failed => "07",
        WorkflowRunStatus::Refused => "08",
        WorkflowRunStatus::Cancelled => "09",
    }
}

fn approval_status_label(status: &ApprovalStatus) -> &'static str {
    match status {
        ApprovalStatus::Pending => "pending",
        ApprovalStatus::Approved => "approved",
        ApprovalStatus::Rejected => "rejected",
    }
}
