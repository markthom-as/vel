# `Now`, `Inbox`, and `Threads` Boundaries

This document defines the working product boundary between the three operator surfaces that still have the most overlap risk.

It is a discovery artifact for Phase 14, not a final shell layout spec.

## Why This Exists

Vel currently risks collapsing three different jobs into one vague "main surface":

- orientation
- triage
- ongoing interactive work

That would make the product feel busy without actually making it clearer.

The correct split is:

- `Now` for orientation and immediate pressure
- `Inbox` for explicit triage
- `Threads` for parallel interactive work and history

## `Now`

`Now` is the operator's first-glance surface.

It should answer:

- what context am I in
- what matters immediately
- what is the current routine block, event, or task
- what high-priority nudges or advisories need attention
- what is the fastest safe next action

It should contain:

- the compact context pane
- daily-loop entry or active-session status
- summary-level trust or onboarding blockers
- the unified action entry
- a compact rendering of today's commitments or priority items when helpful

Contextual rendering rule:

- when something needs immediate action, `Now` may show actionable cards or controls directly
- when no immediate action is needed, `Now` should prefer summary counts and compact previews that link out to `Inbox` or `Threads`
- examples include inbox pressure, nudges needing triage, and threads needing attention
- once an item is acted on, it can leave the active `Now` area and fall into a muted `Now` history section near the bottom of the scroll for lightweight review

Mobile note:

- mobile `Now` should work as a scrollable summary/action surface
- a floating microphone button is a strong fit for voice-first capture or conversation entry on mobile

It should not become:

- the full queue-management surface
- the full conversation history surface
- the place where deep project structure is managed

Rule of thumb:

- `Now` summarizes and routes.
- It may briefly host immediate-action UI when context warrants it.
- It does not own the full long-tail interaction model.
- A lightweight, greyed-out `Now` history section is acceptable for recently handled items, as long as it stays secondary to the active surface.

## `Inbox`

`Inbox` is the explicit work-queue and triage surface.

It should answer:

- what needs sorting, review, or commitment
- what actionable items are waiting
- what has been captured but not yet resolved
- what deserves promotion, deferment, or dismissal

It should contain:

- captures
- todos
- commitments
- reviewable suggestions when they need triage
- project-linked items when they are still actionable queue items

Rendering rule:

- unresolved or still-actionable items should stay in the active inbox area
- recently handled items may fall into a muted recent-history section near the bottom of the inbox
- long-term completed/archive history should not dominate the main inbox view

It should not become:

- the top context/orientation surface
- the canonical long-form conversation history surface
- a dense project-management dashboard

Rule of thumb:

- `Inbox` is where the operator decides what to do with incoming or pending work.
- It is the main work surface, but not the whole product.
- It should mirror `Now` by keeping active items prominent and recently handled items visible but visually demoted.

## `Threads`

`Threads` is the support surface for parallel interactive work.

It should answer:

- what conversations or workstreams are active
- what jobs, agent interactions, or longer-running exchanges exist
- what happened in a specific interaction stream
- how do I search or filter past work context

It should contain:

- thread history
- running or recent workstreams
- rich interaction records
- search and filtering across conversation/work streams
- entry into subordinate or related subfeeds when needed

Default posture:

- `Threads` should lean more toward archive, continuity, and search than toward acting as a second inbox
- active threads may still be visible, but the default model should favor retrieval, filtering, and drill-down over top-level triage pressure

It should not become:

- the default first screen for everyday use
- the primary triage queue
- the place that defines product state for orientation or commitments

Rule of thumb:

- `Threads` is for continuity, parallelism, and searchable history.
- It should be powerful on desktop in particular.
- `Now` and `Inbox` may deep-link into specific thread messages or filtered thread views when attention is needed, without making `Threads` itself the main triage surface.

## Shared Entry, Distinct Destinations

The unified action entry may create overlap because the same initial action can lead to:

- a capture
- a command
- a voice or text conversation
- a thread

That is acceptable if the routing outcome is clear.

Recommended behavior:

- the primary entry lives in `Now`
- it routes automatically by default
- it offers an override when the operator wants to force capture vs chat vs command
- resulting artifacts land in the surface that actually owns them

Examples:

- quick capture lands in `Inbox`
- interactive exchange creates or joins a `Thread`
- daily-loop action returns to `Now`

## Projects Relationship

`Projects` should not compete with these three surfaces.

For now, the working role of Projects is:

- filtering and grouping inbox/work views
- providing project-specific context
- exposing project-level configuration or logic affordances later

That keeps the primary surface model simpler:

- `Now` for orientation
- `Inbox` for triage
- `Threads` for interactive parallel work

## Provisional Navigation Consequence

If forced to choose top-level destinations today, the cleanest set remains:

- `Now`
- `Inbox`
- `Threads`
- `Settings`

`Projects` should remain reachable, but not top-level by default until its role is clearer.

## Open Questions

These remain intentionally open for later Phase 14 discovery:

- how much inbox summary should appear on `Now` before it starts feeling like a second inbox
- whether `Threads` should expose lightweight "running work" cards outside the main thread surface, even if its default posture remains archive/search-oriented
- how much project context should appear inline inside `Inbox`
- whether desktop gets richer thread visibility than mobile by default
