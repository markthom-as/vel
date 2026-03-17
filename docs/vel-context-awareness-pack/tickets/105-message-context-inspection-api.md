---
id: 105
title: message-context-inspection-api
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# message-context-inspection-api

## Summary

Expose a read API to inspect the context packet used for a message.

## Why this exists

The UI cannot truthfully show “used context” unless the server exposes a canonical read surface for the packet and its linked refs.

## Scope

- Add a message-level inspection route.
- Hydrate packet payload, linked refs, and warnings.
- Keep it read-only.

## Deliverables

- `GET /api/messages/:id/context` route.
- DTOs for hydrated packet inspection.
- Server-side lookups from message -> packet -> attached refs.

## Implementation notes

- Resolve packet linked to the assistant message if present; fall back to the user message packet when necessary.
- Return packet metadata, summary, warnings, and attached refs in stable order.
- Include whether refs were auto-selected or manually requested if that data is persisted.

## Acceptance criteria

- Route returns 404 or null cleanly when no packet exists.
- Route returns packet details for grounded messages.
- Response shape is consumable by the existing UI decoding layer.

## Files likely touched

- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/routes/chat.rs` or sibling route module
- `clients/web/src/types.ts`
- tests

## Risks / gotchas

- Avoid conflating this with broader provenance/explain APIs. This route is per-message context inspection, not a dumping ground for every explain surface.
