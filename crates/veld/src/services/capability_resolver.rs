use vel_core::{
    CapabilityDecisionReasonCode, CapabilityResolutionDecision, CapabilityResolutionRequest,
    CapabilityTargetKind, ConfirmationMode, PolicyDecisionKind,
};

pub fn resolve(request: &CapabilityResolutionRequest) -> CapabilityResolutionDecision {
    if request.capability.trim().is_empty() {
        return CapabilityResolutionDecision {
            decision: PolicyDecisionKind::Denied,
            confirmation: ConfirmationMode::Deny,
            reason_code: CapabilityDecisionReasonCode::DeniedUnsupported,
            note: "empty capability request".to_string(),
        };
    }

    if request.dry_run {
        return CapabilityResolutionDecision {
            decision: PolicyDecisionKind::Allowed,
            confirmation: ConfirmationMode::Auto,
            reason_code: CapabilityDecisionReasonCode::AllowedDryRunPreview,
            note: "dry-run preview uses planning path only".to_string(),
        };
    }

    if request.capability.contains("requires_approval") {
        return CapabilityResolutionDecision {
            decision: PolicyDecisionKind::ConfirmationRequired,
            confirmation: ConfirmationMode::Ask,
            reason_code: CapabilityDecisionReasonCode::ConfirmationRequired,
            note: "operation requires operator confirmation".to_string(),
        };
    }

    match request.target_kind {
        CapabilityTargetKind::ReadOnlyExecution
        | CapabilityTargetKind::ToolInvocation
        | CapabilityTargetKind::Mutation => CapabilityResolutionDecision {
            decision: PolicyDecisionKind::Allowed,
            confirmation: ConfirmationMode::Auto,
            reason_code: CapabilityDecisionReasonCode::AllowedReadOnly,
            note: "read-only execution".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::resolve;
    use vel_core::{CapabilityResolutionRequest, CapabilityTargetKind, PolicyDecisionKind};

    #[test]
    fn resolver_is_deterministic_for_same_input() {
        let request = CapabilityResolutionRequest {
            capability: "artifact.write".to_string(),
            target_kind: CapabilityTargetKind::ToolInvocation,
            dry_run: false,
        };
        let first = resolve(&request);
        let second = resolve(&request);
        assert_eq!(first, second);
        assert_eq!(first.decision, PolicyDecisionKind::Allowed);
    }

    #[test]
    fn resolver_allows_dry_run_preview() {
        let request = CapabilityResolutionRequest {
            capability: "capture.create".to_string(),
            target_kind: CapabilityTargetKind::Mutation,
            dry_run: true,
        };
        let decision = resolve(&request);
        assert_eq!(decision.decision, PolicyDecisionKind::Allowed);
    }
}
