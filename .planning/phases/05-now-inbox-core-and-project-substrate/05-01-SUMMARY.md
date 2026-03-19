---
phase: 05-now-inbox-core-and-project-substrate
plan: 01
subsystem: contracts
tags: [phase-5, vel-core, vel-api-types, json-schema, linking, operator-queue, projects]
requires:
  - phase: 04-autonomous-swarm-graph-rag-zero-trust-execution
    provides: contract publication pattern with schemas, examples, manifest wiring, and authority docs
provides:
  - typed project domain contracts in `vel-core`
  - typed action/intervention and review contracts in `vel-core`
  - typed linking scope and linked-node contracts in `vel-core`
  - transport DTOs for projects, action items, review snapshots, and linking
  - schema/example/manifest/doc coverage for the new Phase 05 contracts
affects: [phase-05, projects, now, inbox, sync, linking, web, apple, cli]
tech-stack:
  added: []
  patterns: [typed-domain-first contracts, manifest-backed schema examples, named review snapshot DTOs]
key-files:
  created:
    - crates/vel-core/src/project.rs
    - crates/vel-core/src/operator_queue.rs
    - crates/vel-core/src/linking.rs
    - config/schemas/project-workspace.schema.json
    - config/examples/project-workspace.example.json
    - config/schemas/operator-action-item.schema.json
    - config/examples/operator-action-item.example.json
    - docs/cognitive-agent-architecture/architecture/project-action-linking-contracts.md
  modified:
    - crates/vel-core/src/lib.rs
    - crates/vel-api-types/src/lib.rs
    - config/contracts-manifest.json
    - config/README.md
    - crates/veld/src/routes/chat.rs
    - crates/veld/src/routes/cluster.rs
    - crates/veld/src/routes/now.rs
    - crates/veld/src/routes/sync.rs
key-decisions:
  - "Publish Phase 05 contracts in `vel-core` and `vel-api-types` before persistence or client slices widen the payload boundary."
  - "Use one named `ReviewSnapshot` and `ReviewSnapshotData` shape instead of JSON count blobs."
  - "Keep route-level placeholder compatibility edits minimal until later Phase 05 service work fills the new typed fields with real data."
patterns-established:
  - "Phase contract slices ship domain types, DTOs, schema/example assets, and owner docs together."
  - "Typed review, project, action, and linking fields are added at transport boundaries without introducing opaque JSON placeholders."
requirements-completed: [ACTION-01, PROJ-01, PROJ-02, FAMILY-01]
duration: 1h20m
completed: 2026-03-19
---

# Phase 05-01 Summary

**Typed Phase 05 contracts for projects, action queues, review snapshots, and linking scopes with matching DTOs and manifest-backed schema artifacts**

## Performance

- **Duration:** 1h 20m
- **Started:** 2026-03-19T00:14:00Z
- **Completed:** 2026-03-19T01:34:43Z
- **Tasks:** 3
- **Files modified:** 16

## Accomplishments

- Added canonical `vel-core` contract modules for projects, operator action items, review snapshots, and linking records/scopes.
- Extended `vel-api-types` so `Now`, `Inbox`, sync/bootstrap, and linking surfaces can carry typed Phase 05 data.
- Checked in schema/example artifacts, manifest entries, and owner documentation for the new contract boundary.

## Task Commits

No task commits were created. Execution was recovered inline after an interrupted delegated run touched the correct files but did not produce commits or a summary, so the slice remains as uncommitted work in the current tree for review.

## Files Created/Modified

- `crates/vel-core/src/project.rs` - Project IDs, families, status, roots, provision request, and project record contract.
- `crates/vel-core/src/operator_queue.rs` - Action item, evidence, surfacing vocabulary, and named review snapshot contract.
- `crates/vel-core/src/linking.rs` - Linking scopes, statuses, pairing token record, and linked-node record contract.
- `crates/vel-core/src/lib.rs` - Re-exports the new Phase 05 core modules.
- `crates/vel-api-types/src/lib.rs` - Adds typed DTOs for projects, actions, review snapshots, linking, and Phase 05 payload expansions.
- `config/contracts-manifest.json` - Registers the new schema/example artifacts.
- `config/README.md` - Documents the new contract artifacts and authority doc.
- `config/schemas/project-workspace.schema.json` - Machine-readable project workspace schema.
- `config/examples/project-workspace.example.json` - Checked-in example for project workspace parsing.
- `config/schemas/operator-action-item.schema.json` - Machine-readable surfaced action/intervention schema.
- `config/examples/operator-action-item.example.json` - Checked-in example for surfaced action items.
- `docs/cognitive-agent-architecture/architecture/project-action-linking-contracts.md` - Authority doc for Phase 05 contract vocabulary.
- `crates/veld/src/routes/chat.rs` - Temporary typed inbox compatibility fields so transport additions compile.
- `crates/veld/src/routes/cluster.rs` - Temporary empty typed bootstrap fields for Phase 05 additions.
- `crates/veld/src/routes/now.rs` - Temporary typed `action_items` and `review_snapshot` mapping defaults.
- `crates/veld/src/routes/sync.rs` - Temporary empty typed sync/bootstrap fields for Phase 05 additions.

## Decisions Made

- Published `ProjectFamily` as exactly `Personal`, `Creative`, and `Work`, with snake_case wire encoding.
- Published `ActionKind` and `ActionState` as the shared operator vocabulary for both `Now` and `Inbox`.
- Added minimal route-side typed defaults to preserve compilation until later Phase 05 plans wire real backend data.

## Deviations from Plan

### Auto-fixed Issues

**1. Transport compatibility edits outside the listed plan files**
- **Found during:** Inline recovery after delegated execution stalled
- **Issue:** Adding new DTO fields in `vel-api-types` required boundary code to initialize those fields so the current tree kept compiling.
- **Fix:** Added temporary typed defaults in `crates/veld/src/routes/chat.rs`, `crates/veld/src/routes/cluster.rs`, `crates/veld/src/routes/now.rs`, and `crates/veld/src/routes/sync.rs`.
- **Verification:** `cargo test -p vel-api-types -- --nocapture`
- **Committed in:** Not committed

---

**Total deviations:** 1 auto-fixed
**Impact on plan:** Low. The extra route changes are compatibility shims only and are aligned with later Phase 05 implementation work.

## Issues Encountered

- The delegated executor touched the right contract surfaces but did not emit a completion signal, summary, or commits. The slice was audited and completed inline instead.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 05 now has a typed contract baseline for storage, service, CLI, web, and Apple slices to build against.
- Wave 2 can proceed with persisted project substrate work in `05-02`.

---
*Phase: 05-now-inbox-core-and-project-substrate*
*Completed: 2026-03-19*
