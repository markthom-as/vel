# Vel UI v4 Spec

Purpose: define a high-level redesign for Vel's operator UI around attention hierarchy, cognitive layers, and observability.

This is a planning document only. It does not change shipped behavior by itself. Use [docs/status.md](../status.md) for current implementation truth.

Source material:

- imported packet: `~/Downloads/vel-ui-v4-spec-pack.zip`
- screenshot set: `~/Downloads/localhost_5173_.png` and `~/Downloads/localhost_5173_ (1).png` through `~/Downloads/localhost_5173_ (7).png`

## Why this exists

The current web operator UI has useful pieces, but the screenshot set shows the surfaces are still carrying too many responsibilities at once.

Observed issues from the screenshot set:

- `Now` mixes action selection, schedule, freshness, sync recovery, recent-source visibility, and raw debug in one dense page.
- the right rail stays overloaded with context explanation and raw state across surfaces where it is not always the main task.
- `Suggestions` exposes rich payload/evidence detail, but the framing still feels closer to a debug inspector than a steering workspace.
- `Settings` has meaningful runtime controls, but policy and observability are split across tabs without a clearer top-level IA.
- `Threads` is still not visibly acting like a continuity/process surface in the screenshots.

The redesign goal is not "make it prettier". It is to make each surface answer one primary operator question clearly.

## Information Architecture

Primary navigation:

- Now
- Inbox
- Threads
- Suggestions
- Stats
- Settings

Intended layer split:

- operational: `Now`, `Inbox`
- cognitive: `Threads`, `Suggestions`, `Context`
- observability and control: `Stats`, `Settings`

## Screenshot-backed design guidance

### Current `Now` problem

The screenshots show `Now` carrying:

- active state summary
- upcoming events
- operational state
- backlog/tasks
- freshness and sync actions
- recent source activity
- extra commitments
- raw debug fields
- full context rail

This creates a page that is informative but too diffuse to be a decisive "what should I do now?" surface.

### Current context/right-rail problem

The screenshots show a persistent right rail with:

- compact state cards
- "why this context"
- raw current state fields

This is valuable information, but it needs stronger mode separation so explanation does not always arrive bundled with low-level state inspection.

### Current settings problem

The screenshots show useful controls in `General`, `Integrations`, `Components`, and `Loops`, but the IA still treats policy, source participation, and observability as secondary details rather than a coherent control model.

### Current suggestions problem

The screenshots show strong evidence capture, but the default layout still foregrounds payload JSON and evidence blobs rather than the operator decision.

## Target surface contracts

### Now

The `Now` page should answer:

- what is urgent
- what is active
- what is at risk
- what action should happen next

It should emphasize:

- active commitments
- at-risk commitments
- suggested actions

It should stop acting as the default home for:

- raw debug fields
- broad source-health inspection
- verbose sync-control panels

### Context panel

The context panel should split into explicit modes:

- `State` — what Vel currently believes
- `Why` — why Vel believes it
- `Debug` — the raw source and runtime detail behind the belief

This is the main response to the screenshot pattern where explanation and debug are fused into one continuously visible rail.

### Suggestions

Suggestions should behave like a steering workspace:

- decision first
- evidence second
- raw payload only when explicitly inspected

The suggestion decision should remain legible without requiring the operator to parse JSON blocks.

### Threads

Threads should become process-oriented continuity surfaces.

Each thread should clearly encode:

- status
- type
- last activity

The screenshots make it clear this is still under-expressed in the current UI.

### Stats

Add a dedicated `Stats` tab as the canonical observability surface.

Sections:

- source health
- context formation
- system behavior
- loop performance

This is where much of the diagnostic weight currently placed on `Now` should move.

### Integrations

Integrations need an explicit policy model, not just connection cards.

Policy fields:

- `enabled`
- `sync`
- `visible`
- `contributes_to_context`
- `trust_level`

These controls should explain not just whether a source exists, but how it participates in the system.

### Metaball head

The metaball head remains optional visual/system-language work.

Phase 1:

- color
- pulse

Phase 2:

- deformation
- uncertainty

This should not outrank IA clarity.

## Priority order

### P0

- context panel refactor
- now cleanup
- stats tab

### P1

- integration policy surface
- threads upgrade
- inbox alignment

### P2

- metaball head
- attention tokens

## Parallelization lanes

### Lane A — Now / Context / Stats

Primary ownership:

- `Now` action hierarchy
- context mode split
- observability extraction

Suggested ticket group:

- UI-V4-001
- UI-V4-002
- UI-V4-003

### Lane B — Threads / Inbox / Suggestions IA

Primary ownership:

- continuity-oriented thread framing
- inbox role cleanup
- suggestion decision-first presentation

Suggested ticket group:

- UI-V4-005
- UI-V4-006
- UI-V4-009

### Lane C — Integration policy and settings

Primary ownership:

- source participation model
- settings information architecture
- policy semantics and controls

Suggested ticket group:

- UI-V4-004
- UI-V4-010

### Lane D — Visual language and polish

Primary ownership:

- metaball state signaling
- attention-token visual language

Suggested ticket group:

- UI-V4-007
- UI-V4-008

## Boundaries

- do not invent a second domain model separate from `vel-core`
- keep route handlers thin and preserve current service boundaries
- treat `Stats` as the observability home instead of continuing to overload `Now`
- reconcile UI terms with [docs/vocabulary.md](../vocabulary.md) where relevant
- do not let visual polish outrank actionability and explainability

## Suggested execution order

1. Split the context panel into `State`, `Why`, and `Debug`.
2. Reduce `Now` to an action-first surface.
3. Move runtime/source introspection into `Stats`.
4. Reframe threads and inbox around continuity and triage roles.
5. Add integration participation/policy controls.
6. Rework suggestions to foreground operator decisions over payload internals.
7. Add visual/system-language work only after the IA is stable.
