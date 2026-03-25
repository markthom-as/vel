# Phase 106 Validation

## Required Checks

### Structural

- verify `capability_resolver.rs` no longer defines a single-implementation trait
- verify `tool_runner.rs` no longer defines a single-implementation trait
- verify `policy_config.rs` no longer contains the dead policy structs/accessors flagged by `VD-03`

### Automated

- `cargo test -p veld capability_resolver`
- `cargo test -p veld tool_runner`
- `cargo test -p vel-cli policy_check`
- `cargo check -p veld --all-targets`
- `cargo check -p vel-cli --all-targets`

### Warning-Debt Truthfulness

- compare pre/post dead-code suppression usage in:
  - `crates/veld/src/lib.rs`
  - `crates/veld/src/main.rs`
  - `crates/vel-cli/src/client.rs`
- if any blanket suppression remains, record the exact reason and owning follow-on instead of leaving it implicit

### Manual Review

- inspect the updated `command_lang.rs` call sites to confirm the trait removal did not smuggle in broader behavioral changes
- inspect `policy_config.rs` to confirm only genuinely unused policy surfaces were removed

## Failure Conditions

- warning cleanup relies on adding new blanket suppressions
- dead policy removal breaks active config parsing without a replacement
- trait removal changes behavior beyond direct call-site simplification
