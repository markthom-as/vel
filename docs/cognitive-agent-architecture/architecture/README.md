# Vel Architecture Sub-Pack

This directory holds durable internal architecture docs for cross-subsystem runtime structure.

Use it for:

- eventing and state-graph behavior
- authority-runtime boundaries
- shared contracts that span multiple services or clients
- architecture specs that are broader than a single implementation ticket

Do not use it for:

- shipped-behavior truth
- operator-facing user docs
- narrow implementation tickets

Those belong in:

- [MASTER_PLAN.md](../../MASTER_PLAN.md) for shipped truth and queue shape
- [docs/tickets/README.md](../../tickets/README.md) for execution tickets
- [docs/user/README.md](../../user/README.md) and [docs/api/README.md](../../api/README.md) for operator-facing docs
- [docs/future/](../../future/) for future-only specs that are explicitly not shipped-behavior authority
- [docs/notes/README.md](../../notes/README.md) for working notes and exploratory material

## Status And Placement Rules

Interpret docs in this directory using the broader repository placement rules:

- architecture docs here may be accepted (`status: complete`) or active design-contract material (`status: draft`)
- `status: draft` here means the document may guide implementation, but does not by itself prove the behavior has shipped
- if a concept is future-only and not yet part of an accepted contract or active milestone packet, prefer `docs/future/`
- if a document is an interview log, parked idea, or exploratory sketch, prefer `docs/notes/`

## Current Files

- [canonical-schemas-and-contracts.md](canonical-schemas-and-contracts.md)
- [cross-surface-core-and-adapters.md](cross-surface-core-and-adapters.md)
- [data-layer-choices.md](data-layer-choices.md)
- [distributed-node-choices.md](distributed-node-choices.md)
- [rust-core-portability.md](rust-core-portability.md)
- [backup-and-operator-trust-contracts.md](backup-and-operator-trust-contracts.md)
- [storage-layer.md](storage-layer.md)
- [cross-cutting-trait-audit.md](cross-cutting-trait-audit.md)
- [event-bus.md](event-bus.md)
- [state-graph.md](state-graph.md)
- [spec-draft.md](spec-draft.md)

## Drafting Rule

If a coding agent or contributor needs a new architecture spec and does not yet have a better filename, start from [spec-draft.md](spec-draft.md), then rename or copy it to a focused file name before treating it as durable.
