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

## Current Files

- [event-bus.md](event-bus.md)
- [state-graph.md](state-graph.md)
- [spec-draft.md](spec-draft.md)

## Drafting Rule

If a coding agent or contributor needs a new architecture spec and does not yet have a better filename, start from [spec-draft.md](spec-draft.md), then rename or copy it to a focused file name before treating it as durable.
