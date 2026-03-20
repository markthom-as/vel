# Phase 29 Context

## Phase

29 — Durable routine blocks and operator-managed planning constraints

## Why This Phase Exists

Phase 28 proved the bounded same-day planning substrate, but the current routine-block input is still intentionally limited. Today the backend can only infer protected routine blocks from current context. That is enough for a first planning lane, but it is not durable or operator-shaped enough to carry repeated daily use.

The next useful step is not broader planner autonomy. It is making routine blocks and a small set of planning constraints into backend-owned persisted records that can shape the same `day_plan` and `reflow` substrate consistently across surfaces.

## Required Inputs

- Phase 27 canonical scheduler facets and persisted `scheduler_rules`
- Phase 28 `day_plan` contract and shipped `GET /v1/now` embodiment
- existing `codex-workspace` rule model as preserved in Phase 27/28 docs
- `Vel.csv` pressure toward subtle context, trustworthy daily use, and rich but compact schedule posture

## Target Outcome

Vel should be able to:

- persist routine blocks as typed backend-owned planning records
- persist a bounded set of operator-managed planning constraints
- feed those records into same-day `day_plan` shaping without shell-owned planner logic
- summarize the resulting posture in `Now`, `Threads`, and `Settings`

## Non-Goals

- multi-day optimization
- broad autonomous calendar mutation
- a separate habit-tracker or quantified-self product
- shell-owned planning heuristics
- widening into a full lifestyle planner

## Likely Architectural Center

- `vel-core` contract for durable routine blocks and bounded planning constraints
- `vel-storage` persistence and repository seams
- `veld` day-plan/reflow services consuming durable routine records
- existing `Now` transport and shell summaries, not a second planner surface

## Key Constraint

Phase 29 must deepen the same planning substrate already used by `day_plan` and `reflow`. It should not create a parallel planner model or let any shell become the authority for routine or constraint semantics.
