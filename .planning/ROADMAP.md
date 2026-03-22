# Roadmap: Vel

## Archived Milestones

- `v0.1` archived phase packet: [v0.1-phases](/home/jove/code/vel/.planning/milestones/v0.1-phases)
- `v0.2` shipped true-MVP archive: [v0.2-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.2-ROADMAP.md)
- `v0.3` shipped canonical `Now` + client mesh archive: [v0.3-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.3-ROADMAP.md)

## Active Milestone

Milestone `0.4.x` starts at Phase 52 and closes `Now/UI` MVP conformance gaps that remain after shipped `0.3.0`.

The goal of the `0.4.x` line is to:

- make `Now` feel like a compact operating surface rather than a dashboard
- make the shipped web shell conform to the operator-corrected `Now` contract exactly
- remove shell/helper noise from the primary navigation and core surfaces
- restore data truth between `Now` and `Inbox`
- rebuild `Threads` and `Settings` into compact MVP-usable layouts
- preserve iOS/client parity against the corrected web reference

## Next Milestone Packet

The next backend milestone is already shaped as a frozen future packet:

- [v0.5-core-rewrite/ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/ROADMAP.md)

Use it as future-planning authority for the post-`0.4.x` core rewrite only. It does not supersede the active `0.4.x` line until the current milestone is closed honestly.

## Scope Guardrails

`0.4.x` is only about `Now/UI MVP Conformance Closure`:

- `Now` structure, grouping, input, and nudge presentation
- shell navigation, documentation access, and right-sidebar behavior
- inbox shared-object correctness
- threads layout and row priority
- settings information architecture simplification
- parity closure against the corrected web reference

Do not widen this milestone into:

- new planner/product surfaces
- broad provider or integration expansion
- speculative visual redesign beyond compact MVP conformance
- reopening product semantics already locked unless data truth requires it

## Versioning Policy

Vel now uses semver language for product lines and shipped releases:

- `0.3.0` is the latest shipped release baseline in the repo
- `0.4.x` is the active release line for current roadmap work
- `0.5.0-beta` is reserved for the first beta line after the `0.4.x` closure criteria are met

For roadmap execution inside a release line, use a four-part lineage identifier:

- format: `<major>.<minor>.<phase>.<plan>`
- example: `0.4.54.1` means release line `0.4.x`, Phase `54`, Plan `01`
- phase numbers remain in place for historical continuity with the existing planning system
- shipped artifacts and tags should continue to use normal semver, not the four-part planning identifier

## Phases

- [x] **Phase 52: Full Now/UI conformance implementation chunk** - Implemented the correction memo across `Now`, shell/nav, `Inbox`, `Threads`, `Settings`, and shared parity-sensitive seams with no requested implementation item deferred
- [x] **Phase 53: Operator UI feedback capture and conformance review** - Captured the operator review as a bounded polish authority covering nav, sidebar, Now, Threads, composer, and Settings without reopening milestone scope
- [x] **Phase 54: Final UI cleanup and polish pass** - Applied the accepted navbar, info-panel, contextual-docs, and Now-header polish so the shell lands with a tighter operator-trusted finish
- [x] **Phase 55: Outmoded UI path cleanup and seam hardening** - Removed dead `Suggestions`/`Stats` shell lanes, placeholder routing, and legacy settings-tab compatibility before closeout
- [x] **Phase 56: Conformance verification and milestone closeout** - Closed the milestone with a restored strict-clean web build, focused regression proof, and no remaining hidden closeout debt

Lineage mapping:

- `0.4.52.x` - full conformance implementation
- `0.4.53.x` - operator feedback capture
- `0.4.54.x` - final UI cleanup and polish
- `0.4.55.x` - outmoded UI path cleanup and seam hardening
- `0.4.56.x` - conformance verification and closeout

## Progress

**Execution Order:** 52 -> 53 -> 54 -> 55 -> 56

| Phase | Requirements | Status |
|-------|--------------|--------|
| 52. Full Now/UI conformance implementation chunk | NOWUI-01, NOWUI-02, NOWUI-03, NOWUI-04, NOWUI-05, NOWUI-06, NOWUI-07, SHELL-01, SHELL-02, SHELL-03, SHELL-04, INBOX-01, INBOX-02, THREADS-01, THREADS-02, THREADS-03, SETTINGS-01, SETTINGS-02, SETTINGS-03, PARITY-01, PARITY-02 | Complete |
| 53. Operator UI feedback capture and conformance review | FEEDBACK-01, FEEDBACK-02, FEEDBACK-03, FEEDBACK-04 | Complete |
| 54. Final UI cleanup and polish pass | POLISH-01, POLISH-02, POLISH-03, POLISH-04 | Complete |
| 55. Outmoded UI path cleanup and seam hardening | CLEANUP-01, CLEANUP-02, CLEANUP-03, CLEANUP-04 | Complete |
| 56. Conformance verification and milestone closeout | VERIFY-01, VERIFY-02, VERIFY-03, VERIFY-04, milestone verification and reconciliation | Complete |

## Phase Details

### Phase 52: Full Now/UI conformance implementation chunk

**Goal:** implement the full operator correction memo in a single execution chunk so none of the requested surface, IA, or data-truth changes are deferred behind later implementation phases.
**Requirements:** NOWUI-01, NOWUI-02, NOWUI-03, NOWUI-04, NOWUI-05, NOWUI-06, NOWUI-07, SHELL-01, SHELL-02, SHELL-03, SHELL-04, INBOX-01, INBOX-02, THREADS-01, THREADS-02, THREADS-03, SETTINGS-01, SETTINGS-02, SETTINGS-03, PARITY-01, PARITY-02
**Depends on:** shipped milestone v0.3
**Success Criteria:**
1. `Now` top area is containerless micro-rows, with minimal timing, single-line description, styled nudge boxes, hidden empty-state controls, and a floating bottom-center text/voice input.
2. Tasks are the only dominant visual container and are grouped strictly as `NOW`, `TODAY`, `AT RISK`, and `NEXT`, using real current-day truth and excluding irrelevant project-review noise.
3. The shell uses a compact top nav with `Documentation` promoted to top-level access, and the right sidebar handles moved context/control content without helper prose.
4. `Inbox` is corrected at the data/query level so it contains the same actionable objects surfaced in `Now`.
5. `Threads` and `Settings` are both restructured into compact MVP-usable layouts that match the operator memo instead of the current verbose/debug-leaning shells.
6. No requested item from the operator correction memo is left for a later implementation phase; parity-sensitive client behavior is updated against the corrected web reference in this same chunk.
**Plans:** 1 plan

### Phase 53: Operator UI feedback capture and conformance review

**Goal:** gather operator feedback against the implemented conformance slice, separate true change requests from noise, and define the smallest final UI cleanup set needed before closeout.
**Requirements:** FEEDBACK-01, FEEDBACK-02, FEEDBACK-03, FEEDBACK-04
**Depends on:** Phase 52
**Success Criteria:**
1. The updated `Now`, shell, `Inbox`, `Threads`, and `Settings` surfaces are walked with the operator using a single structured review pass.
2. Feedback is captured as concrete UI deltas tied to the corrected surfaces, with clear separation between must-fix polish, optional follow-up, and out-of-scope ideas.
3. Any accepted changes preserve the `0.4.x` scope guardrails and do not reopen locked product semantics unless the operator explicitly identifies a conformance miss.
4. The resulting cleanup set is small enough to execute in one final polish phase and is recorded as the authority for the remaining UI work.
**Plans:** 1 plan-equivalent review packet

### Phase 54: Final UI cleanup and polish pass

**Goal:** execute the operator-approved cleanup set so the corrected surfaces land with final polish, tighter ergonomics, and no unresolved high-signal UI rough edges.
**Requirements:** POLISH-01, POLISH-02, POLISH-03, POLISH-04
**Depends on:** Phase 53
**Success Criteria:**
1. All accepted review findings from Phase 53 are implemented across the relevant surfaces without widening milestone scope.
2. Shared visual affordances, spacing, empty states, and compact-shell behaviors feel intentional and consistent across `Now`, `Inbox`, `Threads`, `Settings`, and navigation.
3. Any polish changes that affect parity-sensitive behavior are reflected in the web reference and captured for client follow-through before verification.
4. Remaining UI imperfections, if any, are explicitly documented as non-blocking and deferred rather than silently carried into closeout.
**Plans:** 1 plan

### Phase 55: Outmoded UI path cleanup and seam hardening

**Goal:** remove stale or superseded UI code and contract-adapter drift left behind by the conformance work so the repo keeps one clear multiplatform Rust-core lane instead of parallel legacy behavior paths.
**Requirements:** CLEANUP-01, CLEANUP-02, CLEANUP-03, CLEANUP-04
**Depends on:** Phase 54
**Success Criteria:**
1. Superseded view components, helper copy paths, obsolete shell layouts, and dead styling/code branches displaced by Phases 52-54 are removed rather than left dormant.
2. Cleanup is limited to code made outmoded by the `0.4.x` UI correction work; it does not widen into unrelated modernization or speculative refactors.
3. Any transport adapters, selectors, or client-side shaping that duplicate Rust-owned semantics are either removed or reduced to thin boundary mapping.
4. The remaining seam between Rust core, transport DTOs, and client rendering is simpler to reason about after cleanup than before it.
**Plans:** 1 plan

### Phase 56: Conformance verification and milestone closeout

**Goal:** prove the corrected, cleaned, and polished surfaces match the operator memo plus accepted final review deltas, then close the milestone with real evidence.
**Requirements:** VERIFY-01, VERIFY-02, VERIFY-03, VERIFY-04, milestone verification and reconciliation
**Depends on:** Phase 55
**Success Criteria:**
1. The manual conformance checklist is executed first and recorded against the corrected memo.
2. Contract/DTO tests prove grouping, hidden-empty-state behavior, and inbox/now shared-object invariants.
3. A boundary verification section explicitly confirms that the multiplatform Rust core owns product semantics, clients consume stable transport contracts, and no client keeps a shadow behavior model that should live in Rust.
4. UI tests cover the corrected reference embodiment after manual verification passes, including accepted Phase 53 cleanup items implemented in Phase 54 and seam cleanup from Phase 55.
5. Remaining parity limits, if any, are explicit and do not hide unimplemented memo items, deferred polish decisions, or unresolved Rust-core/client boundary drift.
**Plans:** 1 plan

---
*Last updated: 2026-03-21 after adopting `0.4.x` release-line planning and Phase 53 operator review intake*
