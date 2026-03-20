# Durable Routine Planning Contract

## Status

Published in Phase 29 as the contract-first follow-on to the Phase 28 same-day `day_plan` baseline.

Current shipped status:

- `day_plan` and `reflow` already exist as bounded backend-owned same-day planning and recovery lanes
- durable routine blocks and bounded planning constraints are now defined as canonical contract types
- durable routine blocks and planning constraints are now persisted through dedicated backend/storage seams
- `day_plan` and `reflow` now consume those durable records as the first-class same-day planning substrate, with inferred routine fallback only when no durable blocks are available
- shipped surfaces now summarize whether current planning posture is using operator-managed routine blocks or inferred fallback

## Purpose

Vel needs one durable source of planning truth above transient current-context inference.

This contract exists to define:

- what a durable routine block is
- what a bounded planning constraint is
- how both should feed the same `day_plan` / `reflow` substrate

## Core Contracts

- `DurableRoutineBlock`
  - a persisted operator-managed or imported routine block template
  - local-time and weekday aware rather than single-day output only
- `PlanningConstraint`
  - a bounded operator-managed planning policy input such as a preferred default time window or calendar buffer
- `RoutinePlanningProfile`
  - the durable pack of routine blocks and bounded planning constraints

See also:

- `docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md`
  - the typed management seam for editing the durable planning profile without creating shell-owned planning logic

## Relationship To `day_plan`

The relationship should be:

- `RoutinePlanningProfile` is durable planning input
- `day_plan` is same-day shaped planning output
- `reflow` is same-day recovery output after drift

That means:

- durable routine records should feed same-day shaping
- same-day output should remain explainable from durable records, canonical scheduler rules, calendar anchors, and commitments
- shells should manage or summarize the durable inputs, but they must not become the authority for planning semantics

## Hard Rules

- this remains a bounded same-day planning system, not a broad autonomous planner
- routine records and constraints are backend-owned product truth
- raw provider labels remain compatibility metadata, not durable planning truth
- `day_plan` and `reflow` must continue sharing one planning substrate
- the durable routine contract must not imply a full habit-tracker or lifestyle-product scope

## Published Artifacts

- schema: `config/schemas/routine-planning-profile.schema.json`
- example: `config/examples/routine-planning-profile.example.json`

## Current Limit

This contract does not yet claim:

- full CRUD across all shipped shells
- rich recurrence semantics beyond the bounded weekday/local-time template model
- multi-day optimization
- broad autonomous upstream calendar mutation
- a complete routines or habits product
