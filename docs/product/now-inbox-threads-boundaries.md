# `Now`, `Inbox`, and `Threads` Boundaries

This document defines the current durable boundary between the operator surfaces with the highest overlap risk.

For milestone `v0.2`, it should be read under the authority of [v0.2 MVP Operator Loop](./mvp-operator-loop.md), not as an open discovery artifact.

## Why This Exists

Vel currently risks collapsing three different jobs into one vague "main surface":

- orientation
- triage
- ongoing interactive work

That would make the product feel busy without actually making it clearer.

The current split is:

- `Now` for overview, commitments, and immediate pressure
- `Inbox` for explicit triage
- `Threads` for bounded multi-step continuation and history

## `Now`

`Now` is the operator's first-glance MVP surface.

It should answer:

- what context am I in
- what matters immediately
- what is the dominant action
- what high-priority nudge or advisory needs attention
- what is the fastest safe next action

It should contain:

- the compact context pane
- the daily-loop overview and commitment entry
- summary-level trust or onboarding blockers
- the unified action entry
- a compact rendering of today's commitments, timeline, and reflow pressure when helpful

Contextual rendering rule:

- when something needs immediate action, `Now` may show actionable cards or controls directly
- when no immediate action exists, `Now` should show 1-3 suggestions with inline choices or a route into `Threads`
- examples include reflow pressure, nudges needing triage, and commitment decisions
- once an item is acted on, it can leave the active `Now` area and fall into a muted `Now` history section near the bottom of the scroll for lightweight review
- check-ins should normally appear here as inline cards with a suggested next action and an option to continue in `Threads` when the question cannot be resolved inline
- actions shown in `Now` still need an importance/urgency distinction; `reflow` should generally render heavier and notify more aggressively than a normal `check_in` or routine nudge

Mobile note:

- mobile `Now` should work as a scrollable summary/action surface
- a floating microphone button is a strong fit for voice-first capture or conversation entry on mobile

It should not become:

- the full queue-management surface
- the full conversation history surface
- the place where deep project structure is managed

Rule of thumb:

- `Now` summarizes, routes, and owns the inline MVP loop.
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

Project identity rule:

- when an inbox or now item is project-scoped, the compact project marker should come from the backend-owned action seam instead of being reconstructed in the shell from a separate projects query
- when a project-scoped item needs longer-form follow-up, the backend-owned action seam should also carry a typed thread-routing hint so shells can deep-link into filtered `Threads` views without inventing routing semantics locally

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

`Threads` is the support surface for bounded multi-step continuation.

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

- `Threads` should lean more toward continuity, reviewable work context, and search than toward acting as a second inbox
- active threads may still be visible, but the default model should favor bounded escalation, retrieval, filtering, and drill-down over top-level triage pressure

It should not become:

- the default first screen for everyday use
- the primary triage queue
- the place that defines product state for orientation or commitments

Rule of thumb:

- `Threads` is for continuity, bounded escalation, and searchable history.
- It should be powerful on desktop in particular.
- `Now` and `Inbox` may deep-link into specific thread messages or filtered thread views when attention is needed, without making `Threads` itself the main triage surface.
- Those deep links should come from typed backend routing hints where possible, especially for project-scoped actions.

Escalation rule:

- `Threads` should only take over when work becomes genuinely multi-step.
- In `v0.2`, that means at least two of the following are true:
  - the work needs explanation
  - the work needs multiple decisions
  - the work needs tool or context work

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

See [operator-mode-policy.md](operator-mode-policy.md) for the disclosure rules that sit on top of these boundaries.

## Open Questions

These remain intentionally open for later Phase 14 discovery:

- how much inbox summary should appear on `Now` before it starts feeling like a second inbox
- whether `Threads` should expose lightweight "running work" cards outside the main thread surface, even if its default posture remains archive/search-oriented
- how much project context should appear inline inside `Inbox`
- whether desktop gets richer thread visibility than mobile by default
