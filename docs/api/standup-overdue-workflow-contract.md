# Standup Overdue Workflow Contract

Status: mounted baseline implementation.

Last updated: 2026-04-15.

This document defines the backend-owned contract for the first morning standup overdue-task workflow vertical.

## Goal

Provide one supervised action flow for overdue commitments:

- `menu` (what can be done)
- `confirm` (operator confirms chosen action and payload)
- `apply` (mutation commit)
- `undo` (bounded rollback when supported)

All mutation-capable paths remain confirmation-first and idempotent.

## Runtime Endpoints (`/v1`)

### `POST /v1/daily-loop/sessions/:id/overdue/menu`

Returns overdue commitments and bounded action options for each commitment.

The request `today` field is a `YYYY-MM-DD` day boundary. The runtime returns open commitments with `due_at` before the end of that requested UTC day.

Request:

```json
{
  "today": "2026-03-25",
  "include_vel_guess": true,
  "limit": 50
}
```

Response:

```json
{
  "session_id": "dl_01...",
  "items": [
    {
      "commitment_id": "com_01...",
      "title": "ship planning review",
      "due_at": "2026-03-24T17:00:00Z",
      "actions": ["close", "reschedule", "back_to_inbox", "tombstone"],
      "vel_due_guess": {
        "suggested_due_at": "2026-03-26T16:00:00Z",
        "confidence": "medium",
        "reason": "next free block + similar task duration"
      }
    }
  ]
}
```

### `POST /v1/daily-loop/sessions/:id/overdue/confirm`

Creates a mutation proposal for one overdue action.

Request:

```json
{
  "commitment_id": "com_01...",
  "action": "reschedule",
  "payload": {
    "due_at": "2026-03-26T16:00:00Z",
    "source": "vel_guess"
  },
  "operator_reason": "meeting moved"
}
```

Response:

```json
{
  "proposal_id": "mp_01...",
  "confirmation_token": "confirm:mp_01...",
  "requires_confirmation": true,
  "write_scope": ["commitment:com_01...:due_at"],
  "idempotency_hint": "ovd:dl_01...:com_01...:reschedule"
}
```

### `POST /v1/daily-loop/sessions/:id/overdue/apply`

Applies a previously confirmed proposal.

Request:

```json
{
  "proposal_id": "mp_01...",
  "idempotency_key": "ovd:dl_01...:com_01...:reschedule",
  "confirmation_token": "cfm_01..."
}
```

Response:

```json
{
  "applied": true,
  "action_event_id": "evt_01...",
  "run_id": "run_01...",
  "before": { "due_at": "2026-03-24T17:00:00Z", "status": "open" },
  "after": { "due_at": "2026-03-26T16:00:00Z", "status": "open" },
  "undo_supported": true
}
```

### `POST /v1/daily-loop/sessions/:id/overdue/undo`

Requests rollback for the last supported action.

Request:

```json
{
  "action_event_id": "evt_01...",
  "idempotency_key": "ovd-undo:evt_01..."
}
```

Response:

```json
{
  "undone": true,
  "run_id": "run_01...",
  "before": { "due_at": "2026-03-26T16:00:00Z", "status": "open" },
  "after": { "due_at": "2026-03-24T17:00:00Z", "status": "open" }
}
```

## Action Vocabulary

Allowed actions:

- `close`
- `reschedule`
- `back_to_inbox` (remove due date)
- `tombstone`

Unknown actions must fail closed with `400`.

## Error Contract

- `400`: invalid action, malformed payload, unsupported undo target.
- `403`: policy denied (write scope or capability denied).
- `404`: session/proposal/commitment not found.
- `409`: apply attempted without valid confirmation token.
- `422`: due date outside allowed policy constraints.

## CLI Surface

Current CLI wrappers over the API above:

```bash
vel daily-loop overdue menu [--limit 50] [--json]
vel daily-loop overdue confirm <commitment_id> --action <close|reschedule|back_to_inbox|tombstone> [--due-at <iso8601>] [--reason <text>] [--json]
vel daily-loop overdue apply <proposal_id> --confirmation-token <token> [--idempotency-key <key>] [--json]
vel daily-loop overdue undo <action_event_id> [--idempotency-key <key>] [--json]
```

## CLI UX Mock Output Examples

`vel daily-loop overdue menu`:

```text
session: dls_01...
overdue:
  - Ship planning review (com_01...)
    due: 2026-03-24T17:00:00Z
    actions: close, reschedule, back_to_inbox, tombstone
    vel_guess: 2026-03-26T16:00:00Z (Medium) — next free block + similar task duration
```

`vel daily-loop overdue confirm com_01... --action reschedule --due-at 2026-03-26T16:00:00Z --reason "meeting moved"`:

```text
proposal: mp_01...
confirmation_token: confirm:mp_01...
requires_confirmation: true
write_scope: commitment:com_01...:due_at
idempotency_hint: ovd:dl_01...:com_01...:reschedule
```

`vel daily-loop overdue apply mp_01... --confirmation-token confirm:mp_01...`:

```text
applied: true
run_id: run_01...
action_event_id: evt_01...
idempotency_key: ovd:apply:mp_01...
undo_supported: true
before: status=open, due_at=2026-03-24T17:00:00Z
after: status=open, due_at=2026-03-26T16:00:00Z
```

`vel daily-loop overdue undo evt_01...`:

```text
undone: true
run_id: run_01...
action_event_id: evt_01...
idempotency_key: ovd:undo:evt_01...
before: status=open, due_at=unscheduled
after: status=open, due_at=2026-03-24T17:00:00Z
```

## Voice and Accessibility Mapping

Voice and watch reactions must map to the same action vocabulary and proposal/confirm/apply transport when those shells expose overdue actions.

- voice quick phrase examples: `close it`, `reschedule tomorrow 4pm`, `back to inbox`, `delete task`.
- shells may do local STT/TTS, but action authority remains backend-owned.
- if voice parsing is uncertain, shell must request typed confirmation before `apply`.
- current mounted coverage includes the backend contract, CLI contract, Apple Watch quick reactions, and iOS voice overdue quick reactions. Apple shells must continue to fall back to typed confirmation when target selection or reschedule payloads are uncertain.

## Security and Observability Constraints

- no mutation without proposal + confirmation token.
- every apply/undo emits run events with stable `run_id`; apply also returns a durable action event ID for undo.
- apply run payloads include the operator reason and reschedule payload source when supplied.
- logs and artifacts must not include decrypted secrets.
- repeated apply/undo with the same idempotency key returns the deterministic stored response.
- applying an already-applied proposal with a different idempotency key fails closed.

## Linked Implementation Ticket

- [038-standup-overdue-workflow-slice.md](../tickets/phase-5/038-standup-overdue-workflow-slice.md)
