# Phase 6: High-value write-back integrations and lightweight people graph - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 6 turns the typed project, action, linking, and continuity substrate from Phase 5 into a safe write-back and reconciliation layer. The phase must let Vel perform bounded high-value writes against the integrations that matter most to the operator, while keeping upstream systems authoritative, conflicts explicit, provenance durable, and cross-client state deterministic.

This phase also absorbs two re-scoped follow-ons from earlier historical phases:

- deterministic ordering and reconciliation follow-on from historical Phase 2
- richer semantic graph expansion beyond the shipped capture-backed baseline from historical Phase 4

Phase 6 is not a broad provider-sprawl phase. It should prioritize the existing and clearly named product lanes in the roadmap: Todoist, notes, reminders, GitHub, email, transcripts-under-notes, lightweight people identity, and operator-visible conflict/provenance surfaces. It does not include Apple-first quick-loop UX, direct external connect transport, general-purpose broker expansion for integrations, or backup/recovery product work.

</domain>

<decisions>
## Implementation Decisions

### Upstream authority and safe writes
- [auto] Upstream systems remain authoritative for their own records. Vel may create safe net-new records and perform bounded field updates, but it must not silently overwrite ambiguous upstream changes.
- [auto] Every write-capable integration must declare exact allowed operations. Phase 6 should not ship any provider surface that effectively means "Vel can do anything this API supports."
- [auto] Writes default to the smallest useful safe set:
  - Todoist: create task, update content/due/priority/project/completion, reopen task
  - Notes: create note and append/update inside configured notes roots only
  - Reminders: create/update/complete reminder through a mediated local-executor path when available; otherwise queue the action visibly
  - GitHub: create issue, comment on issue/PR, close/reopen issue, update explicit tracked fields only
  - Email: create draft reply first; sending remains confirm-required
- [auto] Every write attempt must persist an inspectable record with operation kind, target, requested payload, resulting upstream reference, status, and conflict/review requirement.

### Conflict and reconciliation model
- [auto] Phase 6 should introduce a deterministic ordering primitive and explicit `NodeIdentity`/origin semantics before widening write-back reconciliation.
- [auto] Upstream-vs-local disagreements must become typed conflict records, not log-only warnings or silent last-write-wins behavior.
- [auto] Conflict records should be routed into existing operator surfaces through backend-owned action/intervention vocabulary so `Now`, `Inbox`, and settings/status views can show them without inventing a separate policy stack in clients.
- [auto] Ambiguous or destructive operations require explicit operator confirmation. Safe replayable operations may auto-apply only when the connector contract and current freshness state say they are safe.

### Todoist boundary
- [auto] Todoist labels remain a compatibility layer only. Vel-native task scheduling and write-back fields must live in typed internal records, not in label syntax.
- [auto] Todoist project/task linkage should reuse the typed Phase 5 project substrate through stored upstream IDs and typed source references rather than loose slug guessing.
- [auto] Todoist write-back must persist enough provenance to explain why a task was created, updated, completed, or held for conflict review.

### Notes, reminders, and transcript handling
- [auto] Notes writes must stay confined to configured local notes roots or project-linked notes roots; Phase 6 must not introduce arbitrary filesystem write scope.
- [auto] Transcripts should fold under notes as a source subtype rather than remain a separate top-level product surface.
- [auto] Reminder write-back should be modeled as a typed write intent plus explicit execution/result tracking so local executors and linked clients can participate without becoming policy owners.

### People registry and graph expansion
- [auto] People identity stays practical first: names, handles, platforms, relationship/context, linked files, birthdays, last-contacted, and commitment/message/project links.
- [auto] People deduplication should be alias-driven and inspectable. Phase 6 should not promise magical automatic identity resolution across providers.
- [auto] Semantic graph expansion should start with the entities Phase 6 already cares about: projects, notes, transcripts, message threads, GitHub artifacts, and people records.
- [auto] Every semantic edge or retrieval hit added in this phase must carry stable provenance back to durable inputs or upstream references.

### Client and operator-surface discipline
- [auto] Rust backend layers own write policy, conflict policy, and durable state. Web, CLI, and Apple surfaces remain shells that display typed status and trigger explicit actions.
- [auto] Phase 6 should reuse the Phase 5 action/intervention and sync/bootstrap vocabulary where possible instead of creating a second write-back-specific queue model.
- [auto] Operator-facing status must expose freshness, last sync, last write result, pending conflicts, and queued actions in typed fields rather than opaque JSON blobs.

### Claude's Discretion
- Exact type/module names for write-back operations, conflict cases, upstream object refs, and people aliases
- Whether conflict queue and write-back history share one table family or remain separate but linked records
- The exact division between server-executed local writes and queued local-executor or linked-client writes for reminders/email
- Retrieval and graph schema details so long as provenance, replayability, and typed contracts remain explicit

</decisions>

<specifics>
## Specific Ideas

- Phase 5 already created the substrate this phase needs: typed projects, ranked operator queue items, linked nodes, and sync/bootstrap continuity. Phase 6 should extend those seams rather than bypass them.
- Existing integration connections and local-source status already have the beginnings of provenance and operator visibility. Planning should elevate those from read/sync health into full write/result/conflict tracking.
- GitHub and email are net-new provider lanes in the current tree. They should begin through the canonical connector contract and typed write-back policy rather than provider-specific ad hoc DTOs.
- Home Assistant remains backlog-only for now. It is not part of the default Phase 6 plan set unless a later inserted phase or focused todo promotes it.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and active planning authority
- `.planning/ROADMAP.md` — Phase 6 goal, requirements IDs, execution order, and re-scoped residuals from historical Phases 2 and 4
- `.planning/PROJECT.md` — accepted product decisions: upstream authority, local-first trust, typed project substrate, and post-Phase-4 direction
- `.planning/STATE.md` — accumulated decisions including Phase 6 notes for Todoist labels, transcripts-under-notes, and practical people identity
- `docs/MASTER_PLAN.md` — canonical shipped-status tracker

### Connector and integration authority
- `docs/tickets/phase-1/022-data-sources-and-connector-architecture.md` — canonical connector vocabulary and guardrails
- `docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md` — source modes, trust boundaries, and write-capable connector rules
- `docs/cognitive-agent-architecture/integrations/data-source-catalog.md` — concrete shipped/planned provider inventory
- `docs/user/integrations/README.md` — currently shipped integration truth and user-facing setup boundary

### Re-scoped historical follow-ons that Phase 6 must absorb
- `docs/tickets/phase-2/005-hlc-sync-implementation.md` — deterministic ordering, `NodeIdentity`, and reconciliation expectations
- `docs/tickets/phase-4/009-semantic-memory-rag.md` — semantic index, hybrid retrieval, graph linkage, and provenance requirements

### Phase 5 substrate that Phase 6 must extend
- `.planning/phases/05-now-inbox-core-and-project-substrate/05-CONTEXT.md` — prior locked decisions about projects, action/intervention surfaces, and thin-client continuity
- `.planning/phases/05-now-inbox-core-and-project-substrate/05-RESEARCH.md` — codebase research on project/action/linking seams
- `.planning/phases/05-now-inbox-core-and-project-substrate/05-02-SUMMARY.md` — shipped typed project substrate and local-first pending-upstream behavior
- `.planning/phases/05-now-inbox-core-and-project-substrate/05-05-PLAN.md` — ranked operator queue and typed `Now`/`Inbox` projection boundary

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/vel-core/src/integration.rs` — canonical family/provider/connection/source-ref types already exist
- `crates/vel-storage/src/repositories/integration_connections_repo.rs` — current connection and event persistence seam
- `crates/vel-storage/src/repositories/semantic_memory_repo.rs` — shipped semantic baseline over captures with hybrid retrieval logic
- `crates/veld/src/services/integrations.rs` and `crates/veld/src/services/integrations_todoist.rs` — existing integration orchestration and Todoist credential-backed sync
- `crates/veld/src/adapters/notes.rs`, `crates/veld/src/adapters/reminders.rs`, `crates/veld/src/adapters/transcripts.rs` — current local-source ingestion seams
- `crates/veld/src/services/client_sync.rs` — cross-client bootstrap seam for linked nodes, projects, and ranked action items
- `crates/veld/src/services/operator_queue.rs` — backend-owned action/intervention vocabulary that can absorb conflict and pending-write items
- `crates/vel-storage/src/repositories/projects_repo.rs` and `crates/veld/src/services/projects.rs` — project substrate and upstream ID anchor points

### Established Patterns
- Keep provider-neutral contracts in `vel-core`, persistence in `vel-storage`, and orchestration in `veld` services/routes
- Persist inspectable events and status for integration work rather than burying connector state in untyped blobs
- Reuse sync/bootstrap and operator-queue seams for cross-surface continuity instead of inventing client-specific policy
- Treat local roots and project roots as explicit write scopes; do not blur observation scope and write scope

### Missing or Thin Areas
- No current GitHub or email connector implementation exists in the runtime tree
- Current Todoist flow is sync-oriented and lacks a typed write-back operation model
- Semantic memory indexes captures today, not projects/notes/messages/people/GitHub entities
- There is no current people registry or typed alias/identity surface
- Deterministic multi-writer ordering and explicit conflict queue handling are not yet present in live Phase 5 surfaces

</code_context>

<deferred>
## Deferred Ideas

- Apple-first quick-loop UX, voice capture priority, and behavioral-signal product work — Phase 7
- Coding-centric supervised execution, external connect/auth transport, and direct WASM guest follow-on — Phase 8
- Backup/export trust workflows and broader operator control surfaces — Phase 9
- Home Assistant integration exploration — backlog only until explicitly promoted from `.planning/BACKLOG.md`

</deferred>

---

*Phase: 06-high-value-write-back-integrations-and-lightweight-people-graph*
*Context gathered: 2026-03-18*
