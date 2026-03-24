# Milestone v0.5.5: API, Functionality, and Polish

**Status:** COMPLETE
**Milestone:** v0.5.5
**Source of truth:** [TODO.md](/home/jove/code/vel/TODO.md)

## Overview

`v0.5.5` is the planned follow-on milestone after the current `0.5.4` UI implementation/revision line.

Its purpose is to take everything still open in `TODO.md` and finish it as one coherent execution line across three scopes:

- API truth
- functional behavior
- final UI polish

This milestone is explicitly not a new-surface milestone. It exists to finish the accepted shell and surfaces honestly, including the transport and persistence seams that were deferred out of `0.5.4`.

## Scope

### In scope

- all unchecked items in [TODO.md](/home/jove/code/vel/TODO.md)
- all API-blocked UI truth gaps currently tracked for `Now`, `Threads`, `System`, assistant entry, and attachments
- remaining accepted runtime polish for shell, navbar, nudges, composer, `Now`, `Threads`, and `System`
- truthful persistence/mutation seams where the current UI still uses local-only approximation

### Out of scope

- new top-level surfaces
- provider expansion beyond what is already in scope
- Apple implementation
- speculative workflow-builder widening
- backend redesign not required to make the accepted web UI truthful

## Planned Requirement Buckets

| ID | Description |
|----|-------------|
| API-55-01 | Add the backend/API seams required by the accepted web UI and listed in `TODO.md`. |
| FUNC-55-01 | Bind accepted operator behaviors to truthful runtime state instead of local-only approximation. |
| POLISH-55-01 | Close the remaining shell/surface/browser-level fidelity issues across navbar, nudges, composer, `Now`, `Threads`, and `System`. |
| VERIFY-55-01 | Close the line with browser acceptance proof, cleanup, and honest remaining-debt accounting. |

## Locked Spec Clarifications

- `System` stats/info may use compact multi-column layout, but config/forms/lists remain single-column and vertically stacked.
- The `System` sidebar remains sticky while scrolling. Third-level children stay collapsed by default and expand/show active state when their parent second-level anchor is in focus.
- On non-`Now` surfaces, the navbar places current event to the left of the center status group and active task to the right.
- The `Now` progress bar must support over-100% completion with a second overflow segment rather than numeric-only overflow.
- `Now` renders all Todoist tags until filtering logic is added in a later slice.
- The accepted `Threads` filter set for `v0.5.5` is `All`, `Unread`, `Needs Review`, `Active`, and `Archived`.
- The navbar `System` info affordance must resolve to repo docs, creating the doc surface if it does not exist yet.
- The plus attachment affordance opens a small menu with `File` and `Image` entries in this milestone, leaving room for future entity insertion modes.
- The navbar `System` info affordance may use a repo-local operator doc as its canonical target in this milestone.
- Attachment selection should immediately render a compact queued attachment chip in the composer while durable upload/API support is still pending.

## Planned Phase Shape

### Phase 93: API contract and persistence truth

**Goal:** close the transport and persistence seams that `0.5.4` could not finish truthfully in frontend code.  
**Depends on:** close or handoff boundary from `0.5.4`
**Status:** COMPLETE

Expected outcomes:

- assistant-entry intent selection is durable
- attachment/file seams exist
- thread-row metadata required by the UI is exposed
- `System` settings/config mutations persist truthfully
- `Now` lane semantics stop depending on client-local approximation

### Phase 94: Functionality binding and behavior completion

**Goal:** bind the accepted UX to the new API seams and complete behavior gaps still open in `TODO.md`.  
**Depends on:** Phase 93
**Status:** COMPLETE

Expected outcomes:

- composer/recording lifecycle is fully supported
- nudge expansion, actions, and error-routing behavior are complete
- `Now` task fields/tags/progress and lane behaviors are functionally complete
- `Threads` default selection/filter/archive/project metadata behaviors are complete
- `System` sidebar depth and config exposure bind to truthful data

### Phase 95: Final shell and surface polish

**Goal:** close the remaining browser-level fidelity issues after behavior truth is in place.  
**Depends on:** Phase 94
**Status:** COMPLETE

Expected outcomes:

- final badge/tag/pill centering and radius cleanup
- final navbar status/event/task rhythm
- final nudge gutter/ring/pill geometry
- final `Now`/`Threads`/`System` density and surface polish
- bubble-tail, header, and row-state fidelity closed to acceptance level

### Phase 96: Browser proof, acceptance audit, and milestone closeout

**Goal:** close the milestone honestly once the runtime is actually acceptable.  
**Depends on:** Phase 95
**Status:** COMPLETE

Expected outcomes:

- browser proof for the accepted surfaces and states
- final acceptance audit against `TODO.md`
- cleanup/removal of temporary compatibility paths
- truthful deferred-work list if anything still remains after execution

## Mirrored TODO Scope

The following backlog is copied into the milestone packet verbatim so `v0.5.5` owns the execution scope directly rather than only pointing back to [TODO.md](/home/jove/code/vel/TODO.md).

### General

- [ ] loading states should have loading spinner (shared component)
- [ ] tags should have lower border radius while action chiplets keep their roundedness
- [ ] plus button should be inside floating action bar on the left
- [ ] errors should be generalized to nudges, for example api errors from the floating nav bar should show as nudges
- [ ] when there is a recorded item in the action bar, there should be an x icon on the right hand side to remove it (asset is still saved)
- [ ] queued assistant feedback and unclear-intent fanout still need final acceptance-level polish and tighter integration with the shared pill language
- [ ] shared pills/tags/filter tags still need final browser-level centering verification across surfaces

Phase mapping:
- Phase 94: loading states, composer affordances, error-to-nudge routing
- Phase 95: shared chip/tag radius and final centering polish

### Nudges

- [ ] nudge pill outlines should match the outline color of their icon ring. this color should also set the background, albeit a more muted transformation
- [ ] nudge icon rings should not be touching their associated pills, nor pushing them to the right
- [ ] nudge action chips need to be standardized
- [ ] if nudges are truncated, they need to be able to be vertically expanded in place to view all of the information / actions -- only one nudge can be expanded at a time
- [ ] the nudge title NUDGES (N) should have the alert icon before it
- [ ] nudge left-gutter icon/ring centering and chip non-overlap still need final browser-level cleanup

Phase mapping:
- Phase 94: nudge behavior, expansion, action standardization
- Phase 95: final geometry, ring/pill color mapping, centering

### Navbar

- [ ] when not on now page, current event and active task (with their icons as showin the now section header) should be shown on either sides of the status icons in the navbar respectfully
- [ ] numbers in badges are too high, not vertically centered
- [ ] info bar / docs link currently does not match the navbar style, no does the link work
- [ ] sync badge still needs exact accepted final temperament for healthy / syncing / error states

Phase mapping:
- Phase 94: non-`Now` context slot behavior, docs/info link wiring
- Phase 95: badge rhythm, alignment, and final sync-state polish

### Now

- [ ] dattime line should have full Day and Month name and year
- [ ] active task icon should be the same star icon as now, it does not need to be in the allcaps ACTIVE TASK (N) header, just the badge tag
- [ ] that same active task icon should show next to to the ACTIVE_TASK line after the current event
- [ ] active task pill on now page should have the active task icon as the left decoration simiar to nudge style
- [ ] active task section should have more headroom spacing
- [ ] belowthe current event and active task line in the header, there should be a progress bar of task completion with support for the extra tasks (over 100%)
- [ ] now section header badge tags should also include completed tasks, overdue tasks
- [ ] the if time allows section should not have the backlog badge on the right side
- [ ] base task text size can be slightly reduced
- [ ] tasks on now are showing double project tags
- [ ] tasks on now should show all tags coming from todoist
- [ ] tasks on now should have separate title and description
- [ ] local drag/drop works, but durable multi-active lane truth and lane ordering still need backend support
- [ ] non-chronological next-up ordering is still only partially truthful until the `now` contract is widened

Phase mapping:
- Phase 93: widen `now` contract for durable lane truth and ordering
- Phase 94: wire truthful progress, tags, title/description, lane semantics
- Phase 95: final spacing, iconography, badge rhythm, and text sizing

### Threads

- [ ] then loading thread page, default active thread should be in active state
- [ ] the threads chat icon does not need to be in the central panel
- [ ] the user circles still do not center their letter occupants. the letters should be the same size as the label text they are on a line with, with rings to fit
- [ ] the archive button should be in a circle
- [ ] thread bubble tails still need final visual tuning to match the accepted seamless same-color shape
- [ ] thread row project tags require thread-list metadata that is not in the current contract
- [ ] thread filters are broader now, but still not at the full accepted filter model

Phase mapping:
- Phase 93: thread-list project metadata contract
- Phase 94: truthful default-selection and filter behavior
- Phase 95: avatar/archive/tail fidelity and remaining view polish

### System

- [ ] background of system page should math the background of now and threads
- [ ] there should be a third level in the system sidebar that shows the children of the second level
- [ ] the top level system header in the central container is not needed
- [ ] action buttons should be justified to the right
- [ ] in the system view, there should be multiple columns of stats / info so that the page isn't as long
- [ ] status tags for integrations like connected and configured should have success status colors
- [ ] system is flatter now, but still needs another compactness/readability pass to fully hit the dense single-document target
- [ ] many visible system fields are still UI-local edits only; truthful persisted mutations need API follow-on work

Phase mapping:
- Phase 93: persisted settings/config mutation seams
- Phase 94: expose truthful fields and navigation depth
- Phase 95: final density, layout, action alignment, and status-color polish

### API

- [ ] assistant-entry intent selection needs a durable persisted contract
- [ ] file / attachment handling needs a real API contract
- [ ] thread list rows need project metadata
- [ ] system settings/config need truthful persisted mutation endpoints

Phase mapping:
- Phase 93: entire section

## Acceptance Standard

`v0.5.5` is only complete when:

- every unchecked item in [TODO.md](/home/jove/code/vel/TODO.md) is either completed or explicitly moved to accepted deferred work
- API-backed seams are truthful, not simulated locally
- browser review is treated as acceptance evidence, not frontend tests alone
