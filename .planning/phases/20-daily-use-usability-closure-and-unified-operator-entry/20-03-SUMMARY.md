# 20-03 Summary

## Outcome

Completed the web embodiment slice for the backend-owned assistant-entry seam.

`Now` can now launch the grounded assistant path directly, and the web shell follows backend-returned `route_target` outcomes instead of guessing locally whether text should become capture, Inbox work, or thread continuity.

## Shipped changes

- Added typed web transport support for the assistant-entry contract in [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts).
- Added the thin shared helper for `POST /api/assistant/entry` in [clients/web/src/data/chat.ts](/home/jove/code/vel/clients/web/src/data/chat.ts).
- Switched [clients/web/src/components/MessageComposer.tsx](/home/jove/code/vel/clients/web/src/components/MessageComposer.tsx) to the shared assistant-entry route so thread continuity no longer uses a separate client-owned send path.
- Updated [clients/web/src/components/ThreadView.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx) to consume the shared response contract while preserving optimistic send reconciliation.
- Added a primary assistant-first entry panel to [clients/web/src/components/NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx), including backend-owned landing into `Inbox`, `Threads`, or inline handling.
- Wired the navigation seam through [clients/web/src/components/MainPanel.tsx](/home/jove/code/vel/clients/web/src/components/MainPanel.tsx) and [clients/web/src/App.tsx](/home/jove/code/vel/clients/web/src/App.tsx).
- Updated [docs/api/chat.md](/home/jove/code/vel/docs/api/chat.md) and [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) to reflect shipped `Now` + `Threads` reuse of the assistant-entry seam.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/ThreadView.test.tsx src/components/MessageComposer.test.tsx src/components/MainPanel.test.tsx src/types.test.ts`

## Notes

- The backend currently routes live web entry to `inbox` or `threads`; `inline` is now typed and rendered in the web shell, but remains reserved for future backend handling.
- This slice did not change Apple or CLI entry behavior.
