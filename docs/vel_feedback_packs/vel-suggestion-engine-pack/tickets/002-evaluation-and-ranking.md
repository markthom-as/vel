---
title: Deterministic Suggestion Evaluation and Ranking
status: proposed
priority: critical
owner: codex
---

# Goal

Replace the current hard-coded trigger function with a structured evaluation pipeline.

# Current starting point

- `crates/veld/src/services/suggestions.rs`

# Concrete code changes

## A. Introduce typed candidate struct

Add in `crates/veld/src/services/suggestions.rs` or a new module:
```rust
struct SuggestionCandidate {
    suggestion_type: vel_core::SuggestionType,
    title: String,
    summary: String,
    priority: i32,
    confidence: vel_core::ConfidenceBand,
    dedupe_key: String,
    payload: serde_json::Value,
    decision_context: serde_json::Value,
    evidence: Vec<SuggestionEvidenceCandidate>,
}
```

## B. Split evaluation into stages

Refactor into:
- `collect_candidates(storage, policy, now_ts)`
- `suppress_candidates(storage, candidates, now_ts)`
- `rank_candidates(candidates)`
- `persist_candidates(storage, candidates)`

## C. Add new candidate families

Start with deterministic families only:
- repeated commute danger -> `IncreaseCommuteBuffer`
- repeated prep-window misses -> `IncreasePrepWindow`
- repeated morning drift -> `ProtectDeepWorkBlock` or `AddStartRoutine`
- repeated response debt -> `AddFollowupBlock`

Do not add every imaginable suggestion. Just make the system extensible.

## D. Rank by explicit priority formula

Example:
- base priority by suggestion type
- add evidence count weighting
- add recency weighting
- add current global risk weighting

Document the formula in code comments.

# Acceptance criteria

- suggestion evaluation is staged and readable
- at least four deterministic suggestion families are supported
- ranking is explicit and testable
