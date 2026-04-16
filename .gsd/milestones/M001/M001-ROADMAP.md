# M001: GSD Migration and Phase Reset

**Vision:** Make the repo’s GSD workflow trustworthy again before larger feature work resumes.

## Success Criteria

- The repo’s current dependency on the local `get-shit-done` v1 install is inventoried with concrete migration constraints.
- The chosen GSD path is implemented honestly as a migration, compatibility bridge, or explicit defer.
- Active planning state stays milestone-local with `01`-based phases and no archived packet confusion.
- Common planning workflows are exercised directly so the repo is not left in a speculative migration state.

## Key Risks / Unknowns

- The repo and Codex skill surface are still tightly coupled to the v1 install layout — a blind cutover could break practical workflow use.
- Upstream `GSD 2` docs and the installed CLI surface may not match from this worktree’s point of view — migration rehearsal could stall on command-surface mismatch.

## Proof Strategy

- v1 coupling and migration blockers → retire in S01 by proving the concrete dependency surface and cutover constraints.
- Workflow truth after bridge or cutover → retire in S02 and S03 by proving repo commands, milestone discovery, and closeout checks behave predictably.

## Verification Classes

- Contract verification: direct command checks for progress, roadmap analysis, health, new-milestone handling, and closeout notes.
- Integration verification: local v1 toolchain behavior plus repo-local planning files and docs.
- Operational verification: milestone discovery, archive targeting, and active-state routing remain stable after the chosen change.
- UAT / human verification: confirm the repo’s planning workflow reads truthfully and does not imply adoption that has not actually happened.

## Milestone Definition of Done

This milestone is complete only when all are true:

- the active GSD path is described truthfully in repo artifacts and docs
- milestone-local active planning remains separate from archived history
- the chosen migration, bridge, or defer path is actually exercised
- success criteria are re-checked against live workflow behavior, not just copied text
- residual migration debt is explicit instead of hidden by milestone language

## Requirement Coverage

- Covers: R001, R002, R003, R004
- Partially covers: none
- Leaves for later: none
- Orphan risks: mismatch between upstream `GSD 2` docs, installed `gsd-pi` runtime/dependency behavior, and shipped CLI surface until separately resolved

## Slices

- [x] **S01: GSD 2 Readiness and Compatibility Audit** `risk:high` `depends:[]`
  > After this: the repo’s real v1 dependency surface and migration blockers are documented concretely enough to drive the next step.
- [x] **S02: GSD 2 Migration Cutover and Codex Integration** `risk:high` `depends:[S01]`
  > After this: the chosen migration or compatibility path is wired in and repo-local workflow expectations match the actual toolchain state.
- [x] **S03: GSD 2 Verification and Closeout** `risk:medium` `depends:[S02]`
  > After this: common planning workflows have direct verification evidence and milestone closeout language can stay honest.

## Boundary Map

### S01 → S02

Produces:
- concrete audit of local v1 install paths, skill coupling, repo coupling, and migration blockers
- explicit recommendation for whether to migrate, bridge, or defer

Consumes:
- nothing (first slice)

### S02 → S03

Produces:
- chosen migration or compatibility bridge implementation
- corrected repo-local docs and workflow pointers for the active toolchain state
- stable milestone discovery and archive-targeting behavior

Consumes:
- S01 audit findings and cutover constraints
