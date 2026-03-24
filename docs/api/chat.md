# Vel Operator And Chat API (`/api`)

This document describes the currently mounted operator and chat API exposed by `veld` under `/api`, plus the matching WebSocket endpoint.

For repo-wide implementation truth, see [`../MASTER_PLAN.md`](../MASTER_PLAN.md). For route-level authority, inspect `crates/veld/src/app.rs`, `crates/veld/src/routes/chat.rs`, `crates/veld/src/routes/components.rs`, and `crates/veld/src/routes/integrations.rs`.

## Conversations

### `GET /api/conversations`
### `POST /api/conversations`
### `GET /api/conversations/:id`
### `PATCH /api/conversations/:id`

- list, create, inspect, and update conversations
- conversation records include identifiers, title, kind, pinned/archive state, thread-level `call_mode_active`, and timestamps
- `PATCH /api/conversations/:id` now also owns the browser call-mode flag for that thread, so shells can start or end call mode without inventing a second runtime state model

## Messages and interventions

### `POST /api/assistant/entry`

- create a backend-owned assistant entry without preselecting a thread
- accepts plain operator text plus an optional `conversation_id` continuity hint
- also accepts optional voice provenance metadata when a shell is submitting transcript-driven input
- persists the user message through the existing conversation/message model
- returns a typed route outcome:
  - `threads` when the backend classifies the entry as continuity/back-and-forth work
  - `inbox` when the backend classifies the entry as quick capture or triage work
  - `inline` when the backend can answer directly or route into a typed bounded flow such as morning overview or standup without pushing the operator into thread continuity first
- the web shell now uses this same route from both `Now` and `Threads`, so entry routing stays backend-owned instead of client-guessed
- desktop/browser push-to-talk feeds this same route after local speech-to-text; transcript provenance stays explicit and the backend still owns routing into `Now`, `Inbox`, or `Threads`
- when a thread has `call_mode_active`, browser shells may also speak the returned assistant reply locally through platform TTS, but that still rides on the same persisted conversation and assistant-entry path instead of a separate call transport
- may also return conversation continuity data, optional assistant reply/error state, and optional typed `daily_loop_session` data when assistant entry starts or resumes the canonical morning/standup flow
- may also return typed `end_of_day` data when assistant entry starts the run-backed closeout flow inline
- may also return typed `assistant_context` data: a backend-owned bounded recall pack with summary, focus lines, hit counts, source breakdown, scores, and provenance for the current request
- may also return a typed assistant proposal when the operator asks for a bounded action that still belongs in the supervised operator queue
- morning and standup assistant entry must reuse the existing typed daily-loop session authority instead of inventing assistant-only planning state
- end-of-day assistant entry must reuse the existing run-backed closeout/context pipeline so the returned summary remains explainable from persisted state and artifacts
- when longer `check_in`, `reflow`, or action follow-through needs more than an inline reply, the backend escalates that work into durable thread continuity with typed resolution metadata instead of leaving meaning to shell history text alone
- assistant proposals now use that same rule: the backend may create a dedicated `assistant_proposal` continuity thread with typed follow-through metadata instead of expecting shells to infer approval or applied state from chat text
- proposal follow-through is explicit and fail-closed:
  - direct operator confirmation when the proposal only needs bounded confirmation
  - execution handoff review when repo-local or other supervised write work still requires approval
  - gated follow-through when SAFE MODE, missing writeback enablement, or another trust blocker keeps the proposal unavailable
  - applied or reversed continuity stays on the same proposal thread once the operator uses the existing intervention/review lanes

### `GET /api/conversations/:id/messages`
### `POST /api/conversations/:id/messages`

- list and create messages in a conversation
- message creation uses `MessageCreateRequest`
- responses use `CreateMessageResponse`, including the persisted user message and optional assistant reply or assistant error
- when an LLM profile is configured, assistant replies are grounded in the typed Vel inspect/`Now` state and may use a bounded read-only Vel tool surface for local recall over persisted captures, notes, projects, people, threads, transcripts, commitments, and daily-loop state
- assistant-capable responses may include backend-owned `assistant_context` so shells can show or inspect the same bounded recall summary the backend used instead of inferring their own memory state
- the assistant chat surface does not bypass existing write, review, or SAFE MODE rules; tool access is read-only and thread/daily-loop continuity stays aligned with the backend-owned product lanes
- thread-local continuity still persists through conversation messages, but the web composer now reuses the shared assistant-entry contract rather than maintaining a separate shell-owned send path
- Apple still uses the dedicated `/v1/apple/voice/turn` compatibility route for typed Apple quick-loop replies, but supported Apple voice turns now preserve the same shared thread continuity substrate instead of inventing a separate local conversation policy
- for default daily use, treat `Threads` as continuity/search over persisted conversations, not as the primary triage queue
- thread continuity now also carries durable resolution context for deferred, edited, resolved, or still-pending follow-through work so shells can deep-link into the right thread without inventing separate resolution policy
- current limit: assistant proposals are not an ambient write lane. They can become `approved`, `applied`, `failed`, or `reversed`, but only through the existing intervention, execution-review, and writeback seams. Provider-level undo behavior is not invented just because proposal-thread reversal metadata exists.
- current limit: local recall is only as good as persisted Vel data and the current hybrid retrieval baseline. The backend returns source counts, scores, and provenance, but it does not imply broad graph memory or internet-backed knowledge.

### `GET /api/conversations/:id/interventions`
### `GET /api/messages/:id/interventions`
### `GET /api/messages/:id/provenance`

- inspect conversation-level interventions, message-level interventions, and message provenance

### `GET /api/inbox`

- operator inbox for surfaced interventions and related review items
- this is the backend-owned triage queue for the Inbox surface
- surfaced actions may include operator affordances such as `open_thread`, resolve, dismiss, or snooze depending on the evidence attached to the item

### `POST /api/interventions/:id/snooze`
### `POST /api/interventions/:id/resolve`
### `POST /api/interventions/:id/dismiss`

- explicit operator actions for intervention lifecycle changes

## Settings

### `GET /api/settings`
### `PATCH /api/settings`

- read and update chat/operator UI settings
- `PATCH /api/settings` now also owns daemon-held LLM routing updates for:
  - `openai_compat_profiles` for localhost OpenAI-compatible OAuth proxies
  - `openai_api_profiles` for direct OpenAI API profiles
- direct OpenAI API keys are stored server-side in daemon settings records, not written back into checked-in model profile TOML

### `GET /api/llm/profiles/:id/health`
### `POST /api/llm/handshake`

- inspect saved-profile health or run a draft handshake for an LLM profile from the `System` UI
- handshake checks use provider metadata endpoints only:
  - `llama_cpp` and OpenAI-compatible providers use `GET /models`
  - the handshake confirms auth and reachability without sending a chat completion request
- `POST /api/llm/handshake` accepts draft profile data so operators can verify a new OpenAI proxy or direct OpenAI API key before saving it

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
