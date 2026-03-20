# 20-02 Summary

## Outcome

Added the first backend-owned unified assistant-entry seam.

The new authenticated `POST /api/assistant/entry` route now accepts operator text outside a preselected thread and returns typed backend-owned route outcomes:

- `inbox` for capture-like entry
- `threads` for continuity / follow-up entry

The backend remains the owner of that routing decision. The route persists through the existing conversation/message model, archives capture-only conversations out of default thread lists, and safely returns `assistant_error` when no model is configured instead of making remote inference mandatory.

Capture-like entry now also surfaces into `Inbox` through the existing intervention queue by marking the persisted user message with backend-owned entry metadata.

## Files

- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/services/chat/messages.rs`
- `crates/veld/src/services/chat/interventions.rs`
- `crates/veld/tests/chat_assistant_entry.rs`
- `docs/api/chat.md`

## Verification

- `cargo fmt --all`
- `cargo test -p veld --test chat_assistant_entry -- --nocapture`
- `cargo test -p veld chat::tools -- --nocapture`

## Notes

- `chat::tools` still runs with the repo’s existing `dead_code` warning noise.
- The new route is implemented and documented, but the web shell has not fully embodied the `Now`-first flow yet; that is the next slice.
