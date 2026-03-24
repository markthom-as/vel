# Phase 91 Acceptance Checklist

Source: full UI feedback thread from 2026-03-23.

Status vocabulary:
- `done`: implemented in the current web client
- `partial`: some work landed, but the accepted behavior or fidelity is still not fully closed
- `not started`: accepted requirement not yet implemented in a meaningful way
- `API-blocked`: truthful closure requires follow-on contract work tracked in Phase 92

This checklist is the acceptance ledger for the ongoing remediation work. It is intentionally stricter than tests or build status.

## Cross-Surface

- `done` Bottom legacy `voice/capture/ask/command/threads` bar removed.
- `partial` Shared pill/header rhythm is being normalized to the accepted current design direction; archived `Now` is no longer a fidelity target by itself.
- `partial` Over-explained helper text and paneling significantly reduced, but `System` still has density and clarity drift.
- `done` Passing frontend tests are no longer treated as acceptance evidence.

## Composer / Global Input

- `done` Shared `Ask, Capture, or talk to Vel...` input is the only global entry.
- `partial` Composer height/thickness improved, but final visual fit still needs review.
- `partial` Queued follow-up is actionable and dismissible now, but the full accepted affordance behavior is not yet fully closed.
- `partial` Post-send unclear-intent fallback now appears as a right-side popout/fanout instead of an inline row, but final behavior and fidelity still need browser acceptance.
- `API-blocked` Manual fallback intent selection persisting to data level depends on the assistant-entry intent contract in Phase 92.
- `partial` Attachment affordance now exists as a left-side plus-in-circle with no paperclip, but real file behavior is still contract-blocked.
- `API-blocked` Durable file/attachment handling is tracked in Phase 92.
- `partial` Voice unsupported state raises a nudge and the composer now surfaces stronger recorded/transcribing state, but final browser-level polish is still needed.

## Navbar / Shell

- `done` `Vel` wordmark is restored.
- `done` Left navbar content now shows `CLIENT | LOCATION` with mono date/time/timezone below.
- `partial` Extra spacing to the right of the wordmark was improved, but may still need browser tuning.
- `partial` Center badge group is slimmer and color-coded, but exact accepted “faded icon/count-only” fidelity is still not fully closed.
- `partial` Sync badge still needs exact accepted state treatment: faded green when healthy, faded spinning yellow while syncing, red on error.
- `done` Center badges deep-link to relevant surface anchors instead of only switching views.
- `done` Old right-side timezone/instrument strip was removed.
- `done` Docs/info affordance exists to the right of `System`.
- `partial` Badge color semantics are improved, but sync/error/success final visual temperament still needs acceptance review.
- `done` Surface nav links keep their corresponding icons.

## Shared Tags / Badges / Filters

- `partial` Vertical centering of text/icons in pills, tags, and filter tags is still not visually correct enough, including thread filter tags.
- `partial` Filter tags on `Threads` are smaller and closer to the target, but exact parity with the rest of the system is still not closed.
- `done` Filter/search inputs in `Threads` and `System` now use one shared search-field primitive with a right-aligned search icon.

## Nudges

- `done` Encapsulated + floating dual treatment removed; nudges use one floating treatment.
- `done` Heading now reads `NUDGES (N)`.
- `done` Deferred count is surfaced in the heading when present.
- `done` Nudges are no longer page-limited in code.
- `done` Ordering is urgency-first.
- `partial` Floating icon/ring gutter treatment now runs through a shared floating-pill primitive, but exact centering and non-displacing behavior still need visual review.
- `partial` Type color now drives shell/icon treatment, but exact icon/ring/pill matching still needs acceptance review.
- `done` Relative age such as `N MIN AGO` is shown.
- `partial` Action chiplets are smaller and more icon-led, but final density/icon-only behavior, right-edge spacing, and non-overlap are still not fully closed.
- `done` Defer affordance is present.
- `done` Related-thread action can render as icon-only.
- `partial` Nudge body containment is improved, but overflow behavior still needs manual acceptance.
- `done` `+N more` overflow collapse behavior is implemented for the nudge lane.
- `partial` Nudge pills are closer to the task-pill family, but final radius/fidelity still needs visual confirmation.

## Now

- `done` Top-level duplicated `CLIENT | LOCATION` line was removed from `Now` after moving it into the navbar.
- `done` Mono date/time remains in the page header.
- `done` Current event/context line remains under the heading.
- `done` Trust/system nudges are no longer rendered as a separate main-surface `Now` block.
- `partial` Section header badge rhythm is closer to the accepted target, but still needs further fidelity work.
- `done` Top-right `Now` badges now include icons.
- `done` Section titles are uppercase.
- `done` Active-task heading pluralizes.
- `partial` Active tasks can be dragged between local sections, but durable lane semantics are still limited.
- `API-blocked` Durable multi-active lane truth and lane ordering require `/v1/now` contract work in Phase 92.
- `done` Active task rows now use the same base component as other tasks.
- `done` Incomplete tasks use square empty checkboxes.
- `partial` Tag alignment is improved, but final right-edge layout still needs browser review.
- `done` `If time allows` rows are slightly de-emphasized.
- `done` Active tasks are slightly larger / zoomed and brighter.
- `done` `Next up` is a mixed event + committed-task list.
- `partial` Accepted ordering for non-chronological items is only partially truthful because lane semantics are still thin.
- `done` Separate `Current and next event` section was removed.
- `done` Replacement section is `IF TIME ALLOWS`.
- `done` `IF TIME ALLOWS` hides when empty.
- `done` Completed items collapse at the bottom.
- `done` Completed items can be unchecked.
- `done` `If time allows` now carries a backlog marker.
- `done` `Next up` has additional head room.
- `done` `NONE` empty-task label is thin/all-caps.
- `partial` Local section badge content is closer to the accepted counts/icons/colors, but still may need final fidelity polish.

## Threads

- `done` Collapse functionality is removed.
- `done` Sidebar header is `THREADS (N)`.
- `done` Sidebar rows are slightly faded when inactive.
- `partial` Active row brand outline/glow is stronger now, but may still not be at the accepted shimmer fidelity.
- `done` Sidebar separators between thread rows were removed.
- `done` Sidebar has fade treatment at top and bottom.
- `done` Unread dot marker exists in the left gutter.
- `partial` Sidebar row metadata still does not fully match the accepted target because project-tag truth is missing.
- `API-blocked` Project tags in sidebar rows require thread-list metadata in Phase 92.
- `done` Thread header is floating rather than boxed in a large helper panel.
- `partial` Current-thread header now shows participants, last, created, message count, and archive control, but participant sizing and vertical centering still need polish.
- `done` Archive button is smaller and icon-driven.
- `done` Thread title is inline-editable with autosave through the existing route.
- `done` Message bubbles dock by sender and are narrower than full width.
- `partial` Bubble tails were revised again, but the exact accepted seamless same-color tail still needs browser review.
- `partial` Message meta rows are uppercase and de-emphasized, but final tag opacity and alignment still need review.
- `done` Assistant copy affordance exists.
- `done` Review/provenance helper side panel language that was explicitly rejected remains removed.
- `partial` Filter system is broader now, including archived rows, but it is still narrower than the full accepted spec.

## System

- `done` Shared shell dark background wrapper was removed.
- `partial` `System` still has density/readability drift even after repeated flattening and denser shared document rows.
- `done` `On this page`/duplicated anchor widget remains removed.
- `done` Sidebar remains two-level and docked while scrolling.
- `done` Sidebar filter exists.
- `done` Main pane is one vertical anchor document, not a separate browse/detail shell.
- `done` Side-by-side subsection layouts were removed from the main document.
- `partial` Visible container/panel treatment is reduced and more of `System` now runs through shared document rows, but not all dense-document expectations are closed yet.
- `partial` Overview is flatter and more shared-document driven, but `Status` / `Activity` information architecture still needs final acceptance-level cleanup.
- `done` Integrations/accounts/projects/capabilities are colocated with their relevant config/info instead of a separate second-layer detail panel.
- `partial` Activity/recovery are flatter and more colocated with their related detail, but some list/log adjacency still needs cleanup.
- `partial` Some section headers and eyebrow/helper rhythms still feel more designed than the accepted compact target.
- `partial` Fields are now exposed inline, but many are still UI-local edits only and not yet truthful persisted config.
- `API-blocked` Durable persisted `System` settings/config mutations require Phase 92 follow-on work.
- `done` Destructive/integration actions remain visually separated from inline fields.

## Explicit Remaining Implementation Priorities

1. Close the remaining visual-fidelity drift on pills/tags/badges, especially centering and density.
2. Finish the nudge gutter/action behavior, including `+N more`.
3. Tighten `Now` header/badge/task-row fidelity against the accepted current checklist.
4. Tighten `Threads` active-row and bubble-tail fidelity.
5. Flatten `System` further and reduce any lingering pseudo-panel feel.
6. Keep Phase 92 API truth gaps explicit instead of hiding them behind frontend-local state.
