# Planning Profile Application Contract

## Status

Published in Phase 32 as the contract-first lifecycle layer for supervised planning-profile proposal approval and application.

Current contract status:

- `/v1/planning-profile` remains the canonical backend-owned read/patch seam for durable routine blocks and bounded planning constraints
- `POST /v1/planning-profile/proposals/:id/apply` is now shipped as the supervised application lane for staged planning-profile proposals
- `/v1/planning-profile` and `/v1/now` now both carry compact planning-profile proposal continuity so summary surfaces can report pending review and recent applied/failed outcomes without inventing shell-local state

## Purpose

Vel needs one explicit vocabulary for the next step after planning-profile edit staging:

- how a staged planning-profile proposal represents lifecycle state
- how later approval/application slices can report outcome continuity without inventing a planner-specific exception path
- how those lifecycle transitions stay tied to the canonical planning-profile mutation seam instead of shell-local planner writes

This contract exists to define:

- the lifecycle state carried by `PlanningProfileEditProposal`
- the minimum typed outcome vocabulary needed for supervised proposal application
- the relationship between planning-profile proposals and the already-shipped assistant proposal lifecycle model

## Core Contracts

- `PlanningProfileEditProposal`
  - the typed assistant- or voice-capable staging payload for a bounded planning-profile edit
  - now carries:
    - `source_surface`
    - `state`
    - `mutation`
    - `summary`
    - `requires_confirmation`
    - `continuity`
    - optional `outcome_summary`
    - optional `thread_id` / `thread_type`

- `AssistantProposalState`
  - reused for planning-profile proposal lifecycle rather than inventing a second planner-specific state enum
  - current values:
    - `staged`
    - `approved`
    - `applied`
    - `failed`
    - `reversed`

- `outcome_summary`
  - an optional typed summary string for explicit operator-facing continuity such as approval, failure, or applied outcome explanation
  - now shipped on applied or failed proposal threads and surfaced through backend summary reads
  - not a substitute for full thread continuity or canonical backend persistence

- `PlanningProfileProposalSummary`
  - a compact backend-owned summary over `planning_profile_edit` threads
  - currently exposes:
    - `pending_count`
    - `latest_pending`
    - `latest_applied`
    - `latest_failed`
  - used by `Now`, web `Settings`, CLI, and Apple summary surfaces to reflect the same supervised continuity

## Relationship To Existing Seams

The relationship should remain:

- `RoutinePlanningProfile` stays the one durable backend-owned planning input pack
- `PlanningProfileMutation` remains the one typed edit grammar
- `PlanningProfileEditProposal` remains the conversational staging payload over that mutation grammar
- approval/application resolves back through the canonical planning-profile mutation seam instead of inventing a second planner write path
- proposal lifecycle should reuse `AssistantProposalState` semantics so supervised action behavior stays coherent across domains

## Hard Rules

- this contract must not create a second planner model or a second planner-specific lifecycle vocabulary
- assistant or voice planning-profile edits must not silently mutate saved routines or constraints
- `state` and `outcome_summary` are continuity fields, not permission to bypass review or canonical mutation
- when continuity is required, `Threads` remains the explicit follow-through lane
- summary-first shells may reflect lifecycle state, but they should not become a second review system

## Published Artifacts

- schema: `config/schemas/planning-profile-edit-proposal.schema.json`
- example: `config/examples/planning-profile-edit-proposal.example.json`

## Current Limit

This contract still does not claim:

- broad reversal semantics beyond explicit continuity metadata
- broad natural-language planner mutation outside the current bounded routine/constraint grammar
- multi-day planning or autonomous calendar mutation
