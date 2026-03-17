# Plugin / Integration System

Vel should expose a plugin layer for integrations.

For the canonical source-family list and connector contract, see [canonical-data-sources-and-connectors.md](canonical-data-sources-and-connectors.md).
For the concrete provider inventory, see [data-source-catalog.md](data-source-catalog.md).

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
- secret handling mode
- data freshness expectations
- failure modes
- write permissions
- allowed hosts or resource scopes
- source mode (`local_file`, `snapshot`, `oauth_api`, `brokered_tool`, `delegated_runtime`)
- source mode (`local_file`, `local_directory`, `local_snapshot`, `oauth_api`, `brokered_tool`, `delegated_runtime`)
- emitted entity and signal schemas

## Example Capability Declaration

```json
{
  "name": "calendar_google",
  "capabilities": ["read_events", "write_events"],
  "secret_mode": "brokered_injection",
  "freshness_sla": "60s",
  "write_requires_confirmation": true,
  "allowed_hosts": ["www.googleapis.com"]
}
```

## Rule

Unavailable integrations degrade confidence.
They should never be silently assumed healthy.

Integration plugins should prefer mediated capability execution over handing raw credentials to the calling agent.
