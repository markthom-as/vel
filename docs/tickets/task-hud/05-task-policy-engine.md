---
title: Create HUD policy engine
status: ready
owner: agent
priority: P0
area: vel-task-hud
---

# Goal
Decide what belongs on each HUD surface.

## Scope
Create `vel-task-policy` with:
- `should_show_on_full_panel`
- `should_show_on_compact_hud`
- `should_show_on_ambient`
- `should_notify`
- `should_escalate`

## Requirements
- Policy must be explainable.
- Separate policy from ranking.
- Surface-specific rules should be explicit.

## Example rules
Compact HUD:
- show top actionable tasks
- allow one blocked item only if user-actionable
- always allow pinned tasks unless completed or hidden

Ambient:
- no dense checklist rendering
- suppress low-signal backlog
- expose pressure/risk rather than exhaustive list

Notifications:
- respect snooze
- cap repeat nudges
- escalate only on changed risk or missed window

## Tests
- completed tasks suppressed everywhere
- snoozed tasks suppressed from notifications
- ritual due task can appear even if not high priority
- pinned tasks survive some ranking thresholds

## Done when
- policy functions are pure and test-covered
- rules are documented in code

