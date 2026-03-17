---
title: Add CLI project commands and workspace view
status: ready
owner: agent
priority: P1
area: projects
---

# Goal
Give the CLI parity with the Projects page using the same backend workspace contract.

## Scope
- add `vel project` command tree
- add list/open/tasks/add-task/queue/steer/feedback subcommands
- add a readable workspace output mode

## Example commands
```bash
vel project list
vel project open vel
vel project tasks vel
vel project add-task vel "wire project workspace routes" --tag backend --tag api
vel project queue vel codex "add session outbox persistence"
vel project steer vel codex "avoid parallel task authority"
vel project feedback vel claude --type thumbs_down --note "too vague"
```

## Requirements
- use existing CLI style and `ApiClient`
- do not invent separate endpoint shapes for CLI convenience
- support `--json` on list/open/tasks where sensible
- workspace text output should summarize:
  - project header
  - key counts
  - top tasks
  - sessions
  - queued items

## Tests
- command wiring smoke tests where feasible
- output and JSON mode tests for core commands

## Done when
- CLI can fully operate the project workspace without the web UI
