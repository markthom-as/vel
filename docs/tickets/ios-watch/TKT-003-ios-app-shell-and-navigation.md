---
id: TKT-003
status: proposed
title: Build iOS app shell, auth bootstrap, and core navigation
priority: P1
estimate: 2-3 days
depends_on: [TKT-001, TKT-002]
owner: agent
---

## Goal

Create the iPhone app shell with sane navigation and app lifecycle wiring.

## Scope

Top-level tabs or sections:

- Today
- Inbox / Capture
- History
- Settings

Add:

- launch/bootstrap flow
- session restoration
- error state surfaces
- empty states
- global sync status indicator

## Implementation notes

- Keep nav structure shallow
- Today should be the default root
- Surface degraded/offline state explicitly; do not fake freshness
- Add preview fixtures for all major screens

## Acceptance criteria

- App launches into mock Today screen
- Navigation between top-level sections works
- Loading / empty / error / offline states exist for each major surface
