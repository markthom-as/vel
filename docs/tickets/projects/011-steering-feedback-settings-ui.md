---
id: VEL-PROJ-011
title: Add steering, feedback, and session settings controls to project chats
status: proposed
priority: P1
estimate: 2-4 days
dependencies:
  - VEL-PROJ-007
  - VEL-PROJ-010
labels:
  - web
  - control-plane
  - settings
---

# Goal

Expose the agent control plane in the UI so the operator can actually steer and critique project-specific sessions.

# Scope

- Add steering creation UI
- Add feedback UI (rating + notes at minimum)
- Add per-session settings editor for fields such as:
  - autonomy level
  - approval requirement
  - style/tone hints
  - preferred route/agent
  - noisy/paused state
- Display recent steering and feedback history for a session

# Deliverables

- steering modal/drawer/form
- feedback form component
- session settings form component
- resource wiring and local state management
- tests for control submissions and state refresh

# Acceptance criteria

- User can store structured steering for a session.
- User can store project-scoped feedback for a session.
- User can update session settings and see the result reflected in the UI.
- Controls respect capability flags and degraded/manual states.

# Notes for agent

This is where operator sovereignty lives. Don’t bury it under six clicks and a euphemism.
