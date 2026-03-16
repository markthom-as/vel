---
title: Add ambient task HUD mode
status: ready
owner: agent
priority: P3
area: vel-task-hud
---

# Goal
Create a low-distraction representation of task pressure.

## Scope
Implement a minimal desktop ambient mode that:
- avoids full task list rendering
- shows pressure/risk state
- expands into richer UI on interaction

## Notes
This is where Vel can start feeling less like a todo app and more like a living cognitive prosthetic. But do not disappear into aesthetic fog. The user must still be able to read the situation.

## Requirements
- low motion
- low entropy
- clear expand path
- shared semantics with regular HUD

## Tests
- state mapping tests
- accessibility/readability checks where possible

## Done when
- ambient mode exists behind a feature flag
- mode consumes shared view model / policy inputs

