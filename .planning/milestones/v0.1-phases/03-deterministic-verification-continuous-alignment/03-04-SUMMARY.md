---
phase: 03-deterministic-verification-continuous-alignment
plan: 04
subsystem: simulation, verification, runtime
tags: [phase-3, simulation, deterministic-replay, runtime, verification]

requires:
  - phase: 03-deterministic-verification-continuous-alignment
    plan: 01
    provides: trace-aware run contract and runtime event linkage
  - phase: 03-deterministic-verification-continuous-alignment
    plan: 02
    provides: operator-visible run lineage surfaces
  - phase: 03-deterministic-verification-continuous-alignment
    plan: 03
    provides: doc entrypoints for operator-facing verification guidance

provides:
  - Shared fixed/system clock seam in `vel-core`
  - Explicit `*_at(...)` deterministic service entrypoints for context generation, inference, and evaluate
  - `vel-storage` snapshot/capture helpers that accept fixed timestamps for replay paths
  - New `vel-sim` crate with an in-memory day replay harness and normalized state/event assertions

affects:
  - 03-05-PLAN.md (eval runner can reuse fixed-time seams and replay harness patterns)

requirements-completed:
  - VERIFY-01 (partial)
  - VERIFY-02 (partial)

duration: 34min
completed: 2026-03-18
---

# Phase 3 Plan 04: Deterministic Replay Harness Summary

Implemented the first deterministic day-simulation slice by introducing a shared clock seam, threading fixed-time variants through replay-sensitive services, and adding a new `vel-sim` crate that replays a fixed fixture into run-backed context generation while asserting normalized state and boundary events.

## Accomplishments

- Added `Clock`, `SystemClock`, and `FixedClock` in [`crates/vel-core/src/time.rs`](/home/jove/code/vel/crates/vel-core/src/time.rs) and re-exported them from [`crates/vel-core/src/lib.rs`](/home/jove/code/vel/crates/vel-core/src/lib.rs)
- Added `build_*_at(...)` deterministic builders in [`crates/veld/src/services/context_generation.rs`](/home/jove/code/vel/crates/veld/src/services/context_generation.rs)
- Added deterministic `generate_*_at(...)` and retry entrypoints in [`crates/veld/src/services/context_runs.rs`](/home/jove/code/vel/crates/veld/src/services/context_runs.rs)
- Added fixed-time `run_at(...)` seams in [`crates/veld/src/services/evaluate.rs`](/home/jove/code/vel/crates/veld/src/services/evaluate.rs) and [`crates/veld/src/services/inference/mod.rs`](/home/jove/code/vel/crates/veld/src/services/inference/mod.rs)
- Added timestamp-aware storage helpers in [`crates/vel-storage/src/db.rs`](/home/jove/code/vel/crates/vel-storage/src/db.rs) and [`crates/vel-storage/src/repositories/captures_repo.rs`](/home/jove/code/vel/crates/vel-storage/src/repositories/captures_repo.rs)
- Added the new replay harness crate in [`crates/vel-sim/src/lib.rs`](/home/jove/code/vel/crates/vel-sim/src/lib.rs) and registered it in [`Cargo.toml`](/home/jove/code/vel/Cargo.toml)
- Added focused deterministic tests in [`crates/veld/tests/runtime_loops.rs`](/home/jove/code/vel/crates/veld/tests/runtime_loops.rs)

## Verification

- `cargo test -p vel-core -- --nocapture`
- `cargo test -p vel-sim -- --nocapture`
- `cargo test -p veld evaluate_run_at_persists_fixed_computed_at -- --nocapture`
- `cargo test -p veld build_today_uses_supplied_time_for_date -- --nocapture`
- `cargo test -p veld runtime_loops -- --nocapture`

## Notes

- The harness currently proves deterministic replay on the run-backed context generation path with normalized identifiers and explicit boundary-event assertions.
- Full eval/judge orchestration remains Phase 3 Plan 05 work.
