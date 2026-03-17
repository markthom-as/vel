# Vel Chat API (`/api`)

This document describes the chat API exposed by `veld` under `/api`.

For the full chat spec, see:

- `docs/specs/vel-chat-interface-implementation-brief.md`
- `docs/specs/vel-chat-execution-plan.md`

## Conversations

### `GET /api/conversations`
### `POST /api/conversations`
### `GET /api/conversations/:id`
### `PATCH /api/conversations/:id`

Conversations have:

- `id`
- `title`
- `kind`
- `pinned`
- `archived`
- timestamps

## Messages

### `GET /api/conversations/:id/messages`

- list messages in a conversation (supports `limit`).

### `POST /api/conversations/:id/messages`

Request:

- `MessageCreateRequest` (`role`, `kind`, `content`).

Response:

- `CreateMessageResponse`:
  - `user_message: MessageData`
  - `assistant_message?: MessageData | null`
  - `assistant_error?: string | null`

## Inbox and interventions

### `GET /api/inbox`

- returns items of `InboxItemData` (id, message_id, kind, state, surfaced_at, snoozed_until, confidence).

### `POST /api/interventions/:id/actions/:action_id`

- invoke inline actions on interventions (e.g. snooze/resolve/dismiss).

## Settings

### `GET /api/settings`
### `PATCH /api/settings`

- read and update chat UI settings (e.g. speech toggle).

## WebSocket

### `GET /ws`

WebSocket events include (see `vel-chat-execution-plan` for full list):

- `messages:new`
- `interventions:new`
- `interventions:updated`
- `context:updated`
- `runs:updated`
- `components:updated`
