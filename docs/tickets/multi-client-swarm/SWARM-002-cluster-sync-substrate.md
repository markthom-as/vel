---
id: SWARM-002
title: Build append-only cluster sync substrate
status: open
owner: agent
priority: p0
area: sync/replication
depends_on: [SWARM-001]
---

# Goal

Build the durable cluster sync foundation for nodes, actions, signals, work-unit events, and downstream state snapshots.

# Tasks

1. Add per-node identity and durable sync cursor storage.
2. Define append-only log entry schema with `origin_node_id`, `origin_local_seq`, `authority_epoch`, `idempotency_key`, and payload metadata.
3. Add first-class transport metadata for Tailscale routing, including configured tailnet hostname/IP awareness and preferred sync base URL selection.
4. Implement upstream sync ingestion for actions/signals/work-unit events with idempotent retry handling.
5. Implement downstream publication of versioned state snapshots for current context, nudges, commitments, and swarm metadata.

# Acceptance Criteria

- Nodes can sync append-only upstream entries without duplicate effects.
- Downstream snapshots are versioned and authority-tagged.
- Durable cursors survive restart and support replay.
- Tailscale-backed node routing is part of the sync substrate, not bolted on later.
- No raw SQLite file sync is required for correctness.

# Spec reference

- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md) — Log Model, Cursors, Idempotency, State Publication Model
- [docs/specs/vel-distributed-and-ambient-architecture-spec.md](../../specs/vel-distributed-and-ambient-architecture-spec.md) — Replication Model, Sync Semantics
