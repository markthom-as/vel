# Skill Package Specification

## Package layout

Recommended package structure:

```text
skills/
  core/
    daily-brief/
      skill.yaml
      prompt.md
      input.schema.json
      output.schema.json
      hooks/
        prepare.ts
        execute.ts
        cleanup.ts
      templates/
        daily-brief.md
      assets/
      tests/
        smoke.yaml
        fixtures/
```

Not every skill needs every file. But this shape is a sane default.

## Required files for MVP

### `skill.yaml`
Required. Defines metadata, execution mode, capabilities, routing hints, limits, and references to other files.

### `prompt.md`
Required for prompt-only and hybrid skills. Contains primary system/task instructions and templated context regions.

### `input.schema.json`
Recommended for MVP, effectively required for any skill expected to be composed or exposed through CLI/API.

### `output.schema.json`
Recommended for MVP, especially for composable skills.

## Optional files

- `hooks/prepare.*`
- `hooks/execute.*`
- `hooks/cleanup.*`
- templates
- tests
- assets
- examples
- migration notes
- compatibility shims

## Manifest structure

The manifest should include:

- `apiVersion`
- `kind`
- `metadata`
- `spec`

Example:

```yaml
apiVersion: vel/v1
kind: Skill

metadata:
  name: daily-brief
  namespace: core
  version: 0.1.0
  displayName: Daily Brief
  description: Generate a morning overview from calendar, tasks, and recent context.
  tags: [planning, morning, overview]

spec:
  type: hybrid
  entry:
    prompt: ./prompt.md
    prepare: ./hooks/prepare.ts
    execute: ./hooks/execute.ts
    cleanup: ./hooks/cleanup.ts

  inputSchema: ./input.schema.json
  outputSchema: ./output.schema.json

  capabilities:
    tools:
      - calendar.read
      - tasks.read
      - memory.read
      - files.read
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
    dataAccess:
      pii: limited
      secrets: denied
      externalNetwork: denied

  routing:
    triggers:
      intents:
        - morning overview
        - daily planning
      events:
        - app.start
        - scheduled.morning
    priority: 80

  dependencies:
    skills:
      - core/context-pack@^0.1

  limits:
    timeoutMs: 25000
    maxTokens: 12000
    maxToolCalls: 12

  ui:
    icon: sun
    color: amber
    surfaces:
      - cli
      - web
      - automation
```

## Manifest field expectations

### Metadata
Contains stable identity, versioning, display metadata, and search tags.

### Spec type
Should support at least:

- `prompt`
- `script`
- `hybrid`
- later: `workflow`

### Entry
Defines how the skill is executed. The runtime should not assume prompt-only forever.

### Capabilities
Declares requested access to tools, connectors, local resources, and models.

### Permissions
Defines the skill’s requested behavior policy. Actual permission grants are decided by runtime policy + user/workspace settings.

### Routing
Provides hints for auto-selection, suggested surfacing, and event-based triggers.

### Dependencies
Supports both skill dependencies and runtime package dependencies where needed.

### Limits
Allows bounded execution to avoid token/policy chaos.

### UI
Allows human-facing metadata without polluting core execution semantics.

## Lifecycle hooks

The runtime should support a simple lifecycle:

1. validate manifest
2. resolve dependencies
3. gather context
4. `prepare`
5. `execute`
6. validate output
7. `cleanup`
8. emit artifacts/logs

### `prepare`
Use for deterministic context collection and normalization.

### `execute`
Use for core skill behavior. In hybrid skills this may call the model and tools.

### `cleanup`
Use for output post-processing, artifact emission, persistence, metrics, and notifications.

## Skill categories

Strongly recommended namespaces:

- `core/*` — first-party foundational behaviors
- `integrations/*` — connector- or source-specific skills
- `local/*` — machine-specific or user-private skills
- later maybe `community/*`, `vendor/*`, `workspace/*`

## Why package shape matters

The package structure should make a skill legible to:

- human authors
- the runtime
- the CLI
- automated tests
- future import/export adapters

This is one of those places where a little bureaucracy saves you from a lot of future filth.
