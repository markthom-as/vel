use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipClass {
    SourceOwned,
    Shared,
    VelOwned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipDefault {
    pub field: String,
    pub owner: OwnershipClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipOverlay {
    pub field: String,
    pub owner: OwnershipClass,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipEvaluation {
    pub field: String,
    pub owner: OwnershipClass,
    pub overlay_applied: bool,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::{OwnershipClass, OwnershipDefault, OwnershipOverlay};

    #[test]
    fn ownership_model_supports_source_owned_shared_and_vel_owned_fields() {
        let default = OwnershipDefault {
            field: "due".to_string(),
            owner: OwnershipClass::SourceOwned,
        };
        let overlay = OwnershipOverlay {
            field: "description".to_string(),
            owner: OwnershipClass::VelOwned,
            reason: "workspace local-only override".to_string(),
        };

        assert!(matches!(default.owner, OwnershipClass::SourceOwned));
        assert!(matches!(overlay.owner, OwnershipClass::VelOwned));
    }
}

