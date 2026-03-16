---
title: Context Inference Engine
status: open
---

## Boundary

This ticket must refine the existing context reducer/runtime described in `docs/status.md`; it must not introduce a second independently authoritative inference engine.

- Present-tense context authority remains `current_context`, `context_timeline`, and the explain routes.
- New inference logic should feed that runtime or its explainability surfaces.
- If intermediate belief-like structures are introduced, they are supporting artifacts, not a replacement state model.

# Goal

Improve the existing context inference path so it produces better derived state, uncertainty metadata, and explainability from signals.

# Tasks

1. Extend the current rule-based reducer/inference path rather than creating a parallel engine.
2. Integrate signals from:

- calendar
- device activity
- location
- prior tasks

3. Assign confidence or uncertainty metadata where it improves inspection or explanation.
4. Persist any supporting derived entries only if they clearly attach to the current context/explain runtime.

# Acceptance

- The existing current-context runtime remains the sole present-tense authority.
- Inference improvements are explainable through existing or extended explain surfaces.
- Any derived entries with confidence values are supporting metadata, not a competing truth model.
