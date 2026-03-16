# Vel Chat Interface — Codex / Agent Execution Plan

This document converts the Vel chat interface design into an executable engineering plan.

Core idea: the chat UI is an **agent console**, not a chatbot.

Architecture assumptions:
- Rust backend
- Axum API
- SQLite (WAL)
- React + TypeScript web client
- WebSocket realtime events
- Monorepo

--------------------------------------------------
REPO STRUCTURE
--------------------------------------------------

vel/
  Cargo.toml
  /crates
    vel-core
    vel-events
    vel-store
    vel-policy
    vel-server
  /clients
    web
  /docs

--------------------------------------------------
PHASES
--------------------------------------------------

1. Foundation
2. Persistent Chat
3. Structured Messages
4. Actions + Interventions
5. Provenance + Context
6. Settings + Controls

--------------------------------------------------
PHASE 1 — DOMAIN MODEL
--------------------------------------------------

Core entities:

Conversation
Message
Intervention
Signal
ContextSnapshot
PolicyDecision
EventLog

Message kinds:

text
reminder_card
risk_card
suggestion_card
summary_card
system_notice
tool_event
context_update

Example Rust enum:

pub enum MessageBody {
    Text(TextMessage),
    Reminder(ReminderCard),
    Risk(RiskCard),
    Suggestion(SuggestionCard),
    Summary(SummaryCard),
    SystemNotice(SystemNotice),
}

--------------------------------------------------
PHASE 2 — DATABASE
--------------------------------------------------

SQLite tables:

conversations
messages
interventions
event_log

messages fields:
id
conversation_id
role
kind
content_json
status
importance
created_at

interventions fields:
id
message_id
kind
state
surfaced_at
resolved_at
snoozed_until
confidence

--------------------------------------------------
PHASE 3 — API
--------------------------------------------------

Endpoints:

GET  /api/conversations
POST /api/conversations
GET  /api/conversations/:id/messages
POST /api/conversations/:id/messages

GET  /api/inbox

POST /api/interventions/:id/snooze
POST /api/interventions/:id/resolve
POST /api/interventions/:id/dismiss

GET /api/logs/events
GET /api/context/current

WebSocket:

/ws

Events:

messages:new
interventions:new
interventions:updated
context:updated

--------------------------------------------------
PHASE 4 — WEB CLIENT
--------------------------------------------------

Stack:

React
TypeScript
Tailwind
TanStack Query
WebSocket client

Layout:

Left: navigation + inbox
Center: thread view
Right: context / provenance

Routes:

/
/chat/:conversationId
/logs
/settings

--------------------------------------------------
PHASE 5 — STRUCTURED CARDS
--------------------------------------------------

ReminderCard
RiskCard
SuggestionCard
SummaryCard

Each card supports actions:

mark_done
snooze
resolve
dismiss
show_why

--------------------------------------------------
PHASE 6 — PROVENANCE
--------------------------------------------------

Every intervention must answer:

Why now?
Which signals triggered it?
Which policy allowed it?

Endpoint:

GET /api/messages/:id/provenance

UI element:

Provenance drawer.

--------------------------------------------------
PHASE 7 — CONTEXT PANEL
--------------------------------------------------

Right panel shows:

current time block
active commitments
risk count
routine state
last update time

--------------------------------------------------
PHASE 8 — SETTINGS
--------------------------------------------------

Settings:

quiet hours
disable proactive nudges
toggle reminder/suggestion/risk types

Endpoints:

GET /api/settings
PATCH /api/settings

--------------------------------------------------
PHASE 9 — DEV FIXTURES
--------------------------------------------------

Seed script should insert:

sample conversation
sample reminder card
sample risk card
sample suggestion card
sample provenance
sample event log

--------------------------------------------------
PHASE 10 — ACCEPTANCE CRITERIA
--------------------------------------------------

V1 is complete when:

• conversations persist
• structured cards render
• inbox shows proactive interventions
• actions mutate state
• provenance visible
• event log exists
• realtime updates function

--------------------------------------------------
ANTI‑PATTERNS
--------------------------------------------------

Do NOT:

turn everything into chat bubbles
hide schema inside frontend conditionals
allow state changes without event log entries
fake provenance explanations

--------------------------------------------------
FINAL PRODUCT STANDARD
--------------------------------------------------

Vel should feel like:

an operational console
a debugger for commitments
a transparent agent interface

Not:

a ChatGPT clone
a Slack clone
a motivational nag bot
