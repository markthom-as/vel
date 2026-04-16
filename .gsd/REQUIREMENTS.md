# Requirements

This file is the explicit capability and coverage contract for the project.

## Active

None.

## Validated

### R004 — Verify common planning workflows directly after the change
- Class: failure-visibility
- Status: validated
- Description: Common workflows such as progress, cleanup, roadmap analysis, and milestone creation must be exercised directly after the chosen change.
- Why it matters: A planning-tool change is not done until the actual operator workflow still behaves predictably.
- Source: execution
- Primary owning slice: M001/S03
- Supporting slices: M001/S02
- Validation: validated
- Notes: Proven by S03 direct workflow checks; residual v1 milestone-label debt remains recorded for future tool work.

### R001 — Inventory current v1 dependency and migration constraints
- Class: operability
- Status: validated
- Description: The repo’s current dependency on the local `get-shit-done` v1 install, commands, paths, and assumptions has been documented concretely before cutover.
- Why it matters: Migration decisions are not trustworthy if the actual dependency surface is still implicit.
- Source: execution
- Primary owning slice: M001/S01
- Supporting slices: none
- Validation: validated
- Notes: Proven by the completed readiness audit and command-backed evidence recorded in S01.

### R002 — Implement an honest GSD path for this repo
- Class: operability
- Status: validated
- Description: The chosen path must be implemented honestly as a `GSD 2` migration, a compatibility bridge, or an explicit defer decision.
- Why it matters: The repo should not imply a migration happened if workflow-critical surfaces still depend on v1-only behavior.
- Source: execution
- Primary owning slice: M001/S02
- Supporting slices: M001/S03
- Validation: validated
- Notes: Phase 02 selected a compatibility bridge and preserved v1 command authority while `gsd-pi` remains only partially verified: runtime `PATH` must prefer Node `>=22`, and `graph status` is blocked by missing `@gsd-build/mcp-server`.

### R003 — Keep active planning state milestone-local and truthful
- Class: continuity
- Status: validated
- Description: Active planning state must stay milestone-local with `01`-based phases, and no archived packet should be treated as live work.
- Why it matters: Bad milestone discovery or wrong archive targeting makes planning state untrustworthy.
- Source: execution
- Primary owning slice: M001/S02
- Supporting slices: M001/S03
- Validation: validated
- Notes: Phase 02 verified active `.planning/phases/` contains only the `0.5.8` phase packet and documented dual-state bridge rules.

## Deferred

None.

## Out of Scope

### R005 — Reopen duplex voice work inside this milestone
- Class: anti-feature
- Status: out-of-scope
- Description: `v0.5.8` must not reopen deferred duplex voice implementation work.
- Why it matters: This prevents the migration/tooling milestone from widening into unrelated product delivery.
- Source: user
- Primary owning slice: none
- Supporting slices: none
- Validation: n/a
- Notes: Deferred voice work stays parked in future specs until a later milestone reopens it.

### R006 — Renumber archived milestone history to satisfy tooling heuristics
- Class: constraint
- Status: out-of-scope
- Description: Archived historical milestone packets should not be renumbered just to satisfy planning-tool assumptions.
- Why it matters: Historical records should stay stable; active-state routing should be fixed without rewriting history.
- Source: execution
- Primary owning slice: none
- Supporting slices: none
- Validation: n/a
- Notes: The active milestone should use milestone-local numbering without rewriting archive history.

## Traceability

| ID | Class | Status | Primary owner | Supporting | Proof |
|---|---|---|---|---|---|
| R001 | operability | validated | M001/S01 | none | validated |
| R002 | operability | validated | M001/S02 | M001/S03 | validated |
| R003 | continuity | validated | M001/S02 | M001/S03 | validated |
| R004 | failure-visibility | validated | M001/S03 | M001/S02 | validated |
| R005 | anti-feature | out-of-scope | none | none | n/a |
| R006 | constraint | out-of-scope | none | none | n/a |

## Coverage Summary

- Active requirements: 0
- Mapped to slices: 3
- Validated: 4
- Unmapped active requirements: 0
