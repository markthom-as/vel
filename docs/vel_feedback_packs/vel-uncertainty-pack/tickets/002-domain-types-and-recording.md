---
title: Uncertainty Domain Types and Recording Hooks
status: proposed
priority: critical
owner: codex
---

# Goal

Create one domain vocabulary for uncertainty and begin emitting records from real services.

# Concrete file targets

- `crates/vel-core/src/uncertainty.rs`
- `crates/vel-core/src/lib.rs`
- `crates/veld/src/services/inference.rs`
- `crates/veld/src/services/suggestions.rs`
- `crates/veld/src/services/chat*` where appropriate
- `crates/veld/src/services/uncertainty.rs` (new)

# Concrete code changes

## Add typed enums

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceBand {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionMode {
    Proceed,
    AskUser,
    AskAgent,
    Defer,
    SilentHold,
}
```

## Add helper functions

`services/uncertainty.rs` should contain:
- `band_from_score(score: f64) -> ConfidenceBand`
- `resolution_mode_for(score: f64, decision_kind: &str) -> ResolutionMode`

## Record from real services

Start by instrumenting:
- inference: when current context relies on missing event travel/prep data
- suggestions: when evidence is barely above threshold
- chat: when answer is generated with weak provenance or missing linked evidence

# Acceptance criteria

- uncertainty records are emitted from at least inference and suggestion flows
- bands and resolution modes are normalized rather than ad hoc strings
