---
id: CAL-003
title: Live launched-session schema and session-model extensions
status: todo
priority: P0
dependencies:
  - CAL-001
  - projects/06-agent-session-registry
---

# Goal

Extend the planned `agent_sessions` model so Vel can represent live launched sessions on Connect instances rather than only generic project-linked sessions or transcript-derived activity.

# Scope

- add launched-session fields or related tables
- track instance identity, runtime identity, external session ref, launch origin, and native-open target
- preserve compatibility with existing project workspace session projections
- distinguish imported evidence from live controllable sessions

# Deliverables

- schema/migration plan
- domain/storage updates for launched-session metadata
- DTO/projection updates
- compatibility notes for projects workspace consumers

# Acceptance criteria

- A launched session can be represented durably with target instance and runtime metadata.
- Project workspace projections can expose whether a session is live, launch-backed, read-only, or transcript-only.
- Existing session work is extended rather than forked into a second session model.

# Notes

If the current `projects/06` ticket has not landed yet, adjust that implementation rather than creating overlapping models.
