# Plugin / Integration System

Vel should expose a plugin layer for integrations.

## Integration Classes

- calendar
- reminders
- email
- messaging
- maps / travel
- health / medication
- home automation

## Plugin Contract

Each plugin should declare:

- capabilities
- auth requirements
- data freshness expectations
- failure modes
- write permissions

## Example Capability Declaration

```json
{
  "name": "calendar_google",
  "capabilities": ["read_events", "write_events"],
  "freshness_sla": "60s",
  "write_requires_confirmation": true
}
```

## Rule

Unavailable integrations degrade confidence.
They should never be silently assumed healthy.
