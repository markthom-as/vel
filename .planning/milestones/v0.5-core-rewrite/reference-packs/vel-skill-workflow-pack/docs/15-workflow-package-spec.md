# Workflow Package Specification

## Manifest goals

The workflow manifest should declare:

- identity and version
- trigger bindings
- execution contexts
- interfaces (input/output schemas)
- step graph or ordered stages
- required capabilities and connectors
- policy rules and confirmation gates
- retry / resume / timeout limits
- observability configuration
- UI surfaces

## Recommended manifest file: `workflow.yaml`

```yaml
apiVersion: vel/v1
kind: Workflow

metadata:
  name: morning-orientation
  namespace: core
  version: 0.1.0
  displayName: Morning Orientation
  description: Triggered workflow that runs daily planning and nudge generation in a day/thread/project-aware context.
  tags: [workflow, planning, morning, automation]

spec:
  mode: orchestrated

  entry:
    prompt: ./prompt.md
    onStart: ./hooks/on_start.ts
    onComplete: ./hooks/on_complete.ts
    onError: ./hooks/on_error.ts

  inputSchema: ./input.schema.json
  outputSchema: ./output.schema.json

  triggers:
    - type: schedule
      id: weekday-morning
      enabled: true
      schedule:
        timezone: user
        cron: "0 8 * * 1-5"
    - type: manual
      id: manual-run
      enabled: true
    - type: event
      id: app-day-start
      enabled: true
      event:
        name: app.day_started

  contextBinding:
    required:
      - user
      - workspace
    optional:
      - project
      - task
      - thread
      - nudge
      - scheduleWindow
    resolution:
      project: nearest_active
      task: active_or_due_soon
      thread: resume_or_create
      nudge: attach_open

  steps:
    strategy: graph
    nodes:
      - id: load_context
        type: hook
        ref: ./hooks/load_context.ts
      - id: daily_brief
        type: skill
        skill: core/daily-brief
        after: [load_context]
      - id: task_triage
        type: skill
        skill: core/task-triage
        after: [daily_brief]
      - id: create_nudges
        type: skill
        skill: core/create-nudges
        after: [task_triage]
      - id: human_review
        type: gate
        after: [create_nudges]
        approval:
          mode: ask
          reason: "Before writing task/project changes"
      - id: persist_changes
        type: skill
        skill: integrations/todoist-enrich
        after: [human_review]

  capabilities:
    tools:
      - calendar.read
      - tasks.read
      - tasks.write
      - memory.read
      - thread.write
      - nudge.write
    connectors:
      - google_calendar
      - todoist
    local:
      - fs:read
    models:
      - fast
      - reasoning

  permissions:
    confirmationPolicy:
      default: ask
      exceptions:
        - action: calendar.read
          mode: auto
        - action: tasks.read
          mode: auto
        - action: thread.write
          mode: auto
        - action: tasks.write
          mode: ask
    dataAccess:
      pii: limited
      secrets: denied
      externalNetwork: denied

  execution:
    concurrency: serial
    maxParallel: 1
    timeoutMs: 90000
    retryPolicy:
      maxAttempts: 2
      backoff: exponential
    resumePolicy:
      enabled: true
      checkpointAfterEachStep: true
    idempotency:
      keyTemplate: "{{user.id}}:{{date.local}}:morning-orientation"

  observability:
    logLevel: info
    emitArtifacts: true
    captureStepOutputs: true
    capturePromptSnapshots: true

  ui:
    icon: sunrise
    color: gold
    surfaces:
      - cli
      - web
      - automation
      - thread
```

## Step types

Start with a small fixed set:

- `skill` — invoke a skill package through the runtime
- `tool` — invoke a primitive tool directly for deterministic operations
- `hook` — run local code with structured I/O
- `prompt` — perform LLM reasoning/rendering directly when no standalone skill exists
- `gate` — wait for or request human approval/input
- `branch` — conditional routing
- `emit` — write artifact, thread post, nudge, telemetry, or event
- `noop` — explicit placeholder for development and testing

## State machine

A workflow run should have explicit state transitions:

- pending
- scheduled
- waiting_for_trigger
- ready
- running
- waiting_for_approval
- paused
- retrying
- succeeded
- failed
- canceled
- superseded

Step-level states should mirror these where relevant.

## Required runtime records

Each workflow run should record:

- workflow identity/version
- trigger that fired the run
- bound context entities and their resolved IDs
- step graph and expansion
- grants issued
- prompts used
- step outputs
- artifacts emitted
- operator/human interventions
- final outcome and failure reason

## Human checkpoints

Human review should be modeled declaratively.

Examples:

- approval before task writes
- approval before sending messages
- approval before mutating calendar
- optional approval if confidence below threshold

Avoid improvising approval logic inside prompts. Put it in manifest/runtime policy.
