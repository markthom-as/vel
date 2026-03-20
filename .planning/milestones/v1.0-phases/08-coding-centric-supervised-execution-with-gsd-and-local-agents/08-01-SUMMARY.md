---
phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
plan: 01
subsystem: contracts
tags: [phase-08, contracts, handoffs, execution-context, protocol, config]
requires: []
provides:
  - typed execution-context, routing-policy, and local-agent-manifest vocabulary in `vel-core`
  - repo-aware handoff envelope fields and matching transport DTOs
  - machine-readable schema/example assets plus owner docs for the Phase 08 execution boundary
affects: [phase-08, vel-core, vel-api-types, vel-protocol, config, docs]
tech-stack:
  added: []
  patterns: [contract-first publication, repo-aware handoff metadata, explicit review gates and write scopes]
key-files:
  created:
    - crates/vel-core/src/execution.rs
    - config/schemas/project-execution-context.schema.json
    - config/examples/project-execution-context.example.json
    - config/schemas/execution-handoff.schema.json
    - config/examples/execution-handoff.example.json
    - config/schemas/local-agent-manifest.schema.json
    - config/examples/local-agent-manifest.example.json
    - docs/cognitive-agent-architecture/agents/coding-execution-contracts.md
    - .planning/phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-01-SUMMARY.md
  modified:
    - crates/vel-core/src/lib.rs
    - crates/vel-core/src/run.rs
    - crates/vel-api-types/src/lib.rs
    - crates/vel-protocol/src/lib.rs
    - config/contracts-manifest.json
    - config/README.md
    - docs/cognitive-agent-architecture/agents/handoffs.md
key-decisions:
  - "Extended the existing `HandoffEnvelope` instead of inventing a parallel execution-only envelope."
  - "Kept project context, task kind, agent profile, token budget, review gate, and repo root as explicit typed fields across core and transport layers."
  - "Published schema/example assets and owner docs in the same slice so later runtime work has one governed contract source."
patterns-established:
  - "Repo-aware coding handoffs remain trace-linked and machine-readable."
  - "Local-agent manifests declare runtime kind, tool allowlist, capability allowlist, and writable roots explicitly."
requirements-completed: [EXEC-01, GSD-01, HANDOFF-01, HANDOFF-02, LOCAL-01, POLICY-01]
duration: 20m
completed: 2026-03-19
---

# Phase 08 Plan 01: Contract Publication Summary

**Phase 08 now has a canonical typed execution boundary spanning core types, transport DTOs, protocol metadata, config assets, and handoff docs**

## Performance

- **Duration:** 20 min
- **Completed:** 2026-03-19
- **Files modified:** 15

## Accomplishments

- Added `crates/vel-core/src/execution.rs` with typed execution context, routing policy, review-gate, token-budget, local-runtime, and manifest vocabulary.
- Extended `crates/vel-core/src/run.rs` so repo-aware coding handoffs carry project, profile, token budget, review gate, and repo-root metadata without creating a second envelope type.
- Added matching `vel-api-types` DTOs and a lightweight `vel-protocol` manifest reference type.
- Published the new schema/example assets, registered them in the contracts manifest, and documented the boundary in nearby authority docs.

## Verification

- `cargo test -p vel-core execution -- --nocapture`
- `cargo test -p vel-api-types execution -- --nocapture`
- `cargo test -p vel-protocol -- --nocapture`
- `cargo test -p vel-config -- --nocapture`
- `node scripts/verify-repo-truth.mjs`

## Decisions Made

- Reused the existing Phase 05 project root vocabulary for `notes_root` while introducing a dedicated repo worktree type for execution-facing repo metadata.
- Kept write scopes explicit but allowed them to be empty arrays when a manifest or policy input is read-only.
- Added contract-level example parsing tests in `vel-core` so the shipped example assets stay executable rather than drifting into documentation-only status.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Corrected handoff deadline wire serialization**
- **Found during:** focused contract verification
- **Issue:** the extended `HandoffEnvelope` still serialized `deadline` with the default `OffsetDateTime` shape, so the new execution handoff example would not parse.
- **Fix:** switched `deadline` to RFC3339 option serde handling and reran the focused tests.
- **Files modified:** `crates/vel-core/src/run.rs`
- **Verification:** `cargo test -p vel-core execution -- --nocapture`

## Issues Encountered

- `crates/vel-core/src/lib.rs`, `crates/vel-api-types/src/lib.rs`, and the config/docs authority files were already active Phase 07/08 seams, so the patch was kept contract-focused and avoided widening into runtime behavior.

## User Setup Required

None.

## Next Phase Readiness

- `08-02` can persist project execution context against the published contract instead of inventing its own transport shape.
- `08-03` can enforce runtime kind and writable-root rules against the same manifest and handoff vocabulary.
