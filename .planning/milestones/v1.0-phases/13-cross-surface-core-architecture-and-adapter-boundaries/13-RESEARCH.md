# Phase 13: Cross-surface core architecture and adapter boundaries - Research

**Researched:** 2026-03-19
**Domain:** Canonical Rust-owned cross-surface architecture for Apple, web, and future desktop shells
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Locked product/architecture direction
- [locked] Vel should use one real Rust product core across shells rather than letting Apple, web, or future desktop clients own their own product logic.
- [locked] The important goal is universal business logic and stable cross-surface contracts, not crate-name purity.
- [locked] Contracts and canonical read models should be defined before broad new UI work widens.
- [locked] Tauri/desktop should be planned for, but it can wait as an implementation target.

### Current-repo alignment
- [locked] The existing repo is not a greenfield workspace. Planning must align with the current split around `vel-core`, `vel-storage`, `vel-api-types`, `veld`, `vel-cli`, and the current Apple/web clients.
- [locked] Phase 13 should prefer incremental evolution of current crates and seams over a mass rename into a brand-new crate taxonomy.
- [locked] Existing architectural drift should not be normalized, but nearby good-enough seams should be extended rather than rewritten for aesthetics.

### Apple integration stance
- [locked] Apple currently uses an HTTP-first model through `VelAPI` talking to `veld`; that remains the active integration model.
- [locked] Phase 13 should document the future Apple embedded-Rust / FFI migration path, but should not require a full Apple FFI migration in this phase.
- [locked] Apple-native presentation, App Intents, widgets, notifications, complications, and lifecycle glue remain shell-owned concerns; product logic should remain Rust-owned.

### Web and desktop stance
- [locked] Web should continue to consume Rust through authenticated HTTP/JSON contracts and streaming where helpful, not through direct browser exposure to internal core structs.
- [locked] Future desktop/Tauri work should be designed as a shell over the same Rust-owned contracts, ideally compatible with either local-daemon or in-process adapter patterns.
- [locked] Phase 13 should plan with embedded, daemon, and server runtime topologies in mind, even if not all are implemented now.

### Sequencing and scope discipline
- [locked] Phase 13 is architecture-first.
- [locked] Phase 14 should handle product discovery, operator modes, advanced/dev gating, and milestone shaping.
- [locked] Later phases should do incremental architecture migration first, then canonical Rust business logic, then broader shell/UI embodiment.
- [locked] UX discovery should not be left to emerge accidentally from current UI work.

### Deferred Ideas (OUT OF SCOPE)
- Full Apple FFI / UniFFI migration
- A real Tauri/desktop shell implementation
- Broad UI redesign, navigation overhaul, or shell-specific product-definition work
- Multi-phase crate reorganization that is not justified by a narrow seam migration
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| ARCH-XS-01 | Define one canonical cross-surface architecture for embedded-capable, daemon-capable, and server-capable Vel runtimes. | Current repo already embodies server/daemon authority through `veld`; planning should ratify the topology rather than inventing it. |
| ARCH-XS-02 | Publish current-state-to-target-state ownership for crates, shells, commands, queries, read models, and transport DTOs. | Current seams exist but are implicit; they need one authoritative mapping doc. |
| ADAPT-01 | Define shell adapter boundaries for Apple, web, and future desktop/Tauri without moving business logic into shell-specific layers. | Apple and web boundaries are real today (`VelAPI`, `vel-api-types`, `veld` routes); future desktop can be planned against the same model. |
| ADAPT-02 | Prove the architecture on one real existing flow so later migration phases target a demonstrated seam rather than a speculative diagram. | Daily loop and agent inspect are already multi-surface examples with backend-owned policy and typed transport. |
| APPLE-ARCH-01 | Document the future Apple embedded-Rust / FFI migration path without forcing it into the current phase. | Apple is explicitly HTTP-first today; the plan should capture why and when an embedded path would be justified. |
| API-ARCH-01 | Preserve the current HTTP/JSON authority boundary for web and current Apple clients while clarifying future runtime-host options. | `docs/api/runtime.md` and `clients/apple/README.md` already make `veld` the authority. |
</phase_requirements>

## Summary

The codebase is already much closer to the user’s target architecture than the raw crate-name proposal suggests. `veld` is the authority/runtime host, `vel-core` owns domain vocabulary, `vel-storage` owns persistence, `vel-api-types` owns transport DTOs, `vel-cli` is a shell, and Apple/web are already clients over backend-owned flows. The architectural problem is therefore not “how do we replace a frontend-owned app with Rust?” The problem is “how do we ratify and extend this pattern before shell growth or future desktop work creates drift?”

The Apple boundary is the most important concrete reality check. `clients/apple/README.md` states that Apple apps talk to the same daemon over HTTP and that no business logic should live in the client. `VelClient.swift` confirms that the active integration boundary is typed HTTP/JSON over `VelAPI`. That means the right near-term move is to document a future embedded/FFI mode as an optional topology, not to pretend the repo is already organized around that mode or to force a premature migration.

The strongest existing proof seams are Phase 10 and Phase 11. Daily loop and agent inspect both place business logic in Rust backend services, expose typed contracts, and let Apple/web/CLI consume those results without defining local policy. Those are exactly the kinds of flows Phase 13 should point to as “honest architecture examples.” A good Phase 13 plan therefore mixes architecture docs and ownership rules with one proof-oriented slice that names a live flow and shows how it maps to the target cross-surface model.

**Primary recommendation:** Make Phase 13 a 4-plan architecture phase:
1. ratify the cross-surface architecture and current-to-target map
2. define canonical command/query/read-model and DTO ownership rules
3. document the Apple embedded/FFI and future desktop/Tauri adapter paths
4. prove the architecture against one live flow and capture migration guardrails for later phases

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `vel-core` | workspace `0.1.0` | Domain vocabulary and invariants | Already the closest existing equivalent to the desired portable product-core layer |
| `vel-storage` | workspace `0.1.0` | Durable storage and repository boundaries | Keeps persistence out of shells and transport |
| `vel-api-types` | workspace `0.1.0` | Shared transport DTOs | Existing canonical seam for HTTP-facing contracts |
| `veld` | workspace `0.1.0` | Runtime authority, services, routes, auth, orchestration | Current daemon/server host and source of truth |
| `vel-cli` | workspace `0.1.0` | Operator shell | Existing shell that should consume services/contracts rather than own behavior |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Swift package `VelAPI` | current repo package | Typed Apple HTTP client | Current Apple boundary and future comparison point for FFI migration decisions |
| Web `types.ts` + `src/data/*` | current repo frontend | Typed web decoder/loader layer | Existing browser-side contract consumption pattern |
| Config schemas/examples/manifests | current repo assets | Durable contract publication | Use when architecture outputs become stable enough to warrant schema-backed assets |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Incremental architecture ratification over existing crates | Big-bang rename to `vel-domain`/`vel-application`/`vel-runtime` etc. | Cleaner on paper, but high-churn and weakly justified by the current repo state |
| Apple HTTP-first now, documented FFI later | Immediate UniFFI/embedded-Rust migration | Misaligned with current shipped architecture and likely too broad for an architecture phase |
| Future desktop planned as a shell over shared contracts | Tauri-first product phase now | Creates UI/package pressure before architecture and product discovery are settled |
| Proof slice over an existing flow | Pure docs with no live reference flow | Easier to write, but more likely to drift into abstract architecture diagrams |

## Architecture Patterns

### Pattern 1: Preserve Rust-Owned Product Truth, Vary Only The Shell Boundary
**What:** Treat Rust as the application substrate and shells as presentation/integration layers.
**When to use:** Always, across Apple, web, CLI, and future desktop shells.
**Example:** `veld` services own daily loop and agent inspect logic; Apple/web/CLI consume typed results.

### Pattern 2: Canonical Commands / Queries / Read Models, Not Screen-Shaped APIs
**What:** Name the core in terms of product actions and snapshots rather than one shell’s UI structure.
**When to use:** When publishing new contracts or evaluating whether a boundary belongs in core vs shell.
**Example:** `start daily loop`, `submit daily-loop turn`, `get agent inspect`, `get now`, `review handoff readiness`.

### Pattern 3: Stable Transport DTOs At The Boundary, Not In Core Semantics
**What:** Keep transport DTOs in `vel-api-types` or future adapter crates rather than letting them infect `vel-core`.
**When to use:** Any HTTP, FFI, or desktop-facing boundary.
**Example:** `AgentInspectData`, `DailyLoopSessionData`, `NowData` remain transport shapes even when backed by `vel-core` domain types.

### Pattern 4: Document Future Embedded / Local-Daemon / Server Topologies Without Forcing Them All Now
**What:** Ratify multiple supported runtime topologies as design targets.
**When to use:** Cross-surface planning, especially Apple and future desktop work.
**Example:** Apple today = HTTP client to daemon/server; future Apple = optional embedded library for selective flows; future desktop = local daemon or in-process shell over the same core-owned contracts.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Architecture truth | Ad hoc assumptions in shell code or chat history | One canonical architecture doc + roadmap-backed phase sequence | Preserves intent and avoids drift |
| Apple portability plan | “Maybe UniFFI later” with no explicit path | Documented migration decision tree and adapter boundary | Future phases need more than folklore |
| Desktop strategy | Tauri-specific business logic | Shared command/query/read-model contracts and an adapter seam | Keeps future desktop optional and portable |
| Core ownership | DTOs or view models leaking into `vel-core` | Existing `vel-core`/`vel-api-types` separation with clearer rules | Aligns with current repo architecture |

## Common Pitfalls

### Pitfall 1: Treating Crate Renaming As Architecture Progress
**What goes wrong:** The repo incurs churn without materially improving shell-owned logic or transport discipline.
**How to avoid:** Tie any structural move to a concrete migration seam or ownership defect.

### Pitfall 2: Designing Around Future Apple FFI Instead Of Current Shipped Truth
**What goes wrong:** The architecture drifts away from the live repo and planning stops being executable.
**How to avoid:** Start from the current Apple HTTP boundary, then document when embedded Rust becomes justified.

### Pitfall 3: Letting UI Growth Define Product Semantics
**What goes wrong:** Web or Apple interactions become the de facto business layer.
**How to avoid:** Phase 13 locks the architecture, Phase 14 locks the product modes, and only then do broader UI phases widen.

### Pitfall 4: Planning Tauri As If It Already Exists
**What goes wrong:** Desktop packaging concerns hijack the architecture phase.
**How to avoid:** Plan the adapter/runtime host choices now, defer the actual shell to a later phase.

## Recommended Execution Shape

- **13-01:** publish canonical architecture doc and current-state-to-target-state ownership map
- **13-02:** define contract/ownership rules for commands, queries, read models, DTOs, and shell capability surfaces
- **13-03:** document Apple embedded/FFI migration path plus future desktop/Tauri adapter path
- **13-04:** prove the architecture on one existing flow and record guardrails for later migration phases

---

*Phase: 13-cross-surface-core-architecture-and-adapter-boundaries*
*Research completed: 2026-03-19*
