---
title: Realign Inbox, Threads, and Suggestions to distinct operator roles
status: todo
owner: agent
priority: P1
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-001-shell-ia-and-route-ownership.md
  - WUI-003-realtime-and-mutation-reconciliation.md
labels:
  - web
  - inbox
  - threads
  - suggestions
---

# Goal

Stop overlap between triage, continuity, and steering surfaces.

## Scope

- inbox action framing
- threads continuity framing
- suggestions decision-first layout

## Requirements

1. `Inbox` focuses on things that need acknowledgement or action now.
2. `Threads` focuses on continuity state, not only message chronology.
3. `Suggestions` leads with the decision and only then shows evidence/payload detail.
4. Cross-links between these surfaces should hand off cleanly without duplicating their core jobs.

## Write scope

- inbox page/components
- thread list and thread detail framing
- suggestions page/detail layout

## Acceptance criteria

- a user can tell why each of the three surfaces exists
- suggestion decisions are legible without parsing raw JSON
- thread surfaces read as continuity/process tools rather than generic chat history
