---
id: SWARM-003
title: Add authority epoch and temporary authority handoff
status: in_progress
owner: agent
priority: p0
area: cluster/authority
depends_on: [SWARM-002]
---

# Goal

Make cluster authority explicit so temporary failover and reconciliation can happen without hidden split-brain behavior.

# Current implemented slice

Partially landed:

- bootstrap and sync-cluster payloads now publish `authority_node_id`, `authority_epoch`, and cluster-view metadata
- clients have Tailscale-aware bootstrap routing metadata and operator inspection surfaces

Still missing from this ticket:

- temporary authority claim behavior
- durable authority-change events
- reconciliation from temporary authority back to preferred authority
- stale-authority rejection and full rebinding flow

# Tasks

1. Add authority metadata surfaces: `authority_node_id`, `authority_epoch`, and cluster-view version.
2. Implement configured fallback ordering and temporary authority claim behavior.
3. Make Tailscale-aware authority routing first-class so clients can rebind across tailnet hosts without manual endpoint rewrites.
4. Require all sync exchanges and accepted log entries to carry authority metadata.
5. Implement reconciliation flow from temporary authority back to preferred authority at the log level.

# Acceptance Criteria

- Authority changes increment epoch and emit durable authority-change events.
- Clients can detect stale authority assumptions and rebind.
- Tailnet-based clients can follow authority handoff without bespoke per-client reconfiguration.
- Temporary authority can accept writes and later reconcile them back to preferred authority.
- Failover does not rely on ambiguous local heuristics.

# Spec reference

- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md) — Authority Epoch, Temporary Authority Handoff, Reconciliation After Failover
- [docs/specs/vel-distributed-and-ambient-architecture-spec.md](../../specs/vel-distributed-and-ambient-architecture-spec.md) — Temporary Authority
