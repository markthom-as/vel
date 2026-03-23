# Phase 71 Apple Handoff

## Purpose

This packet hands the frozen `v0.5.1` web reconnection doctrine to Apple without pulling Apple implementation into this milestone.

Apple parity in the next milestone must conform to backend doctrine with platform-appropriate rendering. It does **not** require pixel parity with web.

## Surface Model

Apple should align to the same three-surface product model:

- `Now` — temporal and operational truth
- `Threads` — contextual conversation and bounded interaction
- `System` — structural object, capability, and configuration truth

Explicitly out:

- no standalone `Inbox`
- no standalone `Settings`
- no `Projects` surface resurrection
- no workflow builder or simulation layer

## Truthful Surface Doctrine

Apple must follow [0.5.1-truthful-surface-doctrine.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.1-truthful-surface-doctrine.md).

Minimum rules:

- render canonical read models only
- all semantic mutations go through backend-owned canonical actions
- no client-side semantic derivation
- no optimistic semantic truth
- degraded state must be explicit
- no silent fallback to stale or guessed behavior

## Canonical Route And Contract Map

### Now

Canonical sources:

- `GET /v1/now`
- supporting explain reads already used by the shell and context stack:
  - `GET /v1/explain/context`
  - `GET /v1/explain/drift`

Canonical mutations:

- `PATCH /v1/commitments/:id`

Current shipped mutation scope:

- completion is proven
- reopen / defer / due-date movement must remain backend-owned and only surface when the canonical contract is already stable

Surface rules:

- tasks and calendar commitments render as adjacent canonical sections
- no merged cross-type ranking feed
- no client-side prioritization or urgency inference

### Threads

Canonical sources:

- `GET /api/conversations`
- `GET /api/conversations/:id/messages`

Canonical mutation:

- `POST /api/assistant/entry`

Surface rules:

- invocation is object-scoped only when exactly one bound canonical object exists
- zero-bound-object threads may guide the user to attach/create an object first
- no floating or multi-object workflow invocation
- no workflow-builder behavior

Current shipped truth:

- read and gating are proven
- no live workflow invocation route is exposed yet through the shipped client/backend surface

### System

Intentional bounded reads:

- `GET /v1/agent/inspect`
- `GET /api/integrations`
- `GET /api/integrations/connections`

Surface rules:

- one top-level `/system` surface with internal sections
- read-only browsing by default
- no structural mutation UI beyond explicit allow-listed canonical actions

Shipped section set:

- `Domain`
  - `People`
  - `Calendar`
  - `Knowledge`
- `Capabilities`
  - `Tools`
  - `Workflows`
  - `Templates`
- `Configuration`
  - `Modules`
  - `Integrations`
  - `Accounts`
  - `Scopes`

## Mutation And WriteIntent Rules

Apple must mirror these boundaries:

- semantic truth is backend-owned
- local drafts, selection state, and loading state may be ephemeral
- semantic state changes must reconcile from backend responses
- no client-side “smart triage,” inferred workflow orchestration, or capability interpretation

`Now` mutations:

- allowed only when the backend already exposes the canonical action
- must reconcile from backend-owned refreshed truth

`System` actions:

- only explicitly named allow-listed actions may render
- no composite actions
- no inferred actions from state

## Allow-Listed Configuration Actions

Frozen `v0.5.1` allow-list:

- `Modules`
  - `activate`
  - `deactivate`
- `Integrations`
  - `connect`
  - `disconnect`
  - `refresh` only if already canonical in backend
- `Accounts`
  - `reauthorize`
  - `disconnect`
- `Scopes`
  - `enable`
  - `disable` only if already canonical in backend

Apple should not invent any action outside this list.

## Degraded-State And Drift Handling

Required posture:

- show last-known-good data only when it is explicitly canonical, stable, and labeled stale
- otherwise render an explicit degraded or failure state
- raw backend details may live in logs or diagnostics, not operator-facing UI

Apple must not silently fall back to:

- legacy DTOs
- guessed defaults
- stale structural state without labeling
- simulated workflow capabilities

## Per-Surface Parity Expectations

Apple parity means:

- same backend doctrine
- same canonical source boundaries
- same mutation rules
- same degraded-state honesty

Apple parity does **not** mean:

- identical layout
- identical typography
- identical navigation chrome

## Non-Goals

- no Apple implementation in `v0.5.1`
- no pixel-parity spec
- no workflow builder
- no client-side schema negotiation
- no backend widening just to satisfy a platform preference

## Follow-On Expectation

The first Apple implementation milestone should reuse the canonical transport and truthful-surface rules from `v0.5.1`, not reinterpret them.
