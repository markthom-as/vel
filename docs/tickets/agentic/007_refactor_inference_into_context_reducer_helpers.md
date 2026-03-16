# Ticket 007 — Refactor inference into explicit context-reducer helpers

## Goal

Reduce the cognitive load and architectural risk inside `crates/veld/src/services/inference.rs` by splitting the current monolithic reducer into named helper functions without changing behavior.

## Why now

The current repo already correctly centralizes a lot of state derivation in inference, which is good. But the file is becoming the kind of monarchy that eventually starts appointing itself pope.

The issue is not that inference owns too much product meaning. The issue is that too much of that meaning is encoded inline instead of behind crisp helper boundaries.

## Current starting point

`crates/veld/src/services/inference.rs` currently handles, among other things:

- day/time windows
- meds status
- next-event selection
- prep and commute windows
- attention/drift fields
- current context assembly
- timeline append logic
- persistence to `inferred_state` and `current_context`

## Deliverable

Refactor inference into internal helper functions or submodules, while preserving behavior.

Recommended helper surface:

- `derive_meds_status(...)`
- `select_next_event(...)`
- `derive_temporal_windows(...)`
- `derive_attention_state(...)`
- `select_next_commitment(...)`
- `build_current_context_json(...)`
- `is_material_context_change(...)`

## Implementation plan

### 1. Extract without redesigning behavior
Do not mix this ticket with new heuristics.
This is structure first.

### 2. Keep present-tense ownership clear
Inference should still own:
- present-state reduction
- current-context assembly
- material-change detection

### 3. Add or preserve focused tests
If helper extraction makes unit tests easier, add them.
At minimum, keep existing integration coverage green.

## Files likely touched

- `crates/veld/src/services/inference.rs`
- optionally `crates/veld/src/services/inference_helpers.rs` or similar
- maybe `crates/veld/src/services/mod.rs`
- tests in or around inference

## Tests

Use existing app-level tests as the behavior backstop.
Add focused helper tests where it helps, especially for:
- meds status
- event selection
- drift derivation

## Acceptance criteria

- inference becomes easier to read top-to-bottom
- behavior is preserved
- helper names make responsibility explicit
- no product logic is moved into the wrong subsystem

## Out of scope

- changing thresholds
- redesigning risk
- redesigning nudge policies
- adding new product features

## Suggested agent prompt

Implement Ticket 007.

Refactor the inference service into named helpers without changing behavior.
Preserve architectural ownership: inference remains the context reducer.
Keep the patch reviewable and supported by existing tests.
