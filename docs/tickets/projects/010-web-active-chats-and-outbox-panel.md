---
id: VEL-PROJ-010
title: Build active chats and outbox panels for project workspaces
status: proposed
priority: P1
estimate: 3-5 days
dependencies:
  - VEL-PROJ-006
  - VEL-PROJ-007
  - VEL-PROJ-008
labels:
  - web
  - chats
  - outbox
---

# Goal

Render project-linked chat sessions and queued outbound messages in the web Projects page.

# Scope

- Build active session cards showing:
  - source badge
  - title/summary
  - last activity
  - queue depth
  - status
  - capability-derived action set
- Build outbox panel showing queued/draft/sent/failed/manual-dispatch messages.
- Add message queue composer for a selected session.
- Show steering and feedback affordances inline or in drawers.

# Deliverables

- `ProjectActiveChatsPanel` component(s)
- `ProjectOutboxPanel` component(s)
- queue message composer UI
- wiring to session control APIs
- frontend tests for queue and manual-dispatch rendering

# Acceptance criteria

- Project page shows active sessions from both Vel-native and external sources.
- User can queue a message for a selected session.
- Queue state updates are visible in the outbox panel.
- Sources without direct dispatch show clear manual-dispatch state.
- Basic websocket refresh or query invalidation keeps the panels fresh.

# Notes for agent

A queued message without visible state is just a fancy way to lose work.
