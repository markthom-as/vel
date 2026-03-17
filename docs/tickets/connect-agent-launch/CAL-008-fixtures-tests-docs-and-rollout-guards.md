---
id: CAL-008
title: Fixtures, tests, docs, and rollout guards
status: todo
priority: P1
dependencies:
  - CAL-001
  - CAL-002
  - CAL-003
  - CAL-004
  - CAL-005
  - CAL-006
  - CAL-007
---

# Goal

Harden the Connect-backed agent launch feature with fixtures, tests, rollout controls, and documentation updates that distinguish shipped capability from planned breadth.

# Scope

- add fixtures for instance/runtime combinations and launch outcomes
- add tests for compatibility selection, launch, session projection, and action handling
- add feature flags or rollout guards where risk is non-trivial
- update status/docs/user-facing operator guidance as implementation lands

# Deliverables

- automated tests
- seed/fixture coverage
- rollout strategy notes
- status/doc updates

# Acceptance criteria

- The happy path and key failure paths are covered by tests.
- Docs do not overclaim runtime/vendor breadth beyond what is actually implemented.
- Rollout risk is bounded with explicit flags or staging where needed.

# Notes

This ticket should be the point where the pack is reconciled back into `docs/status.md`.
