# 03. Template System

## 3.1 Why templates matter

Templates should not be treated as mere text snippets or thin preset objects.

In Vel, a `Template` should function as a **composable blueprint layer** that can:

- target one or more core object types
- activate traits/aspects
- enable or disable field groups
- define defaults and derived values
- provide validation extensions
- contribute UI hints
- attach workflow hooks
- declare source mapping hints
- encode policy expectations

This turns templates into structural building blocks rather than decorative shortcuts.

## 3.2 Template goals

Templates should support:

- rapid object creation
- domain-specific presets
- reusable semantic patterns
- integration-specific overlays
- workspace-specific conventions
- composition and inheritance
- policy-safe activation of complexity

## 3.3 Template architecture

A template should have:

- metadata
- target types
- trait composition
- field group controls
- defaults
- validations
- computed fields
- UI hints
- workflow hooks
- policy hints
- adapter hints

## 3.4 Example use cases

### Example: Haircut task template
Base type: `Task`

Adds:

- `Schedulable`
- default estimated duration of 45m
- grooming project association
- preferred tags
- soft reminder nudges
- pre-appointment prep checklist
- post-completion follow-up schedule

### Example: Todoist synced task overlay
Base type: `Task`

Adds:

- `Syncable`
- source mapping hints
- field owner restrictions
- sync conflict labels
- source-specific warnings

### Example: Standup workflow template
Base type: `Workflow`

Adds:

- morning trigger defaults
- task/event/project context bindings
- summary artifact template
- preferred model routing

## 3.5 Template composition model

Templates should support composition rather than only singular inheritance.

Example composition stack:

- `Task/BaseTask`
- `Task/TimedTask`
- `Task/ErrandTask`
- `Task/HaircutTask`
- `Task/TodoistSyncedOverlay`

Effective object config is the resolved overlay result.

## 3.6 Resolution order

Recommended precedence from lowest to highest:

1. base core type defaults
2. trait defaults
3. inherited/composed template defaults
4. workspace defaults
5. source adapter hints
6. explicit creation-time values
7. policy-enforced overrides

That order gives stability without making explicit user data lose to magical hidden config.

## 3.7 Template safety

Templates should not bypass policy.

A template may suggest:

- default tags
- field ownership hints
- workflow hooks
- confirmation recommendations

But core still decides whether resulting actions are legal.

## 3.8 Template package format

Suggested structure:

```text
templates/
  task/
    haircut/
      template.yaml
      defaults.yaml
      ui.yaml
      validations.yaml
      README.md
```

## 3.9 Example template manifest

```yaml
apiVersion: vel/v1
kind: Template
metadata:
  name: haircut-task
  namespace: personal
  version: 0.1.0
spec:
  targetTypes:
    - Task
  compose:
    - core/task-base
    - core/task-timed
  traits:
    include:
      - Schedulable
      - Taggable
      - Auditable
  fieldGroups:
    enable:
      - core
      - time
      - planning
      - governance
  defaults:
    title: Haircut
    estimatedDuration: 45m
    tags:
      - grooming
    status: active
  relations:
    suggest:
      projectId: project_self_maintenance
  workflows:
    onCreate:
      - workflow: personal/prep-and-remind
  uiHints:
    icon: scissors
    color: plum
```

## 3.10 Template targeting across multiple types

Templates may target more than one type when appropriate.

Example:

- a “meeting pack” template that configures both `Event` and related `Thread`
- a “project kickoff” template that creates a `Project`, `Thread`, initial `Tasks`, and a starter `Workflow`

That could be implemented as a composite object template or as a workflow-backed template.

## 3.11 Template UI behavior

Templates should optionally contribute:

- labels
- icons
- colors
- field ordering
- collapsed/expanded sections
- default forms
- suggested quick actions

This should remain hints, not source of truth.

## 3.12 Template + workflow relationship

Templates and workflows are cousins, not twins.

- **Template**: shapes what an object is or starts as
- **Workflow**: shapes what happens over time or through execution

A template may attach workflow hooks such as:

- `onCreate`
- `onComplete`
- `beforeDue`
- `afterSync`

## 3.13 Summary recommendation

Templates should be treated as composable semantic blueprints with enough structure to shape objects, validation, UI, and automation hooks without mutating the underlying law of core types.
