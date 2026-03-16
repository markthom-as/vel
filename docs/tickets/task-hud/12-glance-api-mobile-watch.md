---
title: Expose glance payload for mobile/watch
status: ready
owner: agent
priority: P2
area: vel-task-hud
---

# Goal
Provide a compressed endpoint for watch and mobile surfaces.

## Scope
Add an endpoint or internal API such as:
```text
GET /tasks/glance
```

Return:
- current_task
- next_task
- top_risk
- next_ritual

## Requirements
- payload should be tiny and stable
- no dense task arrays by default
- suitable for lock screen, watch, widget, or wearable consumer

## Tests
- serialization tests
- empty-state tests
- compact payload selection tests

## Done when
- one consumer can request a glance payload successfully

