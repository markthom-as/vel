# 12. UI, CLI, and Observability

## 12.1 Why this matters

The richer the semantic and policy system gets, the more dangerous opacity becomes.
Users and developers need inspection tools, not spiritual faith.

## 12.2 Must-have inspection surfaces

### Object inspection
Show:

- fields
- active templates
- active traits
- relations
- field ownership
- source mappings
- warnings/facts
- recent mutations

### Source inspection
Show:

- capabilities
- sync profile
- last sync
- known limitations
- warnings/facts
- conflicts

### Workflow/skill inspection
Show:

- trigger
- context bindings
- capabilities requested
- last runs
- outputs
- failures

### Policy inspection
Show:

- actor grants
- denied actions
- why a write is blocked
- confirmation requirements

## 12.3 CLI expectations

Need commands such as:

```bash
vel object explain task_123
vel template resolve personal/haircut-task
vel module inspect integrations/todoist
vel source inspect todoist
vel policy explain --action task.update --object task_123 --field dueAt
vel workflow logs morning-standup
```

## 12.4 Debugging philosophy

Do not make developers guess:

- which adapter wrote a field
- why a workflow didn’t run
- why a sync conflicted
- why a mutation was denied
- which template fields were active

Make the system confess.

## 12.5 Summary recommendation

Observability is not garnish here. It is structural. Without it, the architecture will feel more magical right up until it feels fraudulent.
