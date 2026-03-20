# 25-03 Summary

## Outcome

Assembled a stronger backend-owned assistant context pack from recall results so chat and assistant entry can return a bounded summary-first grounding artifact instead of only raw tool-shaped recall output.

## What Changed

- added `AssistantContextData` to `vel-api-types` and aligned the web decoders in `clients/web/src/types.ts`
- widened `CreateMessageResponse` and `AssistantEntryResponse` so assistant-capable chat flows can return typed `assistant_context`
- taught `crates/veld/src/services/chat/tools.rs` to build a bounded assistant context pack with summary, focus lines, recall provenance, and a grounding hint derived from the canonical agent-inspect seam
- taught `crates/veld/src/services/chat/assistant.rs` to render that context pack into a better LLM-facing tool-result block instead of only dumping raw JSON
- taught `crates/veld/src/services/chat/messages.rs` and `crates/veld/src/routes/chat.rs` to attach backend-owned `assistant_context` to assistant-capable message and assistant-entry responses
- tightened lexical query normalization in `crates/vel-storage/src/repositories/semantic_memory_repo.rs` so conversational punctuation does not break capture FTS during assistant-context assembly
- extended `crates/veld/tests/chat_grounding.rs` and `clients/web/src/types.test.ts` to verify the new contract and the grounded assistant path

## Verification

- `cargo fmt --all`
- `cargo test -p veld --test chat_grounding -- --nocapture`
- `cargo test -p veld context_runs -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`

## Notes

- The main bug found during this slice was an FTS failure caused by passing raw conversational punctuation into `captures_fts MATCH ?`; the fix now normalizes capture FTS queries through the same tokenization path used by record-level lexical scoring.
- `veld` still emits the same pre-existing unused/dead-code warnings during Rust test builds.
