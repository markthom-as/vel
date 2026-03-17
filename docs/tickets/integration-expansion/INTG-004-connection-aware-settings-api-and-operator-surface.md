---
id: INTG-004
title: Connection-aware settings API and operator surface
status: proposed
priority: P0
estimate: 4-6 days
dependencies:
  - INTG-001
  - INTG-003
---

# Goal

Replace the one-card-per-family operator model with family, provider, and connection-aware settings and status surfaces.

# Scope

- Add APIs to list providers and connections.
- Add create/update/disable flows for connections.
- Add per-connection sync controls and history.
- Preserve current integrations UI until the new read model is available.
- Make room for multiple messaging providers, note sources, transcript sources, and workspace accounts.

# Deliverables

- `/api/integrations/providers`
- `/api/integrations/connections`
- connection detail and sync history endpoints
- web settings model and UI changes
- API tests and decoder updates

# Acceptance criteria

- Operator can see multiple connections in the same family.
- Operator can run sync for a single connection.
- Connection guidance and failures are shown at the right level.
- Current integration behavior continues to work during migration.

# Notes

If this remains family-only, every new provider will turn Settings into a lie.
