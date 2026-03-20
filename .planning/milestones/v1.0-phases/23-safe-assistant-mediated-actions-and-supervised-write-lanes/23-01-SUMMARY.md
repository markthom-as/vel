# 23-01 Summary

Phase 23 plan `23-01` is complete.

## Outcome

The assistant can now stage an explicit bounded action proposal through the shared assistant-entry seam without performing a silent mutation or inventing a second action model.

## What changed

- Added the canonical proposal contract in [operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) and re-exported it from [lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs)
- Widened the transport boundary in [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs), [chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs), and [types.ts](/home/jove/code/vel/clients/web/src/types.ts) so assistant entry can return an optional typed proposal
- Updated [messages.rs](/home/jove/code/vel/crates/veld/src/services/chat/messages.rs) so conversational assistant entry can:
  - inspect the canonical operator queue
  - stage a bounded proposal over an existing action item
  - preserve that staging as an explicit `assistant.proposal.staged` event instead of applying a write
- Added focused integration coverage in [chat_assistant_entry.rs](/home/jove/code/vel/crates/veld/tests/chat_assistant_entry.rs) proving a conversational assistant entry can stage a proposal from the existing queue after capture/intervention work already exists
- Updated [types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so the web decoder understands the widened assistant-entry contract

## Verification

- `cargo fmt --all`
- `cargo test -p veld --test chat_assistant_entry -- --nocapture`
- `cargo test -p veld operator_queue -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`

## Notes

- The first proposal seam is intentionally staged-only: it surfaces a bounded suggestion over an existing action item, but it does not apply any mutation or bypass review/writeback gates.
- `veld` still emits the existing unused/dead-code warnings during Rust test builds.
