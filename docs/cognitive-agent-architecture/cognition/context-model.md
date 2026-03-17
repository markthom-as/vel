# Context Model

Context is the input substrate for Vel's reasoning. It is transformed from raw telemetry into decision-relevant abstractions.

## Context Domains

- **Time**: Local time, day of week, upcoming windows.
- **Calendar**: Next events, event proximity.
- **Location**: Current location (when available), travel estimates.
- **Activity**: Inferred activity (e.g., "working", "commuting", "sleeping").
- **Commitments**: Open tasks, due soon, top risks.
- **System Health**: Integration freshness, confidence penalties.

## CurrentContextV1 Schema

Vel uses a versioned, typed schema for the current context. Unknown or future fields are preserved in an `extra` map.

```json
{
  "computed_at": 1710000000,
  "mode": "day_mode",
  "morning_state": "engaged",
  "meds_status": "pending",
  "global_risk_level": "medium",
  "global_risk_score": 0.72,
  "attention_state": "on_task",
  "drift_type": "none",
  "drift_severity": "none",
  "attention_confidence": 0.8,
  "attention_reasons": ["recent git activity indicates active work"],
  "next_commitment_id": "com_123",
  "active_nudge_ids": ["nud_456"],
  "top_risk_commitment_ids": ["com_123", "com_789"],
  "signals_used": ["sig_1", "sig_2"],
  "commitments_used": ["com_123"],
  "risk_used": ["risk_abc"],
  "health_summary": { ... },
  "git_activity_summary": { ... },
  "mood_summary": { ... },
  "pain_summary": { ... },
  "note_document_summary": { ... },
  "assistant_message_summary": { ... },
  "message_waiting_on_me_count": 1,
  "extra": {
    "custom_future_field": "value"
  }
}
```

## Transformation Rule

Context is not raw telemetry worship.
It must be transformed into decision-relevant abstractions.

1.  **Reduce**: Filter out noise and deduplicate events.
2.  **Abstract**: Map raw events to states (e.g., "many git commits" -> "engaged in work").
3.  **Project**: Calculate proximity to deadlines and windows.
4.  **Score**: Apply policy-driven risk and confidence scores.
