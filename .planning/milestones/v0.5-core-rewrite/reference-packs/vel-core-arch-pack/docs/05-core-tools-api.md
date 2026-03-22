# 05. Core Tools API

## 5.1 Purpose

The Core Tools API is the **only legal membrane** through which CLI, workflows, skills, modules, agents, and UI mutate or retrieve state.

This is the central discipline that keeps the system coherent.

## 5.2 API goals

The API should be:

- typed
- inspectable
- permission-aware
- auditable
- stable
- composable
- equally usable from CLI and programmatic tool runtimes

## 5.3 Tool categories

### Object tools
- `object.get`
- `object.query`
- `object.explain`
- `object.validate`

### Task tools
- `task.create`
- `task.update`
- `task.complete`
- `task.cancel`
- `task.snooze`
- `task.linkProject`

### Project tools
- `project.create`
- `project.update`
- `project.archive`
- `project.linkTask`

### Event tools
- `event.create`
- `event.update`
- `event.linkTask`
- `event.attachThread`

### Thread/message tools
- `thread.create`
- `thread.appendMessage`
- `message.create`
- `message.linkTask`

### Template tools
- `template.apply`
- `template.resolve`
- `template.list`

### Workflow/skill tools
- `workflow.run`
- `workflow.inspect`
- `skill.run`
- `skill.inspect`

### Source/sync tools
- `source.list`
- `source.inspect`
- `source.sync`
- `source.mapping.inspect`

### Policy tools
- `policy.evaluate`
- `policy.explain`
- `grant.inspect`

### Audit tools
- `audit.list`
- `audit.explain`
- `mutation.replayPreview`

## 5.4 Action model

Each tool/action should define:

- action name
- input schema
- output schema
- required capabilities
- confirmation policy defaults
- mutability class
- affected object types
- audit requirements

## 5.5 Example action manifest

```yaml
action: task.update
description: Update one or more writable fields on a Task object.
inputSchema: ./schemas/task.update.input.json
outputSchema: ./schemas/task.update.output.json
requiresCapabilities:
  - task.update
confirmationPolicy: conditional
mutability: write
audit:
  level: full
```

## 5.6 Object explainability endpoint

One of the most useful actions will be `object.explain`.

It should show:

- canonical fields
- active templates
- active traits
- relations
- source mappings
- field ownership
- last mutations
- warnings/facts
- blocked actions and why

This is the antidote to opaque automation gaslighting.

## 5.7 CLI mirror

The CLI should mirror the tool-use API.

Examples:

```bash
vel object get task_123
vel object explain task_123
vel task create --template haircut
vel task update task_123 --field estimatedDuration=45m
vel source sync todoist
vel workflow run morning-standup --context today
```

## 5.8 Query language

A light query layer should support filtering by:

- object type
- fields
- tags
- relations
- time windows
- source ownership
- sync status
- warnings/facts labels
- project/thread context

Example query concepts:

- all active tasks due today in project X
- all Todoist-backed tasks with sync conflicts
- all threads related to event Y
- all nudges currently relevant for active task Z

## 5.9 Bulk action safety

Bulk actions should be explicitly supported but policy-aware.

Examples:

- bulk tag apply
- bulk task complete
- bulk sync reconciliation
- bulk relation linking

Bulk operations should expose previews/dry runs before destructive application where appropriate.

## 5.10 Tool implementation notes

Primitive tools may be implemented directly in core or delegated to controlled executors.

The important rule is that, to callers, they remain:

- typed
- policy-gated
- audited
- explainable

## 5.11 Summary recommendation

Treat the Core Tools API as the constitutional membrane of Vel. If an operation matters, it should have a named, typed, inspectable action in this layer.
