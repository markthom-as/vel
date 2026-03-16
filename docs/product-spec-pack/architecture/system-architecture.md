# Vel System Architecture

Vel operates across three conceptual layers.

## 1. Experience Layer

User‑visible surfaces.

- CLI
- Chat
- Voice
- Notifications
- Dashboard

## 2. Decision Layer

Engines that interpret context.

- Context Engine
- Risk Engine
- Suggestion Engine
- Policy Engine

## 3. Data Layer

Persistent state.

- commitments
- suggestions
- risk assessments
- activity logs
- user context

## Interaction Flow

```
User
 ↓
Surface (CLI / Chat / Voice)
 ↓
Interaction Layer
 ↓
Policy Engine
 ↓
Context Engine
 ↓
Risk Engine
 ↓
Suggestion Engine
 ↓
Response
```

Vel's architecture favors:

- deterministic state
- explicit decisions
- observable reasoning