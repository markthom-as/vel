---
title: Integrate Task HUD with risk engine
status: ready
owner: agent
priority: P1
area: vel-task-hud
---

# Goal
Feed risk engine outputs into task ranking and display.

## Scope
Integrate at least:
- lateness risk
- dependency pressure
- schedule collision
- prep window / travel risk if available

## Display examples
- `Leave in 12 min`
- `Blocked by waiting on X`
- `Overdue`
- `Prep window started`

## Requirements
- do not duplicate risk logic already owned elsewhere
- define a narrow interface from risk engine -> task ranking/hud
- preserve explainability for why a threat is shown

## Tests
- threat surfacing tests
- ranking boosts from risk
- suppression when risk resolves

## Done when
- risk fields influence score and threat group
- UI can render at least one concrete threat badge

