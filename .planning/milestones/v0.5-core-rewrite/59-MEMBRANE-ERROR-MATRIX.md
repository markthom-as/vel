# Phase 59 Membrane Error Matrix

## Purpose

Define the canonical error surface for the `0.5` action membrane before implementation spreads bespoke error semantics across actions, workflows, and adapters.

## Matrix

| Error | Produced By | Retryable? | Auditable? | Explainable? | Expected Operator Posture |
| --- | --- | --- | --- | --- | --- |
| `ValidationError` | action contract / input validation | no | yes | yes | fix input |
| `NotFound` | object/registry/query lookup | maybe | yes | yes | inspect missing target |
| `PolicyDenied` | policy evaluator | no | yes | yes | blocked until policy changes |
| `ConfirmationRequired` | policy evaluator / confirmation layer | yes | yes | yes | ask operator |
| `ReadOnlyViolation` | policy evaluator / account-workspace-module posture | no | yes | yes | enable write mode or stop |
| `GrantMissing` | grant resolver | maybe | yes | yes | acquire narrower/valid grant |
| `StaleVersion` | optimistic concurrency / revision check | yes | yes | yes | refresh then retry |
| `OwnershipConflict` | ownership evaluator | maybe | yes | yes | inspect source ownership and reconcile |
| `PendingReconciliation` | sync/ownership layer | maybe | yes | yes | reconcile before mutating |
| `ExecutionDispatchFailed` | runtime dispatcher | maybe | yes | yes | inspect execution path |
| `AuditCaptureFailed` | audit layer | maybe | yes | yes | investigate trust gap |
| `UnsupportedCapability` | action registry / feature gate | no | yes | yes | feature unavailable |

## Requirement

Phase 59 should keep this matrix aligned with:

- policy precedence
- grant semantics
- `WriteIntent` lifecycle
- hostile-path membrane verification
