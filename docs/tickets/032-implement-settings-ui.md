---
title: "Implement Settings UI"
status: todo
owner: agent
type: implementation
priority: medium
created: 2026-03-15
depends_on:
  - 031-implement-settings-api
  - 021-build-app-shell
labels:
  - vel
  - chat-interface
---
Build the settings page.

## Route

- `/settings`

## Controls

- quiet hours
- notification toggles
- proactive intervention toggles

## Acceptance Criteria

- page loads from API
- edits persist
- state reflects saved values after reload

## Notes for Agent

Clarity beats sophistication here. The settings page is where users recover agency.
