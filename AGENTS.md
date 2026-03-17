# AGENTS.md

This document defines durable repository rules for AI coding agents working in Vel.

## Authority

- Repo-wide implementation truth lives in `docs/status.md`.
- `AGENTS.md` defines durable boundaries and workflow rules, not a mutable feature ledger.
- If `AGENTS.md` and a status claim appear to conflict, treat `docs/status.md` as canonical for shipped behavior and `AGENTS.md` as canonical for repository boundaries.

## Mission

Vel is a local-first cognition runtime for capture, recall, and daily orientation.

Product principle:
- optimize for repeated personal use before broad generality
- prefer daily loops over speculative automation
- prefer capture/review ergonomics over agent complexity
- prefer trust/export over speculative integrations

## Durable Repository Rules

- `vel-core` owns domain semantics and domain types.
- `vel-storage` must not depend on `vel-api-types`.
- `vel-api-types` contains transport DTOs only; map from core types at the boundary.
- Route handlers should remain thin: parse request, call service, map response/error.
- Services should hold application logic.
- Run-backed operations must emit run events and persist terminal state.
- Prefer structured payloads such as `serde_json::Value` over raw JSON strings in domain and API layers.
- Docs must distinguish implemented behavior from planned behavior; use `docs/status.md` for current truth.

## Development Principles

### Local-First

- Prefer local files, local databases, and user-controlled infrastructure.
- Remote services should be optional.

### Modular Architecture

- Keep subsystem boundaries clear.
- Prefer explicit interfaces between capture, memory, context, execution, and interface layers.

### Data Ownership

- User data should remain under user control.
- Default storage choices should be inspectable and local-first.

### Explainability

- Vel decisions should be traceable.
- Suggestions and nudges should make it possible to determine what context and rules were used.

## Coding Expectations

- Prefer readable code over clever code.
- Avoid unnecessary dependencies.
- Keep builds reproducible.
- Write tests where appropriate.
- Document new modules or contracts when adding them.

## Agent Workflow

Before substantial implementation work, read:

1. `docs/README.md`
2. `docs/status.md`
3. `docs/product-spec.md`
4. `docs/architecture.md`
5. `docs/data-model.md`
6. `docs/mvp.md`

Then:

1. implement the minimum viable slice first
2. keep architecture boundaries intact
3. write or update tests where appropriate
4. update documentation for any changed module, API contract, or workflow

## Priority Order

1. capture system
2. memory graph
3. context recall
4. daily alignment engine
5. execution automation

## Early Non-Goals

Avoid early overreach into:

- complex distributed systems before needed
- unnecessary cloud dependencies
- premature optimization
- excessive UI complexity
- speculative productization features

The priority is core cognition features.
