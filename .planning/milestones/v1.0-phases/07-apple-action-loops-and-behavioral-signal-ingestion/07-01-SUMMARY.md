---
phase: 07-apple-action-loops-and-behavioral-signal-ingestion
plan: 01
subsystem: apple-contract-foundation
tags: [phase-7, apple, ios, watch, contracts, schemas, swift, vel-core, vel-api-types]
requires:
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: typed operator/action/writeback surfaces that Apple loops will reuse rather than re-invent
provides:
  - canonical Apple voice-turn domain and DTO vocabulary in Rust
  - Swift decode parity for Apple voice, schedule, and bounded behavior-summary payloads
  - machine-readable schema/example/doc assets for the new Phase 07 Apple contracts
affects: [phase-07, apple, ios, watch, contracts, docs]
tech-stack:
  added: []
  patterns: [backend-owned Apple contract publication, Swift transport parity, schema-plus-example contract governance]
key-files:
  created:
    - crates/vel-core/src/apple.rs
    - config/schemas/apple-voice-turn.schema.json
    - config/examples/apple-voice-turn.example.json
    - config/schemas/apple-behavior-summary.schema.json
    - config/examples/apple-behavior-summary.example.json
    - docs/cognitive-agent-architecture/apple/apple-action-loop-contracts.md
    - .planning/phases/07-apple-action-loops-and-behavioral-signal-ingestion/07-01-SUMMARY.md
  modified:
    - crates/vel-core/src/lib.rs
    - crates/vel-api-types/src/lib.rs
    - clients/apple/VelAPI/Sources/VelAPI/Models.swift
    - config/contracts-manifest.json
    - config/README.md
key-decisions:
  - "Apple voice turns now carry explicit operation, intent, provenance, reasons, and evidence fields so backend-owned answers remain explainable."
  - "Schedule snapshots are a dedicated typed Apple payload derived from backend truth, not a client-owned query heuristic."
  - "Phase 07 behavior summaries are intentionally bounded to `step_count`, `stand_hours`, and `exercise_minutes`."
patterns-established:
  - "New durable Apple surfaces must ship as a three-layer contract slice: `vel-core` domain types, `vel-api-types` DTOs, and matching Swift decoders."
  - "Apple contract governance follows the existing config-manifest pattern: schema, example, manifest registration, and owner doc together."
requirements-completed: [IOS-01, IOS-02, HEALTH-02, APPLE-01]
duration: 11m
completed: 2026-03-19
---

# Phase 07-01 Summary

**Phase 07 now has one backend-owned Apple contract set for voice turns, schedule snapshots, and bounded daily behavior summaries**

## Performance

- **Duration:** 11 min
- **Started:** 2026-03-19T06:47:42Z
- **Completed:** 2026-03-19T06:58:44Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added `vel-core` Apple contract types and re-exports covering client surface, requested operation, voice intents, transcript provenance, schedule snapshots, response evidence, queued mutation summaries, and bounded behavior summaries.
- Extended `vel-api-types` with matching transport DTOs plus focused serialization tests for request/response parity.
- Added matching Swift wire models in `VelAPI` so Apple clients can decode the new backend-owned payloads without inventing alternate naming or local policy.
- Published schema/example assets and an Apple architecture contract doc, then registered them in the config manifest and config asset map.

## Task Commits

No task commits were created. The delegated worker was interrupted before commit/summary creation, and the slice was completed inline in the existing worktree for review continuity.

## Files Created/Modified

- `crates/vel-core/src/apple.rs` and `crates/vel-core/src/lib.rs`: define and export the canonical Phase 07 Apple vocabulary.
- `crates/vel-api-types/src/lib.rs`: adds Apple request/response DTOs, enum mappings, and focused transport tests.
- `clients/apple/VelAPI/Sources/VelAPI/Models.swift`: adds typed Swift decoding for Apple voice turns, schedule snapshots, and behavior summaries.
- `config/schemas/apple-voice-turn.schema.json` and `config/examples/apple-voice-turn.example.json`: publish the machine-readable Apple voice contract.
- `config/schemas/apple-behavior-summary.schema.json` and `config/examples/apple-behavior-summary.example.json`: publish the bounded behavior-summary contract.
- `config/contracts-manifest.json` and `config/README.md`: register and document the new contract assets.
- `docs/cognitive-agent-architecture/apple/apple-action-loop-contracts.md`: records owner modules, intent vocabulary, bounded metric scope, and the no-local-synthesis rule.

## Decisions Made

- Apple clients stay thin transport/render shells; backend services own interpretation, schedule derivation, and behavior-summary explainability.
- The behavior-summary schema explicitly limits Phase 07 to steps, stand hours, and exercise minutes.
- Voice-turn schema examples include both request and response shapes so offline provenance and backend evidence are governed together.

## Deviations from Plan

- None in scope. The delegated execution was interrupted mid-slice, but the remaining planned contract/doc work was completed without widening into route or client behavior changes.

## Issues Encountered

- The first executor left a partial Rust contract slice without the required summary or asset registration. I preserved that useful work, verified it, and completed the missing Swift/schema/doc half locally.

## User Setup Required

None. This slice only publishes typed contracts and documentation.

## Next Phase Readiness

- Phase 07 now has the contract foundation required for backend voice and behavior implementation.
- Wave 2 can build authenticated Apple voice orchestration on top of these DTOs and schema assets without widening Swift policy ownership.

---
*Phase: 07-apple-action-loops-and-behavioral-signal-ingestion*
*Completed: 2026-03-19*
