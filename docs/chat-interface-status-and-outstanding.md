# Chat interface — status and outstanding

> Repo-wide implementation status is tracked in `docs/status.md`. This document provides chat-specific detail and outstanding work.

## Ticket status (001–035)

| Range   | Status   | Notes |
|--------|----------|--------|
| 001–012 | **Done** | Monorepo, crates, IDs, message/intervention models, migrations, conversation/message/intervention/event_log repos |
| 013    | **Done** | Axum server skeleton |
| 014–019 | **Done** | Conversation, message, inbox, intervention actions API; WebSocket /ws; broadcast messages:new, interventions:updated |
| 020–024 | **Done** | React client (Vite, TS, Tailwind), app shell, conversation list, thread view, message composer |
| 025–027 | **Done** | Card renderer (reminder, risk, suggestion, summary); inline actions (snooze, resolve, dismiss, show why); inbox view |
| 028     | **Done** | Context panel (GET /v1/context/current) |
| 029–033 | **Done** | Provenance API + drawer; settings API + UI; seed script; New conversation in sidebar |
| 034     | **Done** | Backend tests: chat list/create/get 404, create message then list, inbox empty, settings get/patch, intervention snooze 404, WebSocket GET /ws |
| 035     | **Done** | Frontend tests: MessageRenderer (text, cards, onSnooze/onShowWhy), MessageComposer (send, error), SettingsPage (load, Back, apiPatch on toggle) |

## Alignment with docs

- **Specs:** [vel-chat-interface-implementation-brief.md](specs/vel-chat-interface-implementation-brief.md), [vel-chat-execution-plan.md](specs/vel-chat-execution-plan.md) — API shape, domain model, WebSocket, React client match current implementation.
- **Ticket pack:** [docs/tickets/](tickets/) — 001–035 implemented (034–035 tests added).
- **Status / index:** repo-wide canonical ledger is [status.md](status.md); this file and [vel-documentation-index-and-implementation-status.md](vel-documentation-index-and-implementation-status.md) defer to it for rollout truth.

## “Nothing happens when I send a message”

- **Backend:** `POST /api/conversations/:id/messages` is implemented and returns `CreateMessageResponse` (`{ ok, data: { user_message, assistant_message?, assistant_error? } }`). Verified with curl.
- **Frontend:** MessageComposer posts, then on success calls `onSent(res.data)` and ThreadView appends via `setMessages(prev => [...prev, msg])`. So the message should appear.
- **Change made:** Composer now has **error state**: if the request fails or the API returns `ok: false`, the error is shown under the composer (red text). That will surface network errors, 4xx/5xx, or “Send failed” when `res.data` is missing.
- **If it still does nothing:** Check the browser devtools Network tab for the POST to `/api/conversations/.../messages`: status code and response body. Ensure you’re hitting the Vite dev server (so the proxy forwards to veld). Ensure veld is the current binary (restart after pull/build).

## Outstanding (concise)

1. **Optional:** Inbox “Open thread” link to conversation/message; quiet_hours in settings UI when backend supports it; provenance `signals` / `policy_decisions` populated from real data.
2. **Optional:** Real-time message list updates via WebSocket (client currently only appends on send; could subscribe to `messages:new` for the current thread).
