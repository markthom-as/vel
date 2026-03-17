//! Command-language service scaffolding.
//!
//! This module intentionally provides a pure validation/planning layer over
//! `vel_core::ResolvedCommand` and does not execute side effects yet.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use vel_core::{DomainKind, DomainOperation, ResolvedCommand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandPlanMode {
    Ready,
    DryRunOnly,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationIssueCode {
    UnsupportedOperation,
    MissingTargets,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandValidationIssue {
    pub code: ValidationIssueCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CommandValidation {
    pub is_valid: bool,
    pub issues: Vec<CommandValidationIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPlanStep {
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExecutionPlan {
    pub operation: DomainOperation,
    pub target_kinds: Vec<DomainKind>,
    pub mode: CommandPlanMode,
    pub summary: String,
    pub steps: Vec<CommandPlanStep>,
    pub validation: CommandValidation,
}

pub fn build_execution_plan(command: &ResolvedCommand) -> CommandExecutionPlan {
    let validation = validate_command(command);
    let mode = if !validation.is_valid {
        CommandPlanMode::Unsupported
    } else if is_dry_run_only(command.operation) {
        CommandPlanMode::DryRunOnly
    } else {
        CommandPlanMode::Ready
    };

    let target_kinds = command.targets.iter().map(|t| t.kind).collect::<Vec<_>>();
    let steps = build_plan_steps(command);
    let summary = build_summary(command, mode, target_kinds.len());

    CommandExecutionPlan {
        operation: command.operation,
        target_kinds,
        mode,
        summary,
        steps,
        validation,
    }
}

pub fn validate_command(command: &ResolvedCommand) -> CommandValidation {
    let mut issues = Vec::new();

    if !is_supported_operation(command.operation) {
        issues.push(CommandValidationIssue {
            code: ValidationIssueCode::UnsupportedOperation,
            message: format!(
                "operation `{}` is not supported by the command-language service scaffold",
                command.operation
            ),
        });
    }

    if requires_targets(command.operation) && command.targets.is_empty() {
        issues.push(CommandValidationIssue {
            code: ValidationIssueCode::MissingTargets,
            message: format!("operation `{}` requires at least one target", command.operation),
        });
    }

    CommandValidation {
        is_valid: issues.is_empty(),
        issues,
    }
}

fn build_plan_steps(command: &ResolvedCommand) -> Vec<CommandPlanStep> {
    let mut steps = vec![CommandPlanStep {
        title: "Validate command".to_string(),
        detail: format!("Check operation `{}` and target shape", command.operation),
    }];

    steps.push(CommandPlanStep {
        title: "Resolve target mapping".to_string(),
        detail: format!("Resolve {} target(s) to service domain calls", command.targets.len()),
    });

    if is_dry_run_only(command.operation) {
        steps.push(CommandPlanStep {
            title: "Dry-run summary only".to_string(),
            detail: "No side effects in scaffold mode".to_string(),
        });
    } else {
        steps.push(CommandPlanStep {
            title: "Execute via service adapter".to_string(),
            detail: "Hook to be implemented by future command route/CLI integration".to_string(),
        });
    }

    steps
}

fn build_summary(command: &ResolvedCommand, mode: CommandPlanMode, target_count: usize) -> String {
    format!(
        "operation={} targets={} mode={}",
        command.operation,
        target_count,
        match mode {
            CommandPlanMode::Ready => "ready",
            CommandPlanMode::DryRunOnly => "dry_run_only",
            CommandPlanMode::Unsupported => "unsupported",
        }
    )
}

fn requires_targets(operation: DomainOperation) -> bool {
    matches!(
        operation,
        DomainOperation::Create
            | DomainOperation::Inspect
            | DomainOperation::Update
            | DomainOperation::Link
            | DomainOperation::Execute
    )
}

fn is_supported_operation(operation: DomainOperation) -> bool {
    matches!(
        operation,
        DomainOperation::Create
            | DomainOperation::Inspect
            | DomainOperation::List
            | DomainOperation::Explain
            | DomainOperation::Execute
    )
}

fn is_dry_run_only(operation: DomainOperation) -> bool {
    matches!(operation, DomainOperation::Execute)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vel_core::{TypedTarget, TargetSelector};

    #[test]
    fn validate_create_with_target_is_valid() {
        let command = ResolvedCommand {
            operation: DomainOperation::Create,
            targets: vec![TypedTarget {
                kind: DomainKind::Capture,
                id: None,
                selector: Some(TargetSelector::Custom("inline_text".to_string())),
                attributes: serde_json::json!({"text":"hello"}),
            }],
            ..ResolvedCommand::default()
        };

        let validation = validate_command(&command);
        assert!(validation.is_valid);
        assert!(validation.issues.is_empty());
    }

    #[test]
    fn validate_update_is_unsupported() {
        let command = ResolvedCommand {
            operation: DomainOperation::Update,
            targets: vec![TypedTarget::new(DomainKind::Capture)],
            ..ResolvedCommand::default()
        };

        let validation = validate_command(&command);
        assert!(!validation.is_valid);
        assert!(
            validation
                .issues
                .iter()
                .any(|issue| issue.code == ValidationIssueCode::UnsupportedOperation)
        );
    }

    #[test]
    fn build_plan_for_execute_is_dry_run_only() {
        let command = ResolvedCommand {
            operation: DomainOperation::Execute,
            targets: vec![TypedTarget::new(DomainKind::Context)],
            ..ResolvedCommand::default()
        };

        let plan = build_execution_plan(&command);
        assert_eq!(plan.mode, CommandPlanMode::DryRunOnly);
        assert!(plan.validation.is_valid);
        assert!(plan.summary.contains("dry_run_only"));
    }
}
