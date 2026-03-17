---
title: Adapter Sync Loop Orchestration
status: proposed
priority: high
owner: codex
---

# Goal

Move adapter syncs toward a consistent looped runtime instead of only manual endpoints.

# Concrete file targets

- `crates/veld/src/worker.rs`
- `crates/veld/src/adapters/*.rs`
- `crates/veld/src/routes/sync.rs`
- `config/policies.yaml`

# Concrete code changes

## Add sync loop runners
Support looped execution for:
- calendar
- todoist
- activity
- messaging

Keep notes/transcripts optional if they are heavier or more local.

## Bound each sync loop
Each sync loop should:
- be independently enableable
- log last success/error
- not automatically trigger infinite chained work

## Link to evaluate
After a sync that materially changed signals or commitments, schedule or immediately run the evaluate loop once.

Start simple:
- after successful sync with changes, invoke evaluate synchronously once
or
- bring next due time for evaluate loop forward

# Acceptance criteria

- common adapters can be sync-looped automatically
- sync success/failure is inspectable
- signal freshness no longer depends only on manual operator action
