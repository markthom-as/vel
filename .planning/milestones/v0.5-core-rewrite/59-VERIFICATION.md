# Phase 59 Verification

**Phase:** 59 - Action membrane, policy engine, and audit authority  
**Status:** Draft verification target  
**Updated:** 2026-03-22

## Purpose

Define what must be true before Phase 59 can be considered complete as the lawful membrane phase.

## Required Outputs

Phase 59 should leave behind:

- action registry and generic action contracts
- policy evaluator and explicit grants
- ownership resolution and stale/conflict classification
- audit/explainability core types and services
- `WriteIntent` lifecycle and execution-dispatch path
- hostile-path and error-surface membrane tests

## Verification Checks

### A. Action backbone

- [ ] Generic object actions exist and specialized action posture is layered, not parallel.
- [ ] Action contracts expose capability, confirmation, audit, and error metadata.

### B. Governance

- [ ] Policy precedence is explicit.
- [ ] Grants are explicit, scoped, and narrowing.
- [ ] Read-only enforcement exists at the intended loci.

### C. Ownership and disagreement states

- [ ] Ownership resolution is runtime-evaluable over overlays.
- [ ] Stale state is distinct from conflict state.
- [ ] Pending reconciliation and tombstone/write race are representable.

### D. Audit and explainability

- [ ] Denied and dry-run paths are auditable.
- [ ] `policy.explain` and related explain payloads are implemented enough to verify.
- [ ] `WriteIntent` remains lifecycle/dispatch-oriented rather than full provider logic.

### E. Membrane proof

- [ ] Happy-path tests pass.
- [ ] Hostile-path tests pass.
- [ ] Error-surface verification matches the canonical matrix.

## Suggested Command-Backed Checks

```bash
rg -n "object.get|object.query|object.update|object.explain" crates/vel-core/src crates/veld/src/services
rg -n "Grant|PolicyEvaluator|ask_if_external_write|ReadOnlyViolation" crates/vel-core/src crates/veld/src/services
rg -n "StaleVersion|OwnershipConflict|PendingReconciliation|WriteIntent|policy_explain" crates/vel-core/src crates/veld/src/services
rg -n "ValidationError|PolicyDenied|ConfirmationRequired|ReadOnlyViolation|GrantMissing|StaleVersion|OwnershipConflict|PendingReconciliation|ExecutionDispatchFailed|AuditCaptureFailed|UnsupportedCapability" crates/veld/tests
```

## Exit Standard

Phase 59 is verified when the membrane behaves as one typed, auditable, explainable, hostile-path-safe contract over the Phase 58 substrate.

---

*Verification target for the Phase 59 planning packet*
