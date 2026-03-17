use serde::Serialize;
use vel_core::DomainKind;

#[derive(Debug, Clone, Serialize)]
pub struct RegistryEntry {
    pub kind: DomainKind,
    pub aliases: &'static [&'static str],
    pub selectors: &'static [&'static str],
    pub operations: &'static [&'static str],
}

pub fn default_registry() -> Vec<RegistryEntry> {
    vec![
        RegistryEntry {
            kind: DomainKind::Capture,
            aliases: &["capture", "note"],
            selectors: &["id", "latest", "recent"],
            operations: &["create", "inspect", "list", "link", "explain"],
        },
        RegistryEntry {
            kind: DomainKind::Commitment,
            aliases: &["commitment", "todo", "task"],
            selectors: &["id", "open", "due_today", "latest"],
            operations: &["create", "inspect", "list", "update", "link", "explain"],
        },
        RegistryEntry {
            kind: DomainKind::Run,
            aliases: &["run", "job"],
            selectors: &["id", "latest", "today"],
            operations: &["inspect", "list", "update", "explain"],
        },
        RegistryEntry {
            kind: DomainKind::Artifact,
            aliases: &["artifact", "output"],
            selectors: &["id", "latest", "type"],
            operations: &["create", "inspect", "list", "link", "explain"],
        },
        RegistryEntry {
            kind: DomainKind::SpecDraft,
            aliases: &["spec", "spec_draft"],
            selectors: &["topic", "latest"],
            operations: &["create", "inspect", "list", "explain"],
        },
        RegistryEntry {
            kind: DomainKind::ExecutionPlan,
            aliases: &["plan", "execution_plan"],
            selectors: &["topic", "latest"],
            operations: &["create", "inspect", "list", "explain"],
        },
        RegistryEntry {
            kind: DomainKind::Thread,
            aliases: &["thread"],
            selectors: &["id", "open", "latest"],
            operations: &["create", "inspect", "list", "update", "link", "explain"],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::default_registry;
    use vel_core::DomainKind;

    #[test]
    fn includes_uniform_domain_entries() {
        let registry = default_registry();
        assert!(registry
            .iter()
            .any(|entry| entry.kind == DomainKind::Commitment));
        assert!(registry
            .iter()
            .any(|entry| entry.kind == DomainKind::Artifact));
        assert!(registry.iter().any(|entry| entry.kind == DomainKind::Run));
        assert!(registry
            .iter()
            .any(|entry| entry.kind == DomainKind::Thread));
        assert!(registry.iter().all(|entry| !entry.operations.is_empty()));
        assert!(registry.iter().all(|entry| !entry.selectors.is_empty()));
    }
}
