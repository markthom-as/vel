---
title: Agent Runtime Ticket Pack
status: todo
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
  - agent-runtime
  - tickets
created: 2026-03-15
updated: 2026-03-17
---

# Agent Runtime — Ticket Pack

Implementation tickets for the Vel agent runtime: lifecycle, executor, memory contracts, introspection, and replay.

## Pack schema

- `class: expansion`
- `authority: design`
- `status_model: todo | in_progress | deferred | done`
- `source_of_truth: docs/status.md`

## Entry criteria

Use this pack when:

- extending the current run/execution substrate toward a broader agent runtime,
- designing memory, replay, or supervised execution behavior,
- reconciling agent-runtime plans with the shipped bounded execution surfaces.

Do not use this pack alone to infer that the full agent runtime already exists.

**Spec:** [docs/specs/vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md)

Related specs:

- [vel-cognitive-loop-spec.md](../../specs/vel-cognitive-loop-spec.md) — Observe → Evaluate → Suggest → Act → Reflect
- [vel-stavrobot-integration-spec.md](../../specs/vel-stavrobot-integration-spec.md) — capability isolation, tiered memory, plugins vs skills

## Tickets

| ID | Title | Status |
|----|--------|--------|
| TICKET-001 | Runtime Skeleton | todo |
| TICKET-002 | Executor Integration | todo |
| TICKET-003 | Memory Contracts | todo |
| TICKET-004 | Introspection + HUD | todo |
| TICKET-005 | Replay + Reflection | todo |

## Exit criteria

- the pack no longer relies on ambiguous open/review-style planning status,
- runtime-expansion work is clearly separated from current shipped execution behavior,
- related follow-on tickets cite current bounded-runtime truth before adding breadth.
