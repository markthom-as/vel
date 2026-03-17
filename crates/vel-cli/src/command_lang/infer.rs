use crate::command_lang::ast::{ParsedCommand, PhraseFamily, Verb};
use crate::command_lang::parse::parse;
use serde::Serialize;
use serde_json::json;
use vel_core::{
    CommandConfidenceBand, DomainKind, DomainOperation, IntentResolution, ParseMode,
    ResolutionConfidence, ResolutionMeta, ResolvedCommand, TargetSelector, TypedTarget,
};

#[derive(Debug, Clone, Serialize)]
pub struct CommandResolution {
    pub parsed: ParsedCommand,
    pub resolved: ResolvedCommand,
    pub intent: IntentResolution,
}

pub fn parse_and_resolve(input: &[String]) -> anyhow::Result<CommandResolution> {
    let parsed = parse(input)?;
    let mut intent = IntentResolution::default();
    let resolved = resolve(&parsed, &mut intent);
    Ok(CommandResolution {
        parsed,
        resolved,
        intent,
    })
}

fn resolve(parsed: &ParsedCommand, intent: &mut IntentResolution) -> ResolvedCommand {
    let (operation, targets, inferred, assumptions, confidence) =
        match (&parsed.family, &parsed.verb) {
            (PhraseFamily::Should, Verb::Capture) => (
                DomainOperation::Create,
                vec![TypedTarget {
                    kind: DomainKind::Capture,
                    id: None,
                    selector: Some(TargetSelector::Custom("inline_text".to_string())),
                    attributes: json!({
                        "text": parsed.joined_target(),
                        "capture_type": "quick_note"
                    }),
                }],
                json!({
                    "capture_type": "quick_note",
                    "source_device": "vel-command"
                }),
                vec!["capture commands default to capture_type=quick_note".to_string()],
                vec![ResolutionConfidence {
                    field: "capture_type".to_string(),
                    band: CommandConfidenceBand::High,
                }],
            ),
            (PhraseFamily::Should, Verb::Feature) => (
                DomainOperation::Create,
                vec![TypedTarget {
                    kind: DomainKind::Capture,
                    id: None,
                    selector: Some(TargetSelector::Custom("inline_text".to_string())),
                    attributes: json!({
                        "text": parsed.joined_target(),
                        "capture_type": "feature_request"
                    }),
                }],
                json!({
                    "capture_type": "feature_request",
                    "source_device": "vel-command"
                }),
                vec!["feature commands currently compile to a typed capture".to_string()],
                vec![ResolutionConfidence {
                    field: "capture_type".to_string(),
                    band: CommandConfidenceBand::High,
                }],
            ),
            (PhraseFamily::Should, Verb::Review) => (
                DomainOperation::Execute,
                vec![TypedTarget {
                    kind: DomainKind::Context,
                    id: None,
                    selector: parsed
                        .primary_target()
                        .map(|target| TargetSelector::Custom(target.to_string())),
                    attributes: json!({
                        "scope": parsed.primary_target()
                    }),
                }],
                json!({
                    "review_scope": parsed.primary_target()
                }),
                vec!["review commands compile to existing read-oriented review flows".to_string()],
                vec![ResolutionConfidence {
                    field: "review_scope".to_string(),
                    band: CommandConfidenceBand::Medium,
                }],
            ),
            (PhraseFamily::Should, Verb::Synthesize) => (
                DomainOperation::Execute,
                vec![TypedTarget {
                    kind: DomainKind::Artifact,
                    id: None,
                    selector: Some(TargetSelector::Custom("week".to_string())),
                    attributes: json!({
                        "scope": "week"
                    }),
                }],
                json!({
                    "synthesis_scope": "week"
                }),
                vec![
                    "synthesize commands currently resolve to the weekly synthesis flow"
                        .to_string(),
                ],
                vec![ResolutionConfidence {
                    field: "synthesis_scope".to_string(),
                    band: CommandConfidenceBand::High,
                }],
            ),
            (PhraseFamily::Should, Verb::Spec) => (
                DomainOperation::Create,
                vec![TypedTarget {
                    kind: DomainKind::SpecDraft,
                    id: None,
                    selector: Some(TargetSelector::Custom("topic".to_string())),
                    attributes: json!({
                        "topic": parsed.joined_target(),
                        "status": "planned"
                    }),
                }],
                json!({
                    "artifact_kind": "spec_draft",
                    "planning_status": "planned",
                    "topic": parsed.joined_target(),
                    "suggested_path": suggest_spec_path(parsed),
                }),
                vec![
                    "spec commands resolve to planned spec draft intents in this scaffold"
                        .to_string(),
                ],
                vec![
                    ResolutionConfidence {
                        field: "artifact_kind".to_string(),
                        band: CommandConfidenceBand::High,
                    },
                    ResolutionConfidence {
                        field: "planning_status".to_string(),
                        band: CommandConfidenceBand::High,
                    },
                ],
            ),
            (PhraseFamily::Should, Verb::Plan) => (
                DomainOperation::Create,
                vec![TypedTarget {
                    kind: DomainKind::ExecutionPlan,
                    id: None,
                    selector: Some(TargetSelector::Custom("topic".to_string())),
                    attributes: json!({
                        "goal": parsed.joined_target(),
                        "status": "planned"
                    }),
                }],
                json!({
                    "artifact_kind": "execution_plan",
                    "planning_status": "planned",
                    "goal": parsed.joined_target(),
                    "suggested_title": parsed.joined_target(),
                }),
                vec![
                    "plan commands resolve to planned execution-plan intents in this scaffold"
                        .to_string(),
                ],
                vec![
                    ResolutionConfidence {
                        field: "artifact_kind".to_string(),
                        band: CommandConfidenceBand::High,
                    },
                    ResolutionConfidence {
                        field: "planning_status".to_string(),
                        band: CommandConfidenceBand::High,
                    },
                ],
            ),
            (PhraseFamily::Should, Verb::Delegate) => (
                DomainOperation::Create,
                vec![TypedTarget {
                    kind: DomainKind::DelegationPlan,
                    id: None,
                    selector: Some(TargetSelector::Custom("goal".to_string())),
                    attributes: json!({
                        "goal": parsed.joined_target(),
                        "status": "planned"
                    }),
                }],
                json!({
                    "artifact_kind": "delegation_plan",
                    "planning_status": "planned",
                    "goal": parsed.joined_target(),
                    "suggested_title": parsed.joined_target(),
                }),
                vec![
                    "delegate commands resolve to planned delegation-plan intents in this scaffold"
                        .to_string(),
                ],
                vec![
                    ResolutionConfidence {
                        field: "artifact_kind".to_string(),
                        band: CommandConfidenceBand::High,
                    },
                    ResolutionConfidence {
                        field: "planning_status".to_string(),
                        band: CommandConfidenceBand::High,
                    },
                ],
            ),
            (PhraseFamily::Should, Verb::Commit) => (
                DomainOperation::Create,
                vec![TypedTarget {
                    kind: DomainKind::Commitment,
                    id: None,
                    selector: Some(TargetSelector::Custom("inline_text".to_string())),
                    attributes: json!({
                        "text": parsed.joined_target(),
                        "status": "open"
                    }),
                }],
                json!({
                    "status": "open",
                    "source": "vel-command"
                }),
                vec!["commit commands resolve to commitment-create intents".to_string()],
                vec![ResolutionConfidence {
                    field: "status".to_string(),
                    band: CommandConfidenceBand::High,
                }],
            ),
            (PhraseFamily::Should, Verb::Explain) => resolve_explain(parsed),
            _ => (
                DomainOperation::Explain,
                vec![TypedTarget::new(DomainKind::Context)],
                json!({
                    "status": "planned_only"
                }),
                vec![
                    "this verb is recognized but not executable yet in the CLI scaffold"
                        .to_string(),
                ],
                vec![ResolutionConfidence {
                    field: "status".to_string(),
                    band: CommandConfidenceBand::Medium,
                }],
            ),
        };

    intent.explicit = json!({
        "family": parsed.family,
        "verb": parsed.verb,
        "target_tokens": parsed.target_tokens,
    });
    intent.inferred = inferred.clone();
    intent.assumptions = assumptions.clone();
    intent.confidence = confidence;
    intent.requires_confirmation = false;

    ResolvedCommand {
        operation,
        targets,
        inferred,
        assumptions,
        resolution: ResolutionMeta {
            parser: ParseMode::Deterministic,
            model_assisted: false,
            confirmation_required: false,
        },
    }
}

fn resolve_explain(
    parsed: &ParsedCommand,
) -> (
    DomainOperation,
    Vec<TypedTarget>,
    serde_json::Value,
    Vec<String>,
    Vec<ResolutionConfidence>,
) {
    match parsed.primary_target() {
        Some("drift") => (
            DomainOperation::Explain,
            vec![TypedTarget {
                kind: DomainKind::Context,
                id: None,
                selector: Some(TargetSelector::Custom("drift".to_string())),
                attributes: json!({
                    "scope": "drift"
                }),
            }],
            json!({
                "explain_target": "drift"
            }),
            vec!["explain drift commands resolve to the current drift explain surface".to_string()],
            vec![ResolutionConfidence {
                field: "explain_target".to_string(),
                band: CommandConfidenceBand::High,
            }],
        ),
        Some("commitment") => (
            DomainOperation::Explain,
            vec![TypedTarget {
                kind: DomainKind::Commitment,
                id: parsed.target_tokens.get(1).cloned(),
                selector: Some(TargetSelector::Custom("id".to_string())),
                attributes: json!({
                    "scope": "commitment"
                }),
            }],
            json!({
                "explain_target": "commitment",
                "commitment_id": parsed.target_tokens.get(1).cloned()
            }),
            vec![
                "explain commitment commands require a commitment id in the second position"
                    .to_string(),
            ],
            vec![ResolutionConfidence {
                field: "explain_target".to_string(),
                band: CommandConfidenceBand::High,
            }],
        ),
        _ => (
            DomainOperation::Explain,
            vec![TypedTarget {
                kind: DomainKind::Context,
                id: None,
                selector: Some(TargetSelector::Custom("context".to_string())),
                attributes: json!({
                    "scope": "context"
                }),
            }],
            json!({
                "explain_target": "context"
            }),
            vec!["explain commands default to the current context explain surface".to_string()],
            vec![ResolutionConfidence {
                field: "explain_target".to_string(),
                band: CommandConfidenceBand::High,
            }],
        ),
    }
}

fn suggest_spec_path(parsed: &ParsedCommand) -> String {
    let slug = parsed
        .target_tokens
        .iter()
        .map(|token| {
            token
                .chars()
                .map(|ch| {
                    if ch.is_ascii_alphanumeric() {
                        ch.to_ascii_lowercase()
                    } else {
                        '-'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("-")
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if slug.is_empty() {
        "docs/cognitive-agent-architecture/architecture/spec-draft.md".to_string()
    } else {
        format!("docs/cognitive-agent-architecture/architecture/{}.md", slug)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_and_resolve;
    use vel_core::{DomainKind, DomainOperation};

    #[test]
    fn resolves_capture_to_create_capture() {
        let input = vec![
            "should".to_string(),
            "capture".to_string(),
            "test".to_string(),
        ];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Create);
        assert_eq!(resolution.resolved.targets[0].kind, DomainKind::Capture);
    }

    #[test]
    fn resolves_spec_to_spec_draft_intent() {
        let input = vec![
            "should".to_string(),
            "spec".to_string(),
            "cluster".to_string(),
            "sync".to_string(),
        ];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Create);
        assert_eq!(resolution.resolved.targets[0].kind, DomainKind::SpecDraft);
        assert_eq!(
            resolution.resolved.inferred["suggested_path"],
            "docs/cognitive-agent-architecture/architecture/cluster-sync.md"
        );
    }

    #[test]
    fn resolves_plan_to_execution_plan_intent() {
        let input = vec![
            "should".to_string(),
            "plan".to_string(),
            "offline".to_string(),
            "bootstrap".to_string(),
        ];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Create);
        assert_eq!(
            resolution.resolved.targets[0].kind,
            DomainKind::ExecutionPlan
        );
        assert_eq!(
            resolution.resolved.inferred["suggested_title"],
            "offline bootstrap"
        );
    }

    #[test]
    fn resolves_delegate_to_delegation_plan_intent() {
        let input = vec![
            "should".to_string(),
            "delegate".to_string(),
            "review".to_string(),
            "queue".to_string(),
            "cleanup".to_string(),
        ];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Create);
        assert_eq!(
            resolution.resolved.targets[0].kind,
            DomainKind::DelegationPlan
        );
        assert_eq!(
            resolution.resolved.inferred["artifact_kind"],
            "delegation_plan"
        );
    }

    #[test]
    fn resolves_explain_drift_to_context_explain_intent() {
        let input = vec![
            "should".to_string(),
            "explain".to_string(),
            "drift".to_string(),
        ];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Explain);
        assert_eq!(resolution.resolved.targets[0].kind, DomainKind::Context);
        assert_eq!(resolution.resolved.inferred["explain_target"], "drift");
    }

    #[test]
    fn resolves_explain_commitment_to_commitment_explain_intent() {
        let input = vec![
            "should".to_string(),
            "explain".to_string(),
            "commitment".to_string(),
            "cmt_123".to_string(),
        ];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Explain);
        assert_eq!(resolution.resolved.targets[0].kind, DomainKind::Commitment);
        assert_eq!(
            resolution.resolved.targets[0].id.as_deref(),
            Some("cmt_123")
        );
        assert_eq!(resolution.resolved.inferred["explain_target"], "commitment");
    }

    #[test]
    fn resolves_synthesize_to_weekly_synthesis_intent() {
        let input = vec![
            "should".to_string(),
            "synthesize".to_string(),
            "week".to_string(),
        ];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Execute);
        assert_eq!(resolution.resolved.targets[0].kind, DomainKind::Artifact);
        assert_eq!(resolution.resolved.inferred["synthesis_scope"], "week");
    }
}
