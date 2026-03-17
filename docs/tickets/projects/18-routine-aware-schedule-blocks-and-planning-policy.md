---
title: Add routine-aware schedule-block projection and planning policy
status: ready
owner: agent
priority: P2
area: projects
depends_on:
  - 16-project-dependency-and-blocker-projection.md
  - 17-routine-definitions-and-project-anchors.md
---

# Goal

Define the policy-backed planning layer that combines routine blocks, tags, and dependencies into explicit schedule-block projections and future operator scheduling decisions.

## Scope

- schedule-block projection contract
- policy rules for routine block fit, tag constraints, and dependency pressure
- explainability for why something did or did not fit
- visibility for unschedulable work

## Requirements

- schedule blocks remain derived projections, not silent durable authority over source systems
- tags and dependencies may influence fit, but must be inspectable
- unscheduled work stays visible instead of being dropped
- any write-back or calendar mutation remains explicitly gated

## Suggested write scope

- planning/scheduler policy layer
- typed `ScheduleBlockData` or equivalent contract
- explain/debug surfaces for scheduling decisions
- tests for conflicting blocks, blocked tasks, and non-fitting work

## Acceptance criteria

- Vel has a canonical backend plan for routine-aware scheduling instead of scattered heuristics
- operators can inspect why work was placed, deferred, or left unscheduled
- this layer is clearly separated from the current runtime loops substrate and from provider calendars
