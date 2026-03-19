---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
current_phase: 16
current_phase_name: logic-first-product-closure-on-canonical-core-surfaces
current_plan: 4
status: executing
stopped_at: Completed Phase 16-04; next logical step is execute 16-05
last_updated: "2026-03-19T22:42:08Z"
last_activity: 2026-03-19
progress:
  total_phases: 17
  completed_phases: 12
  total_plans: 73
  completed_plans: 65
  percent: 71
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-19)

**Core value:** Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.
**Current focus:** Phase 16 execution — logic-first-product-closure-on-canonical-core-surfaces

Status: Phase 16 in progress; 16-01 through 16-04 are complete and 16-05 is next
Current Phase: 16
Current Phase Name: logic-first-product-closure-on-canonical-core-surfaces
Current Plan: 4
Total Plans in Phase: 5
Progress: 80%
Last Activity: 2026-03-19
Last Activity Description: Completed 16-04 by adding canonical backend-owned trust/readiness follow-through actions for degraded recovery and review posture

## Current Position

Phase: 16 (logic-first-product-closure-on-canonical-core-surfaces) — IN PROGRESS
Plan: 4 of 5

## Performance Metrics

**Velocity:**

- Total plans completed: 29
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
- Phase 16 planning created (2026-03-19): 5-plan rollout covering transition contracts, backend-owned `check_in`, backend-owned `reflow`, trust/readiness follow-through, and project-scoped action behavior
- Phase 16 execution started (2026-03-19): 16-01 completed by publishing typed backend-owned `check_in` and `reflow` transitions and wiring them through the `Now`/web contract
- Phase 16 execution advanced (2026-03-19): 16-02 completed by implementing backend-owned `check_in` submit/bypass validation, typed resolution history, and daily-loop persistence through API, web, and Apple boundaries
- Phase 16 execution advanced (2026-03-19): 16-03 completed by implementing backend-owned `reflow` apply/edit handling, typed persisted follow-up status on current context, and thread-backed edit escalation surfaced through `Now`
- Phase 16 execution advanced (2026-03-19): 16-04 completed by surfacing canonical backend-owned trust/readiness follow-through actions for degraded backup, freshness, conflict, and review posture
- Phase 17 planning created (2026-03-19): 4-plan rollout covering web shell classification, default-surface embodiment, advanced/support disclosure, and Apple/CLI alignment

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
- Phase 16 is active; the next lane is 16-05 execution

## Session Continuity

Last session: 2026-03-19T07:35:55.445Z
Stopped at: Completed Phase 16-04; next logical step is 16-05 execution
Resume file: None
