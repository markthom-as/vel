---
id: SWARM-001
title: Add Swarm Task and Work Unit domain model
status: open
owner: agent
priority: p0
area: runtime/orchestration
depends_on: []
---

# Goal

Introduce explicit `SwarmTask` and `WorkUnit` domain types so parallel orchestration is modeled as a task graph rather than free-form prompt chaining.

# Tasks

1. Add domain types for `SwarmTask`, `SwarmTaskId`, `WorkUnit`, `WorkUnitId`, `WorkerClass`, `WorkUnitState`, `Budget`, and `ResultContractRef`.
2. Add dependency graph fields so work units can declare predecessors and fan-out safely.
3. Define lifecycle states for work units: `created`, `queued`, `running`, `waiting`, `completed`, `failed`, `cancelled`, `expired`.
4. Add storage/API contract shapes for top-level task inspection and per-work-unit inspection.

# Acceptance Criteria

- Swarm tasks support parent task, output spec, deadline, side-effect policy, and budget.
- Work units support dependencies, allowed tools, memory scope, TTL, and result-contract metadata.
- The task model is explicit enough to back a DAG scheduler without ad hoc side channels.

# Spec reference

- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md) — Swarm Task Model, Dependency Graph, Result Contracts
- [docs/tickets/orchestration/001_task_model.md](../orchestration/001_task_model.md)
