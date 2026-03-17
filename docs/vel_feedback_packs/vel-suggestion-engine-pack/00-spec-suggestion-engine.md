---
title: Vel Suggestion Engine Spec
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Purpose

The suggestion engine is not a chatbot flourish. It is Vel's steering layer.

A nudge says:
- do this now

A suggestion says:
- the system keeps seeing a pattern; change your setup, defaults, or commitments

# Existing repo starting point

Current implementation:
- `crates/veld/src/services/suggestions.rs`

Current limitations:
- hard-coded suggestion types
- hard-coded thresholds and windows
- evidence implicit in counts only
- dedupe = "is there already a pending suggestion of this type"
- no ranking
- no reason string
- no policy tuning
- no durable evidence rows
- no feedback loop beyond accept/reject state

# Target architecture

## Inputs

- latest/current context
- commitment risk history
- nudge history
- suggestion history
- policy config
- thread/message pressure where relevant

## Outputs

A persisted suggestion with:
- id
- suggestion_type
- state
- title
- summary
- payload
- evidence summary
- confidence band
- priority
- dedupe key
- created_at
- resolved_at

## Non-goals

- free-form LLM advice generation
- hidden heuristics
- uninspectable scores
- "AI said so" steering

# Concrete code changes

## A. Expand suggestion persistence model

Update schema to support:
- `title TEXT`
- `summary TEXT`
- `priority INTEGER`
- `confidence TEXT`
- `dedupe_key TEXT`
- `decision_context_json TEXT`
- evidence table

## B. Introduce typed suggestion domain model

Add to `vel-core`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    IncreaseCommuteBuffer,
    IncreasePrepWindow,
    SplitLargeCommitment,
    AddFollowupBlock,
    ReduceConcurrentMeetings,
    ProtectDeepWorkBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceBand {
    Low,
    Medium,
    High,
}
```

## C. Introduce deterministic evaluation pipeline

Refactor `services/suggestions.rs` into phases:
1. collect candidate evidence
2. build typed candidates
3. dedupe/suppress
4. rank
5. persist accepted candidates

# Acceptance criteria

- suggestions are durable and evidence-backed
- operator can inspect why a suggestion exists
- repeat patterns create stable suggestions without spam
- configuration moves thresholds out of magic constants
