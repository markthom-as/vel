---
title: Ticket 503 - Add loop registry and scheduler for durable background work
status: proposed
owner: codex
priority: critical
---

# Goal

Turn implicit background intentions into explicit loop kinds with durable execution records.

# Files

## New
- `crates/vel-core/src/looping.rs`
- `crates/veld/src/services/loop_registry.rs`
- `crates/veld/src/services/loop_scheduler.rs`
- `migrations/0030_loop_runs.sql`

# Loop kinds
Start with:
- `sync_projects`
- `sync_todoist`
- `sync_calendar`
- `recompute_context`
- `evaluate_risk`
- `evaluate_suggestions`
- `evaluate_uncertainty`
- `morning_review`
- `midday_review`
- `evening_review`
- `weekly_review`

# Scheduler behavior
Minimal v1:
- configurable polling interval
- each loop kind has cadence + backoff
- records a loop run row per attempt
- stores summary result JSON

# Acceptance criteria

- loop kinds are explicit enums, not string soup
- loop runs are inspectable after the fact
