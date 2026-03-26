# Phase 90 Acceptance Feedback

Source: direct user acceptance review on 2026-03-23

## Cross-Surface

- The current UI is not in a passing state.
- Passing tests, builds, and proof artifacts do not count as acceptance.
- The most important archived elements to reuse are:
  - pill layout for tasks and nudges
  - section-header badges/tags from archived `Now`
- The main failure mode is over-explained UI and excessive paneling.

## Composer / Global Input

- Remove the bottom `voice/capture/ask/command/threads` bar entirely.
- Move all command/capture/voice entry into the shared `Ask, Capture, or talk to Vel...` field.
- Infer mode by default.
- If intent is unclear, show a small popout/fanout from the right side of the composer after send/submit.
- The suggested intent should appear at the top.
- No intent fanout should appear when there is no pending payload.
- Manual mode selection is still required for this milestone as fallback behavior.

## Nudges

- There should not be both encapsulated and floating nudge treatments.
- Nudges should use the floating treatment only.
- Layout should follow the archived design more closely.
- Nudges should be vertically slim, roughly one to two lines tall.
- The glowing icon should float outside on the left.
- Action chiplets should sit on the right.
- This should apply on every page.
- Rename `Nudge zone` to `Nudges`.
- Remove `Separate interruption lane.`
- Order nudges by urgency first.
- Overflow should collapse behind `+N more`.
- All nudges share one shape; icon and color vary by type.
- Trust/sync nudges keep the same shape but use distinct icon/color treatment.

## `Now`

- Put `CLIENT_NAME | LOCATION` above the `Now` heading where `Current moment` currently is.
- For now, use `Location Unknown` when location is missing.
- Below the `Now` heading, show the date/time string including timezone.
- Below that, show `current event | context`.
- The trust-state nudge should not appear inside the main `Now` surface.
- Trust/sync nudges belong in the sidebar with the other nudges.
- The `Now` page should use a section header with badges counting alerts and showing sync state on the right side, closer to the archived design.
- Under that header should be `Active Task`.
- Remove:
  - `Dominant work object`
  - `Anchor`
- Tasks should use large left-side checkboxes.
- Remove the right-side completion chip/action for tasks.
- The old task layout is the ideal baseline.
- `Next up` should be one mixed chronological list of:
  - upcoming calendar events
  - committed tasks
- If some items have no chronology, place them after calendar events in descending priority order.
- Remove:
  - `one subordinate slot`
  - `1 slot`
  - any limiting behavior on item count there
- Remove the separate `Current and next event` section.
- Replace that area with `If time allows`.
- `If time allows` contains same-day uncommitted tasks.
- Hide `If time allows` when empty.
- Checking a task should complete it immediately.
- Completed items should move into a collapsed section at the very bottom of the `Now` list.
- Completed items should be able to be unchecked.
- The input bar on `Now` should stay where it is.

## `Threads`

- Remove helper text:
  - `Sticky per thread.... identity.`
  - `Continuity stream`
  - `Chronology follows.... identity.`
- Remove the entire `Shared review panel`.
- Remove the entire sticky side panel.
- The thread header should be floating like the archived `Now` section header.
- Chat bubbles below it should also float.
- Bubbles should dock left/right by speaker.
- Thread filters should use the same slim tag style as the rest of the system.
- Thread filtering should be expanded to match the spec.
- The active thread in the left rail should use the standard active brand accent.
- The left rail should be collapsible.
- Thread-list items should show:
  - relative freshness as primary timestamp
  - last message time
  - maybe thread creation date
- The opened thread header should show:
  - title
  - thread creation date
  - last message date
  - number of messages
  - tags
  - archive button
- Review affordances that require user input should appear as visible, interactable messages in the thread with action chiplets.
- Thread creation date can stay in the opened header rather than every row.

## `System`

- `System` is currently the worst surface.
- Remove helper text:
  - `One surface, five sections. Trust stays legible. Operations stay bounded.`
- Remove the entire top `System / Structural truth...` panel.
- The interface needs to be more compact and information-dense.
- There are too many nested panels.
- The left rail should be compact.
- The left rail should use separator lines.
- Remove active-state tags from the left rail.
- The rail should stay two-level with indentation for the second level.
- The current second-layer browse panel should go away as a separate content block.
- The bottom of the left rail should have a contextual ToC with links to session anchors.
- The main pane should become a long full-width scrollable settings document.
- This leaves the full width of the section for actual settings.
- Settings sections also need to be compact.
- Prefer:
  - line separators
  - less padding
  - less helper text
  - fewer tags
- `System` should not have its own dark background.
- Values in these menus must be editable.
- Editing should be inline.
- Save/apply behavior should be immediate.
- Destructive/integration actions should be separated visually from inline editing.

---

## Second Acceptance Pass

Source: direct user acceptance review on 2026-03-23 after the first remediation checkpoint

### `Now`

- top-right `Now` badges should include icons
- titles such as `CLIENT | LOCATION`, `ACTIVE TASK`, `NEXT UP`, and `IF TIME ALLOWS` should be all caps
- `Active task` should pluralize when multiple tasks are active
- users should be able to drag tasks between sections, changing state
- the active task should use the same underlying task component as the other task rows, only styled as active
- active-task tags should sit inside the task row
- incomplete tasks should use square empty checkboxes
- task tags should be right aligned
- `If time allows` tasks should be slightly less opaque
- active tasks should be slightly larger / zoomed relative to the rest

### `Nudges`

- heading should show count as `Nudges (N)`
- nudges should not be artificially limited on specific pages
- nudge type color should drive border, background, icon, and icon ring
- icons should be perfectly centered within rings
- icon/ring and pill body should be vertically centered together
- icons should sit in the left gutter as decorative markers rather than displacing the pill body
- nudges should show action chiplets

### Shared composer / action entry

- queued follow-up state is currently stranded:
  - it is not clickable
  - it does not clear
  - there is no way to mark intent
- the shared entry should likely be slightly taller to match the design language
- microphone / recording currently appears non-functional

### `Threads`

- remove collapse functionality
- `THREADS` sidebar header should be all caps
- filter tags should match the size of tags used elsewhere, especially the message tags
- active thread rows need the active brand shimmer outline
- non-active thread rows should be slightly less opaque
- sidebar rows should remove created-time pills and use project tags instead
- sidebar should be scrollable with fade treatment at top and bottom
- archive button should be small, subtle, and icon-driven
- thread header metadata should use separators
- message tags should be right aligned while time stays left aligned
- message bubbles should adopt the standard chat tail treatment

### `System`

- a dark background treatment still remains and should be removed
- remove `On this page` from the sidebar
- separator structure should align more closely with `Threads`
- `System` should stay a single scrolling page with anchors in the left rail
- main content should stack vertically; no side-by-side sections
- system items should not sit in visible containers with dominant borders/padding
- lists should show complete content
- general stats should lead each section, with configuration below
