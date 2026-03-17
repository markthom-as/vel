---
id: INTG-001
title: Foundation family, provider, and connection model
status: proposed
priority: P0
estimate: 3-5 days
dependencies: []
---

# Goal

Introduce the canonical integration substrate: families, providers, connections, provider-scoped provenance refs, and connection event history.

# Scope

- Add `integration_family`, `integration_provider`, and `integration_connection` domain types in `vel-core`.
- Add storage tables for:
  - integration connections
  - connection settings references
  - connection lifecycle / sync events
- Define a canonical source object reference shape that every adapter can attach to emitted objects.
- Preserve current family-oriented config behavior during the migration path.

# Deliverables

- core domain types and enums
- migrations for connection storage
- storage CRUD for connections
- connection event append/list support
- docs for provider/connection boundaries

# Acceptance criteria

- Core can represent multiple providers in one family.
- Core can represent multiple connections for one provider.
- Every connection has a stable id and provider/family provenance.
- Existing single-source integrations can be mapped into the new model without ambiguity.

# Notes

Do not start with provider-specific fields in core. The first job is to stop pretending one provider equals one family card.
