---
id: TKT-012
status: proposed
title: Harden privacy, platform permissions, telemetry, and QA instrumentation
priority: P1
estimate: 3-4 days
depends_on: [TKT-003, TKT-006, TKT-007, TKT-010, TKT-011]
owner: agent
---

## Goal

Close the loop on platform reality: permissions, debugging, privacy boundaries, and quality instrumentation.

## Scope

Permissions flows for:

- notifications
- microphone / speech recognition
- calendar access, if used locally
- background refresh expectations

Observability:

- structured client logs
- sync health screen
- notification scheduling audit screen
- last action / last refresh / pending queue counts
- redaction rules for sensitive fields

QA:

- test matrix for iPhone sizes and watch sizes
- manual scenarios for offline, timezone changes, DST, and reboot
- smoke tests for notification categories and widget rendering

## Acceptance criteria

- App explains why permissions are requested in-context
- Sensitive user data is redacted from logs by default
- Debug screen makes it possible to diagnose sync/notification failures without divination
