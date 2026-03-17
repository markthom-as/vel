---
id: VTKT-010
title: Ticket tests, fixtures, and rollout
status: proposed
priority: P0
estimate: 2-4 days
dependencies:
  - VTKT-001
  - VTKT-002
  - VTKT-003
---

# Goal

Create fixture coverage and rollout guidance for native and provider-backed tickets.

# Scope

- native Vel ticket fixtures
- GitHub, Linear, Jira, and Todoist fixture sets
- migration notes from commitment-only assumptions
- docs separating shipped ticket behavior from planned work

# Deliverables

- fixture pack
- integration tests
- rollout checklist
- docs updates

# Acceptance criteria

- provider adapters can be validated against shared fixtures
- canonical ticket/commitment distinction is documented clearly
- rollout path is explicit and test-backed

# Notes

If the fixtures are weak, the semantics will drift immediately.
