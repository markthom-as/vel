
# Addendum: Calendar, Todoist, Tagging, and Scheduling Integration

This document extends **vel_v1_nudge_spec_and_morning_state_machine.md** by incorporating patterns from the provided Codex workspace. The workspace demonstrates a mature workflow where **calendar events, Todoist tasks, tagging conventions, daily notes, and scheduled automation** work together to maintain situational awareness.  

Vel should adopt these patterns to support real daily operation and to align with the user’s existing cognitive workflow.

---

# 1. Guiding Principle

Vel should **integrate with existing systems rather than replace them**.

External systems remain authoritative:

| Domain | Source of Truth |
|------|------|
| Calendar events | Google / Apple Calendar |
| Tasks | Todoist or Apple Reminders |
| Notes / PKM | Obsidian-style markdown workspace |
| Metrics / health | Apple Health / device metrics |
| Automation | scheduled workflows |

Vel's job is to:

- ingest
- contextualize
- infer
- nudge

---

# 2. Tagging Conventions

The workspace demonstrates strong reliance on **semantic tags embedded in text and files**.

Vel should support lightweight tagging extracted from:

- Todoist task text
- daily notes
- captures
- markdown documents

Example tags:

```
todo:
meds:
vel:
project:
meeting:
commute:
```

Tags should map to Vel concepts:

| Tag | Vel interpretation |
|----|----|
| `todo:` | commitment candidate |
| `meds:` | medication commitment |
| `project:` | thread / project association |
| `meeting:` | calendar-linked obligation |
| `commute:` | travel window required |

Vel should parse these automatically during ingestion.

---

# 3. Calendar Integration Pattern

The workspace shows structured calendar ingestion via a CLI tool and scheduled workflow.

Vel should follow the same pattern:

### Calendar Sync Workflow

```
calendar API
→ sync tool
→ JSON snapshot
→ Vel ingestion
```

Vel should ingest:

```
event_id
title
start_time
end_time
location
calendar_name
tags (derived)
```

### Derived Fields

Vel should compute:

```
prep_start = start_time - prep_minutes
leave_by = start_time - travel_minutes
```

`prep_minutes` and `travel_minutes` may be:

- defaults
- tags
- explicit event metadata

Example tag usage:

```
Meeting with team #prep:30 #travel:40
```

---

# 4. Todoist Integration Pattern

The workspace already stores Todoist snapshots.

Vel should ingest Todoist snapshots rather than hitting the API constantly.

### Snapshot file

```
data/todoist/snapshot.json
```

Vel ingestion job:

```
snapshot.json
→ parse tasks
→ normalize into Vel commitments
```

Fields needed:

```
task_id
text
completed
due_time
labels
project
priority
```

---

# 5. Commitment Extraction

Tasks become Vel commitments.

Example mapping:

Todoist task:

```
"Take meds"
labels: ["health"]
```

Vel commitment:

```
type: medication
source: todoist
state: open
```

Another example:

```
"reply to Dimitri"
labels: ["communication"]
```

Vel commitment:

```
type: communication
thread: Dimitri
```

---

# 6. Daily Notes Integration

The workspace contains:

```
daily/YYYY-MM-DD.md
```

Vel should treat daily notes as contextual inputs.

Possible signals extracted:

```
check-in status
priorities
explicit TODO lines
```

Example pattern:

```
TODO: finish grant budget
```

Vel could parse:

```
TODO:
ACTION:
FOLLOWUP:
```

These can become soft commitments.

---

# 7. Scheduled Automation

The workspace demonstrates scheduled jobs defined in YAML.

Example:

```
schedules/dashboard.yaml
schedules/inbox-sweep.yaml
schedules/healthkit-refresh.yaml
```

Vel should support similar scheduled ingestion.

### Example Vel scheduler

```
calendar_sync      every 5 minutes
todoist_snapshot   every 10 minutes
metrics_refresh    every hour
daily_context_run  every morning
```

Vel does not need a full scheduler immediately but should support **cron-like workflows**.

---

# 8. Project Detection

The workspace contains logic for detecting projects within notes.

Vel should mirror this idea.

Projects may be inferred from:

```
tags
Todoist project
directory names
repeated captures
```

Example:

```
#project:vel
#project:onassis
#project:mimesis
```

Vel threads should be linked to projects automatically.

---

# 9. Metrics and Context Signals

The workspace includes ingestion of:

```
healthkit
screentime
chrome history
github metrics
```

These should not drive nudges initially, but they can provide context for later synthesis.

Example insights Vel could later produce:

```
You tend to miss morning prep when sleep < 6h.
You snooze commitments more often on heavy meeting days.
```

These signals should be stored but not yet heavily interpreted.

---

# 10. Inbox Workflow

The workspace includes an `inbox/` directory for unsorted notes.

Vel should support this pattern.

Possible flow:

```
capture → inbox
daily process → classify
classified → commitments / threads
```

This supports low-friction capture while allowing later organization.

---

# 11. Vel Workflow Alignment

With this addendum applied, Vel's operational loop becomes:

```
calendar sync
todoist snapshot
daily note ingestion
→ signals

signals
→ inferred state

inferred state
→ nudges

user responses
→ commitments updated

weekly synthesis
→ insights
```

---

# 12. Implementation Adjustments

The coding agent implementing Vel should now:

1. Add parsers for Todoist snapshot files
2. Add parsers for calendar JSON exports
3. Add tag extraction from text fields
4. Support ingestion of daily markdown notes
5. Support scheduled ingestion jobs
6. Map tags to Vel domain objects
7. Maintain links between commitments, calendar events, and notes

---

# 13. Minimal Implementation Priority

For the first iteration integrating these patterns:

1. Calendar ingestion
2. Todoist snapshot ingestion
3. Tag extraction
4. Commitment extraction
5. Nudge generation

Everything else can follow.

---

# 14. Summary

This addendum aligns Vel with an already functioning workflow ecosystem built around:

- markdown notes
- calendar events
- Todoist tasks
- scheduled ingestion
- semantic tagging

Vel should act as the **context and inference layer** above these systems rather than duplicating them.

This approach preserves compatibility with existing tools while enabling Vel to provide the higher-level behavior it is designed for:

**context awareness, drift detection, and intelligent nudging.**
