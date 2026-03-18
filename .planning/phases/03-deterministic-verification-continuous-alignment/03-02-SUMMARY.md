---
phase: 03-deterministic-verification-continuous-alignment
plan: 02
subsystem: cli, web, runtime-api
tags: [phase-3, trace, operator-surfaces, cli, web]

requires:
  - phase: 03-deterministic-verification-continuous-alignment
    plan: 01
    provides: trace-aware run DTOs and compatibility fallback semantics

provides:
  - `vel runs` output includes trace lineage summaries
  - `vel run inspect` prints trace and parent-run metadata before event inspection
  - Settings runtime Recent Runs cards show trace and parent-run lineage
  - Settings websocket updates preserve and display refreshed trace lineage without refetching

affects:
  - 03-03-PLAN.md (docs parity can now reference shipped trace-visible operator surfaces)
  - 03-04-PLAN.md (simulation and replay reporting can target visible operator trace affordances)

requirements-completed:
  - TRACE-03 (partial)

duration: 7min
completed: 2026-03-18
---

# Phase 3 Plan 02: Operator Trace Surfaces Summary

Extended the existing CLI and runtime-tab operator surfaces so trace linkage is visible where runs are already inspected, without introducing a new standalone trace explorer.

## Accomplishments

- Updated [`crates/vel-cli/src/commands/runs.rs`](/home/jove/code/vel/crates/vel-cli/src/commands/runs.rs) so `vel runs` includes a trace column and `vel run inspect` prints `Trace` and `Parent run`
- Added focused CLI unit tests for trace summary formatting in [`crates/vel-cli/src/commands/runs.rs`](/home/jove/code/vel/crates/vel-cli/src/commands/runs.rs)
- Extended the runtime Recent Runs cards in [`clients/web/src/components/SettingsPage.tsx`](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) with trace and parent-run lineage
- Updated [`clients/web/src/components/SettingsPage.test.tsx`](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx) to verify both initial rendering and websocket-driven trace updates
- Clarified the operator interpretation of `trace_id` and `parent_run_id` in [`docs/api/runtime.md`](/home/jove/code/vel/docs/api/runtime.md)

## Verification

- `cargo test -p vel-cli runs -- --nocapture`
- `npm test -- --run src/components/SettingsPage.test.tsx -t "recent run|updates rendered runs"`

## Notes

- This slice intentionally reuses the existing Recent Runs and CLI run-inspection surfaces. A richer multi-step trace explorer remains future work.
