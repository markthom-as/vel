# Vel Chat Interface — Implementation Brief

## Purpose

Vel needs a first-class chat interface, but the product should **not** be framed as "an LLM chat app." It should be treated as an **agent console with conversational affordances**: a control surface for commitments, reminders, risk, context, interventions, and trust.

The first release should optimize for:

- inspectability
- interruptibility
- structured actions
- provenance
- persistence
- future multi-client reuse

---

## Product Thesis

Vel is not valuable because it can talk. It is valuable because it can continuously answer:

1. what matters right now
2. why it matters
3. what is likely to be dropped
4. what the lightest useful intervention is

The chat interface is where these judgments are surfaced, negotiated, overridden, debugged, and refined.

---

## Non-Goals for V1

Do **not** try to build all of this immediately:

- full multimodal voice agent
- perfect mobile parity
- heavy social/collaborative chat features
- open-ended "assistant knows everything" behavior
- speculative autonomous action without tight auditability

Also avoid shipping a generic bubble-only chat UI. That would be the wrong abstraction.

---

## Core UX Principles

### 1. Structured objects over endless prose
Many outputs should be cards, alerts, summaries, and stateful objects—not paragraphs.

### 2. Every intervention must answer "why now?"
The user should be able to inspect provenance and triggering signals for proactive nudges.

### 3. The user must be able to act inline
Every reminder/suggestion/risk item should support direct actions like done, snooze, mute, defer.

### 4. Keep the superego on a leash
Avoid hectoring, shamey, or moralizing tone. Nudges should feel useful, not punitive.

### 5. Separate cognition from presentation
Core agent logic, APIs, and UI should be modular so multiple clients can exist later.

---

## Recommended System Shape

### Architectural split

Use three layers:

- **vel-core** — domain logic, event model, policies, risk/suggestion engines
- **vel-server** — API, persistence, auth, websocket/event stream, orchestration
- **vel-chat** — UI client (web first, desktop shortly after)

This preserves:

- testability
- reuse across clients
- clean boundaries
- easier future Apple/mobile/voice work
- better operational clarity

---

## Recommended Repo Strategy

### Preferred near-term option: monorepo with hard boundaries

```text
vel/
  Cargo.toml
  /crates
    /vel-core
    /vel-server
    /vel-store
    /vel-events
    /vel-policy
  /clients
    /web
    /desktop
  /docs
    /architecture
    /product
    /api
```

This keeps coordination tight while the system is still crystallizing.

### Recommendation

For now: **monorepo**. Vel is still co-evolving at the product/domain/protocol level. Splitting too early risks premature bureaucratization.

---

## Technology Recommendation

### Backend

- **Rust**
- **Axum** for HTTP/WebSocket
- **SQLite** with WAL for initial persistence
- `serde` tagged enums for message/card payloads
- background worker loop for interventions and event processing

### Frontend

- **React + TypeScript**
- **Next.js** or **Vite** web app for initial UI
- **Tauri** wrapping the React app for desktop

### State / Data Fetching

- TanStack Query for API state
- WebSocket subscription for live events/interventions

### Styling

- Tailwind CSS
- Headless primitives (Radix or similar)
- clarity > ornamental chrome

---

## Product Surface for V1

1. **Inbox** — proactive interventions, unresolved reminders, risk escalations, summaries
2. **Threads** — persistent conversations, history, contextual debugging
3. **Live context panel** — what Vel currently thinks is true/relevant
4. **Logs / provenance view** — why an intervention fired, signals, policy decisions
5. **Settings / controls** — quiet hours, proactivity thresholds, domain permissions, kill switches

---

## Domain Objects to Model Early

- `Conversation`
- `Message`
- `Commitment`
- `Reminder`
- `Suggestion`
- `RiskItem`
- `Routine`
- `ContextSnapshot`
- `Signal`
- `PolicyDecision`
- `Intervention`
- `ActionOutcome`

---

## Message Model

Use a discriminated union / tagged enum. Message kinds: `text`, `suggestion_card`, `risk_card`, `reminder_card`, `summary_card`, `system_notice`, `tool_event`, `context_update`. Shared fields: id, threadId, createdAt, role, kind, importance, status, source, provenance, linkedObjects, actions.

---

## Persistence Model

Core tables: `conversations`, `messages`, `commitments`, `reminders`, `suggestions`, `risk_items`, `policy_decisions`, `signals`, `context_snapshots`, `interventions`, `message_actions`, `event_log`. Maintain current-state tables for fast UI and append-only event log for auditability.

---

## API Recommendation

- **Conversations:** GET/POST /api/conversations, GET/PATCH /api/conversations/:id
- **Messages:** GET/POST /api/conversations/:id/messages, POST /api/messages/:id/actions
- **Inbox / interventions:** GET /api/inbox, GET /api/interventions, POST /api/interventions/:id/snooze|resolve|dismiss
- **Context:** GET /api/context/current, GET /api/context/history
- **Risk / suggestions:** GET /api/risks, GET /api/suggestions, POST accept/reject
- **Logs / provenance:** GET /api/logs/events, GET /api/messages/:id/provenance
- **Settings:** GET/PATCH /api/settings

WebSocket: messages:new, interventions:new|updated, context:updated, etc.

---

## Implementation Phases

**Phase 1 — Usable Chat Shell:** conversation list, thread view, composer, persistence, websocket, text messages, structured card scaffold, SQLite-backed APIs, React three-pane shell.

**Phase 2 — Agent-Native Functionality:** reminder/risk/suggestion cards, inbox, inline actions, live context panel, provenance drawer.

**Phase 3 — Trust, Tuning, Debuggability:** quiet hours, mute pattern, per-domain controls, audit/event timeline, policy decision viewer.

**Phase 4 — Multimodal / Companion:** desktop notifications, TTS, voice input, widgets, watch, mobile — do not start here.

---

## Immediate Implementation Order

1. Define core message/event/card schemas  
2. Create SQLite migrations for conversations/messages/interventions/event_log  
3. Implement Rust domain types and serialization  
4. Stand up Axum API routes for conversations/messages/inbox  
5. Add websocket event broadcasting  
6. Scaffold React app shell with three-pane layout  
7. Render thread + inbox using mock/real data  
8. Implement message composer and send flow  
9. Add structured card rendering  
10. Add inline actions (done/snooze/resolve)  
11. Add provenance drawer  
12. Add settings for quiet hours / proactivity toggles  

---

## Acceptance Criteria for V1

- User can hold persistent conversations with Vel  
- Vel can surface proactive items in inbox/thread  
- Items are structured cards  
- User can resolve/snooze/dismiss inline  
- User can inspect why an item appeared  
- Current context visible in side panel  
- Event log/audit path exists  
- Architecture does not trap future mobile/voice work  

---

## Final Product Framing

Vel chat should feel like: a structured conversation with your own operational memory; a debugger for your commitments and context; a place where the agent can be questioned, constrained, and tuned. Not a ChatGPT clone or a Slack clone.
