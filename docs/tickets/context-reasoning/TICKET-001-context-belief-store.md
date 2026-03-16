---
title: Implement Context Belief Store
status: open
---

# Goal

Create a persistent belief store representing Vel's understanding of user context.

# Tasks

1. Create database schema for ContextBelief.
2. Implement CRUD API.
3. Implement confidence ranking queries.
4. Implement expiration logic for scoped beliefs.
5. Implement suppression flags.

# Acceptance Criteria

- Beliefs stored and retrievable.
- Expired beliefs auto-invalidated.
- Confidence sorting supported.
