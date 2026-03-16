---
title: Add retrieval and validation resolvers
status: todo
priority: P1
owner: core-runtime
labels: [uncertainty, retrieval, validation]
---

# Goal

Collapse uncertainty through evidence gathering or execution checks before escalating to a human.

# Deliverables

- `packages/core/resolvers/retrieval-resolver.ts`
- `packages/core/resolvers/validation-resolver.ts`
- integration hooks for repo inspection, search, typecheck, targeted tests, or dry-run execution

# Requirements

- Retrieval resolver should attach evidence refs back to the uncertainty item.
- Validation resolver should support dry-run and low-cost checks first.
- Both resolvers must emit structured outcomes that can update or close uncertainties.
- Validation failures should increase predictive uncertainty rather than vanish into logs.

# Acceptance criteria

- One integration test shows repo inspection resolving style/pattern uncertainty.
- One integration test shows a dry-run resolving predictive uncertainty.
- Evidence refs are visible from task history.

# Notes

Vel should earn the right to interrupt the user by first checking the obvious places.
