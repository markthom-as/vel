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

## Draft Canonical Fields

The exact DTO or Rust type name is for later phases, but the discovery-level schema should include these concepts.

### `action_kind`

Examples:

- triage
- reply
- review
- commit
- defer
- approve
- connect
- fix
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

### `urgency`

Examples:

- now
- today
- soon
- passive

This helps control whether `Now` should show an actionable card or just a linked summary.

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
- `Needs review` may include actions whose `action_kind` is `review` or `approve`
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
- `recover_freshness`
  When: a stale/degraded input state warrants a sync, refresh, or rerun
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
  When: a suggestion, writeback, or proposed action needs judgment
- `approve`
  When: a supervised or gated action is ready for human confirmation
- `connect`
  When: onboarding or integration setup requires linking a source or enabling a capability
- `fix`
  When: a broken source, stale connector, or invalid setup path needs repair
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

### `Settings`-leaning action kinds

These belong to onboarding, integration, trust, or advanced operator surfaces.

- `begin_onboarding`
  When: a first-time or relaunchable setup journey should start
- `continue_onboarding`
  When: setup is incomplete and the next safe step is known
- `link_client`
  When: a new device or node needs to be attached to the same Vel identity/runtime
- `configure_connector`
  When: a provider, snapshot path, or integration family needs setup
- `repair_connector`
  When: an existing integration is broken, stale, or partially invalid
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
