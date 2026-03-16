---
id: TKT-005
status: proposed
title: Implement quick check-in, completion, and reason capture flows
priority: P1
estimate: 3-4 days
depends_on: [TKT-004]
owner: agent
---

## Goal

Support extremely low-friction interaction loops for marking things done, skipped, snoozed, or blocked.

## Scope

- Inline completion for routine/task items
- Optional follow-up reason capture when skipping or deferring
- “Done earlier” action for late logging
- Blocker tagging:
  - forgot
  - unavailable
  - low energy
  - in meeting
  - intentionally skipped
  - custom note

## Implementation notes

- Reason capture must be optional in the happy path
- For ADHD ergonomics, optimize for one-thumb, one-screen actions
- Log enough structured data to feed later suggestion/risk analysis

## Acceptance criteria

- User can mark items done in <= 2 taps from Today
- Skips/deferrals can attach a structured blocker reason
- Events are persisted locally and queued for sync when offline
