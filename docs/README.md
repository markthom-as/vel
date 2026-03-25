# Vel Docs Guide

This file is the top-level guide for navigating Vel documentation.

Use it to answer three questions quickly:

1. what is real now
2. what should be worked on next
3. what is historical context rather than current authority

## Authority

These are the current authoritative entrypoints:

- [MASTER_PLAN.md](MASTER_PLAN.md): canonical implementation truth, phase status, and active ticket list.
- [cognitive-agent-architecture/README.md](cognitive-agent-architecture/README.md): internal architecture pack entrypoint.
- [cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md](cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md): overarching internal concept and system model.
- [cognitive-agent-architecture/01-cross-cutting-system-traits.md](cognitive-agent-architecture/01-cross-cutting-system-traits.md): repo-wide traits for modularity, accessibility, configurability, logging, replay, and composability.
- [cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md](cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md): canonical object, schema, template, and manifest ownership.
- [user/README.md](user/README.md): canonical user-facing operating docs.
- [api/README.md](api/README.md): current API docs for `/v1`, `/api`, and `/ws`.
- [templates/README.md](templates/README.md): canonical templates for docs, specs, tickets, prompts, and runbooks.
- [../config/README.md](../config/README.md): canonical config, template, schema, and example asset map.
- [documentation-catalog.json](documentation-catalog.json): canonical surfaced documentation catalog for CLI, web settings, and Apple clients.

## Planning Surfaces

These files drive ongoing implementation work:

- `.planning/ROADMAP.md`: top-level roadmap entrypoint and active milestone pointer
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`: active `0.5` backend rewrite packet
- `.planning/PROJECT.md`: active product definition and accepted planning decisions
- `.planning/BACKLOG.md`: non-phase future work that is worth preserving but not yet committed
- `.planning/todos/pending/*.md`: execution-ready micro-task queue used by GSD todo workflows
- `.planning/phases/05-*` through `.planning/phases/09-*`: active phase-planning directories
- `docs/future/*.md`: future-facing product and architecture specs that are explicitly not shipped-behavior authority
- [tickets/README.md](tickets/README.md): queue index and phase navigation
- [tickets/architecture-first-parallel-queue.md](tickets/architecture-first-parallel-queue.md): documentation/contracts-first execution order and parallel work waves
- `docs/tickets/phase-1/*.md`
- `docs/tickets/phase-2/*.md`
- `docs/tickets/phase-3/*.md`
- `docs/tickets/phase-4/*.md`
- `docs/tickets/phase-5/*.md`

Phase 1 and Phase 3 are complete historical implementation lanes. Phase 2 and Phase 4 also remain historical, but some unfinished original-scope work from those lanes was explicitly re-scoped into later milestone planning. Active implementation planning now runs through the `0.5` packet.

Use the ticket closest to the boundary you are changing when touching historical architecture surfaces, and use the active phase planning files for future product work.

## Historical And Secondary Material

These docs can still be useful, but they are subordinate to the authority chain above:

- individual architecture leaf docs under `docs/cognitive-agent-architecture/`
- config templates, schemas, manifests, and examples under `config/` and `configs/models/`
- user-facing troubleshooting and setup guides under `docs/user/`
- API detail docs under `docs/api/`

## Minimum Reading Order

For a coding agent or new contributor, start here:

1. [MASTER_PLAN.md](MASTER_PLAN.md)
2. [README.md](../README.md)
3. [cognitive-agent-architecture/README.md](cognitive-agent-architecture/README.md)
4. [cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md](cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md)
5. [cognitive-agent-architecture/01-cross-cutting-system-traits.md](cognitive-agent-architecture/01-cross-cutting-system-traits.md)
6. [cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md](cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md) when the task touches shared contracts
7. [../config/README.md](../config/README.md) when the task touches config, templates, schemas, or examples
8. the relevant phase ticket under `docs/tickets/`
9. the closest subtree `README.md` or `AGENTS.md` for the surface you are touching

## Authority Rules

- If a ticket, user doc, or API doc conflicts with [MASTER_PLAN.md](MASTER_PLAN.md), trust `MASTER_PLAN.md` for current shipped behavior and phase status.
- Treat the concept spec and cross-cutting traits spec as the target internal architecture unless the Master Plan says that work has not shipped yet.
- Treat tickets as the source of truth for the next intended change, not proof that the change is already implemented.
- Use user docs for operating the product and API docs for current interface shape; do not infer current implementation from stale historical material.
- If a doc points to a missing file or dead authority chain, repair it instead of adding another parallel explanation.
