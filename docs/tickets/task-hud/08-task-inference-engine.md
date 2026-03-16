---
title: Add task inference engine
status: ready
owner: agent
priority: P1
area: vel-task-hud
---

# Goal
Allow Vel to derive tasks from other sources instead of relying only on manual entry.

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

## Tests
- deduplication tests
- provenance tests
- calendar window inference tests

## Done when
- at least one inferred task source is live
- inferred tasks are distinguishable from manual tasks

