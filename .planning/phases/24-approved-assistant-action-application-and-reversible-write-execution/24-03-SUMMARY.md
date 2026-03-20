# 24-03 Summary

Phase 24 plan `24-03` is complete.

## Outcome

Applied and reversed assistant outcomes now preserve backend-owned provenance in proposal threads, and `Threads` can explain proposal lifecycle without shell-local policy inference.

## What changed

- Updated [crates/veld/src/services/chat/messages.rs](crates/veld/src/services/chat/messages.rs) so assistant proposal threads now start with explicit lineage and reversal metadata instead of only a raw staged follow-through payload.
- Updated [crates/veld/src/services/chat/interventions.rs](crates/veld/src/services/chat/interventions.rs) so the existing intervention actions synchronize assistant proposal thread continuity:
  - `resolve` / `acknowledge` mark the proposal thread as `applied`
  - `dismiss` marks it `failed` or `reversed` depending on prior proposal state
  - `snooze` preserves pending continuity without inventing a second queue model
- Updated [crates/veld/src/services/operator_queue.rs](crates/veld/src/services/operator_queue.rs) so assistant proposal action items read companion proposal-thread metadata and surface current lifecycle/follow-through state instead of freezing the original staged payload forever.
- Updated [crates/veld/src/routes/threads.rs](crates/veld/src/routes/threads.rs) so assistant proposal thread detail reads expose lifecycle stage from `proposal_state`, allowing shells and CLI to explain `approved`, `applied`, `failed`, and `reversed` states without guessing from raw JSON.
- Added focused regressions in [crates/veld/tests/chat_assistant_entry.rs](crates/veld/tests/chat_assistant_entry.rs) and [crates/veld/src/routes/threads.rs](crates/veld/src/routes/threads.rs) to prove proposal threads preserve the `approved -> applied -> reversed` story over the existing intervention and thread routes.

## Verification

- `cargo fmt --all`
- `cargo test -p veld threads -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `cargo test -p veld --test chat_assistant_entry -- --nocapture`

## Notes

- Reversal remains intentionally bounded. The new thread metadata only records reversible assistant follow-through where the existing product already supports it; it does not invent new provider-level undo behavior.
- Rust test runs still emit the existing unused/dead-code warnings in `veld`.
