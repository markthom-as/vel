
# vel_addendum_self_model_and_assistant_continuity.md

Status: Addendum to Vel Behavioral Constitution and Implementation Directive  
Audience: Coding agent implementing Vel  
Purpose: Introduce Vel self-awareness, assistant continuity, transcript ingestion, and feedback loops.

---
# 1. Overview

Vel should evolve beyond a scheduling/nudge engine into a **stateful assistant with self-awareness and ideation continuity**.

This addendum introduces three new capabilities:

1. **Vel Self-Model**
2. **Assistant / Chat Continuity**
3. **User Feedback Loop (including NPS-style signals)**

These systems enable Vel to:

- evaluate its own effectiveness
- learn from interaction patterns
- connect ideation with execution
- improve through dogfooding

These capabilities are **not required for the earliest MVP**, but the schema and architecture should support them early.

---
# 2. Vel Self-Model

Vel should maintain a model of **its own performance and usefulness**.

This allows the system to:

- measure whether nudges help
- measure whether suggestions are useful
- detect false positives
- adapt escalation policies
- generate self-improvement artifacts

## 2.1 Self-Model Metrics

Examples of metrics Vel should track:

- nudge_sent_count
- nudge_resolved_rate
- snooze_rate
- escalation_success_rate
- suggestion_acceptance_rate
- suggestion_rejection_rate
- response_delay_after_nudge
- ignored_nudges
- false_alarm_reports
- contexts where Vel was helpful
- contexts where Vel was annoying

These metrics should support both:

- policy tuning
- reflective synthesis

---
# 3. Self-Model Schema

Create table:

```
vel_self_metrics
----------------
id
metric_type
metric_value
context_json
timestamp
created_at
```

Example metric types:

```
nudge_effectiveness
suggestion_acceptance
feedback_score
assistant_helpfulness
assistant_annoyance
```

Indexes:

```
metric_type, timestamp
```

---
# 4. User Feedback Signals

Vel should occasionally prompt the user for feedback about its behavior.

Examples:

- “Was that reminder helpful?”
- “Did that suggestion improve the situation?”
- “How annoying was that escalation?”
- “Did Vel help you complete this task?”

This feedback should be lightweight and optional.

### NPS-style feedback

Vel may use a simple rating scale such as:

```
-2 very annoying
-1 slightly annoying
0 neutral
1 helpful
2 very helpful
```

Store feedback as signals:

signal_type:

```
vel_feedback
```

Example payload:

```
{
  "nudge_id": "nud_123",
  "feedback_score": 1,
  "comment": "good timing"
}
```

---
# 5. Assistant Continuity

Vel should act as the **central assistant interface** for:

- planning
- ideation
- task creation
- project discussion
- synthesis
- agent orchestration

External models (OpenAI, etc.) may be used internally, but Vel remains the **stateful orchestrator**.

---
# 6. Chat / Transcript Ingestion

Vel should ingest chat transcripts as a reflective signal source.

Sources may include:

- exported ChatGPT conversations
- Vel-native chat interface
- other assistant transcripts in the future

These transcripts should feed:

- capture generation
- thread discovery
- project tagging
- synthesis inputs

## Transcript Schema

```
assistant_transcripts
---------------------
id
source
conversation_id
timestamp
role
content
metadata_json
created_at
```

Indexes:

```
conversation_id
timestamp
```

---
# 7. Transcript Processing

When transcripts are ingested Vel should attempt:

- project detection
- thread linking
- capture extraction
- commitment suggestion
- idea clustering

However, these should be **suggestions**, not automatic commitments.

Example suggestion:

“Conversation about Vel risk model appears related to project ‘Vel Core’. Link?”

---
# 8. Project Relevance Scoring

Vel should attempt to infer which projects are active in conversation history.

Signals:

- repeated project mentions
- code references
- capture tags
- thread associations

This allows Vel to produce artifacts like:

```
Active ideation threads
Projects with recent design discussion
Ideas not yet converted into commitments
```

---
# 9. Voice Interface (Future)

Vel should support a future voice interaction mode.

Voice interactions may be converted into transcripts and stored in the same transcript schema.

Example pipeline:

```
voice_input → transcription → transcript signal → processing
```

Voice feedback prompts could include:

- “Was that helpful?”
- “Do you want to adjust this reminder?”

Voice should **not change the core architecture**, only the input channel.

---
# 10. Self-Review Artifacts

Vel should periodically produce reflective artifacts including:

### Weekly Vel Self Review

Example artifact:

```
vel_self_review_weekly
```

Possible sections:

- nudge effectiveness
- ignored nudges
- suggestions accepted/rejected
- contexts where Vel was helpful
- contexts where Vel lacked enough information
- proposed policy adjustments

---
# 11. Dogfooding Loop

Vel should help improve itself through:

1. capturing design ideas about Vel
2. tagging them to the Vel project
3. synthesizing them into backlog items
4. suggesting implementation commitments

Example artifact:

```
vel_improvement_backlog
```

Sources:

- transcripts
- captures
- rejected suggestions
- feedback signals

---
# 12. Design Principles

The following principles must guide this subsystem.

### Principle 1 — Vel remains stateful

Chat interactions must connect to:

- threads
- commitments
- captures
- projects

### Principle 2 — Vel learns from feedback

Feedback signals must influence:

- nudge timing
- escalation policy
- suggestion frequency

### Principle 3 — Suggestions remain steerable

Vel must not automatically convert ideation into commitments without user confirmation.

### Principle 4 — Reflection is separate from operations

Real-time operations remain deterministic.

Self-analysis and pattern discovery may use LLM synthesis.

---
# 13. Implementation Order

The following order is recommended:

1. transcript ingestion schema
2. feedback signal ingestion
3. vel_self_metrics storage
4. transcript linking to threads
5. feedback prompts for nudges
6. weekly Vel self-review artifact

Voice interface can be added later without schema changes.

---
# 14. Expected Outcomes

When implemented, Vel should be able to:

- track whether its nudges work
- learn which interventions are effective
- capture ideation from assistant conversations
- connect ideas with projects and commitments
- improve its own policies through reflective synthesis

This subsystem enables **Vel to improve Vel** while remaining grounded in observable signals.
