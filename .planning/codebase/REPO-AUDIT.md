# Vel Repository Audit

**Date:** 2026-03-25
**Scope:** Full repository — Rust workspace, React frontend, docs, planning, CI, config, containerization
**Codebase:** 425 Rust files (149k lines), 155 TS/TSX files (35k lines), 13 crates, 50 migrations

---

## Executive Summary

Vel is a well-architected local-first system with strong foundational decisions. The crate layering is clean, the documented architecture matches reality, and the planning/milestone system is impressively thorough. The main concerns are accumulated drift from rapid development: thick route handlers, monolithic files, a stalled time library migration, suppressed dead code warnings, and 11 failing tests (3 Rust, 8 frontend). None of these are correctness-critical, but they compound maintenance cost.

### Overall Scores

| Dimension | Score | Trend |
|-----------|-------|-------|
| Architecture & Layering | **9/10** | Stable |
| Documentation | **7/10** | Needs pruning |
| Code Quality | **6/10** | Declining (drift) |
| Test Health | **5/10** | Failing tests on main |
| CI/CD | **6/10** | Minimal pipeline |
| Frontend | **7/10** | Strict TS, good patterns |
| Security | **8/10** | Auth-by-default, exposure gates |
| Config & DevEx | **7/10** | Good but naming collisions |
| Planning & Process | **9/10** | Exceptional for solo project |
| Technical Debt | **5/10** | Known, documented, accumulating |

---

## 1. Architecture & Layering (9/10)

### What's Working Well

- **Clean dependency graph**: `vel-storage` does NOT depend on `vel-api-types` — the documented layering rule holds
- **Domain isolation**: `vel-core` depends only on `vel-protocol`, `serde`, `time`, `uuid` — truly pure domain
- **Auth-by-default**: Routes split into `public_routes()` and `operator_authenticated_routes()` with middleware enforcement
- **Exposure gates**: `ExposureGate` middleware enforces route classification (public, operator, worker)
- **Adapter pattern**: Google Calendar and Todoist integrations are separate crates with their own Cargo.toml

### Issues

#### 1a. `vel-cli` depends on `vel-storage` (MEDIUM)
The CLI binary directly depends on `vel-storage`, breaking the expected architecture where CLI is a thin HTTP client. This creates a path for the CLI to bypass the API and directly manipulate the database.

**Recommendation:** Remove `vel-storage` from `vel-cli/Cargo.toml`. Any CLI operations needing storage should go through the API.

#### 1b. Thick Route Handlers (HIGH)
`routes/now.rs:update_now_task_lane` (lines 45-134) contains 90 lines of business logic including:
- Direct storage calls (`state.storage.get_commitment_by_id`, `state.storage.update_commitment`)
- Lane mutation logic (`remove_commitment_from_all_lanes`, lane insertion)
- Context serialization and persistence

This violates the documented "thin handler" pattern. Similar patterns exist in `routes/chat.rs` (967 lines) and `routes/daily_loop.rs` (704 lines).

**Recommendation:** Extract business logic from route handlers into corresponding service functions. Route handlers should be: parse request → call service → map to DTO → return response.

#### 1c. `app.rs` is 12,842 lines (MEDIUM)
The route registration file is the largest file in the entire codebase. While it's essentially a routing table, it's unwieldy.

**Recommendation:** Split into `routes/mod.rs` subrouters by domain (now, chat, threads, system, integrations, etc.), each returning a `Router<AppState>`.

---

## 2. Documentation (7/10)

### What's Working Well

- **MASTER_PLAN.md** is genuinely canonical — phase statuses match reality
- **Docs guide** (`docs/README.md`) with authority hierarchy is excellent
- **Config README** with asset classes (live/template/schema) is clear
- **User docs** exist for quickstart, setup, integrations, privacy, troubleshooting
- **API docs** exist in `docs/api/`
- **Architecture docs** in `docs/cognitive-agent-architecture/` are comprehensive (60+ files)

### Issues

#### 2a. Documentation Volume vs. Signal (HIGH)
The `docs/cognitive-agent-architecture/` directory contains 60+ spec files spanning architecture, cognition, devices, voice, agents, integrations, policies, and metrics. Many reference future systems (WASM guest runtime, swarm protocol, semantic memory graph RAG) that don't exist yet. This creates a "spec graveyard" problem where it's unclear what's implemented vs. aspirational.

**Recommendation:** Add a status badge (`[IMPLEMENTED]`, `[SPEC-ONLY]`, `[DEFERRED]`) to the top of each architecture doc. Alternatively, move unimplemented specs to a `docs/future/` directory (one already exists but is underused).

#### 2b. Stale Cross-References (MEDIUM)
- MASTER_PLAN.md references "Phases 6 and 8" for re-scoped work, but no `docs/tickets/phase-6/` or `phase-8/` directories exist
- `docs/README.md` references `.planning/phases/05-*` through `.planning/phases/09-*` as "active" — these are long since completed
- README.md references emoji (`🚀`) which is inconsistent with the emoji-free tone elsewhere

**Recommendation:** Audit all doc cross-references. Remove or update forward-references to phases that don't have ticket directories yet.

#### 2c. `documentation-catalog.json` Freshness (LOW)
Exists as a machine-readable catalog but its freshness relative to actual docs is unverified.

**Recommendation:** Add a `make verify-docs-catalog` target or integrate into `verify-repo-truth`.

#### 2d. Duplicate Config Directories (MEDIUM)
- `config/` contains agent-specs, policies, schemas, templates
- `configs/` contains model configuration files

This is confusing. Both the README and CLAUDE.md reference `config/` but `configs/models/` is where model weights and routing live.

**Recommendation:** Merge `configs/` into `config/models/` to eliminate the confusing naming.

---

## 3. Code Quality (6/10)

### What's Working Well

- Zero TODO/FIXME markers in the frontend (clean)
- TypeScript strict mode enabled with `noUnusedLocals`, `noUnusedParameters`
- Error types use `thiserror` consistently
- Repository pattern is clean and consistent across 46 repos

### Issues

#### 3a. Crate-Level `#![allow(dead_code)]` (HIGH)
Three files suppress dead code warnings at the crate level:
- `crates/veld/src/main.rs`
- `crates/veld/src/lib.rs`
- `crates/vel-cli/src/client.rs`

This masks potentially hundreds of unused functions across the two largest crates. The existing drift audit (VD-04) estimates 228+ warning sites.

**Recommendation:** Remove suppressions. Run `cargo clippy` and either use or delete dead code. This is the single highest-impact quality improvement available.

#### 3b. Stalled `chrono` → `time` Migration (MEDIUM)
The workspace declares both `chrono` and `time` as dependencies. Current usage: 10 `chrono` imports vs 197 `time` imports. The migration is 95% done but stalled.

**Recommendation:** Complete the final 10 chrono usages (likely in `veld` timezone handling). Remove `chrono` and `chrono-tz` from workspace deps.

#### 3c. Monolithic Files (MEDIUM)
| File | Lines | Issue |
|------|-------|-------|
| `veld/src/app.rs` | 12,842 | Route registration blob |
| `vel-api-types/src/lib.rs` | 8,489 | All DTOs in one file |
| `veld/src/services/now.rs` | 4,240 | Now service logic |
| `veld/src/services/client_sync.rs` | 2,770 | Sync logic |
| `vel-storage/src/db.rs` | 2,601 | Storage facade |
| `vel-cli/src/main.rs` | 2,596 | CLI entry point |
| `web/src/types.ts` | 6,226 | All frontend types |

**Recommendation:** Prioritize splitting `vel-api-types/src/lib.rs` into domain modules (now, chat, threads, system, etc.) and `app.rs` into domain routers. These two alone would significantly improve navigability.

#### 3d. 80 TODO/FIXME Comments in Rust (LOW-MEDIUM)
41 of these are in `integrations_todoist.rs` alone, suggesting incomplete integration work. The rest are scattered across 12 files.

**Recommendation:** Triage into: fix now, track as tickets, or delete if obsolete.

#### 3e. 45 `#[allow(...)]` Suppressions (LOW)
Spread across 20 files. Most are `allow(dead_code)` or `allow(unused)` at the item level (beyond the crate-level ones). Some in `policy_config.rs` (6 suppressions) and `nudges_repo.rs` (10 suppressions) suggest code that was written speculatively.

---

## 4. Test Health (5/10)

### Current State

- **Rust:** 3 failing tests on main branch
  - `agent_grounding_round_trips_typed_sections`
  - `agent_grounding_contract_assets_parse_and_register`
  - `recall_context_round_trips_named_counts_and_scores`
- **Frontend:** 8 failing tests across 3 test files (183 passing / 191 total)
  - `SystemView.test.tsx` failures
- **Total:** 11 failing tests on main = CI should be red

### Issues

#### 4a. Failing Tests on Main (CRITICAL)
Tests should never be red on main. The 3 Rust failures appear to be in `vel-api-types` serialization round-trip tests, suggesting a recent DTO change wasn't accompanied by test updates. The 8 frontend failures are in SystemView, suggesting UI contract drift.

**Recommendation:** Fix immediately. This is the highest-priority item in the audit.

#### 4b. Low Test Count (MEDIUM)
- Rust: ~46 tests across the entire workspace for 149k lines of code
- Frontend: 191 tests for 35k lines — much better ratio
- No integration tests for the HTTP API layer
- No end-to-end tests

**Recommendation:** Add HTTP integration tests for critical routes (now, chat, captures) using `axum::test` utilities. The `vel-sim` crate exists but appears unused — consider reviving it for this purpose.

#### 4c. CI Pipeline is Minimal (MEDIUM)
Single workflow with: `make ci` + `make smoke` + eval fixture smoke. No:
- Caching (Rust target, npm node_modules)
- Matrix testing (multiple Rust versions)
- Separate frontend/backend jobs
- Coverage reporting
- Security audit (`cargo audit`)

**Recommendation:** Add Rust/npm caching (saves 5-10 min per run), add `cargo audit` step, split into parallel jobs.

---

## 5. Security (8/10)

### What's Working Well

- Auth-by-default with explicit public route opt-in
- Exposure gate middleware (`ExposureGate`, `RouteExposureClass`)
- Token-based auth (`VEL_OPERATOR_API_TOKEN`, `VEL_WORKER_API_TOKEN`)
- Abuse guard on public linking endpoints
- `.env` in `.gitignore`
- SQLx compile-time verified queries (prevents SQL injection)

### Issues

#### 5a. No `cargo audit` in CI (MEDIUM)
No automated vulnerability scanning for Rust dependencies.

**Recommendation:** Add `cargo audit` to CI. Consider `cargo deny` for license compliance too.

#### 5b. `.env.example` Doesn't Document Token Variables (LOW)
The `.env.example` only lists bind address, DB path, artifact root, log level, and base URL. It doesn't mention `VEL_OPERATOR_API_TOKEN` or `VEL_WORKER_API_TOKEN`.

**Recommendation:** Add commented-out token vars to `.env.example` with documentation.

#### 5c. Gitignore Gaps (LOW)
Missing common entries:
- `*.env.local` (some tools create this)
- `.playwright-mcp/` (already exists in working tree as untracked)
- No `*.pem` or `*.key` pattern

**Recommendation:** Add security-relevant patterns to `.gitignore`.

---

## 6. Frontend (7/10)

### What's Working Well

- TypeScript strict mode with all lint rules enabled
- Clean dependency list (only 8 production deps)
- Component organization: `core/`, `shell/`, `views/`, `data/`, `api/`
- Co-located test files (`*.test.tsx` next to components)
- Good test-to-code ratio (191 tests / 155 files)

### Issues

#### 6a. `types.ts` is 6,226 Lines (HIGH)
All TypeScript types in a single file. This mirrors the Rust `vel-api-types/src/lib.rs` problem.

**Recommendation:** Split by domain: `types/now.ts`, `types/chat.ts`, `types/system.ts`, etc. Re-export from `types/index.ts`.

#### 6b. `SystemView.tsx` is 3,086 Lines (MEDIUM)
Single component file. The test file is also large (1,039 lines) and has failures.

**Recommendation:** Extract into subcomponents: `SystemSidebar`, `SystemIntegrations`, `SystemActivity`, `SystemSettings`, etc.

#### 6c. Two Icon Libraries (LOW)
Both `lucide-react` and `react-icons` are in dependencies. This is redundant.

**Recommendation:** Standardize on one icon library. `lucide-react` is the modern choice with consistent design.

#### 6d. `video.js` as a Dependency (LOW)
Video.js is a heavy media player library. Unclear if it's actually used for the personal cognition runtime use case.

**Recommendation:** Verify usage. If only for audio playback, consider the native `<audio>` element or a lighter library.

---

## 7. Config & Developer Experience (7/10)

### What's Working Well

- Nix shell for reproducible toolchain
- Comprehensive Makefile with well-named targets
- Docker/Podman/container-agnostic support
- `make dev` runs both backend and frontend together
- `make verify` runs full lint + test suite
- `make seed` and `make bootstrap-demo-data` for development data

### Issues

#### 7a. `config/` vs `configs/` (MEDIUM)
Two config directories with confusing names:
- `config/` — agent specs, policies, schemas, templates, contracts manifest
- `configs/` — model configuration and weights

**Recommendation:** Consolidate to `config/models/` and update all references.

#### 7b. `vel.toml` at Repo Root (LOW)
The main config file sits at the repo root, mixing runtime config with project files.

**Recommendation:** This is a reasonable default for development. Document that production deployments should use a different path.

#### 7c. Node Root `package.json` (LOW)
There's a `package.json` at the repo root in addition to `clients/web/package.json`. Also a root `node_modules/`. This suggests scripts that run from the root.

**Recommendation:** Document what the root `package.json` is for. If it's just for `verify-repo-truth.mjs`, consider moving that to a shell script.

---

## 8. Planning & Process (9/10)

### What's Working Well

- **Milestone system** with clear progression (v0.2 → v0.3 → 0.4.x → 0.5 → 0.5.1 → ... → 0.5.7)
- **Phase numbering** with context, plans, verification, and validation artifacts
- **STATE.md** with machine-readable frontmatter tracking current position
- **PROJECT.md** as the canonical product definition
- **Drift cleanup audit** (VELOCITY-DRIFT-CLEANUP.md) already identifies the main code issues
- **Codebase analysis** directory with prior architecture/convention/structure docs
- **TODO.md** directly feeds milestone scope

### Issues

#### 8a. Planning Directory Size (LOW)
`.planning/` is comprehensive but growing. 40+ phase directories, multiple milestone archives. Consider archiving older milestones more aggressively.

#### 8b. Overlap Between `docs/MASTER_PLAN.md` and `.planning/` (MEDIUM)
MASTER_PLAN.md tracks phases 1-5 with tickets in `docs/tickets/`. The `.planning/` directory tracks phases 54-109 with its own artifacts. These are two parallel systems.

**Recommendation:** Document the relationship explicitly: `docs/MASTER_PLAN.md` = historical phases 1-5, `.planning/` = active GSD-managed work from 0.5+ forward.

---

## 9. Technical Debt Inventory (Priority-Ordered)

| Priority | Item | Impact | Effort | Reference |
|----------|------|--------|--------|-----------|
| **P0** | Fix 11 failing tests | CI reliability | 2-4h | Section 4a |
| **P1** | Remove `#![allow(dead_code)]` | Code quality signal | 4-8h | Section 3a, VD-04 |
| **P1** | Extract business logic from route handlers | Architecture compliance | 8-12h | Section 1b |
| **P2** | Complete chrono → time migration | Dependency hygiene | 4-6h | Section 3b, VD-05 |
| **P2** | Split monolithic files (app.rs, api-types, types.ts) | Maintainability | 12-16h | Section 3c |
| **P2** | Add CI caching and `cargo audit` | DevEx + Security | 2-3h | Section 4c, 5a |
| **P3** | Merge `configs/` into `config/` | DevEx clarity | 1-2h | Section 7a |
| **P3** | Remove `vel-storage` from `vel-cli` | Layering correctness | 4-6h | Section 1a |
| **P3** | Add status badges to architecture docs | Doc signal | 2-4h | Section 2a |
| **P3** | Drop orphaned DB tables | Schema hygiene | 2-3h | VD-06 |
| **P4** | Triage 80 TODO/FIXME comments | Code hygiene | 2-4h | Section 3d |
| **P4** | Consolidate icon libraries | Bundle size | 1h | Section 6c |
| **P4** | Verify video.js necessity | Bundle size | 30m | Section 6d |

---

## 10. Best Practice Migration Recommendations

### 10a. Rust Best Practices

1. **Structured error types per crate**: Each crate should have a focused `Error` enum. Currently `veld` has 4 different error types scattered across modules. Consider a crate-level `error.rs` that re-exports.

2. **Builder pattern for complex service calls**: `update_commitment` takes 7 positional `Option` parameters (lines 78-87 of now.rs). Use a builder or update struct:
   ```rust
   // Instead of:
   storage.update_commitment(id, None, Some(status), None, None, None, None)
   // Use:
   storage.update_commitment(id, CommitmentUpdate { status: Some(status), ..Default::default() })
   ```

3. **Newtype IDs**: Commitment IDs, capture IDs, etc. are all `String`. Using newtypes (`struct CommitmentId(String)`) prevents mixing up ID types at compile time.

4. **`#[must_use]` on important return values**: Service functions that return `Result` should have `#[must_use]` to prevent silent error drops.

### 10b. Frontend Best Practices

1. **React Query / TanStack Query**: If not already using it (check `data/`), adopt for server state management with caching, deduplication, and background refetching.

2. **Error boundaries**: Add React error boundaries around each view to prevent full-app crashes from a single component failure.

3. **Accessibility audit**: No mention of a11y testing. Consider `@testing-library/jest-dom` matchers and axe-core integration.

### 10c. CI/CD Best Practices

1. **Cache Rust build artifacts** with `actions/cache` keyed on `Cargo.lock`
2. **Cache npm** with `actions/setup-node` built-in caching
3. **Run `cargo audit`** for vulnerability scanning
4. **Separate jobs** for Rust checks, frontend checks, and smoke tests (parallel execution)
5. **Add branch protection** requiring CI pass before merge

### 10d. Repository Hygiene

1. **Add `.playwright-mcp/` to `.gitignore`** (already showing as untracked)
2. **Add `CODEOWNERS`** file for review routing
3. **Add `CONTRIBUTING.md`** if external contributors are expected
4. **Add `renovate.json` or `dependabot.yml`** for automated dependency updates

---

## 11. Existing Drift Audit Alignment

The existing `docs/VELOCITY-DRIFT-CLEANUP.md` already identifies 11 cleanup tickets (VD-01 through VD-11). This repo audit **confirms and validates** all of those findings. Additional findings from this audit that go beyond VD-*:

- Failing tests on main (not covered by VD-*)
- CLI-to-storage layering bypass (not covered)
- Documentation status badges needed (not covered)
- CI pipeline improvements (not covered)
- Frontend monolith files (not covered)
- Security scanning gaps (not covered)
- Config directory naming (not covered)

**Recommendation:** Merge the VD-* tickets with this audit's findings into a single prioritized cleanup queue, scheduled as the post-0.5.7 work referenced in STATE.md.

---

*Generated by repo audit, 2026-03-25*
