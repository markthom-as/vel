# Phase 5: now-inbox-core-and-project-substrate - Research

**Researched:** 2026-03-18
**Domain:** Rust-backed operator projections, typed project substrate, and cross-client continuity
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)
- Safe autonomous write-back and upstream conflict resolution beyond the new-project confirmation flow - Phase 6
- Richer semantic project/people/notes graph expansion - Phase 6
- Apple-first quick loops, voice capture emphasis, and behavioral signal productization - Phase 7
- Coding-agent project execution context, GSD handoff policy, direct WASM guest follow-on, and external connect transport closure - Phase 8
- Backup/export trust surfaces and broader operator control work - Phase 9
</user_constraints>

<phase_requirements>
## Phase Requirements

Phase 5 requirement IDs are present in `.planning/ROADMAP.md`, but `.planning/REQUIREMENTS.md` has not been expanded with Phase 5 rows yet. Descriptions below are inferred from the roadmap and `05-CONTEXT.md`.

| ID | Description | Research Support |
|----|-------------|-----------------|
| NOW-01 | `Now` stays the primary decision cockpit for next work and intervention awareness | Backend-owned action/intervention projection pattern; extend `/v1/now` instead of adding client logic |
| NOW-02 | `Now` surfaces a small ranked set of salience-ordered actions/interventions with explainability | Single service-layer ranking vocabulary with evidence refs and DTO mapping |
| INBOX-01 | `Inbox` is the triage queue for unresolved intervention items and newly surfaced work | Rework inbox around backend triage records, not messages |
| INBOX-02 | `Inbox` supports explicit acknowledgement/triage flows without becoming a generic thread list | Preserve `/api/inbox` intervention posture and add typed actions/state transitions |
| ACTION-01 | A unified backend action/intervention model anchors nudges, suggestions, conflicts, and commitments | Introduce typed domain records or typed projection over existing evidence sources |
| REVIEW-01 | Daily and weekly review loops reinforce `Now`/`Inbox`/Projects coordination | Reuse current review/synthesis cadence; add project-aware weekly review hooks |
| CONTINUITY-01 | Web, CLI, and Apple share one coherent persisted action/intervention/project state | Extend `sync/bootstrap` and DTOs from Rust backend outward |
| CONTINUITY-02 | Guided node linking is trustworthy, scoped, inspectable, and operable through CLI or web | Pairing token + trust visibility + status surfaces should sit on existing cluster/bootstrap seams |
| PROJ-01 | Projects become typed local-first records rather than loose strings | New `vel-core` types, storage tables, and migration path from `commitment.project` |
| PROJ-02 | Each project carries family, primary repo, primary notes root, and optional secondary links/upstream IDs | Typed project schema and boundary DTOs |
| PROJ-03 | New-project setup is local-first and operator-confirmed for upstream side effects | Workflow should separate local record creation from optional upstream provisioning |
| FAMILY-01 | `Personal`, `Creative`, and `Work` are stable project families, not projects | Use explicit enum/vocabulary and shared DTOs across clients |
</phase_requirements>

## Summary

Phase 5 should be planned as a backend-contract phase first and a UI phase second. The codebase already has the right shape for that: `Now` is a dedicated Rust service and route, `Inbox` is a chat/intervention-backed queue, Apple clients hydrate a shared bootstrap payload and queue low-risk actions offline, and the web shell already treats `Projects` as intentionally pending. The missing piece is not "add another screen"; it is "publish a typed project/action/intervention contract that every surface can consume without inventing local policy."

The main architectural tension is migration. Commitments, `Now`, Apple offline cache, web types, and sync bootstrap still treat `project` as `Option<String>`, while phase decisions require typed projects, project families, and unified action/intervention semantics. There is also no existing project table or project repository in the live tree to extend, so this must start as net-new domain and storage work rather than a UI-only pass. Plan this as a staged contract expansion: define new core/storage types first, project them through `veld` services and DTOs, then update web/CLI/Apple in the same slice. Do not let clients derive ranking, triage, or continuity rules on their own.

Linked-node closure also belongs here because the live tree has bootstrap and worker visibility but no real pairing flow. Reuse the cluster/bootstrap/auth seams and add short-lived scoped pairing tokens, explicit trust visibility, and status inspection. Do not widen this into external connect transport or broad execution permissions; keep it local, scoped, and fail-closed.

**Primary recommendation:** Plan Phase 5 around three contract-first lanes: typed project substrate, unified backend action/intervention projection, and bootstrap/pairing continuity closure.

## Standard Stack

Versions below are the repo baseline verified from checked-in manifests on 2026-03-18, not registry-latest lookups.

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-core` | `0.1.0` | Domain types and invariants | Phase 5 needs new typed project/family/action contracts here first |
| `vel-storage` | `0.1.0` | SQLite repositories and migrations | Existing repository pattern is the right seam for project/link persistence |
| `vel-api-types` | `0.1.0` | Transport DTOs | Clients already consume shared DTOs; Phase 5 should extend them, not fork them |
| `veld` | `0.1.0` | Services, routes, projections | `Now`, sync bootstrap, chat inbox, and cluster bootstrap already live here |
| `axum` | `0.7` | HTTP routing | Existing thin-route pattern is established and tested |
| `sqlx` | `0.8` | SQLite access and migrations | Standard persistence layer already used across repositories |
| `tokio` | `1.44` | Async runtime | Existing daemon/services run on Tokio; no reason to introduce another runtime |
| `serde` / `serde_json` | `1.0` / `1.0` | Typed serialization | Required to keep project/action DTOs typed and explainable |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `react` / `react-dom` | `19.2.4` | Web operator shell | Extend existing `Now`, `Inbox`, and `Projects` shell once backend DTOs land |
| `typescript` | `5.9.3` | Web type enforcement | Use for DTO parity and decoder updates |
| `vite` | `8.0.0` | Web dev/build | Existing web app baseline |
| `vitest` | `2.1.x` | Web component/data tests | Use for `Now`/`Inbox`/Projects contract tests |
| Swift + `VelAPI` | Swift 5.9 baseline | Apple thin client models/cache | Extend bootstrap/models/offline cache for continuity; keep policy in Rust |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Backend-owned action/intervention projection | Client-side ranking in React/Swift | Faster locally, but violates thin-client rules and creates continuity drift |
| New typed project records in `vel-core` + `vel-storage` | Keep extending `commitment.project: Option<String>` | Simpler short-term, but blocks project families, linked roots, and stable IDs |
| Extend existing bootstrap contracts | Per-client local state models | Increases divergence; Apple already shows the cost of contract skew |

**Installation:**
```bash
cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture
npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/InboxView.test.tsx
```

**Version verification:** Verified against `Cargo.toml`, crate manifests, and `clients/web/package.json`. No external registry lookup was performed because this phase should stay on the repository's existing stack.

## Architecture Patterns

### Recommended Project Structure
```text
crates/vel-core/src/
├── project.rs            # Typed project/family/action domain contracts
├── commitment.rs         # Legacy compatibility touches only
└── link.rs               # Pairing/link scope contracts if introduced

crates/vel-storage/src/repositories/
├── projects_repo.rs      # Project CRUD + family/project lookup
├── action_queue_repo.rs  # If persisted queue chosen
└── link_tokens_repo.rs   # Pairing token + link status persistence

crates/veld/src/services/
├── now.rs                # Extend projection to ranked action/intervention summary
├── inbox.rs              # New service layer if inbox stops being chat-only
├── projects.rs           # Project registry/workflow orchestration
├── client_sync.rs        # Extend bootstrap with project/action continuity
└── linking.rs            # Pairing issue/redeem/status orchestration

crates/veld/src/routes/
├── now.rs
├── projects.rs
├── sync.rs
└── cluster.rs / linking.rs

clients/web/src/
├── data/
├── components/
└── types.ts

clients/apple/VelAPI/Sources/VelAPI/
├── Models.swift
├── VelClient.swift
└── OfflineStore.swift
```

### Pattern 1: Domain -> Storage -> Service -> DTO -> Client
**What:** Add new typed project/action/linking contracts in `vel-core`, persist them via `vel-storage`, synthesize projections in `veld` services, then map to DTOs at routes.
**When to use:** Any new project, family, action/intervention, or link-status surface.
**Example:**
```rust
// Source: crates/veld/src/routes/now.rs
pub async fn get_now(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<NowData>>, AppError> {
    let data = services::now::get_now(&state.storage, &state.config).await?;
    Ok(response::success(data.into()))
}
```

### Pattern 2: Projection Surfaces, Not Source-of-Truth Surfaces
**What:** `Now` and `Inbox` should remain projections over persisted commitments, interventions, sync state, and later project records.
**When to use:** Ranking or composing cross-domain operator views.
**Example:**
```rust
// Source: crates/veld/src/services/client_sync.rs
let commitments = state
    .storage
    .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
    .await?;
```

### Pattern 3: Bootstrap-Based Multi-Client Continuity
**What:** Keep web/CLI/Apple continuity centered on shared bootstrap payloads and queued low-risk actions.
**When to use:** Any state that must survive offline or move between clients.
**Example:**
```swift
// Source: clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift
public func hydrate(from bootstrap: SyncBootstrapData) {
    if let currentContext = bootstrap.current_context {
        saveCachedContext(currentContext)
    }
    saveCachedNudges(bootstrap.nudges)
    saveCachedCommitments(bootstrap.commitments)
}
```

### Pattern 4: Compatibility-First Migration
**What:** Keep legacy `project: Option<String>` readable while introducing typed project IDs and richer project DTOs.
**When to use:** All slices that touch commitments, `Now`, sync bootstrap, Apple cache, and web decoders.
**Example:**
```rust
// Source: crates/vel-core/src/commitment.rs
pub struct Commitment {
    pub id: CommitmentId,
    pub text: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: CommitmentStatus,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub created_at: OffsetDateTime,
    pub resolved_at: Option<OffsetDateTime>,
    pub metadata_json: serde_json::Value,
}
```

### Anti-Patterns to Avoid
- **Client-owned ranking:** Do not rank or fuse actions/interventions in React or Swift.
- **String-only project evolution:** Do not deepen `commitment.project` into the permanent project model.
- **Parallel queue sprawl:** Do not add another standalone queue beside nudges, suggestions, interventions, and commitments.
- **Untyped bootstrap growth:** Do not append opaque JSON blobs to sync/bootstrap for project or action state.
- **Linking via raw credentials:** Do not hand long-lived secrets to clients for pairing; use short-lived scoped tokens.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Cross-client continuity | Per-client caches with custom semantics | Extend `/v1/sync/bootstrap` + existing Apple queued action lane | Existing clients already rely on shared bootstrap semantics |
| Action/intervention explainability | Ad hoc UI copy detached from evidence | Backend projection with persisted evidence refs | Phase decisions require traceability to commitments, sync state, runs, or policy |
| Project identity | Free-form strings or slugs as the only contract | Typed project IDs plus backward-compatible display fields | Families, repos, notes roots, and upstream IDs need a stable typed anchor |
| Linking security | Broad reusable auth tokens | Short-lived scoped pairing tokens + explicit trust scopes | Matches repo security rules and the re-scoped onboarding ticket |
| Websocket sync logic | New client-only invalidation system | Existing query-cache and ws-sync pattern | Current `Inbox`/`Now` already use it and tests exist |

**Key insight:** Phase 5 is mostly about consolidating already-existing seams. The expensive mistakes here are custom local shortcuts that bypass those seams.

## Common Pitfalls

### Pitfall 1: Treating `project` string as the project substrate
**What goes wrong:** Phase 5 ends up with display-only project labels but no durable project identity, family model, or linked roots.
**Why it happens:** `Commitment.project` already exists, so it looks cheaper to reuse forever.
**How to avoid:** Add a real project domain type and storage table first; keep string project only as migration compatibility.
**Warning signs:** New APIs still only expose `project: string | null`; no stable project IDs exist.

### Pitfall 2: Letting clients fork contract shape
**What goes wrong:** Web, CLI, and Apple disagree on what continuity/bootstrap contains.
**Why it happens:** One client updates faster than shared DTOs.
**How to avoid:** Update Rust DTOs and affected client boundary code in the same slice.
**Warning signs:** Current repo already shows this: Apple bootstrap models include `capabilities`, `branch_sync`, and `validation_profiles`; the web `ClusterBootstrapData` decoder still ignores them.

### Pitfall 3: Making `Inbox` a generic message list
**What goes wrong:** `Inbox` duplicates Threads and loses triage semantics.
**Why it happens:** The current route lives in `chat.rs`, which invites chat-centric expansion.
**How to avoid:** Keep `Inbox` typed as operator triage state, even if backed by interventions/messages.
**Warning signs:** Planning language starts talking about "messages in Inbox" rather than interventions needing acknowledgement.

### Pitfall 4: Hiding ranking logic in UI composition
**What goes wrong:** `Now` works differently on web, CLI, and Apple.
**Why it happens:** UI components are tempting places to rank small lists.
**How to avoid:** Put ranking and selection in Rust services and deliver one typed projection.
**Warning signs:** React or Swift starts sorting by urgency/confidence locally.

### Pitfall 5: Pairing without explicit scope and status
**What goes wrong:** Linked-node setup is hard to trust or diagnose.
**Why it happens:** Existing bootstrap visibility is mistaken for a complete onboarding flow.
**How to avoid:** Ship issue/redeem/status flows together with TTL, scope, revocation, and trust text.
**Warning signs:** No persisted pairing status exists; no CLI/web fallback path exists.

### Pitfall 6: Forgetting offline continuity when extending bootstrap
**What goes wrong:** Apple can create/read state that does not round-trip cleanly after reconnect.
**Why it happens:** Bootstrap adds fields, but `OfflineStore` and queued action contracts are not updated.
**How to avoid:** Treat Apple `Models.swift`, `VelClient.swift`, and `OfflineStore.swift` as first-class contract consumers.
**Warning signs:** New project/action state appears in web only; Apple caches silently drop it.

## Code Examples

Verified patterns from local authoritative sources:

### Thin route mapping
```rust
// Source: crates/veld/src/routes/now.rs
impl From<services::now::NowTaskOutput> for NowTaskData {
    fn from(value: services::now::NowTaskOutput) -> Self {
        Self {
            id: value.id,
            text: value.text,
            source_type: value.source_type,
            due_at: value.due_at,
            project: value.project,
            commitment_kind: value.commitment_kind,
        }
    }
}
```

### Shared bootstrap hydration
```swift
// Source: clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift
public func queuedActionRequests() -> [SyncActionRequestData] {
    queuedActions().map { action in
        SyncActionRequestData(
            action_id: action.id.uuidString,
            action_type: action.kind.rawValue.replacingOccurrences(of: ".", with: "_"),
            target_id: action.targetID,
            text: action.text,
            minutes: action.minutes
        )
    }
}
```

### Existing inbox projection seam
```rust
// Source: crates/veld/src/services/chat/reads.rs
pub(crate) async fn list_inbox_items(
    state: &AppState,
    limit: u32,
) -> Result<Vec<InboxItem>, AppError> {
    let list = state.storage.list_interventions_active(limit).await?;
    Ok(list
        .into_iter()
        .map(intervention_record_to_inbox_output)
        .collect())
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Client shells own local interpretation | Rust backend owns policy and clients stay thin | Phase 1 architecture baseline | Phase 5 should deepen shared contracts, not client logic |
| One-off dashboard surfaces | Explicit operator projections like `/v1/now` and `/api/inbox` | Phases 1-4 shipped baseline | Extend projections instead of inventing new parallel views |
| Loose project labels | Typed project substrate with migration compatibility | Phase 5 target | Requires new domain/storage/DTO work before serious UI |
| Bootstrap as context/nudges/commitments only | Bootstrap as broader continuity contract | Phase 5 target | Projects and action/intervention state should ride this seam |
| Diagnostics-only linked-node visibility | Guided scoped pairing with trust/status workflow | Re-scoped from Phase 2 to Phase 5 | Requires new persistence and operator flows |

**Deprecated/outdated:**
- `project: Option<String>` as the main project contract: keep only as compatibility.
- `Projects` as dashboard-first primary shell: Phase 5 decisions reject that.
- Client-specific continuity rules: conflicts with the accepted multi-client product direction.

## Open Questions

1. **Should the unified action/intervention model be persisted as its own table or projected on demand?**
   - What we know: Context allows either a persisted queue, a projection over typed records, or a hybrid.
   - What's unclear: Expected lifecycle/history requirements for triage actions versus source evidence.
   - Recommendation: Start with a service-layer projection unless triage-specific state transitions require independent persistence.

2. **Where should pairing token and link status persistence live?**
   - What we know: No live pairing flow or obvious project/link table exists.
   - What's unclear: Whether link status belongs in dedicated repositories or existing cluster worker/state storage.
   - Recommendation: Use dedicated persistence for pairing tokens and link state; do not overload worker heartbeat records.

3. **How far should Phase 5 go on project-aware synthesis/review?**
   - What we know: Weekly/project synthesis exists, and daily/weekly review loops are part of the accepted flow.
   - What's unclear: Whether Phase 5 only needs project anchoring or also project-aware review surfaces.
   - Recommendation: Land project-aware data hooks and minimal review affordances now; save richer project synthesis UX for later slices if needed.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` (Rust unit/integration) + Vitest 2.1.x (`clients/web`) |
| Config file | `clients/web/vitest.config.ts`; Rust uses Cargo defaults |
| Quick run command | `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture && npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/InboxView.test.tsx` |
| Full suite command | `make verify && npm --prefix clients/web test && make check-apple-swift` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| NOW-01 | `Now` remains the primary operator projection | unit/integration | `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture` | ✅ partial |
| NOW-02 | `Now` returns ranked action/intervention data with explainability | integration | `cargo test -p veld now_ -- --nocapture` | ❌ Wave 0 |
| INBOX-01 | Inbox renders triage items, not messages | web component | `npm --prefix clients/web test -- --run src/components/InboxView.test.tsx` | ✅ partial |
| INBOX-02 | Inbox triage actions update queue state correctly | route/web integration | `cargo test -p veld chat:: -- --nocapture && npm --prefix clients/web test -- --run src/components/InboxView.test.tsx` | ✅ partial |
| ACTION-01 | Unified action/intervention vocabulary spans source evidence | service/integration | `cargo test -p veld action_ -- --nocapture` | ❌ Wave 0 |
| REVIEW-01 | Daily/weekly review loops can consume project-aware action data | integration/manual | `cargo test -p veld synthesis_ -- --nocapture` | ✅ partial |
| CONTINUITY-01 | Sync/bootstrap carries shared action/intervention/project state | integration | `cargo test -p veld sync_bootstrap_endpoint_returns_ok -- --nocapture` | ✅ partial |
| CONTINUITY-02 | Pairing issue/redeem/status flow is scoped and inspectable | integration/smoke | `cargo test -p veld pairing_ -- --nocapture` | ❌ Wave 0 |
| PROJ-01 | Typed project registry exists with stable IDs and migration compatibility | repository/unit | `cargo test -p vel-storage project_ -- --nocapture` | ❌ Wave 0 |
| PROJ-02 | Project record carries family, roots, and optional links/upstream IDs | repository/route | `cargo test -p veld projects_ -- --nocapture` | ❌ Wave 0 |
| PROJ-03 | New-project workflow requires operator confirmation for upstream effects | service/integration | `cargo test -p veld project_workflow_ -- --nocapture` | ❌ Wave 0 |
| FAMILY-01 | Project families are stable shared vocabulary across clients | unit/web/apple | `cargo test -p vel-core family_ -- --nocapture && npm --prefix clients/web test -- --run src/components/MainPanel.test.tsx` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p veld <targeted_test> -- --nocapture` and/or `npm --prefix clients/web test -- --run <targeted file>`
- **Per wave merge:** `make verify && npm --prefix clients/web test`
- **Phase gate:** `make verify && npm --prefix clients/web test && make check-apple-swift`

### Wave 0 Gaps
- [ ] `crates/vel-core/src/project.rs` tests for family/project ID and schema invariants
- [ ] `crates/vel-storage/src/repositories/projects_repo.rs` tests for CRUD, lookup, and migration reads from legacy strings
- [ ] `crates/veld/src/services/projects.rs` tests for local-first project workflow and confirmation gates
- [ ] `crates/veld/src/services/linking.rs` or equivalent tests for token TTL/scope/redeem/revoke
- [ ] `crates/veld/src/routes/projects.rs` tests for DTO parity across create/read/update flows
- [ ] `clients/web/src/components/ProjectsView.test.tsx` for the first real Projects surface
- [ ] `clients/web/src/components/NowView.test.tsx` additions covering action/intervention ranking and project labels
- [ ] `clients/web/src/components/InboxView.test.tsx` additions covering triage actions beyond realtime refresh
- [ ] Apple contract checks for new bootstrap/project models; no checked-in Apple test target was found, so use `make check-apple-swift` plus manual simulator validation

## Sources

### Primary (HIGH confidence)
- `./CLAUDE.md` - project-specific workflow, layering, and verification rules
- `./docs/MASTER_PLAN.md` - canonical phase status and re-scope authority
- `./.planning/ROADMAP.md` - Phase 5 goal, requirement IDs, and scope boundaries
- `./.planning/STATE.md` - recent decisions, blockers, and re-scoped continuity gaps
- `./.planning/phases/05-now-inbox-core-and-project-substrate/05-CONTEXT.md` - locked product decisions for this phase
- `./docs/user/surfaces.md` - shipped meanings of `Now`, commitments, nudges, and inspectability
- `./docs/user/daily-use.md` - daily/weekly operator loop to preserve
- `./docs/api/runtime.md` - live route inventory, auth classes, `/v1/now`, and sync/bootstrap surfaces
- `./docs/tickets/phase-2/012-tester-readiness-onboarding.md` - pairing/linking requirements re-scoped into Phase 5
- `./crates/veld/src/services/now.rs` - current `Now` projection seam
- `./crates/veld/src/services/client_sync.rs` - sync/bootstrap and cluster continuity seam
- `./crates/veld/src/routes/now.rs` - thin route/DTO mapping pattern
- `./crates/veld/src/routes/chat.rs` and `./crates/veld/src/services/chat/reads.rs` - current inbox/intervention seam
- `./crates/vel-core/src/commitment.rs` and `./crates/vel-storage/src/repositories/commitments_repo.rs` - legacy project-string compatibility seam
- `./clients/web/src/components/NowView.tsx` and `./clients/web/src/components/InboxView.tsx` - existing primary web operator surfaces
- `./clients/web/src/components/MainPanel.tsx` and `./clients/web/src/components/Sidebar.tsx` - current shell hierarchy and Projects placeholder
- `./clients/apple/README.md`, `./clients/apple/VelAPI/Sources/VelAPI/Models.swift`, `./clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift` - Apple thin-client continuity contract

### Secondary (MEDIUM confidence)
- `./clients/web/src/types.ts` and `./clients/web/src/data/operator.ts` - useful for identifying current web contract drift
- `./crates/veld/src/app.rs` tests - confirms sync bootstrap route is mounted and tested only at a shallow level

### Tertiary (LOW confidence)
- None. This research was dominated by repository-specific authority and live code, so no external unverified sources were needed.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - existing repo stack is explicit in manifests and already exercised
- Architecture: HIGH - route/service/bootstrap/client seams are visible in shipped code and authority docs
- Pitfalls: MEDIUM - most are strongly supported by current drift, but the exact unified action/pairing shape is still a planning choice

**Research date:** 2026-03-18
**Valid until:** 2026-04-17
