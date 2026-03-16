---
id: APPLE-003
title: Implement local store and sync spine for Apple clients
status: proposed
owner: agent
priority: p0
area: packages/sync
depends_on: [APPLE-002]
---

# Goal

Build the Apple sync spine so iOS/watch are not ornamental skins but actual clients of Vel's evented system.

# Requirements

- Add `VelAppleSync` package
- Implement:
  - local persistence layer
  - pull/apply sync
  - optimistic local mutation queue
  - retry/backoff
  - conflict handling policy for MVP
  - sync cursor persistence
- Define one clear source of truth for local state
- Expose a high-level `SyncEngine` API usable by app, watch, and widgets

# Persistence

Use a pragmatic local persistence approach suitable for Apple platforms, e.g.:

- SwiftData if it stays tame
- or SQLite/GRDB if consistency with Vel event-log patterns matters more

Pick one, document why, and do not build a little ORM religion around it.

# MVP conflict policy

For initial implementation, choose an explicit policy such as:

- append-only event log for actions
- server adjudicates canonical state
- client reconciles on next pull
- local pending actions visibly marked until confirmed

# Acceptance criteria

- can bootstrap local state from fixtures
- can enqueue mutation from notification/watch
- can resync and reconcile server response
- offline actions survive app restart
- sync tests cover happy path + duplicate replay + conflict case
