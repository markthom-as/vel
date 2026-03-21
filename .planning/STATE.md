---
gsd_state_version: 1.0
milestone: v0.1
milestone_name: milestone
current_phase: 46
current_phase_name: canonical-now-contract-boundaries-and-milestone-lock
current_plan: 1
status: Phase 46 is planned and ready for execution
stopped_at: Completed 21-01; next logical step is 21-02 web/desktop push-to-talk polish over the shared assistant seam
last_updated: "2026-03-21T06:48:57.968Z"
last_activity: 2026-03-21
progress:
  total_phases: 6
  completed_phases: 0
  total_plans: 4
  completed_plans: 0
  percent: 89
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-19)

**Core value:** Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.
**Current focus:** Phase 46 — canonical-now-contract-boundaries-and-milestone-lock

Status: Phase 46 is planned and ready for execution
Current Phase: 46
Current Phase Name: canonical-now-contract-boundaries-and-milestone-lock
Current Plan: 1
Total Plans in Phase: 4
Progress: 89%
Last Activity: 2026-03-21
Last Activity Description: Phase 46 execution started

## Current Position

Phase: 46 (canonical-now-contract-boundaries-and-milestone-lock) — EXECUTING
Plan: 1 of 4

## Performance Metrics

**Velocity:**

- Total plans completed: 38
- Average duration: 22m
- Total execution time: 186m

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1.1 P01 | 1 | 29m | 29m |
| 02 P01 | 1 | 9m | 9m |
| 03 P01 | 1 | 47m | 47m |
| 03 P02 | 1 | 7m | 7m |
| 03 P03 | 1 | 4m | 4m |
| 03 P04 | 1 | 34m | 34m |
| 03 P05 | 1 | 23m | 23m |

**Recent Trend:**

- Last 5 plans: 23m
- Trend: stable

*Updated after each plan completion*
| Phase 07 P1 | 11m | 2 tasks | 12 files |
| Phase 07 P02 | 6m | 2 tasks | 9 files |
| Phase 07 P3 | 5m | 2 tasks | 8 files |
| Phase 07 P04 | 7m | 2 tasks | 9 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [Init]: GSD phases match master plan phases (2, 3, 4) — tickets already grouped, avoids re-scoping
- [Init]: Ticket files in `docs/tickets/` are the authoritative implementation specs — no re-derivation needed
- [Init]: Domain research skipped — domain is well-understood; tickets are prescriptive
- [Phase 1.1]: Re-export pattern for backward compat: only OPERATOR/WORKER_AUTH_HEADER re-exported from app.rs; test module imports directly from crate::middleware
- [Phase 1.1]: main.rs requires explicit mod middleware; declaration - Rust binary targets have independent module trees from lib.rs
- [Phase 1.1]: Phase gate clippy fixes: pre-existing clippy warnings fixed to achieve zero-warning state (4 vel-core, 5 vel-storage, 20 veld, 11 vel-cli)
- [Phase 2 P01]: Diagnostics route in own file (routes/diagnostics.rs) not signals.rs — clean separation of concerns
- [Phase 2 P01]: Freshness threshold 5 minutes for fresh/stale classification — matches typical heartbeat intervals
- [Phase 2 P01]: Broker scope: agents-only (ticket 016) — integration-level brokering deferred per 2026-03-18 decision
- [Phase 2 P01]: CLI connect stubs use eprintln + exit 0 (not error) — informative, non-alarming for shell scripts
- [Phase 3 P01]: Trace fallback rule — when older persisted runs lack explicit trace metadata, surface `run_id` as `trace_id` for operator compatibility
- [Phase 3 P01]: Trace contract starts at run/operator boundary first — richer persistence and UI follow in later Phase 3 slices
- [Phase 3 P02]: Reuse existing Recent Runs and `vel run inspect` surfaces before building any separate trace explorer
- [Phase 3 P03]: User-doc support guidance should point operators to shipped inspect surfaces and runtime API docs before deeper architecture docs
- [Phase 3 P04]: Deterministic replay should normalize identifier churn (run/artifact/capture IDs) and assert semantic outputs plus boundary events instead of raw row identity
- [Phase 3 P05]: Eval fixtures should carry explicit judge policy and rubric, while deterministic replay remains the hard gate and judge failure remains separately reportable
- [Phase 4 P01]: Publish Phase 4 contracts in `vel-core` plus config schemas/examples/templates before selecting concrete runtime/index implementation details
- [Phase 4 P02]: Ship a deterministic local semantic baseline first — capture-backed indexing plus provenance-bearing hybrid retrieval — before widening to heavier embedding backends
- [Phase 4 P03]: Close the broker TODOs before claiming sandbox isolation; the host executor can ship over decoded ABI envelopes first, while concrete WASM guest embedding remains a later implementation choice
- [Phase 4 P04]: Move swarm envelope ownership into a dedicated crate before building SDKs, but keep `vel-core` as a compatibility re-export until runtime consumers migrate
- [Phase 4 P05]: Keep the first shipped SDK slice crate-local and deterministic — prove the envelope, lease, broker, and sandbox flow end-to-end in Rust before widening to additional SDK languages or transports
- [Phase 5+]: `Now + Inbox` remain the primary operator surfaces; projects exist first as typed substrate and shared workspace context
- [Phase 5+]: There is no remaining active roadmap work before Phase 5; unfinished original-scope work from historical Phase 2 and Phase 4 was explicitly re-scoped into Phases 5, 6, and 8
- [Phase 5+]: Vel should optimize equally for action and intervention; `Now` and `Inbox` must surface both next actions and slipping/conflicted/stale items that need operator attention
- [Phase 5+]: Multi-client continuity is part of the product, not just sync plumbing; web, Apple, CLI, and later worker surfaces must preserve one coherent action/intervention state
- [Phase 5+]: `Personal`, `Creative`, and `Work` are project families, not projects
- [Phase 5+]: Each project has one primary repo and one primary notes root, plus optional secondary repos/notes links
- [Phase 5+]: Upstream systems stay authoritative; Vel may autonomously perform safe writes, but prompts on conflicts or risky changes
- [Phase 5+]: External project creation in Todoist/notes roots is operator-confirmed as part of the new-project workflow
- [Phase 6+]: Todoist labels remain supported for compatibility, but Vel should prefer typed internal scheduling/writeback fields over tag syntax
- [Phase 6+]: Transcripts should fold under notes as a source subtype rather than stay a separate top-level product surface
- [Phase 6+]: People/identity should stay practical first: names, handles, platforms, relationship/context, linked files, birthdays, last-contacted, and commitments
- [Phase 7+]: Apple priority is fast voice capture, current schedule retrieval, and nudge response; useful health signals are steps/stand/exercise-style behavior indicators
- [Phase 8+]: Task handoff is part of the product, not just protocol plumbing; human-to-agent and agent-to-agent delegation must be explicit, inspectable, and reviewable
- [Phase 8+]: GSD integration should begin through repo-local docs/context that GSD already consumes; Vel should eventually route by token budget and agent profile
- [Phase 9+]: Backup matters mainly as trust against loss; recovery is lower priority than core usability and backup/export confidence
- [Phase 10+]: The next major product-value lane after backup/trust should be a strict daily loop built on existing `Now`, commitments, calendar/Todoist, and Apple voice seams rather than a brand-new planner subsystem.
- [Phase 11+]: Agent grounding over real Vel state and bounded tool surfaces is important enough to be a committed roadmap phase, not just backlog.
- [Phase 12+]: The raw UI/integration backlog should be narrowed to shell, onboarding, docs, and connector ergonomics first; broad provider proliferation stays deferred until the daily loop and agent grounding are working.
- [Phase 07]: Apple voice turns now persist transcript provenance before any query or mutation response is returned.
- [Phase 07]: Apple schedule answers are derived from backend /v1/now output rather than Swift-local synthesis.
- [Phase 07]: Low-risk Apple voice mutations reuse the existing client_sync action path and fail closed when the backend cannot resolve a safe target.
- [Phase 07]: Apple schedule retrieval now uses typed /v1/now transport and cache data instead of Swift-local schedule synthesis.
- [Phase 07]: Supported iPhone voice replies route through the backend Apple voice endpoint; offline fallback is limited to provenance capture, cached backend rendering, and queued safe actions.
- [Phase 07]: Apple quick-loop auth stays in shared VelAPI transport via explicit operator/bearer header configuration rather than per-view request logic.

### Roadmap Evolution

- Phase 1.1 inserted after Phase 1 (2026-03-18): Preflight hardening — integration startup panics, WAL mode, app.rs decomp (URGENT — gates Phase 2)
- Phase 3 planning created (2026-03-18): 5-plan rollout covering trace/doc closure, simulation harness, and eval pipeline
- Phase 4 planning created (2026-03-18): 5-plan rollout covering contract foundations, semantic retrieval, sandbox runtime, protocol fixtures, and SDK closure
- Phase 5 added (2026-03-18): Now + Inbox core and project substrate
- Phase 6 added (2026-03-18): High-value write-back integrations and lightweight people graph
- Phase 7 added (2026-03-18): Apple action loops and behavioral signal ingestion
- Phase 8 added (2026-03-18): Coding-centric supervised execution with GSD and local agents
- Phase 9 added (2026-03-18): Backup-first trust surfaces and simple operator control
- Phase 5-9 refined (2026-03-18): concrete scope now captures project families, typed project substrate, safe write-back/conflict rules, action-plus-intervention loops, multi-client continuity, Apple quick loops, GSD-aware execution context, explicit handoff boundaries, and backup-first trust priorities
- Pre-Phase-5 audit and re-scope (2026-03-19): Phase 2 and Phase 4 completion claims were too broad; unfinished original-scope work was moved into Phases 5, 6, and 8 while keeping the old phases closed as historical baselines
- Phase 6 planning created (2026-03-18): 7-plan rollout covering contracts, reconciliation, Todoist write-back, notes/reminders, people/graph expansion, GitHub/email, and operator-surface closure
- Phase 7 planning created (2026-03-19): 4-plan rollout covering Apple contracts, backend-owned voice loops, bounded behavior ingestion, and Apple client/doc closure
- Phase 8 planning created (2026-03-19): 6-plan rollout covering execution contracts, repo-local GSD artifacts, connect transport closure, routing/handoff review, direct guest runtime, and SDK/docs closure
- Phase 8 parallel execution advanced (2026-03-19): plans 08-04 through 08-06 completed after 08-01 through 08-03, leaving Phase 08 ready to verify behind the active Phase 07 lane
- Phase 7 and Phase 8 closed (2026-03-19): Phase 07 completed after skipped-manual UAT, then Phase 08 UAT was skipped and the roadmap advanced to Phase 09
- Phase 10 added (2026-03-19): Daily-loop morning overview and standup commitment engine
- Phase 11 promoted (2026-03-19): Agent grounding and operator-relevant data/tool awareness
- Phase 12 added (2026-03-19): Operator shell, onboarding, and connector ergonomics
- CSV backlog triaged (2026-03-19): daily-loop fixes assigned to Phase 10, agent awareness promoted into Phase 11, shell/interface fixes assigned to Phase 12, and remaining provider/platform expansion captured in BACKLOG.md
- Phase 10 planning created (2026-03-19): 5-plan rollout covering typed session contracts, morning backend, standup/CLI closure, web shell, and transcript-first Apple voice closure
- Phase 11 planning created (2026-03-19): 3-plan rollout covering contract publication, backend inspect/export grounding, and thin CLI/web trust surfaces
- Phase 12 planning created (2026-03-19): 4-plan rollout covering shell/help contracts, shell navigation/freshness polish, project/settings ergonomics, and onboarding/linking/path-discovery closure
- Phase 12 execution started (2026-03-19): 12-01 completed with a docs-first decision to reuse existing typed runtime routes and user guides for contextual help/setup routing
- Phase 12 execution advanced (2026-03-19): 12-02 completed with primary/support shell grouping, latest-thread fallback, and calmer freshness wording over the same backend-owned actions
- Phase 12 execution advanced (2026-03-19): 12-03 completed with bounded project draft handoff, clearer Settings help framing, and explicit operator-versus-internal path separation
- Phase 12 completed (2026-03-19): 12-04 closed onboarding, linking, Apple/macOS path discovery, and setup/troubleshooting doc alignment
- Phase 13 added (2026-03-19): Cross-surface core architecture and adapter boundaries
- Phase 14 added (2026-03-19): Product discovery, operator modes, and milestone shaping
- Phase 15 added (2026-03-19): Incremental core migration and canonical Rust service seams
- Phase 16 added (2026-03-19): Logic-first product closure on canonical core surfaces
- Phase 17 added (2026-03-19): Shell embodiment, operator-mode application, and surface simplification
- Phase 13 planning created (2026-03-19): 4-plan rollout covering architecture truth, contract vocabulary, future Apple/desktop migration paths, and one shipped proof flow
- Phase 13 execution started (2026-03-19): 13-01 began with the canonical cross-surface architecture authority doc and current-to-target mapping
- Phase 14 discovery started (2026-03-19): parallel research launched to shape operator modes, advanced/dev gating, milestone boundaries, and possible new follow-on phases
- Phase 14 planning created (2026-03-19): 4-plan rollout covering surface taxonomy, onboarding/trust journeys, operator-mode policy, and milestone reshaping
- Phase 13 completed (2026-03-19): cross-surface architecture, contract vocabulary, future Apple/desktop paths, and proof flows are all documented and Phase 14 is now the active lane
- Phase 14 execution started (2026-03-19): 14-01 began with the canonical operator-surface taxonomy
- Phase 14 execution advanced (2026-03-19): 14-02 published onboarding/trust/recovery journeys and aligned daily-use/setup routing
- Phase 14 execution advanced (2026-03-19): 14-03 published operator-mode policy, minimal `Now` rules, heavier `reflow`, inline `check_in`, and project-scoped action guidance
- Phase 14 completed (2026-03-19): milestone reshaping locked 15 -> 16 -> 17 sequencing, preserved the action model and disclosure policy, and advanced the next lane to Phase 15 planning
- Phase 15 planning created (2026-03-19): 5-plan migration rollout covering contract tightening, `check_in`, `reflow`, trust/readiness projections, and project-scoped action ownership
- Phase 15 execution started (2026-03-19): 15-01 completed by tightening the core queue/DTO contract with explicit permission and scope semantics and by documenting `operator_queue` as the migration seam
- Phase 15 execution advanced (2026-03-19): 15-02 completed by introducing the first backend-owned `check_in` seam, sourced from active daily-loop prompt state and exposed through the `Now` read model
- Phase 15 execution advanced (2026-03-19): 15-03 completed by introducing the first backend-owned `reflow` seam, sourced from typed current-context drift, stale schedule age, and missed-event timing
- Phase 15 execution advanced (2026-03-19): 15-04 completed by composing the first backend-owned trust/readiness summary from backup trust, freshness, conflicts/writebacks, and supervised review pressure
- Phase 15 completed (2026-03-19): 15-05 preserved project-scoped operator-action ownership through core, service, DTO, and shell seams and advanced the next lane to Phase 16 planning
- Phase 16 discussion completed (2026-03-19): wrote `16-CONTEXT.md` to lock logic-first closure priorities, product rules, and code-context references ahead of planning
- Phase 24 added (2026-03-20): approved assistant action application and reversible write execution became the next feature lane after staged assistant proposals landed in Phase 23
- Phase 25 added (2026-03-20): local recall, semantic memory, and grounded assistant context became the next feature lane after approved assistant application landed in Phase 24
- Phase 25 planning created (2026-03-20): 4-plan rollout covering recall/grounding contract tightening, semantic retrieval quality, backend-owned assistant context assembly, and shell/docs verification closure
- Phase 25 execution started (2026-03-20): 25-01 completed by publishing a typed recall-context pack over canonical semantic retrieval and exposing it through the bounded assistant tool seam
- Phase 25 execution advanced (2026-03-20): 25-02 completed by broadening lexical scoring beyond capture FTS, improving hybrid candidate selection, and verifying real lexical credit for non-capture entities
- Phase 25 execution advanced (2026-03-20): 25-03 completed by assembling backend-owned assistant context packs, attaching them to assistant-capable chat responses, and normalizing conversational recall queries for safe capture FTS use
- Phase 25 completed (2026-03-20): 25-04 aligned shell and docs wording so the shipped product now teaches one honest bounded-local-recall story across API, runtime, setup, daily use, web, and CLI
- Phase 26 added (2026-03-20): real day-plan reflow and schedule reconciliation became the next feature lane after bounded local recall and grounded assistant context closed in Phase 25
- Phase 16 planning created (2026-03-19): 5-plan rollout covering transition contracts, backend-owned `check_in`, backend-owned `reflow`, trust/readiness follow-through, and project-scoped action behavior
- Phase 16 execution started (2026-03-19): 16-01 completed by publishing typed backend-owned `check_in` and `reflow` transitions and wiring them through the `Now`/web contract
- Phase 16 execution advanced (2026-03-19): 16-02 completed by implementing backend-owned `check_in` submit/bypass validation, typed resolution history, and daily-loop persistence through API, web, and Apple boundaries
- Phase 16 execution advanced (2026-03-19): 16-03 completed by implementing backend-owned `reflow` apply/edit handling, typed persisted follow-up status on current context, and thread-backed edit escalation surfaced through `Now`
- Phase 16 execution advanced (2026-03-19): 16-04 completed by surfacing canonical backend-owned trust/readiness follow-through actions for degraded backup, freshness, conflict, and review posture
- Phase 16 completed (2026-03-19): 16-05 closed project-scoped action behavior with typed thread routing hints and filtered thread-route support, leaving Phase 17 to embody already-owned backend semantics
- Phase 17 planning created (2026-03-19): 4-plan rollout covering web shell classification, default-surface embodiment, advanced/support disclosure, and Apple/CLI alignment
- Phase 17 execution started (2026-03-19): 17-01 completed by publishing one shared web shell taxonomy source and reclassifying top-level navigation around daily-use, support, advanced, and detail surfaces
- Phase 17 execution advanced (2026-03-19): 17-02 completed by tightening web `Now` into a minimal urgent-first surface, keeping `Inbox` triage-first, and reframing `Threads` as continuity/history over the same typed backend-owned state
- Phase 17 execution advanced (2026-03-19): 17-03 completed by reframing `Projects` as contextual drill-down, making `Settings` summary-first before runtime internals, and demoting `Stats`/`Suggestions` into passive detail surfaces
- Phase 17 completed (2026-03-19): 17-04 aligned Apple and CLI wording, grouping, and docs to the same summary-first shell taxonomy, completing the embodiment lane without widening backend semantics
- Milestone audit created (2026-03-19): v0.1 audit reported closeout gaps in verification coverage, requirements reconciliation, and roadmap/archive readiness; milestone archival is blocked pending gap planning
- Milestone gap phases planned (2026-03-19): added Phase 18 for verification/requirements reconciliation and Phase 19 for archive readiness, re-audit, and milestone closeout
- Phase 18 planning created (2026-03-19): 4-plan rollout covering closeout inventory/rules, historical verification backfill, shipped-phase verification backfill, and requirements reconciliation
- Phase 18 execution started (2026-03-19): 18-01 created the stable closeout inventory and reconciliation rules, mapping milestone phases to summary/verification coverage and ledger posture before backfill begins
- Phase 18 execution advanced (2026-03-19): 18-02 backfilled durable verification artifacts for Phases 2-4, verifying Phase 3 as complete and Phases 2/4 as historical baselines with explicit deferred scope
- Phase 18 execution advanced (2026-03-19): 18-03 backfilled durable verification artifacts for Phases 5-17, preserving the original no-UAT and environment-gap notes while creating one phase-level verification substrate per shipped phase
- Phase 18 completed (2026-03-20): 18-04 reconciled REQUIREMENTS.md against verification truth, satisfied CLOSEOUT-01/CLOSEOUT-02, updated the audit follow-up note, and left a concrete Phase 19 handoff for archive-readiness and rerun audit work
- Phase 19 discussion completed (2026-03-20): wrote `19-CONTEXT.md` to lock the repaired closeout truth, remaining audit blockers, and archive-readiness deliverables before planning
- Phase 19 planning created (2026-03-20): 4-plan rollout covering metadata repair, milestone-level integration evidence, milestone-level flow evidence, and rerun audit/archival readiness
- Phase 19 deferred (2026-03-20): operator chose to skip closeout bookkeeping for now and move back to feature work focused on usability
- Phase 20 discussion completed (2026-03-20): wrote `20-CONTEXT.md` to lock the next feature lane around daily-use usability, unified entry, and reduced setup friction
- Phase 20 completed (2026-03-20): the default web loop now enters through `Now`, triages in `Inbox`, continues in `Threads`, and uses summary-first `Settings` for assistant readiness and setup depth
- Phase 21 planning created (2026-03-20): 4-plan rollout covering shared backend voice entry, desktop/browser push-to-talk polish, Apple voice alignment, and cross-surface parity closeout
- Phase 21 execution started (2026-03-20): 21-01 completed by extending assistant entry with explicit voice provenance and reusing the same normalization helper for Apple transcript capture
- Phase 21 execution advanced (2026-03-20): 21-02 completed by making desktop/browser voice a clearer push-to-talk path into the shared assistant seam with explicit local-STT fallback behavior and web voice provenance on assistant entry
- Phase 21 execution advanced (2026-03-20): 21-03 completed by aligning Apple voice onto the shared assistant continuity seam while preserving typed Apple responses and bounded offline/cache behavior
- Phase 21 completed (2026-03-20): 21-04 closed cross-surface voice docs and verification notes so web/browser and Apple now teach one backend-owned voice story with explicit remaining platform limits
- Phase 22 planning created (2026-03-20): 4-plan rollout covering assistant-capable morning/standup routing, assistant-capable closeout, durable thread-resolution follow-through, and shell/docs verification closure

### Pending Todos

0 todos remaining (8 completed in 02-01/04-04):

- DONE: Ticket 006: add Current Baseline section (02-01)
- DONE: Ticket 016: add broker scope decision record (agents-only) (02-01)
- DONE: Ticket 005: add NodeIdentity prereq + WAL mode step (02-01)
- DONE: Ticket 007: define vel-sim crate interface contract in SP2 harness slice (03-04)
- DONE: Ticket 008: add judge model strategy via explicit `judge` routing task + per-fixture override (03-05)
- DONE: Ticket 009: add embedding model, index rebuild trigger, hybrid ranking contracts (04-02)
- DONE: Ticket 010: host ABI policy enforcement, deny-by-default decisions, and operator-visible diagnostics baseline (04-03)
- DONE: Ticket 014: define protocol versioning strategy and dedicated crate ownership baseline (04-04)
- DONE: Ticket 014: ship reference Rust SDK helpers plus end-to-end scoped capability flow through the authority runtime (04-05)

### Blockers/Concerns

- Phase 2 residual gaps identified by audit: live `/v1/connect` routes still stubbed; `vel node link` / pairing flow absent; `NodeIdentity` / ordering primitive not found in live code
- Phase 4 residual gaps identified by audit: sandbox is a host-executor baseline rather than direct WASM guest runtime; semantic memory is capture-backed rather than full graph expansion; external SDK/connect transport exposure is still absent
- There is no remaining active future roadmap work before Phase 5 because those gaps were re-scoped into Phases 5, 6, and 8
- Phase 06 is complete; the next active implementation lane is Phase 07 execution
- Concrete WASM guest runtime choice is still unresolved, but it no longer blocks Phase 4 closure because the shipped boundary is the decoded-ABI host executor plus supervised protocol/runtime mediation baseline
- Phase 6 execution should build on the completed typed project/action/linking substrate and preserve backend-owned conflict and write-back policy
- Phase 08 implementation is complete and closed; the next active lane is Phase 09 backup/trust execution
- Phase 20 is complete; the default web daily-use loop now enters through Now, triages in Inbox, continues in Threads, and defers setup depth to summary-first Settings

## Session Continuity

Last session: 2026-03-19T07:35:55.445Z
Stopped at: Completed 21-01; next logical step is 21-02 web/desktop push-to-talk polish over the shared assistant seam
Resume file: None
