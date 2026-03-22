# Workflow Context Binding: project, task, nudge, thread

## Why explicit context binding matters

Without context binding, automation becomes uncanny sludge: it runs, it writes, and nobody can clearly answer “on behalf of what exactly?”

Every workflow run should be attached to a primary locus and may optionally attach to neighboring loci.

## Primary context anchors

### Project context
Used when the workflow concerns a durable domain of work.

Examples:
- weekly project review
- drift detection
- stale project reactivation
- project-level standup generation

### Task context
Used when the workflow concerns a specific actionable item.

Examples:
- missing metadata enrichment
- deadline rescue workflow
- post-completion wrap-up

### Nudge context
Used when the workflow concerns reminders, prompts, soft interventions, or escalation chains.

Examples:
- nudge aging
- repeated ignored nudge escalation
- convert nudge to task/project note

### Thread context
Used when the workflow concerns a conversational or deliberative object.

Examples:
- follow-up from voice capture
- convert discussion into commitments
- summarize and branch into tasks

## Context resolution rules

The runtime should support declarative resolution strategies.

Examples:

```yaml
contextBinding:
  required:
    - user
    - workspace
  optional:
    - project
    - task
    - nudge
    - thread
  resolution:
    project: nearest_active
    task: trigger_subject_or_due_soon
    nudge: attach_open_or_create
    thread: resume_by_topic_or_create
```

Suggested strategies:

- `trigger_subject`
- `trigger_subject_or_fail`
- `nearest_active`
- `resume_or_create`
- `attach_open`
- `attach_open_or_create`
- `due_soon`
- `active_or_due_soon`
- `topic_match`
- `explicit_only`

## Context packet shape

A bound workflow run should receive typed context buckets.

```json
{
  "user": {"id": "usr_1"},
  "workspace": {"id": "ws_main"},
  "project": {"id": "proj_vel", "name": "Vel"},
  "task": {"id": "tsk_42", "title": "Implement workflow runtime"},
  "thread": {"id": "thr_88", "title": "Morning planning"},
  "nudge": {"id": "ndg_2", "kind": "soft-reminder"},
  "time": {"timezone": "America/Denver", "localDate": "2026-03-21"},
  "policy": {...},
  "memory": {...}
}
```

## UI implications

Because context binding is explicit, UI can show:

- this workflow ran for **Project: Vel**
- it attached to **Thread: Morning planning**
- it created **3 nudges** and proposed **2 task changes**
- it paused at **Human approval gate**

That is what makes automation legible rather than spooky.
