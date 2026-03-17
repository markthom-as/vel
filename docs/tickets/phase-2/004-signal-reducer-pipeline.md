---
title: Pluggable Signal Reducer Pipeline
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
labels:
  - veld
  - inference
  - modularity
---

Replace the monolithic `inference.rs` with a pluggable signal reducer pipeline where each domain (Git, Health, etc.) defines its own isolated reduction logic.

## Technical Details
- **SignalReducer Trait**: Define `trait SignalReducer { fn apply(&self, context: &mut CurrentContext, signal: &Signal); }`.
- **Registry**: Create a registry in `veld` to register active reducers at startup.
- **Engine Refactor**: Update `InferenceEngine` to iterate through all registered reducers for each signal being processed.
- **Domain Modules**: Extract existing inference logic into `GitReducer`, `CalendarReducer`, `HealthReducer`, etc.
- **Deterministic Replay**: Reducers must remain deterministic under replay and avoid hidden ambient state.
- **Composable Registration**: Reducers should be pluggable through explicit registration and capability declarations, not giant match ladders.

## Acceptance Criteria
- `inference.rs` is reduced to a generic reduction loop.
- Each domain's logic lives in a separate, testable reducer file.
- New integration types can be added without modifying the core engine.
- Reducer execution order and replay behavior are explicit and testable.
- All inference-related tests pass.
