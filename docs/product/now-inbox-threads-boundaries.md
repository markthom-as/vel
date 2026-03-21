# `Now`, `Inbox`, and `Threads` Boundaries

This document defines the current durable boundary between the operator surfaces with the highest overlap risk.

Milestone `v0.2` shipped the bounded MVP loop, but this boundary doc is now read primarily under the authority of [Canonical Now Surface Contract](./now-surface-canonical-contract.md) for post-`v0.2` `Now` work.

The MVP loop in [v0.2 MVP Operator Loop](./mvp-operator-loop.md) still governs escalation discipline and loop shape, but the stricter `Now` surface contract now owns the detailed product behavior.

## Why This Exists

Vel currently risks collapsing three different jobs into one vague "main surface":

- orientation
- triage
- ongoing interactive work

That would make the product feel busy without actually making it clearer.

The current split for `v0.3` planning is:

- `Now` for compressed execution state, top operational pressure, and lightweight actions
- `Inbox` for explicit queue ownership, including daily tasks and previous-day carry-forward tasks
- `Threads` for continuity, explanation, filterable history, and multi-step follow-through

## `Now`

`Now` is the operator's first-glance execution surface.

It should answer:

- what context am I in
- what matters immediately
- what is the dominant action
- what high-priority nudge or advisory needs attention
- what is the fastest safe next action

It should contain:

- the compact status and context lane
- top operational pressure and nudge/action bars
- the compact task subset for the current day
- compact trust, sync, or client-mesh blockers when they affect immediate action
- the docked capture or voice entry

Contextual rendering rule:

- when something needs immediate action, `Now` may show lightweight actions directly
- when deeper continuity is required, `Now` keeps only compact status chips or bars and routes the deeper work into `Threads`
- examples include reflow pressure, needs-input nudges, raw-capture follow-through, and trust or mesh warnings
- recently handled items may remain as compact, muted continuity markers, but `Now` must not grow back into a broad dashboard or second inbox

Mobile note:

- mobile `Now` should keep the same contract with reduced density where needed
- voice and docked capture remain part of the same continuity model rather than a second product lane

It should not become:

- the full queue-management surface
- the full conversation history surface
- the place where deep project structure is managed

Rule of thumb:

- `Now` compresses current execution truth, routes lightweight actions, and preserves continuity markers.
- It may briefly host immediate-action UI when context warrants it.
- It does not own the full queue or long-tail interaction model.
- It should never require the shell to invent alternate task, thread, or sync policy.

## `Inbox`

`Inbox` is the explicit work-queue and triage surface.

It should answer:

- what needs sorting, review, or commitment
- what actionable items are waiting
- what has been captured but not yet resolved
- what deserves promotion, deferment, or dismissal

It should contain:

- todos
- commitments
- daily tasks
- previous-day carry-forward tasks
- captures or promoted items once backend routing makes them inbox-owned work
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
- Surfacing an inbox-owned item in `Now` does not transfer ownership away from `Inbox`.

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
- filterable thread views over shared metadata such as projects, tags, and continuation categories
- entry into subordinate or related subfeeds when needed
- explicit escalation reason, continuation context, and remaining review gate for bounded MVP follow-through

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
- thread detail should expose why the work escalated, what bounded context came with it, and what review/apply gate still exists, rather than forcing the shell to infer continuation semantics from raw message history alone.
- `Threads` also owns canonical `day thread` and `raw capture` continuity lanes once the backend defines them.

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

- raw capture creates thread-backed continuity first and only becomes inbox-owned when backend routing explicitly promotes it there
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

## Closed For Phase 46

These boundary questions are no longer open for `v0.3` planning:

- `Inbox` owns daily tasks and previous-day carry-forward tasks
- `Now` may surface only the highest-priority inbox-owned subset when it becomes current operational pressure
- raw docked capture is thread-first, not inbox-first
- urgent client-mesh or sync problems may surface in `Now`, but deep diagnosis and repair stay in support surfaces
