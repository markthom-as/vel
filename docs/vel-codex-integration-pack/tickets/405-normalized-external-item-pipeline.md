---
title: Ticket 405 - Add shared external sync service and normalized ingestion pipeline
status: proposed
owner: codex
priority: high
---

# Goal

Factor common ingestion logic out of source adapters and into a shared service.

# Files

## New
- `crates/veld/src/services/external_sync.rs`

## Changed
- `crates/veld/src/services/mod.rs`
- `crates/veld/src/worker.rs`
- `crates/veld/src/adapters/todoist.rs`
- `crates/veld/src/adapters/calendar.rs`

# Service responsibilities

The service should handle:
- source fetch start/end logging
- watermark lookup/update
- external item upsert
- fingerprint change detection
- signal emission helpers
- link creation helpers
- item count / changed count metrics

# Suggested trait

```rust
pub trait ExternalAdapter {
    fn source_kind(&self) -> ExternalSourceKind;
    async fn collect(&self, storage: &Storage, config: &AppConfig) -> Result<Vec<ExternalItem>, AppError>;
}
```

Do not overabstract immediately. Two adapters is enough to prove the pattern.

# Worker integration

Add worker jobs:
- `sync_todoist`
- `sync_calendar`
- `sync_projects`

Use explicit loop claims later; for now, make service callable from existing worker entrypoints.

# Acceptance criteria

- adapters only worry about source parsing / mapping
- shared service owns persistence and bookkeeping
- worker can run each external sync deterministically
