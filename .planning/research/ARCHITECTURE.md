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
