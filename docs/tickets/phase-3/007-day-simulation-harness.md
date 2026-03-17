---
title: Deterministic Day-Simulation Harness
status: planned
owner: staff-eng
type: verification
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 004-signal-reducer-pipeline
  - 017-execution-tracing-reviewability
labels:
  - veld
  - simulation
  - reliability
  - phase-3
---

# Context & Objectives

Vel has strong run/event persistence and retry workflows, but it does not yet have a dedicated deterministic replay harness for "day in the life" regression testing.

This ticket introduces a simulation harness that can replay realistic signal and run sequences against a controlled clock and produce stable assertions for state and workflow boundaries.

# Impacted Files & Symbols

- **Crate**: `veld` (runtime services and loop orchestration)
  - **Symbols**: context/inference/evaluate execution seams
- **Crate**: `vel-core`
  - **Symbols**: deterministic time and ordering seams used by replay-sensitive logic
- **Crate**: `vel-storage`
  - **Symbols**: fixture loading and deterministic event ordering guarantees
- **Crate (new)**: `crates/vel-sim` or equivalent simulation module
  - **Symbols**: simulation scenario runner, assertions, fixture loader

# Technical Requirements

- **Controllable Time**: simulation paths must run against an injectable clock/time source.
- **Replay Scenarios**: support replay of large signal/action sequences with stable ordering.
- **State + Event Assertions**: verify both final state and emitted run/event boundaries.
- **Deterministic Output**: identical fixture input must produce identical replay output.
- **Execution Speed**: simulation should remain fast enough for regular CI use.

# Cross-Cutting Trait Impact

- **Modularity**: required — simulation should use existing service seams, not duplicate orchestration logic.
- **Accessibility**: affected — simulation output should be operator-readable in test logs/reports.
- **Configurability**: affected — scenario fixtures and clock controls should be explicit.
- **Data Logging**: required — run/event assertions are first-class test outputs.
- **Rewind/Replay**: required — this ticket is the deterministic replay foundation.
- **Composability**: required — scenarios should be reusable by eval and regression workflows.

# Implementation Steps (The How)

1. **Clock seam**: add controllable time source where needed for replay determinism.
2. **Scenario runner**: implement day-simulation runner for ordered signal/action fixtures.
3. **Assertions**: capture and compare terminal state plus expected run/event sequences.
4. **Performance pass**: keep simulation runtime bounded for CI usage.

# Acceptance Criteria

1. [ ] A day-simulation suite replays realistic multi-hour workflows deterministically.
2. [ ] Replay asserts both end-state outputs and key run/event boundaries.
3. [ ] Re-running the same scenario yields identical results.
4. [ ] Simulation runtime is fast enough for routine automated verification.

# Verification & Regression

- **Unit Test**: clock and scenario ordering determinism tests.
- **Integration Test**: end-to-end day replay with state + event assertions.
- **Smoke Check**: run a representative scenario from CLI/test harness entrypoint.
- **Invariants**: no nondeterministic ordering or ambient wall-clock dependence in replay path.
