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
    }
    for assumption in &resolution.intent.assumptions {
        out.push(format!("Assumption: {}", assumption));
    }
    out.join("\n")
}
