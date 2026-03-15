# vel — Explicit Implementation Brief for the Next Phase
Version: 0.1  
Status: Implementation directive  
Audience: Coding agent / implementation team  
Scope: Commitments, source ingestion, signals/inferences/nudges, first synthesis workflows

---

# 1. Objective

Implement the next phase of Vel so it becomes practically useful for daily dogfooding.

This phase should prioritize:

1. **commitments** as first-class objects
2. **source ingestion** from calendar, Todoist/Reminders-style task sources, and computer activity
3. **signals, inferred state, and nudges**
4. **weekly and project synthesis** as run-backed workflows

The purpose is not to broaden Vel into a generic productivity platform.

The purpose is to make Vel good at:

- knowing what matters today
- keeping unresolved commitments visible
- detecting drift
- nudging gently and explicitly
- helping the user improve Vel from inside Vel

---

# 2. Product framing

Treat Vel as:

> a day-centered, commitment-aware, context-sensitive assistant with gentle escalation

Vel is **not**:

- a PKM replacement
- the primary task manager
- the primary calendar
- a generic workflow engine
- a general-purpose chat shell

Vel should integrate with external systems for source truth where appropriate, while maintaining its own durable runtime model.

---

# 3. Implementation order

Implement in this order:

## Phase A
Commitments

## Phase B
Source ingestion:
- calendar
- external tasks
- computer activity

## Phase C
Signals, inferred state, nudges

## Phase D
Synthesis:
- week
- project vel

Do not start nudges or synthesis before commitments and source ingestion exist in minimally usable form.

---

# 4. Commitments — exact implementation spec

## 4.1 Goal

Introduce a first-class **commitment** object representing something the user needs to remember, do, prepare for, or respond to.

A commitment is distinct from a capture.

- a **capture** is raw input
- a **commitment** is actionable / reviewable / statusful

## 4.2 Schema

Add a new table:

```sql
CREATE TABLE commitments (
  id TEXT PRIMARY KEY,
  text TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_id TEXT,
  status TEXT NOT NULL,
  due_at TEXT,
  project TEXT,
  commitment_kind TEXT,
  created_at TEXT NOT NULL,
  resolved_at TEXT,
  metadata_json TEXT NOT NULL
);
```

### Required indexes

```sql
CREATE INDEX idx_commitments_status ON commitments(status);
CREATE INDEX idx_commitments_due_at ON commitments(due_at);
CREATE INDEX idx_commitments_project ON commitments(project);
CREATE INDEX idx_commitments_source ON commitments(source_type, source_id);
CREATE INDEX idx_commitments_created_at ON commitments(created_at);
```

## 4.3 Field meanings

- `id`: stable id
- `text`: human-facing description
- `source_type`: origin category
- `source_id`: id in originating system if any
- `status`: open/done/cancelled
- `due_at`: optional due timestamp
- `project`: optional project bucket
- `commitment_kind`: optional type such as `todo`, `medication`, `reply`, `prep`, `followup`
- `created_at`: creation timestamp
- `resolved_at`: set when done/cancelled
- `metadata_json`: structured extension data

## 4.4 Allowed statuses

Only these statuses in v1:

- `open`
- `done`
- `cancelled`

Do **not** add `snoozed` as a commitment status. Snoozing belongs to nudges, not commitments.

## 4.5 Domain type

Add `Commitment` to `vel-core`.

Required fields in the domain type should mirror the table and use structured JSON for metadata.

## 4.6 API

Add endpoints:

### Create commitment
`POST /v1/commitments`

Request body:

```json
{
  "text": "take meds",
  "source_type": "manual",
  "source_id": null,
  "due_at": null,
  "project": null,
  "commitment_kind": "medication",
  "metadata": {}
}
```

### List commitments
`GET /v1/commitments`

Support filters:

- `status`
- `project`
- `kind`
- `due_before`
- `due_after`
- `limit`

At minimum, implement:
- `GET /v1/commitments?status=open`

### Inspect commitment
`GET /v1/commitments/:id`

### Update commitment
`PATCH /v1/commitments/:id`

Allowed updates in v1:

- `status`
- `due_at`
- `project`
- `commitment_kind`
- `metadata`

Setting `status=done` or `status=cancelled` should set `resolved_at` automatically.

## 4.7 CLI

Implement:

```bash
vel commitments
vel commitments --open
vel commitments --project vel
vel commitment add "take meds" --kind medication
vel commitment done <id>
vel commitment cancel <id>
vel commitment inspect <id>
```

### Required behavior

`vel commitments` defaults to open commitments, newest/most relevant first.

`vel commitment done <id>` maps to the PATCH endpoint and marks the commitment done.

## 4.8 Commitment creation rule

### v1 rule

A commitment is created in one of three ways:

1. explicit command:
   - `vel commitment add ...`
2. capture promotion:
   - any capture with `capture_type == "todo"` auto-creates a commitment
3. external source import:
   - Todoist/task source import creates commitments

### Important

Do **not** implement fuzzy LLM-based extraction from arbitrary captures in this phase. Too early, too mushy.

Keep the creation rule explicit and inspectable.

---

# 5. Source ingestion — exact implementation spec

The first external sources are:

1. calendar
2. external tasks (Todoist first, architecture supports Reminders later)
3. computer activity

These should be implemented as **source ingestion jobs** run by `veld`.

## 5.1 Calendar — v1 scope

### v1 decision

Calendar v1 is **read-only** and **pull-based**.

Use one configured calendar source. Prefer a simple initial integration path.

If existing calendar connector work is not available inside the repo, use:

- `.ics URL` in config, or
- one local ICS file path

Pick one and document it.

### Recommended choice for v1

Use:

```toml
[calendar]
ics_url = "..."
poll_interval_minutes = 15
```

If URL fetching is too much for immediate implementation, allow a local path variant in config as fallback.

## 5.2 Calendar schema

Add table:

```sql
CREATE TABLE calendar_events (
  id TEXT PRIMARY KEY,
  source_type TEXT NOT NULL,
  external_id TEXT NOT NULL,
  title TEXT NOT NULL,
  start_at TEXT NOT NULL,
  end_at TEXT,
  location TEXT,
  travel_minutes INTEGER,
  prep_minutes INTEGER,
  metadata_json TEXT NOT NULL,
  imported_at TEXT NOT NULL,
  UNIQUE(source_type, external_id)
);
```

### Required indexes

```sql
CREATE INDEX idx_calendar_events_start_at ON calendar_events(start_at);
CREATE INDEX idx_calendar_events_external ON calendar_events(source_type, external_id);
```

## 5.3 Calendar semantics

Required fields used by Vel:

- title
- start_at
- end_at
- location
- travel_minutes
- prep_minutes

### v1 defaults

If `travel_minutes` and `prep_minutes` are not supplied externally:

- `travel_minutes = 0`
- `prep_minutes = 15`

This is crude but usable.

Later you can improve with richer parsing or maps integration.

## 5.4 External tasks — v1 scope

### v1 decision

Use one task source adapter with architecture that allows others later.

If Todoist is the immediate real source, implement Todoist first, but design the ingestion layer as:

```text
task source adapter -> normalized external task record -> commitment upsert
```

### v1 sync mode

Pull-based polling is sufficient.

No webhooks needed in v1.

## 5.5 External task schema

Add table:

```sql
CREATE TABLE external_tasks (
  id TEXT PRIMARY KEY,
  source_type TEXT NOT NULL,
  external_id TEXT NOT NULL,
  text TEXT NOT NULL,
  due_at TEXT,
  completed_at TEXT,
  labels_json TEXT NOT NULL,
  metadata_json TEXT NOT NULL,
  imported_at TEXT NOT NULL,
  UNIQUE(source_type, external_id)
);
```

### Required indexes

```sql
CREATE INDEX idx_external_tasks_source ON external_tasks(source_type, external_id);
CREATE INDEX idx_external_tasks_due_at ON external_tasks(due_at);
CREATE INDEX idx_external_tasks_completed_at ON external_tasks(completed_at);
```

## 5.6 External task normalization rule

On import, each open external task should upsert a corresponding commitment.

### Mapping rules

- `source_type = "todoist"` or similar
- `source_id = external_tasks.id`
- `text = external task text`
- `due_at = external task due_at`
- `status = open` if external task incomplete
- `status = done` if external task completed
- `commitment_kind` inferred only from simple deterministic rule:
  - if text or label matches configured medication identifiers → `medication`
  - else `todo`

### Important

Do not rely on LLM interpretation for this phase.

## 5.7 Computer activity — v1 scope

Computer activity is a local signal source, not a remote source of truth.

### v1 decision

Implement minimal ingestion first:

- shell start / explicit `vel` invocation
- optional login timestamp if easy
- first activity of day

Do **not** attempt full activity tracking or surveillance nonsense.

## 5.8 Computer activity schema

Add table:

```sql
CREATE TABLE activity_signals (
  id TEXT PRIMARY KEY,
  signal_type TEXT NOT NULL,
  signal_at TEXT NOT NULL,
  source TEXT NOT NULL,
  metadata_json TEXT NOT NULL
);
```

### signal_type values in v1

- `shell_invoked`
- `workstation_login`
- `first_activity_today`

## 5.9 Ingestion execution model

All sources should support:

- on-demand sync command
- periodic polling via `veld`

### Required CLI

```bash
vel sync calendar
vel sync tasks
vel sync activity
vel sync all
```

### Daemon behavior

`veld` may also poll on interval where configured.

For v1, polling cadence:

- calendar: every 15 minutes
- tasks: every 10 minutes
- activity: local event write or on-demand

---

# 6. Morning planning — exact implementation spec

## 6.1 Goal

Make `vel morning` and eventually bare `vel` reflect:

- next scheduled commitment / event
- prep window
- commute window
- unresolved commitments
- meds status if relevant

## 6.2 Morning context inputs

Morning context generation should now consume:

- calendar events for today
- open commitments
- recent captures
- external tasks
- activity signals

## 6.3 Derived fields

Compute:

- `first_calendar_event`
- `prep_start_at = start_at - prep_minutes`
- `leave_by_at = prep_start_at - travel_minutes` OR `start_at - travel_minutes` depending on workflow
- `open_commitments_due_today`
- `meds_commitment_status`

### v1 rule

Use:

```text
prep_start_at = event.start_at - prep_minutes
leave_by_at = event.start_at - travel_minutes
```

Keep them separate.

## 6.4 Morning output shape

Update the morning artifact / payload to include:

```json
{
  "date": "2026-03-14",
  "first_event": {
    "title": "Meeting",
    "start_at": "...",
    "prep_start_at": "...",
    "leave_by_at": "...",
    "location": "Salt Lake City"
  },
  "open_commitments": [...],
  "meds_status": {
    "logged": false,
    "source": "todoist"
  },
  "signals_summary": {...}
}
```

Keep it structured and modest.

---

# 7. Signals, inferred state, nudges — exact implementation spec

## 7.1 Goal

Implement the first proactive subsystem using explicit signals and explainable rules.

## 7.2 First signal set

Signals are facts, not guesses.

### Required signal types in v1

- `calendar_event_upcoming`
- `external_task_open`
- `external_task_done`
- `shell_invoked`
- `workstation_login`
- `first_activity_today`

Optional later:
- watch state
- presence sensors
- smart speaker acknowledgement

Do not implement those in this phase.

## 7.3 First inferred states

Add explicit inferred states, probably represented in code first and persisted as part of nudge evaluation or state snapshots if useful.

Required inferred states:

- `meds_logged`
- `first_event_upcoming`
- `prep_window_active`
- `morning_started`
- `morning_drift`
- `behind_schedule`

### Suggested rules

#### meds_logged
True if:
- there exists an open medication commitment that is done today
or
- corresponding external task completed today

#### first_event_upcoming
True if:
- there is a calendar event today with start_at in the future and earliest among today’s events

#### prep_window_active
True if:
- `now >= prep_start_at`
and
- event has not started

#### morning_started
True if any of:
- workstation_login today
- shell_invoked today
- first_activity_today exists
- medication logged today

Use simple OR logic in v1.

#### morning_drift
True if:
- current time is later than configured morning drift threshold
and
- morning_started == false

#### behind_schedule
True if:
- prep_window_active == true
and
- required relevant commitments unresolved
or
- leave_by_at passed and event not yet started

Keep this conservative.

## 7.4 Nudge schema

Add table:

```sql
CREATE TABLE nudges (
  id TEXT PRIMARY KEY,
  nudge_type TEXT NOT NULL,
  state TEXT NOT NULL,
  triggered_at TEXT NOT NULL,
  snoozed_until TEXT,
  resolved_at TEXT,
  commitment_id TEXT,
  related_run_id TEXT,
  signals_snapshot_json TEXT NOT NULL,
  inference_snapshot_json TEXT NOT NULL,
  message TEXT NOT NULL,
  metadata_json TEXT NOT NULL
);
```

### Required indexes

```sql
CREATE INDEX idx_nudges_state ON nudges(state);
CREATE INDEX idx_nudges_type ON nudges(nudge_type);
CREATE INDEX idx_nudges_triggered_at ON nudges(triggered_at);
CREATE INDEX idx_nudges_snoozed_until ON nudges(snoozed_until);
```

## 7.5 Allowed nudge states

Exactly these:

- `pending`
- `active`
- `snoozed`
- `resolved`

## 7.6 First nudge types

Implement only these three in v1:

- `meds_not_logged`
- `prep_window_approaching`
- `morning_drift`

## 7.7 Nudge evaluation timing

### v1 decision

Evaluate nudges in two places:

1. on demand when user runs `vel`, `vel morning`, or a dedicated nudge command
2. daemon loop every N minutes if `veld` is running

### v1 interval

Use:
- evaluation every 5 minutes in daemon loop

That is enough for now.

## 7.8 Nudge surfacing

### v1 scope

Support:
- CLI output
- persisted nudge records
- API endpoints

Do **not** implement watch or desktop notifications in this phase unless they already exist nearly for free.

The data model and evaluation system should come first.

## 7.9 Nudge API

Add endpoints:

- `GET /v1/nudges?state=active`
- `GET /v1/nudges/:id`
- `POST /v1/nudges/:id/done`
- `POST /v1/nudges/:id/snooze`

### Snooze request body

```json
{
  "minutes": 10
}
```

Default to 10 minutes if omitted.

## 7.10 Nudge CLI

Add:

```bash
vel nudges
vel nudge done <id>
vel nudge snooze <id> --minutes 10
```

### Required behavior

`vel nudges` shows currently active nudges first.

## 7.11 Nudge generation rules

### Rule 1 — meds_not_logged

Trigger if:
- there exists an open medication commitment due today or marked recurring
- `meds_logged == false`
- current time is later than a configured expected meds time or later than morning drift threshold

### Rule 2 — prep_window_approaching

Trigger if:
- first event exists today
- current time is within configurable lead window before `prep_start_at`

Recommended default lead window:
- 15 minutes before prep start

### Rule 3 — morning_drift

Trigger if:
- current time is later than configured morning drift threshold
- `morning_started == false`

### Important

On each evaluation, either:
- create a new nudge if none exists in active/snoozed state for that type/context
or
- keep/update the existing one

Avoid duplicate nudge spam.

## 7.12 Done / Snooze protocol

This is the official interaction contract.

Every surfaced nudge resolves through one of two user actions:

- `done`
- `snooze`

No other action states are needed in v1.

### Done semantics

- mark nudge `resolved`
- if linked commitment exists, mark commitment done if appropriate **only when this is semantically correct**
- record `resolved_at`

### Snooze semantics

- set nudge state to `snoozed`
- set `snoozed_until = now + duration`

When `snoozed_until` passes, the nudge may become active again on next evaluation if the condition still holds.

---

# 8. Synthesis — exact implementation spec

## 8.1 Goal

Implement the first run-backed synthesis workflows that help the user reflect and improve Vel from inside Vel.

## 8.2 Run kind decision

Use:

- `RunKind::Synthesis`

Do **not** create separate run kinds for week vs project.

Instead, use one synthesis run kind with `synthesis_kind` in input payload.

## 8.3 Input payload shape

### Week synthesis

```json
{
  "synthesis_kind": "week",
  "window_days": 7
}
```

### Project synthesis

```json
{
  "synthesis_kind": "project",
  "project": "vel",
  "window_days": 14
}
```

## 8.4 Inputs

### Week synthesis uses:
- captures from last 7 days
- open commitments
- context artifacts from last 7 days
- active/recent nudges if available

### Project vel synthesis uses:
- captures matching project == vel or containing "vel" if project field absent
- open commitments for project vel
- relevant review/context artifacts
- recent runs/artifacts referencing vel if easily available

### Important

Keep matching deterministic in v1. Do not build elaborate semantic retrieval here yet.

## 8.5 Output artifact type

Use:
- `artifact_type = "synthesis_brief"`
- `storage_kind = managed`
- canonical representation = JSON

Markdown or human-readable renderings can come later.

## 8.6 Output JSON shape

### Week synthesis artifact

```json
{
  "synthesis_kind": "week",
  "window_days": 7,
  "recurring_themes": [],
  "unresolved_commitments": [],
  "repeated_pain_points": [],
  "project_imbalance": [],
  "suggested_priorities": []
}
```

### Project vel synthesis artifact

```json
{
  "synthesis_kind": "project",
  "project": "vel",
  "window_days": 14,
  "repeated_friction": [],
  "unresolved_issues": [],
  "candidate_next_steps": [],
  "suggested_priorities": []
}
```

## 8.7 CLI

Add:

```bash
vel synthesize week
vel synthesize project vel
```

These must be run-backed.

## 8.8 API

Add endpoints:

- `POST /v1/synthesis/week`
- `POST /v1/synthesis/project`

or a single generalized endpoint if it fits current API style better:

- `POST /v1/synthesis`

with input payload indicating kind

Use whichever is more consistent with current codebase conventions.

## 8.9 Implementation note

The actual synthesis generation logic can initially be deterministic / template-driven and placeholder-ish if needed, but the workflow must be:

```text
request -> run -> artifact -> refs -> inspection
```

Do not bypass the runtime spine.

---

# 9. Documentation work required

Create or update these docs:

## 9.1 New spec
`docs/specs/commitments.md`

Must include:
- schema
- API
- CLI
- creation rules
- source integration role

## 9.2 New spec
`docs/specs/signals-inference-nudges.md`

Must include:
- signal types
- inferred states
- nudge types
- done/snooze protocol
- evaluation timing
- surfacing

## 9.3 Update
`docs/data-model.md`

Add:
- commitments
- calendar_events
- external_tasks
- activity_signals
- nudges

## 9.4 Update
`docs/roadmap.md`

Replace stale early-phase items with:
- commitments
- source ingestion
- nudge engine
- synthesis
- trust/export improvements

## 9.5 Update
`docs/status.md`

Track:
- commitments implemented / partial
- calendar ingestion implemented / partial
- external tasks implemented / partial
- nudges implemented / partial
- synthesis implemented / partial

---

# 10. Testing requirements

## 10.1 Commitments tests

Required:
- create manual commitment
- create commitment via todo capture
- list open commitments
- mark commitment done
- mark commitment cancelled

## 10.2 Source ingestion tests

Required:
- calendar import inserts/updates calendar_events
- external task import inserts/updates external_tasks
- open task creates or updates commitment
- completed external task marks linked commitment done
- computer activity ingestion records signals

## 10.3 Nudge tests

Required:
- meds_not_logged nudge appears when expected
- prep_window_approaching nudge appears at proper threshold
- morning_drift nudge appears when no activity exists
- snooze suppresses retrigger until expiration
- done resolves nudge and closes relevant loop where appropriate
- duplicate active nudges are not created for same condition/context

## 10.4 Synthesis tests

Required:
- week synthesis creates run
- week synthesis writes artifact
- project vel synthesis creates run
- synthesis artifacts linked via refs
- run inspect shows synthesis artifacts

---

# 11. Suggested implementation sequence in code

## Step 1
Add commitments schema and API/CLI

## Step 2
Implement capture promotion rule for `capture_type == todo`

## Step 3
Add external task ingestion and commitment upsert

## Step 4
Add calendar ingestion and morning context use of prep/leave fields

## Step 5
Add activity signals ingestion

## Step 6
Add nudge schema, evaluation logic, API/CLI, done/snooze

## Step 7
Add synthesis workflows

## Step 8
Update docs and tests comprehensively

---

# 12. Hard constraints

These constraints are mandatory.

## Constraint 1
No fuzzy LLM-based commitment extraction in this phase.

## Constraint 2
No watch / desktop notification adapter work before the nudge model exists cleanly in data and CLI/API.

## Constraint 3
All synthesis flows must be run-backed.

## Constraint 4
Nudges must be inspectable and explainable from persisted signal/inference snapshots.

## Constraint 5
Capture must remain low-friction. Do not force structured fields at capture time except optional explicit type.

---

# 13. Definition of done for this phase

This phase is complete when all of the following are true:

- commitments exist as first-class objects
- todo captures create commitments
- one external task source can sync into commitments
- one calendar source can sync into calendar events
- activity signals can be recorded
- morning context uses commitments + calendar timing
- three nudge types exist and can be done/snoozed
- synthesis week and synthesis project vel are run-backed
- docs and tests reflect the new phase accurately

---

# 14. Final instruction to the coding agent

Prioritize **clarity over breadth**.

The correct implementation is not the one with the most integrations or the most abstractions. It is the one that makes Vel genuinely useful in the smallest real loop:

- know what matters today
- know what is unresolved
- know when drift begins
- nudge gently
- reflect weekly
- use those reflections to improve Vel

That loop is the product.