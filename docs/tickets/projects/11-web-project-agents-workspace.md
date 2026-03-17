---
title: Implement web Chats/Agents workspace for projects
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Ship the session/operator half of the Projects page.

## Scope
- active session list/cards
- queue message composer
- steering and feedback affordances
- per-session settings panel
- recent queued message / outbox states

## Requirements
- show at least source, title, status, mode, queue depth, last activity, latest summary
- queue composer can target a specific session
- steering/feedback are durable actions, not transient local UI only
- read-only adapters still support local outbox and visible delivery state

## Suggested components
```text
clients/web/src/components/projects/ProjectSessionsPane.tsx
clients/web/src/components/projects/AgentSessionCard.tsx
clients/web/src/components/projects/SessionQueueComposer.tsx
clients/web/src/components/projects/SessionFeedbackPanel.tsx
clients/web/src/components/projects/SessionSettingsPanel.tsx
```

## Tests
- render sessions from workspace data
- queue message action
- feedback action
- settings update reflected in UI after refetch/invalidation

## Done when
- project-linked active chats are visible and operable from web UI
