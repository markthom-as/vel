# Operator Mode Policy

This document defines the progressive-disclosure and mode policy for Vel's operator product.

It answers:

- what should surface by default
- what should stay secondary
- what should remain advanced or internal
- how actions should escalate across `Now`, `Inbox`, `Threads`, and `Settings`

This is a Phase 14 policy document, not a final shell layout spec.

## Core Policy

Vel should optimize the default operator mode for:

- ADHD-friendly minimal actionable context
- one obvious next action
- summary-first trust and recovery guidance
- escalation only when needed

The default product should not expose every pending item equally.

## Default Mode

Default mode is centered on:

- `Now`
- `Inbox`
- the daily loop
- unified action entry
- compact context pane

Policy:

- `Now` should only show items directly when they are urgent enough to deserve immediate attention
- non-urgent work should not clutter `Now`
- `Now` may still show badges, counts, or compact indicators in a subtle context bar or equivalent compact area
- tapping those indicators should usually deep-link into the matching filtered `Inbox` view first
- `Inbox` remains the explicit triage queue
- `Threads` remains the longer-form continuity and archive surface

## Advanced Operator Mode

Advanced mode should expose:

- deeper trust inspection
- grounding inspection
- richer connector and linking detail
- context/stats drill-down
- relaunchable onboarding

This mode is still product-facing, but it is not the first thing the operator should learn.

## Internal / Developer Mode

Internal/developer mode should expose:

- runtime controls
- component control
- low-level diagnostics
- implementation-aware recovery detail

These surfaces should not define the product story.

## `Now` Policy

`Now` should be:

- minimal
- contextual
- action-capable when justified

Rules:

- urgent items may render as inline action cards
- non-urgent items should appear as status badges, counts, or compact deep links
- recently handled items may fall into muted `Now` history at the bottom of the scroll
- `Now` should support a floating microphone or equivalent primary voice entry on mobile

### `Now` action policy

- `review_nudge`
  Usually lighter-weight; should only stay in `Now` when genuinely time-sensitive
- `check_in`
  Usually an inline card with direct accept/answer affordances; may escalate to `Threads`
- `reflow`
  Heavier than routine nudges or check-ins; may carry stronger notification posture

### `Now` reflow card shape

Preferred first surface:

- `Day changed`
- compact, scrollable reflow preview
- primary `Accept`
- secondary `Edit`

`Edit` should open the `Threads` interface so the operator can give feedback and shape the recalculation.

Severity rule:

- lower-severity reflows may apply on direct accept from the compact preview
- higher-severity reflows should show a stronger confirmation or clearer diff before applying
- backend logic should distinguish what may move and what must stay fixed or explicitly constrained

## `Inbox` Policy

`Inbox` is the explicit triage and action queue.

Rules:

- unresolved/actionable items stay primary
- recently handled items may fall into muted recent history
- `Inbox` may expose filters such as `Needs triage`, `Needs reply`, `Needs review`, and `Commitments`
- those filters must remain projections over the canonical action model, not the product model itself

## `Threads` Policy

`Threads` should lean archive/search-first by default.

Rules:

- `Threads` is not the main triage queue
- it is the escalation path for longer clarification flows
- it should support durable history when an interaction becomes meaningfully multi-step
- it does not need to become a durable thread for every one-step inline interaction

## `Projects` Policy

`Projects` should not remain a co-equal primary surface in the default mode.

Current role:

- filtering/context for work
- drill-down into project-specific state
- eventual home for project-specific actions

Project-specific actions should generally remain project-specific rather than being flattened back into generic global surfaces.

Examples:

- project review
- project reflow
- project status correction

Those may still surface through `Inbox` or `Threads` when relevant, but their semantic ownership can remain project-scoped.

Cross-surface identity rule:

- when work is project-scoped, it should usually carry a visible project tag, color, or equivalent identity marker wherever it appears

## Action Visibility Policy

Action presentation should be derived from multiple axes:

- `urgency`
- `importance`
- `blocking_state`
- `disruption_level`

These should remain separate.

Guidance:

- `urgency` and `importance` may be useful to expose to the operator in some contexts, but they should primarily drive product behavior first
- `blocking_state` should remain explicit and separate from salience
- `disruption_level` should drive notification posture and visual interruption cost

## `Check_in` Policy

`check_in` should default to:

- inline `Now` card
- suggested action or answer
- suggested responses with optional custom input
- optional freeform response
- voice-capable response path where appropriate

It may become blocking when it gates:

- fatal recovery
- state reconciliation
- morning start
- end-of-day closure
- supervised transitions

If a blocking `check_in` is ignored:

- keep it pinned
- allow bypass with a warning
- prefer suggested bypass reasons with optional custom voice/text note
- require the operator to note why the bypass happened

## Apple Policy

Apple should remain summary-first.

Rules:

- bounded trust, freshness, and check-in cues are appropriate
- grounding and deeper advanced surfaces should remain less prominent than on web
- eventual parity is desirable, but default mobile embodiment should remain compact and contextual

## Stats Policy

Stats should not remain a separate top-level peer by default.

Preferred direction:

- treat stats/context as a drill-down from the compact context surface
- keep passive observability available without letting it dominate the main operator mode

## Morning And End-Of-Day Policy

Morning start and end-of-day closure can remain special named flows, but they should fit within the same action system rather than becoming a separate product logic family.

Recommended stance:

- keep the named flows
- model them through the same action/check-in/review semantics
- pair them with an auto-generated summary when helpful
- allow them to become blocking when the daily loop explicitly requires closure or confirmation

## Acceptance Criteria

1. Default mode is explicitly minimal and action-focused rather than dashboard-heavy.
2. `Now` only shows direct action cards for genuinely urgent items; other pending work stays summarized.
3. `reflow` is visually and notification-heavier than routine nudges or check-ins.
4. `check_in` stays inline by default but can escalate to `Threads`.
5. `Threads` remains archive/search-first and only becomes durable by default for meaningfully multi-step interactions.
6. `Projects` remains secondary in navigation but may own project-specific action semantics.
7. The policy keeps urgency, importance, blocking, and disruption as separate axes.
