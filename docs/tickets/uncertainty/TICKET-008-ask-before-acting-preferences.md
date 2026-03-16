---
title: Add ask-before-acting preference modes
status: todo
priority: P1
owner: product
labels: [uncertainty, preferences, ux]
---

# Goal

Let the user tune how aggressively Vel interrupts for clarification.

# Deliverables

- preference schema for modes:
  - `hands_off`
  - `balanced`
  - `high_clarity`
  - `delegate_to_agents_first`
  - `ask_before_destructive_actions_only`
- settings UI / config binding
- policy engine integration

# Requirements

- Modes change thresholds, not hard safety rules.
- Current mode should be visible in UI.
- Preference changes should take effect without restart if runtime architecture permits.

# Acceptance criteria

- Policy tests verify threshold changes by mode.
- UI/state tests verify mode persistence.
- Default mode is documented and justified.

# Notes

This is product design disguised as configuration. Treat it accordingly.
