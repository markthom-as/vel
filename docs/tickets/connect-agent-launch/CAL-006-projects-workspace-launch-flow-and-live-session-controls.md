---
id: CAL-006
title: Projects workspace launch flow and live session controls
status: todo
priority: P0
dependencies:
  - CAL-002
  - CAL-003
  - CAL-004
  - CAL-005
---

# Goal

Make Connect-backed launch and live-session interaction available from the Projects workspace in web and CLI surfaces using the shared workspace contract.

# Scope

- add launch affordance to Projects workspace
- add instance picker and runtime picker
- render live launched-session metadata and controls
- keep web and CLI aligned on the same projection and action model

# Deliverables

- workspace projection additions
- web UI launch composer and session controls
- CLI commands for launch and interaction
- realtime invalidation or event wiring for session updates

# Acceptance criteria

- A user can launch a compatible runtime for a project from Vel.
- Launched sessions appear in the same project workspace session list as other sessions.
- Web and CLI do not diverge into separate session/launch contracts.

# Notes

This should extend the existing Projects surface, not create a detached Connect-only operator page.
