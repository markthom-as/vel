---
status: todo
owner: agent
priority: high
---

# 002 — Build `vel-visual-morphology`

## Goal
Map affect state to facial/body/material parameters.

## Deliverables
- `types.ts`
- `tokens.ts`
- `mapStateToMorphology.ts`
- tests

## Instructions
1. Use the provided morphology shape as a starting point.
2. Encode:
   - head scale
   - tilt
   - primary mass
   - satellite count/scale
   - brow
   - nose bridge
   - mouth seam
   - jaw/cheek tension
   - noise/glow/color
3. Keep mappings legible and monotonic where practical.
4. Treat faciality as emergence strength, not a boolean.
5. Avoid any eye-related parameters.

## Acceptance criteria
- State changes produce plausible morphology shifts.
- High fragmentation increases multiplicity.
- High speaking increases mouth activation.
- High warning increases compression/tension.
