---
title: Harden tests, fixtures, docs, and rollout for Projects
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Finish the feature like adults: tests, fixtures, status updates, and rollout guardrails.

## Scope
- add backend integration tests for project routes and mutations
- add frontend tests for Projects page panes
- add CLI coverage where feasible
- add seed fixtures covering Todoist-backed project + external sessions
- update docs/status and relevant API docs
- add feature flag(s) if rollout risk warrants it

## Requirements
- include at least one canonical test fixture with:
  - multiple projects
  - Todoist-backed tasks
  - local-only tasks
  - active Codex/Claude/Vel sessions
  - queued outbox items
- update `docs/status.md` with implemented vs planned truth
- update API docs if routes are shipped
- note any deferred adapter delivery limitations honestly

## Done when
- tests exist across backend/frontend where appropriate
- docs are aligned
- feature is roll-outable without lying about capability
