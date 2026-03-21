# Phase 50: Apple parity and reduced watch embodiment - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 50 brings iPhone, iPad, Mac, and reduced watch shells into parity with the canonical compact `Now` contract already embodied in web during Phase 49.

This phase owns Apple-shell embodiment of:

- the compact `Now` frame over shared Rust-owned transport
- thread-aware handoff and compact continuity cues
- compact mesh/trust posture over shared authority state
- reduced-watch embodiment that stays contract-faithful rather than divergent

This phase must stay thin over the Rust-owned `Now` DTOs, mesh summary, governed config, and thread continuation metadata already established in Phases 47-49. It must not introduce Apple-local product logic, local ranking, or a separate watch-specific policy model.

</domain>

<decisions>
## Implementation Decisions

### Surface ownership
- **D-01:** Apple `Now` must stay a compact execution surface, not a separate dashboard or summary narrative layer.
- **D-02:** `Inbox` remains the canonical owner of daily and carry-forward task queues; Apple `Now` only surfaces the highest-priority subset.
- **D-03:** `Threads` remains the deep continuity lane for explanation, follow-through, and multi-step work on Apple as it does on web.

### Apple parity rules
- **D-04:** iPhone, iPad, and Mac should consume the same typed `Now` blocks as web: header, status row, context line, mesh summary, nudge bars, task lane, and docked input continuity.
- **D-05:** Apple quick-entry, voice, and reduced-watch affordances are shell wrappers over backend-owned routing and thread continuity, not separate Apple-local capture policy.
- **D-06:** Reduced watch stays contract-faithful but density-reduced: top status, top nudge, current task, compact voice/capture entry, and reduced thread response/handoff.
- **D-07:** Client-mesh warnings may appear as compact `Now` trust posture on Apple, but deeper linking/repair remains in support/settings surfaces.
- **D-08:** Apple thread surfaces should reuse shared continuation category/open-target metadata instead of inventing local routing semantics.

### Non-goals
- **D-09:** Phase 50 does not widen into backend contract changes unless a parity blocker is found.
- **D-10:** Phase 50 does not redesign non-`Now` Apple surfaces beyond what is required for parity and truthful compact continuity.
- **D-11:** Phase 50 does not turn watch into a full conversation browser or planner.

### the agent's Discretion
- Exact SwiftUI structure and view splits, as long as visible behavior stays faithful to the canonical contract and remains thin over shared transport.
- Exact reduced-watch layout choices, as long as information architecture and handoff semantics stay aligned with web/iPhone/Mac.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- [`docs/product/now-surface-canonical-contract.md`](/home/jove/code/vel/docs/product/now-surface-canonical-contract.md) is the durable product authority for Apple parity.
- [`docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md`](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md) defines the shared Rust-owned transport and authority boundaries Apple must consume.
- [`clients/apple/Apps/VeliOS/ContentView.swift`](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift), [`clients/apple/Apps/VelMac/ContentView.swift`](/home/jove/code/vel/clients/apple/Apps/VelMac/ContentView.swift), and [`clients/apple/Apps/VelWatch/ContentView.swift`](/home/jove/code/vel/clients/apple/Apps/VelWatch/ContentView.swift) are the embodiment targets.
- [`clients/apple/VelAPI/Sources/VelAPI/Models.swift`](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/Models.swift) already carries the shared Rust-owned DTOs needed for parity work.

### Established Patterns
- Apple clients should present summary-first shells over backend-owned state and avoid Swift-local synthesis for product behavior.
- Phase 44 already aligned Apple shell hierarchy to `Now`, `Inbox`, `Threads`, `Projects`, and `Settings`; Phase 50 should deepen parity inside that shell, not invent a new hierarchy.
- Watch already has a bounded `Now` lane and quick actions; Phase 50 should tighten it to the reduced canonical contract rather than widen scope.

### Integration Points
- `clients/apple/Apps/VeliOS/ContentView.swift`
- `clients/apple/Apps/VelMac/ContentView.swift`
- `clients/apple/Apps/VelWatch/ContentView.swift`
- `clients/apple/VelAPI/Sources/VelAPI/Models.swift`
- `clients/apple/VelAPI/Tests/VelAPITests/DailyLoopTests.swift`
- `clients/apple/README.md`
- `docs/user/daily-use.md`

</code_context>

<specifics>
## Specific Ideas

- Land Apple parity in narrow passes: iPhone/iPad frame first, Mac compact continuity second, reduced watch third, then phase-close verification.
- Prefer removing stale summary/explanatory copy over layering new copy on top.
- Keep watch reduced by removing density, not by inventing different product rules.

</specifics>

<deferred>
## Deferred Ideas

- backend contract expansion unless parity reveals a concrete blocker
- deep Apple-only setup workflows outside what compact mesh parity requires
- broader widget/complication work beyond reduced-watch `Now` truth

</deferred>

---

*Phase: 50-apple-parity-and-reduced-watch-embodiment*
*Context gathered: 2026-03-21*
