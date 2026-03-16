---
title: Build task ranking engine
status: ready
owner: agent
priority: P0
area: vel-task-hud
---

# Goal
Score tasks for attention so HUD surfaces can remain sparse and useful.

## Scope
Create `vel-task-ranking` with:
- `compute_attention_score`
- `compute_decay_state`
- `compute_lateness_risk`
- helper scoring functions

## Inputs
Use only stable task/context inputs at first:
- urgency
- priority
- deadline proximity
- scheduled proximity
- snooze state
- dependency pressure
- decay / neglect
- explicit pinning
- risk engine fields if available

## Requirements
- Keep weights centralized and inspectable.
- Prefer deterministic pure functions.
- Return structured debug contributions if feasible so tuning is easier.

## Suggested output
```rust
pub struct AttentionScoreBreakdown {
    pub total: f32,
    pub urgency_weight: f32,
    pub deadline_weight: f32,
    pub decay_weight: f32,
    pub dependency_weight: f32,
    pub risk_weight: f32,
    pub negative_modifiers: f32,
}
```

## Tests
- near-deadline task outranks low-urgency backlog
- pinned task gets positive boost
- snoozed task gets suppressed
- stale high-priority task drifts upward
- blocked non-actionable task is deprioritized

## Done when
- ranking is deterministic
- test fixtures cover edge cases
- breakdowns can be surfaced in logs/devtools

