---
title: Current Context Contract Hardening
status: proposed
priority: critical
owner: codex
---

# Goal

Turn `current_context` into an explicit contract rather than a blob implicitly assembled in inference.

# Why this matters

Vel is converging on `current_context` as the runtime truth surface:
- nudge engine reads it
- explain flows rely on it
- operator surfaces summarize it
- future loops will depend on it

That means the shape must be typed, versioned enough to reason about, and tested.

# Concrete code changes

## Create domain type
Create:
- `crates/vel-core/src/current_context.rs`

Export from:
- `crates/vel-core/src/lib.rs`

## Update inference writer
Update:
- `crates/veld/src/services/inference.rs`

Instead of assembling a large `serde_json::json!` object inline, assemble a `CurrentContext` struct and serialize it at the storage boundary.

## Update API DTOs
Update:
- `crates/vel-api-types/src/lib.rs`

Add or refine DTOs for current-context reads if needed.

## Add schema version field in JSON
Add:
- `schema_version: u32`

Not for migration theatrics; just to allow safe evolution.

Example:
```rust
pub struct CurrentContext {
    pub schema_version: u32,
    pub computed_at: i64,
    // ...
}
```

## Add required-key tests
Add tests that deserialize persisted `current_context` JSON into the struct.

# Acceptance criteria

- `current_context` has a typed owner in `vel-core`
- inference writes a typed struct
- current context JSON can round-trip through tests
- changes to the context shape become deliberate
