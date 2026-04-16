# Phase 108 Validation

## Required Checks

### Error Boundary

- inspect the chosen seam before and after to confirm the conversion chain is simpler and more uniform
- verify routes still map to correct HTTP-visible behavior after normalization
- first slice: `execution_context` seam has service-local storage mapping plus HTTP route coverage for request validation, missing context, and corrupted persisted context shape

### Crate Integration

- `vel-sim` remains green in deterministic replay tests
- `vel-agent-sdk` remains green in protocol/SDK tests
- the chosen CLI surfaces actually mention or exercise the retained crates
- first crate-integration slice: `vel evaluate` help/output and docs mention `veld-evals` / `vel-sim` replay without adding a `vel-cli` dependency on those crates
- final crate-integration slice: `vel exec` help/output and coding-execution docs mention `vel-agent-sdk` as the reference external/runtime envelope client without changing dependency structure

### Automated

- `cargo test -p veld agent_sdk -- --nocapture`
- `cargo test -p vel-agent-sdk -- --nocapture`
- `cargo test -p vel-sim -- --nocapture`
- `cargo test -p veld-evals -- --nocapture`
- targeted tests for the normalized route/service seam
- first slice: `cargo test -p veld execution_context -- --nocapture`
- final slice: `cargo test -p vel-cli exec`

### Documentation Truth

- update runtime/coding-workflow or adjacent docs so the crates are described as current integration surfaces, not historical leftovers
- document any remaining unintegrated SDK/simulation capabilities as follow-ons

## Failure Conditions

- adding a flashy new CLI surface instead of integrating under existing commands
- keeping the crates but still leaving them with no discoverable workflow attachment
- normalizing errors only in prose while leaving the implementation pattern unchanged
