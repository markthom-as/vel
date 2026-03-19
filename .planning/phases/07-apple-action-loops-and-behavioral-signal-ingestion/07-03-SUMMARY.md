---
phase: 07-apple-action-loops-and-behavioral-signal-ingestion
plan: 03
subsystem: apple-behavior-summary
tags: [phase-7, apple, health, behavior, now, veld, integration-tests]
requires:
  - phase: 07-apple-action-loops-and-behavioral-signal-ingestion
    provides: typed Apple behavior-summary contracts from 07-01
  - phase: 07-apple-action-loops-and-behavioral-signal-ingestion
    provides: authenticated Apple route namespace and backend Apple service seam from 07-02
provides:
  - bounded Apple Health ingestion for step, stand, and exercise metrics only
  - backend-owned Apple behavior summary route with freshness and explainable evidence
  - optional `Now` projection of the same backend behavior summary payload
affects: [phase-07, apple, health, now, sync]
tech-stack:
  added: []
  patterns: [bounded metric allowlist, summary-level explainability, backend-owned Apple behavior projection]
key-files:
  created:
    - crates/veld/tests/apple_behavior_summary.rs
    - .planning/phases/07-apple-action-loops-and-behavioral-signal-ingestion/07-03-SUMMARY.md
  modified:
    - crates/veld/src/adapters/health.rs
    - crates/veld/src/services/apple_behavior.rs
    - crates/veld/src/services/mod.rs
    - crates/veld/src/routes/apple.rs
    - crates/veld/src/services/now.rs
    - crates/veld/src/app.rs
key-decisions:
  - "Phase 07 Apple behavior summaries only ingest `step_count`, `stand_hours`, and `exercise_minutes`; everything else is ignored for this surface."
  - "The backend summary exposes freshness and source/device evidence directly so Apple clients can explain state without local heuristics."
  - "The same summary is reusable from the dedicated Apple route and the `Now` source-activity projection."
patterns-established:
  - "Apple health ingestion should narrow at the adapter boundary instead of letting broader raw metrics leak into summary logic."
  - "Top-level summary reasons should carry through the concrete source/device provenance already present on metric-level evidence."
requirements-completed: [HEALTH-01, HEALTH-02, APPLE-01]
duration: 5m
completed: 2026-03-19
---

# Phase 07-03 Summary

**Vel now ingests a bounded Apple Health metric set and serves one backend-owned daily behavior summary with freshness and source-backed reasons**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-19T07:17:41Z
- **Completed:** 2026-03-19T07:22:31Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Added failing integration coverage for bounded Apple behavior ingestion, out-of-scope metric filtering, and freshness/explainability requirements.
- Kept Phase 07 health ingestion scoped to `step_count`, `stand_hours`, and `exercise_minutes` by filtering at the adapter boundary.
- Exposed a dedicated authenticated Apple behavior-summary endpoint and reused the same summary in `Now` as a backend-owned source activity projection.
- Fixed the final explainability gap by promoting source/device provenance into summary-level reasons so clients can explain behavior state without local interpretation.

## Task Commits

1. **Task 1: Add failing tests for bounded Apple behavior ingestion and explainable summary derivation** - `1a5ef3a` (`test`)
2. **Task 2: Implement bounded health ingestion and backend-owned Apple behavior summaries** - completed inline in the worktree after an interrupted delegated execution left the service mostly implemented but short of the final explainability requirement

## Files Created/Modified

- `crates/veld/tests/apple_behavior_summary.rs` - integration coverage for bounded metrics, freshness, and explainability.
- `crates/veld/src/adapters/health.rs` - filters Phase 07 ingestion to the supported Apple behavior metric set.
- `crates/veld/src/services/apple_behavior.rs` - rolls up daily metrics, computes freshness, and emits summary plus metric reasons.
- `crates/veld/src/routes/apple.rs` and `crates/veld/src/app.rs` - expose the authenticated `/v1/apple/behavior-summary` route.
- `crates/veld/src/services/now.rs` - projects the same Apple behavior summary into `Now.sources.health`.
- `crates/veld/src/services/mod.rs` - exports the Apple behavior service.

## Decisions Made

- Unsupported Apple Health metrics remain out of scope for Phase 07 even if they appear in raw snapshots.
- Summary-level reasons must carry through source/device provenance, not just metric keys and timestamps.
- `Now` can mirror the Apple behavior summary, but the canonical client-facing contract remains the dedicated Apple route.

## Deviations from Plan

- The interrupted worker completed most of the implementation path but did not finish verification or create the summary. I preserved that work, fixed the remaining summary-level explainability requirement inline, and completed the focused verification locally.

## Issues Encountered

- The first verification pass failed because the summary reasons did not yet mention the persisted source/device evidence (`Apple Watch`, `Health`, `Workout`) required by the plan-facing tests. Promoting the first metric-level evidence line into summary-level reasons resolved it without widening scope.

## User Setup Required

None. This slice changes backend ingestion and summary behavior only.

## Next Phase Readiness

- Phase `07-04` can now wire Apple clients to the backend-owned behavior-summary route and `Now` schedule source without inventing local behavior interpretation.
- The dedicated Apple route and the `Now` projection now share the same bounded, explainable backend summary.

---
*Phase: 07-apple-action-loops-and-behavioral-signal-ingestion*
*Completed: 2026-03-19*
