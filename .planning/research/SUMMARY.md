# Project Research Summary

**Project:** Vel
**Domain:** local-first personal operator cognition runtime, focused on a strict daily-loop MVP
**Researched:** 2026-03-20
**Confidence:** HIGH

## Executive Summary

Vel v0.2 should not be treated as a broad cleanup of everything already shipped. The research converges on a much narrower true MVP backbone: one Rust-owned daily operator loop spanning `Now` overview, commitment selection, same-day drift detection, explainable reflow, thread-backed follow-through, and lightweight review. Experts would build this by tightening the existing Rust core and typed transport seams first, then letting web and Apple act as thin shells over one canonical product model.

The recommended approach is Rust-core-first, not shell-first and not migration-for-migration's-sake. Keep the current Rust workspace, Axum, Tokio, SQLx/SQLite, existing run/tracing model, React web shell, and SwiftUI Apple shell. Add only the missing product seams that complete the operator loop: a canonical `MvpOverview`/review/follow-through contract, stronger Rust-owned nudge and reflow services, and an Apple local calendar export path that feeds the same Rust-owned planner/reflow lane rather than introducing Apple-local planning logic.

The main risks are scope collapse, client-side policy drift, and turning calendar reflow into a planner rewrite. Mitigation is straightforward: freeze MVP scope around the current-day loop only, lock typed decision/read-model contracts before UI refresh, keep all planning/reflow/review semantics in Rust, and require provenance plus supervised apply for any meaningful reflow or action change.

## Key Findings

### Recommended Stack

The stack recommendation is conservative on purpose. The repo already has the right technical foundation, and the milestone problem is product closure rather than platform churn. Keep the Rust workspace (`vel-core`, `vel-storage`, `veld`, `vel-api-types`) as the product authority, keep Tokio/Axum/SQLx+SQLite/tracing as the runtime baseline, keep React 19 + TypeScript + Vite as a thin web shell, and keep SwiftUI + `VelAPI` + `VelAppleModules` as the Apple shell boundary.

The one material addition worth making is Apple-local calendar export through EventKit into the existing Rust ingest path. That is the only credible local-first calendar move that improves the MVP without forking planner logic into Swift or inventing a browser-local calendar story that the platform cannot actually support.

**Core technologies:**
- Rust workspace: canonical domain, services, storage, and transport seams; this is the MVP product core.
- Tokio + Axum: preserve the current typed async daemon/API boundary instead of opening a new runtime lane.
- SQLx + SQLite: keep SQLite as the only durable store; the missing work is typed state/read models, not a new datastore.
- `tracing` + run events: provenance is mandatory for trustable reflow, nudges, threads, and review.
- React 19 + TypeScript + Vite: keep the web shell thin and typed; no new client state architecture is justified.
- SwiftUI + `VelAPI` + `VelAppleModules`: keep Apple as a presentation/offline queue shell, with EventKit read/export as the narrow native addition.

### Expected Features

The MVP must feel like a serious daily operator surface, not a dashboard and not a generic chat shell. Table stakes are a decision-first `Now` overview with one dominant current action, inline commitment editing, one unified today lane across time and commitments, same-day reflow when the day drifts, actionable nudges, thread escalation for messy work, lightweight closeout review, and parity across web and Apple because the behavior is Rust-owned.

The differentiators worth keeping are also narrow: one canonical Rust-owned daily loop across shells, explainable local-first reflow, nudges grounded in persisted context rather than assistant prose, threads as continuity rather than chat, and review as operational closure rather than analytics.

**Must have (table stakes):**
- Daily overview with one dominant current action and immediate time context.
- Commitment selection plus lightweight inline editing.
- Unified today lane across calendar events, commitments, and priority work.
- Same-day reflow with explainable outcomes and operator-supervised apply.
- Actionable nudges/check-ins tied to drift, blockages, and stale commitments.
- Thread escalation for non-trivial follow-through.
- Lightweight review/closeout with carry-forward.
- Cross-surface consistency across web and Apple.

**Should have (competitive):**
- Rust-owned canonical daily loop across shells.
- Explainable local reflow with visible rationale.
- Provenance-grounded nudges and suggestions.
- Threads as structured continuity, not assistant chat.
- Review as closure that prepares tomorrow.

**Defer (v2+ / out of scope):**
- Multi-day or autonomous planning.
- Generic chat-first assistant shell.
- Project/workspace management expansion.
- Silent or opaque automation.
- Heavy analytics, journaling, or review scoring.
- Routine-builder redesign or drag-and-drop planner UI.
- Shell-specific planning logic.
- Broad FFI migration, new databases, CalDAV/Exchange/provider expansion, or full local calendar write-back.

### Architecture Approach

The architecture research is consistent: do not introduce a new product stack, and do not let shells own policy. The correct move is to converge the existing Rust seams into one explicit MVP service cluster inside `veld`, backed by typed `vel-core` flow models and persisted through `vel-storage`, with `vel-api-types` carrying canonical queries and mutations to web and Apple.

**Major components:**
1. `vel-core` MVP flow types: canonical states, commands, and invariants for overview, daily loop, nudges, reflow, threads, and review.
2. `vel-storage` repositories: typed persistence for sessions, proposals, review state, thread continuation, and calendar evidence.
3. `veld` overview service: the primary aggregate read model for current-day truth.
4. `veld` daily loop service: commitment flow authority and session continuity.
5. `veld` orientation/nudge service: ranked interventions with explicit reasons.
6. `veld` reflow service: same-day schedule repair, proposal generation, and apply-or-stage decisions.
7. `veld` threads/review services: long-running follow-through and end-of-day closure over persisted outcomes.
8. Web and Apple shells: rendering, input, caching, and queueing only; no local product policy.

### Critical Pitfalls

1. **Treating the whole shipped surface as the MVP** — freeze scope around the current-day operator loop only.
2. **Turning Rust-core-first into a full re-platforming mandate** — extract only the contracts and services needed for the MVP flows.
3. **Thin clients in name only** — ban shell-owned ranking, reflow, review, and decision synthesis.
4. **Refreshing UI before locking the decision model** — define typed overview/nudge/reflow/review contracts before screen redesign.
5. **Letting same-day reflow become a planner rewrite** — bound it to current-day repair, explicit constraints, and supervised apply.
6. **Weak provenance for local-first mutations** — persist inputs, reasons, run/thread IDs, and terminal state for every meaningful suggestion or apply action.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Lock the Canonical MVP Contracts
**Rationale:** Everything else depends on a stable decision model. If UI or Apple work begins first, shell drift returns immediately.
**Delivers:** Typed `MvpOverview`, nudge cards, reflow proposal/apply results, thread-continuation summaries, and review summary contracts in `vel-core`/`vel-api-types`, plus typed persisted payloads in `vel-storage`.
**Addresses:** Daily overview, commitment continuity, nudges, reflow, review, cross-surface parity.
**Avoids:** UI-first redesign, JSON-blob regression, and thin-clients-in-name-only failure.

### Phase 2: Make Rust Own Current-Day Truth
**Rationale:** Once contracts exist, the backend needs to become the single source of operator-facing state before shell simplification starts.
**Delivers:** `Now`/overview aggregation in Rust, unified orientation/nudge seam, and explicit thread-continuation and review posture feeding one canonical read model.
**Uses:** Existing Rust workspace, Axum DTO boundary, SQLite persistence, tracing/run events.
**Implements:** Overview, daily loop, orientation, threads, and review service cluster.

### Phase 3: Finish Explainable Same-Day Reflow
**Rationale:** Reflow is the hardest MVP differentiator and depends on canonical commitments plus current-day truth.
**Delivers:** Rust-owned same-day drift detection, proposal generation, supervised apply/stage semantics, provenance trails, and thread-backed continuation for ambiguous edits.
**Addresses:** Unified today lane, same-day repair, actionable nudges, explainable reflow.
**Avoids:** Planner rewrite, silent automation, and provider/UI split-brain.

### Phase 4: Add Local-First Calendar Input the Narrow Way
**Rationale:** Local-first calendar reflow is feasible only if event acquisition stays adapter-scoped and planner logic stays in Rust.
**Delivers:** Apple EventKit read/export into the existing local-source or `.ics` ingest path, plus explicit freshness/authority handling. No Apple-local planner logic and no broad write-back.
**Addresses:** Local-first calendar feasibility for the MVP.
**Avoids:** Apple-only planning contracts, browser-local calendar dead ends, and provider sprawl.

### Phase 5: Thin the Shells Around the Canonical Loop
**Rationale:** Shell work should consume stable Rust seams, not invent them.
**Delivers:** Web and Apple `Now`/thread/review surfaces driven directly by backend DTOs, minimal client shaping, offline queueing where already supported, and parity fixtures against shared payloads.
**Addresses:** Cross-surface consistency, one dominant current action, actionable nudges, thread escalation.
**Avoids:** Client-side policy drift and migration-by-screen-count.

### Phase 6: Close the End-to-End MVP Loop
**Rationale:** Only after contracts, current-day truth, reflow, and shell consumption exist does it make sense to verify the real daily loop.
**Delivers:** End-to-end operator rehearsal of overview -> commitment -> nudge/reflow -> thread follow-through -> review, degraded-state handling, and explicit out-of-scope/deferred list for post-MVP work.
**Addresses:** Trust, closeout, and milestone closure.
**Avoids:** Declaring completion from polished screens without a working operator loop.

### Phase Ordering Rationale

- Contracts come first because the dominant failure mode is UI or shell work outrunning the decision model.
- Current-day overview and orientation come before reflow because reflow depends on canonical commitments, schedule truth, and typed intervention reasons.
- Calendar locality comes after the reflow core because local-first value is an input-adapter problem, not a planner-logic problem.
- Shell refresh is intentionally late so web and Apple can become thinner rather than hardening fresh drift.
- Verification is its own phase because progress should be measured by a completed operator loop, not by migrated components or prettier screens.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3:** Reflow apply/stage policy and provenance shape need careful contract validation because trust depends on reversible, explainable outcomes.
- **Phase 4:** Apple EventKit export format choice (`.ics` reuse versus a small typed snapshot) needs a short focused implementation spike.
- **Phase 5:** Apple offline queue/cache behavior should be checked against the current embedded/daemon contract to avoid accidental policy leakage into Swift.

Phases with standard patterns (skip research-phase):
- **Phase 1:** Typed Rust contracts and DTO/schema publication follow existing repo architecture and documentation rules.
- **Phase 2:** Service aggregation and storage-backed read-model composition are already established in the current Rust stack.
- **Phase 6:** Cross-surface verification and degraded-state rehearsal use standard repo verification practices already established by the implementation protocol.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Strongly grounded in current repo architecture and official framework/platform docs; recommendation is mostly to keep the existing baseline. |
| Features | HIGH | Research cleanly converges on a narrow current-day loop with clear table stakes, differentiators, and anti-features. |
| Architecture | HIGH | Backed by repo authority docs plus concrete existing services/routes; the main recommendation is consolidation, not invention. |
| Pitfalls | HIGH | Risks are consistent across repo rules and the four research outputs; they map directly to likely execution failures. |

**Overall confidence:** HIGH

### Gaps to Address

- Local calendar mutation remains intentionally unresolved for MVP: keep v0.2 to local calendar read/export and supervised same-day reflow proposals unless a narrowly bounded apply path already exists.
- The best export format for Apple-local events is still a tactical choice: prefer the existing `.ics` ingest lane first, and only add a typed snapshot if `.ics` is too lossy for same-day reflow.
- Shell degraded states need explicit acceptance criteria: permissions denied, stale calendar data, failed apply actions, and offline/queued Apple interactions should be tested as first-class MVP cases.
- Vocabulary drift is still a risk: `Now`, `thread`, `review`, `nudge`, and `reflow` should be normalized in contracts and UI copy before implementation spreads.

## Sources

### Primary (HIGH confidence)
- `.planning/research/STACK.md` — recommended stack, calendar feasibility, and explicit out-of-scope guidance
- `.planning/research/FEATURES.md` — MVP table stakes, differentiators, anti-features, and dependency order
- `.planning/research/ARCHITECTURE.md` — canonical Rust-core-first architecture and migration order
- `.planning/research/PITFALLS.md` — scope, client drift, reflow, and provenance risks
- `.planning/PROJECT.md` — active product direction and accepted decisions
- `docs/MASTER_PLAN.md` — implementation truth and current roadmap authority
- `README.md` — repo entrypoint and current product baseline

### Secondary (MEDIUM confidence)
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` — cross-surface product-core boundary
- `docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md` — Apple integration boundary
- `docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md` — Apple embedded/offline contract framing
- `docs/user/integrations/local-sources.md` and `docs/user/integrations/apple-macos.md` — local-source and Apple integration patterns
- `crates/veld/src/services/now.rs`, `daily_loop.rs`, `reflow.rs`, `apple_voice.rs`, and `routes/threads.rs` — current implementation seams

### Tertiary (LOW confidence)
- External product pattern references cited in research (`Sunsama`, `Routine`, `Akiflow`) — useful as UX pattern checks, not as architectural authority
- MDN File System Access API reference — confirms browser-local calendar limits but does not materially affect the core recommendation

---
*Research completed: 2026-03-20*
*Ready for roadmap: yes*
