---
title: "Implement Message Composer"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 023-implement-thread-view
  - 015-implement-message-api
labels:
  - vel
  - chat-interface
---
Add the message composer.

## Features

- textarea
- send button
- enter to send
- shift+enter newline

## Acceptance Criteria

- submitting posts a message
- thread updates after send
- UX is stable under slow or duplicate responses

## Notes for Agent

Avoid over-abstracting the composer before the actual message flows settle.
