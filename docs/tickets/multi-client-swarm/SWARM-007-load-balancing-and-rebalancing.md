---
id: SWARM-007
title: Add cluster-aware load balancing and rebalancing
status: open
owner: agent
priority: p2
area: swarm/load-balancing
depends_on: [SWARM-004, SWARM-005, SWARM-006]
---

# Goal

Place work on the most appropriate available workers and rebalance when nodes become overloaded, slow, or unavailable.

# Tasks

1. Implement placement heuristics using worker load, queue depth, reachability, compute class, power class, thermal constraints, data locality, and Tailscale path availability.
2. Prioritize interactive low-latency work differently from heavy synthesis or batch analysis.
3. Prefer tailnet-reachable cross-machine workers before ad hoc remote fallbacks when policy allows.
4. Add spillover and load-shedding rules that preserve policy and authority boundaries.
5. Support mid-run rebalancing for idempotent work units and receipt-aware retry handling for side-effecting units.

# Acceptance Criteria

- Interactive work prefers low-latency workers.
- Heavy work prefers compute-rich workers.
- Edge devices are not saturated with inappropriate background work.
- Tailscale connectivity materially influences placement and rebinding decisions.
- Rebalancing never bypasses authority, memory-scope, or side-effect rules.

# Spec reference

- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md) — Load Balancing, Rebalancing
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md) — Load-Balanced Placement Support
