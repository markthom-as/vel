---
title: Add routine definitions and project-affine recurring anchors
status: ready
owner: agent
priority: P1
area: projects
depends_on:
  - 14-project-registry-source-mappings-and-sync-policies.md
---

# Goal

Introduce a first-class routine-definition substrate for recurring anchors such as meds, prep, shutdown, and writing blocks without creating a second hidden calendar authority.

## Scope

- routine definition model and storage
- optional project association for routines
- recurrence and preferred-window fields
- routine projection into project overview, `Now`, and future HUD consumers

## Requirements

- routines are definitions, not silent replacement calendar events
- routine block semantics are explicit (`busy`, `free`, `preferred`, `ritual`)
- project-linked routines remain optional and local-first
- routine outputs are inspectable and explainable

## Suggested write scope

- schema/domain for routine definitions
- service layer for routine projection
- DTOs for routine summaries and upcoming anchors
- docs/spec alignment with task HUD ritual planning

## Acceptance criteria

- Vel has one backend representation for recurring anchors instead of scattered one-off logic
- project surfaces can show relevant routines without inventing frontend-only rules
- routine semantics are clearly separated from provider calendar truth
