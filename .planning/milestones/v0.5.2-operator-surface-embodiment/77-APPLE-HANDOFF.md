# Phase 77 Apple Handoff

## Purpose

This packet hands the embodied `v0.5.2` web surface model to Apple without pulling Apple implementation into the milestone.

Apple parity after `v0.5.2` means the same doctrine with platform-native rendering. It does **not** mean pixel parity with web.

## Governing Docs

Apple work should treat these as the active authorities:

- [0.5.1-truthful-surface-doctrine.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.1-truthful-surface-doctrine.md)
- [0.5.2-operator-surface-doctrine.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.2-operator-surface-doctrine.md)
- [72-UI-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/72-UI-SPEC.md)

## Surface Model

Apple should still implement exactly three first-class surfaces:

- `Now`
- `Threads`
- `System`

Still out:

- no standalone `Inbox`
- no standalone `Settings`
- no workflow builder
- no client-side schema negotiation

## Canonical Source List Per Surface

### `Now`

Canonical reads:

- `GET /v1/now`
- `GET /v1/explain/context`
- `GET /v1/explain/drift`

Canonical mutation:

- `PATCH /v1/commitments/:id`

Embodied posture:

- singular dominant `Focus`
- adjacent `Commitments`
- today-first `Calendar`
- subordinate `Triage`
- always-available but visually subordinate capture

### `Threads`

Canonical reads:

- `GET /api/conversations`
- `GET /api/conversations/:id/messages`

Canonical mutation still in scope:

- `POST /api/assistant/entry`

Embodied posture:

- bound object state first
- chronology second
- compressed inline provenance
- deep provenance in dedicated expansion
- no floating or multi-object invocation

### `System`

Intentional bounded reads:

- `GET /v1/agent/inspect`
- `GET /api/integrations`
- `GET /api/integrations/connections`

Embodied posture:

- one `/system` surface
- visible grouped navigation:
  - `Domain`
  - `Capabilities`
  - `Configuration`
- one active detail pane at a time
- split-pane continuity on large layouts; stacked drill-in on narrow layouts

## Mutation / Action Rules

Apple must preserve the frozen posture:

- semantic truth is backend-owned
- mutations reconcile from backend responses
- no client-side semantic derivation
- no optimistic semantic truth
- no composite or inferred configuration actions

`System` allow-listed actions remain:

- `Integrations`: `connect`, `disconnect`, `refresh` only when canonical
- `Modules`: `activate`, `deactivate` only when canonical
- `Accounts`: `reauthorize`, `disconnect` only when canonical
- `Scopes`: `enable`, `disable` only when canonical

The current embodied web line visibly uses only the named canonical actions already exposed by backend truth.

## Degraded-State / Drift Handling

Apple must keep the same honesty posture:

- last-known-good data may appear only when labeled stale and provenance-safe
- otherwise show explicit degraded state
- no silent fallback to guessed or legacy behavior
- raw backend detail belongs in logs/diagnostics, not operator UI

## Parity Expectations

Apple should match:

- doctrine
- canonical source boundaries
- mutation discipline
- degraded-state honesty
- surface hierarchy

Apple should not copy:

- exact web layout
- typography
- panel geometry
- desktop split proportions

## Non-Goals

- no Apple implementation in `v0.5.2`
- no pixel-parity spec
- no new backend surface widening for platform convenience
