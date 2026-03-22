# Permissions and Policy

## Why this matters

A pluggable system without strict permissions is just a nicely formatted security incident.

Vel’s skill system should assume from the start that features are toggleable, contextual, and potentially sensitive. That means the runtime must mediate access rather than letting skills inherit ambient authority.

## Core model

Use a three-part model:

### Capabilities
What the skill says it may need.

Examples:

- `calendar.read`
- `calendar.write`
- `tasks.read`
- `tasks.write`
- `messages.send`
- `files.read`
- `files.write`
- `shell.exec`
- `network.fetch`
- `location.read`
- `microphone.capture`
- `memory.read`
- `memory.write`

### Policy
What the system and user/workspace settings permit.

Policy can vary by:

- user
- workspace
- device type
- execution surface
- time/context mode
- explicit confirmation settings

### Grants
What is actually granted for a specific execution.

A skill may request ten things and receive three.

## Confirmation modes

Recommend these modes:

- `auto` — allowed without prompting
- `ask` — requires confirmation for this execution
- `ask_once` — remember approval in bounded scope
- `deny` — not allowed

## Policy hierarchy

Recommended order of influence:

1. hard safety restrictions
2. runtime/platform restrictions
3. workspace policy
4. user preferences
5. skill manifest requests
6. per-run overrides

The skill manifest is a request, not a royal decree.

## Permission categories

### Read-only source access
Lower risk but still sensitive.

Examples:

- calendar read
- task read
- file read
- history read
- memory read

### Mutating source access
Higher risk and often confirmation-worthy.

Examples:

- create task
- edit event
- send message
- write memory
- modify files

### Local execution
Potentially dangerous.

Examples:

- shell commands
- arbitrary subprocesses
- filesystem write
- device APIs

### External network
Potential exfiltration risk.

Examples:

- third-party APIs
- web fetch
- webhook dispatch

## Recommended MVP enforcement rules

For MVP:

- allow read-only connector access when explicitly enabled in user config
- deny write actions by default unless skill is first-party and user-approved
- deny arbitrary shell access by default
- deny arbitrary external network by default
- log all requested vs granted capabilities
- require explicit confirmation for mutating actions

## Example manifest policy section

```yaml
permissions:
  confirmationPolicy:
    default: ask
    exceptions:
      - action: calendar.read
        mode: auto
      - action: tasks.read
        mode: auto
      - action: tasks.write
        mode: ask
      - action: shell.exec
        mode: deny
  dataAccess:
    pii: limited
    secrets: denied
    externalNetwork: denied
```

## Auditing

Every run should record:

- requested capabilities
- granted capabilities
- denied capabilities
- confirmation decisions
- mutating side effects

This is not just for safety. It is also for debuggability and user trust.

## Device and surface awareness

Policy should eventually vary by surface.

Examples:

- watchOS / voice mode may allow fewer destructive actions
- server automation may allow approved background reads
- desktop may allow controlled file access
- mobile may allow location-aware skills only with explicit opt-in

## Recommendation

Treat permissions as a first-class product feature, not an implementation footnote. If you leave it until later, “later” will arrive with a knife.
