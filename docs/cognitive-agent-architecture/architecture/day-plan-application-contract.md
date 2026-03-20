# Day-Plan Application Contract

## Status

Published in Phase 33 slice `33-01` as the contract-first lifecycle layer for supervised same-day `day_plan` and `reflow` application over commitment scheduling.

Current contract status:

- `day_plan` and `reflow` already exist as backend-owned, explainable same-day planning outputs
- `CommitmentSchedulingProposal` is now the published lifecycle payload for turning those bounded outputs into supervised scheduling changes
- `POST /v1/commitment-scheduling/proposals/:id/apply` is now shipped as the supervised backend apply lane for staged same-day scheduling proposals
- actionable `reflow` acceptance now routes through that canonical commitment scheduling seam instead of only suppressing the card
- compact continuity summaries for those proposals are now shipped across `GET /v1/now`, CLI review output, and Apple quick-loop `Now` surfaces

## Purpose

Vel needs one explicit vocabulary for the next useful step after bounded same-day planning output:

- how a proposed same-day schedule repair or shaping change is represented
- how lifecycle state stays coherent with the existing supervised proposal model
- how later approval/application slices report durable outcome continuity without inventing a planner-specific exception path

This contract exists to define:

- the lifecycle payload for bounded same-day schedule application
- the minimum typed mutation vocabulary for commitment scheduling changes
- the relationship between same-day planning proposals and the already-shipped `AssistantProposalState` lifecycle model
- the supervised backend apply lane that resolves a staged proposal into durable commitment scheduling state

## Core Contracts

- `CommitmentSchedulingProposal`
  - the typed proposal payload for bounded same-day schedule application
  - currently carries:
    - `source_kind`
    - `state`
    - `summary`
    - `requires_confirmation`
    - `continuity`
    - `mutations`
    - optional `outcome_summary`
    - optional `thread_id` / `thread_type`

- `CommitmentSchedulingSourceKind`
  - identifies whether the proposal came from:
    - `day_plan`
    - `reflow`

- `CommitmentSchedulingMutation`
  - one bounded commitment scheduling change over the canonical commitment model
  - currently carries:
    - `commitment_id`
    - `kind`
    - `title`
    - `summary`
    - optional `project_label`
    - optional `previous_due_at_ts`
    - optional `next_due_at_ts`

- `CommitmentSchedulingMutationKind`
  - the current bounded mutation vocabulary:
    - `set_due_at`
    - `clear_due_at`

- `AssistantProposalState`
  - reused for same-day schedule-application lifecycle instead of inventing a planner-specific state enum
  - current values:
    - `staged`
    - `approved`
    - `applied`
    - `failed`
    - `reversed`

- `outcome_summary`
  - an optional compact operator-facing continuity field for applied or failed proposal outcomes
  - not a substitute for full thread continuity or canonical backend persistence

- `CommitmentSchedulingProposalSummary`
  - compact summary-first continuity for same-day schedule proposals and outcomes
  - currently carries:
    - `pending_count`
    - optional `latest_pending`
    - optional `latest_applied`
    - optional `latest_failed`

## Relationship To Existing Seams

The relationship should remain:

- `DayPlanProposal` and `ReflowProposal` stay the explainable same-day planning outputs
- `CommitmentSchedulingProposal` is the supervised application-layer payload over those outputs
- `CommitmentSchedulingProposalSummary` is the compact read-model for summary surfaces over that same apply lane
- approved application must resolve back through canonical backend-owned commitment mutation seams instead of shell-local planner writes
- lifecycle state should reuse `AssistantProposalState` semantics so supervised behavior stays coherent across domains

## Hard Rules

- this contract is same-day and bounded; it is not a multi-day planner
- this contract is about commitment scheduling only; it is not broad autonomous calendar editing
- shells may reflect continuity, but they must not invent planner writes locally
- `state` and `outcome_summary` are continuity fields, not permission to bypass review or canonical mutation
- when continuity is required, `Threads` remains the explicit follow-through lane

## Published Artifacts

- schema: `config/schemas/commitment-scheduling-proposal.schema.json`
- example: `config/examples/commitment-scheduling-proposal.example.json`

## Shipped Apply Path

- `POST /v1/commitment-scheduling/proposals/:id/apply`
  - applies a staged same-day scheduling proposal through canonical commitment mutation seams
  - records `approved`, `applied`, or `failed` continuity back into the proposal thread
  - currently accepts staged proposal threads such as:
    - `reflow_edit`
    - `day_plan_apply`

## Current Limit

This contract still does not claim:

- multi-day schedule optimization
- shell-owned scheduling mutation
- broad calendar-event editing
- automatic reversal beyond explicit continuity metadata
