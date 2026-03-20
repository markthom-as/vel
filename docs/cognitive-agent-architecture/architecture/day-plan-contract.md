# Day-Plan Contract

## Status

Published and partially implemented in Phase 28.

Current shipped baseline:

- the backend can derive an optional same-day `day_plan` proposal from open commitments, same-day calendar anchors, durable routine blocks when present, inferred routine fallback when not, and canonical scheduler rules
- bounded planning constraints can now influence default time-window selection, calendar buffer windows, and bounded day caps
- `GET /v1/now` can carry that typed `day_plan` output directly to shells
- web `Now` renders the compact plan summary and routine-block context without inventing planning semantics locally
- `Threads` remains the longer-form continuity lane for disagreement or shaping work
- `Settings` may summarize plan posture, but it is not a second planner surface

This document defines the canonical backend-owned contract for bounded same-day planning before schedule drift occurs.

## Purpose

Vel needs one explainable planning substrate that can:

- shape the current day before drift occurs
- respect calendar anchors and routine blocks
- reuse canonical scheduler rules from commitments
- feed later `reflow` without creating a second planner model

## Core Contracts

- `RoutineBlock`
  - typed same-day planning input with `label`, `source`, start/end timestamps, and protection flag
- `DayPlanChange`
  - one explainable planning outcome row
  - currently `scheduled`, `deferred`, `did_not_fit`, or `needs_judgment`
- `DayPlanProposal`
  - bounded same-day plan summary with aggregate counts, explicit change rows, and the routine blocks used to shape the proposal

## Hard Rules

- this contract is same-day and bounded
- routine blocks are backend-owned planning inputs, not shell hints
- raw provider labels remain compatibility metadata; canonical scheduler rules remain the durable planning semantics
- shells must consume typed day-plan output rather than owning placement logic
- later `reflow` should be a recovery lane over the same planning substrate, not a separate planner

## Relationship To `reflow`

`day_plan` and `reflow` are different moments in the same story:

- `day_plan` shapes the day before drift
- `reflow` repairs the day after drift

Both should rely on:

- calendar anchors
- canonical scheduler rules
- explicit operator-visible outcomes

The supervised application-layer contract for turning bounded same-day planning output into explicit commitment scheduling changes is published separately in [day-plan-application-contract.md](./day-plan-application-contract.md).

## Published Artifacts

- schema: `config/schemas/day-plan-proposal.schema.json`
- example: `config/examples/day-plan-proposal.example.json`
- durable input contract: [durable-routine-planning-contract.md](./durable-routine-planning-contract.md)

## Current Limit

This contract does not yet claim:

- multi-day optimization
- automatic broad upstream calendar mutation
- shell-owned planning behavior
- final routine-block ingestion policy across every source
- full routine CRUD across every shipped surface
- shipped supervised application of day-plan outcomes to commitment scheduling in this slice
