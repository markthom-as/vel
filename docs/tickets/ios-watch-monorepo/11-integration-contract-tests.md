---
id: APPLE-011
title: Add integration and contract tests across Apple flows
status: proposed
owner: agent
priority: p0
area: tests
depends_on: [APPLE-006, APPLE-007]
---

# Goal

Test the real cross-boundary flows rather than congratulating ourselves because a couple view models compile.

# Required test categories

- fixture decode / contract tests
- sync reconciliation tests
- notification action idempotency tests
- watch-originated mutation tests
- offline queue persistence tests
- pre-meeting medication flow test

# Suggested fixtures

Include representative fixtures for:

- due medication before meeting
- acknowledged but unconfirmed med state
- overdue task with rising risk band
- successful action confirmation from server
- duplicate delivery / replay

# Acceptance criteria

- test suite runs from scripted command
- at least one end-to-end contract flow is fixture-driven
- failures point to the broken layer clearly
