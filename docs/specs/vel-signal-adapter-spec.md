
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

Create adapters under:

```
crates/vel-signals/adapters/
```

Initial adapters:

```
calendar_adapter
todoist_adapter
activity_adapter
capture_adapter
transcript_adapter
feedback_adapter
```

---
# 4. Calendar Adapter

Source systems:

- Google Calendar
- Apple Calendar
- ICS feeds

Signal types produced:

```
calendar_event_created
calendar_event_updated
calendar_event_deleted
calendar_event_started
calendar_event_ended
```

Payload example:

```
{
  "event_id": "...",
  "title": "...",
  "start_time": 1700000000,
  "end_time": 1700003600,
  "location": "...",
  "attendees": ["..."],
  "calendar": "..."
}
```

Adapter responsibilities:

- detect event changes
- normalize timestamps
- emit signals

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

Signal types:

```
task_created
task_updated
task_completed
task_deleted
```

Payload example:

```
{
  "task_id": "...",
  "content": "...",
  "due_time": 1700000000,
  "labels": ["health"],
  "priority": 3,
  "project": "..."
}
```

Responsibilities:

- track task state changes
- emit completion events
- normalize due times

---
# 6. Activity Adapter

Source:

- workstation activity
- shell sessions
- login signals

Signal types:

```
shell_login
shell_exit
computer_activity
idle_state
```

Payload example:

```
{
  "host": "workstation",
  "activity": "shell_login"
}
```

Later expansions may include:

- application focus
- directory activity
- git commits

---
# 7. Capture Adapter

Source:

Vel CLI captures.

Signal types:

```
capture_created
capture_updated
capture_promoted
```

Payload example:

```
{
  "capture_id": "...",
  "content": "todo: review budget",
  "tags": ["todo","finance"]
}
```

Responsibilities:

- record captures as signals
- link to capture table entries

---
# 8. Transcript Adapter

Sources:

- ChatGPT exports
- Vel native assistant conversations
- future agent transcripts

Signal types:

```
assistant_message
assistant_conversation_start
assistant_conversation_end
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
vel sync tasks
vel ingest transcripts
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
