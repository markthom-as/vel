use crate::command_lang::infer::CommandResolution;
use serde::Serialize;
use vel_core::DomainKind;
use vel_core::SHOULD_COMMAND_VERBS;

pub fn next_tokens(input: &[String]) -> Vec<&'static str> {
    match input {
        [] => vec!["should"],
        [head] if head == "should" => SHOULD_COMMAND_VERBS.to_vec(),
        [head, verb] if head == "should" && verb == "review" => vec!["today", "week"],
        [head, verb] if head == "should" && verb == "explain" => {
            vec!["context", "drift", "commitment <id>"]
        }
        [head, verb] if head == "should" && verb == "synthesize" => vec!["week"],
        [head, verb] if head == "should" && verb == "spec" => {
            vec!["<topic>", "for", "with"]
        }
        [head, verb] if head == "should" && verb == "plan" => {
            vec!["<goal>", "for", "with"]
        }
        [head, verb] if head == "should" && verb == "delegate" => {
            vec!["<goal>", "to", "with", "into"]
        }
        [head, verb] if head == "should" && verb == "commit" => {
            vec!["<text>", "today", "tomorrow"]
        }
        [head, verb] if head == "should" && (verb == "capture" || verb == "feature") => {
            vec!["<text>"]
        }
        _ => Vec::new(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IntentHints {
    pub target_kind: String,
    pub mode: &'static str,
    pub suggestions: Vec<&'static str>,
}

pub fn intent_hints(resolution: &CommandResolution) -> Option<IntentHints> {
    let target_kind = resolution.resolved.targets.first()?.kind;
    let suggestions = match target_kind {
        DomainKind::Capture => vec!["quick capture", "feature request", "inbox note"],
        DomainKind::Commitment => vec!["open commitment", "project link", "due date"],
        DomainKind::Context => vec!["today review", "week review", "read only"],
        DomainKind::SpecDraft => vec!["planned doc", "suggested path", "design constraints"],
        DomainKind::ExecutionPlan => vec!["task breakdown", "ordered steps", "planning only"],
        DomainKind::DelegationPlan => vec!["worker split", "ownership", "review gate"],
        _ => vec!["typed target"],
    };
    let mode = match target_kind {
        DomainKind::Context => "execute",
        DomainKind::SpecDraft | DomainKind::ExecutionPlan | DomainKind::DelegationPlan => {
            "planning_artifact"
        }
        _ => "create",
    };

    Some(IntentHints {
        target_kind: target_kind.to_string(),
        mode,
        suggestions,
    })
}

#[cfg(test)]
mod tests {
    use super::{intent_hints, next_tokens};
    use crate::command_lang::infer::parse_and_resolve;

    #[test]
    fn suggests_spec_and_plan_tails() {
        let spec = next_tokens(&["should".to_string(), "spec".to_string()]);
        assert!(spec.contains(&"<topic>"));
        let plan = next_tokens(&["should".to_string(), "plan".to_string()]);
        assert!(plan.contains(&"<goal>"));
    }

    #[test]
    fn suggests_delegate_tails() {
        let delegate = next_tokens(&["should".to_string(), "delegate".to_string()]);
        assert!(delegate.contains(&"<goal>"));
        assert!(delegate.contains(&"to"));
    }

    #[test]
    fn exposes_intent_hints_for_delegate_resolution() {
        let resolution = parse_and_resolve(&[
            "should".to_string(),
            "delegate".to_string(),
            "queue".to_string(),
            "cleanup".to_string(),
        ])
        .expect("resolve");
        let hints = intent_hints(&resolution).expect("intent hints");
        assert_eq!(hints.target_kind, "delegation_plan");
        assert_eq!(hints.mode, "planning_artifact");
        assert!(hints.suggestions.contains(&"worker split"));
    }
}
