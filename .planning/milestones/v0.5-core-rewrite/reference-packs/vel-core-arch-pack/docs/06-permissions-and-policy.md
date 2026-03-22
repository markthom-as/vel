# 06. Permissions and Policy

## 6.1 Purpose

Vel needs a strong permission and policy system early.
A pluggable architecture without policy becomes a very sophisticated own-goal.

## 6.2 Core concepts

### Capability
A named action permission, such as:

- `task.read`
- `task.update`
- `project.create`
- `event.linkTask`
- `workflow.run`
- `source.sync`

### Policy
A rule set defining when and how capabilities may be exercised.

### Grant
An approved capability scope for a given actor/context.

### Actor
Entity requesting action:

- user
- module
- workflow
- skill
- sync adapter
- CLI session
- automation run

### Scope
Boundary where a grant is valid:

- global
- workspace
- source
- object type
- specific object
- field group
- execution session

## 6.3 Permission model

Permission evaluation should consider:

1. actor identity
2. requested action
3. target object(s)
4. target field(s)
5. source ownership rules
6. active workspace policy
7. execution context
8. confirmation policy
9. trust labels / warnings
10. destructive action class

## 6.4 Field-level authorization

Do not stop at object-level auth.
You need field-level checks.

Example:

A Todoist adapter may update:
- title
- dueAt
- labels

But not:
- energy
- rationale
- Vel-only semantic notes

## 6.5 Ownership and write domains

Each object and relevant field should carry write domain metadata.

Example:

```yaml
ownership:
  objectOwner: core
  fieldOwners:
    title:
      allowedWriters: [core, todoist]
    dueAt:
      allowedWriters: [todoist, core]
    energy:
      allowedWriters: [core]
```

## 6.6 Confirmation policy

Actions should support confirmation modes such as:

- `auto`
- `ask`
- `ask_if_destructive`
- `ask_if_cross_source`
- `deny`

This can be set by:

- default tool definition
- workspace policy
- source policy
- user settings
- execution mode

## 6.7 Facts and warnings labels

Objects, modules, tools, workflows, and sources should expose machine-readable labels.

Example:

```yaml
facts:
  sourceOfTruth: hybrid
  pii: possible
  destructiveActions: false
  externalRoundTrip: partial
warnings:
  - External source does not preserve all recurrence semantics.
  - Completion may sync upstream and affect shared collaborators.
constraints:
  - title max length 500
  - no native thread support upstream
```

These labels should be surfaced in CLI, UI, and tool introspection.

## 6.8 Audit model

Every meaningful mutation should log:

- actor
- timestamp
- action
- object(s) affected
- old values/new values where allowed
- policy evaluation summary
- confirmation source
- related workflow/skill/source IDs

## 6.9 Policy bundles

A workspace or module may provide policy bundles, such as:

- strict sync mode
- read-only integration mode
- high-confirmation mode
- quiet background sync mode
- personal experimental mode

## 6.10 Policy explainability

Need a `policy.explain` action.

Given an attempted action, it should say:

- allowed or denied
- why
- which rule matched
- whether confirmation is needed
- whether a field owner blocked it

This is crucial when automation starts feeling haunted.

## 6.11 Summary recommendation

Vel should implement a capability + policy + grant model with field-level ownership, confirmation rules, audit logs, and machine-readable facts/warnings labels from the start of real integration work.
