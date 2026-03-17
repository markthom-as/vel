---
title: Ticket 506 - Add question engine and approval gates for ambiguous or risky actions
status: proposed
owner: codex
priority: high
---

# Goal

Route uncertainty and risky proposals into explicit questions and approvals instead of fake certainty.

# Files

## New
- `crates/veld/src/services/question_engine.rs`

## Changed
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/routes/suggestions.rs`
- `crates/veld/src/routes/sync.rs`

# Implementation

## Question kinds
- missing project mapping
- choose current focus task
- approve calendar move
- approve Todoist relabel / move
- clarify competing priorities

## Gate rules
Require explicit approval for:
- external writes
- schedule changes affecting existing busy events
- destructive task/project changes

Allow auto-apply for:
- low-risk local-only summaries
- informational nudges
- context recomputation

# Acceptance criteria

- unresolved ambiguity can surface as a user-facing question
- proposals can be approved from API/UI without custom one-off flows
