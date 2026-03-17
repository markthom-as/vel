use crate::command_lang::infer::CommandResolution;

pub fn render_explanation(resolution: &CommandResolution) -> String {
    let mut out = Vec::new();
    out.push(format!(
        "Parsed `{}` as {:?} -> {}",
        resolution.parsed.source_text, resolution.parsed.family, resolution.parsed.verb
    ));
    for assumption in &resolution.intent.assumptions {
        out.push(format!("Assumption: {}", assumption));
    }
    out.join("\n")
}
