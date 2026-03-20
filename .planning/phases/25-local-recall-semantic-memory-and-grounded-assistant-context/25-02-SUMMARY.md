# 25-02 Summary

## Outcome

Improved local hybrid retrieval quality so non-capture semantic records can receive real lexical credit and survive hybrid candidate selection.

## What Changed

- broadened lexical scoring in `crates/vel-storage/src/repositories/semantic_memory_repo.rs` from capture-only FTS to a merged record-level lexical signal across semantic memory records
- changed hybrid candidate selection so rerank truncation uses combined preliminary lexical+semantic score instead of semantic score alone
- aligned final hit scoring to record-level lexical scores instead of looking lexical scores up by `source_id`
- added repo-level semantic-memory coverage proving projects and notes now get lexical credit in hybrid retrieval
- added `veld` integration coverage proving non-capture entities receive lexical score in real retrieval flows

## Verification

- `cargo fmt --all`
- `cargo test -p veld semantic_memory -- --nocapture`
- `cargo test -p veld retrieval -- --nocapture`
- `cargo test -p veld --test chat_grounding -- --nocapture`

## Notes

- The `cargo test -p veld semantic_memory -- --nocapture` pattern is broad and mostly filters down to the integration target; the concrete new recall assertions are demonstrated by the `retrieval` run and the new integration test.
- `veld` still emits the same pre-existing unused/dead-code warnings during Rust test builds.
