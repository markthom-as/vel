# Technology Stack

**Project:** Vel v0.2 true MVP
**Researched:** 2026-03-20

## Recommended Stack

### Core Runtime

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Rust workspace (`vel-core`, `vel-storage`, `veld`, `vel-api-types`) | current repo baseline (`edition = 2021`) | Canonical MVP product logic, orchestration, and transport | This milestone should consolidate more behavior into the existing Rust-owned core, not introduce a second product stack. The repo already has the right authority split: domain in `vel-core`, storage in `vel-storage`, thin transport in `vel-api-types`, and runtime host in `veld`. |
| Tokio | `1.44.x` | Async runtime for daemon/services | Keep current baseline. No runtime change is justified for MVP closure. |
| Axum | `0.7.x` | Typed HTTP boundary for web and Apple shells | Keep current HTTP-first boundary as the default cross-surface contract. This matches the documented Apple/web adapter model and avoids shell-owned logic drift. |
| SQLx + SQLite | `0.8.x` + SQLite | Local-first persistence | Keep SQLite as the only durable store for MVP. The milestone needs stronger Rust-owned read models and reflow/application state, not a database change. |
| `tracing` / existing run-event model | current repo baseline | Provenance and explainability | Daily overview, nudges, threads, and review all need stronger run/event evidence, but that should land on the existing tracing/run stack rather than a new observability platform. |

### Web Shell

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| React | `19.2.4` in repo | Minimal operator shell over typed backend state | Keep React 19 and continue treating the browser as a thin shell over Rust-owned read models. Do not move MVP logic into client-side reducers. |
| React DOM | `19.2.4` in repo | Browser rendering | No change needed. |
| TypeScript | `5.9.3` in repo | DTO decoding and shell safety | Keep strict typed boundary decoding. It is already the right seam for a thin web shell. |
| Vite | `8.0.0` in repo | Dev/build toolchain | Keep current tooling. This milestone is product closure, not frontend toolchain migration. |
| Existing typed data layer (`src/data/*`, `src/types.ts`) | repo-local | API adapter and shell hydration | Reuse it. Do not add a second client data abstraction unless an actual bottleneck appears. |

### Apple Shell

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| SwiftUI | current Apple-platform baseline, Swift tools `5.9` in repo | Minimal native shell for iPhone, watch, and macOS | Keep SwiftUI as the presentation layer. It is already the documented Apple shell boundary. |
| `VelAPI` | repo-local | Shared HTTP transport adapter to `veld` | Keep as the default path. Apple is still HTTP-first today, and this milestone should not replace that truth. |
| `VelAppleModules` | repo-local | Explicit Apple shell/application/platform seams | Keep and use these modules to prevent Swift from becoming the owner of planner, review, or thread policy. |
| EventKit | Apple framework | Optional local calendar read/export seam for Apple-local calendar input | This is the only material Apple-side addition worth making for MVP: read local calendars with user permission, then export/sync them into the Rust-owned planner/reflow lane. |

### Local-First Calendar Input

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Existing `.ics` calendar adapter in `veld` | repo-local | Canonical local calendar ingest path | Prefer extending the existing `.ics` path instead of inventing a new planner-side calendar stack. |
| Apple-local calendar export via `VelLocalSourceExporter` extension | new repo addition, no new third-party dependency required | Make Apple-local calendars usable by the Rust core | The practical MVP move is to add calendar export beside the existing reminders/health/messages snapshots. Reuse the local-source bridge model already documented for Apple/macOS. |
| Google Calendar connector | existing shipped path | Remote calendar source when local Apple export is unavailable | Keep as-is for users who already rely on Google-backed sync. Do not widen provider scope in this milestone. |

### Narrow Additions Recommended

| Addition | Scope | Why |
|---------|-------|-----|
| Apple calendar export/snapshot path | Apple + `veld` local-source ingest | Needed to make “reflow calendar, ideally locally” feasible without moving planner logic into Swift. |
| Rust-owned MVP read model for daily overview / commitment / nudge / review | `vel-core` + `veld` services + DTOs | The milestone goals are mostly product-core closure, not framework churn. Add stronger Rust-owned read models instead of more UI libraries. |
| Optional iPhone-only embedded helper use of the existing bridge seam | Only for cache hydration / queue shaping helpers if a concrete latency issue remains | Allowed by current architecture, but keep it narrow and additive. Do not broaden into full embedded runtime or general FFI migration. |

## Calendar Reflow Feasibility

### Recommended MVP Approach

1. Compute reflow in Rust from canonical commitments, current context, planning profile, and calendar events.
2. Accept calendar events from whichever source is already available:
   - Google Calendar sync
   - local `.ics`
   - new Apple-local EventKit export feeding the same local-source ingest lane
3. Render proposal/review/apply state in web and Apple as thin shells over the same typed backend result.

### Feasibility Notes

| Surface | Feasibility | Recommendation |
|---------|-------------|----------------|
| Apple local calendar read | **High** | Feasible with EventKit permission. Use it only to supply event data into the Rust-owned reflow substrate. |
| Apple local calendar write/re-schedule | **Medium/Low for MVP** | Do **not** add this in v0.2. It widens into provider-specific mutation, conflict handling, and review semantics. |
| Web direct local calendar access | **Low** | Browser shells do not have a practical ambient local-calendar API. Treat web as daemon-backed; at most support explicit file/path selection handled through the daemon host. |
| Cross-platform local-first reflow | **Medium** | Feasible if “local-first” means local data on the daemon host or Apple-exported local snapshots feeding Rust. Not feasible if it means every shell mutates native calendars independently. |

### Strong Recommendation

Use **Apple-local calendar export into the existing Rust ingest/reflow path**, not Apple-local planner logic and not browser-local calendar handling.

If the Apple-local export shape must be added, prefer:

- exporting into the existing `.ics` ingest lane first, or
- adding one small typed calendar snapshot format only if `.ics` proves too lossy

Do not create a separate Apple-only planner contract.

## UI Shell Implications

### Web

| Decision | Recommendation | Why |
|----------|----------------|-----|
| State management | Keep the current typed fetch/decoder layer | MVP needs less shell complexity, not React-side authority. |
| Design system | Do not add one | A fresh minimal UI can be built with the current stack. A design-system rollout would slow the milestone and harden the wrong abstractions. |
| Real-time/sync | Reuse current websocket/query seams where already present | Enough for continuity; no new frontend state platform is needed. |
| Local calendar | Do not pursue browser-native calendar access | Keep that concern on the daemon host or Apple exporter path. |

### Apple

| Decision | Recommendation | Why |
|----------|----------------|-----|
| Primary transport | Keep `VelAPI` HTTP-first | Still the default and most portable cross-surface model. |
| Embedded Rust | Keep additive and narrow | Only use the existing embedded seam for explicit helper flows if required. |
| Native framework use | Add EventKit events read/export only | This is the one Apple-native addition that materially helps MVP local-first reflow. |
| Product logic ownership | Keep in Rust | SwiftUI should render, queue, capture permissions, and export local data; it should not own planning/review semantics. |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Apple portability | HTTP-first Apple shell plus narrow optional embedded helper | Full UniFFI / broad FFI migration | Current repo authority explicitly says no immediate UniFFI migration; too much boundary churn for MVP. |
| Calendar locality | EventKit read/export into Rust ingest | Apple-local planner/reflow logic | Forks semantics and violates Rust-core-first goal. |
| Web shell data | Existing typed fetch/decoder layer | React Query / Redux / Zustand rollout | Adds client authority and migration work without solving the milestone problem. |
| Calendar mutation | Review/apply stays backend-owned and bounded | Full local calendar write-back across Apple/web | Too much provider-specific complexity for this milestone. |
| Desktop packaging | Existing daemon + shells | Tauri/Electron/Desktop re-platform | Not needed for true-MVP closure. |

## What Should NOT Be Added In This Milestone

- No new database, sync engine, or hosted backend.
- No broad frontend state library migration.
- No cross-platform design system rollout.
- No broad FFI migration, UniFFI adoption, or “Apple becomes the runtime” rewrite.
- No full local calendar write-back engine.
- No CalDAV stack, Exchange stack, or new calendar providers beyond the existing Google/local-source paths.
- No shell-owned planner, suggestion, thread, or review semantics.
- No Tauri/Electron packaging work.
- No Graph-RAG, wider agent platform work, or other Phase-8-style expansion unless directly required for the MVP operator flows.

## Installation

```bash
# Rust core: keep existing workspace stack
cargo build

# Web shell: keep existing shell stack
cd clients/web && npm install

# Apple shell: keep existing Swift packages/Xcode path
# Add EventKit calendar permission/export work in the existing Apple targets only if local calendar MVP is selected
```

## Sources

- Internal authority: `docs/MASTER_PLAN.md`
- Internal authority: `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md`
- Internal authority: `docs/cognitive-agent-architecture/apple/apple-rust-integration-path.md`
- Internal authority: `docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md`
- Internal authority: `clients/apple/README.md`
- Internal authority: `docs/user/integrations/local-sources.md`
- Internal authority: `docs/user/integrations/apple-macos.md`
- Internal implementation: `clients/web/package.json`
- Internal implementation: `Cargo.toml`
- Internal implementation: `clients/apple/VelAPI/Sources/VelAPI/VelLocalSourceExporter.swift`
- React docs: https://react.dev/
- Vite docs: https://vite.dev/guide/
- SwiftUI docs: https://developer.apple.com/documentation/swiftui
- EventKit docs: https://developer.apple.com/documentation/eventkit
- MDN File System Access API: https://developer.mozilla.org/docs/Web/API/File_System_API
