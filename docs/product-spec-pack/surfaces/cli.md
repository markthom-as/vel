# Surface Spec: CLI

## Intent

The CLI is Vel's **highest‑precision interface**.

It is designed for:

- power users
- scripting
- deterministic interaction

## Entry Points

```
vel
vel ask
vel commitments
vel risk
vel suggest
```

## Core Interactions

### Ask System

```
vel ask "what should I do next"
```

### List Commitments

```
vel commitments
```

### Resolve Commitment

```
vel resolve <id>
```

### Snooze Reminder

```
vel snooze <id>
```

## Output Modes

```
plain
table
json
tui
```

## Behavioral Rules

- CLI never interrupts user
- CLI outputs deterministic results
- CLI favors structured data