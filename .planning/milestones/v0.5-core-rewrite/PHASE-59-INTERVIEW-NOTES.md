# Phase 59 Interview Notes

Captured on `2026-03-22` while shaping Phase 59 of milestone `0.5`.

## Core Direction

Phase 59 should be authored against the authoritative intended outputs of Phase 57 and Phase 58 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

Phase 59 is the membrane/governance phase. It should not drift backward into storage redesign or forward into adapter-specific write logic, transport/UI work, or workflow authoring UX.

## Approved Phase 59 Chunking

Use this sharpened chunk sequence:

1. action registry, generic object actions, and action contracts
2. policy evaluator, grants, confirmation modes, and precedence
3. ownership resolution, stale handling, and conflict classification
4. audit, explainability, and `WriteIntent` lifecycle/execution dispatch
5. membrane proving tests, hostile-path scenarios, and error-surface verification

## Important Reinforcements

### 1. Chunk 1 must include action contracts, not just action names

This chunk should lock:

- action registry structure
- generic action namespace
- action metadata
- input/output contracts
- capability declarations
- confirmation hooks
- audit hooks
- typed error surface

### 2. Chunk 2 must make grants concrete

This chunk should answer:

- whether grants are explicit or derived
- whether grants are durable or run-scoped
- how workflow grants are shaped
- how grants interact with module/workspace/account policy
- whether grants can narrow but not widen authority
- how grants are audited and explained

### 3. Chunk 3 must separate stale from conflict

Do not conflate:

- `stale` = version/time/order mismatch
- `conflict` = incompatible claims, ownership collisions, or competing writes

The chunk should classify at least:

- stale version
- missing authority
- pending reconciliation
- ownership conflict
- provider divergence
- tombstone/write race

### 4. Chunk 4 must keep `WriteIntent` partial

Phase 59 should define the lifecycle and execution-dispatch contract for `WriteIntent`, but not smuggle in full provider adapter semantics.

This chunk should lock:

- intent creation
- policy evaluation
- confirmation
- approval/rejection
- execution dispatch contract
- downstream operation recording
- result/error capture

### 5. Chunk 5 must prove hostile paths, not only happy paths

Required hostile-path coverage includes:

- stale write rejected correctly
- destructive write asks when policy says ask
- read-only account blocks external write
- cross-source ask path triggers
- ownership explain is accurate under conflict
- audit trail records denied and dry-run attempts
- typed/stable error surface under refusal paths

## Additional Recommended Packet Additions

### 1. Continuity sheet

Add a one-page continuity sheet that states:

- what Phase 57 established
- what Phase 58 established
- what Phase 59 will establish

### 2. Explicit Phase 59 non-goals

Call out that Phase 59 does **not**:

- define full provider adapter write logic
- define UI/API transport layer
- define workflow authoring UX
- redefine storage/bootstrap work owned by Phase 58
- define generalized cross-provider merge behavior beyond membrane ownership/conflict contracts

### 3. Canonical membrane error matrix

By the end of Phase 59, the packet should name a typed membrane error matrix including at least:

- `ValidationError`
- `NotFound`
- `PolicyDenied`
- `ConfirmationRequired`
- `ReadOnlyViolation`
- `GrantMissing`
- `StaleVersion`
- `OwnershipConflict`
- `PendingReconciliation`
- `ExecutionDispatchFailed`
- `AuditCaptureFailed`
- `UnsupportedCapability`

With columns for:

- producing layer
- retryability
- audit requirement
- explainability
- expected operator-facing posture

### 4. Membrane spikes

Add at least two early risk spikes:

#### Spike A

Generic action invocation path:

- `object.get`
- `object.query`
- `object.update`
- policy check
- audit emission
- typed error return

#### Spike B

`WriteIntent` lifecycle:

- local mutation request
- ownership evaluation
- confirmation required
- approval
- execution dispatch stub
- downstream result capture

## Condensed Guidance

- Keep Phase 59 authored against intended Phase 57/58 authority outputs, not the current filesystem alone.
- Keep the phase about lawful membrane behavior, not storage redesign or adapter implementation.
- Make action contracts explicit.
- Make grants explicit.
- Separate stale handling from conflict classification.
- Treat `WriteIntent` as lifecycle plus execution dispatch, not full provider behavior.
- Require hostile-path verification and a canonical membrane error matrix.
- Add explicit non-goals to keep the phase clean.
