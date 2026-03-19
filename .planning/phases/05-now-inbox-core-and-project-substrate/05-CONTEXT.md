# Phase 5: Now + Inbox core and project substrate - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 5 delivers the first product-shaping slice after the architecture program: keep `Now` and `Inbox` as the primary operator surfaces while adding a typed project substrate, project-family structure, and a unified action/intervention model that can anchor work across commitments, notes, messages, suggestions, conflicts, and linked clients. This phase also absorbs the re-scoped guided node-linking and multi-client continuity closure that historical Phase 2 did not finish.

This phase clarifies how the operator should experience and organize work. It does not introduce broad safe write-back automation, richer semantic graph expansion, Apple-first quick loops, coding-agent execution policy, or backup/recovery product flows beyond the minimum continuity and linking seams required here.

</domain>

<decisions>
## Implementation Decisions

### Surface hierarchy and review loops
- [auto] `Now` remains the default decision cockpit. It should answer both "what should I do next?" and "what needs intervention before I trust or act on this state?"
- [auto] `Inbox` is the triage queue for unresolved intervention items, newly surfaced work, and items awaiting explicit operator classification. It is not a generic message list or a second copy of Threads.
- [auto] `Projects` becomes a supporting coordination surface, not the primary home screen. It exists to anchor work, context, and continuity behind `Now` and `Inbox`, not to replace them with a dashboard-first workflow.
- [auto] Daily flow stays capture/review oriented: capture fast, orient in `Now`, triage in `Inbox`, inspect deeper in Threads/Suggestions/Explain surfaces, and use weekly review/synthesis for higher-level project maintenance.

### Project substrate and family model
- [auto] Introduce a typed local-first project record rather than continuing to deepen the loose `commitment.project` string as the main contract.
- [auto] Project families are top-level grouping metadata (`Personal`, `Creative`, `Work`) and remain distinct from concrete projects. Families should be enumerable and stable enough to appear consistently across CLI, web, and Apple surfaces.
- [auto] Each project should carry one primary repo and one primary notes root, plus optional secondary links and upstream identifiers. This becomes the substrate for later coding, notes, and write-back phases.
- [auto] Existing commitments and synced records should remain backward compatible during migration: legacy string project references can coexist temporarily, but new backend and client contracts should move toward typed project identifiers at the boundary.
- [auto] New-project setup is local-first and operator-confirmed for any upstream creation or linking side effect.

### Unified action and intervention model
- [auto] Model action and intervention as equally first-class. Vel must surface both concrete next work and items that are stale, blocked, conflicted, degraded, or need review.
- [auto] `Now` should show a ranked small set of highest-salience actions/interventions, while `Inbox` holds the fuller queue that still needs operator triage or acknowledgement.
- [auto] Existing surfaces such as Nudges, Suggestions, and commitments remain inspectable, but Phase 5 planning should converge them toward one backend-owned action/intervention vocabulary instead of adding more parallel queues.
- [auto] Explainability remains mandatory: every surfaced action or intervention must be traceable to persisted commitments, signals, sync state, run data, or documented policy rules.

### Cross-surface continuity and guided linking
- [auto] Web, CLI, and Apple should consume one coherent persisted action/intervention/project state from the Rust backend. Clients stay thin shells and must not fork policy or durable business logic.
- [auto] Reuse the existing sync/bootstrap pattern for continuity. Linked clients should hydrate a shared baseline for current context, open commitments, nudges/interventions, and later project summaries rather than inventing client-specific state models.
- [auto] Guided node linking belongs in this phase as minimal trustworthy closure: short-lived scoped pairing tokens, explicit trust/read-write-execute visibility, and operable fallback through either CLI or web.
- [auto] Phase 5 should close the product workflow around linked-node setup/status and coherent multi-client state, but not broaden into richer Apple-native action loops or external delegated-runtime transport work.

### Claude's Discretion
- Exact domain type and table names for projects, families, action/intervention queues, and link-status persistence
- Whether action/intervention ranking is implemented as one persisted queue, one projection over multiple typed records, or a hybrid service-layer synthesis, provided explainability and traceability remain intact
- UI density and component composition for the eventual Projects surface and `Now`/`Inbox` refinements, as long as `Now` and `Inbox` stay primary
- Migration sequencing for legacy `project: Option<String>` references in commitments and transport DTOs

</decisions>

<specifics>
## Specific Ideas

- Auto-mode was used because `gsd-next` invoked this phase without interactive questioning; the recommended defaults above should be treated as the starting product contract for planning.
- The existing web shell already states the intended truth: `Projects` is a placeholder until the backend project workspace API exists, while `Now` and `Inbox` are already real operator surfaces.
- The current `Now` surface already exposes freshness, schedule, Todoist-backed work, and recovery actions; planning should evolve that surface toward explicit action-plus-intervention ranking instead of replacing it.
- Apple clients already hydrate cached current context, commitments, and nudges via sync bootstrap and queue low-risk actions offline; Phase 5 should preserve that "one brain, thin clients" model while adding coherent linking and future project continuity.
- Historical Phase 2 onboarding work is only partially closed. Phase 5 should finish the minimal trustworthy operator journey for linking a device/node and understanding what that link can read, write, and execute.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and planning authority
- `.planning/ROADMAP.md` — Phase 5 goal, requirements summary, execution order, and explicit re-scope from historical Phases 2 and 4
- `.planning/PROJECT.md` — accepted project-level decisions that already lock `Now + Inbox`, project families, upstream authority, and multi-client continuity
- `docs/MASTER_PLAN.md` — canonical implementation status, including the statement that active future work begins at Phase 5

### Operator surface behavior
- `docs/user/surfaces.md` — shipped definitions for `Now`, current context, commitments, settings, and inspectability expectations
- `docs/user/daily-use.md` — the intended daily and weekly operator loops that Phase 5 must reinforce rather than disrupt
- `docs/api/runtime.md` — current API surfaces for `Now`, current context, sync, synthesis, runs, and explainability boundaries

### Linked-client and onboarding closure
- `docs/tickets/phase-2/012-tester-readiness-onboarding.md` — residual guided linking, scoped pairing, trust visibility, and fallback-path requirements re-scoped into this phase
- `clients/apple/README.md` — current Apple client contract and the thin-client/offline-bootstrap assumptions that Phase 5 continuity work must preserve
- `clients/web/README.md` — documents the truthful Projects placeholder and confirms the current web shell boundary before project APIs exist

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/components/NowView.tsx` — current operator-facing `Now` surface with freshness actions, schedule cards, Todoist backlog, and operational state panels
- `clients/web/src/components/InboxView.tsx` — existing intervention-style queue surface that can evolve into the Phase 5 triage model
- `clients/web/src/components/MainPanel.tsx` and `clients/web/src/components/Sidebar.tsx` — already establish `Now`/`Inbox` as primary nav and keep `Projects` as an intentional placeholder
- `crates/veld/src/services/now.rs` and `crates/veld/src/routes/now.rs` — backend-owned `Now` projection and DTO mapping seam to extend rather than reimplement in clients
- `crates/vel-core/src/commitment.rs` and `crates/vel-storage/src/repositories/commitments_repo.rs` — existing actionable layer with legacy optional `project` string that Phase 5 must migrate carefully
- `crates/veld/src/services/client_sync.rs` — existing sync bootstrap and cluster state builders that already hydrate context, commitments, and nudges across clients
- `clients/apple/Apps/VelMac/ContentView.swift` plus `clients/apple/README.md` — current thin-shell Apple baseline using cached context, commitment actions, and sync bootstrap

### Established Patterns
- Rust backend owns policy, projections, and durable state; clients are shells and should not fork decision logic
- Route handlers stay thin and map service/domain output to transport DTOs; new project and continuity contracts should follow the same boundary discipline
- Current operator surfaces are truthful about maturity: real shipped surfaces remain usable while placeholder surfaces explicitly say when a backend contract does not exist yet
- Multi-client continuity already uses bootstrap hydration and queued low-risk actions, which is the right seam for carrying project/action state forward

### Integration Points
- New typed project records will likely start in `vel-core` and `vel-storage`, then flow through new `veld` services/routes and web/CLI/Apple transport DTOs
- `Now` and `Inbox` refinements should extend existing web components and backend projections, not replace them with an unrelated shell
- Guided linking and trust visibility should connect to the existing cluster/bootstrap and sync services in `crates/veld/src/services/client_sync.rs`, plus the corresponding CLI/web onboarding/status surfaces
- Legacy commitments, suggestions, nudges, and sync freshness state are the existing evidence sources from which a unified action/intervention view can be projected

</code_context>

<deferred>
## Deferred Ideas

- Safe autonomous write-back and upstream conflict resolution beyond the new-project confirmation flow — Phase 6
- Richer semantic project/people/notes graph expansion — Phase 6
- Apple-first quick loops, voice capture emphasis, and behavioral signal productization — Phase 7
- Coding-agent project execution context, GSD handoff policy, direct WASM guest follow-on, and external connect transport closure — Phase 8
- Backup/export trust surfaces and broader operator control work — Phase 9

</deferred>

---

*Phase: 05-now-inbox-core-and-project-substrate*
*Context gathered: 2026-03-18*
