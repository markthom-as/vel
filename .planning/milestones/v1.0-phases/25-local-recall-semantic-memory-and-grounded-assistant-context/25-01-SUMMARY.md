# 25-01 Summary

## Outcome

Published the first typed recall-context contract on top of the existing semantic retrieval seam.

## What Changed

- added `RecallContextPack`, `RecallContextHit`, and `RecallContextSourceCount` to `vel-core` so recall-oriented context assembly has one backend-owned typed shape instead of assistant-only ad hoc memory state
- added matching transport DTOs in `vel-api-types` and decoder coverage in `clients/web/src/types.ts`
- added `build_recall_context_pack` in `crates/veld/src/services/retrieval.rs`
- exposed the new bounded contract through `vel_get_recall_context` in `crates/veld/src/services/chat/tools.rs`
- updated `crates/veld/tests/chat_grounding.rs` so the assistant grounding path now uses the typed recall-context seam rather than only raw `vel_search_memory`
- documented the additive contract in `docs/cognitive-agent-architecture/cognition/semantic-memory-contract.md`

## Verification

- `cargo fmt --all`
- `cargo test -p vel-core semantic -- --nocapture`
- `cargo test -p veld chat::tools -- --nocapture`
- `cargo test -p veld --test chat_grounding -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`

## Notes

- This slice is contract-first only. It does not yet claim improved retrieval quality or richer assistant context assembly.
- `veld` still emits the same pre-existing unused/dead-code warnings during Rust test builds.
