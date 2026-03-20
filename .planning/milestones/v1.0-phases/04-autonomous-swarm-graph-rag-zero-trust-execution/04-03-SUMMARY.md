# 04-03 Summary

## Outcome

Completed the Phase 4 sandbox-policy slice. Capability brokering is now real instead of stubbed, the sandbox host executor enforces deny-by-default policy over decoded ABI envelopes, and sandbox outcomes are visible through run events plus CLI inspection.

## Delivered

- Implemented broker grant, deny, and execute persistence in `crates/veld/src/services/broker.rs`
- Added sandbox execution service in `crates/veld/src/services/sandbox.rs` that:
  - validates batch ABI/version consistency
  - creates or resumes an agent run
  - enforces `allowed_calls` fail-closed behavior
  - routes explicit calls through the broker
  - records `sandbox_call_evaluated` and `sandbox_run_completed` run events
- Registered the new service module from `crates/veld/src/services/mod.rs`
- Extended run event vocabulary in `crates/vel-core/src/run.rs`
- Updated `vel run inspect` in `crates/vel-cli/src/commands/runs.rs` to print payloads for high-value sandbox/search diagnostic events
- Updated `docs/cognitive-agent-architecture/agents/sandbox-host-abi.md` to distinguish shipped host-executor behavior from remaining planned runtime work

## Verification

- `cargo fmt`
- `cargo test -p veld broker -- --nocapture`
- `cargo test -p veld sandbox -- --nocapture`
- `cargo test -p vel-cli runs -- --nocapture`

## Notes

- This slice ships the deny-by-default host executor over decoded ABI envelopes, not a fully embedded WASM guest runtime yet. The concrete runtime-engine choice and direct guest execution remain open follow-on work.
