---
phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
plan: 01
subsystem: contracts
tags: [phase-6, contracts, writeback, conflicts, people, api, config, docs]
requires:
  - phase: 05-now-inbox-core-and-project-substrate
    provides: typed project/action/linking substrate and operator-facing transport seams from Phase 05
provides:
  - canonical `vel-core` vocabulary for write-back operations, conflict cases, and people records
  - typed API DTOs exposing pending write-backs, conflicts, and people across operator-facing surfaces
  - schema/example/owner-doc assets for the new durable Phase 06 contract boundaries
affects: [phase-06, vel-core, vel-api-types, config, docs, contracts]
tech-stack:
  added: []
  patterns: [typed contract-first rollout, DTO boundary mapping, schema-and-example pairing, owner-doc publication]
key-files:
  created:
    - crates/vel-core/src/writeback.rs
    - crates/vel-core/src/conflicts.rs
    - crates/vel-core/src/people.rs
    - config/schemas/writeback-operation.schema.json
    - config/examples/writeback-operation.example.json
    - config/schemas/person-record.schema.json
    - config/examples/person-record.example.json
    - docs/cognitive-agent-architecture/integrations/writeback-and-conflict-contracts.md
  modified:
    - crates/vel-core/src/lib.rs
    - crates/vel-api-types/src/lib.rs
    - config/contracts-manifest.json
    - config/README.md
key-decisions:
  - "Phase 06 starts from one canonical write-back/conflict/people vocabulary in `vel-core` before any provider-specific storage or service slice widens around it."
  - "Operator-facing DTOs expose typed `pending_writebacks`, `conflicts`, and `people` collections instead of opaque JSON placeholders."
  - "The API boundary needed explicit `IntegrationFamilyData` and `IntegrationSourceRefData` wrappers so the new contracts stay transport-shaped instead of leaking additional core structs."
patterns-established:
  - "When a new durable contract appears, ship the core type, the transport mapping, the schema/example pair, and the owner doc in the same slice."
  - "When DTOs reference shared integration vocabulary, add the missing transport wrapper at the boundary instead of falling back to ad hoc strings."
requirements-completed: [WB-01, CONFLICT-01, PROV-01, PEOPLE-01]
duration: 13m
completed: 2026-03-19
---

# Phase 06-01 Summary

**Phase 06 now has one typed contract set for write-back, conflict, and people surfaces**

## Performance

- **Duration:** 13 min
- **Started:** 2026-03-19T03:28:00Z
- **Completed:** 2026-03-19T03:41:15Z
- **Tasks:** 2
- **Files modified:** 12
- **Files created:** 8

## Accomplishments

- Added canonical `vel-core` modules for write-back operations, conflict cases, and people records, including prefixed IDs, fixed status/kind vocabularies, shared target/provenance seams, and example-parse coverage.
- Extended `vel-api-types` with typed write-back, conflict, and people DTOs plus operator-facing `pending_writebacks`, `conflicts`, and `people` arrays on the Phase 05 surfaces that already carry action/linking state.
- Published the new contract assets in `config/` and the owner doc in `docs/cognitive-agent-architecture/integrations/`, so later Phase 06 slices can build against a stable vocabulary instead of provider-specific payload drift.

## Task Commits

No task commits were created. This slice was executed inline in the current Phase 06 worktree and left uncommitted for review.

## Files Created/Modified

- `crates/vel-core/src/writeback.rs` - Adds the canonical write-back ID, risk, status, kind, target, and record contracts.
- `crates/vel-core/src/conflicts.rs` - Adds the canonical conflict ID, kind, status, and persisted case record contract.
- `crates/vel-core/src/people.rs` - Adds the practical people registry contract with aliases, links, birthdays, and last-contacted state.
- `crates/vel-core/src/lib.rs` - Re-exports the new Phase 06 core modules from the shared crate boundary.
- `crates/vel-api-types/src/lib.rs` - Adds transport DTOs for write-backs, conflicts, people, and the missing integration-family/source wrappers used by those DTOs.
- `config/schemas/writeback-operation.schema.json` - Publishes the machine-readable write-back contract.
- `config/examples/writeback-operation.example.json` - Provides a checked-in write-back example that parses via `vel-core`.
- `config/schemas/person-record.schema.json` - Publishes the machine-readable people contract.
- `config/examples/person-record.example.json` - Provides a checked-in people example that parses via `vel-core`.
- `config/contracts-manifest.json` - Registers the new schema/example artifacts and authority doc.
- `config/README.md` - Documents the new Phase 06 config contract assets and ownership.
- `docs/cognitive-agent-architecture/integrations/writeback-and-conflict-contracts.md` - Names the owner modules, fixed vocabulary, and provider-slice rules for Phase 06.

## Decisions Made

- The conflict/write-back/people vocabulary is frozen in `vel-core` before storage and provider implementations begin, so later slices inherit names and statuses instead of inventing them.
- API DTOs remain transport-shaped even when the core already has matching enums and structs; the missing integration-family/source wrappers were added to preserve that boundary.
- Contract publication is part of the implementation boundary, not optional follow-up documentation, so schemas/examples/docs shipped in the same slice as the Rust types.

## Deviations from Plan

- `crates/vel-api-types/src/lib.rs` needed two extra DTO seams, `IntegrationFamilyData` and `IntegrationSourceRefData`, because the existing transport layer did not already define wrappers for those shared integration types.

## Issues Encountered

- The first `vel-api-types` test run failed because the new write-back and people DTOs referenced missing transport wrapper types. The slice was corrected inline, and the focused test suite passed on rerun.

## User Setup Required

- None.

## Next Phase Readiness

- Phase 06 now has a stable typed contract surface for write-back, conflicts, and people.
- The next dependent slice is `06-02`, installing deterministic ordering, durable conflict queue/history state, and upstream-ownership foundations on top of these contracts.

---
*Phase: 06-high-value-write-back-integrations-and-lightweight-people-graph*
*Completed: 2026-03-19*
