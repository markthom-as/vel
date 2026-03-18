---
created: 2026-03-18T07:25:40.260Z
title: Ticket 005 - add NodeIdentity prerequisite and WAL mode step
area: docs
files:
  - docs/tickets/phase-2/005-hlc-sync-implementation.md
  - crates/vel-core/src/
---

## Problem

Two gaps in ticket 005 (HLC Sync):

1. **Node identity**: HLC requires a stable, unique node ID for clock disambiguation (HLC = physical timestamp + logical counter + node ID). No `NodeId` or `NodeIdentity` type exists in `vel-core`. This is a prerequisite for implementing the ordering primitive but isn't called out in the ticket.

2. **SQLite WAL mode**: WAL mode is documented in the codebase concerns doc as a prerequisite for distributed multi-writer scenarios, but it's not assigned to any ticket. Without WAL mode, concurrent readers + background writers in Phase 2 multi-node scenarios will contend badly. Ticket 005 is the logical home for this prerequisite step.

## Solution

Add to ticket 005:
- Step 0: "Introduce `NodeIdentity` value type in `vel-core` with UUID-based generation and stable persistence (survives restarts)"
- Step 0b: "Enable SQLite WAL mode in `vel-storage` database initialization (`infra.rs`) and add migration safety note"
