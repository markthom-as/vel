
# vel_signal_adapter_spec.md

Status: Canonical signal adapter specification  
Audience: coding agent implementing Vel  
Purpose: define how external data sources are normalized into Vel signals

---
# 1. Adapter Philosophy

Vel should ingest many external systems but normalize them into a **single signal format**.

Adapters must:

- translate external system data into normalized signals
- avoid embedding inference logic
- remain deterministic
- be idempotent where possible

Adapters should **not**:

- compute risk
- create nudges
- interpret context
- perform synthesis

Those belong to other modules.

Pipeline:

external system → adapter → normalized signal → signals table

---
# 2. Canonical Signal Structure

All adapters must emit signals with this structure.

```
{
  "id": "<generated_id>",
  "signal_type": "<type>",
  "source": "<system>",
  "source_ref": "<external_id_optional>",
  "timestamp": <unix_seconds>,
  "payload_json": { ... }
}
```

Meaning:

| Field | Meaning |
|------|------|
| id | unique signal id |
| signal_type | canonical Vel signal category |
| source | external system name |
| source_ref | external identifier |
| timestamp | time event occurred |
| payload_json | normalized structured data |

---
# 3. Adapter Modules

Implemented adapters currently live under:

```
crates/veld/src/adapters/
```

Initial adapters:

```
calendar
todoist
activity
git
notes
transcripts
```

---
# 4. Calendar Adapter

Source systems:

- Google Calendar
- Apple Calendar
- ICS feeds

Current signal type produced:

```
calendar_event
```

Payload example:

```
{
  "event_id": "...",
  "title": "...",
  "start": 1700000000,
  "end": 1700003600,
  "location": "...",
  "description": "...",
  "status": "confirmed",
  "url": "https://...",
  "attendees": ["..."],
  "prep_minutes": 15,
  "travel_minutes": 20
}
```

Adapter responsibilities:

- normalize timestamps
- emit replay-safe normalized event signals from ICS snapshots/feeds
- preserve event metadata needed by inference and explain surfaces

Adapter must **not** infer:

- prep windows
- commute time
- commitments

Those belong in the inference layer.

---
# 5. Todoist / Task Adapter

Sources:

- Todoist API
- Apple Reminders
- task snapshots

Current signal type:

```
external_task
```

Payload example:

```
{
  "task_id": "...",
  "content": "...",
  "checked": false,
  "due": "2026-03-16",
  "labels": ["health"],
  "priority": 3,
  "project": "..."
}
```

Responsibilities:

- normalize task snapshots into replay-safe signals
- reconcile linked commitments in place from external task lifecycle
- preserve due/project/label metadata for downstream context and review

---
# 6. Activity Adapter

Source:

- workstation activity
- shell sessions
- login signals

Signal types:

```
shell_login
computer_activity
idle_state
vel_invocation
```

Payload example:

```
{
  "host": "workstation",
  "activity": "shell_login"
}
```

Current dedicated expansion:

- `git_activity` is emitted by the separate git adapter

---
# 7. Capture Adapter

Notes and captures are not currently a general “capture adapter” signal layer.

- raw captures are stored directly through the capture system
- notes sync emits replay-safe `note_document` captures plus `note_document` signals
- capture promotion into commitments remains a capture/runtime concern, not an adapter concern

---
# 8. Transcript Adapter

Sources:

- ChatGPT exports
- Vel native assistant conversations
- future agent transcripts

Current signal type:

```
assistant_message
```

Payload example:

```
{
  "conversation_id": "...",
  "role": "user",
  "content": "...",
  "project_hint": "vel"
}
```

Responsibilities:

- normalize messages
- preserve order
- dedupe replay by stable transcript identity
- allow thread linking later

Adapters must **not** attempt heavy interpretation.

---
# 9. Feedback Adapter

Source:

User feedback prompts.

Signal types:

```
vel_feedback
```

Payload example:

```
{
  "nudge_id": "...",
  "score": 1,
  "comment": "good timing"
}
```

Responsibilities:

- store user feedback
- link feedback to nudge or suggestion

---
# 10. Adapter Idempotency

Adapters should attempt idempotent behavior.

Strategies:

- track last processed timestamp
- track source_ref values
- skip duplicate signals

Adapters should **not** depend on fragile heuristics.

---
# 11. Scheduling Strategy

Adapters may run via:

- periodic jobs
- event hooks
- manual sync commands

Examples:

```
vel sync calendar
vel sync todoist
vel sync activity
vel sync git
vel sync notes
vel sync transcripts
```

---
# 12. Adapter Testing

Each adapter must support:

- replay of sample data
- deterministic signal output
- duplicate suppression tests

Testing approach:

input dataset → adapter → expected signal sequence

---
# 13. Future Adapters

The system must be designed so new adapters can be added easily.

Potential future sources:

- Apple Health
- sleep tracking
- smart home presence
- messaging platforms
- email
- GPS/location

Adapters should follow the same normalization rules.

---
# 14. Design Rule

Adapters convert **external facts into normalized Vel signals**.

They must never:

- interpret intent
- evaluate commitments
- trigger nudges
- compute risk

Adapters are **pure translation layers**.

---
# 15. Success Criteria

The adapter layer is successful when:

- external data reliably becomes signals
- signals are deduplicated
- signals are timestamped correctly
- downstream modules can operate purely on signals

Once signals are stable, the rest of the system can evolve independently.
