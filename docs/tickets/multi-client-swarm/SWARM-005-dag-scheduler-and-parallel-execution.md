---
id: SWARM-005
title: Implement DAG scheduler and bounded parallel execution
status: open
owner: agent
priority: p1
area: swarm/scheduler
depends_on: [SWARM-001, SWARM-004]
---

# Goal

Implement the swarm scheduler that can fan out independent work units in parallel while respecting dependencies, budgets, and side-effect policy.

# Tasks

1. Build a DAG-aware scheduler that releases work units only when dependencies are satisfied.
2. Enforce per-task and per-work-unit budgets for time, concurrency, tools, and side effects.
3. Support safe parallel fan-out for retrieval and independent analysis branches.
4. Add cancellation, waiting, timeout, and expiry handling across work-unit trees.

# Acceptance Criteria

- Independent work units run in parallel.
- Dependency-unsafe or conflicting writes remain serialized.
- Budget violations are rejected before runaway execution.
- Receipts capture each work-unit claim/start/completion so retries only reissue units whose previous receipt shows failure or expiry, and `GET /v1/sync/work-queue` now reports pending batches after terminal receipts are filtered.
- Scheduler state is durable enough to survive restart or replay, and the receipt lane exposes TTL/reclaim semantics so dangling `claimed` receipts older than the stale window can be reassigned without duplicating side effects.
- A background scheduler loop pulls `POST /v1/sync/work-queue/claim-next` through the loops runtime, publishes loop events, and keeps receipt transitions in lockstep with worker self-polling so retries obey backoff without manual operator intervention.

# Spec reference

- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md) — Dependency Graph, Parallel Execution Rules, Budget Model
- [docs/specs/vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md) — Agent Lifecycle, Runtime Budgets
