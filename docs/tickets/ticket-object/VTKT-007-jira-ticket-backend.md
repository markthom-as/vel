---
id: VTKT-007
title: Jira ticket backend
status: proposed
priority: P2
estimate: 4-6 days
dependencies:
  - VTKT-001
  - VTKT-003
---

# Goal

Add Jira as a provider-backed ticket backend with support for workflow state, issue type, and board/sprint metadata.

# Scope

- map Jira issue keys and project keys
- support issue type and state normalization
- preserve sprint, epic, parent, and custom fields in structured metadata

# Deliverables

- Jira provider adapter
- Jira-to-canonical mapping layer
- fixture coverage for representative issue types

# Acceptance criteria

- Jira issues normalize into canonical tickets
- provider custom fields remain inspectable and structured

# Notes

Jira is exactly where provider-specific sprawl will eat the model if the core schema is weak.
