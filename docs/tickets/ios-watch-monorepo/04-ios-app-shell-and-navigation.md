---
id: APPLE-004
title: Build iOS app shell and primary navigation
status: proposed
owner: agent
priority: p1
area: apps/ios
depends_on: [APPLE-003]
---

# Goal

Create the iOS shell that exposes the first useful Vel mobile surface: today's obligations, current risk posture, and quick completion flows.

# Screens for MVP

- Today
- Reminders / medications
- Suggestions / nudges
- Activity / recent events
- Settings / debug sync

# Requirements

- SwiftUI app target under `Apps/VeliOS`
- Shared design tokens / components via `VelAppleUI`
- Root navigation with deterministic state restoration
- Debug panel showing:
  - last sync time
  - pending mutations
  - current notification auth state
  - fixture/demo mode toggle if needed

# UX notes

Bias toward:

- low-friction completion
- strong temporal clarity
- legible urgency levels
- minimal ornamental chrome

This is not a journaling app pretending to be a life raft. It should feel fast, calm, and slightly severe.

# Acceptance criteria

- app launches into Today screen
- navigation works across all MVP tabs/screens
- screen state is backed by shared observable models
- debug info is available without contaminating main UX
