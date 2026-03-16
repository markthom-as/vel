---
title: Vel Apple Platform Tickets
status: proposed
created: 2026-03-15
owner: agent
---

# Vel Apple Platform Tickets

This package contains implementation tickets for the iOS + watchOS surface area of **Vel**.

## Recommended architecture

Use a **separate Apple repo/workspace** (`vel-apple`) that consumes Vel through a stable boundary rather than trying to force the Rust server, CLI, and Apple UI code into one repo too early.

Suggested shape:

- `apps/ios` — iPhone app target
- `apps/watch` — watchOS app target
- `packages/VelAppCore` — Swift domain/state layer
- `packages/VelSync` — sync client, auth, background refresh
- `packages/VelNotifications` — local notifications + scheduling wrappers
- `packages/VelWidgets` — widgets / Live Activities / complications glue
- `packages/VelSharedModels` — DTOs mirrored from Vel core API

## Product assumptions encoded in these tickets

- The Apple clients are **capture + awareness surfaces**, not the sole source of truth.
- Core planning/risk/suggestion logic should live in Vel core/backend wherever possible.
- Apple clients should support **fast acknowledgement**, **med logging**, **meeting-aware reminders**, and **ambient glanceability**.
- watchOS should optimize for **sub-10 second interactions**; anything deeper belongs on iPhone.

## Ticket order

1. Platform architecture + repo bootstrap
2. Shared models + sync contract
3. App shell + navigation
4. Timeline / Today surface
5. Check-in + completion flows
6. Med adherence + meeting-aware reminders
7. Notification actions + background refresh
8. watchOS quick actions
9. Complications / widgets / Live Activities
10. Voice capture pipeline
11. Offline-first sync + conflict handling
12. Privacy / permissions / observability hardening
