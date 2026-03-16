---
title: Define AR HUD protocol and constraints
status: ready
owner: agent
priority: P3
area: vel-task-hud
---

# Goal
Write the protocol/spec for future AR HUD consumers without committing to premature UI implementation.

## Scope
Document:
- minimal fields AR surfaces should receive
- update cadence constraints
- attention budget rules
- suppression rules for visual overload
- voice / gesture action hooks

## AR payload should prefer
- current task
- next task
- one risk
- one ritual

## Anti-goals
- porting the desktop panel into glasses
- dense text overlays
- multi-column UI

## Deliverable
A markdown spec under docs/ describing:
- semantics
- payload examples
- update behavior
- interaction model
- safety / overload constraints

## Done when
- AR consumers have a clean contract to target later

