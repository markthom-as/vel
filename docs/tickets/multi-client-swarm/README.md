---
title: Multi-Client Swarm Ticket Pack
status: in_progress
owner: agent
class: expansion
authority: design
status_model:
  - todo
  - in_progress
  - deferred
  - done
source_of_truth: docs/status.md
labels:
  - planning
  - swarm
  - cluster-sync
  - tickets
created: 2026-03-17
updated: 2026-03-17
---

# Multi-Client Swarm — Ticket Pack

Implementation tickets for the Vel multi-client swarm, load balancing, and cluster-aware sync substrate.

This pack assumes Tailscale is a first-class transport for multi-machine Vel clusters, not an optional afterthought.

## Pack schema

- `class: expansion`
- `authority: design`
- `status_model: todo | in_progress | deferred | done`
- `source_of_truth: docs/status.md`

## Entry criteria

Use this pack when:

- extending the shipped cluster/bootstrap/control-plane slice into broader orchestration,
- designing scheduler, supervisor, or multi-worker behavior beyond the current receipt/queue runtime,
- reconciling swarm plans with the current sync implementation.

Do not use this pack alone to claim that distributed execution is already shipped.

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
| SWARM-001 | Add Swarm Task and Work Unit domain model | todo |
| SWARM-002 | Build append-only cluster sync substrate | todo |
| SWARM-003 | Add authority epoch and temporary authority handoff | in_progress |
| SWARM-004 | Implement worker presence and capacity registry | in_progress |
| SWARM-005 | Implement DAG scheduler and bounded parallel execution | todo |
| SWARM-006 | Implement supervisor integration and conflict handling | todo |
| SWARM-007 | Add cluster-aware load balancing and rebalancing | todo |
| SWARM-008 | Add observability, replay, and end-to-end failover tests | todo |

## Partial implementation note

The repo now has an initial shipped slice of the cluster/sync design:

- `GET /v1/cluster/bootstrap`
- `GET /v1/cluster/workers`
- `GET /v1/sync/bootstrap`
- `GET /v1/sync/cluster`
- `POST /v1/sync/heartbeat`
- `GET /v1/sync/work-assignments`
- `POST /v1/sync/work-assignments`
- `PATCH /v1/sync/work-assignments`
- `GET /v1/sync/work-queue`
- `POST /v1/sync/work-queue/claim-next`
- `POST /v1/sync/actions`

These surfaces provide authority metadata, Tailscale-aware routing metadata, unified client cache hydration, low-risk action batching, a durable heartbeat-backed worker registry, receipt-aware work-unit assignment, queue inspection for pending worker-class work, queue-level retry/reclaim metadata, a `claim-next` scheduler primitive, and first-pass queued-work placement metadata. A background scheduler loop now polls `POST /v1/sync/work-queue/claim-next` via the runtime loops surface, keeps receipts in sync with retries/backoff, and surfaces loop events for operators.

Queue inspection now integrates the scheduler's retry/reclaim rules: stale `claimed` receipts (currently >300 s) are visible as reclaimable units, duplicate queue attempts check the latest receipt before enqueuing, failures surface retriable reasons instead of silently dropping the work, and retry timing/exhaustion is now driven by per-work-type policy config rather than hardcoded queue behavior.

They do not yet provide:

- multi-node membership and replication
- authority claim/handoff
- scheduler-driven distributed work placement

## Intended execution order

1. SWARM-001
2. SWARM-002
3. SWARM-003
4. SWARM-004
5. SWARM-005
6. SWARM-006
7. SWARM-007
8. SWARM-008

## Exit criteria

- planned swarm-specific work is either implemented, deferred, or re-scoped,
- the boundary between shipped sync/runtime behavior and future orchestration is explicit,
- follow-on work cites current cluster truth instead of assuming the whole spec is live.
