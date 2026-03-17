---
id: 103
title: context-selector-service
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# context-selector-service

## Summary

Build the rule-based selector that gathers and ranks context for a turn.

## Why this exists

Current chat generation passes only thread history and a static system prompt. The missing center of gravity is a service that decides what context belongs on the turn.

## Scope

- Create a server-side service that selects bounded context items.
- Support `auto`, `manual`, and `none` modes.
- Gather from current context, commitments, explain surfaces, risk, messaging pressure, and thread links.

## Deliverables

- `ContextSelector` service module.
- Ranked inclusion/exclusion reasons.
- Freshness/staleness warnings.
- Serialized packet builder.

## Implementation notes

- Add a new service module under `crates/veld/src/services/` for context-aware chat assembly.
- Start with explicit heuristics keyed off the user message text and current system state.
- Always include current context in `auto` when available.
- In `manual`, include selected refs first, then only minimal safety context (e.g. current context summary + stale warnings).
- In `none`, include no packet attachments beyond conversation history and an explicit marker in metadata.
- Add size caps so the packet remains bounded.

## Acceptance criteria

- Selector returns deterministic packets for fixed fixtures.
- Manual attachments override auto ranking.
- Stale integrations produce warnings.
- Packet size is capped and reported.

## Files likely touched

- `crates/veld/src/services/` (new module)
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/services/integrations.rs` (read freshness/status, do not mutate)
- relevant app tests

## Risks / gotchas

- Do not hit external APIs inside selection. This service must compose already-persisted state, not become a side-effect carnival.
