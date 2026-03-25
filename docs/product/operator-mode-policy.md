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
- the web sidebar should default to a thin icon rail: visible enough to discover, compact enough to ignore, and not a place for long explanatory blurbs

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

`Settings` should stay summary-first even when it grows. The general tab should group durable controls into a few clear buckets such as daily-use defaults, planning/recovery, devices/sync, and support/docs instead of one long blended document.

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
- the default `Now` order should be: compact context bar, current status, primary ask/capture/talk input, next event, unified today lane, then compressed attention indicators
- the backend-owned “today” feeding that surface should be sleep-relative rather than midnight-only, so late-night current-day work is not fragmented just because the clock crossed 00:00
- the today lane should stay commitment-first: active item, next up, must/should commitments, then pullable tasks and collapsed recent completions
- `next event` should stay strictly calendar-relevant and future-facing: routine blocks, all-day noise, free/transparent holds, declined events, and cancelled events should not occupy that slot
- `Now` may resurface one clearly ranked thread when follow-through is still immediately relevant, but it should not widen that into a thread inbox by default
- freshness, sync, debug, and broader operational posture should be demoted behind secondary controls rather than occupying primary scroll space

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

Current shipped baseline:

- the backend now computes a bounded same-day remaining-day proposal from persisted commitments and calendar events
- those commitments now carry canonical scheduler rules as durable backend semantics, so shells should not infer scheduling meaning from raw label syntax
- durable routine blocks and bounded planning constraints now feed the same backend-owned planning substrate before `reflow` runs, with inferred routine fallback only when no durable blocks are configured
- `Now` should render aggregate counts plus compact `moved`, `unscheduled`, and `needs_judgment` rows from that typed proposal
- `Settings` may summarize the same recovery posture, but it should not become a second planner surface
- `Threads` remains the escalation lane for longer shaping or disagreement

Current planning-contract note:

- Phase 28 now publishes a bounded proactive day-plan contract over routine blocks, calendar anchors, and canonical scheduler rules
- implementation should keep `day_plan` and later `reflow` on one backend-owned substrate rather than creating a second planner model
- current shipped shells now consume optional typed `day_plan` output from `GET /v1/now` directly: `Now` shows the compact plan plus whether the day is using operator-managed routines or inferred fallback, `Threads` carries longer shaping/disagreement, and `Settings` summarizes posture without becoming a planner
- Phase 30 now exposes typed planning-profile management over that same substrate: `Settings` can inspect and mutate durable routine blocks and bounded planning constraints, but shells still do not own planning semantics locally
- Phase 31 extends inspection parity and staged edit parity across CLI, Apple, and assistant/voice entry: those surfaces can now read the same planning profile and stage bounded profile edits, but confirmation and thread continuity remain explicit and the profile is not silently mutated by conversational shells
- Phase 32 now closes the supervised apply lane: approved planning-profile proposals can apply through the canonical backend mutation seam, but proposal state and applied/failed outcomes still remain explicit continuity in `Threads`, `Now`, and summary surfaces rather than becoming inline planner writes
- Phase 33 now applies the same pattern to same-day schedule changes: `day_plan` / `reflow` proposals can resolve through the canonical commitment-scheduling apply seam, while `Now`, CLI, and Apple only show compact pending/applied/failed continuity from backend state

`Edit` should open the `Threads` interface so the operator can give feedback and shape the recalculation.

Severity rule:

- lower-severity reflows may apply on direct accept from the compact preview
- higher-severity reflows should show a stronger confirmation or clearer diff before applying
- backend logic should distinguish what may move and what must stay fixed or explicitly constrained
- shells should consume typed `reflow` transitions instead of inferring lifecycle actions from button labels alone

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
- the default framing should emphasize continuity and resume-ability over “chat” identity, so operators understand it as follow-through rather than a second inbox

## `Projects` Policy

`Projects` should not remain a co-equal primary surface in the default mode.

Current role:

- filtering/context for work
- drill-down into project-specific state
- eventual home for project-specific actions

Project-specific actions should generally remain project-specific rather than being flattened back into generic global surfaces.

Examples:

- project workflow cadence
- project reflow
- project status correction

Project-workflow cadence is currently disabled in active operator queues while this lane is reworked. It should be turned into a workflow-owned cadence surface in the workflow migration phase so project follow-through stays explicit and attributable.

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

Current migration note:

- the first backend-owned trust/readiness seam is allowed to compose backup trust, freshness state, pending writebacks/conflicts, and pending execution review pressure into one summary-first `Now` projection
- shells should consume that typed readiness summary instead of independently recomputing trust posture from lower-level fields
- when readiness is degraded, shells should render backend-provided follow-through actions from the same canonical action model instead of inventing separate recovery semantics

## `Check_in` Policy

`check_in` should default to:

- inline `Now` card
- suggested action or answer
- suggested responses with optional custom input
- optional freeform response
- voice-capable response path where appropriate

Implementation rule for the current migration lane:

- the first backend-owned `check_in` seam may derive from active daily-loop prompt state
- `Now` should consume that typed seam rather than owning check-in semantics locally
- escalation toward `Threads` should remain metadata/linkage rather than hard-coded shell behavior
- shells should treat the typed `check_in` transition list as the valid next-step contract rather than improvising additional semantic actions
- submit and bypass validation, including any required bypass note, should be enforced in Rust service layers rather than left to shell goodwill

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

Phase 37 adds an additive iPhone embedded-capable seam for bounded local helper flows, but Apple still must not become a second policy or authority brain. Embedded use is justified for responsiveness and offline resilience only where the boundary is explicit and fail-closed; daemon-backed truth remains primary.

Rules:

- bounded trust, freshness, and check-in cues are appropriate
- local-first voice recovery may surface as compact draft/pending/merged continuity in Apple `Now` and `Threads`, but Apple must still defer to canonical thread identity and backend-owned answers when reconnect happens
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
