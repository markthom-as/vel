---
title: "Implement Message Domain Model"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 004-implement-core-id-types
labels:
  - vel
  - chat-interface
---
Define the first useful message schema in `vel-core`.

## Location

`crates/vel-core/src/message.rs`

## Required Enum

```rust
pub enum MessageBody {
    Text(TextMessage),
    ReminderCard(ReminderCard),
    RiskCard(RiskCard),
    SuggestionCard(SuggestionCard),
    SummaryCard(SummaryCard),
    SystemNotice(SystemNotice),
}
```

## Shared Fields

- `id`
- `thread_id`
- `role`
- `kind`
- `importance`
- `status`
- `provenance`
- `actions`
- `created_at`

## Acceptance Criteria

- serde roundtrip tests pass
- enum variants serialize correctly
- model is suitable for frontend discriminated-union rendering

## Notes for Agent

Do not reduce every message to raw text plus metadata. That road ends in bubble prison.
