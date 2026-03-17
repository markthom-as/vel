---
id: 104
title: chat-prompt-assembly-and-reply-service
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# chat-prompt-assembly-and-reply-service

## Summary

Refactor assistant generation so replies are assembled from a structured prompt plus context packet.

## Why this exists

The current reply path uses a single hard-coded system prompt and empty metadata. That is why “what is current status” can still yield generic wallpaper.

## Scope

- Introduce a prompt builder.
- Include context packet and conversation history in a stable order.
- Persist prompt assembly metadata used for the turn.

## Deliverables

- Chat reply service extracted from route handler.
- Prompt-builder versioning.
- LLM metadata populated with packet metadata instead of `{}`.
- Better default grounding instructions.

## Implementation notes

- Extract `generate_assistant_reply(...)` into a service that: load history -> build/select packet -> assemble prompt -> call router -> persist assistant message -> persist provenance.
- Keep route handlers thin.
- Structure prompt layers explicitly.
- Ensure the model is told not to invent unseen context and to name stale/missing sources when relevant.
- Keep output text-only for now; do not mix this ticket with tool calling.

## Acceptance criteria

- Replies are still generated end-to-end.
- LLM metadata includes packet id, mode, attachment count, warning count, and prompt-builder version.
- A status question produces a reply reflecting current context fixtures.
- Empty packet / none mode still works without panics.

## Files likely touched

- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/services/` (new chat reply service)
- `crates/vel-llm/src/types.rs` only if metadata typing needs extension
- app tests

## Risks / gotchas

- Do not let prompt assembly sprawl across route code, service code, and UI assumptions. One builder, one format, one place.
