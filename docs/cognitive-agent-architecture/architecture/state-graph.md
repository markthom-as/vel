# State Graph

Vel should model behavior as explicit state transitions rather than hidden mutable soup.

## Core Entity Graph

```text
User
 ├── Commitments
 ├── Suggestions
 ├── Risk Assessments
 ├── Context Snapshots
 ├── Habits / Routines
 └── Reflection Findings
```

## System States

```text
idle
observing
reasoning
nudging
waiting_for_user
executing
reflecting
degraded
```

## Example Transition

```text
idle
→ observing
→ reasoning
→ suggestion.generated
→ waiting_for_user
→ user.accepted
→ executing
→ completed
```

## Degraded State

Vel must have an explicit degraded mode when:

- integrations are unavailable
- confidence is too low
- clock/location data is stale
- memory index is unhealthy

In degraded mode:

- reduce proactivity
- avoid high-confidence tone
- surface uncertainty explicitly
