use serde::Serialize;
use vel_core::DomainKind;

#[derive(Debug, Clone, Serialize)]
pub struct RegistryEntry {
    pub kind: DomainKind,
    pub aliases: &'static [&'static str],
}

pub fn default_registry() -> Vec<RegistryEntry> {
    vec![
        RegistryEntry {
            kind: DomainKind::Capture,
            aliases: &["capture", "note"],
        },
        RegistryEntry {
            kind: DomainKind::Commitment,
            aliases: &["commitment", "todo", "task"],
        },
        RegistryEntry {
            kind: DomainKind::Run,
            aliases: &["run"],
        },
        RegistryEntry {
            kind: DomainKind::Artifact,
            aliases: &["artifact"],
        },
        RegistryEntry {
            kind: DomainKind::SpecDraft,
            aliases: &["spec", "spec_draft"],
        },
    ]
}
