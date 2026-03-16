---
id: SK-005
title: Expose code and documentation navigation APIs
status: proposed
priority: P1
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Provide query APIs so Vel can answer architecture and implementation questions from its self-model.

# Required API surface

- `find_symbol(name)`
- `find_module(name)`
- `find_documentation(entity)`
- `find_tests(entity)`
- `find_dependencies(entity)`
- `find_reverse_dependencies(entity)`
- `explain_component(name)`

# Tasks

1. Implement typed API layer over graph + evidence store.
2. Add fuzzy matching fallback with exact-match preference.
3. Return freshness, canonicality, and contradiction info in responses.
4. Add integration tests using representative repo fixtures.

# Acceptance Criteria

- API can answer “where is X implemented?” for indexed fixtures.
- API can list related docs/tests for a component.
- Ambiguous lookups return ranked candidates instead of brittle failure.
- Contradiction / uncertainty information survives the API boundary.

