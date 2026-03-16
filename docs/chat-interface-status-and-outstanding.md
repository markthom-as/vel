# Chat interface — status and outstanding

> Repo-wide implementation status is tracked in `docs/status.md`. This document provides chat-specific detail and outstanding work.

## Ticket status (001–035)

| Range   | Status   | Notes |
|--------|----------|--------|
| 001–012 | **Done** | Monorepo, crates, IDs, message/intervention models, migrations, conversation/message/intervention/event_log repos |
| 013    | **Done** | Axum server skeleton |
| 014–019 | **Done** | Conversation, message, inbox, intervention actions API; WebSocket /ws; broadcast `messages:new`, `interventions:new`, `interventions:updated` |
| 020–024 | **Done** | React client (Vite, TS, Tailwind), app shell, conversation list, thread view, message composer |
| 025–027 | **Done** | Card renderer (reminder, risk, suggestion, summary); inline actions (snooze, resolve, dismiss, show why); inbox view |
| 028     | **Done** | Context panel (GET /v1/context/current) |
| 029–033 | **Done** | Provenance API + drawer; settings API + UI; seed script; New conversation in sidebar |
| 034     | **Done** | Backend tests: chat list/create/get 404, create message then list, inbox empty, settings get/patch, intervention snooze 404, WebSocket GET /ws |
| 035     | **Done** | Frontend tests: MessageRenderer (text, cards, onSnooze/onShowWhy), MessageComposer (send, error), SettingsPage (load, Back, apiPatch on toggle, integrations actions, run controls) |

## Alignment with docs

- **Specs:** [vel-chat-interface-implementation-brief.md](specs/vel-chat-interface-implementation-brief.md), [vel-chat-execution-plan.md](specs/vel-chat-execution-plan.md) — API shape, domain model, WebSocket, React client match current implementation.
- **Ticket pack:** [docs/tickets/](tickets/) — 001–035 implemented (034–035 tests added).
- **Status / index:** repo-wide canonical ledger is [status.md](status.md); this file and [vel-documentation-index-and-implementation-status.md](vel-documentation-index-and-implementation-status.md) defer to it for rollout truth.
- **Realtime contract:** `/ws` carries `messages:new` for newly persisted chat messages, `interventions:new` for newly created interventions, and `interventions:updated` for snooze/resolve/dismiss actions.
- **Transport contract cleanup:** chat/websocket DTOs now live in `crates/vel-api-types`, and the web client consumes them through a centralized runtime decoder layer instead of per-component ad hoc guards.
- **Shared client data layer:** thread, inbox, context, settings, and provenance now load through a shared query/cache path with targeted realtime invalidation instead of each screen owning its own fetch/refetch state machine.
- **Settings / integrations operator surface:** the Settings page now includes `/api/integrations` management for Google Calendar and Todoist plus recent-run retry/block controls, with frontend tests covering inline action isolation and websocket-aware run state updates.
- **Context panel:** the web Context panel now reads `/v1/explain/context` plus `/v1/explain/drift`, so operators see derived state together with reasons, drift summary, and hydrated signal summaries instead of only the raw `current_context` blob.
- **Provenance:** message provenance is no longer placeholder-only. The route/drawer now returns linked objects plus card/intervention-derived evidence, so “Show why” surfaces actual supporting data beyond message event history.

## “Nothing happens when I send a message”

- **Backend:** `POST /api/conversations/:id/messages` is implemented and returns `CreateMessageResponse` (`{ ok, data: { user_message, assistant_message?, assistant_error? } }`). Verified with curl.
- **Frontend:** ThreadView now inserts the user message optimistically as soon as send starts, then reconciles that provisional row against both the websocket `messages:new` echo and the confirmed `POST /api/conversations/:id/messages` payload. That keeps a single message row even if the websocket event arrives before or after the HTTP response.
- **Failure behavior:** Composer surfaces an inline error if the request fails or the API returns `ok: false`, and the optimistic message is removed if send confirmation never arrives.
- **Intervention actions:** Snooze, resolve, and dismiss now hide optimistically in thread/inbox state and restore the intervention if the POST fails.
- **If it still does nothing:** Check the browser devtools Network tab for the POST to `/api/conversations/.../messages`: status code and response body. Ensure you’re hitting the Vite dev server (so the proxy forwards to veld). Ensure veld is the current binary (restart after pull/build).

## Outstanding (concise)

1. **State orchestration polish:** tighten loading/error/empty presentation now that the fetch path is shared.
2. **Optional product polish:** Inbox “Open thread” link to conversation/message; quiet_hours in settings UI when backend supports it; richer provenance presentation now that the route returns real supporting data.
3. **Realtime contract hardening:** add event sequencing or stable envelope ids if later work needs deterministic replay or stronger websocket resume semantics.
