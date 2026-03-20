# 24-01 Summary

Phase 24 execution has started. `24-01` is complete.

What changed:

- Added a typed assistant proposal lifecycle state in [crates/vel-core/src/operator_queue.rs](crates/vel-core/src/operator_queue.rs) and re-exported it from [crates/vel-core/src/lib.rs](crates/vel-core/src/lib.rs).
- Extended the transport contract in [crates/vel-api-types/src/lib.rs](crates/vel-api-types/src/lib.rs) and [clients/web/src/types.ts](clients/web/src/types.ts) so assistant proposals now carry explicit lifecycle state on the wire.
- Updated [crates/veld/src/services/chat/messages.rs](crates/veld/src/services/chat/messages.rs) so newly staged assistant proposals are explicitly marked `staged` and persist `proposal_state` in dedicated `assistant_proposal` thread metadata.
- Updated focused regressions in [crates/veld/tests/chat_assistant_entry.rs](crates/veld/tests/chat_assistant_entry.rs) and [clients/web/src/types.test.ts](clients/web/src/types.test.ts) so the current shipped behavior proves proposal lifecycle state is explicit rather than implied by permission mode or thread text.

Verification:

- `cargo test -p veld --test chat_assistant_entry -- --nocapture`
- `cargo test -p veld operator_queue -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo fmt --all`

Notes:

- This slice only publishes the lifecycle contract and persists current proposals as `staged`.
- It does not yet apply approved assistant proposals; that follow-through remains Phase `24-02`.
- Rust test runs still emit the same pre-existing dead-code and unused warnings in `veld`.
