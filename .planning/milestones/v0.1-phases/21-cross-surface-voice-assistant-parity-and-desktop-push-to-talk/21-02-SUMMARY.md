# 21-02 Summary

## Outcome

Completed the web/desktop push-to-talk polish slice over the shared assistant-entry seam.

## What changed

- Extended the web assistant-entry request boundary to carry optional voice provenance in [types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [chat.ts](/home/jove/code/vel/clients/web/src/data/chat.ts).
- Updated [MessageComposer.tsx](/home/jove/code/vel/clients/web/src/components/MessageComposer.tsx) so local browser speech-to-text:
  - uses hold-to-talk push-to-talk controls,
  - shows explicit supported/unsupported local-STT guidance,
  - keeps the transcript local until send,
  - submits through the same `/api/assistant/entry` path with explicit web voice provenance.
- Updated [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) so the primary assistant-entry panel teaches the push-to-talk path without inventing separate routing behavior.
- Added focused regression coverage in [MessageComposer.test.tsx](/home/jove/code/vel/clients/web/src/components/MessageComposer.test.tsx) and [NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx).
- Updated [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) to reflect the shipped desktop/browser voice story and typed fallback behavior.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/components/MessageComposer.test.tsx src/components/NowView.test.tsx`

## Notes

- This slice keeps web voice on the same assistant seam as typed entry; it does not introduce a separate browser-only conversation policy.
- Apple voice parity and bounded offline/cache alignment remain in `21-03`.
