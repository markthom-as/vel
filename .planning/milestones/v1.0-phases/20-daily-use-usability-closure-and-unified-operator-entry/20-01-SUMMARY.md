# 20-01 Summary

## Outcome

Closed the known `ThreadView` / composer contract drift before widening Phase 20 assistant-entry work.

The current `Threads` empty state and shared composer now teach the intended assistant-first continuity posture consistently:

- `ThreadView` empty-state copy now says this is where `Now` or `Inbox` can escalate for deeper follow-up
- `MessageComposer` test coverage now locks the current ask/capture/talk placeholder contract explicitly
- nearby docs now state the current truth: the grounded assistant entry still lives under `Threads` today, while `Now`-first routing is follow-on Phase 20 work

## Files

- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/ThreadView.test.tsx`
- `clients/web/src/components/MessageComposer.test.tsx`
- `docs/api/chat.md`
- `docs/user/daily-use.md`

## Verification

- `npm --prefix clients/web test -- --run src/components/ThreadView.test.tsx src/components/MessageComposer.test.tsx`
- `rg -n "Threads|assistant|capture|conversation" docs/api/chat.md docs/user/daily-use.md`

## Notes

- This slice intentionally did not add new routing or backend behavior.
- It repaired stale placeholder expectations first so later assistant-entry work will fail on real regressions instead of copy drift.
