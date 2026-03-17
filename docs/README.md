# Vel Docs Guide

This file is the top-level guide for navigating Vel documentation.

Use it to answer three questions quickly:

1. what is real now
2. what should be worked on next
3. what is historical context rather than current authority

## Doc Classes

### Current truth

These files describe the current implementation and operational reality.

- [status.md](status.md): canonical implementation ledger
- [api.md](api.md): HTTP/API surface overview
- [vel-documentation-index-and-implementation-status.md](vel-documentation-index-and-implementation-status.md): coverage map for documented subsystems
- [specs/vel-user-documentation-spec.md](specs/vel-user-documentation-spec.md): target shape for full-fat end-user documentation
- [user/README.md](user/README.md): canonical user-facing docs entrypoint

### Active plan

These files describe the current implementation sequence and near-term convergence work.

- [tickets/README.md](tickets/README.md): ticket-pack inventory, maturity index, and triage entry point
- [tickets/repo-feedback/README.md](tickets/repo-feedback/README.md): one active convergence packet for architecture and cleanup work
- [roadmap.md](roadmap.md): broader product direction, subordinate to `status.md` for shipped behavior

### Historical review

These files are useful design or review history, but they are not authoritative for shipped behavior.

- [reviews/README.md](reviews/README.md): historical review index for repo reviews and feedback rounds
- [specs/](specs/): design specs and planned architecture; validate against `status.md` before treating as implemented

## Minimum Reading Order

For a coding agent or new contributor, start here:

1. [status.md](status.md)
2. [tickets/README.md](tickets/README.md)
3. [README.md](../README.md)
4. [product-spec.md](product-spec.md)
5. [architecture.md](architecture.md)
6. [data-model.md](data-model.md)

## Authority Rules

- If a ticket or spec conflicts with [status.md](status.md), trust `status.md` for current behavior.
- Use [tickets/README.md](tickets/README.md) to choose the right active plan or ticket pack for the task at hand.
- Treat [tickets/repo-feedback/README.md](tickets/repo-feedback/README.md) as one convergence-oriented packet, not as the only possible active plan.
- Treat files under [reviews/](reviews/) as historical input, not current requirements.
