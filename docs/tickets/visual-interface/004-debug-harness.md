---
status: todo
owner: agent
priority: high
---

# 004 — Build debug harness

## Goal
Create a small app/page for driving the system interactively.

## Deliverables
- slider panel for affect dimensions
- mode selector
- preset selector
- event buttons
- packet preview
- morphology preview
- FPS display

## Instructions
1. Make this the primary tuning surface.
2. Add buttons for:
   - user speech start/stop
   - agent thinking start/stop
   - agent speaking start/stop
   - warn
   - overload
3. Show current canonical affect state JSON.
4. Show current sync packet JSON.
5. Show current morphology JSON.

## Acceptance criteria
- A designer can tune the system without touching code.
- The harness can drive the renderer live.
