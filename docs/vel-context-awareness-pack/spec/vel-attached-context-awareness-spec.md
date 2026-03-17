---
title: Vel Attached Context Awareness Spec
status: proposed
owner: vel
updated: 2026-03-16
---

# Vel Attached Context Awareness Spec

## 1. Problem statement

Right now Vel can *display* context, but it cannot reliably *think with* the context already attached to the system.

The current codebase has several important building blocks already in place:

- persistent current context via `/v1/context/current`
- explain surfaces for context / drift / commitments
- integrations status surfaces and sync adapters
- transcripts, messaging, calendar, Todoist, notes, and git ingestion
- thread and commitment primitives
- a working web chat shell with realtime updates

But the actual chat reply path is still context-thin.

In the current implementation, assistant replies are generated from:

1. the last ~50 messages in the conversation
2. a fixed system prompt
3. no explicit context packet
4. no selected attachments from the operator
5. no persisted provenance for *which context objects were used to answer*

That means Vel can look context-rich in the UI while still replying like a goldfish with a nice dashboard.

## 2. Current-state assessment

### What is already true

- `crates/veld/src/routes/chat.rs` persists user messages and generates an assistant reply.
- `generate_assistant_reply(...)` already loads recent thread history and calls the configured LLM router.
- `docs/status.md` says persistent current context, context timeline, explainability, signal adapters, threads, and integrations are implemented.
- The web app already has `Now`, `ContextPanel`, `ProvenanceDrawer`, and `Settings` surfaces.
- The repo already has a generic `refs` system and `attached_to` relation type that can be reused for message/context provenance.

### What is missing

- `MessageCreateRequest` has no concept of attached context or operator-selected evidence.
- The web composer sends only `{ role, kind, content }`.
- The LLM request metadata is an empty JSON object.
- The assistant reply builder does not query current context, commitments, threads, recent signals, or synced source objects.
- Assistant replies do not persist a machine-readable record of which context packet was used.
- There is no token-budgeted ranking strategy for “what context gets attached now”.
- There is no UI affordance for manual attach/pin/exclude of context.
- There is no regression test proving that a question like “what is current status” grounds itself in current context instead of replying generically.

## 3. Goal

Make Vel aware of its attached context such that, when a user asks a question in chat, the reply path can:

- automatically gather the most relevant current context
- optionally include operator-selected attachments
- ground the model in structured evidence
- expose what was used and why
- degrade honestly when relevant context is missing or stale

## 4. Non-goals

- full autonomous retrieval over every historical object in the system
- arbitrary tool execution inside the chat path
- giant raw-context dumps into the prompt
- pretending uncertain inferences are facts
- replacing the existing current-context subsystem

## 5. Product behavior

### 5.1 Desired user experience

When the user asks a question like:

- “what is current status”
- “what matters right now?”
- “what am I waiting on?”
- “what is Vel aware of?”
- “what’s the risk today?”

Vel should answer using a context packet assembled from the live system state, not just the literal thread transcript.

The answer should be able to reference:

- current mode / morning state / drift state
- top open commitments
n- upcoming calendar windows
- waiting-on-me messaging pressure
- recent synced notes / transcripts / git activity when relevant
- uncertainty or missing-source caveats

### 5.2 Attached-context modes

Every chat send should support one of three modes:

- `auto`: Vel selects relevant context automatically
- `manual`: use operator-selected attachments plus minimal automatic safety context
- `none`: only conversation history; useful for pure ideation or debugging

Default should be `auto`.

## 6. Proposed architecture

## 6.1 New concept: Context Packet

Introduce a first-class `ContextPacket` assembled per assistant turn.

A context packet is a bounded, structured object that summarizes the evidence Vel is allowed to think with for that turn.

### Suggested shape

```json
{
  "packet_id": "ctxp_123",
  "generated_at": 1760000000,
  "conversation_id": "conv_123",
  "message_id": "msg_123",
  "mode": "auto",
  "summary": {
    "current_mode": "workday_focus",
    "morning_state": "engaged",
    "global_risk_level": "medium",
    "message_waiting_on_me_count": 3,
    "next_commitment": {
      "id": "com_1",
      "text": "draft residency budget",
      "due_at": 1760003600
    }
  },
  "attachments": [
    {
      "ref_type": "current_context",
      "ref_id": "singleton",
      "title": "Current context",
      "reason": "Asked about current status",
      "confidence": 0.98,
      "freshness_seconds": 43
    },
    {
      "ref_type": "commitment",
      "ref_id": "com_1",
      "title": "draft residency budget",
      "reason": "Top open commitment",
      "confidence": 0.86,
      "freshness_seconds": 120
    }
  ],
  "warnings": [
    "calendar sync stale by 4h"
  ],
  "budget": {
    "items_considered": 21,
    "items_included": 6,
    "chars_used": 4120
  }
}
```

The packet should be persisted as a typed artifact or JSON row and linked via refs to the user message and assistant message.

## 6.2 Context sources for v1

For v1 of attached-context awareness, restrict source selection to:

1. current context singleton
2. explain/context summary
3. top open commitments
4. next calendar window / current prep or commute window
5. recent messaging-pressure summary
6. conversation thread history
7. operator-selected attachments
8. recent thread links when present

Later versions can add full retrieval over transcripts, notes, docs, and repo self-knowledge.

## 6.3 Retrieval / ranking strategy

Build a lightweight `ContextSelector` service.

### Inputs

- current user message text
- conversation id
- selected attachment refs
- attached-context mode
- current context singleton
- explain/context payload
- top commitments / risk
- thread links
- freshness of integrations

### Outputs

- ranked context items
- reasons for inclusion / exclusion
- warnings when sources are stale or absent
- token/character bounded context packet

### Initial heuristics

- Always include current context in `auto` unless unavailable.
- Boost commitments and calendar when the user asks about status, today, next, due, risk, or what matters.
- Boost messaging summary for “waiting on me”, “reply”, “inbox”, “status”.
- Boost manual attachments above automatic items.
- Penalize stale integrations.
- Hard-cap included items and serialized size.

This should be explicit rule-based ranking first, not premature embedding soup.

## 6.4 Prompt assembly

Replace the single static chat prompt with a structured prompt builder.

### Layers

1. **Core system identity**
2. **Operating instructions for groundedness**
3. **Structured context packet**
4. **Conversation history**
5. **Current user message**

### Required grounding instructions

The assistant must be instructed to:

- prefer attached context over generic filler
- name missing/stale sources when they matter
- distinguish facts from inference
- avoid claiming awareness of context not present in the packet
- answer tersely when the question is narrow

## 6.5 Provenance model

Persist provenance for each assistant reply.

### Minimum provenance requirements

- packet id used for the turn
- ordered list of attached refs
- inclusion reasons
- stale/missing warnings
- prompt assembly metadata version

### Storage approach

Use the existing `refs` table where possible:

- `message(user) -> context_packet` (`attached_to`)
- `message(assistant) -> context_packet` (`derived_from`)
- `context_packet -> commitment/current_context/thread/...` (`attached_to`)

Use either:

- a new `context_packets` table, or
- an artifact record with `artifact_type = context_packet` plus JSON payload

Recommended approach: use artifact storage semantics for inspectability and reuse existing artifact/ref patterns.

## 6.6 API changes

### Extend `MessageCreateRequest`

```json
{
  "role": "user",
  "kind": "text",
  "content": { "text": "what is current status" },
  "context_mode": "auto",
  "attached_refs": [
    { "ref_type": "commitment", "ref_id": "com_1" }
  ]
}
```

### Extend `CreateMessageResponse`

Return optional context-packet metadata:

```json
{
  "user_message": { "...": "..." },
  "assistant_message": { "...": "..." },
  "assistant_error": null,
  "context_packet": {
    "packet_id": "ctxp_123",
    "mode": "auto",
    "attachment_count": 6,
    "warnings": ["calendar sync stale by 4h"]
  }
}
```

### New read route

`GET /api/messages/:id/context`

Returns the hydrated packet and refs used for that turn.

## 6.7 Web UX changes

### Composer

Add:

- attached-context mode toggle (`Auto`, `Manual`, `None`)
- attach current context button
- attach commitment / thread / run / artifact affordances from nearby UI
- small chip row showing what will be attached

### Thread surface

Add:

- “Used context” pill on assistant messages
- click-through drawer showing packet summary, included refs, warnings, and freshness

### Now / Context panel integration

Allow pinning an item from Now / Context / Provenance into the next message.

## 6.8 Honest-failure behavior

When the packet lacks needed evidence, Vel should say so.

Examples:

- “I can see current context, but calendar sync is stale, so schedule claims may be wrong.”
- “I don’t have an attached status packet for this turn, so this answer is based only on the chat thread.”

## 7. Data model

## 7.1 API types

Add:

- `AttachedContextMode`
- `AttachedRefRequest`
- `ContextPacketSummaryData`
- `MessageContextData`

## 7.2 Artifact metadata

If packets are persisted as artifacts, use metadata like:

```json
{
  "generator": "chat_context_packet_v1",
  "conversation_id": "conv_123",
  "user_message_id": "msg_user",
  "assistant_message_id": "msg_assistant",
  "mode": "auto",
  "attachment_count": 6,
  "prompt_builder_version": "v1"
}
```

## 8. Implementation plan

### Phase 1 — Grounding backbone

- add request/response types for attached context
- build server-side context selector
- assemble and persist context packet
- wire LLM prompt builder to use the packet
- persist refs/provenance

### Phase 2 — UX and inspection

- add composer mode toggle and chip row
- add message-level context drawer
- add “pin to next message” from Now / Context panel

### Phase 3 — Smarter evidence selection

- query thread links and recent commitments/risk
- improve ranking heuristics
- add stale-source policy/warnings

## 9. Acceptance criteria

Vel counts as “aware of attached context” when all of the following are true:

1. A message can be sent with `auto`, `manual`, or `none` attached-context mode.
2. In `auto`, the assistant turn uses current context plus ranked relevant items.
3. In `manual`, the assistant turn honors operator-selected refs.
4. The server persists a context packet and links it to both user and assistant messages.
5. The UI can inspect what context was used for a reply.
6. Tests prove that “what is current status” produces a grounded answer when context exists.
7. Tests prove that stale/missing context yields honest caveats instead of false certainty.

## 10. Concrete repo touchpoints

These are the most likely files/modules to change first:

- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/services/` (new chat context services)
- `crates/vel-storage/src/db.rs`
- `clients/web/src/components/MessageComposer.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/ProvenanceDrawer.tsx` or sibling drawer
- `clients/web/src/types.ts`
- `docs/status.md`
- `docs/chat-interface-status-and-outstanding.md`

## 11. Why this ordering

The temptation is to bolt “memory” onto chat as an undifferentiated slop bucket. Don’t.

Vel already has the beginnings of a symbolic order:

- current context
- commitments
- nudges
- explainability
- refs
- threads
- integrations

The right move is not “more context everywhere.” The right move is a disciplined attachment model that says:

- what was attached
- why it was attached
- what was excluded
- how fresh it was
- what the assistant actually used

That makes the system debuggable instead of mystical.
