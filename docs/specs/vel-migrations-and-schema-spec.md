# vel_migrations_and_schema_spec.md

Status: Canonical schema and migration reference  
Audience: coding agent / implementation lead  
Purpose: define the durable data model, migration order, relationships, and indexing strategy for Vel’s next implementation phases

---

# 1. Schema Philosophy

Vel should be built around a small number of durable, inspectable entities.

The schema must support:

- append-only signals/events
- persistent current context
- commitments and dependencies
- risk snapshots
- nudges and acknowledgements
- suggestions / steering
- threads / unfinished thread graphs
- artifacts and provenance
- runs / run_events
- future reflective synthesis

The schema should favor:

- explicit relationships
- append-only logs where possible
- inspectability
- incremental recomputation
- future extensibility without ontology soup

---

# 2. Existing Foundations

The following are already present or partially present in the repo and should be preserved / extended rather than duplicated:

- `captures`
- `artifacts`
- `refs`
- `runs`
- `run_events`
- `commitments`

Agents must inspect existing migrations and extend them carefully rather than introducing duplicate concepts with near-identical meaning.

---

# 3. Migration Order

Implement future migrations in this order.

## Migration 0009 — signals

Purpose: durable log of normalized external/internal facts

## Migration 0010 — current_context + context_timeline

Purpose: persistent live inferred state and timeline snapshots

## Migration 0011 — commitment_dependencies + commitment_risk

Purpose: dependency graph and risk snapshots

## Migration 0012 — nudges + nudge_events

Purpose: durable nudge state, escalation history, done/snooze protocol

## Migration 0013 — suggestions

Purpose: steerable proposed adjustments and operational suggestions

## Migration 0014 — threads + thread_links

Purpose: unfinished threads / person/project/conversation graphs

## Migration 0015 — synthesis_artifacts or synthesis_runs helpers (optional)

Purpose: convenience indexing / linkage for weekly synthesis and reflective artifacts if needed after implementation experience

Do not jump ahead unless a prior migration is implemented and stabilized.

---

# 4. Table Specifications

## 4.1 signals

Purpose: append-only log of normalized facts from adapters and internal events.

### Table

```sql
CREATE TABLE signals (
  id TEXT PRIMARY KEY,
  signal_type TEXT NOT NULL,
  source TEXT NOT NULL,
  source_ref TEXT,
  timestamp INTEGER NOT NULL,
  payload_json TEXT NOT NULL,
  created_at INTEGER NOT NULL
);
```

### Notes

- `id`: UUID/ULID/string consistent with repo conventions
- `signal_type`: e.g. `calendar_event`, `todoist_task`, `shell_login`
- `source`: system origin, e.g. `calendar`, `todoist`, `activity`, `vel`
- `source_ref`: optional external identifier such as event id or task id
- `timestamp`: when the fact occurred
- `created_at`: when Vel stored it
- `payload_json`: normalized signal body

### Indexes

```sql
CREATE INDEX idx_signals_type_timestamp ON signals(signal_type, timestamp DESC);
CREATE INDEX idx_signals_source_timestamp ON signals(source, timestamp DESC);
CREATE INDEX idx_signals_source_ref ON signals(source, source_ref);
CREATE INDEX idx_signals_created_at ON signals(created_at DESC);
```

### Deduplication strategy

Prefer deduplication in adapter logic.  
If needed, add a unique index later on a stable dedupe key. Do **not** prematurely hardcode brittle uniqueness before observing real ingestion patterns.

---

## 4.2 current_context

Purpose: single latest durable context snapshot.

### Table

```sql
CREATE TABLE current_context (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  computed_at INTEGER NOT NULL,
  context_json TEXT NOT NULL
);
```

### Notes

- singleton table; one row only
- `id = 1` enforces singleton semantics
- `context_json` contains current best inferred state

### Suggested JSON shape

```json
{
  "morning_state": "underway",
  "next_commitment_id": "com_123",
  "prep_window_active": true,
  "commute_window_active": false,
  "meds_status": "pending",
  "active_nudge_ids": ["nud_001"],
  "current_risk_level": 0.72,
  "inferred_activity": "computer_active",
  "open_thread_ids": ["thr_45"]
}
```

---

## 4.3 context_timeline

Purpose: append-only snapshots of meaningful context transitions over time.

### Table

```sql
CREATE TABLE context_timeline (
  id TEXT PRIMARY KEY,
  timestamp INTEGER NOT NULL,
  context_json TEXT NOT NULL,
  trigger_signal_id TEXT,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (trigger_signal_id) REFERENCES signals(id)
);
```

### Notes

This table allows:

- context over the day
- debugging state transitions
- later temporal synthesis

### Indexes

```sql
CREATE INDEX idx_context_timeline_timestamp ON context_timeline(timestamp DESC);
CREATE INDEX idx_context_timeline_trigger_signal ON context_timeline(trigger_signal_id);
```

---

## 4.4 commitment_dependencies

Purpose: dependency graph among commitments.

### Table

```sql
CREATE TABLE commitment_dependencies (
  id TEXT PRIMARY KEY,
  parent_commitment_id TEXT NOT NULL,
  child_commitment_id TEXT NOT NULL,
  dependency_type TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (parent_commitment_id) REFERENCES commitments(id),
  FOREIGN KEY (child_commitment_id) REFERENCES commitments(id)
);
```

### Dependency type examples

- `requires`
- `prep_for`
- `commute_for`
- `blocks`
- `subtask_of`

### Constraints

```sql
CREATE UNIQUE INDEX idx_commitment_dependency_unique
ON commitment_dependencies(parent_commitment_id, child_commitment_id, dependency_type);
```

### Indexes

```sql
CREATE INDEX idx_commitment_dependencies_parent ON commitment_dependencies(parent_commitment_id);
CREATE INDEX idx_commitment_dependencies_child ON commitment_dependencies(child_commitment_id);
```

---

## 4.5 commitment_risk

Purpose: record risk score snapshots for commitments over time.

### Table

```sql
CREATE TABLE commitment_risk (
  id TEXT PRIMARY KEY,
  commitment_id TEXT NOT NULL,
  risk_score REAL NOT NULL,
  risk_level TEXT NOT NULL,
  factors_json TEXT NOT NULL,
  computed_at INTEGER NOT NULL,
  FOREIGN KEY (commitment_id) REFERENCES commitments(id)
);
```

### Notes

- `risk_score`: numeric score, e.g. 0.0–1.0 or wider if desired
- `risk_level`: e.g. `low`, `medium`, `high`, `critical`
- `factors_json`: explanation ingredients such as consequence, proximity, dependency pressure

### Indexes

```sql
CREATE INDEX idx_commitment_risk_commitment_time ON commitment_risk(commitment_id, computed_at DESC);
CREATE INDEX idx_commitment_risk_level_time ON commitment_risk(risk_level, computed_at DESC);
```

---

## 4.6 nudges

Purpose: durable proactive prompts.

### Table

```sql
CREATE TABLE nudges (
  id TEXT PRIMARY KEY,
  nudge_type TEXT NOT NULL,
  level TEXT NOT NULL,
  state TEXT NOT NULL,
  commitment_id TEXT,
  thread_id TEXT,
  suggestion_id TEXT,
  created_at INTEGER NOT NULL,
  last_sent_at INTEGER,
  snoozed_until INTEGER,
  resolved_at INTEGER,
  metadata_json TEXT NOT NULL,
  FOREIGN KEY (commitment_id) REFERENCES commitments(id),
  FOREIGN KEY (thread_id) REFERENCES threads(id),
  FOREIGN KEY (suggestion_id) REFERENCES suggestions(id)
);
```

### Level examples

- `gentle`
- `warning`
- `danger`

### State examples

- `pending`
- `active`
- `snoozed`
- `resolved`

### Metadata examples

```json
{
  "rule": "meeting_prep_window",
  "channel": "watch",
  "reason": "prep window active and no progress signals"
}
```

### Indexes

```sql
CREATE INDEX idx_nudges_state_created_at ON nudges(state, created_at DESC);
CREATE INDEX idx_nudges_commitment ON nudges(commitment_id);
CREATE INDEX idx_nudges_snoozed_until ON nudges(snoozed_until);
CREATE INDEX idx_nudges_level_state ON nudges(level, state);
```

---

## 4.7 nudge_events

Purpose: append-only event history for nudges.

### Table

```sql
CREATE TABLE nudge_events (
  id TEXT PRIMARY KEY,
  nudge_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  timestamp INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (nudge_id) REFERENCES nudges(id)
);
```

### Event type examples

- `nudge_created`
- `nudge_sent`
- `nudge_snoozed`
- `nudge_escalated`
- `nudge_resolved`
- `nudge_auto_resolved`
- `nudge_channel_failed`

### Indexes

```sql
CREATE INDEX idx_nudge_events_nudge_time ON nudge_events(nudge_id, timestamp DESC);
CREATE INDEX idx_nudge_events_type_time ON nudge_events(event_type, timestamp DESC);
```

---

## 4.8 suggestions

Purpose: steerable proposed adjustments / inferred operational commitments / policy suggestions.

### Table

```sql
CREATE TABLE suggestions (
  id TEXT PRIMARY KEY,
  suggestion_type TEXT NOT NULL,
  state TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  resolved_at INTEGER
);
```

### Suggestion type examples

- `increase_commute_buffer`
- `increase_prep_window`
- `add_operational_commitment`
- `adjust_nudge_timing`
- `schedule_focus_block`

### State examples

- `pending`
- `accepted`
- `rejected`
- `modified`
- `expired`

### Indexes

```sql
CREATE INDEX idx_suggestions_state_created_at ON suggestions(state, created_at DESC);
CREATE INDEX idx_suggestions_type_created_at ON suggestions(suggestion_type, created_at DESC);
```

---

## 4.9 threads

Purpose: first-class unfinished threads / people / projects / conversations / thematic strands.

### Table

```sql
CREATE TABLE threads (
  id TEXT PRIMARY KEY,
  thread_type TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL,
  metadata_json TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
```

### Thread type examples

- `project`
- `person`
- `conversation`
- `theme`
- `logistics`

### Status examples

- `open`
- `dormant`
- `closed`

### Indexes

```sql
CREATE INDEX idx_threads_type_status ON threads(thread_type, status);
CREATE INDEX idx_threads_updated_at ON threads(updated_at DESC);
```

---

## 4.10 thread_links

Purpose: relate threads to entities.

### Table

```sql
CREATE TABLE thread_links (
  id TEXT PRIMARY KEY,
  thread_id TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  relation_type TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (thread_id) REFERENCES threads(id)
);
```

### Entity types

- `commitment`
- `capture`
- `signal`
- `artifact`
- `calendar_event` (if represented directly)
- `suggestion`

### Relation types

- `concerns`
- `originated_from`
- `blocks`
- `associated_with`

### Constraints

```sql
CREATE UNIQUE INDEX idx_thread_link_unique
ON thread_links(thread_id, entity_type, entity_id, relation_type);
```

### Indexes

```sql
CREATE INDEX idx_thread_links_thread ON thread_links(thread_id);
CREATE INDEX idx_thread_links_entity ON thread_links(entity_type, entity_id);
```

---

# 5. Existing Table Extensions

The following existing tables should be extended carefully if needed.

## 5.1 commitments

The existing `commitments` table should support, either now or soon:

- `kind`
- `project`
- `due_at`
- `source_type`
- `source_id`
- `resolved_at`
- `risk_class` (optional later)
- `counterparty_type` or equivalent (self vs external) (optional later)

Do not duplicate commitments under a second table.

## 5.2 runs / run_events

Keep using these for:

- ingestion runs
- context recomputation runs when meaningful
- synthesis runs
- explanation-generating runs if you model them that way

Do not collapse run_events and nudge_events; they serve different scopes.

## 5.3 refs

Continue using `refs` for durable provenance relationships.

Potential examples:
- run → artifact
- artifact → commitment
- artifact → thread
- suggestion → commitment
- synthesis artifact → input thread(s)

Do not invent overlapping provenance tables unless `refs` is clearly insufficient.

---

# 6. Relationships

## 6.1 Core relationships

- signals influence current_context
- current_context influences risk and nudges
- commitments may depend on other commitments
- commitments may link to threads
- nudges may point to commitments, threads, or suggestions
- suggestions may produce or modify commitments
- synthesis artifacts link via refs to commitments, nudges, threads, and signals

## 6.2 Typical chain

```text
signal → current_context → commitment_risk → nudge → nudge_event
```

## 6.3 Reflective chain

```text
signals + commitments + nudges + threads → synthesis run → artifact + refs
```

---

# 7. Migration Rules for the Agent

1. **Do not duplicate concepts already represented by existing tables.**
2. **Do not collapse append-only logs into mutable state tables.**
3. **Prefer additive migrations.**
4. **Use INTEGER Unix timestamps if current repo conventions have standardized on them.**
5. **Add indexes for every lookup path that will be used operationally.**
6. **Use JSON payload columns for flexible detail, but keep core invariants in explicit columns.**
7. **Keep singleton semantics for `current_context`.**
8. **Use unique indexes to prevent accidental duplicate dependency and thread-link rows.**

---

# 8. Implementation Order Mapping

## After Phase A (commitments)

### 0009 signals
Implement `vel-signals`

### 0010 current_context + context_timeline
Implement `vel-context`

### 0011 commitment_dependencies + commitment_risk
Implement dependency graph + risk engine

### 0012 nudges + nudge_events
Implement `vel-nudges`

### 0013 suggestions
Implement steering loop

### 0014 threads + thread_links
Implement unfinished threads graph

### 0015 optional synthesis helper schema
Only if needed after reflective synthesis implementation begins

---

# 9. Query Patterns to Support

The schema must support these operational queries efficiently.

## Signals
- latest calendar events
- latest task completion state
- latest activity signals
- signals by source/type/time range

## Current context
- current singleton context
- timeline across day/week

## Commitments
- open commitments
- commitments due today
- commitments by project
- commitments by thread
- dependency graph for one commitment

## Risk
- highest-risk commitments now
- risk history for one commitment
- commitments whose risk rose sharply

## Nudges
- active nudges
- snoozed nudges
- nudge history for one commitment
- danger nudges in the last day/week

## Suggestions
- pending suggestions
- accepted/rejected history
- suggestions by type

## Threads
- open threads
- threads with most linked commitments
- dormant unresolved threads

---

# 10. Examples

## 10.1 Meeting prep path

Signal:
- calendar event imported

Derived:
- commitment dependency created (`meeting` requires `prep`, `commute`)

Computed:
- risk snapshot recorded

Output:
- nudge created (`meeting_prep_window`, gentle)

History:
- nudge_events append `nudge_created`, `nudge_sent`

## 10.2 Meds reminder path

Signal:
- Todoist snapshot shows meds incomplete

Derived:
- meds commitment open
- current context says morning underway
- risk snapshot updated

Output:
- nudge created (`meds_not_logged`)

Later:
- task completed
- new signal ingested
- nudge auto-resolved
- nudge_event appended

---

# 11. Inspection Expectations

The schema must make these views possible later:

- `vel signals`
- `vel context`
- `vel risk`
- `vel nudges`
- `vel suggestions`
- `vel threads`
- `vel explain ...`

Agents should keep this in mind when designing storage APIs and indexes.

---

# 12. Final Guidance

The schema is supposed to support a system that is:

- event-driven
- inspectable
- stateful
- commitment-centric
- risk-aware
- steerable
- synthesis-capable

If a proposed table does not clearly support one of those properties, it probably does not belong in this phase.