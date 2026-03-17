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
    let (operation, targets, inferred, assumptions, confidence) = match (&parsed.family, &parsed.verb)
    {
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
        _ => (
            DomainOperation::Explain,
            vec![TypedTarget::new(DomainKind::Context)],
            json!({
                "status": "planned_only"
            }),
            vec!["this verb is recognized but not executable yet in the CLI scaffold".to_string()],
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

#[cfg(test)]
mod tests {
    use super::parse_and_resolve;
    use vel_core::{DomainKind, DomainOperation};

    #[test]
    fn resolves_capture_to_create_capture() {
        let input = vec!["should".to_string(), "capture".to_string(), "test".to_string()];
        let resolution = parse_and_resolve(&input).expect("resolve");
        assert_eq!(resolution.resolved.operation, DomainOperation::Create);
        assert_eq!(resolution.resolved.targets[0].kind, DomainKind::Capture);
    }
}
