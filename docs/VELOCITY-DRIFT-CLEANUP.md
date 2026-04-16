# Velocity Drift & Cleanup Audit

**Status date:** 2026-03-25
**Scope:** Full Rust workspace — all crates in `crates/*`, migrations, and Cargo workspace config.
**Overall verdict:** Architecture is sound; file-level organization and some design choices accumulated drift during rapid development. No correctness issues. All findings are cleanup/polish, not bugs.

---

## Summary Scorecard

| Dimension | Score | Notes |
|-----------|-------|-------|
| Layering correctness | 9/10 | Excellent. No coupling violations found. |
| Dead code | 6/10 | Crate-level `#![allow(dead_code)]` masks 228+ warning sites |
| Over-engineering | 6/10 | Two traits with a single implementation each |
| File organization | 5/10 | Four monolithic files, one 280KB DTO blob |
| Error handling consistency | 6/10 | Three different error patterns across layers |
| Dependency hygiene | 7/10 | `chrono` + `time` both in workspace (mid-migration stall) |
| Schema ownership clarity | 8/10 | Phase 107 recheck found one active backup table and two planned foundation tables, not drop-safe orphaned schema |
| Config field usage | 9/10 | All fields read. No dead config. |

---

## Ticket Index

| ID | Title | Severity | Effort | Risk |
|----|-------|----------|--------|------|
| VD-01 | Remove single-impl `CapabilityResolver` trait | MEDIUM | 1h | LOW |
| VD-02 | Remove single-impl `ToolRunner` trait | MEDIUM | 1h | LOW |
| VD-03 | Delete dead policy structs in `policy_config.rs` | LOW | 30m | LOW |
| VD-04 | Lift crate-level `#![allow(dead_code)]` suppressions | HIGH | 2–4h | LOW |
| VD-05 | Complete chrono → time migration | MEDIUM | 6h | MEDIUM |
| VD-06 | Reclassify flagged schema tables (`vel_self_metrics`, `storage_targets`, `verification_records`) | MEDIUM | 3h | LOW |
| VD-07 | Split `app.rs` route registration (12,842 lines) | LOW | 4h | NONE |
| VD-08 | Split `now.rs` service (4,240 lines) | LOW | 8h | NONE |
| VD-09 | Reorganize `vel-api-types/src/lib.rs` (8,485 lines) | LOW | 6h | NONE |
| VD-10 | Standardize error handling across layers | MEDIUM | 8h | MEDIUM |
| VD-11 | Decide fate of `vel-sim` and `vel-agent-sdk` crates | LOW | 1h | NONE |

---

## VD-01 — Remove single-impl `CapabilityResolver` trait

**File:** `crates/veld/src/services/capability_resolver.rs`
**Severity:** MEDIUM — unnecessary indirection, confuses the "is this swappable?" question
**Effort:** ~1 hour
**Risk:** LOW — stateless, no mocking, no test doubles

### Finding

The `CapabilityResolver` trait has exactly one implementation and is always called via `DefaultCapabilityResolver.resolve(...)` — never via a dynamic or generic dispatch path.

```rust
// capability_resolver.rs:6–13
pub trait CapabilityResolver {
    fn resolve(&self, request: &CapabilityResolutionRequest) -> CapabilityResolutionDecision;
}

pub struct DefaultCapabilityResolver;

impl CapabilityResolver for DefaultCapabilityResolver { ... }
```

**All call sites** (in `command_lang.rs`) construct the concrete type directly:

```rust
// command_lang.rs:20
use crate::services::capability_resolver::{CapabilityResolver, DefaultCapabilityResolver};

// command_lang.rs:497
let decision = DefaultCapabilityResolver.resolve(&CapabilityResolutionRequest { ... });

// command_lang.rs:615
let policy_decision = DefaultCapabilityResolver.resolve(&CapabilityResolutionRequest { ... });
```

`DefaultCapabilityResolver` is a unit struct (`struct DefaultCapabilityResolver;`) — it carries no state.

### Fix

Delete the trait. Convert to a module-level free function:

```rust
// capability_resolver.rs — after
pub fn resolve(request: &CapabilityResolutionRequest) -> CapabilityResolutionDecision {
    // same body from impl block
}
```

Update two call sites in `command_lang.rs:497` and `command_lang.rs:615` to call `capability_resolver::resolve(...)`.

Delete the now-unused import `use crate::services::capability_resolver::{CapabilityResolver, DefaultCapabilityResolver}` and replace with `use crate::services::capability_resolver::resolve as resolve_capability`.

---

## VD-02 — Remove single-impl `ToolRunner` trait

**File:** `crates/veld/src/services/tool_runner.rs`
**Severity:** MEDIUM — same pattern as VD-01
**Effort:** ~1 hour
**Risk:** LOW

### Finding

```rust
// tool_runner.rs:6–26
pub trait ToolRunner {
    async fn run(&self, request: &ToolInvocationRequest) -> ToolInvocationOutcome;
}

pub struct ShellToolRunner;

impl ToolRunner for ShellToolRunner {
    async fn run(&self, request: &ToolInvocationRequest) -> ToolInvocationOutcome { ... }
}
```

Only one call site in the workspace:

```rust
// command_lang.rs:24
use crate::services::tool_runner::{ShellToolRunner, ToolRunner};

// command_lang.rs:728
let tool_outcome = ShellToolRunner.run(&tool_request).await;
```

`ShellToolRunner` is a unit struct with no state.

### Fix

Delete the trait. Convert to a free async function:

```rust
// tool_runner.rs — after
pub async fn run_tool(request: &ToolInvocationRequest) -> ToolInvocationOutcome {
    // same body
}
```

Update `command_lang.rs:728` to call `tool_runner::run_tool(&tool_request).await`.

---

## VD-03 — Delete dead policy structs in `policy_config.rs`

**File:** `crates/veld/src/policy_config.rs`
**Severity:** LOW — misleads readers into thinking these policies are active
**Effort:** 30 minutes
**Risk:** LOW — zero callers outside the file itself

### Finding

Two policy-map structs are dead in the runtime policy config: `PolicyMedsNotLogged` and `PolicyMorningDrift`. Their accessor methods are also dead; runtime behavior for those nudge families is driven by context/nudge strings and `SuggestionPolicies::morning_drift`, not by `policies.*` typed accessors.

`PolicyMeetingPrepWindow` is active and must stay: `crates/veld/src/services/suggestions.rs` reads `meeting_prep_window().default_prep_minutes` when producing prep-time suggestions. `PolicyCommuteLeaveTime` is also active through `nudge_engine.rs` and `suggestions.rs`.

```rust
pub struct PolicyMedsNotLogged { /* no active typed accessor consumers */ }
pub struct PolicyMorningDrift { /* superseded by SuggestionPolicies::morning_drift */ }
```

The parent `PoliciesMap` holds `Option<PolicyMedsNotLogged>` and `Option<PolicyMorningDrift>` as fields only for config parsing/defaults. Those fields are safe to remove together with the matching config/schema entries.

### Fix

1. Delete `PolicyMedsNotLogged` and `PolicyMorningDrift` and their `impl Default` blocks.
2. Delete the two corresponding `Option<…>` fields from `PoliciesMap`.
3. Delete the two accessor methods.
4. Remove the two defaults from the `PoliciesMap::default()` factory.
5. Remove the inactive `policies.meds_not_logged` and `policies.morning_drift` keys from checked-in policy config/template/example/schema files.

---

## VD-04 — Lift crate-level `#![allow(dead_code)]` suppressions

**Files:**
- `crates/veld/src/lib.rs:1`
- `crates/veld/src/main.rs:1`
- `crates/vel-cli/src/client.rs:1`

**Severity:** HIGH — these blanket suppressions hide 228+ warning sites across 42 files. See `docs/DEAD-CODE.md` for the pre-suppression snapshot.
**Effort:** 2–4 hours per crate (depends on how many sites need `#[allow]` or deletion)
**Risk:** LOW — warnings are not errors; lifting the blanket is safe

### Finding

```rust
// crates/veld/src/lib.rs:1
#![allow(dead_code)]

// crates/veld/src/main.rs:1
#![allow(dead_code)]

// crates/vel-cli/src/client.rs:1
#![allow(dead_code)]
```

`DEAD-CODE.md` documents the clusters (see that file for per-file counts). Top contributors before suppression:

| File | Warning Sites |
|------|--------------|
| `services/writeback.rs` | 25 |
| `services/integrations_github.rs` | 22 |
| `services/integrations_email.rs` | 16 |
| `services/workflow_runner.rs` | 15 |
| `routes/integrations.rs` | 13 |
| `adapters/notes.rs` | 10 |

These are not random dead code — they are **implemented-but-not-yet-routed** surfaces (writeback, email, GitHub integrations). The blanket suppression was applied to make the build quiet during active development.

### Fix

Work through each cluster in priority order:

1. **Route the surface** if it's ready (writeback, GitHub) — this is the preferred fix.
2. **Apply a targeted `#[allow(dead_code)]`** on the specific item if it's intentionally future work, with a comment explaining why.
3. **Delete the item** if it's genuinely abandoned.

After triaging all 42 files, remove the three crate-level suppression attributes.

Do this work in a single focused session per crate — it's mechanical but must be done as a batch to avoid re-accumulating the blanket.

---

## VD-05 — Complete chrono → time migration

**Affected files:**
- `crates/veld/src/services/planning_profile.rs`
- `crates/veld/src/services/timezone.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/services/availability_projection.rs`
- `crates/veld/src/services/recurrence_materialization.rs`
- `crates/veld/tests/phase64_recurrence_and_availability.rs`
- `crates/veld/tests/phase62_recurrence_and_attendees.rs`
- `crates/veld/tests/phase62_availability.rs`
- `crates/veld/tests/phase64_gcal_black_box.rs`

**Workspace Cargo.toml:**
- `chrono` at line 49
- `chrono-tz` at line 50
- `time` at line 38 (target crate)

**Severity:** MEDIUM — two temporal libraries in one binary, chrono is heavier and offset-naive by default
**Effort:** ~6 hours
**Risk:** MEDIUM — temporal logic is fragile; datetime arithmetic and timezone handling need careful migration

### Finding

The codebase started on `chrono`, migrated vel-core and vel-api-types to `time`, but left 5 service files and 4 test files still using `chrono`. Both crates are compiled into the `veld` binary, adding ~200KB of unnecessary binary weight and forcing contributors to reason about two different datetime APIs.

```toml
# Cargo.toml:38,49-50
time = { version = "0.3", features = ["formatting", "macros", "parsing", "serde"] }
chrono = { version = "0.4", default-features = false, features = ["clock"] }
chrono-tz = "0.10"
```

### Migration map

| chrono type | time equivalent |
|-------------|----------------|
| `chrono::DateTime<Utc>` | `time::OffsetDateTime` |
| `chrono::NaiveDate` | `time::Date` |
| `chrono::NaiveTime` | `time::Time` |
| `chrono::Duration` | `time::Duration` |
| `chrono_tz::Tz` | `time_tz::TimeZone` or `time::UtcOffset` |
| `Utc::now()` | `OffsetDateTime::now_utc()` |
| `.timestamp()` | `.unix_timestamp()` |
| `.format(...)` | `.format(&format_description!(...))` |

### Fix steps

1. Add `time-tz = { version = "0.5", features = ["db-tzdb"] }` to workspace deps (for timezone-aware operations currently using `chrono-tz`).
2. Migrate `timezone.rs` first — it is the isolated utility; fixes will cascade to callers.
3. Migrate `recurrence_materialization.rs` and `availability_projection.rs` next.
4. Migrate `planning_profile.rs` last — largest and most complex.
5. Update test files to match.
6. Run `cargo test --workspace --all-features` after each file to catch regressions early.
7. Remove `chrono` and `chrono-tz` from `Cargo.toml:49-50`.

---

## VD-06 — Reclassify flagged schema tables

**Migration files:**
- `migrations/0020_vel_self_metrics.sql` — creates `vel_self_metrics`
- `migrations/0033_storage_backup_foundation.sql` — creates `storage_targets` and `verification_records`

**Severity:** MEDIUM — stale audit language made active/planned schema look disposable
**Effort:** 30 min to verify + documentation cleanup
**Risk:** LOW — no schema migration is required

### Finding

Phase 107 re-verification found the original orphaned-table claim was too broad. `storage_targets` is active in the backup repositories and later backup job schema. `verification_records` and `vel_self_metrics` do not currently have Rust repositories, but they are planned foundation schema for backup verification/trust and self-awareness/reflective tuning respectively. They are not safe to drop under the Phase 107 ownership gate.

| Table | Created in | Classification | Decision |
|-------|------------|----------------|----------|
| `storage_targets` | `0033_storage_backup_foundation.sql` | Active backup foundation | Keep; runtime backup repositories and `v0_backup_jobs` depend on it |
| `verification_records` | `0033_storage_backup_foundation.sql` | Planned backup verification/trust foundation | Keep; no active repository yet, but aligned with backup integrity and restore-planning direction |
| `vel_self_metrics` | `0020_vel_self_metrics.sql` | Planned self-awareness / reflective-tuning substrate | Keep; no active repository yet, but aligned with self-awareness contracts and nudge effectiveness tracking |

Tables that were initially suspected but are **actually in use** (false positives):
- `event_log` — used in `vel-storage/src/repositories/chat_repo.rs:479–519`
- `interventions` — used in `chat_repo.rs:282–447`
- `planning_constraints` — used in `planning_profiles_repo.rs:32–258`
- `routine_blocks` — Rust field name only, not a table name; lives in `planning_profile` JSON column
- `vel_sim` — used by `veld-evals` crate

### Fix

1. Keep all three tables.
2. Treat `storage_targets` as active backup infrastructure when touching backup or restore code.
3. Treat `verification_records` and `vel_self_metrics` as planned foundation schema. Before adding new tables in those areas, either use these contracts or update their owner docs.
4. Only create a future drop migration if a table fails both checks: no live-code use and no Master Plan or accepted architecture ownership.

---

## VD-07 — Split `app.rs` route registration

**File:** `crates/veld/src/app.rs` — **12,842 lines**
**Severity:** LOW — organizational; no correctness impact
**Effort:** ~4 hours
**Risk:** NONE — pure file split, no logic change

### Finding

`app.rs` is the daemon's assembly root and currently contains:
- 82 `pub mod` service module declarations (lines ~1–200)
- `public_routes()` function — ~60 lines
- `operator_authenticated_routes()` function — ~200 lines
- `worker_authenticated_routes()` function — ~80 lines
- Error handling glue (`AppError`, `IntoResponse` impls) — ~100 lines
- 1,000+ lines of integration test fixtures and `#[tokio::test]` functions

This means a search for route definitions requires scanning 12K lines. Test fixture changes intermingle with production code.

### Fix

Proposed split:

```
crates/veld/src/
  app.rs                    → keep: build_app(), AppState wiring, module declarations
  routes/mod.rs             → new: re-exports all route modules (already exists, expand)
  app/public_routes.rs      → new: move public_routes() fn here
  app/operator_routes.rs    → new: move operator_authenticated_routes() fn here
  app/worker_routes.rs      → new: move worker_authenticated_routes() fn here
  app/error.rs              → new: AppError, IntoResponse impls
  tests/integration/        → new: move all #[tokio::test] fixtures here
```

`app.rs` should reduce to ~200 lines after the split.

---

## VD-08 — Split `now.rs` service

**File:** `crates/veld/src/services/now.rs` — **4,240 lines**
**Severity:** LOW — organizational; the domain complexity is real, the file structure is not
**Effort:** ~8 hours
**Risk:** NONE — pure file split

### Finding

`now.rs` is the "Now surface" service — the most-used read path in the daemon. It currently houses:
- 36 output DTOs (structs for the Now response shape)
- 71+ functions across: data extraction, lane building, nudge generation, task prioritization, formatting
- Internal `#[cfg(test)]` unit tests

All correct code. The problem is navigability: finding the function that builds a specific lane requires knowing line numbers, and changing the task-lane logic risks accidentally touching nudge logic 200 lines away.

### Fix

```
crates/veld/src/services/now/
  mod.rs          → public API: re-exports, build_now_output() entry point
  output.rs       → all 36 DTO structs
  extractors.rs   → raw data extraction functions (reading from AppState, context)
  builders.rs     → build_* functions (header, status_row, task_lane, nudge_bars, etc.)
  composition.rs  → assembly logic (how builders are called and combined)
  reducers.rs     → signal reduction / priority sorting logic
```

Maintain identical public API — only the file layout changes. No rename of `pub` items.

---

## VD-09 — Reorganize `vel-api-types/src/lib.rs`

**File:** `crates/vel-api-types/src/lib.rs` — **8,485 lines, ~288 structs**
**Severity:** LOW — IDE intellisense and search performance
**Effort:** ~6 hours
**Risk:** NONE — pure file split, all items re-exported from `lib.rs`

### Finding

All 288 HTTP DTOs live in one file. The DTOs themselves are necessary — the API surface is genuinely large. The organizational problem is that finding `CommandPlanStepData` requires knowing it's somewhere in 8,485 lines, and IDE symbol lookup returns 288 candidates with no grouping.

### Fix

```
crates/vel-api-types/src/
  lib.rs           → re-exports only: pub use commands::*; pub use captures::*; etc.
  responses.rs     → ApiResponse<T>, ApiError, standard envelope types
  commands.rs      → Command* DTOs (~60 types)
  captures.rs      → Capture* DTOs (~20 types)
  projects.rs      → Project* DTOs (~20 types)
  commitments.rs   → Commitment* DTOs (~25 types)
  execution.rs     → Execution*, handoff, context DTOs (~30 types)
  chat.rs          → Chat*, message, intervention, thread DTOs (~30 types)
  sync.rs          → Sync, cluster, node DTOs (~20 types)
  integrations.rs  → Integration*, connection DTOs (~20 types)
  settings.rs      → Settings*, profile DTOs (~15 types)
```

`lib.rs` becomes a re-export index. All existing consumers continue to work (`use vel_api_types::CommandPlanRequest` stays valid).

---

## VD-10 — Standardize error handling across layers

**Affected:** all layers — routes, services, repos
**Severity:** MEDIUM — inconsistency forces per-call `.map_err()` boilerplate and makes error tracing harder
**Effort:** ~8 hours
**Risk:** MEDIUM — touches every error path; test carefully

### Finding

Three distinct error patterns coexist:

**Pattern A — Route layer** (`Result<impl IntoResponse, AppError>`):
```rust
// routes/captures.rs
pub async fn create_capture(...) -> Result<Json<ApiResponse<CaptureData>>, AppError>
```

**Pattern B — Service layer** (`Result<T, ServiceError>`):
```rust
// services/command_lang.rs
pub async fn plan_command(...) -> Result<CommandPlanOutput, ServiceError>
```

**Pattern C — Repository layer** (raw `sqlx::Error`):
```rust
// vel-storage/src/repositories/captures_repo.rs
pub async fn insert(...) -> Result<(), sqlx::Error>
```

This creates a conversion chain at every boundary: `sqlx::Error → StorageError → ServiceError → AppError`. Each conversion is bespoke.

### Fix

Define a single `VelError` in `vel-core` (or a new `vel-error` crate):

```rust
#[derive(Debug, thiserror::Error)]
pub enum VelError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("validation: {0}")]
    Validation(String),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("internal: {0}")]
    Internal(String),
}
```

Repositories return `Result<T, VelError>`. Services return `Result<T, VelError>`. Routes map `VelError → AppError` (HTTP codes) in one place via `impl From<VelError> for AppError`.

This removes the layered conversion chain and makes `?` propagation work cleanly through all three tiers.

---

## VD-11 — Decide fate of `vel-sim` and `vel-agent-sdk`

**Files:**
- `crates/vel-sim/` — 16KB simulation harness
- `crates/vel-agent-sdk/` — 16KB agent SDK wrapper

**Severity:** LOW — no correctness issue; just crate count and CI overhead
**Effort:** 1 hour (decision + action)
**Risk:** NONE

### vel-sim

`vel-sim` is used by `veld-evals` (`crates/veld-evals/Cargo.toml:19`, `crates/veld-evals/src/lib.rs:14`). It is a legitimate crate supporting the evaluation framework. **Status: KEEP.**

No action needed — the earlier concern about orphaning was a false alarm.

### vel-agent-sdk

`crates/vel-agent-sdk/src/lib.rs` is a thin wrapper (~200 lines) around `vel-protocol`. Its only real consumer is `crates/veld/tests/agent_sdk.rs` (one integration test).

Two options:
1. **Keep and grow it** — if external agent authors are a near-term target, flesh out the SDK with connection pooling, retry logic, and type-safe capability builders.
2. **Inline the test fixture** — move the test helper structs into `tests/agent_sdk_sim.rs` directly, and remove the crate until there are real external consumers.

**Recommendation:** If no external SDK launch is planned in the next milestone, inline and remove. If planned, file a ticket to grow the SDK to be actually useful rather than leaving it as a thin proxy.

---

## What Is Warranted Complexity

For completeness, the following were reviewed and found to be **justified**:

| Item | Why it's fine |
|------|--------------|
| `vel-api-types` size (288 DTOs) | One DTO per endpoint field is correct; size = API surface |
| `services/now.rs` logic depth | "Now" surface is genuinely the most complex read in the system |
| `services/command_lang.rs` (2,142 lines) | NL intent parsing + safety checks + execution planning — real domain |
| `services/client_sync.rs` (2,770 lines) | Distributed sync protocol + cluster discovery — real domain |
| 46 repositories | 50 tables → 46 repos is correctly scoped; no trivially thin repos |
| `LlmProvider` trait | 6 implementations — legitimate polymorphism |
| `SignalReducer` trait | 4 implementations — legitimate polymorphism |
| All 50 migrations | Every migration in use or recently dropped via `event_log`/`interventions` |
| Auth middleware pattern | Token-class enforcement is correct for multi-role API |

---

## Recommended Execution Order

### Immediate (< 2h combined)
1. **VD-01** — Remove `CapabilityResolver` trait → function
2. **VD-02** — Remove `ToolRunner` trait → function
3. **VD-03** — Delete dead policy structs

### Short-term
4. **VD-04** — Lift blanket `#![allow(dead_code)]` suppressions (do per crate, test between each)
5. **VD-06** — Drop orphaned DB tables (verify + migration)
6. **VD-05** — Complete chrono → time migration (do after VD-04 so warnings are visible)

### Polish (next cleanup sprint)
7. **VD-10** — Standardize error handling
8. **VD-07** — Split `app.rs`
9. **VD-09** — Reorganize `vel-api-types`
10. **VD-08** — Split `now.rs`
11. **VD-11** — Decide vel-agent-sdk fate
