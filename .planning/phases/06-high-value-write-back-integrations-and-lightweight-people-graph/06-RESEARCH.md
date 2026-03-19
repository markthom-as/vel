# Phase 6: high-value-write-back-integrations-and-lightweight-people-graph - Research

**Researched:** 2026-03-18
**Domain:** safe integration write-back, deterministic reconciliation, lightweight people identity, and provenance-bearing graph expansion
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)
- Apple-first quick-loop UX, voice capture priority, and behavioral-signal product work - Phase 7
- Coding-centric supervised execution, external connect/auth transport, and direct WASM guest follow-on - Phase 8
- Backup/export trust workflows and broader operator control surfaces - Phase 9
- Home Assistant integration exploration - backlog only until explicitly promoted
</user_constraints>

<phase_requirements>
## Phase Requirements

Phase 6 requirement IDs are present in `.planning/ROADMAP.md`, but `.planning/REQUIREMENTS.md` has not yet been expanded with post-Phase-4 rows. Descriptions below are inferred from the roadmap, `PROJECT.md`, `STATE.md`, and `06-CONTEXT.md`.

| ID | Description | Research Support |
|----|-------------|-----------------|
| WB-01 | Safe write-capable integrations expose explicit allowed operations and durable operation records | Contract-first write-back operation model with exact capability lists and typed status/result records |
| WB-02 | High-value provider slices can create or update bounded upstream records without widening to arbitrary provider access | Provider-specific services/routes on top of shared write-back policy and conflict guards |
| WB-03 | Operator surfaces show pending writes, results, and review-needed actions with typed status fields | Reuse Phase 5 operator queue, sync/bootstrap, settings/history panels, and CLI review/status outputs |
| CONFLICT-01 | Upstream-vs-local conflicts become explicit queued cases instead of silent overwrite behavior | Typed conflict queue records, deterministic tie-breaks, and review-required states |
| PROV-01 | Every read/write/retrieval result remains explainable from persisted source refs, events, and operation history | Persist `IntegrationSourceRef`, write-back records, semantic provenance, and conflict events |
| RECON-01 | Cross-client and upstream reconciliation follows one deterministic ordering rule | `NodeIdentity`, ordering primitive, and replayable merge/conflict logic from the re-scoped Phase 2 follow-on |
| TODO-01 | Todoist label syntax remains compatibility-only while Vel uses typed internal scheduling/write-back fields | Todoist adapter boundary translates labels to typed fields; write-back never treats labels as core state |
| NOTES-01 | Notes and transcripts become one coherent write/read/recall lane rooted in explicit note scopes | Project-linked note roots, transcript-under-notes source subtype, and semantic indexing over notes artifacts |
| REMIND-01 | Reminders support safe create/update/complete with inspectable execution/result tracking | Typed reminder intents plus mediated execution/result path |
| GH-01 | GitHub becomes a bounded project-linked integration lane with provenance and safe writes | New connector/service slice limited to issue/comment/state operations and project/upstream refs |
| EMAIL-01 | Email becomes a bounded communication lane with operator-safe reply handling | Draft-first email write path and explicit send confirmation boundary |
| PEOPLE-01 | Phase 6 ships a practical minimal people registry | Typed person and alias records with inspectable links to commitments, messages, and projects |
| PEOPLE-02 | People identity participates in recall, intervention, and cross-source linkage without opaque heuristics | Alias-driven linking, semantic graph edges, and typed people refs in integration and operator outputs |

</phase_requirements>

## Summary

Phase 6 should be planned as one contract-and-reconciliation foundation followed by bounded provider slices. The current tree already has the right backbone for this: typed projects and action items from Phase 5, canonical integration family/provider types in `vel-core`, integration connection persistence in `vel-storage`, semantic retrieval infrastructure in `semantic_memory_repo`, and thin-client continuity in `client_sync`. The missing pieces are a typed write-back operation model, a conflict queue, deterministic origin ordering, people records, and new provider lanes for GitHub/email.

The most important architectural decision is to keep write-back policy backend-owned and integration-specific. The repo already warns against provider sprawl and raw broad credentials. That means Phase 6 should not add a generic `execute provider request` abstraction for integrations. Instead, each provider slice should be a bounded service that maps a typed operation contract to a small allowed set of upstream actions, emits stable events, and defers ambiguous cases to a conflict/review queue.

The second major tension is reminder and email execution. The Rust runtime does not currently own native system reminder APIs, and email sending is inherently higher risk than task or note edits. The safest plan is: notes write server-side within configured roots; reminders use typed intents plus explicit execution/result tracking so local executors or linked clients can participate; email starts as draft-first and confirm-required for actual send.

The third tension is semantic graph breadth. The shipped semantic baseline already uses a local hybrid retrieval seam and stable provenance for captures. Phase 6 should widen that system only to the entities it is already making durable: projects, notes, transcripts, message threads, GitHub artifacts, and people. Do not broaden into speculative graph modeling beyond those typed records.

**Primary recommendation:** Plan Phase 6 as seven slices:
1. contract/doc/schema foundations for write-back, conflicts, and people
2. deterministic ordering plus storage/service conflict foundation
3. Todoist typed write-back and label-boundary closure
4. notes/reminders/transcripts write lane
5. people registry plus graph expansion over local/project/message entities
6. GitHub and email provider slices
7. operator-visible status/conflict/history/docs closure

## Standard Stack

Versions below are the checked-in repo baseline on 2026-03-18, not registry-latest lookups.

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-core` | `0.1.0` | Domain types, write-back/conflict/people contracts | Phase 6 needs typed operation and identity records here first |
| `vel-storage` | `0.1.0` | SQLite migrations and repositories | Conflict queue, people records, upstream refs, and write history belong here |
| `vel-api-types` | `0.1.0` | Transport DTOs | Clients should consume typed conflict/write-back/people payloads, not JSON blobs |
| `veld` | `0.1.0` | Services, routes, orchestration | Existing integration, sync, operator queue, and semantic seams already live here |
| `axum` | `0.7` | HTTP routing | Existing thin-route boundary stays valid |
| `sqlx` | `0.8` | SQLite access and migrations | Repository/migration standard already established |
| `tokio` | `1.44` | Async runtime | Current services and background loops run here |
| `serde` / `serde_json` | `1.0` / `1.0` | Typed serialization | Required for typed operation/event/provenance payloads |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `reqwest` | `0.12` | Provider HTTP clients | Existing Google/Todoist pattern for GitHub/email provider slices |
| `react` / `react-dom` | `19.2.4` | Operator web shell | Surface conflict queue, write-back history, and provider status after backend slices land |
| `typescript` | `5.9.3` | Web DTO parity | Needed for conflict/write-back/people transport updates |
| `vitest` | `2.1.x` | Web tests | Use for settings/status/history and typed-decoder coverage |
| Swift + `VelAPI` | Swift 5.9 baseline | Thin-client and local-executor boundary | Only if reminder execution or write-intent hydration needs linked-client support; keep policy in Rust |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Typed provider-specific write ops | Generic "provider request" executor | Faster at first, but violates capability clarity and becomes impossible to reason about safely |
| Conflict queue records | Silent last-write-wins | Simpler superficially, but breaks trust, replayability, and operator reviewability |
| Alias-driven people linking | Auto-merge heuristics across providers | Lower manual effort, but too opaque for the current trust bar |
| Server-side notes writes + queued reminder intents | Broad client-owned write policy | Faster UI iteration, but violates backend-owned policy and continuity discipline |

**Installation / verification examples:**
```bash
cargo test -p vel-core integration -- --nocapture
cargo test -p vel-storage semantic_memory_repo integration_connections_repo -- --nocapture
cargo test -p veld integrations client_sync now -- --nocapture
```

## Architecture Patterns

### Pattern 1: Typed operation record before provider write logic
**What:** Add a provider-neutral write-back operation record that captures intent, target, status, review requirement, and result refs.
**When to use:** Any provider write path in this phase.
**Why:** Prevents ad hoc provider-specific status handling and gives conflict/replay/provenance one place to anchor.

### Pattern 2: Conflict queue feeds the existing operator queue
**What:** Persist conflict cases and project them into `ActionItem`/`Inbox`/status surfaces instead of inventing a new parallel queue.
**When to use:** Local-vs-upstream divergence, cross-client write collisions, stale-write denial, executor-unavailable cases.
**Why:** Phase 5 already established the operator-facing action/intervention seam.

### Pattern 3: Connector contract stays authoritative at the provider boundary
**What:** Map new provider lanes through `IntegrationFamily`, `IntegrationProvider`, `IntegrationConnection`, and `IntegrationSourceRef`.
**When to use:** GitHub/email additions and Todoist write-back closure.
**Why:** The repo already has the canonical vocabulary and explicitly rejects provider-sprawl contracts.

### Pattern 4: Semantic graph expansion follows durable typed records
**What:** Index and link only entities that already have durable typed records or durable upstream refs.
**When to use:** Projects, notes, transcripts, message threads, people, GitHub artifacts.
**Why:** Keeps retrieval explainable and replayable; avoids graph edges with no durable anchor.

### Pattern 5: Thin clients display status and execute narrow local actions only
**What:** Web/CLI/Apple render typed status, pending conflicts, and write history; local executors only handle the narrow action they are explicitly allowed to execute.
**When to use:** Reminders/email execution where the authority process cannot or should not act directly.
**Why:** Preserves the repo’s backend-owned policy model while keeping local-first execution possible.

## Validation Architecture

Phase 6 validation should be wave-gated:

- **Wave 0:** contract and repository tests for new core types, ordering primitive, conflict/write-back repos, people repos, and semantic graph records
- **Wave 1+:** provider-specific focused service tests (Todoist, notes, reminders, GitHub, email)
- **After each provider slice:** route/integration tests proving allowed operations, denied operations, and conflict path persistence
- **Before close-out:** web/CLI decoder and status-surface tests for conflict queue, write-back history, and typed people/project linkage

The highest-risk regressions are:

1. silent overwrite of upstream state
2. untyped JSON growth in operator payloads
3. provider slices widening beyond the allowed operation set
4. people or graph identity edges without durable provenance

Validation must therefore prove not only that writes succeed, but also that denied or conflicted writes persist inspectable state and leave source-of-truth boundaries intact.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Provider write policy | Free-form provider payload passthrough | Typed operation kinds and provider-specific service methods | Prevents accidental broad capability scope |
| Conflict visibility | Logs or warning strings only | Durable conflict records plus action/intervention projection | Trust depends on inspectable unresolved cases |
| People identity | Heuristic-only merge engine | Person records + alias table + explicit source links | Keeps identity explainable and reversible |
| GitHub/email linkage | Provider-specific project slugs in metadata only | Typed project upstream refs and person aliases | Phase 5 already created the project substrate |
| Transcript handling | Separate permanent transcript surface | Notes source subtype plus shared retrieval/indexing path | Matches Phase 6 state decision and reduces surface sprawl |

## Common Pitfalls

### Pitfall 1: Turning write-back into a hidden side effect
**What goes wrong:** A service silently updates upstream records during sync or ranking work.
**Why it happens:** Existing sync services already talk to providers, so it feels convenient to reuse them for writes.
**How to avoid:** Separate read/sync execution from explicit write-back operation services and durable operation records.
**Warning signs:** A sync route starts mutating remote state or no operation history table exists.

### Pitfall 2: Using client UI state as the conflict source of truth
**What goes wrong:** Web or Apple tracks unresolved conflicts locally and different clients disagree about what needs review.
**Why it happens:** Conflict prompts often start as UI concerns.
**How to avoid:** Persist conflict cases in backend storage and project them through existing action/intervention DTOs.
**Warning signs:** Conflict state only appears in component-local state or offline cache.

### Pitfall 3: Letting provider metadata become the core model
**What goes wrong:** Todoist labels, GitHub labels, or email folder names become the durable Vel contract.
**Why it happens:** Provider metadata already exists and is easy to parse.
**How to avoid:** Translate provider syntax at adapter boundaries and store typed internal fields.
**Warning signs:** New domain APIs expose label arrays or provider field names as their primary contract.

### Pitfall 4: Widening graph scope before durable anchors exist
**What goes wrong:** Semantic edges are created for entities that cannot be replayed, inspected, or reconciled later.
**Why it happens:** Graph work encourages broad linking.
**How to avoid:** Only index/link entities with durable IDs, typed records, and provenance refs.
**Warning signs:** Retrieval hits lack stable `source_id`/upstream refs or cannot point back to persisted records.

### Pitfall 5: Treating reminders/email like low-risk task writes
**What goes wrong:** High-risk human-facing actions auto-execute without proper review or execution boundary checks.
**Why it happens:** The operator wants convenience and the implementation path looks similar to Todoist.
**How to avoid:** Use queued intents, draft-first behavior, and explicit confirm-required states for riskier operations.
**Warning signs:** Email send appears in the same safe bucket as note append or task completion.

