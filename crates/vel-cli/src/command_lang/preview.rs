use crate::command_lang::infer::CommandResolution;

pub fn render(resolution: &CommandResolution) -> String {
    let mut lines = vec![
        format!("Source: {}", resolution.parsed.source_text),
        format!("Family: {:?}", resolution.parsed.family),
        format!("Verb: {}", resolution.parsed.verb),
        format!("Operation: {}", resolution.resolved.operation),
    ];

    if !resolution.parsed.target_tokens.is_empty() {
        lines.push(format!(
            "Target: {}",
            resolution.parsed.target_tokens.join(" ")
        ));
    }

    if !resolution.resolved.targets.is_empty() {
        lines.push("Resolved targets:".to_string());
        for target in &resolution.resolved.targets {
            lines.push(format!(
                "  - kind={} selector={} attributes={}",
                target.kind,
                target
                    .selector
                    .as_ref()
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "(none)".to_string()),
                target.attributes
            ));
        }
    }

    if !resolution.intent.assumptions.is_empty() {
        lines.push("Assumptions:".to_string());
        for assumption in &resolution.intent.assumptions {
            lines.push(format!("  - {}", assumption));
        }
    }

    lines.join("\n")
}
