---
id: SWARM-006
title: Implement supervisor integration and conflict handling
status: open
owner: agent
priority: p1
area: swarm/integration
depends_on: [SWARM-005]
---

# Goal

Implement the supervisor-side integration path that validates work-unit results, resolves conflicts, and emits canonical outputs.

# Tasks

1. Validate structured result contracts for every completed work unit.
2. Merge evidence bundles, analysis outputs, and action receipts into a coherent integrated result.
3. Apply deterministic conflict rules for stale, duplicate, or lower-confidence outputs.
4. Emit final artifacts, proposed actions, or clarification requests from one integration authority.

# Acceptance Criteria

- Workers cannot directly write canonical truth outside approved contracts.
- Integration remains centralized and inspectable.
- Conflicts produce deterministic outcomes and structured metadata.
- High-risk ambiguity routes to clarification rather than silent guessing.

# Spec reference

- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md) — Integration Rules, Safety Rules, Result Contracts
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md) — Conflict Model
