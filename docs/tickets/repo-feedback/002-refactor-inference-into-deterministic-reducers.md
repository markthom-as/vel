---
title: "Refactor inference into deterministic reducers and explicit selectors"
status: todo
owner: agent
type: implementation
priority: critical
created: 2026-03-15
depends_on:
  - 001-enforce-evaluate-read-boundary.md
labels:
  - vel
  - inference
  - backend
  - maintainability
---
`services/inference.rs` is doing real work now, which is progress, but it is also becoming a sovereign state. The file is carrying too many responsibilities and is on the verge of becoming sacred mud.

## Why this matters

Inference is the semantic center of Vel. If it becomes too monolithic, every future behavior change becomes risky because:

- state derivation is hard to reason about
- ordering bugs become invisible
- product behavior hardens around incidental implementation details
- tests become coarse and low-signal

Right now the file is still salvageable. Later it becomes archaeology.

## Concrete concerns

The current inference flow appears to mix:

- signal gathering
- commitment gathering
- meds-state derivation
- temporal window derivation
- next-event selection
- next-commitment selection
- drift/attention derivation
- risk fallback logic
- current-context JSON assembly
- persistence
- event emission

That is too much for one unit of code.

## Required outcome

Split inference into pure-ish internal reducers/selectors plus a thin orchestration layer.

Suggested internal structure:

- `collect_inputs(...)`
- `derive_meds_status(...)`
- `select_next_event(...)`
- `select_next_commitment(...)`
- `derive_temporal_windows(...)`
- `derive_attention_state(...)`
- `derive_global_risk_summary(...)`
- `build_current_context(...)`
- `persist_inference_outputs(...)`

## Specific behavior changes required

### Next event selection
Do **not** choose the earliest event of the day. Choose the next relevant future event, with sane fallback when all events are in the past.

### Next commitment selection
Do **not** rely on `open_commitments.first()` semantics. Introduce an explicit ranking policy, at minimum considering:

1. due timestamp proximity
2. calendar-linked or externally anchored commitments
3. blocking dependencies / prerequisites
4. highest-risk open commitments
5. recent user activity / context relevance

### Policy defaults
Remove ad hoc default policy numbers from inference logic when those values should come from config or a dedicated fallback contract.

## Tasks

- Break `services/inference.rs` into private helper functions or submodules.
- Make selectors deterministic and individually unit-testable.
- Add narrow tests for:
  - next future event selection
  - no-future-event fallback
  - next commitment ranking
  - meds pending / meds done today behavior
  - commute/prep window activation
  - attention/drift classification
- Keep orchestration readable: gather -> derive -> persist.

## Acceptance Criteria

- `services/inference.rs` no longer reads like a bag of side effects with a philosophy degree.
- Next-event and next-commitment selection are explicit, deterministic, and tested.
- Pure derivation logic can be tested without full app/router setup.
- Policy values come from config or one well-defined default source, not scattered literals.

## Notes for Agent

This is classic "pay now or pay with blood later." Prefer boring, explicit helpers over clever compression.
