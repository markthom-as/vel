---
id: VEL-PROJ-013
title: Harden project workspace with tests, fixtures, docs, and rollout notes
status: proposed
priority: P1
estimate: 2-3 days
dependencies:
  - VEL-PROJ-009
  - VEL-PROJ-010
  - VEL-PROJ-011
  - VEL-PROJ-012
labels:
  - tests
  - docs
  - rollout
---

# Goal

Finish the Projects feature like adults: with fixtures, integration tests, documentation, and explicit rollout boundaries.

# Scope

- Add canonical fixtures covering:
  - Todoist-backed project tasks
  - multiple project aliases
  - Vel + Codex + Claude transcript sessions
  - queued manual-dispatch message
  - steering and feedback history
- Add backend integration tests for main flows.
- Add frontend tests for core workspace interactions.
- Update `docs/status.md` and docs index.
- Add operator notes for degraded mode and capability gaps.

# Deliverables

- test fixtures in backend/frontend appropriate locations
- integration coverage for read/write project workspace flows
- docs updates referencing the new spec and ticket pack
- rollout section documenting feature flag or staged enablement if used

# Acceptance criteria

- Main read/write flows are covered by tests.
- Docs reflect what actually shipped.
- Degraded/manual-dispatch states are documented.
- The feature can be enabled without pretending unsupported external adapters are magically complete.

# Notes for agent

This is the anti-fantasy ticket. Ship the truth, not the dream sequence.
