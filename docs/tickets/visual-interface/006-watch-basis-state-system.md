---
status: todo
owner: agent
priority: high
---

# 006 — Implement watch basis-state system

## Goal
Build the watch embodiment using authored basis states and interpolation.

## Deliverables
- basis-state definitions
- nearest-state selection
- weighted blending
- local idle fallback
- event-cue transitions

## Instructions
1. Do not port the desktop renderer directly.
2. Implement at least 8 basis states.
3. Blend nearest 2–3 states.
4. Add low-cost live modifiers:
   - pulse
   - hue
   - asymmetry
   - faciality gain
5. Preserve silhouette and seam readability.

## Acceptance criteria
- Watch feels like Vel, not a separate character.
- Works without continuous phone frame streaming.
- Disconnect fallback degrades gracefully.
