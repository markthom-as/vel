---
id: SK-006
title: Add system map and coverage CLI commands
status: proposed
priority: P1
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Expose the self-knowledge model through developer-facing CLI commands.

# Commands

- `vel system map`
- `vel system coverage`
- `vel system explain <component>`
- `vel system drift`
- `vel system changed --since <window>`

# Tasks

1. Add CLI subcommands and output formatting.
2. Support human-readable and JSON output.
3. Add coverage report for undocumented modules / weakly evidenced areas.
4. Add change summary report using git freshness data when available.

# Acceptance Criteria

- CLI commands function end-to-end against indexed data.
- JSON mode is stable enough for automation.
- Coverage report highlights missing docs and low-confidence areas.
- Explain command displays evidence and confidence in a readable form.

