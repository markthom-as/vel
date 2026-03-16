---
title: Build desktop Task HUD UI
status: ready
owner: agent
priority: P1
area: vel-task-hud
---

# Goal
Expose the first visible Task HUD on desktop.

## Scope
Implement:
- full task panel in the main app shell
- compact always-on-top HUD mode
- expandable/collapsible behavior
- quick actions: done, snooze, expand

## UX requirements
- low visual entropy
- readable at a glance
- no naggy motion
- obvious state changes on completion/snooze

## Compact HUD should show
- top 1-3 active tasks
- one risk indicator
- next ritual
- expand affordance

## Notes
- put behind feature flag if rollout risk is non-trivial
- preserve room for future visual language work
- do not overfit to AR constraints yet

## Tests
- component tests if framework supports them
- interaction tests for quick actions
- state sync tests with task store

## Done when
- desktop shell exposes HUD
- quick actions work
- panel and compact modes both render from shared view model

