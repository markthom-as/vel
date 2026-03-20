# Phase 30 Context

## Phase

30 — Routine and planning-profile management surfaces

## Why This Phase Exists

Phase 29 made routine blocks and bounded planning constraints durable, backend-owned inputs to the same `day_plan` and `reflow` substrate. That closes the runtime truth problem, but it does not yet make the substrate operable in daily use. Today the shipped surfaces can summarize durable routine-backed posture, but the operator still lacks a real typed way to inspect and manage those records directly.

The next useful step is not richer planner autonomy. It is making durable routine blocks and bounded planning constraints manageable through backend-owned typed seams and thin shipped surfaces, especially in `Settings`, while keeping `Now` and `Threads` summary-first over the same planning profile.

## Required Inputs

- Phase 27 canonical scheduler facets and persisted `scheduler_rules`
- Phase 28 bounded `day_plan` contract and shipped same-day planning embodiment
- Phase 29 durable routine-block and planning-constraint persistence/runtime consumption
- existing product direction that `Settings` should expose summary-first recovery and planning posture without becoming a second planner
- `codex-workspace` scheduler/rule semantics already preserved in Phases 26-29

## Target Outcome

Vel should be able to:

- inspect the durable routine/planning profile through typed backend seams
- create, edit, and remove durable routine blocks without shell-owned planning logic
- manage a bounded set of planning constraints through the same canonical backend profile
- keep `Now`, `Threads`, and `Settings` aligned over one backend-owned planning-profile truth

## Non-Goals

- multi-day optimization
- broad autonomous calendar mutation
- a separate habit-tracker or lifestyle-planning product
- shell-owned planning heuristics
- replacing the existing bounded `day_plan` / `reflow` substrate

## Likely Architectural Center

- `vel-core` planning-profile contract and validation rules
- `vel-storage` planning-profile repository seams and migration-safe persistence
- `veld` service/route seams for typed planning-profile inspection and mutation
- existing shipped surfaces, especially `Settings`, consuming typed planning-profile DTOs without becoming planner authority

## Key Constraint

Phase 30 must make the durable planning substrate operator-manageable without creating a second planner model. Planning semantics still belong in backend-owned contracts and services; shells should only inspect, edit through typed seams, and summarize the resulting posture.
