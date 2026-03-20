# Phase 28 Context

## Phase

28 — Routine blocks and commitment-aware day planning

## Why this phase exists

Phase 26 made `reflow` a real backend-owned same-day recovery lane.
Phase 27 made scheduler/tag semantics canonical and durable.

The next product-useful step is to use that same canonical rule system before drift happens, not only after drift happens.

Vel needs a bounded backend-owned day-planning lane that can:

- read calendar anchors and remaining commitments
- respect canonical scheduler rules from the `codex-workspace` model
- account for routine blocks as typed planning inputs
- produce an explainable same-day plan with explicit `scheduled`, `deferred`, and `did_not_fit` outcomes

## Product direction

This phase should improve the morning and daytime operator loop without widening into a speculative autonomous planner.

It should preserve the current product shape:

- `Now` stays summary-first and action-capable
- `Threads` stays the continuity/escalation lane
- `Settings` can surface planning/recovery posture but should not become a second planner shell
- shells remain thin over backend-owned planning output

## Required carry-forward context

- `codex-workspace` remains the reference for proven scheduling semantics:
  - `block:*`
  - `cal:free`
  - duration tags/tokens
  - `time:*`
  - local urgent/defer
  - fixed-start anchors
  - didn't-fit handling
- `Vel.csv` feedback still applies:
  - subtle context/status should stay visible
  - richer behavior should not turn the top-level shell into clutter
  - Settings should support recovery/trust understanding without dumping operators into runtime internals

## Boundary rule

This phase is about bounded same-day shaping, not:

- multi-day optimization
- automatic broad upstream calendar mutation
- shell-local planner logic
- replacing the existing `reflow` lane

Instead, it should make initial day shaping and later reflow part of one coherent backend-owned planning/recovery story.
