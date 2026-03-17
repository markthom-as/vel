---
title: Ticket 410 - Add fixtures, reconciliation tests, and backfill support
status: proposed
owner: codex
priority: high
---

# Goal

Prove the external-awareness model works from current repo state instead of only on happy-path live data.

# Files

## New fixture directories
- `crates/veld/tests/fixtures/todoist/`
- `crates/veld/tests/fixtures/calendar/`
- `crates/veld/tests/fixtures/projects/`

# Add test coverage

## Todoist fixtures
- open task
- completed task
- due date changed
- unresolved project id
- label-only change

## Calendar fixtures
- busy event
- free event
- event moved
- event without stable source id
- event linked to project alias

## Project registry fixtures
- valid markdown table
- alias case
- renamed display name retaining slug
- malformed row recovery

## Scenario tests
- sync projects -> sync todoist -> generate context
- sync calendar -> evaluate schedule pressure -> create proposal
- unresolved project mapping -> uncertainty record instead of fake project slug

# Backfill task
Add a one-shot command or worker task to:
- import current snapshots into `external_items`
- create missing `external_item_links`
- preserve existing commitments where possible

# Acceptance criteria

- repo has deterministic fixtures for external awareness
- migration/backfill path works from existing local databases
