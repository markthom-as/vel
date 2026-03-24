# Milestone v0.5.5 Requirements

**Status:** ACTIVE  
**Milestone:** v0.5.5
**Theme:** API, functionality, and polish

## Primary Input

- [TODO.md](/home/jove/code/vel/TODO.md)

## Requirement Buckets

- [ ] **API-55-01**: assistant-entry follow-up behavior, thread metadata, `Now` lane truth, and persisted `System` settings/config are supported by typed backend/API seams
- [ ] **FUNC-55-01**: accepted shell/surface behaviors bind to truthful runtime state rather than client-local approximation
- [ ] **POLISH-55-01**: remaining shell/surface/browser fidelity issues from `TODO.md` are closed
- [ ] **VERIFY-55-01**: browser proof and acceptance audit close the milestone honestly

## Locked Clarifications

- [ ] `System` may use compact multi-column layout for stats/info only; config forms, editable fields, and long lists stay single-column and vertically stacked
- [ ] `System` sidebar stays sticky while scrolling; third-level navigation stays closed by default and expands to show active children when the focused second-level anchor is in view
- [ ] when not on `Now`, the navbar shows current event on the left side of the center status group and active task on the right side
- [ ] the `Now` completion bar supports progress over 100% by rendering a second overflow segment
- [ ] `Now` renders all Todoist tags until explicit filtering logic exists
- [ ] the accepted `Threads` filter set for this milestone is `All`, `Unread`, `Needs Review`, `Active`, and `Archived`
- [ ] the navbar `System` info button links to repo docs; create the doc if needed
- [ ] the plus attachment affordance opens a tiny menu with `File` and `Image` entries now, leaving room for future object insertion modes
- [ ] the navbar `System` info button may use a repo-local operator doc as its canonical target for this milestone
- [ ] attachment selection should immediately show a compact queued attachment chip in the composer while durable asset upload remains a follow-on seam

## Coverage Map

### General

- [ ] loading states should have loading spinner (shared component)
- [ ] tags should have lower border radius while action chiplets keep their roundedness
- [ ] plus button should be inside floating action bar on the left
- [ ] errors should be generalized to nudges, for example api errors from the floating nav bar should show as nudges
- [ ] when there is a recorded item in the action bar, there should be an x icon on the right hand side to remove it (asset is still saved)
- [ ] queued assistant feedback and unclear-intent fanout still need final acceptance-level polish and tighter integration with the shared pill language
- [ ] shared pills/tags/filter tags still need final browser-level centering verification across surfaces

### Nudges

- [ ] nudge pill outlines should match the outline color of their icon ring. this color should also set the background, albeit a more muted transformation
- [ ] nudge icon rings should not be touching their associated pills, nor pushing them to the right
- [ ] nudge action chips need to be standardized
- [ ] if nudges are truncated, they need to be able to be vertically expanded in place to view all of the information / actions -- only one nudge can be expanded at a time
- [ ] the nudge title NUDGES (N) should have the alert icon before it
- [ ] nudge left-gutter icon/ring centering and chip non-overlap still need final browser-level cleanup

### Navbar

- [ ] when not on now page, current event and active task (with their icons as showin the now section header) should be shown on either sides of the status icons in the navbar respectfully
- [ ] numbers in badges are too high, not vertically centered
- [ ] info bar / docs link currently does not match the navbar style, no does the link work
- [ ] sync badge still needs exact accepted final temperament for healthy / syncing / error states

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

### Threads

- [ ] then loading thread page, default active thread should be in active state
- [ ] the threads chat icon does not need to be in the central panel
- [ ] the user circles still do not center their letter occupants. the letters should be the same size as the label text they are on a line with, with rings to fit
- [ ] the archive button should be in a circle
- [ ] thread bubble tails still need final visual tuning to match the accepted seamless same-color shape
- [ ] thread row project tags require thread-list metadata that is not in the current contract
- [ ] thread filters are broader now, but still not at the full accepted filter model

### System

- [ ] background of system page should math the background of now and threads
- [ ] there should be a third level in the system sidebar that shows the children of the second level
- [ ] the top level system header in the central container is not needed
- [ ] action buttons should be justified to the right
- [ ] in the system view, there should be multiple columns of stats / info so that the page isn't as long
- [ ] status tags for integrations like connected and configured should have success status colors
- [ ] system is flatter now, but still needs another compactness/readability pass to fully hit the dense single-document target
- [ ] many visible system fields are still UI-local edits only; truthful persisted mutations need API follow-on work

### API

- [ ] assistant-entry intent selection needs a durable persisted contract
- [ ] file / attachment handling needs a real API contract
- [ ] thread list rows need project metadata
- [ ] system settings/config need truthful persisted mutation endpoints

## Completion Rule

Nothing in [TODO.md](/home/jove/code/vel/TODO.md) should remain unchecked at milestone close unless it is explicitly moved into accepted deferred work with a stated reason.
