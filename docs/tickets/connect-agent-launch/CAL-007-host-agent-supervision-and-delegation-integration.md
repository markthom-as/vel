---
id: CAL-007
title: Host-agent supervision and delegation integration
status: todo
priority: P1
dependencies:
  - CAL-002
  - CAL-004
  - CAL-005
  - CAL-006
  - SWARM-006
---

# Goal

Let Vel's main host agent discover compatible Connect instances, launch external sessions, and supervise them as bounded workers within the existing orchestration model.

# Scope

- add host-agent read access to instance/runtime compatibility data
- allow host agent to initiate launches
- track supervisor linkage between host-agent work and launched external sessions
- support inspection, follow-up input, and blocked-state escalation

# Deliverables

- host-agent orchestration contract updates
- supervisor linkage in session metadata
- delegation/integration rules for external session workers
- docs clarifying authority boundaries

# Acceptance criteria

- The host agent can choose and launch a compatible external runtime for a task.
- Launched sessions remain subordinate workers rather than alternate planners.
- User-visible surfaces can show that a session is host-agent-supervised.

# Notes

Keep this aligned with the swarm authority model: one planner/integrator, many bounded workers.
