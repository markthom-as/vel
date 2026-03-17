# Codebase Concerns

**Analysis Date:** 2026-03-17

## Error Handling & Panic Safety

**Widespread use of unwrap() and expect() in critical paths:**
- Issue: 2,559+ instances of `unwrap()`, `expect()`, `panic!()` across the codebase create runtime crash vectors
- Files affected:
  - `crates/vel-cli/src/command_lang/parse.rs` (14+ expect() calls in tests and parsing)
  - `crates/vel-cli/src/client.rs` (reqwest client initialization with expect())
  - `crates/vel-cli/src/commands/journal.rs` (response data extraction with expect())
  - `crates/veld/src/services/integrations.rs` (Todoist settings loading with expect())
  - `crates/vel-config/src/lib.rs` (serde_json serialization with unwrap())
- Impact: Unhandled panics in background workers, CLI commands, and service initialization can crash entire daemon
- Fix approach:
  - Replace test-only expect() with Result propagation; only use expect() in truly unreachable code with documentation
  - Convert service layer expect() calls to proper error types (Result<T, AppError>)
  - Add graceful degradation for optional settings (e.g., Todoist, Google Calendar)
  - Use `?` operator and error mapping instead of expect() in production code

**Missing error context in some service methods:**
- Issue: Many service methods return generic errors without context about what operation failed
- Files: `crates/veld/src/services/integrations.rs`, `crates/veld/src/services/inference.rs`, `crates/veld/src/services/suggestions.rs`
- Impact: Difficult to debug production failures; error messages lack actionable information
- Fix approach: Wrap errors with context using `context()` or `with_context()` from anyhow/thiserror

---

## Complexity & Maintainability

**Monolithic route and service files:**
- Issue: Single files exceed 1000+ lines with mixed responsibilities
- Files:
  - `crates/veld/src/app.rs` (11,036 lines) — combines route registration, auth middleware, exposure policy, tests
  - `crates/veld/src/services/client_sync.rs` (1,934 lines) — sync coordination with many struct definitions
  - `crates/veld/src/services/inference.rs` (1,796 lines) — complex state machine for context generation
  - `crates/veld/src/services/command_lang.rs` (1,671 lines) — command parsing and resolution
  - `crates/veld/src/worker.rs` (1,638 lines) — background job orchestration
  - `crates/vel-storage/src/db.rs` (1,777 lines) — Storage facade with all repository imports
- Impact: Difficult to review, test, and modify; high cognitive load for new contributors
- Fix approach:
  - Extract auth middleware and exposure gate logic from app.rs to separate module
  - Split inference.rs into domain logic + context builders + temporal windows
  - Move test helper constants from app.rs tests to separate fixture module
  - Refactor worker.rs to break loop runners into separate files per loop type
  - Consider extracting Storage facade methods into trait implementations

**Client-side component complexity:**
- Issue: `clients/web/src/components/SettingsPage.tsx` is 2,550 lines with dense state management
- Impact: Single component handles routing, integrations, settings, logs, runs — very hard to reason about state changes
- Fix approach:
  - Extract integration management to separate container component
  - Pull settings management into dedicated hooks
  - Separate each integration type (Google, Todoist, local sources) into sub-components

**vel-api-types.rs too large (2,173 lines):**
- Issue: Single file with all transport DTOs makes discovery and maintenance harder
- Fix approach: Organize by feature (e.g., captures.rs, commitments.rs, chat.rs, etc.)

---

## Test Coverage Gaps

**Insufficient integration tests:**
- Issue: Only 2 integration test files; most testing is unit-level
- Files: Most service behavior tested only via unit tests with mocked storage
- Impact: Route handlers, middleware, and cross-service flows not well-covered; integration bugs slip through
- Priority: High
- Fix approach:
  - Add integration tests for critical sync paths (client_sync, work_assignments)
  - Test auth middleware with various token scenarios
  - Add end-to-end tests for capture → inference → nudge flow
  - Create fixtures for common test data setup

**Web frontend test gaps:**
- Issue: SettingsPage.tsx (2,550 lines) has corresponding test at 1,681 lines but with limited edge case coverage
- Impact: Complex state interactions not validated; integration changes risk UI regressions
- Fix approach:
  - Expand SettingsPage.test.tsx with mocking for failed API calls, retry scenarios, concurrent syncs
  - Add visual regression tests for multi-integration state combinations

**No tests for error paths in sync and background workers:**
- Issue: Worker.rs (1,638 lines) has minimal test coverage for failure scenarios
- Impact: Background job failures may not be properly handled or reported
- Fix approach: Add tests for partial failures, retries, and deadline exceeded scenarios

---

## Fragile Areas & Coupling

**Todoist and Google Calendar integration tightly coupled:**
- Issue: Settings split across public/secret storage with expect() assumptions
- Files:
  - `crates/veld/src/services/integrations_todoist.rs:177-202` (expect() for settings loading)
  - `crates/veld/src/services/integrations_google.rs` (similar pattern)
- Why fragile: If settings table is corrupted or missing, daemon panics on startup
- Safe modification: Add fallback defaults, validate settings exist before expect()
- Test coverage: Missing tests for missing/corrupted settings

**Database schema assumptions embedded in Rust:**
- Issue: Hard-coded JSON keys and column names scattered across repository layer
- Files:
  - `crates/vel-storage/src/repositories/` (multiple files reference specific settings keys)
  - `crates/veld/src/services/operator_settings.rs:30` (HashMap iteration without type safety)
- Impact: Schema changes require careful code search and update
- Fix approach: Use constants for all magic strings (already partially done with TODOIST_SETTINGS_KEY)

**Web frontend API integration assumes exact response shape:**
- Issue: Types extracted from API without validation; missing fields cause silent failures
- Files: `clients/web/src/types.ts`, `clients/web/src/data/resources.ts`
- Impact: API changes may silently break UI without clear error messages
- Fix approach: Add runtime validation using Zod schemas or similar

---

## Performance Concerns

**Large data structure allocations in inference path:**
- Issue: `crates/veld/src/services/inference.rs` builds multiple HashMaps and vectors for context generation
- Impact: May be slow for large commitment/signal counts; runs on every evaluate loop
- Fix approach: Profile to identify allocations; consider lazy evaluation or streaming

**N+1 queries for integration operations:**
- Issue: Integration sync operations may load data inefficiently
- Files: `crates/veld/src/services/integrations.rs` (1,636 lines with multiple repository calls)
- Impact: Sync operations may be slow with many calendars, tasks, or messages
- Fix approach: Batch repository queries; use efficient indexing in database

**Web frontend state query triggers:**
- Issue: `clients/web/src/data/ws-sync.ts` and `clients/web/src/data/query.ts` may trigger excessive re-renders
- Impact: Sluggish UI during real-time sync with large result sets
- Fix approach: Add virtualization, pagination, or pagination for large lists

---

## Security Considerations

**Auth token passed in environment variables:**
- Risk: VEL_OPERATOR_API_TOKEN and VEL_WORKER_API_TOKEN in environment can be exposed via process listing or logs
- Files: `crates/veld/src/app.rs:37-38` (HttpExposurePolicy::from_env)
- Current mitigation: Tokens checked against exact string match (not timing-safe, but OK for local use)
- Recommendations:
  - Add warning if tokens not set in strict auth mode
  - Consider HMAC-based auth or OAuth for external clients
  - Never log token values (ensure logging doesn't expose them)

**CORS policy is permissive:**
- Risk: `CorsLayer::permissive()` in app.rs:472 allows any origin to make requests
- Files: `crates/veld/src/app.rs:472`
- Impact: Vulnerable to CSRF if used over HTTPS without additional protections
- Recommendation: Use explicit origin list for production; document local-only assumption

**Settings stored as JSON blobs in database:**
- Risk: Settings validation happens at application layer, not database layer
- Files: `crates/veld/src/services/operator_settings.rs`
- Impact: Invalid settings could corrupt state if hand-edited in database
- Recommendation: Add database constraints or JSON schema validation at insert/update

---

## Known Issues & Workarounds

**Todoist and Google Calendar disconnect doesn't fully clean up:**
- Symptom: Reconnecting immediately may fail or show stale data
- Files: `crates/veld/src/services/integrations_todoist.rs`, `crates/veld/src/services/integrations_google.rs`
- Workaround: Wait a few seconds between disconnect and reconnect
- Fix: Ensure all cached state is cleared on disconnect, add cache invalidation triggers

**Web frontend authentication token refresh not automatic:**
- Symptom: API calls fail with 401 after operator token changes
- Files: `clients/web/src/api/client.ts`
- Workaround: Reload browser tab
- Fix: Implement token refresh flow or session management

---

## Scaling Limits

**SQLite as single-writer bottleneck:**
- Current capacity: Suitable for single user/local node
- Limit: Concurrent writes will queue; not suitable for distributed cluster without replication
- Scaling path: Phase 2/3 should add WAL mode, consider RocksDB or Postgres for multi-node
- Status: Documented in MASTER_PLAN as Phase 2 distributed state work

**Broadcast channel with fixed buffer:**
- Issue: `tokio::sync::broadcast::channel(64)` in app.rs:421 has fixed 64-message capacity
- Impact: With many concurrent clients, messages may be dropped silently
- Fix: Monitor broadcast queue depth; increase buffer or add backpressure handling

**Web frontend renders entire state on every sync message:**
- Issue: `clients/web/src/data/ws-sync.ts` invalidates entire query on each update
- Impact: Can't handle thousands of items efficiently
- Fix: Implement incremental sync with delta updates

---

## Dependencies at Risk

**sqlx compile-time verification dependency on migrations:**
- Risk: sqlx::migrate! macro requires migrations folder at compile time; CI/build breaks if migrations change unexpectedly
- Impact: Contributors must run cargo check after schema changes; can cause CI flakes
- Mitigation: Migrations are numbered and idempotent; test suite catches schema issues
- Note: This is intentional design for safety; consider worth the friction

**Chrono dependency with security history:**
- Risk: Time parsing without strong validation could be attack vector
- Files: `crates/vel-core/src/`, `crates/veld/src/` (chrono used throughout)
- Current mitigation: Time data is internal; not exposed to untrusted input parsing
- Recommendation: Keep chrono/time dependencies updated

**JavaScript dependencies in web/node_modules:**
- Issue: 300K+ lines of dependencies not reviewed
- Impact: Typical supply chain risk; assumes npm ecosystem is trustworthy
- Mitigation: `package-lock.json` pins exact versions; regular npm audit recommended

---

## Missing Critical Features

**No built-in observability for daemon startup failures:**
- Problem: If daemon fails to start, errors may not be visible in logs
- Impact: Makes local development harder; production debugging difficult
- Fix: Add structured startup checks, health endpoint that validates all subsystems

**No rollback mechanism for migrations:**
- Problem: SQLite migrations are forward-only; no down migration support
- Impact: Schema mistakes require manual database recovery
- Fix: Add migration rollback support or separate down migrations

**Worker node discovery is manual:**
- Problem: Operator must manually configure worker node URLs in sync endpoints
- Impact: Doesn't scale; Phase 2 must add auto-discovery (MASTER_PLAN item)
- Current: Documented in docs/tickets/phase-2/

---

## Technical Debt Summary

| Area | Severity | Impact | Status |
|------|----------|--------|--------|
| Unwrap/panic overuse | High | Crash risk | Needs systematic refactor |
| File size/complexity | Medium | Maintainability | Needs breaking down |
| Integration tests | High | Quality regression risk | Needs expansion |
| Error context | Medium | Debugging difficulty | Gradual improvement |
| Todoist/Google fragility | Medium | Startup panic risk | Needs defaults |
| Web auth flow | Low | UX friction | Known limitation |
| SQLite scaling | Medium | Limits clustering | Phase 2 work |
| CORS permissive | Low | CSRF risk (local use) | Document assumption |
| Settings validation | Low | Data corruption risk | Add constraints |

---

*Concerns audit: 2026-03-17*
