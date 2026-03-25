use vel_core::{
    MutationCommitRequest, MutationCommitResult, MutationCommitStatus, MutationProposal,
};

fn in_write_scope(target_ref: &str, write_scope: &[String]) -> bool {
    if write_scope.is_empty() {
        return true;
    }
    write_scope
        .iter()
        .map(|scope| scope.trim())
        .filter(|scope| !scope.is_empty())
        .any(|scope| target_ref.starts_with(scope))
}

pub fn evaluate_commit(
    request: &MutationCommitRequest,
    already_applied: bool,
) -> MutationCommitResult {
    let MutationProposal {
        target_ref,
        write_scope,
        ..
    } = &request.proposal;

    if request.dry_run {
        return MutationCommitResult {
            status: MutationCommitStatus::SkippedDryRun,
            note: "dry-run mode skips mutation commit".to_string(),
        };
    }

    if already_applied {
        return MutationCommitResult {
            status: MutationCommitStatus::AlreadyApplied,
            note: "mutation idempotency key already applied".to_string(),
        };
    }

    if !request.approved {
        return MutationCommitResult {
            status: MutationCommitStatus::WaitingForApproval,
            note: "operator approval required before commit".to_string(),
        };
    }

    if !in_write_scope(target_ref, write_scope) {
        return MutationCommitResult {
            status: MutationCommitStatus::RejectedOutOfScope,
            note: "target is outside write scope".to_string(),
        };
    }

    MutationCommitResult {
        status: MutationCommitStatus::Applied,
        note: "mutation commit approved".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate_commit;
    use vel_core::{MutationCommitRequest, MutationCommitStatus, MutationProposal};

    fn proposal() -> MutationProposal {
        MutationProposal {
            mutation_kind: "capture_create".to_string(),
            idempotency_key: "mut_test_1".to_string(),
            target_ref: "capture:quick_note".to_string(),
            write_scope: vec!["capture:".to_string()],
            metadata_json: serde_json::json!({}),
        }
    }

    #[test]
    fn blocks_without_approval() {
        let result = evaluate_commit(
            &MutationCommitRequest {
                proposal: proposal(),
                approved: false,
                dry_run: false,
            },
            false,
        );
        assert_eq!(result.status, MutationCommitStatus::WaitingForApproval);
    }

    #[test]
    fn duplicate_key_is_safe() {
        let result = evaluate_commit(
            &MutationCommitRequest {
                proposal: proposal(),
                approved: true,
                dry_run: false,
            },
            true,
        );
        assert_eq!(result.status, MutationCommitStatus::AlreadyApplied);
    }
}
