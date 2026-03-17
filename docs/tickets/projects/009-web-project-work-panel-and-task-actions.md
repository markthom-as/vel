---
id: VEL-PROJ-009
title: Build project work panel with Todoist-backed task actions
status: proposed
priority: P1
estimate: 3-5 days
dependencies:
  - VEL-PROJ-003
  - VEL-PROJ-008
labels:
  - web
  - tasks
  - todoist
---

# Goal

Render project work items and let the user add, tag, complete, reopen, and edit them from the Projects page.

# Scope

- Build a work panel component for project tasks/work items.
- Add task creation form with:
  - title
  - project
  - labels/tags
  - due date/time
- Add inline actions for:
  - complete/reopen
  - tag add/remove
  - quick edit
- Surface sync status and source badges.
- Add optimistic updates only where rollback is deterministic.

# Deliverables

- work panel components
- task create/edit action handlers
- UI state for unsynced/manual-fallback tasks if API returns degraded mode
- tests for create/tag/complete flows

# Acceptance criteria

- User can create a task from the project page.
- User can assign at least one tag during creation.
- User can complete and reopen a task inline.
- Sync/degraded status is clearly visible.
- UI tests cover at least one happy path and one failure/degraded path.

# Notes for agent

The task form should feel like an instrument panel, not a tax return.
