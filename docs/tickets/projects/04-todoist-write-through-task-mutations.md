---
title: Add Todoist write-through task mutations for project tasks
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Allow the Projects surface to create and update project tasks while preserving Todoist authority when connected.

## Scope
- add project-scoped task mutation endpoints
- reuse commitments as the canonical Vel task object
- write through to Todoist for Todoist-backed projects when possible
- reconcile local commitment mirror after successful remote write

## Routes
- `POST /v1/projects/:slug/tasks`
- `PATCH /v1/projects/:slug/tasks/:id`
- `POST /v1/projects/:slug/tasks/:id/done`
- `POST /v1/projects/:slug/tasks/:id/cancel`

## Requirements
- if project has `todoist_project_id` and Todoist is connected, create/update remotely first
- persist enough metadata to preserve Todoist linkage (`todoist_task_id`, raw labels if needed)
- explicit failure state when remote write fails
- do not fake success on remote failure
- support local-only fallback only when intentionally allowed by project or integration settings

## Suggested implementation
- extend integrations service with Todoist create/update/close helpers
- add a project task mutation orchestrator in `services/projects.rs`
- normalize request payload into commitment create/update structures

## Tests
- create task for local-only project
- create task for Todoist-backed project with mocked/snapshot path
- mark Todoist-backed task done and verify local reconciliation
- remote failure yields error and no silent local success

## Done when
- project task mutations work end-to-end
- Todoist-backed flows are test-covered
- failure semantics are explicit
