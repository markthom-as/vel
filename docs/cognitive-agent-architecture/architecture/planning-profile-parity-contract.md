# Planning Profile Parity Contract

## Status

Published in Phase 31 as the contract-first parity layer over the planning-profile management seam shipped in Phase 30.

Current contract status:

- `/v1/planning-profile` is already the canonical backend-owned read/patch seam for the durable routine/planning profile
- web `Settings` is the only shipped management surface today
- this contract publishes the shared cross-surface vocabulary needed to extend that same seam into CLI, Apple, and assistant/voice entry without creating a second planner

## Purpose

Vel needs one explicit vocabulary for two adjacent concerns:

- how non-web surfaces participate in planning-profile parity over the same backend-owned profile
- how assistant- or voice-capable routine/profile edits stage bounded changes without bypassing the canonical mutation seam

This contract exists to define:

- which product surfaces are participating in planning-profile parity
- how assistant-capable planning-profile edit proposals are represented before later execution slices wire them into concrete routes and shells
- how those proposals stay tied to `PlanningProfileMutation` rather than inventing a second edit grammar

For supervised proposal approval/application lifecycle vocabulary, see [planning-profile application contract](./planning-profile-application-contract.md).

## Core Contracts

- `PlanningProfileSurface`
  - identifies the requesting or staging surface for planning-profile parity work
  - current values:
    - `web_settings`
    - `cli`
    - `apple`
    - `assistant`
    - `voice`
- `PlanningProfileContinuity`
  - indicates whether a staged edit is expected to remain inline or continue through `Threads`
  - current values:
    - `inline`
    - `thread`
- `PlanningProfileEditProposal`
  - a typed assistant-capable staging payload for a bounded planning-profile edit
  - carries:
    - `source_surface`
    - `state`
    - `mutation`
    - `summary`
    - `requires_confirmation`
    - `continuity`
    - optional `outcome_summary`
    - optional `thread_id` / `thread_type` once a staged edit has been persisted into explicit `Threads` continuity

## Relationship To Existing Seams

The relationship should remain:

- `RoutinePlanningProfile` stays the one durable backend-owned planning input pack
- `PlanningProfileMutation` remains the one typed edit grammar for routine blocks and bounded planning constraints
- `PlanningProfileEditProposal` stages a future assistant/voice-capable edit over that same mutation grammar
- later CLI, Apple, and assistant work should read or mutate the same `/v1/planning-profile` seam rather than introducing shell-local planning state

## Hard Rules

- this parity contract must not create a second planner model
- assistant or voice-driven routine edits must still resolve to `PlanningProfileMutation`
- cross-surface parity does not imply autonomous planner mutation
- when continuity is required, `Threads` should remain the explicit follow-through lane rather than hidden shell-local state
- staged assistant or voice edits should attach their continuity thread to the typed proposal instead of requiring the client to guess thread identity
- lifecycle values for planning-profile proposals should reuse `AssistantProposalState` semantics instead of inventing a planner-specific state enum

## Published Artifacts

- schema: `config/schemas/planning-profile-edit-proposal.schema.json`
- example: `config/examples/planning-profile-edit-proposal.example.json`

## Current Limit

This contract does not yet claim:

- shipped CLI or Apple management surfaces
- shipped assistant/voice routine-edit execution over the planning profile
- richer recurrence semantics beyond the current bounded weekday/local-time template model
- multi-day planning or broad autonomous calendar mutation
