# Event Bus Architecture

Vel should be built around an **event-driven architecture**.

## Why

An event bus gives Vel:

- decoupled services
- replayable history
- observable state transitions
- easier agent coordination

## Canonical Event Types

```text
commitment.created
commitment.updated
commitment.resolved
commitment.snoozed

suggestion.generated
suggestion.surfaced
suggestion.accepted
suggestion.ignored
suggestion.dismissed

risk.calculated
risk.threshold_crossed

context.changed
calendar.updated
location.updated
activity.updated

agent.task_started
agent.task_completed
agent.task_failed

reflection.cycle_started
reflection.finding_recorded
reflection.policy_updated
```

## Event Requirements

Every event should include:

```json
{
  "id": "uuid",
  "type": "event.name",
  "timestamp": "ISO8601",
  "source": "component_name",
  "entity_id": "optional primary entity id",
  "correlation_id": "optional request or workflow id",
  "payload": {}
}
```

## Rules

- events are append-only
- derived state is rebuildable from events plus snapshots
- policy mutations must emit explicit events
- silent mutation is forbidden
