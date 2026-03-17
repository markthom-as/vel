---
id: VEL-PROJ-012
title: Add CLI commands for projects, tasks, queued messages, and steering
status: proposed
priority: P1
estimate: 2-4 days
dependencies:
  - VEL-PROJ-005
  - VEL-PROJ-007
labels:
  - cli
  - projects
  - control-plane
---

# Goal

Provide command-line parity for the Projects surface so the same workflow is available without the web UI.

# Scope

Add subcommands such as:

- `vel projects list`
- `vel projects show <slug>`
- `vel projects task add <slug> --title ... --tag ...`
- `vel projects task complete <slug> <task-id>`
- `vel projects task reopen <slug> <task-id>`
- `vel projects chat queue <slug> <session-id> --message ...`
- `vel projects chat steer <slug> <session-id> --message ...`
- `vel projects chat feedback <slug> <session-id> --rating ... --notes ...`

Use the same backend contracts as the web client.

# Deliverables

- CLI command tree additions in `vel-cli`
- human-readable terminal formatting
- JSON output mode if the CLI already supports or benefits from it
- tests for argument parsing and at least smoke-level HTTP integration

# Acceptance criteria

- Operator can list projects and inspect a workspace from CLI.
- Operator can create/tag a task from CLI.
- Operator can queue a message and add steering from CLI.
- CLI output clearly indicates manual-dispatch or degraded states.

# Notes for agent

The CLI should be crisp, not ceremonial. A control surface, not a liturgy.
