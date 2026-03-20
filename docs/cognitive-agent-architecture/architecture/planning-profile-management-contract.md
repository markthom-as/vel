# Planning Profile Management Contract

## Status

Published in Phase 30 as the contract-first management layer over the durable routine-planning substrate shipped in Phase 29.

Current shipped status:

- durable routine blocks and bounded planning constraints already persist as one backend-owned `RoutinePlanningProfile`
- `day_plan` and `reflow` already consume that profile as bounded same-day planning input
- backend/storage mutation and validation seams now exist behind the canonical management contract
- `/v1/planning-profile` now exposes typed read and patch behavior over that profile
- the shipped web `Settings` surface can now inspect, add, and remove routine blocks and bounded planning constraints without introducing client-owned planning logic
- cross-shell management parity is still pending; this contract currently claims the canonical backend seam plus the shipped web management lane
- Phase 31 publishes a companion parity/staging contract in `planning-profile-parity-contract.md` for future CLI, Apple, and assistant/voice participation over the same mutation model

## Purpose

Vel needs one explicit read/edit vocabulary for the durable planning profile before shipped surfaces begin managing it.

This contract exists to define:

- how routine blocks and planning constraints are addressed for management
- how typed upsert/remove mutations are represented
- how future transport and route seams can expose management without turning shells into planning authorities

## Core Contracts

- `RoutinePlanningProfile`
  - the durable backend-owned pack of routine blocks and bounded planning constraints
- `PlanningProfileMutation`
  - one typed change against that profile
  - currently supports:
    - `upsert_routine_block`
    - `remove_routine_block`
    - `upsert_planning_constraint`
    - `remove_planning_constraint`
- `PlanningProfileRemoveTarget`
  - the minimal typed removal target for routine blocks or planning constraints

## Relationship To Existing Planning Output

The relationship should remain:

- planning-profile management edits durable inputs
- `day_plan` reads those inputs for same-day shaping
- `reflow` reads those inputs for same-day recovery
- shells inspect or submit typed mutations, but do not own planning semantics

## Hard Rules

- this management contract must not create a second planner model
- generic settings JSON is not the authority for routine/profile edits
- typed mutations should remain explicit and explainable from persisted profile state
- future management routes should stay bounded to the same-day planning substrate rather than widening into a full routine product

## Published Artifacts

- schema: `config/schemas/planning-profile-mutation.schema.json`
- example: `config/examples/planning-profile-mutation.example.json`

## Current Limit

This contract does not yet claim:

- shell-complete editing flows across all shipped clients
- rich recurrence semantics beyond the bounded weekday/local-time template model
- multi-day planning or broad autonomous calendar mutation
