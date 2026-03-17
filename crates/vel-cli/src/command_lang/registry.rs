use serde::Serialize;
use vel_core::{dsl_registry_entries, DomainKind};

#[derive(Debug, Clone, Serialize)]
pub struct RegistryEntry {
    pub kind: DomainKind,
    pub aliases: &'static [&'static str],
    pub selectors: &'static [&'static str],
    pub operations: &'static [&'static str],
}

pub fn default_registry() -> Vec<RegistryEntry> {
    dsl_registry_entries()
        .filter_map(|entry| {
            entry.domain_kind.map(|kind| RegistryEntry {
                kind,
                aliases: entry.aliases,
                selectors: entry.dsl_selectors,
                operations: entry.dsl_operations,
            })
        })
        .collect()
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
        assert!(registry
            .iter()
            .any(|entry| entry.kind == DomainKind::DelegationPlan));
        assert!(registry.iter().all(|entry| !entry.operations.is_empty()));
        assert!(registry.iter().all(|entry| !entry.selectors.is_empty()));
    }
}
