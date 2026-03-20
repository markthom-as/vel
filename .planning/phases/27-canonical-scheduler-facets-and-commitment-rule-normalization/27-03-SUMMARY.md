# 27-03 Summary

## Completed

- widened assistant-facing transport contracts so bounded assistant context can carry typed commitments with canonical `scheduler_rules`
- updated backend assistant context assembly to surface normalized scheduler semantics through summary and focus lines instead of relying on raw-label-only reasoning
- aligned the command-language route and web boundary with the widened commitment contract in the same slice
- verified that recall and grounded assistant flows can now expose scheduler facets like `block:*`, duration, `time:*`, `cal:free`, `urgent`, and `defer`

## Main files

- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/chat/tools.rs`
- `crates/veld/src/routes/command_lang.rs`
- `clients/web/src/types.ts`
- `clients/web/src/types.test.ts`
- `docs/cognitive-agent-architecture/architecture/canonical-scheduler-facets.md`

## Verification

- `cargo fmt --all`
- `cargo test -p vel-api-types assistant_context_round_trips_summary_and_focus_lines -- --nocapture`
- `cargo test -p veld chat::tools -- --nocapture`
- `cargo test -p veld --test chat_grounding -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`

## Notes

- assistant context remains intentionally bounded; this slice makes scheduler semantics durable and explainable inside that bounded context rather than widening into a broader planner
- raw provider labels still remain available for compatibility and search, but backend-owned assistant grounding now prefers canonical scheduler semantics when they exist
