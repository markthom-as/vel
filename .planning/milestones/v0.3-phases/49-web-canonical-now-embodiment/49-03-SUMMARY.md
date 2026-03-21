## 49-03 Summary

Finished the compact web `Now` input continuity seam in [`NowView.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.tsx) by preserving a thread handoff path for docked-input outcomes instead of treating inline or inbox routing as dead ends. Docked input outcomes now keep an explicit `Open thread` handoff, and inline transcript replies open the backing thread directly from the compact preview bubble.

Updated [`ThreadView.tsx`](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx) so continuation metadata also shows the typed continuation category and open target already supplied by the backend contract. Extended [`NowView.test.tsx`](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) and [`ThreadView.test.tsx`](/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx) to verify the new handoff and metadata behavior.

Verification:

- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/ThreadView.test.tsx`
