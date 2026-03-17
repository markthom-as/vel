---
id: 102
title: context-packet-storage-and-provenance
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# context-packet-storage-and-provenance

## Summary

Persist per-turn context packets and link them to messages and attached entities.

## Why this exists

Without persistence, “attached context” becomes ephemeral prompt goo and cannot be inspected, explained, or debugged later.

## Scope

- Add storage representation for context packets.
- Link user/assistant messages to the packet.
- Link packet to the entities it includes.

## Deliverables

- Migration for either `context_packets` table or artifact-backed packet persistence.
- Storage CRUD helpers.
- Ref-linking helpers using existing `refs` relations.
- Metadata schema version for packets.

## Implementation notes

- Preferred approach: persist packets as artifacts with `artifact_type = context_packet` and JSON payload in file or storage URI, then link using `refs`.
- Link `user_message -> packet` with `attached_to`.
- Link `assistant_message -> packet` with `derived_from`.
- Link `packet -> current_context/commitment/thread/...` with `attached_to`.
- Store enough metadata to inspect mode, warnings, counts, and prompt-builder version.

## Acceptance criteria

- After a chat send, a packet record exists.
- The packet can be located from either the user or assistant message.
- Included refs are queryable and stable.
- Packet payload survives restarts.

## Files likely touched

- `crates/vel-storage/src/db.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/vel-core/src/provenance.rs` (only if new relation helpers are needed)
- migrations directory

## Risks / gotchas

- Avoid inventing a second provenance system when `refs` already exists. Reuse the sharp tool already on the bench.
