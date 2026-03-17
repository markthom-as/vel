# Vel Operator And Chat API (`/api`)

This document describes the currently mounted operator and chat API exposed by `veld` under `/api`, plus the matching WebSocket endpoint.

For repo-wide implementation truth, see [`../MASTER_PLAN.md`](../MASTER_PLAN.md). For route-level authority, inspect `crates/veld/src/app.rs`, `crates/veld/src/routes/chat.rs`, `crates/veld/src/routes/components.rs`, and `crates/veld/src/routes/integrations.rs`.

## Conversations

### `GET /api/conversations`
### `POST /api/conversations`
### `GET /api/conversations/:id`
### `PATCH /api/conversations/:id`

- list, create, inspect, and update conversations
- conversation records include identifiers, title, kind, pinned/archive state, and timestamps

## Messages and interventions

### `GET /api/conversations/:id/messages`
### `POST /api/conversations/:id/messages`

- list and create messages in a conversation
- message creation uses `MessageCreateRequest`
- responses use `CreateMessageResponse`, including the persisted user message and optional assistant reply or assistant error

### `GET /api/conversations/:id/interventions`
### `GET /api/messages/:id/interventions`
### `GET /api/messages/:id/provenance`

- inspect conversation-level interventions, message-level interventions, and message provenance

### `GET /api/inbox`

- operator inbox for surfaced interventions and related review items

### `POST /api/interventions/:id/snooze`
### `POST /api/interventions/:id/resolve`
### `POST /api/interventions/:id/dismiss`

- explicit operator actions for intervention lifecycle changes

## Settings

### `GET /api/settings`
### `PATCH /api/settings`

- read and update chat/operator UI settings

## Components

### `GET /api/components`
### `GET /api/components/:id/logs`
### `POST /api/components/:id/restart`

- inspect background components, tail recent component logs, and request component restarts

## Integrations

### `GET /api/integrations`
### `GET /api/integrations/connections`
### `GET /api/integrations/connections/:id`
### `GET /api/integrations/connections/:id/events`
### `GET /api/integrations/:id/logs`

- inspect integration health, connections, events, and logs

### `PATCH /api/integrations/:id/source`

- update local source paths for file-backed or snapshot-backed integrations

### `PATCH /api/integrations/google-calendar`
### `POST /api/integrations/google-calendar/disconnect`
### `POST /api/integrations/google-calendar/auth/start`
### `GET /api/integrations/google-calendar/oauth/callback`

- save Google Calendar settings, start OAuth, complete the callback flow, or disconnect

### `PATCH /api/integrations/todoist`
### `POST /api/integrations/todoist/disconnect`

- save Todoist credentials or disconnect the integration

## WebSocket

### `GET /ws`

Current event types include:

- `messages:new`
- `interventions:new`
- `interventions:updated`
- `context:updated`
- `runs:updated`
- `components:updated`

The WebSocket surface is for operator updates and broadcast notifications. Durable state still lives behind the HTTP API and persisted runtime records.
