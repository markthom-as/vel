# Milestone v0.5.8: GSD Migration and Phase Reset

**Status:** IN PROGRESS
**Milestone:** v0.5.8  
**Theme:** stabilize GSD workflow state before larger follow-on work

## Overview

`v0.5.8` becomes the active follow-on after `v0.5.7` was deferred as future duplex work.

Its purpose is to make planning workflow state reliable again before reopening larger feature delivery:

- evaluate whether `GSD 2` can safely replace the current local `get-shit-done` install
- preserve current Codex/GSD workflow continuity during any migration or fallback bridge
- make active milestone scope milestone-local again by resetting live phase numbering to `01`
- leave deferred duplex voice work parked in `docs/future` instead of mixing it into active planning

## Active Packet

- [ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/ROADMAP.md)
- [REQUIREMENTS.md](/home/jove/code/vel/.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/REQUIREMENTS.md)
- [13-NEXT-STEPS.md](/home/jove/code/vel/.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/13-NEXT-STEPS.md)

## In Scope

- inventory the repo’s current dependency on the local `get-shit-done` install, commands, paths, and assumptions
- decide whether `GSD 2` is ready for safe adoption in this repo
- implement the chosen migration, compatibility bridge, or documented defer decision
- verify the common repo workflows still behave predictably after the change

## Out of Scope

- reopening duplex voice implementation in this milestone
- renumbering archived historical phase packets
- pretending a migration happened if compatibility is not demonstrated
- widening this line into unrelated product or UI work

## Requirement Buckets

| ID | Description |
|----|-------------|
| AUDIT-58-01 | The current repo dependency on `get-shit-done` v1 is inventoried with concrete compatibility risks and cutover constraints. |
| MIGRATE-58-01 | The chosen GSD path is implemented honestly: migrate to `GSD 2`, add a compatibility bridge, or defer with explicit rationale. |
| STATE-58-01 | Active planning state stays milestone-local, with `01`-based phases and no archived packet confusion under `.planning/phases/`. |
| VERIFY-58-01 | Common planning workflows are exercised directly so the repo is not left in a speculative migration state. |

## Planned Phases

### Phase 01: GSD 2 readiness and compatibility audit

**Goal:** prove what the repo currently depends on before any toolchain cutover.  
**Depends on:** deferred `0.5.7` closeout  
**Status:** NOT STARTED

Expected outcomes:

- current `get-shit-done` v1 entrypoints, local install assumptions, and Codex workflow dependencies are enumerated
- concrete incompatibilities between the current setup and `GSD 2` are identified
- migration preconditions, fallback needs, and rollback shape are documented

### Phase 02: GSD 2 migration cutover and Codex integration

**Goal:** implement the chosen migration or compatibility path without breaking repo-local workflows.  
**Depends on:** Phase 01  
**Status:** QUEUED

Expected outcomes:

- the selected `GSD 2` cutover, compatibility bridge, or explicit defer mechanism is implemented
- repo-local docs and workflow entrypoints match the actual installed behavior
- milestone-local phase numbering and active-state discovery remain stable after the change

### Phase 03: GSD 2 verification and closeout

**Goal:** verify that the planning workflow behaves predictably after the chosen change.  
**Depends on:** Phase 02  
**Status:** QUEUED

Expected outcomes:

- roadmap and health tooling are exercised against the updated planning state
- common repo flows such as progress, next-step routing, cleanup, and milestone creation are checked directly
- residual migration debt is recorded explicitly instead of being hidden under a “finished” claim

## Execution Order

Planned sequence:

`01 -> 02 -> 03`

## Acceptance Standard

`v0.5.8` closes only when:

- the repo’s actual GSD toolchain state is documented and reproducible
- the chosen migration or defer path is reflected in repo docs and local workflow expectations
- active milestone discovery stays clean under `.planning/phases/`
- verification shows the common planning flows still work without getting lost in archived history
