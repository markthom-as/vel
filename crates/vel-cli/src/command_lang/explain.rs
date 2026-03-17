use crate::command_lang::completion;
use crate::command_lang::infer::CommandResolution;
use vel_core::glossary_entry_for_kind;

pub fn render_explanation(resolution: &CommandResolution) -> String {
    let mut out = Vec::new();
    out.push(format!(
        "Parsed `{}` as {:?} -> {}",
        resolution.parsed.source_text, resolution.parsed.family, resolution.parsed.verb
    ));
    if let Some(kind) = resolution
        .resolved
        .targets
        .first()
        .map(|target| target.kind)
    {
        if let Some(entry) = glossary_entry_for_kind(kind) {
            out.push(format!("Vocabulary: {} — {}", entry.term, entry.summary));
        }
        out.push(format!("Target kind: {}", kind));
        out.push(format!(
            "Resolved operation: {}",
            resolution.resolved.operation
        ));
    }
    if let Some(hints) = completion::intent_hints(resolution) {
        out.push(format!("Resolved mode: {}", hints.mode));
        out.push(format!("Intent hints: {}", hints.suggestions.join(", ")));
    }
    for assumption in &resolution.intent.assumptions {
        out.push(format!("Assumption: {}", assumption));
    }
    if !resolution.intent.inferred.is_null() {
        out.push(format!("Inferred: {}", resolution.intent.inferred));
    }
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::render_explanation;
    use crate::command_lang::infer::parse_and_resolve;

    #[test]
    fn explanation_includes_inferred_delegate_fields() {
        let resolution = parse_and_resolve(&[
            "should".to_string(),
            "delegate".to_string(),
            "queue".to_string(),
            "cleanup".to_string(),
        ])
        .expect("resolve");
        let output = render_explanation(&resolution);
        assert!(output.contains("delegation plan"));
        assert!(output.contains("Resolved mode: planning_artifact"));
        assert!(output.contains("\"artifact_kind\":\"delegation_plan\""));
    }
}
