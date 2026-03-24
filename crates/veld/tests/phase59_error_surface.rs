use vel_core::ActionErrorKind;

#[test]
fn membrane_error_surface_matches_the_canonical_matrix() {
    let names = vec![
        ActionErrorKind::ValidationError,
        ActionErrorKind::NotFound,
        ActionErrorKind::PolicyDenied,
        ActionErrorKind::ConfirmationRequired,
        ActionErrorKind::ReadOnlyViolation,
        ActionErrorKind::GrantMissing,
        ActionErrorKind::StaleVersion,
        ActionErrorKind::OwnershipConflict,
        ActionErrorKind::PendingReconciliation,
        ActionErrorKind::ExecutionDispatchFailed,
        ActionErrorKind::AuditCaptureFailed,
        ActionErrorKind::UnsupportedCapability,
    ]
    .into_iter()
    .map(|kind| format!("{kind:?}"))
    .collect::<Vec<_>>();

    assert_eq!(
        names,
        vec![
            "ValidationError",
            "NotFound",
            "PolicyDenied",
            "ConfirmationRequired",
            "ReadOnlyViolation",
            "GrantMissing",
            "StaleVersion",
            "OwnershipConflict",
            "PendingReconciliation",
            "ExecutionDispatchFailed",
            "AuditCaptureFailed",
            "UnsupportedCapability",
        ]
    );
}

#[test]
fn membrane_error_surface_remains_typed_instead_of_collapsing_into_generic_failure() {
    assert_ne!(
        ActionErrorKind::PolicyDenied,
        ActionErrorKind::ReadOnlyViolation
    );
    assert_ne!(
        ActionErrorKind::StaleVersion,
        ActionErrorKind::OwnershipConflict
    );
    assert_ne!(
        ActionErrorKind::ExecutionDispatchFailed,
        ActionErrorKind::AuditCaptureFailed
    );
}
