---
title: "Build App Shell"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 020-initialize-react-client
labels:
  - vel
  - chat-interface
---
Build the main three-pane application layout.

## Layout

- Left: navigation
- Center: thread/inbox content
- Right: context/provenance

## Components

- `AppShell`
- `Sidebar`
- `MainPanel`
- `ContextPanel`

## Acceptance Criteria

- three-column layout renders
- layout is responsive enough for desktop use
- no major state logic is buried in dumb layout components

## Notes for Agent

This should feel more like a cockpit than a messenger clone.
