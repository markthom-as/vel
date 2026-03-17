---
id: VEL-PROJ-005
title: Add projects API routes and live update contracts
status: proposed
priority: P0
estimate: 2-3 days
dependencies:
  - VEL-PROJ-003
  - VEL-PROJ-004
labels:
  - api
  - websocket
  - projects
---

# Goal

Expose project workspace data and project task mutations through typed APIs, plus live-update hooks for the web client.

# Scope

Add routes for:

- `GET /api/projects`
- `GET /api/projects/:slug`
- `GET /api/projects/:slug/workspace`
- `POST /api/projects/:slug/tasks`
- `PATCH /api/projects/:slug/tasks/:id`
- `POST /api/projects/:slug/tasks/:id/complete`
- `POST /api/projects/:slug/tasks/:id/reopen`
- `POST /api/projects/:slug/tasks/:id/tags`
- `DELETE /api/projects/:slug/tasks/:id/tags/:tag`

Also define websocket events for project workspace refreshes.

# Deliverables

- new route module(s), likely `crates/veld/src/routes/projects.rs`
- route mounting in `crates/veld/src/app.rs`
- DTO additions in `vel-api-types`
- websocket event enums/payloads for project updates
- backend tests for read and mutation endpoints

# Acceptance criteria

- Projects list and detail endpoints return typed data.
- Task mutation endpoints write through to Todoist service or clear degraded/manual state.
- Websocket payloads are typed and consistent with existing chat event conventions.
- Tests cover at least one full create-tag-complete-read cycle.

# Notes for agent

Mirror the existing chat API style. Reinventing the API response envelope here would be comedy, not progress.
