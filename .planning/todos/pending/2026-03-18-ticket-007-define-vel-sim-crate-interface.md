---
created: 2026-03-18T07:25:40.260Z
title: Ticket 007 - define vel-sim crate interface contract in SP1 scope
area: docs
files:
  - docs/tickets/phase-3/007-day-simulation-harness.md
  - docs/tickets/phase-3/parallel-execution-board.md
---

## Problem

Ticket 007 (Day-Simulation Harness) plans a new `crates/vel-sim/` crate, but the dependency architecture is undefined. `vel-sim` cannot depend on `veld` (would create a circular dependency), so it must consume `vel-core` + `vel-storage` + a thin replay-interface trait that `veld` injects at test time. If this interface contract is designed during implementation rather than during Phase 3 SP1 contract work, the crate architecture will be improvised and likely require rework.

## Solution

Add to the Phase 3 Sub-Phase 1 scope: define the `vel-sim` replay interface contract — specifically, which service seams `veld` must expose for the simulation harness to inject an injectable clock, trigger reducers, and observe emitted run/event boundaries. This contract should live in `vel-core` so both `veld` and `vel-sim` depend on it without circular edges.
