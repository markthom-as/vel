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

### Active plan

These files describe the current implementation sequence and near-term convergence work.

- [tickets/repo-feedback/README.md](tickets/repo-feedback/README.md): highest-priority convergence packet
- [tickets/README.md](tickets/README.md): ticket-pack maturity index
- [roadmap.md](roadmap.md): broader product direction, subordinate to `status.md` for shipped behavior

### Historical review

These files are useful design or review history, but they are not authoritative for shipped behavior.

- [reviews/](reviews/): repo reviews, feedback rounds, and historical architecture notes
- [specs/](specs/): design specs and planned architecture; validate against `status.md` before treating as implemented

## Minimum Reading Order

For a coding agent or new contributor, start here:

1. [status.md](status.md)
2. [tickets/repo-feedback/README.md](tickets/repo-feedback/README.md)
3. [README.md](../README.md)
4. [product-spec.md](product-spec.md)
5. [architecture.md](architecture.md)
6. [data-model.md](data-model.md)

## Authority Rules

- If a ticket or spec conflicts with [status.md](status.md), trust `status.md` for current behavior.
- Treat [tickets/repo-feedback/README.md](tickets/repo-feedback/README.md) as the active convergence queue unless an explicit decision supersedes it.
- Treat files under [reviews/](reviews/) as historical input, not current requirements.
