use std::collections::HashMap;

use vel_core::{OwnershipDefault, OwnershipEvaluation, OwnershipOverlay};

#[derive(Debug, Default)]
pub struct OwnershipResolver;

impl OwnershipResolver {
    pub fn resolve(
        &self,
        defaults: &[OwnershipDefault],
        overlays: &[OwnershipOverlay],
    ) -> Vec<OwnershipEvaluation> {
        let overlay_map: HashMap<&str, &OwnershipOverlay> =
            overlays.iter().map(|overlay| (overlay.field.as_str(), overlay)).collect();

        defaults
            .iter()
            .map(|default| match overlay_map.get(default.field.as_str()) {
                Some(overlay) => OwnershipEvaluation {
                    field: default.field.clone(),
                    owner: overlay.owner.clone(),
                    overlay_applied: true,
                    reason: overlay.reason.clone(),
                },
                None => OwnershipEvaluation {
                    field: default.field.clone(),
                    owner: default.owner.clone(),
                    overlay_applied: false,
                    reason: "static default".to_string(),
                },
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::OwnershipResolver;
    use vel_core::{OwnershipClass, OwnershipDefault, OwnershipOverlay};

    #[test]
    fn ownership_resolver_applies_overlay_over_static_field_defaults() {
        let resolver = OwnershipResolver;
        let resolved = resolver.resolve(
            &[
                OwnershipDefault {
                    field: "due".to_string(),
                    owner: OwnershipClass::SourceOwned,
                },
                OwnershipDefault {
                    field: "description".to_string(),
                    owner: OwnershipClass::Shared,
                },
            ],
            &[OwnershipOverlay {
                field: "description".to_string(),
                owner: OwnershipClass::VelOwned,
                reason: "local policy override".to_string(),
            }],
        );

        assert!(resolved.iter().any(|field| {
            field.field == "due"
                && matches!(field.owner, OwnershipClass::SourceOwned)
                && !field.overlay_applied
        }));
        assert!(resolved.iter().any(|field| {
            field.field == "description"
                && matches!(field.owner, OwnershipClass::VelOwned)
                && field.overlay_applied
        }));
    }
}

