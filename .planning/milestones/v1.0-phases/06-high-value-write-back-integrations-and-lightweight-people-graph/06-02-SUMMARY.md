---
phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
plan: 02
subsystem: reconciliation
tags: [phase-6, reconciliation, ordering, writeback, conflicts, sync, storage]
requires:
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: typed write-back, conflict, and people contracts from 06-01
provides:
  - deterministic node-origin ordering contracts for later multi-writer reconciliation
  - durable write-back history, conflict queue, and upstream ownership persistence
  - backend projections exposing pending write-backs and conflicts through sync and Now continuity
affects: [phase-06, vel-core, vel-storage, veld, sync, now, operator-queue]
tech-stack:
  added: []
  patterns: [deterministic ordering primitive, storage-first reconciliation foundation, backend-projected conflict continuity]
key-files:
  created:
    - crates/vel-core/src/node_identity.rs
    - crates/vel-core/src/ordering.rs
    - migrations/0040_phase6_conflicts_and_writebacks.sql
    - crates/vel-storage/src/repositories/writeback_operations_repo.rs
    - crates/vel-storage/src/repositories/conflict_cases_repo.rs
    - crates/vel-storage/src/repositories/upstream_refs_repo.rs
    - crates/veld/src/services/writeback.rs
  modified:
    - crates/vel-core/src/lib.rs
    - crates/vel-storage/src/db.rs
    - crates/vel-storage/src/lib.rs
    - crates/vel-storage/src/repositories/mod.rs
    - crates/veld/src/services/operator_queue.rs
    - crates/veld/src/services/client_sync.rs
    - crates/veld/src/services/now.rs
    - crates/veld/src/services/mod.rs
    - crates/veld/src/routes/now.rs
    - crates/veld/src/routes/sync.rs
    - crates/veld/src/routes/cluster.rs
key-decisions:
  - "Phase 6 reconciliation starts with one explicit `NodeIdentity` and `OrderingStamp` contract in `vel-core` before any provider-specific write lane widens."
  - "Write-back attempts, conflict cases, and upstream object refs persist as first-class records with stable IDs instead of hiding inside provider-specific payload blobs."
  - "Pending writes and open conflicts project into `Now` and sync/bootstrap immediately so clients hydrate the same operator truth before any provider write UI exists."
patterns-established:
  - "When a provider slice needs reconciliation, add the durable record seam first and let services/routes project typed continuity from storage."
  - "If an old plan references a consumed migration number, preserve monotonic sqlx ordering and record the deviation rather than forcing a conflicting filename."
requirements-completed: [RECON-01, CONFLICT-01, PROV-01]
duration: 24m
completed: 2026-03-19
---

# Phase 06-02 Summary

**Phase 06 now has deterministic ordering plus durable write-back/conflict persistence that can already surface into operator continuity**

## Performance

- **Duration:** 24 min
- **Started:** 2026-03-19T03:41:15Z
- **Completed:** 2026-03-19T04:05:07Z
- **Tasks:** 2
- **Files modified:** 18
- **Files created:** 7

## Accomplishments

- Added `NodeIdentity` and `OrderingStamp` in `vel-core`, giving Phase 06 one deterministic ordering rule for node-origin reconciliation instead of ad hoc timestamp comparison.
- Added migration-backed persistence for `writeback_operations`, `conflict_cases`, and `upstream_object_refs`, with dedicated storage repos and `vel-storage` facade methods for insert/get/list/update flows.
- Added a provider-neutral `writeback` service plus `Now`/sync/operator-queue projections so queued write-backs and open conflicts are already typed backend continuity state.

## Task Commits

No task commits were created. This slice was executed inline in the current Phase 06 worktree and left uncommitted for review.

## Files Created/Modified

- `crates/vel-core/src/node_identity.rs` - Adds the stable node-origin identifier newtype with ordering and serde support.
- `crates/vel-core/src/ordering.rs` - Adds deterministic ordering stamps ordered by physical time, logical counter, then node ID.
- `crates/vel-core/src/lib.rs` - Re-exports the new Phase 06 reconciliation primitives from the shared core boundary.
- `migrations/0040_phase6_conflicts_and_writebacks.sql` - Creates durable write-back, conflict, and upstream-object ownership tables plus supporting indexes.
- `crates/vel-storage/src/repositories/writeback_operations_repo.rs` - Adds write-back history persistence helpers.
- `crates/vel-storage/src/repositories/conflict_cases_repo.rs` - Adds conflict queue persistence helpers.
- `crates/vel-storage/src/repositories/upstream_refs_repo.rs` - Adds durable upstream object ownership/upsert lookup helpers.
- `crates/vel-storage/src/repositories/mod.rs` - Exposes the new storage modules through the repository tree.
- `crates/vel-storage/src/db.rs` - Extends the storage facade with typed write-back/conflict/upstream-ref methods.
- `crates/vel-storage/src/lib.rs` - Re-exports the new persistence records and reconciliation primitives at the storage boundary.
- `crates/veld/src/services/writeback.rs` - Adds provider-neutral queue/apply/fail/conflict orchestration over the new durable records.
- `crates/veld/src/services/operator_queue.rs` - Projects pending write-backs and conflicts into the ranked action snapshot.
- `crates/veld/src/services/client_sync.rs` - Hydrates pending write-backs, conflicts, and people into sync/bootstrap payloads.
- `crates/veld/src/services/now.rs` - Extends the `Now` service output with pending write-back and conflict continuity.
- `crates/veld/src/routes/now.rs` - Maps the new `Now` continuity fields to API DTOs.
- `crates/veld/src/routes/sync.rs` - Maps pending write-backs, conflicts, and people into the sync/bootstrap API boundary.
- `crates/veld/src/routes/cluster.rs` - Keeps cluster bootstrap aligned with the new sync continuity payload.

## Decisions Made

- Reconciliation state is durable before any provider-specific mutation flow lands, so Todoist/notes/GitHub/email write lanes inherit one history/conflict model.
- Cross-client continuity owns pending writes and conflicts in the backend; clients receive typed projections rather than inferring queue state from provider payloads.
- The migration used `0040_phase6_conflicts_and_writebacks.sql` instead of the plan's stale `0039_...` name because `0039_phase5_linking.sql` already exists and sqlx migration versions must stay unique.

## Deviations from Plan

- The plan referenced `migrations/0039_phase6_conflicts_and_writebacks.sql`, but the repository already contains `0039_phase5_linking.sql`. The migration shipped as `0040_phase6_conflicts_and_writebacks.sql` to preserve monotonic migration ordering and avoid a duplicate version collision.

## Issues Encountered

- The first `veld` compile pass failed because existing route tests still constructed the pre-06-02 `NowOutput` shape. Those test initializers were updated inline before rerunning the focused service/route suites.
- One new assertion initially overcounted pending writes by double-counting an open conflict; the expectation was corrected so queued write-backs and conflicts stay projected as distinct collections.

## User Setup Required

- None.

## Next Phase Readiness

- Phase 06 now has one deterministic ordering and durable reconciliation substrate for provider-specific write-back.
- The next dependent slice is `06-03`, closing the Todoist lane with typed project linkage, explicit write operations, and conflict-aware execution on top of these records.

---
*Phase: 06-high-value-write-back-integrations-and-lightweight-people-graph*
*Completed: 2026-03-19*
