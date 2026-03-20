# Phase 31 Context

## Title

Cross-surface planning-profile parity and assistant-managed routine edits

## Why this phase exists

Phase 30 closed the canonical planning-profile management seam for the web shell, but the shipped product still has a real gap:

- web `Settings` can now inspect and mutate durable routine blocks and bounded planning constraints
- CLI, Apple, and assistant/voice entry do not yet offer equivalent access to that same durable planning profile
- the assistant and voice lanes are already strong product entry points, so leaving planning-profile edits web-only would create an avoidable shell split

The next useful step is not a new planner. It is parity over the same backend-owned planning-profile seam.

## What must stay true

- there is still only one backend-owned `RoutinePlanningProfile`
- `day_plan` and `reflow` remain readers of that profile rather than separate planner authorities
- assistant and voice flows may stage bounded edits, but they must preserve explicit confirmation, provenance, and fail-closed behavior
- CLI, Apple, web, and assistant entry should all read or mutate the same typed planning-profile seam instead of inventing shell-local routine state

## Likely implementation lanes

- add canonical read surfaces for the planning profile where CLI and Apple need them
- route assistant/voice routine or planning-constraint edits through the typed planning-profile mutation contract
- preserve thread continuity and confirmation posture for multi-step or ambiguous edits
- keep shell-specific UI work thin; planning semantics stay in Rust

## Out of scope

- autonomous planner mutation without confirmation
- broad calendar editing or upstream routine provisioning
- multi-day planning expansion
- a separate routine-management product detached from `day_plan` / `reflow`

## Key references

- [planning-profile-management-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md)
- [durable-routine-planning-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md)
- [day-plan-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/day-plan-contract.md)
- [runtime.md](/home/jove/code/vel/docs/api/runtime.md)
- [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md)
