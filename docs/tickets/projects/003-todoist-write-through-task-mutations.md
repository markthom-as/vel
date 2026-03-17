---
id: VEL-PROJ-003
title: Implement Todoist write-through task creation and tagging APIs
status: proposed
priority: P0
estimate: 3-5 days
dependencies:
  - VEL-PROJ-001
  - VEL-PROJ-002
labels:
  - todoist
  - integrations
  - write-path
---

# Goal

Let the Projects page actually add and tag tasks instead of merely gazing wistfully at them.

# Scope

Extend the existing Todoist integration service to support:

- list Todoist projects for selection
- list or normalize Todoist labels/tags
- create task
- update task content/title
- update due date/time
- add/remove labels
- close/reopen task
- move task project where supported
- reconcile write results back into commitments and signals

# Required implementation details

- Build on `crates/veld/src/services/integrations.rs`, where Todoist read sync already exists.
- Add typed request/response helpers for Todoist write endpoints.
- Preserve graceful fallback behavior when Todoist is disconnected.
- When write succeeds, update or re-sync the matching commitment projection.
- When write cannot execute, return a degraded but explicit API response, not silent failure.

# Deliverables

- Todoist service methods for task mutation
- API DTOs for create/update/tag operations
- route handlers or service functions consumed by project APIs
- tests with mocked Todoist responses or service-layer fakes

# Acceptance criteria

- User can create a Todoist-backed task through Vel.
- User can apply at least one tag/label in the same flow.
- Created task becomes visible in the project workspace after reconciliation.
- Close/reopen and tag update flows are covered by tests.
- Error states are explicit and preserve operator trust.

# Notes for agent

No shadow task store. If Todoist is connected, write through to Todoist. If it is not, later tickets can support local draft mode with a giant “not synced” sign hanging from it.
