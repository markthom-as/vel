---
id: 101
title: context-packet-domain-and-api
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# context-packet-domain-and-api

## Summary

Introduce first-class request/response/domain types for attached-context chat turns.

## Why this exists

The current `MessageCreateRequest` only carries role/kind/content, so there is no contract for auto/manual/none context modes or operator-selected refs.

## Scope

- Add API/domain types for attached-context mode and attachment refs.
- Extend chat request/response DTOs.
- Keep changes backward-compatible by defaulting missing mode to `auto` and missing attachments to `[]`.

## Deliverables

- `AttachedContextMode` enum (`auto`, `manual`, `none`).
- `AttachedRefRequest { ref_type, ref_id }`.
- `ContextPacketSummaryData`.
- `CreateMessageResponse.context_packet`.
- Runtime decoders on the web side.

## Implementation notes

- Update `crates/vel-api-types/src/lib.rs`.
- Update `clients/web/src/types.ts` and decoders/tests.
- Make sure existing callers that do not send new fields still compile and behave the same.
- Default semantics belong server-side too, not only in the client.

## Acceptance criteria

- Existing chat flows still work with no client changes.
- New request fields are accepted and decoded.
- New response fields round-trip through API types and TS decoders.
- Tests cover missing-field defaults.

## Files likely touched

- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/types.ts`
- `clients/web/src/types.test.ts`
- `clients/web/src/components/MessageComposer.test.tsx`

## Risks / gotchas

- Do not create a DTO that implies attachments were *used* when they were merely *requested*. Keep requested-vs-actual distinct.
