---
id: APPLE-013
title: Enforce monorepo boundaries for Apple clients
status: proposed
owner: agent
priority: p1
area: architecture
depends_on: [APPLE-001, APPLE-002]
---

# Goal

Preserve the benefits of monorepo without letting it become one giant dependency swamp.

# Rules to enforce

- Apple app targets may depend on local Apple packages
- Apple packages may depend on shared repo contracts/interfaces
- Core business logic should not depend on Apple packages
- UI packages may not define canonical business policy
- Widget/watch targets should consume already-shaped state, not re-derive domain policy independently

# Implementation options

Use one or more of:

- package boundaries
- lint/import rules
- architectural tests
- documented dependency graph
- CI checks on forbidden imports or path usage

# Deliverable

Create `clients/apple/docs/dependency-graph.md` with a simple graph and examples of allowed vs forbidden dependency directions.

# Acceptance criteria

- at least one automated check exists
- forbidden dependency example fails clearly
