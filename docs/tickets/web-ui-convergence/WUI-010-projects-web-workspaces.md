---
title: Ship web Projects tasks and sessions workspaces
status: todo
owner: agent
priority: P0
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-009-projects-registry-and-workspace-contract.md
  - WUI-003-realtime-and-mutation-reconciliation.md
labels:
  - projects
  - web
---

# Goal

Ship Projects as a first-class web surface using the shared shell, query, and realtime model.

## Scope

- Projects page shell
- tasks workspace
- sessions workspace
- project activity and quick actions where supported

## Requirements

1. Add `Projects` to the global shell as a first-class page.
2. Support project index, task operations, and session operations from one workspace model.
3. Reuse shared page-state, freshness, and mutation patterns.
4. Make Todoist write-through failures and outbox delivery state explicit.

## Write scope

- Projects page and child components
- project query hooks/resources
- task/session mutation paths

## Acceptance criteria

- Projects is visible and usable end-to-end from the web shell
- project tasks and sessions operate under the same mental model
- the page does not invent a bespoke data architecture outside the shared shell rules
