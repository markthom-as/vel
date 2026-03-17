---
title: Chat service-boundary extraction plan
status: in_progress
owner: agent
type: architecture
priority: medium
created: 2026-03-17
depends_on:
  - 004-architecture-map-and-module-boundary-audit.md
labels:
  - vel
  - chat
  - services
---

Plan extraction of chat route responsibilities into clearer application services after the repo map exists.

## Current hotspot

Primary file:

- [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs)

This route file currently mixes six concerns:

1. HTTP handlers and router wiring
2. transport mapping from storage records to API DTOs
3. chat event-log emission
4. intervention classification and creation
5. assistant reply orchestration and fallback handling
6. provenance and settings projection

That makes it a route module, application service, realtime fanout point, and query/projection layer at the same time.

## Scope

- conversation/message write orchestration
- intervention lifecycle helpers
- assistant reply generation
- provenance assembly
- settings orchestration boundaries

## Target split

Keep [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs) as a thin HTTP adapter and move behavior into `services/chat/*`.

Recommended service layout:

- `services/chat/conversations.rs`
  - list/create/get/update conversation operations
- `services/chat/messages.rs`
  - list conversation messages
  - create user message
  - shared message mapping helpers
- `services/chat/assistant.rs`
  - assistant reply generation
  - LLM history shaping
  - fallback and retryability rules
- `services/chat/interventions.rs`
  - message-to-intervention classification
  - create intervention for message when needed
  - snooze/resolve/dismiss flows
  - event and websocket fanout for intervention changes
- `services/chat/provenance.rs`
  - message provenance query
  - linked object assembly
  - provenance signal and policy-decision shaping
- `services/chat/settings.rs`
  - settings payload assembly
  - settings patch orchestration
- `services/chat/events.rs`
  - chat-scoped event-log helper(s)

If helpful, add a small [crates/veld/src/services/chat/mod.rs](/home/jove/code/vel/crates/veld/src/services/mod.rs) facade to re-export the service surface.

## Extraction phases

### Phase 1. Helper extraction only

Move pure helpers first without changing route signatures:

- DTO mappers
- fallback classification helpers
- provenance builders
- settings payload builders

Goal:

- behavior-preserving code motion
- no route contract changes

### Phase 2. Message write orchestration

Extract the first high-value write seam from `create_message`:

- user message persistence
- event-log append
- optional intervention creation
- websocket fanout for created user messages

Goal:

- separate write orchestration from route parsing

### Phase 3. Assistant reply service

Move `generate_assistant_reply` and fallback policy into `services/chat/assistant.rs`.

Goal:

- isolate LLM-specific behavior
- keep route returning a service result rather than directly orchestrating the model call

### Phase 4. Intervention action service

Move snooze, resolve, and dismiss flows into `services/chat/interventions.rs`.

Goal:

- unify intervention state change, event append, and websocket fanout logic

### Phase 5. Provenance and settings query services

Move read-heavy projection logic last:

- provenance query assembly
- settings payload assembly and mutation handling

Goal:

- make the route module structurally boring

### Phase 6. Optional transport cleanup

Only after the above is stable:

- decide whether chat services should return transport DTOs or narrower application-layer structs

This is optional for the first pass. Removing service work from the route layer matters more than purifying the return types immediately.

## Tests and guardrails

Keep current coverage stable around:

- conversation create/list/get/update
- create message then list
- inbox reads
- settings get/patch
- intervention 404 and mutation flows
- websocket route coverage
- message provenance hydration

Add service-level tests for:

- create-message orchestration
- assistant fallback path
- no-LLM-config path
- intervention auto-creation classification
- intervention event/websocket fanout
- provenance projection stability
- settings payload including adaptive policy overrides and timezone handling

## Acceptance criteria

- [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs) no longer owns assistant reply orchestration or intervention creation logic directly
- message and intervention write flows live in `services/chat/*`
- provenance and settings query assembly are moved out of the route module
- existing route behavior remains stable under existing integration tests

## Recommended first implementation slice

Start with Phase 1 and Phase 2 only:

1. extract helper modules
2. extract user-message write orchestration

Do not redesign chat semantics in the same change. The point of this ticket is boundary repair, not chat product expansion.
