---
title: Add task inference engine
status: ready
owner: agent
priority: P1
area: vel-task-hud
---

# Goal
Allow Vel to derive HUD-visible work items from other sources without casually creating duplicate durable objects.

## Boundary rule

Before creating new stored tasks, prefer:

- commitment creation where the item is truly actionable and durable,
- derived HUD-only items where the item is ephemeral or presentation-specific.

This ticket must not create a parallel inference lane that duplicates existing commitment, nudge, or risk semantics without an explicit justification.

## Scope
Create `vel-task-inference` and support at least:
- calendar-derived tasks
- agent-suggested tasks
- email-derived followups (if email infra exists already)

## Initial examples
- meeting at 3pm -> `Prepare for meeting`, `Leave for meeting`
- unanswered important email -> `Follow up with X`
- agent suggestion -> `Review generated draft`

## Requirements
- inferred tasks must carry provenance
- users should be able to accept, dismiss, or hide repeated inference
- avoid generating duplicate tasks aggressively
- if a derived item maps cleanly to an existing commitment, prefer reusing/updating that commitment instead of minting a new durable task row

## Tests
- deduplication tests
- provenance tests
- calendar window inference tests

## Done when
- at least one inferred task source is live
- inferred tasks are distinguishable from manual tasks
