---
id: SWARM-008
title: Add observability, replay, and end-to-end failover tests
status: open
owner: agent
priority: p1
area: swarm/testing
depends_on: [SWARM-006, SWARM-007]
---

# Goal

Make the swarm and cluster-sync layers inspectable, replayable, and testable under normal, degraded, and failover conditions.

# Tasks

1. Emit durable events for swarm task creation, work-unit lifecycle, integration, placement, rebalancing, and authority changes.
2. Add inspection surfaces for top-level task status, work-unit DAG, worker assignment, receipts, and final integration summaries.
3. Add replay tooling for sync-log and swarm-log driven reconstruction.
4. Add end-to-end tests for offline action sync, authority failover, worker expiry, rebalancing, deterministic conflict resolution, and Tailscale-routed discovery/rebinding.

# Acceptance Criteria

- Operators can inspect one swarm task end-to-end.
- Replay can reconstruct task and sync behavior from durable events.
- Failover and conflict behavior are exercised by automated tests.
- Tailnet-backed routing and authority rebinding are covered by automated tests.
- The system remains debuggable under parallel execution.

# Spec reference

- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md) — Observability, Cancellation And Preemption, Acceptance Criteria
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md) — Testing Requirements
