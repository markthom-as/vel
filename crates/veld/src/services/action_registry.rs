use std::collections::BTreeMap;

use vel_core::{generic_object_action_contracts, ActionContract};

#[derive(Debug, Clone, Default)]
pub struct ActionRegistry {
    contracts: BTreeMap<String, ActionContract>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        let mut registry = Self::default();
        for contract in generic_object_action_contracts() {
            registry.register(contract);
        }
        registry
    }

    pub fn register(&mut self, contract: ActionContract) {
        self.contracts
            .insert(contract.action_name.clone(), contract);
    }

    pub fn lookup(&self, action_name: &str) -> Option<&ActionContract> {
        self.contracts.get(action_name)
    }

    pub fn list(&self) -> Vec<&ActionContract> {
        self.contracts.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ActionRegistry;

    #[test]
    fn action_registry_registers_and_looks_up_generic_object_actions() {
        let registry = ActionRegistry::new();

        assert!(registry.lookup("object.get").is_some());
        assert!(registry.lookup("object.update").is_some());
        assert!(registry.lookup("object.explain").is_some());
        assert!(registry.list().len() >= 7);
    }
}
