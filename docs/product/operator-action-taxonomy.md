# Operator Action Taxonomy

This document defines the working action taxonomy draft for Phase 14 product discovery.

Its purpose is to prevent UI labels such as `Needs triage` or `Needs review` from becoming the real product model. Those labels should stay as filter views over a more durable action schema.

## Why This Exists

Vel is converging on:

- `Now` as contextual orientation and immediate action
- `Inbox` as explicit triage
- `Threads` as archive/search and interactive continuity

That surface split will not stay coherent unless the underlying action model is explicit.

Without a canonical action taxonomy, the product will drift into:

- one-off filter categories
- shell-specific action semantics
- unclear permission rules
- duplicated logic across web, Apple, CLI, and future desktop shells

## Working Principle

The canonical model should describe:

- what action is being requested
- who can or should perform it
- whether it can happen automatically
- where it should surface by default
- how urgent it is
- what object or state it refers to
- why it is being shown

The canonical model should not be defined by:

- a sidebar label
- a specific UI card type
- one shell's layout

## Phase 15 Migration Rule

The implementation migration seam for this taxonomy is now:

1. core contract in `vel-core::operator_queue`
2. backend queue synthesis in `veld::services::operator_queue`
3. read-model composition in backend services such as `Now`
4. DTO mapping in `vel-api-types`

That keeps action semantics backend-owned while still letting different shells render the same action differently.

## Draft Canonical Fields

The exact DTO or Rust type name is for later phases, but the discovery-level schema should include these concepts.

### `action_kind`

Examples:

- triage
- reply
- review
- check_in
- commit
- defer
- connect
- reflow
- inspect
- resume

This is the core semantic action type.

### `actor`

Examples:

- user
- auto
- suggested
- supervised_agent

This distinguishes who is expected to act or who proposed the action.

### `permission_mode`

Examples:

- auto_allowed
- user_confirm
- blocked
- unavailable

This is necessary so the product does not confuse advisory suggestions with actions that are actually permitted to run.

### `surface_affinity`

Examples:

- now
- inbox
- threads
- settings

This indicates the default surface where the action should appear first, without preventing deep links from other surfaces.

### `scope_affinity`

This answers:

- is the action global, project-scoped, thread-scoped, connector-scoped, or otherwise attached to a narrower domain?

Examples:

- global
- project
- thread
- connector
- daily_loop

This helps preserve semantic ownership even when an action is surfaced through shared queues such as `Now` or `Inbox`.

### `urgency`

This answers:

- when does this matter?

It should likely be modeled as a scalar or tiered ordinal rather than a rigid semantic enum.

### `importance`

This answers:

- how much product emphasis does this deserve?

It should likely be modeled as a scalar or tiered ordinal rather than a rigid semantic enum.

Working rule:

- urgency answers "when does this matter?"
- importance answers "how heavily should the product emphasize it?"

### `blocking_state`

This answers:

- does this action block other work, or is it blocked by something else?

This should remain separate from urgency and importance.

Examples:

- none
- blocking
- blocked_by_user
- blocked_by_system

### `disruption_level`

This answers:

- how interruptive should the presentation or notification be?

This should remain separate from both urgency and importance.

Examples:

- silent
- ambient
- visible
- interruptive

### `state`

Examples:

- active
- snoozed
- handled
- dismissed
- blocked

This supports the active-versus-muted-history behavior already being defined for `Now` and `Inbox`.

### `source_ref`

This points to what produced the action, such as:

- a nudge
- a trust projection
- a connector issue
- a thread event
- a daily-loop state transition

### `target_ref`

This points to what the action is about, such as:

- a commitment
- a capture
- a thread
- a project
- a connector
- a review item

Cross-surface display rule:

- if an action is project-scoped, that project identity should usually remain visible through a tag, color, or similar compact marker wherever the action appears
- the typed action seam should carry compact project identity directly rather than forcing shells to rediscover project labels or families from a separate lookup before they can render that marker

### `explainability`

Each surfaced action should carry:

- a short reason summary
- a path to inspect deeper details when needed

This matches the current Phase 14 rule of summary-first with inspectable raw detail behind it.

## Relationship To Surface Filters

Filters are still useful, but they should be derived from the canonical action taxonomy.

Examples:

- `Needs triage` may include actions whose `action_kind` is `triage`
- `Needs reply` may include actions whose `action_kind` is `reply`
- `Needs review` may include actions whose `action_kind` is `review`
- `Commitments` may be a target-oriented view over actions attached to commitments

The filter vocabulary can evolve without forcing a redesign of the backend-owned action model.

## First-Pass Action Inventory

This is the current discovery-level inventory of action kinds Vel should likely support first.

These are not final DTO values yet. They are the product-semantic set Phase 14 should preserve so later phases do not derive the model from ad hoc UI filters.

### `Now`-leaning action kinds

These are the kinds most likely to deserve direct actionable UI in `Now` when urgency and permission justify it.

- `start_daily_loop`
  When: the operator has not oriented yet and the daily loop should begin
- `resume_daily_loop`
  When: there is already an active morning/standup session
- `review_nudge`
  When: a high-priority advisory or nudge needs explicit attention now
- `acknowledge_advisory`
  When: something should be seen and cleared without becoming a full inbox task
- `check_in`
  When: Vel should query the operator to repair context, collect missing metadata, confirm reality, or clarify ambiguous state
  Typical uses:
  - update current context after drift
  - capture missing task/event metadata
  - confirm whether something was completed, skipped, or changed
  - gather human input before a reflow or downstream suggestion
  Preferred embodiment:
  - default as an inline card in `Now`
  - offer a suggested action the operator can accept directly
  - prefer suggested responses with optional custom voice/text input
  - allow escalation into a longer interactive flow through the `Threads` interface when needed
  Typical emphasis:
  - usually moderate disruption
  - importance depends on how much downstream work is blocked
  Blocking note:
  - check-ins are often non-blocking, but they may become blocking for recovery, state reconciliation, supervised transitions, morning start, or end-of-day closure
- `recover_freshness`
  When: a stale/degraded input state warrants a sync, refresh, or rerun
- `reflow`
  When: missed calendar events, slipped tasks, or changed day context require recalculating the remaining plan
  Default interaction: auto-suggested, user-confirmed
  Likely triggers:
  - stale schedule
  - missed event
  - slipped planned block
  - major sync change affecting today
  - task no longer fits the remaining available time
  Typical emphasis:
  - higher importance by default than routine nudges or check-ins
  - may become blocking when the current plan is no longer trustworthy for the rest of the day
  - should be visually heavier and more notification-forward than routine `review_nudge` or `check_in`
  Acceptance note:
  - direct accept may be enough for lower-severity reflows
  - higher-severity reflows should show a clearer confirmation or diff before applying
  - backend planning should distinguish movable versus fixed constraints
- `continue_current_focus`
  When: the system can confidently route back to the current task, routine block, or event
- `capture`
  When: the operator invokes the unified entry for quick intake
- `start_conversation`
  When: the unified entry should route into text or voice interaction rather than capture
- `resume_thread`
  When: a specific thread needs immediate attention from the top-level context surface

### `Inbox`-leaning action kinds

These are the main triage and queue-management actions.

- `triage`
  When: an item is new, unsorted, or unresolved
- `commit`
  When: a task or capture should become an explicit commitment
- `defer`
  When: work should remain tracked but move out of the active queue
- `dismiss`
  When: an item should be intentionally cleared rather than ignored
- `reply`
  When: the operator owes a response or follow-up
- `review`
  When: a suggestion, writeback, supervised action, or proposed change needs judgment
- `connect`
  When: onboarding or integration setup, relinking, configuration, or repair requires enabling or restoring a capability
- `classify`
  When: a capture, task, or inbox item needs categorization or routing

### `Threads`-leaning action kinds

These actions are usually accessed through deep links or filtered thread views rather than top-level triage pressure.

- `open_thread`
  When: the user wants to inspect a specific workstream or conversation
- `resume_thread`
  When: a thread is active again and the operator should continue it
- `inspect_thread_context`
  When: the operator needs deeper history or provenance for a thread-linked issue
- `reply_in_thread`
  When: the expected action is best expressed as part of an ongoing interaction stream
- `review_thread_output`
  When: a job, agent, or parallel workstream has produced something that needs inspection

Visibility note:

- active-work affordances may stay implicit through search, scroll position, and filtered views rather than requiring a separate top-level thread mode initially

### `Settings`-leaning action kinds

These belong to onboarding, integration, trust, or advanced operator surfaces.

- `begin_onboarding`
  When: a first-time or relaunchable setup journey should start
- `continue_onboarding`
  When: setup is incomplete and the next safe step is known
- `link_client`
  When: a new device or node needs to be attached to the same Vel identity/runtime
- `connect`
  When: a provider, snapshot path, integration family, or linked client needs setup, relinking, or repair
- `inspect_trust`
  When: summary-level trust state needs deeper inspection
- `inspect_grounding`
  When: agent/data/tool awareness needs to be inspected beyond summary state
- `inspect_runtime`
  When: deeper diagnostics or operational controls are needed

## Candidate Filter Views

The likely first filter views derived from the inventory above are:

- `Needs triage`
- `Needs reply`
- `Needs review`
- `Commitments`
- `Setup and recovery`
- `Attention now`

These are useful views, but they should remain projections over the canonical action model.

## Emphasis Guidance

Not every `Now` action should look or notify the same way.

Initial guidance:

- `review_nudge`
  Usually lighter-weight than `reflow`; may remain summary-first unless urgency is truly immediate
- `check_in`
  Usually inline and conversational, not alarm-like; it should ask for clarification without feeling punitive
- `reflow`
  Heavier visual treatment and stronger notification posture because it signals the current plan may no longer be reliable

This should remain a backend-ownable semantic distinction, even if shell-specific styling differs later.

## Multi-Axis Modeling Note

The current discovery direction is:

- keep `action_kind` semantic and categorical
- keep `urgency` separate from `importance`
- keep dependency or blockage state in its own `blocking_state`
- keep interruptiveness in its own `disruption_level`

This avoids collapsing timing, salience, dependency pressure, and notification cost into one overloaded field.

See [operator-mode-policy.md](operator-mode-policy.md) for how these axes should affect default-mode visibility, heavier `reflow` treatment, and blocking `check_in` behavior.

## Evidence Behind The Inventory

This first-pass inventory is grounded in:

- the current Phase 14 `Now` / `Inbox` / `Threads` boundary work
- the existing daily-loop and trust surfaces already present in the repo
- the Todoist-style user-story export in `/home/jove/Downloads/Vel.csv`

The export particularly reinforces:

- freshness/recovery actions
- onboarding and linking actions
- integration repair and setup actions
- thread resumption and thread continuity
- contextual `Now` actions rather than a loud dashboard
- richer projects as drill-down/context instead of a primary action surface
- context-aware prompting and metadata collection as part of daily operation rather than as buried setup work

The local `codex-workspace` scheduler and calendar tooling reinforce one more important action kind:

- `reflow`
  Evidence: `/home/jove/code/codex-workspace/docs/scheduler.md` models scheduling as a recalculated plan over current calendar blocks, due tasks, routine windows, and unscheduled leftovers; `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js` produces plan/apply operations plus unscheduled output and completion-based calendar realignment.
  Product implication: when calendar reality changes or an event is missed, Vel should surface a first-class reflow/recalculate action rather than only a passive warning.

## Migration Note

The long-term direction is that much of the current `codex-workspace` scheduling and calendar-flow logic should migrate into Vel's canonical Rust-owned product core over time.

For now, Phase 14 should preserve the behavioral concepts rather than copying implementation details directly:

- snapshot-driven plan/recalculate flow
- conflict and missed-event detection
- unscheduled leftovers that remain reviewable
- local urgent/defer style operator decisions
- completion-based calendar realignment

Later phases should port the durable logic intentionally into Vel-owned seams instead of leaving it split across external workspace scripts.

## Product Consequences

This taxonomy supports the current product direction:

- `Now` can show direct actionable UI only when urgency and permission justify it
- `Inbox` can act as the main triage queue over canonical action records
- `Threads` can remain archive/search-oriented while still exposing thread-linked actions through deep links
- Apple, web, CLI, and future desktop shells can share one action language even if they embody it differently

## Phase Boundary

Phase 14 should define this taxonomy at the discovery level.

Later phases should use it as follows:

- Phase 15: create the right backend and transport seams for canonical action ownership
- Phase 16: implement Rust-owned action generation, routing, and state transitions
- Phase 17: apply the action model cleanly across shell embodiment and navigation

## Open Questions

These remain open for later discovery or implementation phases:

- whether actions need a first-class confidence field separate from explainability
- whether permission mode should include finer-grained review-gate states
- how much of this taxonomy should be directly visible to operators versus only used internally for routing and filtering
