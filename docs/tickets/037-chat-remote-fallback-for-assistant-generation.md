---
title: "Chat remote fallback and automatic failover for assistant generation"
status: done
owner: agent
type: implementation
priority: high
created: 2026-03-16
depends_on:
  - 034-add-backend-tests
labels:
  - vel
  - chat-interface
  - llm
  - fallback
---
This ticket is already implemented in the current repo.

## Shipped behavior

- `configs/models/routing.toml` may define `fallback_remote` as an optional remote overflow profile.
- If `chat` is missing but `fallback_remote` is configured, chat generation uses the fallback profile directly.
- If both `chat` and `fallback_remote` are configured and differ, assistant generation tries the primary profile first and retries with the fallback profile only for retryable failures:
  - missing provider registration
  - transport errors
  - protocol errors
  - backend errors
- Configuration errors do not trigger fallback.
- Fallback is skipped when it resolves to the same profile id as the primary.
- `POST /api/conversations/:id/messages` still returns the normal `CreateMessageResponse` shape with `user_message`, optional `assistant_message`, and optional `assistant_error`.
- Existing message persistence and websocket broadcast behavior remain unchanged.

## Landed in code

- chat router build + fallback profile resolution:
  - `crates/veld/src/llm.rs`
- assistant generation retry/fallback path:
  - `crates/veld/src/routes/chat.rs`
- optional localhost-only remote provider wrapper:
  - `crates/vel-llm/src/providers/openai_oauth.rs`
- routing/profile docs:
  - `configs/models/README.md`

## Notes

- The current implementation keeps failover narrow to chat assistant generation. It does not introduce generic per-task failover semantics across the repo.
- The remote overflow profile remains opt-in and localhost-scoped via the `openai_oauth` provider gating.
- If later work adds richer provider health selection, backoff, or cross-task routing policy, that should land as a new ticket rather than pretending this slice is still open.
