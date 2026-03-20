# Phase 10: Daily-loop morning overview and standup commitment engine - Research

**Researched:** 2026-03-19
**Domain:** Backend-owned daily session orchestration over `Now`, commitments, calendar/Todoist inputs, and Apple voice
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Product contract from the user spec
- [locked] Total interaction time for the full loop must be `<= 3 minutes`.
- [locked] The full loop is Morning Overview -> Standup -> Execution -> Reflection, but this phase covers only Morning Overview and Standup.
- [locked] Morning Overview is passive and does not create commitments.
- [locked] Standup is the commitment engine and must end with explicit bounded daily commitments.
- [locked] Daily commitments are capped at `1-3`.
- [locked] Each sub-flow may ask at most `3` questions.
- [locked] All flows must be interruptible, skippable, and resumable.
- [locked] Voice is preferred, but text fallback is acceptable.
- [locked] Simple heuristics and low latency are preferred over complex intelligence.

### Morning Overview
- [locked] Trigger should support manual start now (`Start morning`) and be compatible with future automatic start.
- [locked] Required inputs are calendar for the next 12 hours and Todoist for today + overdue.
- [locked] Output structure is:
  - short snapshot (`<= 4 sentences`, `<= ~20 seconds`)
  - at most `2` friction callouts
  - `1-3` intent-gathering questions
- [locked] Morning Overview can capture intent signals such as must-do hints, focus intent, or meeting doubt, but must not create commitments yet.

### Standup
- [locked] Standup may follow Morning Overview immediately or start manually.
- [locked] Standup steps are:
  - calendar reconciliation
  - task compression into must/should/stretch
  - commitment extraction
  - constraint shaping
  - opportunistic metadata enrichment
- [locked] Standup must force reduction if the user proposes more than `3` defining commitments.
- [locked] Final session output must include commitments, deferred tasks, confirmed calendar state, and proposed focus blocks.

### UX and interaction
- [locked] Vel tone should be calm, slightly firm, and brief.
- [locked] Avoid coaching, therapy, or excessive encouragement.
- [locked] Silence or `skip` should advance safely.
- [locked] If the user disengages, partial state must persist and resume later.
- [locked] If no commitments are defined, prompt once more and then exit.

### Architecture constraints
- [auto] Reuse existing `Now`, commitments, and Apple/backend voice surfaces rather than introducing a second planning subsystem.
- [auto] Keep policy and durable state in Rust backend layers. Web, CLI, and Apple remain thin shells.
- [auto] Preserve explainability: friction callouts, focus suggestions, and deferrals must be derivable from persisted inputs and explicit rules.
- [auto] Add typed session state rather than deepening untyped blobs.

### Claude's Discretion
- Exact contract/type names for morning signals, task buckets, daily sessions, resume state, and compression heuristics
- Whether the first shipped surface is CLI-first, web-first, or dual-surface, so long as voice/text fallback and resumability are preserved
- Exact scoring/ranking heuristics for friction callouts, deferrals, and focus-block suggestions

### Deferred Ideas (OUT OF SCOPE)
- Reflection, deeper end-of-day loops, or analytics dashboards
- Broad shell/navigation polish and onboarding work — Phase 12
- Agent grounding over real Vel state — Phase 11
- New provider families, reading/media tracking, or broader external-product expansion
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| MORNING-01 | Manual morning start now, future auto-start compatible, using next-12h calendar plus Todoist today/overdue inputs | New backend `daily_loop` input snapshot builder should reuse calendar selection and commitment ranking seams but add contract-specific filtering not present in current `Now` output |
| MORNING-02 | Morning Overview returns a short passive snapshot, up to 2 friction callouts, and 1-3 questions | Typed morning-overview state/output contract plus deterministic ranking/dedupe rules are required |
| MORNING-03 | Morning captures intent only and must not create commitments | Session state must persist intent signals separately from commitment writes until Standup |
| STANDUP-01 | Standup can follow Morning or start directly and must reconcile calendar plus compress tasks into must/should/stretch | Use one backend session engine with standup phase transitions and a contract-specific input snapshot |
| STANDUP-02 | Standup must end with 1-3 commitments, deferred tasks, confirmed calendar state, and proposed focus blocks | Reuse existing commitment storage for commitments, but persist deferrals/focus blocks in session outcome types because those are not first-class entities today |
| STANDUP-03 | Standup must force reduction above 3 commitments and reprompt once if none are defined | Session policy needs explicit question budget, reduction rule, and terminal reprompt logic |
| SESSION-01 | Flows must be interruptible, skippable, resumable, and durable across disengagement | New typed `daily_sessions` persistence seam should own phase/status/resume cursor instead of storing this in `current_context` |
| VOICE-01 | Voice-first behavior must reuse backend-owned Apple voice seams without becoming Apple-only | Apple voice should become a thin entry/resume shell over the same backend session-turn API used by CLI/text |
</phase_requirements>

## Summary

Vel already has most of the raw ingredients for Phase 10: `Now` is the current orientation authority, `operator_queue` already ranks review/action pressure, commitments are durable and mutable, Apple voice already persists transcript provenance before answering, and run-backed context generation already establishes a pattern for explainable generated outputs. What is missing is the phase-specific orchestration seam: a typed, resumable daily session that turns those inputs into one bounded Morning Overview and one bounded Standup.

The main planning trap is assuming existing shipped surfaces already satisfy the new contract. They do not. The current `/v1/context/morning` flow is an older heuristic context artifact, not a resumable session. `AppleVoiceIntent::MorningBriefing` currently routes to the generic explain path, not to a dedicated morning loop. `Now.tasks.todoist` is a ranked open-commitment slice, not explicitly "today + overdue", and `Now.schedule.upcoming_events` is not explicitly bounded to the next 12 hours. The commitment model also has no first-class deferred-task or focus-block concept yet.

**Primary recommendation:** add one backend-owned `daily_loop` session engine with typed persistence, contract-specific input filtering, and a single session-turn protocol; keep CLI, web, and Apple as thin shells over that engine, and reuse existing `Now`, commitment, suggestion-dedupe, and run/event patterns instead of inventing a second planner.

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-core` | `0.1.0` workspace-pinned | typed daily-session domain structs, phase/status enums, outcome types | domain semantics belong in core, not routes or storage |
| `vel-storage` | `0.1.0` workspace-pinned | SQLite migrations and repositories for new daily-session state | existing typed persistence edge for runtime-owned records |
| `vel-api-types` | `0.1.0` workspace-pinned | transport DTOs for session start/turn/resume/result | preserves route/service boundary discipline |
| `veld` | `0.1.0` workspace-pinned, `axum 0.7` | backend authority: session orchestration, routes, explainability, commitment writes | policy and durable state already live here |
| `vel-cli` | `0.1.0` workspace-pinned, `clap 4.5` | text fallback and strongest current operator shell | daily-use docs already position CLI as the direct operator surface |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `clients/web` | `react 19.2.4`, `vite 8.0.0`, `vitest 2.1.8` | thin session/resume/outcome shell inside `Now` | show state, resume, and final output without client-owned policy |
| `clients/apple` | repo-local app/client code | voice-first entry and resume path | when transcript/TTS/quick-loop UX matters |
| `suggestions` persistence pattern | existing repo seam | dedupe/noise suppression for repeated friction/action suggestions | when the same evidence would otherwise repeat across sessions |
| run/event infrastructure | existing repo seam | lifecycle observability for session generation/mutation boundaries | when session creation/completion/cancellation needs inspection |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| new typed `daily_sessions` persistence | `CurrentContextV1.extra` or client-local state | cheaper short-term, but violates typed-state rule and breaks resumability/inspection |
| dedicated daily-loop routes | overload `/v1/context/morning` | mixes old heuristic artifact generation with interactive session state |
| backend-owned turn engine | client-owned web/Apple logic | faster UI work, but breaks cross-surface consistency and explainability |

**Installation:**
```bash
# No new third-party packages are required for the contract slice.
# Stay on the repo-pinned workspace stack.
```

**Version verification:** repo-pinned versions were verified from [Cargo.toml](/home/jove/code/vel/Cargo.toml) and [package.json](/home/jove/code/vel/clients/web/package.json). This phase does not need new ecosystem dependencies to plan well.

## Architecture Patterns

### Recommended Project Structure

```text
crates/
├── vel-core/src/daily_loop.rs              # typed session/turn/outcome contracts
├── vel-storage/src/repositories/
│   └── daily_sessions_repo.rs              # session persistence and resume queries
├── veld/src/services/
│   ├── daily_loop.rs                       # orchestration over Now/operator queue/commitments
│   └── daily_loop_inputs.rs                # contract-specific calendar/Todoist filtering
├── veld/src/routes/daily_loop.rs           # thin HTTP boundary
└── vel-cli/src/commands/daily_loop.rs      # text fallback shell
clients/
├── web/src/components/NowView.tsx          # resume/status/result shell only
└── apple/...                               # voice entry/resume routed to backend
migrations/
└── 0044_phase10_daily_sessions.sql         # typed daily-session storage
```

### Recommended Slice Order

1. Contract slice: core types, storage migration/repo, DTOs, and failing tests.
2. Backend slice: daily-loop input snapshot builder plus session state machine for Morning Overview and Standup.
3. Mutation slice: commitment extraction, deferral/focus-block persistence in session outcome, CLI text flow.
4. Surface slice: Apple voice entry/resume wiring and minimal web `Now` session shell.

### Pattern 1: Typed daily session row plus turn protocol

**What:** persist a first-class daily session record with phase, status, question budget, intent signals, outcome, and resume cursor. Use one backend turn API for start/respond/skip/resume.

**When to use:** every Morning Overview and Standup interaction, regardless of shell.

**Example:**
```rust
// Pattern source: crates/veld/src/services/context_runs.rs,
// migrations/0043_phase8_execution_handoffs.sql,
// crates/vel-core/src/context.rs
pub enum DailySessionPhase {
    MorningOverview,
    Standup,
}

pub enum DailySessionStatus {
    Active,
    WaitingForInput,
    Completed,
    Cancelled,
}

pub struct DailySessionRecord {
    pub id: DailySessionId,
    pub session_date: String,
    pub phase: DailySessionPhase,
    pub status: DailySessionStatus,
    pub question_count: u8,
    pub input_snapshot: DailyLoopInputSnapshot,
    pub morning_state: Option<MorningOverviewState>,
    pub standup_state: Option<StandupState>,
    pub outcome: Option<DailySessionOutcome>,
}
```

### Pattern 2: Build daily-loop inputs below the route boundary, not from HTTP DTOs

**What:** create a service-level `DailyLoopInputSnapshot` builder that reuses backend ranking/filtering helpers from `now` and `operator_queue`, but does not depend on `NowData` or web DTOs.

**When to use:** whenever Morning Overview or Standup needs calendar/tasks/friction inputs.

**Example:**
```rust
// Pattern source: crates/veld/src/services/now.rs +
// crates/veld/src/services/operator_queue.rs
pub async fn load_daily_loop_inputs(
    storage: &Storage,
    config: &AppConfig,
) -> Result<DailyLoopInputSnapshot, AppError> {
    let now_snapshot = now::get_now(storage, config).await?;
    let action_queue = operator_queue::build_action_items(storage, config).await?;

    Ok(DailyLoopInputSnapshot {
        calendar_next_12h: filter_next_12h(&now_snapshot.schedule),
        todoist_today_or_overdue: filter_today_or_overdue(&now_snapshot.tasks.todoist),
        friction_pool: dedupe_friction_candidates(
            &action_queue.action_items,
            &now_snapshot.pending_writebacks,
            &now_snapshot.conflicts,
        ),
    })
}
```

### Pattern 3: Morning Overview uses `Now` and action queue for friction, but not as the full task source

**What:** morning snapshot should be derived from the contract-specific input snapshot plus deduped `Now`/action evidence. Do not simply read the top `Now` cards back to the user.

**When to use:** building the `<= 4 sentence` summary, `<= 2` friction callouts, and `1-3` questions.

**Example:**
```rust
// Pattern source: crates/veld/src/services/operator_queue.rs +
// crates/veld/src/services/suggestions.rs
let friction_callouts = rank_and_dedupe(
    inputs.friction_pool,
    2,
    |item| item.dedupe_key(),
);
let questions = build_intent_questions(&inputs, &friction_callouts).take(3).collect();
```

### Pattern 4: Standup writes commitments through existing commitment seams and stores non-commitment artifacts separately

**What:** create/update final commitments through the existing commitment storage/API path, but keep deferred tasks, confirmed calendar notes, and focus-block proposals as typed session outcome fields until a later phase gives them first-class entities.

**When to use:** finalizing Standup.

**Example:**
```rust
// Pattern source: crates/veld/src/routes/commitments.rs +
// crates/veld/src/services/client_sync.rs
for candidate in selected_commitments.iter().take(3) {
    storage.insert_commitment(CommitmentInsert {
        text: candidate.text.clone(),
        source_type: "daily_loop".to_string(),
        source_id: candidate.source_id.clone().unwrap_or_default(),
        status: CommitmentStatus::Open,
        due_at: candidate.due_at,
        project: candidate.project.clone(),
        commitment_kind: Some("daily_commitment".to_string()),
        metadata_json: Some(candidate.metadata_json()),
    }).await?;
}
```

### Anti-Patterns to Avoid

- **Overloading `/v1/context/morning`:** it is a run-backed heuristic context artifact today, not a resumable session contract.
- **Storing session state in `CurrentContextV1.extra`:** this deepens the singleton context blob and makes resume semantics brittle.
- **Treating deferrals as cancelled commitments:** deferred work is still relevant to the day outcome and needs explainable session output, not silent semantic loss.
- **Letting Apple or web own compression heuristics:** cross-surface divergence would be immediate and hard to inspect.
- **Building a second planner store:** the phase should orchestrate existing `Now`, commitments, Todoist, and voice seams, not compete with them.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Resumable daily-loop state | ad hoc JSON in `current_context` or local client cache | typed `daily_sessions` table + repo | partial progress and resume need first-class durable state |
| Morning friction ranking | new client-only scoring engine | reuse `operator_queue` evidence and `Now` freshness/conflict/writeback inputs | ranked evidence already exists and is explainable |
| Suggestion/action dedupe | one-off string matching in each surface | `dedupe_key`-style suppression pattern from suggestions | repeated suggestions were explicitly called out as a product trust risk |
| Voice-only planning logic | Apple-specific intent handler tree | same backend session-turn engine used by CLI/text | voice is preferred, not exclusive |
| Deferred-task semantics | new commitment status or silent cancellation | typed session outcome `DeferredTask` records referencing source items | avoids widening commitment semantics prematurely |
| Focus protection | auto-writing calendar blocks | typed proposed focus blocks in outcome, optionally surfaced later as suggestions | Phase 10 requires proposal, not autonomous scheduling |

**Key insight:** the hard part of this phase is not summarization. It is durable, cross-surface, explainable session state over partially overlapping existing seams.

## Common Pitfalls

### Pitfall 1: Mistaking the old morning route for the new Morning Overview
**What goes wrong:** planning extends `/v1/context/morning` instead of creating a session engine.
**Why it happens:** the CLI already has `vel morning`, and the runtime already exposes `GET /v1/context/morning`.
**How to avoid:** keep the old context artifact intact; add dedicated daily-loop routes/services and, if needed, make `vel morning` a compatibility alias to the new backend flow.
**Warning signs:** no persisted session record, no resume cursor, no question budget.

### Pitfall 2: Reusing `Now` outputs without contract-specific filtering
**What goes wrong:** Morning Overview speaks from `Now.tasks.todoist` and `Now.schedule.upcoming_events` as-is.
**Why it happens:** `Now` already looks close to the desired surface.
**How to avoid:** build a separate input snapshot with exact next-12h calendar and Todoist today/overdue filters.
**Warning signs:** morning brief references far-future tasks or events outside the next 12 hours.

### Pitfall 3: Collapsing deferrals and focus blocks into existing commitment fields
**What goes wrong:** session output cannot faithfully represent deferred tasks or proposed focus windows.
**Why it happens:** commitments are the only obvious durable actionable object today.
**How to avoid:** persist those as typed outcome fields on the daily session first.
**Warning signs:** code reaches for `CommitmentStatus::Cancelled` or arbitrary metadata strings to represent deferral.

### Pitfall 4: Losing resumability on silence or `skip`
**What goes wrong:** the session advances in memory but nothing durable reflects partial progress.
**Why it happens:** current Apple/context flows are single-request interactions.
**How to avoid:** persist each turn decision, current phase, and remaining prompt budget before responding.
**Warning signs:** `skip` is implemented only in client code; there is no server-side resume status.

### Pitfall 5: Letting Morning Overview create commitments
**What goes wrong:** the passive orienting step becomes another task-creation flow.
**Why it happens:** commitment storage already exists and is easy to call.
**How to avoid:** Morning should only write intent signals/session state; commitment writes belong only to Standup finalization.
**Warning signs:** morning route calls `insert_commitment`.

### Pitfall 6: Repeating the same friction/action evidence
**What goes wrong:** the loop becomes noisy and loses trust.
**Why it happens:** `Now`, suggestions, writebacks, conflicts, and review items can all point at the same underlying issue.
**How to avoid:** dedupe by stable source/evidence key before selecting friction callouts or proposed focus protections.
**Warning signs:** one underlying calendar pressure appears as multiple spoken warnings in a single session.

### Pitfall 7: Ignoring current validation debt
**What goes wrong:** the planner assumes backend tests are green when they are not.
**Why it happens:** Phase 10 itself is not the source of the compile break.
**How to avoid:** treat the 2026-03-19 linking compile errors as a Wave 0 validation blocker for backend test execution.
**Warning signs:** `cargo test -p veld ...` fails before touching any daily-loop code.

## Code Examples

Verified patterns from shipped repo sources:

### Run-backed session start

```rust
// Source pattern: crates/veld/src/services/context_runs.rs
let run_id = RunId::new();
state
    .storage
    .create_run(&run_id, RunKind::Synthesis, &serde_json::json!({
        "workflow": "daily_loop",
        "phase": "morning_overview",
        "session_id": session_id,
    }))
    .await?;
```

### Suggestion-style dedupe for friction callouts

```rust
// Source pattern: crates/veld/src/services/suggestions.rs
let dedupe_key = format!("{}:{}", evidence.source_kind, evidence.source_id);
if seen.insert(dedupe_key) {
    friction_callouts.push(evidence);
}
```

### Apple voice as thin shell over backend-owned state

```rust
// Source pattern: crates/veld/src/services/apple_voice.rs
let capture_id = persist_transcript_capture(state, &request, transcript).await?;
let session_turn = daily_loop::advance_session_from_voice(
    state,
    request.operation,
    request.intents,
    transcript,
).await?;
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `GET /v1/context/morning` heuristic artifact from capture text | dedicated daily-loop session over `Now`, commitments, calendar/Todoist, and voice | planned for Phase 10 on 2026-03-19 | interactive resume state should not live in old context-generation routes |
| Apple `MorningBriefing` mapped to generic explain response | Apple should start/resume the same Morning Overview session engine as text shells | planned for Phase 10 on 2026-03-19 | voice stays thin and cross-surface-consistent |
| `Now` as the operator’s orientation surface only | `Now` remains orientation authority but also hosts resume/status/outcome for the daily loop | planned for Phase 10 on 2026-03-19 | preserves one primary decision surface instead of adding a dashboard clone |

**Deprecated/outdated:**
- Treating `vel morning` output as the target Morning Overview contract.
- Reading `Now` cards back verbatim instead of building contract-specific session inputs.
- Assuming commitments can represent every standup artifact without widening the model.

## Open Questions

1. **Should Phase 10 keep `vel morning` as the primary CLI verb or introduce an explicit `vel standup` / `vel daily-loop` command family?**
   - What we know: [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) already teaches `vel morning`, and [crates/vel-cli/src/commands/morning.rs](/home/jove/code/vel/crates/vel-cli/src/commands/morning.rs) exists today.
   - What's unclear: whether compatibility aliasing is enough or whether a richer command family improves resumability/discoverability.
   - Recommendation: keep `vel morning` as a compatibility entrypoint, add explicit standup/session commands behind the same backend session engine.

2. **Where should proposed focus blocks live before any calendar write-back exists for them?**
   - What we know: the final session output must include proposed focus blocks, but there is no existing focus-block entity or calendar-write requirement in scope.
   - What's unclear: whether those should become suggestions immediately or remain session outcome records only.
   - Recommendation: persist typed proposed focus blocks in the session outcome first; optionally mirror high-confidence ones into suggestions later, but do not write calendar events in Phase 10.

3. **How much web interaction belongs in Phase 10 versus later shell polish?**
   - What we know: `Now` is the primary web surface, CLI is the strongest direct shell, and Apple already provides a voice path.
   - What's unclear: whether the web should own a full multi-turn composer in the same phase.
   - Recommendation: backend + CLI text + Apple voice should be the core delivery; web should at minimum surface session state/resume/final output inside `Now`, with richer shell work deferred to Phase 12.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` workspace + web `vitest 2.1.8` |
| Config file | [Cargo.toml](/home/jove/code/vel/Cargo.toml), [vitest.config.ts](/home/jove/code/vel/clients/web/vitest.config.ts) |
| Quick run command | `cargo test -p veld --test daily_loop_sessions && npm --prefix /home/jove/code/vel/clients/web test -- --run src/components/NowView.test.tsx` |
| Full suite command | `make verify && make test-web` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| MORNING-01 | manual start builds a morning session from next-12h calendar and Todoist today/overdue | integration | `cargo test -p veld --test daily_loop_sessions morning_start_uses_contract_filtered_inputs -- --exact` | ❌ Wave 0 |
| MORNING-02 | morning output stays within snapshot/callout/question bounds and creates no commitments | integration | `cargo test -p veld --test daily_loop_sessions morning_overview_is_passive_and_bounded -- --exact` | ❌ Wave 0 |
| MORNING-03 | morning stores intent signals only and keeps question budget <= 3 | integration | `cargo test -p veld --test daily_loop_sessions morning_intent_capture_does_not_write_commitments -- --exact` | ❌ Wave 0 |
| STANDUP-01 | standup can start directly or resume from morning and builds must/should/stretch buckets | integration | `cargo test -p veld --test daily_loop_sessions standup_compresses_calendar_and_tasks -- --exact` | ❌ Wave 0 |
| STANDUP-02 | standup finalization creates 1-3 commitments and persists deferrals/calendar/focus outcome | integration | `cargo test -p veld --test daily_loop_sessions standup_persists_bounded_outcome -- --exact` | ❌ Wave 0 |
| STANDUP-03 | more than 3 commitments forces reduction; zero commitments reprompts once then exits | integration | `cargo test -p veld --test daily_loop_sessions standup_enforces_reduction_and_single_reprompt -- --exact` | ❌ Wave 0 |
| SESSION-01 | skip/silence/disengagement persist resume state and can be resumed later | integration | `cargo test -p veld --test daily_loop_sessions session_skip_and_resume_are_durable -- --exact` | ❌ Wave 0 |
| VOICE-01 | Apple voice preserves transcript provenance, then routes morning/standup turns through the same backend session engine | integration | `cargo test -p veld --test apple_voice_loop apple_voice_daily_loop_uses_shared_session_engine -- --exact` | ✅ extend existing |

### Sampling Rate

- **Per task commit:** `cargo test -p veld --test daily_loop_sessions`
- **Per wave merge:** `cargo test -p veld --test daily_loop_sessions && cargo test -p veld --test apple_voice_loop && npm --prefix /home/jove/code/vel/clients/web test -- --run src/components/NowView.test.tsx`
- **Phase gate:** `make verify && make test-web` after unrelated linking compile break is resolved

### Wave 0 Gaps

- [ ] [crates/veld/tests/daily_loop_sessions.rs](/home/jove/code/vel/crates/veld/tests/daily_loop_sessions.rs) — new end-to-end daily session coverage for MORNING/STANDUP/SESSION requirements
- [ ] [crates/vel-core/src/daily_loop.rs](/home/jove/code/vel/crates/vel-core/src/daily_loop.rs) — typed domain contracts for session state, deferrals, and focus-block proposals
- [ ] [crates/veld/src/routes/linking.rs](/home/jove/code/vel/crates/veld/src/routes/linking.rs) and [crates/veld/src/services/linking.rs](/home/jove/code/vel/crates/veld/src/services/linking.rs) — unrelated compile errors currently block `cargo test -p veld` as of 2026-03-19
- [ ] [clients/web/src/components/NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx) or a new dedicated session component test — resume/status rendering coverage if web shell lands in-phase

## Sources

### Primary (HIGH confidence)

- [10-CONTEXT.md](/home/jove/code/vel/.planning/phases/10-daily-loop-morning-overview-and-standup-commitment-engine/10-CONTEXT.md) - locked product contract, architecture constraints, and canonical seam list
- [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md) - Phase 10 goal, requirement IDs, roadmap order, and scope note
- [PROJECT.md](/home/jove/code/vel/.planning/PROJECT.md) - accepted product-direction constraints, especially "no second planning subsystem"
- [STATE.md](/home/jove/code/vel/.planning/STATE.md) - current roadmap priority note and active-lane context
- [MASTER_PLAN.md](/home/jove/code/vel/docs/MASTER_PLAN.md) - canonical shipped-status tracker
- [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) - current repeated operator loop and existing CLI verbs
- [surfaces.md](/home/jove/code/vel/docs/user/surfaces.md) - shipped `Now`, commitments, and current-context behavior
- [runtime.md](/home/jove/code/vel/docs/api/runtime.md) - current `/v1` surface inventory and Apple route contract
- [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) - current web `Now` presentation and freshness/action stack shell
- [context.ts](/home/jove/code/vel/clients/web/src/data/context.ts) - frontend load path for `/v1/now` and commitments
- [now.rs](/home/jove/code/vel/crates/veld/src/services/now.rs) - backend `Now` service seam and current task/event selection rules
- [now.rs route](/home/jove/code/vel/crates/veld/src/routes/now.rs) - service-to-DTO mapping for `Now`
- [operator_queue.rs](/home/jove/code/vel/crates/veld/src/services/operator_queue.rs) - ranked action/intervention projection seam
- [apple_voice.rs](/home/jove/code/vel/crates/veld/src/services/apple_voice.rs) - provenance-first Apple voice handling and current morning-briefing behavior
- [context_runs.rs](/home/jove/code/vel/crates/veld/src/services/context_runs.rs) - run-backed generation pattern
- [context_generation.rs](/home/jove/code/vel/crates/veld/src/services/context_generation.rs) - older morning artifact contract that should not be confused with Phase 10
- [review.rs](/home/jove/code/vel/crates/vel-cli/src/commands/review.rs) - CLI review seam over `Now`
- [morning.rs](/home/jove/code/vel/crates/vel-cli/src/commands/morning.rs) - current CLI morning command behavior
- [commitments.rs](/home/jove/code/vel/crates/vel-cli/src/commands/commitments.rs) - CLI commitment mutation shell
- [commitment.rs](/home/jove/code/vel/crates/vel-core/src/commitment.rs) - current commitment domain limits
- [context.rs](/home/jove/code/vel/crates/vel-core/src/context.rs) - typed current-context pattern and why session state should not be added there
- [apple.rs](/home/jove/code/vel/crates/vel-core/src/apple.rs) - Apple voice intent/request/response contracts
- [run.rs](/home/jove/code/vel/crates/vel-core/src/run.rs) - run lifecycle/status/event model
- [0012_current_context.sql](/home/jove/code/vel/migrations/0012_current_context.sql) - singleton context pattern
- [0013_context_timeline.sql](/home/jove/code/vel/migrations/0013_context_timeline.sql) - append-only timeline pattern
- [0017_suggestions.sql](/home/jove/code/vel/migrations/0017_suggestions.sql) and [0026_suggestion_engine_upgrade.sql](/home/jove/code/vel/migrations/0026_suggestion_engine_upgrade.sql) - evidence-bearing dedupe pattern
- [0043_phase8_execution_handoffs.sql](/home/jove/code/vel/migrations/0043_phase8_execution_handoffs.sql) - typed session-like persistence pattern for new runtime-owned records
- [apple_voice_loop.rs](/home/jove/code/vel/crates/veld/tests/apple_voice_loop.rs) - existing Apple voice integration test seam
- [suggestion_engine.rs](/home/jove/code/vel/crates/veld/tests/suggestion_engine.rs) - existing dedupe/noise suppression test seam

### Secondary (MEDIUM confidence)

- None required. This research was constrained to the repo’s locked contract and shipped code seams.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - repo stack is fixed by workspace manifests and project constraints
- Architecture: MEDIUM - core seam recommendations are well-supported, but exact route/command naming remains discretionary
- Pitfalls: HIGH - derived directly from current code behavior versus locked contract

**Research date:** 2026-03-19
**Valid until:** 2026-03-26
