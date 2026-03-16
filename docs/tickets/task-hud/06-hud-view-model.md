---
title: Implement task HUD view model
status: ready
owner: agent
priority: P0
area: vel-task-hud
---

# Goal
Create a shared view-model layer that turns raw tasks into surface-ready HUD groups.

## Scope
Create `vel-task-hud` with functions:
- `get_now_tasks`
- `get_soon_tasks`
- `get_waiting_tasks`
- `get_drifting_tasks`
- `get_ritual_tasks`
- `get_threat_tasks`
- `build_glance_payload`

## Requirements
- Keep UI rendering out of this crate.
- The output should be stable enough for desktop, mobile, watch, and AR consumers.
- Clamp group sizes per surface.

## Suggested structs
```rust
pub struct HudTaskItem {
    pub id: TaskId,
    pub title: String,
    pub subtitle: Option<String>,
    pub score: f32,
    pub risk_badge: Option<String>,
    pub quick_actions: Vec<QuickAction>,
}
```

## Tests
- grouping tests
- compact clamping tests
- threat selection tests
- watch/mobile glance payload tests

## Done when
- view model crate exists
- multiple surfaces can consume the same grouped output

