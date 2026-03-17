---
title: Multi-Client Swarm Ticket Pack
status: todo
owner: agent
labels:
  - planning
  - swarm
  - cluster-sync
  - tickets
created: 2026-03-17
---

# Multi-Client Swarm — Ticket Pack

Implementation tickets for the Vel multi-client swarm, load balancing, and cluster-aware sync substrate.

This pack assumes Tailscale is a first-class transport for multi-machine Vel clusters, not an optional afterthought.

**Specs:**

- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md)
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md)

Related specs:

- [vel-distributed-and-ambient-architecture-spec.md](../../specs/vel-distributed-and-ambient-architecture-spec.md)
- [vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md)
- [vel-rust-swift-boundary-spec.md](../../specs/vel-rust-swift-boundary-spec.md)

## Tickets

| ID | Title | Status |
|----|--------|--------|
| SWARM-001 | Add Swarm Task and Work Unit domain model | open |
| SWARM-002 | Build append-only cluster sync substrate | open |
| SWARM-003 | Add authority epoch and temporary authority handoff | open |
| SWARM-004 | Implement worker presence and capacity registry | open |
| SWARM-005 | Implement DAG scheduler and bounded parallel execution | open |
| SWARM-006 | Implement supervisor integration and conflict handling | open |
| SWARM-007 | Add cluster-aware load balancing and rebalancing | open |
| SWARM-008 | Add observability, replay, and end-to-end failover tests | open |

## Intended execution order

1. SWARM-001
2. SWARM-002
3. SWARM-003
4. SWARM-004
5. SWARM-005
6. SWARM-006
7. SWARM-007
8. SWARM-008

## Status convention

- `open`
- `in_progress`
- `blocked`
- `review`
- `done`
