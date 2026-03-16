---
title: "Rich Chat Interactions and Markdown Rendering"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-16
depends_on:
  - 023-implement-thread-view
  - 024-implement-message-composer
  - 025-implement-card-renderer
labels:
  - vel
  - chat-interface
---
Add rich content and interaction support to Vel chat so that the interface can render markdown, code blocks, feedback actions, and any other interaction type expressed in chat messages.

## Scope

- Markdown rendering (headings, lists, links, inline code, emphasis, blockquotes).
- Code blocks with language hints and syntax highlighting.
- Inline code and preformatted text for snippets and commands.
- Feedback actions (e.g. thumbs up/down, "Was this helpful?", "Retry", "Copy code").
- Extensible interaction surface for any new action types represented in message payloads.

## Features

- Parse message content into a structured, safe render tree that supports:
  - Markdown text (including inline and fenced code).
  - Structured blocks such as callouts or notes when present in message payloads.
  - One or more action rows attached to a message (buttons, links, or menus).
- Provide a typed renderer layer so new interaction types can be added without changing core thread/inbox wiring.
- Ensure actions round-trip to the backend (or local handler) with enough metadata to identify the originating message and interaction type.

## UX Considerations

- Code blocks should be visually distinct, monospace, and horizontally scrollable on overflow.
- Provide obvious affordances for:
  - Copying code blocks.
  - Re-running or retrying an associated action when supported.
  - Providing quick feedback on responses (e.g. thumbs up/down).
- Keep keyboard navigation and screen-reader behavior reasonable for all interaction types.

## Acceptance Criteria

- Messages that contain markdown render with:
  - Correct headings, lists, links, and emphasis.
  - Inline code and fenced code blocks rendered in monospace with preserved whitespace.
- Code blocks:
  - Respect language hints when present.
  - Are copyable with a single interaction.
  - Do not break layout on narrow viewports.
- Feedback actions:
  - Render consistently under or alongside the message they apply to.
  - Invoke the correct handler and propagate to the backend when applicable.
  - Can be extended with new action types without rewriting the base renderer.
- At least one example flow in the chat interface exercises:
  - A markdown-heavy message.
  - A message containing one or more code blocks.
  - A message with feedback actions wired through end-to-end.

## Notes for Agent

- Prefer a small, composable renderer with clear props over a monolithic message component.
- Be explicit about the contract between backend message payloads and frontend render/interaction components; document how new interaction types are introduced.

