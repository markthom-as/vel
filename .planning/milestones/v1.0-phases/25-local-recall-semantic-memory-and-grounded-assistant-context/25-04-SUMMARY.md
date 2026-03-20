# 25-04 Summary

## Outcome

Closed Phase 25 by aligning the shipped shell and documentation story around bounded local recall, backend-owned assistant context, and honest retrieval limits.

## What Changed

- updated `docs/api/chat.md` so the chat surface now explicitly documents `assistant_context` and the bounded local-recall contract behind assistant-capable responses
- updated `docs/api/runtime.md` so the runtime docs describe assistant recall as building on the same backend-owned inspect/`Now` substrate and preserve the current hybrid retrieval limit
- updated `docs/user/setup.md` and `docs/user/daily-use.md` so the operator-facing docs teach one honest recall story: local-first, persisted-data-backed, explainable, and still bounded
- updated `clients/web/src/components/ThreadView.tsx` and `clients/web/src/components/SettingsPage.tsx` so the web shell teaches bounded backend-owned recall instead of implying broad ambient memory
- updated `crates/vel-cli/src/commands/agent.rs` so the CLI grounding output now names the bounded local-recall scope explicitly

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx`
- `cargo test -p vel-cli agent_inspect -- --nocapture`
- `rg -n "semantic|recall|grounding|assistant|memory" docs/api/chat.md docs/api/runtime.md docs/user/setup.md docs/user/daily-use.md crates/vel-cli/src/commands clients/web/src/components`

## Notes

- This closeout intentionally tightened wording rather than widening implementation. The shipped assistant remains grounded in persisted Vel data and the current hybrid retrieval baseline.
- `vel-cli` still emits the same two pre-existing dead-code warnings in `client.rs` during tests.
