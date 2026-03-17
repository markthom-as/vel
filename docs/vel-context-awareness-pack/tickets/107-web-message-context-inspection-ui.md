---
id: 107
title: web-message-context-inspection-ui
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# web-message-context-inspection-ui

## Summary

Let the operator inspect the actual context used for an assistant message.

## Why this exists

Without UI inspection, attached context is invisible magic. Invisible magic is how trust dies in enterprise software and personal assistants alike.

## Scope

- Show a “Used context” affordance on assistant messages.
- Add a drawer or panel for packet inspection.
- Allow pinning useful items into the next message where practical.

## Deliverables

- Message-level used-context pill/button.
- Packet inspection drawer.
- Hydrated list of attached refs, warnings, and packet summary.

## Implementation notes

- Reuse patterns from `ProvenanceDrawer` rather than inventing a second aesthetic language.
- Add a lightweight query/resource loader for the new API route.
- Show stale warnings prominently.
- Make packet mode visible so the operator can distinguish `auto` from `manual` and `none`.

## Acceptance criteria

- Clicking an assistant message context affordance opens packet details.
- Warnings and attached refs are legible.
- Empty-state messaging is graceful when no packet exists.
- Tests cover open/close and loading/error states.

## Files likely touched

- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/ProvenanceDrawer.tsx` or sibling component
- `clients/web/src/data/resources.ts`
- `clients/web/src/components/*test.tsx`

## Risks / gotchas

- Keep provenance and context inspection adjacent but distinct. One explains *how a message exists*; the other explains *what context the model had on the turn*.
