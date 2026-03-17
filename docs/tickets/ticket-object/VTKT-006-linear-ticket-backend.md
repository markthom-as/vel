---
id: VTKT-006
title: Linear ticket backend
status: proposed
priority: P2
estimate: 3-5 days
dependencies:
  - VTKT-001
  - VTKT-003
---

# Goal

Add Linear as a provider-backed ticket backend.

# Scope

- map Linear issue identity, workflow state, cycle, estimate, and project metadata
- preserve parent/child references and team key

# Deliverables

- Linear provider adapter
- provider metadata mapping docs
- fixtures for common Linear issue shapes

# Acceptance criteria

- Linear issues normalize into canonical tickets without leaking Linear-specific workflow semantics into core fields

# Notes

Keep the core canonical. Preserve the provider richness in metadata.
