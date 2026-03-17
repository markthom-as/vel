---
id: CAL-005
title: Session interaction actions and event lifecycle
status: todo
priority: P0
dependencies:
  - CAL-003
  - CAL-004
  - projects/07-session-outbox-feedback-and-settings
---

# Goal

Add durable interaction actions for launched sessions and make their lifecycle auditable through explicit events and state transitions.

# Scope

- send follow-up message / steering input
- stop or cancel session when supported
- expose open-native-surface action when supported
- integrate with session outbox and feedback concepts where appropriate
- emit durable events for interaction and lifecycle changes

# Deliverables

- APIs for message/steer/stop/open actions
- outbox/delivery semantics for launch-backed sessions
- event contract for launch/session action lifecycle
- failure and unsupported-capability handling rules

# Acceptance criteria

- Operators can interact with a launched session from Vel without relying on external tabs alone.
- Unsupported actions are surfaced honestly based on capability data.
- Session events are inspectable and suitable for replay/debugging.

# Notes

Prefer reusing the project-session control plane where possible instead of creating one-off action endpoints with overlapping semantics.
