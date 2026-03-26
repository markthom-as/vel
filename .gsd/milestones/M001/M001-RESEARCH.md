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

# Architecture Patterns

**Domain:** Vel v0.2 true MVP closure
**Researched:** 2026-03-20

## Recommended Architecture

Vel should close the MVP by converging the existing daily-use seams into one Rust-owned operator-flow core inside the current `vel-core -> vel-storage -> vel-api-types -> veld` architecture, not by adding new shell-local coordinators. The repo already has the right primitives: backend-owned `Now`, `daily_loop`, `reflow`, `planning_profile`, `threads`, and Apple voice routes. The milestone should reshape those into one explicit MVP product lane that web and Apple both consume.

The core rule for this milestone is:

- `vel-core` defines the canonical domain types and flow states for daily overview, nudges, calendar reflow, thread-backed follow-through, and review.
- `vel-storage` persists typed flow/session/thread/proposal state and calendar evidence; it does not know about HTTP or shell DTOs.
- `veld` services orchestrate the MVP flows and produce canonical read models.
- `vel-api-types` exposes those read models and mutations to networked shells.
- `clients/web` and `clients/apple` render those read models, collect operator input, and submit typed commands. They do not derive product policy, reflow logic, nudge ranking, or review semantics locally.

Recommended product-core shape:

```text
Shells (web / Apple / CLI)
  -> typed commands + queries
Transport seam (vel-api-types)
  -> MVP flow services in veld
     - overview/orientation service
     - daily loop service
     - nudge/orientation service
     - calendar reflow service
     - thread follow-through service
     - review service
  -> vel-storage repositories
  -> vel-core domain types / invariants
```

The milestone should not introduce a new parallel "MVP engine" crate unless `veld/services` becomes impossible to reason about. The practical move is to create a clearly bounded MVP service cluster in Rust first, then extract only if the seams prove stable.

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `vel-core` MVP flow types | Canonical flow states, commands, transitions, review/nudge/reflow semantics | `vel-storage`, `veld` services |
| `vel-storage` flow repositories | Persist daily sessions, thread-backed proposals, review state, calendar/reflow evidence, local cache snapshots | SQLite, `veld` services |
| `veld::services::overview` or expanded `now` service | Build the top-level daily overview/read model from context, commitments, events, nudges, reflow status, thread continuity | `vel-storage`, `daily_loop`, `reflow`, `nudge`, `review` |
| `veld::services::daily_loop` | Morning overview/commitment flow state machine and submit/resume behavior | `vel-storage`, overview service, threads |
| `veld::services::orientation` or `nudge_engine` reshape | Rank suggestions/nudges and explain why they surfaced | `vel-storage`, overview service |
| `veld::services::reflow` | Detect schedule drift, compute typed same-day reflow proposal, stage/apply thread-backed edits | `vel-storage`, calendar adapters, threads, commitment scheduling |
| `veld::services::threads` adjunct seam | Own thread-backed continuation for long-running MVP actions: reflow edits, planning changes, assistant escalations, review items | `vel-storage`, overview/review/assistant flows |
| `veld::services::review` | Build end-of-loop and periodic review summaries over persisted sessions, actions, conflicts, threads, and outcomes | `vel-storage`, overview service |
| Calendar adapter boundary | Import calendar events and perform local-first reflow application where possible | external providers via existing integration services |
| `vel-api-types` MVP DTOs | Query/read-model and mutation DTOs for overview, session turn, reflow action, thread continuation, review summaries | web, Apple, CLI |
| Web shell | Render overview, thread follow-through, review, and input controls | `/v1/now`, `/v1/daily-loop/*`, `/v1/threads`, review endpoints |
| Apple shell | Render compact overview/voice-first actions, cache read models, queue safe offline inputs, submit to same Rust flows | `/v1/apple/voice/turn`, `/v1/now`, `/v1/daily-loop/*`, `/v1/threads` |

### Data Flow

1. Overview query:
   Shell requests one canonical overview read model.
   Rust assembles schedule truth, commitments, nudges, thread follow-through, reflow status, and review posture from persisted state.
   Shell renders it without recomputing policy.

2. Commitment flow:
   Shell starts/resumes the daily loop.
   `daily_loop` owns prompts, transitions, history, and output state.
   Resulting commitments and follow-through records persist in storage and appear back in overview/review read models.

3. Suggestion/nudge flow:
   Signals, commitments, and stale/conflict state feed Rust ranking logic.
   Shell receives typed nudges with reasons and available actions.

4. Calendar reflow flow:
   Calendar inputs are normalized in Rust.
   `reflow` derives a same-day proposal.
   Direct-apply remains Rust-owned when the action is safe and local; otherwise Rust stages a thread-backed edit path.

5. Thread continuation flow:
   Any longer-lived action becomes a typed thread-backed continuation.
   Web and Apple deep-link into the same thread record instead of inventing local continuation semantics.

6. Review flow:
   Review service reads persisted session outcomes, open loops, skipped commitments, conflicts, and applied edits.
   Shell renders summaries and review actions from the same source.

## Patterns to Follow

### Pattern 1: One Canonical MVP Read Model
**What:** Treat the daily overview as the canonical operator-facing aggregate, built in Rust and reused across shells.
**When:** For `Now`, daily overview, orientation, Apple compact surfaces, and any future desktop shell.
**Example:**

```rust
pub struct MvpOverview {
    pub summary: OverviewSummary,
    pub schedule: ScheduleSnapshot,
    pub active_commitment_flow: Option<DailyLoopSession>,
    pub nudges: Vec<NudgeCard>,
    pub reflow: Option<ReflowCard>,
    pub follow_through: Vec<ThreadContinuation>,
    pub review: ReviewSnapshot,
}
```

Use the existing `NowOutput` as the starting point, but tighten it into a product read model instead of letting web and Apple each shape their own slice.

### Pattern 2: Threads As The Long-Running Follow-Through Seam
**What:** Use thread-backed records for any MVP action that needs review, continuation, operator judgment, or later resumption.
**When:** Reflow edits, planning-profile edits, assistant escalations, unresolved nudges, review tasks.
**Example:**

```rust
pub enum FollowThroughKind {
    ReflowEdit,
    PlanningEdit,
    ReviewItem,
    AssistantContinuation,
}
```

This matches the existing `reflow_edit` and `planning_profile_edit` behavior and should be expanded rather than replaced.

### Pattern 3: Local Calendar Reflow In Rust, Not In Shells
**What:** Keep calendar normalization, conflict detection, and proposal generation in Rust. Let adapters handle provider I/O, but let the same-day plan logic live in core services.
**When:** Recomputing the rest of the day, handling stale schedule drift, and deciding whether a change is directly applyable or requires thread review.
**Example:**

```rust
pub struct ReflowDecision {
    pub proposal: ReflowProposal,
    pub apply_mode: ApplyMode,
    pub reason_lines: Vec<String>,
}
```

If a purely local calendar store or Apple EventKit bridge becomes available later, it should implement the same adapter contract. The decision logic still stays Rust-owned.

### Pattern 4: Shells May Cache And Queue, But Not Decide
**What:** Web and Apple may cache read models, keep local drafts, and queue safe offline mutations. They must not change ranking, transition, or review logic.
**When:** Apple offline voice/capture, web optimistic refresh, transient local composition.
**Example:**

```text
offline draft -> queued typed mutation -> Rust service validates/applies -> canonical state broadcast back
```

## Anti-Patterns to Avoid

### Anti-Pattern 1: Shell-Owned MVP Flow Logic
**What:** Swift or React deciding morning flow steps, reflow policy, review thresholds, or thread lifecycle.
**Why bad:** It forks behavior across surfaces and breaks the Phase 13 architecture lock.
**Instead:** Add a Rust service/read model/DTO seam first, then render it in shells.

### Anti-Pattern 2: Reflow Split Across Providers And UI
**What:** Provider adapter computes one version of schedule truth while shells compute another.
**Why bad:** Calendar drift and "why did this move?" explanations become inconsistent.
**Instead:** Normalize provider input in adapters, then run one Rust-owned reflow decision path.

### Anti-Pattern 3: New Untyped JSON Buckets For MVP State
**What:** Putting new MVP flow state into growing ad hoc JSON blobs.
**Why bad:** It deepens the exact `current_context` debt the repo rules warn against.
**Instead:** Add typed structs in `vel-core` and persist versioned payloads at the storage edge.

### Anti-Pattern 4: Assistant-As-Separate-Architecture
**What:** Treating assistant, voice, and thread continuity as a second product core.
**Why bad:** The user’s primary MVP flows then diverge by entry mode.
**Instead:** Assistant and Apple voice should enter the same daily-loop, thread, reflow, and review seams.

## Recommended Reshape For Existing Components

### Modify

| Existing Component | Change Needed | Why |
|-------------------|---------------|-----|
| `crates/veld/src/services/now.rs` | Promote from broad snapshot builder to the canonical MVP overview aggregator | This becomes the main cross-surface read model |
| `crates/veld/src/services/daily_loop.rs` | Keep as state-machine authority, but align its outputs directly into overview/review continuity | Daily commitment flow is already Rust-owned; it should anchor the MVP |
| `crates/veld/src/services/reflow.rs` | Make it the only schedule-repair decision seam and feed overview + threads + review | Reflow is an MVP capability, not a side lane |
| `crates/veld/src/services/nudge_engine.rs` / suggestions services | Merge or align into one orientation/nudge seam with typed explanation output | Avoid parallel "suggestions" vs "nudges" products |
| `crates/veld/src/routes/threads.rs` and thread service usage | Formalize thread types used by MVP follow-through and expose filtered continuation views | Threads become the shared continuation substrate |
| `crates/veld/src/services/apple_voice.rs` | Keep as Apple adapter only; route every supported action into canonical overview/daily-loop/reflow/thread services | Apple voice should stay thin even when offline-capable |
| `clients/web/src/data/*` | Reduce shell-side shaping to decoding, fetch, and UI selection | Web should stop carrying product synthesis debt |
| `clients/apple/VelAPI` and Apple modules | Keep offline cache/queue and platform glue only; stop any local derivation of continuation or overview semantics | Preserves embedded/local portability later |

### Add Or Make Explicit

| New/Explicit Component | Responsibility |
|------------------------|---------------|
| `MvpOverview` read-model contract | One typed aggregate for daily overview, nudges, reflow, continuation, and review posture |
| `ThreadContinuation` / follow-through contract | Typed thread summary for the actions that matter in MVP |
| `ReviewSummary` service + DTO | Canonical review surface over sessions, unresolved items, and applied/failed follow-through |
| Calendar reflow adapter contract | Explicit provider/local calendar mutation boundary with safe apply vs stage semantics |
| Shared MVP service module boundary | A documented service cluster inside `veld` covering overview, loop, orientation, reflow, threads, review |

## Suggested Migration Order

1. **Lock the contracts first**
   Refine `vel-core` types and `vel-api-types` DTOs for:
   `MvpOverview`, typed nudge cards, follow-through thread summaries, review summary, and calendar reflow action/apply results.

2. **Make `Now` the canonical overview aggregator**
   Reshape `services/now.rs` around MVP read-model ownership.
   Do this before UI rework so Phase 40 and the new milestone both consume the same truth.

3. **Normalize orientation logic**
   Consolidate `suggestions` and `nudge_engine` outputs into one Rust-owned orientation seam feeding the overview.

4. **Finish calendar reflow as a product-core seam**
   Keep proposal generation and apply/stage decisions in Rust.
   Add an explicit adapter contract for local-first calendar mutation where possible.

5. **Formalize threads as continuation**
   Introduce typed continuation summaries and filtered queries so overview/review can surface the same pending work across web and Apple.

6. **Add review as a first-class service**
   Build canonical review read models from persisted daily-loop outcomes, continuation threads, skipped commitments, conflicts, and reflow/apply history.

7. **Thin the shells**
   Update web loaders and Apple models to consume the new overview/review/continuation DTOs directly.
   Delete or stop extending shell-local shaping once the canonical Rust read models exist.

8. **Only then widen Apple embedded portability**
   If milestone work touches the Phase 37 embedded path, point it at the same Rust services/contracts.
   Do not create an Apple-specific product model.

## Documentation And Contract Updates To Refine During The Milestone

| Doc / Contract | Refinement Needed |
|----------------|-------------------|
| `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` | Add an explicit MVP closure section naming overview, reflow, threads, and review as the canonical shared product seams |
| `docs/api/runtime.md` | Document the MVP query/mutation surfaces and identify overview/review/thread-continuation endpoints as shared cross-surface contracts |
| `docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md` | Clarify that embedded Apple flows consume the same overview/reflow/thread/review services, with offline queue/cache as adapter concerns only |
| `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md` | Register the new/updated DTOs and typed payloads for overview, continuation, review, and calendar reflow apply results |
| `clients/apple/README.md` | Tighten the statement that Apple owns presentation/offline queue only, while MVP flow semantics stay in Rust |
| Web client boundary docs | Document that `clients/web/src/data/*` is a transport adapter layer, not a policy layer |
| Contract manifest / checked-in schemas | Add machine-readable schema coverage for any new DTOs or versioned persisted payloads introduced in this milestone |

## Scalability Considerations

| Concern | At 100 users | At 10K users | At 1M users |
|---------|--------------|--------------|-------------|
| Overview aggregation | Single SQLite-backed read model is fine | Add precomputed projections/caching, keep same contract | Requires projection stores and background refresh, but same product seam |
| Calendar reflow computation | On-demand per request is fine | Cache normalized day inputs and recent proposals | Dedicated planner workers may be needed, but not shell logic |
| Thread continuation | Direct query per screen is fine | Add filtered indexes and summary projections | Separate continuation projection service, still Rust-owned |
| Review generation | Build on demand | Add incremental materialized review summaries | Background summarization/projection layer |
| Apple/web parity | Shared DTOs keep parity manageable | Contract versioning becomes important | Adapter version negotiation likely required |

## Milestone Recommendation

Build the v0.2 MVP around one Rust-owned operator loop:

- `overview` is the primary read model
- `daily_loop` owns commitment flow
- `orientation` owns nudges/suggestions
- `reflow` owns same-day schedule repair
- `threads` own long-running continuation
- `review` closes the loop over outcomes

That gives web and Apple one portable product core and keeps the existing architecture intact. The correct migration is not "move everything into Apple/web parity layers"; it is "finish the Rust-owned MVP seams, then let web and Apple become thinner."

## Sources

- `.planning/PROJECT.md` — HIGH confidence
- `.planning/ROADMAP.md` — HIGH confidence
- `.planning/STATE.md` — HIGH confidence
- `.planning/codebase/ARCHITECTURE.md` — HIGH confidence
- `docs/MASTER_PLAN.md` — HIGH confidence
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` — HIGH confidence
- `crates/veld/src/services/now.rs` — HIGH confidence
- `crates/veld/src/services/daily_loop.rs` — HIGH confidence
- `crates/veld/src/services/reflow.rs` — HIGH confidence
- `crates/veld/src/services/apple_voice.rs` — HIGH confidence
- `crates/veld/src/routes/threads.rs` — HIGH confidence
- `crates/veld/src/routes/planning_profile.rs` — HIGH confidence

# Technology Stack

**Project:** Vel v0.2 true MVP
**Researched:** 2026-03-20

## Recommended Stack

### Core Runtime

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Rust workspace (`vel-core`, `vel-storage`, `veld`, `vel-api-types`) | current repo baseline (`edition = 2021`) | Canonical MVP product logic, orchestration, and transport | This milestone should consolidate more behavior into the existing Rust-owned core, not introduce a second product stack. The repo already has the right authority split: domain in `vel-core`, storage in `vel-storage`, thin transport in `vel-api-types`, and runtime host in `veld`. |
| Tokio | `1.44.x` | Async runtime for daemon/services | Keep current baseline. No runtime change is justified for MVP closure. |
| Axum | `0.7.x` | Typed HTTP boundary for web and Apple shells | Keep current HTTP-first boundary as the default cross-surface contract. This matches the documented Apple/web adapter model and avoids shell-owned logic drift. |
| SQLx + SQLite | `0.8.x` + SQLite | Local-first persistence | Keep SQLite as the only durable store for MVP. The milestone needs stronger Rust-owned read models and reflow/application state, not a database change. |
| `tracing` / existing run-event model | current repo baseline | Provenance and explainability | Daily overview, nudges, threads, and review all need stronger run/event evidence, but that should land on the existing tracing/run stack rather than a new observability platform. |

### Web Shell

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| React | `19.2.4` in repo | Minimal operator shell over typed backend state | Keep React 19 and continue treating the browser as a thin shell over Rust-owned read models. Do not move MVP logic into client-side reducers. |
| React DOM | `19.2.4` in repo | Browser rendering | No change needed. |
| TypeScript | `5.9.3` in repo | DTO decoding and shell safety | Keep strict typed boundary decoding. It is already the right seam for a thin web shell. |
| Vite | `8.0.0` in repo | Dev/build toolchain | Keep current tooling. This milestone is product closure, not frontend toolchain migration. |
| Existing typed data layer (`src/data/*`, `src/types.ts`) | repo-local | API adapter and shell hydration | Reuse it. Do not add a second client data abstraction unless an actual bottleneck appears. |

### Apple Shell

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| SwiftUI | current Apple-platform baseline, Swift tools `5.9` in repo | Minimal native shell for iPhone, watch, and macOS | Keep SwiftUI as the presentation layer. It is already the documented Apple shell boundary. |
| `VelAPI` | repo-local | Shared HTTP transport adapter to `veld` | Keep as the default path. Apple is still HTTP-first today, and this milestone should not replace that truth. |
| `VelAppleModules` | repo-local | Explicit Apple shell/application/platform seams | Keep and use these modules to prevent Swift from becoming the owner of planner, review, or thread policy. |
| EventKit | Apple framework | Optional local calendar read/export seam for Apple-local calendar input | This is the only material Apple-side addition worth making for MVP: read local calendars with user permission, then export/sync them into the Rust-owned planner/reflow lane. |

### Local-First Calendar Input

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Existing `.ics` calendar adapter in `veld` | repo-local | Canonical local calendar ingest path | Prefer extending the existing `.ics` path instead of inventing a new planner-side calendar stack. |
| Apple-local calendar export via `VelLocalSourceExporter` extension | new repo addition, no new third-party dependency required | Make Apple-local calendars usable by the Rust core | The practical MVP move is to add calendar export beside the existing reminders/health/messages snapshots. Reuse the local-source bridge model already documented for Apple/macOS. |
| Google Calendar connector | existing shipped path | Remote calendar source when local Apple export is unavailable | Keep as-is for users who already rely on Google-backed sync. Do not widen provider scope in this milestone. |

### Narrow Additions Recommended

| Addition | Scope | Why |
|---------|-------|-----|
| Apple calendar export/snapshot path | Apple + `veld` local-source ingest | Needed to make “reflow calendar, ideally locally” feasible without moving planner logic into Swift. |
| Rust-owned MVP read model for daily overview / commitment / nudge / review | `vel-core` + `veld` services + DTOs | The milestone goals are mostly product-core closure, not framework churn. Add stronger Rust-owned read models instead of more UI libraries. |
| Optional iPhone-only embedded helper use of the existing bridge seam | Only for cache hydration / queue shaping helpers if a concrete latency issue remains | Allowed by current architecture, but keep it narrow and additive. Do not broaden into full embedded runtime or general FFI migration. |

## Calendar Reflow Feasibility

### Recommended MVP Approach

1. Compute reflow in Rust from canonical commitments, current context, planning profile, and calendar events.
2. Accept calendar events from whichever source is already available:
   - Google Calendar sync
   - local `.ics`
   - new Apple-local EventKit export feeding the same local-source ingest lane
3. Render proposal/review/apply state in web and Apple as thin shells over the same typed backend result.

### Feasibility Notes

| Surface | Feasibility | Recommendation |
|---------|-------------|----------------|
| Apple local calendar read | **High** | Feasible with EventKit permission. Use it only to supply event data into the Rust-owned reflow substrate. |
| Apple local calendar write/re-schedule | **Medium/Low for MVP** | Do **not** add this in v0.2. It widens into provider-specific mutation, conflict handling, and review semantics. |
| Web direct local calendar access | **Low** | Browser shells do not have a practical ambient local-calendar API. Treat web as daemon-backed; at most support explicit file/path selection handled through the daemon host. |
| Cross-platform local-first reflow | **Medium** | Feasible if “local-first” means local data on the daemon host or Apple-exported local snapshots feeding Rust. Not feasible if it means every shell mutates native calendars independently. |

### Strong Recommendation

Use **Apple-local calendar export into the existing Rust ingest/reflow path**, not Apple-local planner logic and not browser-local calendar handling.

If the Apple-local export shape must be added, prefer:

- exporting into the existing `.ics` ingest lane first, or
- adding one small typed calendar snapshot format only if `.ics` proves too lossy

Do not create a separate Apple-only planner contract.

## UI Shell Implications

### Web

| Decision | Recommendation | Why |
|----------|----------------|-----|
| State management | Keep the current typed fetch/decoder layer | MVP needs less shell complexity, not React-side authority. |
| Design system | Do not add one | A fresh minimal UI can be built with the current stack. A design-system rollout would slow the milestone and harden the wrong abstractions. |
| Real-time/sync | Reuse current websocket/query seams where already present | Enough for continuity; no new frontend state platform is needed. |
| Local calendar | Do not pursue browser-native calendar access | Keep that concern on the daemon host or Apple exporter path. |

### Apple

| Decision | Recommendation | Why |
|----------|----------------|-----|
| Primary transport | Keep `VelAPI` HTTP-first | Still the default and most portable cross-surface model. |
| Embedded Rust | Keep additive and narrow | Only use the existing embedded seam for explicit helper flows if required. |
| Native framework use | Add EventKit events read/export only | This is the one Apple-native addition that materially helps MVP local-first reflow. |
| Product logic ownership | Keep in Rust | SwiftUI should render, queue, capture permissions, and export local data; it should not own planning/review semantics. |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Apple portability | HTTP-first Apple shell plus narrow optional embedded helper | Full UniFFI / broad FFI migration | Current repo authority explicitly says no immediate UniFFI migration; too much boundary churn for MVP. |
| Calendar locality | EventKit read/export into Rust ingest | Apple-local planner/reflow logic | Forks semantics and violates Rust-core-first goal. |
| Web shell data | Existing typed fetch/decoder layer | React Query / Redux / Zustand rollout | Adds client authority and migration work without solving the milestone problem. |
| Calendar mutation | Review/apply stays backend-owned and bounded | Full local calendar write-back across Apple/web | Too much provider-specific complexity for this milestone. |
| Desktop packaging | Existing daemon + shells | Tauri/Electron/Desktop re-platform | Not needed for true-MVP closure. |

## What Should NOT Be Added In This Milestone

- No new database, sync engine, or hosted backend.
- No broad frontend state library migration.
- No cross-platform design system rollout.
- No broad FFI migration, UniFFI adoption, or “Apple becomes the runtime” rewrite.
- No full local calendar write-back engine.
- No CalDAV stack, Exchange stack, or new calendar providers beyond the existing Google/local-source paths.
- No shell-owned planner, suggestion, thread, or review semantics.
- No Tauri/Electron packaging work.
- No Graph-RAG, wider agent platform work, or other Phase-8-style expansion unless directly required for the MVP operator flows.

## Installation

```bash
# Rust core: keep existing workspace stack
cargo build

# Web shell: keep existing shell stack
cd clients/web && npm install

# Apple shell: keep existing Swift packages/Xcode path
# Add EventKit calendar permission/export work in the existing Apple targets only if local calendar MVP is selected
```

## Sources

- Internal authority: `docs/MASTER_PLAN.md`
- Internal authority: `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md`
- Internal authority: `docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md`
- Internal authority: `docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md`
- Internal authority: `clients/apple/README.md`
- Internal authority: `docs/user/integrations/local-sources.md`
- Internal authority: `docs/user/integrations/apple-macos.md`
- Internal implementation: `clients/web/package.json`
- Internal implementation: `Cargo.toml`
- Internal implementation: `clients/apple/VelAPI/Sources/VelAPI/VelLocalSourceExporter.swift`
- React docs: https://react.dev/
- Vite docs: https://vite.dev/guide/
- SwiftUI docs: https://developer.apple.com/documentation/swiftui
- EventKit docs: https://developer.apple.com/documentation/eventkit
- MDN File System Access API: https://developer.mozilla.org/docs/Web/API/File_System_API

# Feature Landscape

**Domain:** local-first operator cognition product, true MVP closure for milestone v0.2
**Researched:** 2026-03-20

## Table Stakes

Features the MVP should treat as expected operator behavior. Missing these makes the product feel incomplete even if the substrate already exists.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Daily overview with one dominant current action | Serious planning/operator tools are expected to open on “what matters now,” not on a generic dashboard or chat feed. | Medium | `Now` should answer current commitment, next useful action, next calendar anchor, and whether the day is on track. This is the first-glance surface, not a backlog browser. |
| Commitment selection and lightweight editing | An MVP daily loop needs explicit commitment, not just passive lists. Users expect to pick 1-3 things, adjust them, and see what is active now. | Medium | Should allow continue, break down, defer, and promote to thread without leaving the main flow. Avoid a separate planner subsystem. |
| Unified today lane across calendar + commitments + priority work | Current operator products commonly win by putting tasks and time on one surface. A split task view and calendar view feels unfinished. | High | Use existing Vel commitments/calendar substrate, but present one coherent “today” lane. Meetings/events should affect actionability, not sit as unrelated summaries. |
| Same-day reflow after drift | Once the day changes, operators expect a repair path. Without reflow, the morning plan becomes stale and trust collapses by midday. | High | Keep it bounded and explainable: same-day only, clear moved/unscheduled/needs-judgment outcomes, operator-supervised application. |
| Actionable nudges and check-ins | A daily operator product is expected to notice drift, blocked work, missed anchors, and stale commitments. | Medium | Nudges must be small, specific, and resolvable: “do this now,” “defer,” “open thread,” or “reflow.” Do not show advisory cards with no clear action. |
| Thread escalation for non-trivial loops | When a decision cannot be closed inline, users expect to continue it somewhere durable. | Medium | Threads should open from `Now`/Inbox items as “deeper thought” or continuity lanes, not as generic chat rooms. |
| Review / closeout loop | Good planning tools are expected to help close the day by reviewing what moved, slipped, or needs carry-forward. | Medium | Keep review lightweight: completed, deferred, unscheduled, unresolved thread follow-up. The value is closure and feed-forward into tomorrow, not analytics theater. |
| Cross-surface consistency | For this milestone, web and Apple need the same product behavior or the MVP will feel fragmented. | High | Rust-owned behavior should be canonical. Shells may differ in presentation, but not in planning semantics or thread/reflow meaning. |

## Differentiators

Features worth keeping because they strengthen Vel’s MVP instead of widening it.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Rust-owned canonical daily loop | Makes the MVP portable across web and Apple without re-implementing planner logic in each shell. | High | This is a milestone goal, not optional polish. It is the backbone that keeps the MVP honest. |
| Explainable local reflow | Most tools offer planning; fewer offer a local-first, explainable same-day repair lane with visible rationale. | High | Keep rule-facet-backed explanations and supervised application. This is more valuable than “AI magic” auto-rescheduling. |
| Suggestions/nudges grounded in persisted context | Nudges are useful when they come from commitments, time state, calendar pressure, and thread status instead of generic assistant prose. | Medium | Preserve provenance and explicit routing to action, reflow, or thread. |
| Threads as continuity substrate, not chat | This gives Vel a cleaner operator mental model than “assistant conversation everywhere.” | Medium | A thread should show timeline, decisions made, open questions, and next step. Promote from action into thread and resolve back into action. |
| Review as operational closure | A short closeout that prepares tomorrow is more valuable in MVP than dashboards, scoring, or long retrospectives. | Low | Keep it focused on carry-forward and trust, not quantified-self reporting. |

## Anti-Features

Features to explicitly NOT build in this milestone.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Multi-day or autonomous planner | This milestone is about a trustworthy same-day MVP. Multi-day optimization will dominate scope and weaken explainability. | Keep planning and reflow bounded to the current day. |
| Generic chat-first assistant shell | Threads are for open loops, not for making the whole product feel like a chatbot wrapper. | Keep `Now` decision-first and use threads only when inline resolution is insufficient. |
| Full project/workspace management expansion | Projects already exist as substrate. Rebuilding project dashboards will distract from the daily loop. | Use project context only when it sharpens action, nudges, or thread routing. |
| Opaque AI automation or silent calendar edits | Silent mutation breaks trust fast, especially in calendar/task systems. | Stage proposals, explain outcomes, and require explicit supervised apply for meaningful changes. |
| Heavy review analytics, scores, or productivity reporting | These add UI mass without improving daily operation for MVP. | Keep review to closure, carry-forward, and what needs attention tomorrow. |
| Routine builder redesign or rich drag-and-drop planner studio | This is a classic scope trap. It turns the milestone into a scheduling UI rewrite. | Reuse existing planning-profile/routine substrate with thin inline edits and readable time-block cards. |
| Shell-specific planning logic | Re-implementing behavior in web and Apple will break parity and undercut the milestone’s Rust-owned goal. | Keep shells thin and route all planning semantics through the canonical backend seams. |
| Debug/runtime state in default surfaces | Internal model fields and confidence readouts add noise and make the product feel unfinished. | Move internals behind explicit `State` / `Why` / `Debug` disclosure. |

## Expected Operator Behaviors

For this milestone, the MVP should feel like this:

1. Open `Now` and immediately see one dominant current action, next relevant time context, and at most a few next actions.
2. Commit to a small set of work for the day and edit it inline without entering a separate planning tool.
3. Receive small, actionable nudges when drift appears, with clear next moves: continue, defer, thread, or reflow.
4. Repair the day when reality changes through a bounded reflow proposal that stays local-first and operator-supervised.
5. Escalate messy or ambiguous work into a thread that preserves context, decisions, and next step.
6. End with a brief review that closes loops and cleanly carries unresolved work forward.

## Feature Dependencies

```text
Unified today lane -> Daily overview -> Commitment flow
Commitment flow -> Nudges/check-ins
Unified today lane + commitments + routine constraints -> Same-day reflow
Daily overview + nudges/check-ins -> Thread escalation
Commitment flow + reflow + thread outcomes -> Review / closeout
Rust-owned canonical daily loop -> Web and Apple parity for all MVP flows
```

## MVP Recommendation

Prioritize:

1. Daily overview plus one dominant current action on `Now`
2. Commitment flow with inline continue / break down / defer / thread escalation
3. Same-day reflow with explicit, explainable outcomes
4. Lightweight review / closeout that feeds the next day

Keep as selective differentiators:

1. Rust-owned canonical flow across shells
2. Explainable local reflow
3. Threads as structured continuity, not generic chat

Defer:

- Rich routine editing UI: reuse existing substrate and keep edits thin
- Broad analytics/reporting: no MVP value compared with review closure
- Multi-day planning: too much surface and policy for this milestone
- New project/dashboard concepts: substrate already exists; do not recentralize the product around it

## Sources

- Repo authority: [PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md)
- Repo authority: [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md)
- Repo authority: [STATE.md](/home/jove/code/vel/.planning/STATE.md)
- Repo authority: [MASTER_PLAN.md](/home/jove/code/vel/docs/MASTER_PLAN.md)
- Product boundary: [now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md)
- Architecture contract: [day-plan-reflow-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md)
- Phase input: [40-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.2-phases/40-decision-first-ui-ux-rework-across-now-settings-threads-and-context-surfaces/40-CONTEXT.md)
- Phase input: [40-RESEARCH.md](/home/jove/code/vel/.planning/milestones/v0.2-phases/40-decision-first-ui-ux-rework-across-now-settings-threads-and-context-surfaces/40-RESEARCH.md)
- External pattern check: https://www.sunsama.com/daily-planning
- External pattern check: https://www.sunsama.com/features/guided-planning-and-reviews
- External pattern check: https://help.sunsama.com/docs/importing-meetings
- External pattern check: https://routine.co/features/planner
- External pattern check: https://product.akiflow.com/help/articles/5397877-how-to-plan-a-perfect-week

# Domain Pitfalls

**Domain:** true MVP closure for Vel v0.2 (Rust-core-first with fresh thin clients)
**Researched:** 2026-03-20

## Critical Pitfalls

### Pitfall 1: Calling the existing surface area the MVP
**What goes wrong:** The milestone inherits too much of the already-built product and treats "make current features cleaner" as MVP closure.
**Why it happens:** Brownfield teams anchor on shipped capability instead of the smallest loop that creates repeated daily value.
**Consequences:** The milestone turns into cleanup plus parity work, Phase 40 gets absorbed without narrowing the product, and the team ships another broad beta instead of a crisp daily-use MVP.
**Prevention:** Freeze the v0.2 MVP around one daily loop only: morning overview, commitment choice, suggestion/nudge handling, same-day reflow, thread-backed follow-through, and review. Everything else must justify itself as necessary to that loop. Write an explicit "not in MVP" list before implementation starts.
**Detection:** Requirements keep mentioning general assistant, broader project management, richer memory, multi-day planning, or automation breadth without tying them to the five operator flows above.

### Pitfall 2: Re-platforming everything instead of extracting only canonical product logic
**What goes wrong:** "Rust-owned" becomes a full migration mandate, so teams try to move every legacy path, UI behavior, and integration abstraction before shipping the MVP loop.
**Why it happens:** Architecture cleanup feels safer than choosing which behaviors are essential.
**Consequences:** The core migration stalls, clients remain half-old/half-new, and the MVP misses the chance to prove a stable product seam.
**Prevention:** Extract only the canonical commands, queries, and typed read models needed for the MVP flows. Leave non-MVP surfaces behind feature gates or legacy boundaries. Define migration completion per flow, not per module.
**Detection:** Plans are organized by subsystem rewrites instead of operator outcomes, or success criteria talk about "all core logic moved" without naming the user-visible loop it unblocks.

### Pitfall 3: Thin clients in name only
**What goes wrong:** Web and Apple get fresh UI shells, but decision logic, fallback rules, local caching policy, or derived state still lives in TypeScript/Swift.
**Why it happens:** New UI work often reintroduces convenience logic to make screens feel responsive before backend seams are fully shaped.
**Consequences:** Cross-surface drift returns, Apple and web disagree on suggestions/reflow/review state, and the Rust core stops being the product truth.
**Prevention:** Treat clients as rendering + input + optimistic UX only. All suggestion generation, reflow decisions, thread/tool policy, review state, and current-day truth must come from typed Rust APIs. Require shared fixtures for web and Apple against the same backend payloads.
**Detection:** Client PRs introduce business-rule conditionals, status synthesis, or duplicate filtering/sorting beyond presentation concerns.

### Pitfall 4: Shipping a fresh UI before locking the canonical decision model
**What goes wrong:** The new shells are designed around screens and cards before the MVP's operator decisions are formalized.
**Why it happens:** Phase 40 creates pressure to redesign navigation and presentation first.
**Consequences:** The UI looks cleaner but still forces the operator to interpret too many states, and backend contracts get bent to fit visual layouts instead of decisions.
**Prevention:** Lock the decision model first: "what needs commitment," "what needs intervention," "what can be deferred," "what can be reflowed," and "what must be reviewed." Build the UI hierarchy from those decisions, not from current information architecture.
**Detection:** Mockups or plans lead with page taxonomy, component trees, or dashboard sections rather than decision verbs and dominant actions.

### Pitfall 5: Treating local-first reflow as a planner rewrite
**What goes wrong:** Calendar reflow expands from same-day repair into a broad local scheduling engine.
**Why it happens:** Reflow touches commitments, routines, calendars, nudges, and suggestions, so it is easy to widen from "repair today" into "solve planning."
**Consequences:** The team reopens already-closed planner complexity, loses explainability, and delays the one thing the MVP actually needs: reliable same-day recovery.
**Prevention:** Bound reflow to same-day repair over persisted commitments, explicit constraints, and local calendar state. No speculative multi-day optimization, no hidden automatic rescheduling, no new planning ontology unless required by same-day reflow.
**Detection:** Proposed rules mention week planning, goal balancing, automatic routine generation, or opaque score-based schedule optimization.

### Pitfall 6: Underestimating local-first conflict and provenance requirements
**What goes wrong:** Reflow, suggestions, and tool actions mutate local state or upstream-linked state without a clear provenance trail and reversible review path.
**Why it happens:** MVP pressure encourages "just apply the result" because the app is local-first and single-user.
**Consequences:** The operator loses trust, review becomes post-hoc guesswork, and any sync or upstream reconciliation later becomes harder.
**Prevention:** Every reflow, nudge, tool proposal, and applied action needs persisted inputs, rule basis, run/thread IDs, and terminal state. Keep review artifacts first-class even in MVP scope.
**Detection:** You can see the current plan or action outcome, but not which inputs, rules, or thread/tool invocation produced it.

## Moderate Pitfalls

### Pitfall 1: Making threads a generic chat surface instead of a supervised work context
**What goes wrong:** Threads become a broad conversational bucket for anything the system cannot place cleanly.
**Prevention:** Limit MVP threads to context-bearing follow-through: explain a suggestion, gather tool/context inputs, stage a reflow or action, and support review. Keep generic assistant breadth out of scope.

### Pitfall 2: Overloading the review flow with analytics, journaling, and retrospective ambition
**What goes wrong:** Review tries to become a rich life-logging or performance-analysis surface.
**Prevention:** Keep review narrow: what was committed, what changed, what slipped, what was applied, and what needs carrying forward. Anything beyond that belongs after MVP.

### Pitfall 3: Assuming "local if possible" means provider-free
**What goes wrong:** Calendar reflow or tooling plans assume local-only access even where current integrations still require provider APIs, permissions, or inconsistent device stores.
**Prevention:** Separate "local computation" from "local authority." Compute reflow locally where possible, but model source authority, permissions, freshness, and fallback explicitly per provider or device boundary.

### Pitfall 4: Keeping JSON blobs as the escape hatch for new MVP concepts
**What goes wrong:** New MVP entities for overview state, reflow proposals, or review packets are shoved back into untyped JSON to move quickly.
**Prevention:** Use typed versioned Rust structs at the domain edge and serialize only at storage/API boundaries. Brownfield speed is not a reason to regress the architecture.

### Pitfall 5: Letting broad tool access sneak back in through the assistant path
**What goes wrong:** Because MVP needs threads plus tools/context/data, subordinate tooling expands beyond the exact flows under supervision.
**Prevention:** Keep explicit allowlists: calendar read/reflow inputs, thread context, bounded review artifacts, and approved action lanes only. Unknown tool requests should fail closed.

## Minor Pitfalls

### Pitfall 1: Measuring progress by migrated screens rather than completed operator loops
**What goes wrong:** The milestone looks healthy because web and Apple have new surfaces, but the end-to-end daily loop is still broken.
**Prevention:** Track completion by flow rehearsal: overview -> commitment -> nudge/reflow -> thread/tool follow-through -> review.

### Pitfall 2: Fresh-shell polish masking broken degraded states
**What goes wrong:** Happy-path screenshots improve while stale sync, missing permissions, failed tool calls, or offline edges remain confusing.
**Prevention:** Make degraded states part of the MVP acceptance criteria for both web and Apple.

### Pitfall 3: Reusing old labels that no longer match MVP semantics
**What goes wrong:** Terms like `Now`, `Inbox`, `thread`, `suggestion`, or `review` carry old meaning into the new milestone and confuse planning.
**Prevention:** Rewrite the vocabulary contract for v0.2 up front and use it consistently in APIs, UI copy, and roadmap plans.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| MVP definition / milestone framing | Scope collapse into "improve everything already shipped" | Start with a strict operator-loop contract and an explicit out-of-scope list; make every plan map to one of the five MVP flows |
| Decision model / contracts | UI-first redesign before product decisions are typed | Publish canonical decision/read-model contracts before web or Apple redesign starts |
| Rust-core extraction | Full-system migration instead of flow-scoped migration | Extract only commands/queries needed for overview, commitments, nudges, reflow, threads, and review |
| Web shell refresh | Client-side policy sneaks back in via convenience logic | Ban product-rule synthesis in TypeScript; require backend-owned derived state and contract fixtures |
| Apple thin client refresh | Swift reimplements backend decisions for offline UX | Keep Apple to rendering, capture, caching, and transport; move policy and explanation generation to Rust |
| Daily overview / commitment flow | Overview becomes dashboard-heavy instead of decision-first | One dominant next action, bounded supporting context, explicit deferral path, and visible rationale |
| Suggestions / nudges | Suggestion system widens into general assistant behavior | Restrict MVP nudges to current-day intervention and commitment follow-through with provenance |
| Calendar reflow | Same-day repair expands into speculative planner rewrite | Limit to explainable same-day reconciliation over explicit constraints and current calendar truth |
| Threads + tools/context/data | Generic chat and broad tools widen scope/security surface | Keep threads task-bounded and tools allowlisted; no ambient access expansion |
| Review | Review grows into analytics/journaling product | Limit output to commitments, changes, slips, applied actions, and carry-forward items |
| Verification | Phase 40 visual work closes without execution-backed loop checks | Require cross-surface end-to-end rehearsals plus degraded-state checks for each MVP flow |

## Sources

- Internal authority: [.planning/PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md), [.planning/ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md), [.planning/STATE.md](/home/jove/code/vel/.planning/STATE.md), [docs/MASTER_PLAN.md](/home/jove/code/vel/docs/MASTER_PLAN.md), [.planning/codebase/CONCERNS.md](/home/jove/code/vel/.planning/codebase/CONCERNS.md)
- Ink & Switch, "Local-first software" (used for local-first trust/conflict/provenance pressure; HIGH confidence): https://www.inkandswitch.com/local-first/
- SQLite documentation on WAL and concurrency limits (used for local-first single-writer/reconciliation caution; HIGH confidence): https://sqlite.org/wal.html
- Silicon Valley Product Group, product outcome vs feature-delivery framing (used for MVP scope-discipline caution; MEDIUM confidence): https://www.svpg.com/the-product-model-in-traditional-it/