---
status: todo
owner: agent
priority: high
---

# 001 — Build `vel-affect-core`

## Goal
Create the canonical affect-state package.

## Deliverables
- `types.ts`
- `baseline.ts`
- `presets.ts`
- `events.ts`
- `reducer.ts`
- `smoothing.ts`
- package exports

## Instructions
1. Keep the state renderer-agnostic.
2. Normalize all numeric dimensions.
3. Add utility clamps.
4. Add at least these presets:
   - idle
   - listening
   - thinking
   - speaking
   - warning
   - overloaded
   - sleeping
5. Add reducer cases for speech, thinking, warning, overload, and idle decay.
6. Write tests for:
   - clamping
   - event transitions
   - preset validity

## Acceptance criteria
- No renderer imports.
- No categorical emoji-emotion core model.
- Type exports compile cleanly.
