---
id: SWARM-004
title: Implement worker presence and capacity registry
status: in_progress
owner: agent
priority: p1
area: swarm/registry
depends_on: [SWARM-002, SWARM-003]
---

# Goal

Create the cluster-visible registry for worker presence, capacity, and capabilities that the swarm scheduler will use for placement and load balancing.

# Current implemented slice

Partially landed:

- read/write worker presence surfaces now exist at `GET /v1/cluster/workers`, `GET /v1/sync/cluster`, and `POST /v1/sync/heartbeat`
- worker metadata is stored in a durable `cluster_workers` registry with bounded expiry
- current worker metadata includes `worker_classes`, transport/reachability metadata, Tailscale preference fields, and max/current/available concurrency

Still missing from this ticket:

- multi-node registry rather than local-node-only reporting
- persisted failure-rate and assignment-receipt history
- scheduler consumption of these read models for actual placement

# Tasks

1. Add worker presence schema covering `worker_classes`, `max_concurrency`, `current_load`, `reachability`, `latency_class`, `compute_class`, and `power_class`.
2. Include Tailscale reachability and preferred tailnet endpoint metadata in worker presence.
3. Implement heartbeats and expiry for worker presence.
4. Persist recent worker failures and assignment receipts for scheduling decisions.
5. Expose read models for the supervisor to query live cluster capacity.

# Acceptance Criteria

- Reachable workers publish bounded heartbeats.
- Stale workers age out predictably.
- Scheduler can inspect worker capacity without probing every node ad hoc.
- Tailnet-reachable workers are discoverable as first-class scheduling targets.
- Presence metadata is advisory and does not become a second source of truth for user state.

# Spec reference

- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md) — Worker Classes, Load Balancing, Scheduling Inputs
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md) — Worker Presence And Capacity
