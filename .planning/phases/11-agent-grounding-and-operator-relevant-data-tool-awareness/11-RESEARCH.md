# Phase 11: Agent grounding and operator-relevant data/tool awareness - Research

**Researched:** 2026-03-19
**Domain:** Backend-owned agent grounding over existing Vel state, review queues, and bounded capability seams
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Product intent
- [locked] This phase exists because agent awareness of Vel data and operator-relevant tool access is important enough to be committed roadmap work, not backlog-only future work.
- [locked] Agents should be grounded in real Vel state: current context, projects, people, commitments, review queues, execution handoffs, and bounded tool affordances.
- [locked] Grounding must improve operator-trustworthy action, not create a second opaque assistant layer.

### Trust and review model
- [auto] Agent-visible context must remain inspectable and traceable back to persisted Vel records or explicit execution context packs.
- [auto] Tool affordances should remain bounded and operator-visible. Do not give agents ambient access to everything Vel can see or mutate.
- [auto] Review gates, handoff state, and SAFE MODE/writeback constraints must remain intact. Agent grounding should not bypass the review model shipped in earlier phases.
- [auto] Unknown or unsupported tool/data requests should fail closed.

### Data grounding scope
- [auto] The minimum useful grounding bundle should consider:
  - `Now` state and current context
  - projects and project review candidates
  - people records and people-needing-review signals
  - commitments and commitment-linked project state
  - execution context and execution handoffs
  - pending writebacks, open conflicts, and other operator review obligations
- [auto] Grounding should prefer typed summaries and references over raw unbounded dumps.
- [auto] Repo-local coding context from Phase 08 should remain one grounding input, not the whole product.

### Tool-awareness scope
- [auto] Tool access should be described in operator-relevant terms, not just low-level runtime/internal names.
- [auto] The system should distinguish:
  - read-only context access
  - bounded review/inspection actions
  - bounded mutation affordances still subject to existing approval or SAFE MODE constraints
- [auto] If an agent lacks a required data/tool grant, the operator should be able to see why and what narrow escalation would be needed.

### Client/surface discipline
- [auto] Backend Rust layers own grounding policy, summarization boundaries, and capability decisions.
- [auto] Web/CLI/operator surfaces should expose what the agent can currently see and do, plus why.
- [auto] This phase may add operator-surface visibility for agent grounding, but broad shell/navigation cleanup remains Phase 12.

### Claude's Discretion
- Exact contract/type names for grounded context packs, tool descriptors, review-bound capability summaries, and operator-facing inspect surfaces
- Whether the first shipped grounding product surface is CLI-first, runtime/API-first, web-first, or a thin combination
- How much grounding is pushed through existing execution-context/handoff seams versus a new agent-facing summary seam, provided layering and trust rules are preserved

### Deferred Ideas (OUT OF SCOPE)
- Broad shell/nav/docs/onboarding cleanup — Phase 12
- New provider families, hosted auth scaffolding, and broad platform expansion — backlog/later milestone
- Full autonomy or unsupervised agent behavior that widens beyond explicit handoff/review boundaries
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| AGENT-CTX-01 | Ship one typed grounding bundle that covers `Now`, projects, people, commitments, review obligations, and execution handoffs. | New `agent_grounding` service should assemble from existing `Now`, projects, commitments, people, and handoff seams instead of introducing a second state model. |
| AGENT-CTX-02 | Keep grounding inspectable, traceable, and bounded to persisted records plus explicit explain/reference fields. | Use `Now` as the primary summary contract and attach source IDs, freshness, explain refs, and review refs rather than raw dumps. |
| AGENT-TOOLS-01 | Expose bounded tool awareness in operator terms, grouped by read, review, and mutation classes. | Reuse Phase 08 `read_scopes`, `write_scopes`, `allowed_tools`, `capability_scope`, connect capability allowlists, and SAFE MODE gating. |
| AGENT-TOOLS-02 | Missing or unsupported grants must fail closed and show the narrow escalation path. | Capability summary should surface `available`, `blocked_reason`, `requires_review_gate`, and `requires_writeback_enabled` fields. |
| AGENT-REVIEW-01 | Preserve existing review queues and operator approval paths while making them agent-relevant. | Reuse `operator_queue`, pending execution handoffs, writeback/conflict queues, and existing web/CLI review surfaces. |
| AGENT-TRUST-01 | Operator surfaces must show what the agent can currently see and do, plus why. | Start runtime/API-first, then expose the same DTO through Settings and CLI inspect views so policy and UI never diverge. |
</phase_requirements>

## Summary

The repo already has nearly all of the raw data this phase needs. `GET /v1/now` is a typed operator summary with ranked `action_items`, `review_snapshot`, `pending_writebacks`, `conflicts`, `people`, freshness, reasons, and debug provenance. Projects, commitments, people, current context, explainability, execution context, execution handoffs, connect capability allowlists, and SAFE MODE/writeback state are all already shipped as separate seams. The missing product layer is not more data collection. It is a backend-owned agent grounding contract that packages these existing records into one inspectable, operator-visible bundle.

The strongest reusable policy seam is Phase 08 execution routing. Handoffs already require explicit `read_scopes`, `write_scopes`, `allowed_tools`, `expected_output_schema`, and review gates, and pending handoffs already appear in `Now`, CLI review output, and Settings. The strongest reusable review seam is `operator_queue`, which already turns pending handoffs, writebacks, conflicts, freshness issues, interventions, and project review into ranked action items. Phase 11 should extend those seams to non-coding operator work instead of introducing a second agent-permission model.

The clearest gap is the current assistant path: `crates/veld/src/services/chat/assistant.rs` still sends only conversation history plus a generic system prompt and `tools: vec![]`. That confirms the right Phase 11 shape: do not widen chat first. Add a typed runtime/API grounding + capability summary seam first, then let any future agent/chat surface consume that same contract.

**Primary recommendation:** Add a new backend-owned `agent_grounding` service and typed `AgentInspectData` contract that assembles existing `Now`/project/people/commitment/review/handoff state plus operator-visible capability summaries, then expose it through a thin API and existing Settings/CLI review surfaces.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-core` | workspace `0.1.0` | Domain vocabulary for actions, review, people, projects, capabilities, runs, and execution | Keeps new grounding semantics out of transport and storage layers |
| `vel-storage` | workspace `0.1.0` | Durable access to current context, projects, commitments, people, writebacks, conflicts, and handoffs | Already owns every persisted record Phase 11 needs |
| `veld` services/routes | workspace `0.1.0` | Grounding assembly, auth, policy, review-gate enforcement, and operator-authenticated endpoints | Repo rules require backend ownership for policy and summarization |
| `vel-api-types` | workspace `0.1.0` | Transport DTOs for grounding and capability inspection | Preserves existing route/service/DTO layering |
| `react` | `19.2.4` | Existing operator shell for inspect/review visibility | Already ships the surfaces that show `Now`, Settings review, and trust posture |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `axum` | `0.7` (workspace) | Authenticated runtime/API route layer | New operator-authenticated inspect endpoint(s) |
| `sqlx` | `0.8` (workspace) | SQLite repository access | Read existing typed records; avoid new cache tables in the first slice |
| `tokio` | `1.44` (workspace) | Async service/runtime execution | New service composition and integration tests |
| `vitest` | installed `2.1.9` (`package.json` range `^2.1.8`) | Web/operator surface verification | Settings and `Now` inspect-state tests |
| Config schemas/examples | current repo assets | Durable contract publication | New grounding/capability DTO schemas and examples once the boundary stabilizes |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| New `agent_grounding` service over existing seams | Extend `/api/chat` directly | Faster to demo, but it bakes policy into an already ungrounded assistant path and hides operator trust data |
| Typed grounding bundle rooted in `Now` plus supporting typed records | Raw `current_context.context` JSON dump | Easier short term, but violates the "typed summaries over unbounded dumps" constraint and weakens explainability |
| Phase 08 routing/capability vocabulary reused across product work | New agent-specific permission model | More names, more drift, and duplicated review logic |
| Runtime/API-first with thin Settings + CLI inspect | Web-first bespoke UI contract | Creates frontend-owned policy pressure and increases drift risk |

**Installation:**
```bash
# No new third-party dependencies are justified for the first Phase 11 slice.
# Reuse the existing workspace and operator shell stack.
```

**Version verification:**
- Workspace stack is defined in `/home/jove/code/vel/Cargo.toml` and `clients/web/package.json`.
- `react` registry version verified as `19.2.4` with registry modified date `2026-03-18`.
- `vite` registry version is `8.0.1` as of `2026-03-19`; repo currently pins `^8.0.0`.
- `vitest` registry version is `4.1.0` as of `2026-03-12`; repo currently pins `^2.1.8` and the installed test runner reports `2.1.9`.
- Recommendation: do not hide a tooling upgrade inside Phase 11. Plan against the shipped repo stack unless a separate toolchain slice is approved.

## Architecture Patterns

### Recommended Project Structure
```text
crates/vel-api-types/src/lib.rs                 # Agent grounding + capability DTOs
crates/veld/src/services/agent_grounding.rs     # Grounding-pack assembler and capability summarizer
crates/veld/src/routes/agent_grounding.rs       # Thin operator-authenticated inspect route(s)
crates/veld/tests/agent_grounding.rs            # Integration tests for the new inspect contract
clients/web/src/data/agent-grounding.ts         # Typed loader for inspect surface
clients/web/src/components/SettingsPage.tsx     # Extend existing trust/review surface
crates/vel-cli/src/commands/agent.rs            # Thin CLI inspect/review surface, if shipped in Phase 11
config/schemas/agent-grounding.schema.json      # Durable contract publication
config/examples/agent-grounding.example.json    # Example payload for planner/docs/tests
docs/cognitive-agent-architecture/agents/agent-grounding-contracts.md
```

### Pattern 1: Build One Backend-Owned Grounding Pack
**What:** Add a new service that assembles a typed `AgentGroundingPackData` from shipped records instead of asking agent callers to stitch multiple endpoints together.
**When to use:** Any supervised agent entrypoint that needs product state broader than Phase 08 repo-local coding context.
**Example:**
```rust
// Source pattern: crates/veld/src/services/now.rs
// Source pattern: crates/veld/src/services/operator_queue.rs
// Source pattern: crates/veld/src/services/execution_routing.rs
pub async fn build_agent_grounding_pack(
    state: &AppState,
) -> Result<AgentGroundingPackData, AppError> {
    let now = crate::services::now::get_now(&state.storage, &state.config).await?;
    let projects = crate::services::projects::list_projects(state).await?;
    let people = crate::services::people::list_people(state).await?;
    let commitments = state
        .storage
        .list_commitments(Some(vel_core::CommitmentStatus::Open), None, None, 64)
        .await?;
    let pending_handoffs = crate::services::execution_routing::list_execution_handoffs(
        state,
        None,
        Some(crate::services::execution_routing::HandoffReviewState::PendingReview),
    )
    .await?;

    Ok(AgentGroundingPackData {
        now: now.into(),
        projects: projects.into_iter().map(Into::into).collect(),
        people: people.into_iter().map(Into::into).collect(),
        commitments: commitments.into_iter().map(Into::into).collect(),
        pending_execution_handoffs: pending_handoffs,
        // Add typed references to explain/freshness/debug state instead of raw unbounded dumps.
        current_context_ref: Some(AgentContextRefData::current()),
    })
}
```

### Pattern 2: Derive Capability Summaries From Existing Policy, Not From UI State
**What:** Produce an `AgentCapabilitySummaryData` in Rust that groups affordances into read, review, and mutation classes with operator labels and explicit blockers.
**When to use:** Every operator-visible inspect surface and every agent-launch preparation path.
**Example:**
```rust
// Source pattern: crates/veld/src/services/execution_routing.rs
// Source pattern: crates/veld/src/services/connect_runtime.rs
fn summarize_capabilities(
    writeback_enabled: bool,
    handoff: Option<&ExecutionHandoffRecordData>,
) -> AgentCapabilitySummaryData {
    AgentCapabilitySummaryData {
        read_context: vec![
            capability("current_now_state", "Read current Now state", "available"),
            capability("projects", "Read projects and project review candidates", "available"),
            capability("people", "Read people records and review links", "available"),
        ],
        review_actions: vec![
            capability("execution_handoff_review", "Review execution handoffs", "available"),
            capability("writeback_conflict_review", "Inspect pending writebacks and conflicts", "available"),
        ],
        mutation_actions: vec![
            gated_capability(
                "integration_writeback",
                "Request bounded integration mutations",
                writeback_enabled,
                "SAFE MODE is enabled; operator must enable writeback first",
            ),
            handoff_gated_capability(
                "repo_write_scope",
                "Use declared write scopes from the approved handoff only",
                handoff,
            ),
        ],
    }
}
```

### Pattern 3: One Inspect DTO, Multiple Thin Surfaces
**What:** Expose one typed inspect payload through runtime/API first, then render it in Settings and optionally CLI without changing the policy model.
**When to use:** The first shipped product slice for this phase.
**Example:**
```typescript
// Source pattern: clients/web/src/data/operator.ts
export interface AgentInspectData {
  grounding: AgentGroundingPackData;
  capabilities: AgentCapabilitySummaryData;
  review: {
    pending_execution_handoffs: number;
    pending_writebacks: number;
    open_conflicts: number;
    people_needing_review: number;
  };
}
```

### Anti-Patterns to Avoid
- **Do not add agent policy in React:** `SettingsPage.tsx` should render a Rust-owned capability summary, not decide it.
- **Do not make `/api/chat` the first grounding surface:** the current assistant path has no tools and no typed Vel-state contract.
- **Do not couple grounding to coding-only context:** Phase 08 execution context remains one input, not the whole product story.
- **Do not persist a second "agent state blob":** the first slice should assemble from existing records on demand.
- **Do not widen raw JSON access:** `CurrentContextData.context` and explain payload JSON are supporting evidence, not the primary grounding contract.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Agent state snapshot | Ad hoc prompt string concatenation from random routes | A typed `AgentGroundingPackData` assembled in `veld` | Keeps grounding inspectable, testable, and transport-safe |
| Permission model | New ambient agent permission system | Existing `read_scopes`, `write_scopes`, `allowed_tools`, connect capability allowlists, and SAFE MODE gating | Phase 08 already solved explicit scope and review semantics |
| Review queue | New agent-only approval queue | Existing `operator_queue`, handoff review endpoints, writeback/conflict queues, and `Now` action items | Prevents duplicate trust surfaces |
| Mutation availability checks | Frontend conditionals only | Backend capability summary with blockers such as `writeback_enabled`, review gate, or missing grant | Failing closed must happen server-side |
| Generic agent framework adoption | Provider- or framework-driven autonomy stack | Current local-first runtime plus typed contract publication | Scope is grounding over shipped seams, not autonomy/platform sprawl |

**Key insight:** this phase should productize what Vel already knows, not add a second cognition/runtime architecture next to the one that already exists.

## Common Pitfalls

### Pitfall 1: Using Raw Current-Context JSON As The Main Agent Contract
**What goes wrong:** The agent gets an unbounded blob instead of a stable product contract.
**Why it happens:** `GET /v1/context/current` is convenient, but it exposes `context: JsonValue`.
**How to avoid:** Make `Now` the primary summary payload and attach typed refs to `current_context`, `explain/context`, and `explain/drift` only where needed.
**Warning signs:** New fields are added as anonymous JSON, or the web/CLI code starts parsing deep context keys directly.

### Pitfall 2: Treating Coding Handoff Metadata As A Whole-Product Grounding Model
**What goes wrong:** Non-coding operator work gets forced into repo roots, coding-oriented tool names, and GSD-specific language.
**Why it happens:** Phase 08 already has mature scope/routing contracts.
**How to avoid:** Reuse the policy vocabulary, but create a separate agent-grounding DTO whose payload is product-wide and operator-relevant.
**Warning signs:** The proposed API only makes sense for repos or only talks about `allowed_tools` without product-readable labels.

### Pitfall 3: Bypassing SAFE MODE Through Agent "Awareness"
**What goes wrong:** A grounded agent appears to have write authority the runtime would otherwise block.
**Why it happens:** Read visibility and mutation ability get conflated.
**How to avoid:** Capability summaries must separate `can_read`, `can_review`, and `can_request_mutation`, and every mutation lane must still honor `writeback_enabled`, review gates, and existing routes.
**Warning signs:** Capability output says an agent "can send email" when the runtime is still in SAFE MODE.

### Pitfall 4: Duplicating Review State Instead Of Reusing Existing Queues
**What goes wrong:** `Now`, Settings, CLI, and agent inspect surfaces disagree about what needs review.
**Why it happens:** Each surface recomputes review status independently.
**How to avoid:** Reuse `operator_queue` plus pending handoff/writeback/conflict records and derive counts in one backend service.
**Warning signs:** The same handoff appears pending in one surface and invisible in another.

### Pitfall 5: Extending The Current Chat Assistant Before The Grounding Contract Exists
**What goes wrong:** The agent feels "smarter" but remains opaque and non-reviewable.
**Why it happens:** The chat surface is already present.
**How to avoid:** First ship a typed inspectable grounding/capability seam; only then let chat or other agent surfaces consume it.
**Warning signs:** Work starts in `services/chat/assistant.rs` before any new DTO/schema/service exists.

## Code Examples

Verified patterns from shipped code:

### `Now` Already Carries Review-Relevant State
```rust
// Source: /home/jove/code/vel/crates/veld/src/routes/now.rs
impl From<services::now::NowOutput> for NowData {
    fn from(value: services::now::NowOutput) -> Self {
        Self {
            action_items: value.action_items.into_iter().map(ActionItemData::from).collect(),
            review_snapshot: value.review_snapshot.into(),
            pending_writebacks: value.pending_writebacks.into_iter().map(Into::into).collect(),
            conflicts: value.conflicts.into_iter().map(Into::into).collect(),
            people: value.people.into_iter().map(Into::into).collect(),
            reasons: value.reasons,
            debug: value.debug.into(),
            // other typed `Now` fields omitted
        }
    }
}
```

### Execution Routing Already Enforces Explicit Scope And Review
```rust
// Source: /home/jove/code/vel/crates/veld/src/services/execution_routing.rs
if normalize_strings(input.allowed_tools.clone()).is_empty() {
    return Err(AppError::bad_request("handoff allowed_tools must not be empty"));
}

let read_scopes = normalize_strings(input.read_scopes.clone());
let write_scopes = normalize_strings(input.write_scopes.clone());
if read_scopes.is_empty() && write_scopes.is_empty() {
    return Err(AppError::bad_request(
        "handoff must declare at least one read or write scope",
    ));
}
```

### Web Already Builds Operator Review Status From Shared Typed Inputs
```typescript
// Source: /home/jove/code/vel/clients/web/src/data/operator.ts
export function buildOperatorReviewStatus(now, settings, handoffs = []) {
  return {
    writeback_enabled: settings?.writeback_enabled === true,
    pending_writebacks: now?.pending_writebacks ?? [],
    open_conflicts: now?.conflicts ?? [],
    people_needing_review: collectPeopleFromActionEvidence(now),
    pending_execution_handoffs: handoffs ?? [],
  }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Generic chat assistant with conversation history only | Backend-owned typed grounding pack plus capability summary | Needed for Phase 11 planning on 2026-03-19; current chat path is still ungrounded | Makes agent context inspectable and testable |
| Repo-only execution context for coding work | Product-wide grounding built from `Now`, projects, people, commitments, review queues, and handoffs | Phase 08 shipped coding context on 2026-03-19; Phase 11 must widen beyond it | Agents can reason over real Vel state instead of repo context alone |
| Low-level tool names and implicit trust assumptions | Operator-relevant capability classes with blockers, review gates, and SAFE MODE visibility | Required by locked product intent for Phase 11 | Prevents ambient-authority drift |

**Deprecated/outdated:**
- `/api/chat` as the first product surface for real agent grounding: current code proves it is ungrounded (`tools: vec![]`) and should not become the policy owner.
- Raw `CurrentContextData.context` as the main payload: keep it as explain/debug support only.
- Any plan that introduces broad provider/autonomy expansion in this phase: explicitly out of scope.

## Open Questions

1. **Should the first inspect surface be API-only, API+web, or API+CLI+web?**
   - What we know: Settings already exposes review/trust state and CLI already exposes execution handoff review.
   - What's unclear: whether a new CLI command is worth Phase 11 scope or whether web + API is enough.
   - Recommendation: ship runtime/API first and extend Settings in the same slice; make CLI optional unless the planner needs parity for operator workflows.

2. **Should the first grounding pack include direct mutation actions or only capability descriptors?**
   - What we know: SAFE MODE, writeback routes, and handoff approval already exist and must remain intact.
   - What's unclear: how much operator value is gained by shipping executable mutation affordances immediately.
   - Recommendation: first ship descriptors plus blockers (`requires_writeback_enabled`, `requires_review`, `missing_grant`) and reuse existing mutation routes later.

3. **Should grounding be assembled on demand or persisted as a new durable artifact?**
   - What we know: the source records are already durable and queryable.
   - What's unclear: whether planner wants a historical inspect artifact in Phase 11 or only live inspect.
   - Recommendation: assemble on demand first; only add persisted snapshots if a later task proves replay/audit needs them.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` workspace + web `vitest` (`jsdom`) |
| Config file | `/home/jove/code/vel/clients/web/vitest.config.ts` |
| Quick run command | `cd clients/web && npm run test -- src/data/operator.test.ts src/components/NowView.test.tsx src/components/SettingsPage.test.tsx` |
| Full suite command | `make test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| AGENT-CTX-01 | Agent inspect endpoint returns one typed grounding pack spanning `Now`, projects, people, commitments, and pending handoffs | integration | `cargo test -p veld agent_grounding_pack -- --nocapture` | ❌ Wave 0 |
| AGENT-CTX-02 | Grounding pack exposes typed refs, freshness, and evidence instead of raw unbounded dumps | integration | `cargo test -p veld agent_grounding_provenance -- --nocapture` | ❌ Wave 0 |
| AGENT-TOOLS-01 | Capability summary groups read, review, and mutation affordances with operator labels | unit/integration | `cargo test -p veld agent_capability_summary -- --nocapture` | ❌ Wave 0 |
| AGENT-TOOLS-02 | Unsupported or missing grants fail closed and surface narrow escalation reasons | unit/integration | `cargo test -p veld agent_capability_summary -- --nocapture` | ❌ Wave 0 |
| AGENT-REVIEW-01 | Settings/CLI surface pending handoffs, review gates, SAFE MODE, and other review obligations from the same inspect model | CLI + web | `cargo test -p vel-cli agent_inspect -- --nocapture && cd clients/web && npm run test -- src/components/SettingsPage.test.tsx` | ❌ Wave 0 |
| AGENT-TRUST-01 | The runtime inspect route and execution export share one grounding contract without widening `/api/chat` first | integration/manual | `cargo test -p veld execution_context -- --nocapture` | ⚠️ Extend existing |

### Sampling Rate
- **Per task commit:** quick Rust target for the touched service/route plus the targeted web inspect tests
- **Per wave merge:** `make test`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `/home/jove/code/vel/crates/veld/tests/agent_grounding.rs` — grounding-pack assembly and provenance/ref coverage for AGENT-CTX-01 and AGENT-CTX-02
- [ ] `/home/jove/code/vel/crates/veld/tests/agent_capability_summary.rs` — read/review/write blockers, SAFE MODE, and fail-closed grants for AGENT-TOOLS-01 and AGENT-TOOLS-02
- [ ] Extend `/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx` — render new inspect/capability summary without duplicating policy for AGENT-REVIEW-01
- [ ] Extend `/home/jove/code/vel/clients/web/src/components/NowView.test.tsx` — confirm agent-relevant review counts remain aligned with `Now` for AGENT-REVIEW-01
- [ ] `/home/jove/code/vel/crates/vel-cli/src/commands/agent.rs` tests if CLI inspect ships in this phase
- [ ] Keep the focused validation suite aligned to the new inspect/export targets; older linking-regression notes are no longer the gating Phase 11 risk

## Sources

### Primary (HIGH confidence)
- `/home/jove/code/vel/.planning/phases/11-agent-grounding-and-operator-relevant-data-tool-awareness/11-CONTEXT.md` - locked phase scope, constraints, and discretionary decisions
- `/home/jove/code/vel/.planning/ROADMAP.md` - phase goal, requirement IDs, sequencing, and priority note
- `/home/jove/code/vel/.planning/PROJECT.md` - accepted product decision that agent awareness is committed roadmap work
- `/home/jove/code/vel/docs/api/runtime.md` - mounted endpoints, review surfaces, connect lifecycle, and write boundaries
- `/home/jove/code/vel/crates/veld/src/services/now.rs` - `Now` assembly over typed context, review queue, people, writebacks, and conflicts
- `/home/jove/code/vel/crates/veld/src/services/operator_queue.rs` - ranked action/review queue and handoff/writeback/conflict evidence generation
- `/home/jove/code/vel/crates/veld/src/services/execution_routing.rs` - explicit scope/tool/review routing and launch gating
- `/home/jove/code/vel/crates/veld/src/services/chat/assistant.rs` - current assistant path is ungrounded and tool-less
- `/home/jove/code/vel/clients/web/src/data/operator.ts` - existing operator review-status aggregation over `Now`, settings, and handoffs
- `/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx` - current operator-visible review/trust surface
- `/home/jove/code/vel/clients/web/src/components/NowView.tsx` - current operator-visible `Now` projection and review pressure counts
- `/home/jove/code/vel/crates/vel-cli/src/commands/review.rs` - current CLI review summary over captures, projects, commitments, people, writebacks, and conflicts
- `/home/jove/code/vel/crates/vel-cli/src/commands/exec.rs` - current CLI review formatting for explicit routing/scopes/tools

### Secondary (MEDIUM confidence)
- NPM registry for `react`, `vite`, and `vitest` version/date verification on 2026-03-19
- `/home/jove/code/vel/config/README.md` - contract publication and ownership rules for schema-bearing surfaces
- `/home/jove/code/vel/config/schemas/execution-handoff.schema.json` - durable Phase 08 handoff contract vocabulary
- `/home/jove/code/vel/config/schemas/self-model-envelope.schema.json` - prior self-awareness/read-vs-write scope contract

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - phase should reuse the shipped Rust/React/config stack; no new framework choice is required
- Architecture: HIGH - backend/service/DTO/review seams are already present and clearly point to a new typed grounding service
- Pitfalls: MEDIUM - the biggest risks are clear in the current code, but exact Phase 11 contract names and UI breadth remain discretionary

**Research date:** 2026-03-19
**Valid until:** 2026-03-26
