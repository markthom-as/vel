use vel_core::{MembraneConflict, MembraneConflictKind};

#[derive(Debug, Default)]
pub struct ConflictClassifier;

impl ConflictClassifier {
    pub fn classify_stale_version(
        &self,
        expected_revision: i64,
        actual_revision: i64,
    ) -> Option<MembraneConflict> {
        (expected_revision != actual_revision).then(|| MembraneConflict {
            kind: MembraneConflictKind::StaleVersion,
            field: Some("revision".to_string()),
            reason: format!("expected revision {expected_revision}, found {actual_revision}"),
        })
    }

    pub fn classify_ownership_conflict(
        &self,
        field: &str,
        source_owned: bool,
        local_change_requested: bool,
    ) -> Option<MembraneConflict> {
        (source_owned && local_change_requested).then(|| MembraneConflict {
            kind: MembraneConflictKind::OwnershipConflict,
            field: Some(field.to_string()),
            reason: format!("source-owned field {field} cannot be changed locally"),
        })
    }

    pub fn classify_pending_reconciliation(&self, state: &str) -> Option<MembraneConflict> {
        (state == "pending_reconciliation").then(|| MembraneConflict {
            kind: MembraneConflictKind::PendingReconciliation,
            field: None,
            reason: "object is waiting on reconciliation".to_string(),
        })
    }

    pub fn classify_provider_divergence(&self, state: &str) -> Option<MembraneConflict> {
        (state == "provider_divergence").then(|| MembraneConflict {
            kind: MembraneConflictKind::ProviderDivergence,
            field: None,
            reason: "provider state diverged from canonical state".to_string(),
        })
    }

    pub fn classify_tombstone_write_race(
        &self,
        tombstoned: bool,
        write_requested: bool,
    ) -> Option<MembraneConflict> {
        (tombstoned && write_requested).then(|| MembraneConflict {
            kind: MembraneConflictKind::TombstoneWriteRace,
            field: None,
            reason: "write requested against tombstoned object".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ConflictClassifier;
    use vel_core::MembraneConflictKind;

    #[test]
    fn conflict_classifier_keeps_stale_version_distinct_from_ownership_conflict() {
        let classifier = ConflictClassifier;
        let stale = classifier
            .classify_stale_version(1, 2)
            .expect("stale version should classify");
        let ownership = classifier
            .classify_ownership_conflict("due", true, true)
            .expect("ownership conflict should classify");

        assert_eq!(stale.kind, MembraneConflictKind::StaleVersion);
        assert_eq!(ownership.kind, MembraneConflictKind::OwnershipConflict);
    }

    #[test]
    fn conflict_classifier_distinguishes_reconciliation_divergence_and_tombstone_cases() {
        let classifier = ConflictClassifier;

        assert_eq!(
            classifier
                .classify_pending_reconciliation("pending_reconciliation")
                .expect("pending reconciliation should classify")
                .kind,
            MembraneConflictKind::PendingReconciliation
        );
        assert_eq!(
            classifier
                .classify_provider_divergence("provider_divergence")
                .expect("provider divergence should classify")
                .kind,
            MembraneConflictKind::ProviderDivergence
        );
        assert_eq!(
            classifier
                .classify_tombstone_write_race(true, true)
                .expect("tombstone/write race should classify")
                .kind,
            MembraneConflictKind::TombstoneWriteRace
        );
    }
}
