---
title: "Enforce evaluate/read boundary across inference, risk, nudges, and explain"
status: done
owner: agent
type: architecture
priority: critical
created: 2026-03-15
depends_on: []
labels:
  - vel
  - architecture
  - backend
  - explainability
---
The repo is close to a coherent operating loop, but it is still vulnerable to a nasty category error: **read paths, evaluation paths, and persistence paths are not yet hard-separated enough**.

The good news: `routes/explain.rs` no longer appears to mutate risk state during `explain_commitment`, which is the right direction. The remaining work is to turn that from "better behavior" into a **repo invariant**.

## Why this matters

Vel is supposed to be explainable. That falls apart the second a read endpoint silently recomputes or mutates state. A system cannot honestly answer "why did I think this?" if the act of asking the question changes the thing being explained.

This is the highest-leverage architecture ticket because it prevents quiet semantic drift between:

- inference
- risk
- nudge generation
- explain routes
- CLI inspection commands

## Current smells

- `services/inference.rs`, `services/risk.rs`, and `services/nudge_engine.rs` are still tightly coupled conceptually and may still be callable in ways that blur recompute vs read.
- Explain routes are parsing persisted JSON directly, which is fine for now, but the contract for what counts as **persisted truth** vs **fresh evaluation** is still under-specified in code.
- The system has enough moving pieces now that an innocent future change could reintroduce side effects in read paths.

## Required outcome

Make the following architectural rule explicit and enforced:

### Read-only surfaces
These may **only** read persisted state:
- `GET /v1/explain/*`
- `GET /v1/context/current`
- `GET /v1/context/timeline`
- CLI inspect/explain commands

### Evaluation surfaces
These may recompute and persist:
- `POST /v1/evaluate`
- scheduled/background evaluation
- explicit refresh commands

### Persistence surfaces
These own writes of canonical state:
- inference persistence
- risk snapshot persistence
- nudge state transitions
- event emission

## Tasks

- Introduce an explicit service boundary for evaluation, e.g. `services/evaluate.rs` or equivalent orchestration layer.
- Ensure explain routes depend only on read/query services, never on recomputation services.
- Add a repo-level test that proves explain endpoints do not create new `commitment_risk`, `inferred_state`, `current_context`, or `nudge_events` rows.
- Add code comments and docstrings in the involved services stating whether a function is:
  - read-only
  - recompute-only
  - recompute-and-persist
- Audit CLI commands for the same boundary. `vel explain *` must be read-only; `vel evaluate` may recompute.

## Acceptance Criteria

- Explain endpoints and CLI explain commands are guaranteed read-only by tests.
- There is one explicit orchestration entry point for "compute current truth now".
- No route or CLI inspect path directly calls risk/inference recomputation unless it is explicitly an evaluate/refresh action.
- Architectural docs and code comments agree on this boundary.

## Notes for Agent

This is not glamorous work, but it is anti-haunting work. Do it before adding more product breadth.
