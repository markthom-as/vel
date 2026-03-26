# Codebase Concerns

**Analysis Date:** 2026-03-22

## Tech Debt

**Oversized settings surface with dead legacy branch:**
- Files: `clients/web/src/views/settings/SettingsPage.tsx`, `clients/web/src/views/settings/SettingsPage.test.tsx`, `.planning/STATE.md`
- Issue: `SettingsPage.tsx` is 6,473 lines and still contains a second unreachable `return` branch after the live compact layout. `.planning/STATE.md` calls this out as an active Phase 54 cleanup item.
- Why: The compact left-rail shell was layered on top of the previous settings implementation to land UI conformance first.
- Impact: Review and edits are error-prone because developers can change dead JSX and see no effect. The file also mixes navigation, fetch orchestration, mutation handlers, and rendering for multiple product seams.
- Fix approach: Remove the legacy branch first, then split the file by owned sections such as profile, sync, integrations, and runtime settings under `clients/web/src/views/settings/`.

**Backend entrypoint and routing remain monolithic:**
- Files: `crates/veld/src/app.rs`, `crates/veld/src/middleware/mod.rs`, `crates/veld/src/routes/connect.rs`
- Issue: `crates/veld/src/app.rs` is 12,519 lines and still owns route assembly, exposure policy wiring, fallback handling, and a large in-file test suite.
- Why: Route growth happened inside the main app builder while auth hardening and feature work were landing in parallel.
- Impact: Small route changes have a large blast radius. It is easy to break exposure class assumptions or fallback behavior when editing unrelated endpoints.
- Fix approach: Keep adding new routes in dedicated route modules only, then extract app-level test helpers and exposure-gate tests out of `app.rs` before adding more route families.

**Settings and secret persistence are still generic JSON blobs:**
- Files: `crates/vel-storage/src/repositories/settings_repo.rs`, `crates/veld/src/services/integrations.rs`, `crates/veld/src/services/integrations_todoist.rs`
- Issue: Durable settings, including integration secret records such as `integration_google_calendar_secrets` and `integration_todoist_secrets`, are stored as JSON strings in the shared `settings` table.
- Why: This keeps the storage seam flexible while integrations are still evolving.
- Impact: There is no typed schema at the database edge, malformed JSON is silently coerced to `Null` on read, and secret-bearing records share the same generic persistence path as ordinary settings.
- Fix approach: Move secret-bearing settings to typed storage with explicit validation, and make malformed JSON a surfaced error instead of `JsonValue::Null`.

## Known Bugs

**Legacy settings JSX is still present but unreachable:**
- Files: `clients/web/src/views/settings/SettingsPage.tsx`, `.planning/STATE.md`, `.planning/milestones/v0.4-phases/52-full-now-ui-conformance-implementation-chunk/52-VERIFICATION.md`
- Symptoms: Developers can edit the old tabbed settings UI block and nothing changes at runtime because the compact layout returns earlier in the component.
- Trigger: Any maintenance work that assumes the lower `return` block is still live.
- Workaround: Only touch the first rendered settings shell near the active `loading` guard and compact left-rail layout.
- Root cause: The legacy implementation was left below the new return as temporary cleanup debt.
- Blocked by: Phase 54 cleanup completion.

**Connect transport is only partially exposed:**
- Files: `docs/MASTER_PLAN.md`, `crates/veld/src/app.rs`, `crates/veld/src/routes/connect.rs`, `crates/veld/tests/connect_runtime.rs`
- Symptoms: `/v1/connect/instances` works for operator-authenticated runtime control, but `/v1/connect` and `/v1/connect/worker` still resolve to the deny-by-default future-external fallback.
- Trigger: Clients or docs that expect the broader Phase 2 connect/auth transport rather than the currently shipped internal runtime control seam.
- Workaround: Use the shipped operator-authenticated `/v1/connect/instances` routes only.
- Root cause: Phase 2 connect launch work was explicitly re-scoped in `docs/MASTER_PLAN.md`; the live tree keeps the public-facing connect roots closed.
- Blocked by: Later roadmap closure for external connect/auth transport.

## Security Considerations

**Local-friendly auth defaults remain permissive on loopback:**
- Files: `crates/veld/src/middleware/mod.rs`, `crates/veld/src/app.rs`
- Risk: When `VEL_STRICT_HTTP_AUTH` is unset and no operator or worker token is configured, operator-authenticated and worker-authenticated routes still allow loopback requests.
- Current mitigation: Non-loopback requests are denied without tokens, and explicit tokens are enforced when configured.
- Recommendations: Treat this as a development-only posture. For any shared workstation or exposed daemon, set `VEL_STRICT_HTTP_AUTH=1` and both auth tokens before opening the HTTP surface.

**CORS remains fully permissive:**
- Files: `crates/veld/src/app.rs`
- Risk: `CorsLayer::permissive()` keeps browser-origin restrictions wide open. Combined with local-friendly auth defaults, this increases the damage of accidental exposure or unsafe local browsing contexts.
- Current mitigation: The daemon is designed as a local-first authority process, and future-external routes stay forbidden by default.
- Recommendations: Replace permissive CORS with an explicit allowlist when Vel is run outside a strictly local environment.

**Integration secrets are durable application data, not mediated credentials:**
- Files: `crates/veld/src/services/integrations.rs`, `crates/veld/src/services/integrations_todoist.rs`, `crates/vel-storage/src/repositories/settings_repo.rs`
- Risk: Google Calendar and Todoist secret material is persisted through generic settings writes, widening the set of code paths that can mishandle or mis-serialize credentials.
- Current mitigation: Web responses expose omission flags and public settings types rather than secret values.
- Recommendations: Keep decrypted secrets confined to the narrowest runtime boundary, audit all settings read/write paths touching `*_secrets` keys, and move toward brokered or OS-backed secret storage.

## Performance Bottlenecks

**SQLite is intentionally single-connection:**
- Files: `crates/vel-storage/src/infra.rs`
- Problem: `connect_pool()` sets `max_connections(1)`.
- Measurement: Hard cap of one SQL connection per process.
- Cause: The codebase optimizes for deterministic local SQLite behavior and WAL mode rather than concurrent writer throughput.
- Improvement path: Keep this for local correctness, but any higher-concurrency sync or multi-worker expansion needs a dedicated plan for queueing, sharding, or a different storage backend.

**Settings page fan-in is heavy and centralized:**
- Files: `clients/web/src/views/settings/SettingsPage.tsx`, `clients/web/src/views/settings/SettingsPage.test.tsx`
- Problem: One component loads and mutates settings, linking, cluster, integrations, runs, logs, inspect data, and planning profile state.
- Measurement: `SettingsPage.tsx` is 6,473 lines; its test only covers two compact-shell happy-path cases.
- Cause: The page is still the operator catch-all surface for several runtime seams.
- Improvement path: Break data loading into section-specific hooks and render subtrees lazily by active section so inactive panels do not carry the same render and state-management weight.

## Fragile Areas

**`SettingsPage` refactors are easy to break silently:**
- Files: `clients/web/src/views/settings/SettingsPage.tsx`, `clients/web/src/views/settings/SettingsPage.test.tsx`
- Why fragile: The file mixes multiple async resources, optimistic UI updates, and an unreachable legacy branch in a single component.
- Common failures: Editing dead JSX, forgetting to invalidate the right query key, and introducing section regressions that current tests do not catch.
- Safe modification: Remove dead code before feature work, isolate each section behind a helper hook or child component, and add a focused test for the exact section being changed.
- Test coverage: Only shallow layout and integrations-section smoke tests exist in `clients/web/src/views/settings/SettingsPage.test.tsx`.

**Exposure gating lives at a high-blast-radius seam:**
- Files: `crates/veld/src/app.rs`, `crates/veld/src/middleware/mod.rs`
- Why fragile: Public, operator-authenticated, worker-authenticated, and future-external routes are assembled in one place, then wrapped by policy middleware.
- Common failures: Mounting a route under the wrong exposure class, assuming `/v1/connect` and `/v1/connect/instances` behave the same, or weakening loopback-only assumptions without updating policy.
- Safe modification: Add or update route tests in `crates/veld/src/app.rs` before moving endpoints between route groups, and verify both strict-auth and non-strict loopback behavior.
- Test coverage: Coverage exists, but it is embedded inside `crates/veld/src/app.rs`, which makes the auth contract harder to review and maintain.

**Sync and authority state remain cognitively dense:**
- Files: `crates/veld/src/services/client_sync.rs`, `crates/veld/src/services/operator_queue.rs`, `crates/veld/tests/execution_routing.rs`
- Why fragile: Authority election data, worker capabilities, queue routing, and operator-facing evidence are all tightly coupled.
- Common failures: Breaking requested-capability routing, drifting authority metadata, or changing queue evidence shapes without updating downstream UI and API DTOs.
- Safe modification: Change one routing seam at a time, keep `vel-core` and `vel-api-types` mapping boundaries explicit, and verify affected flows with targeted `crates/veld/tests/` coverage.
- Test coverage: Targeted Rust integration tests exist, but there is no small architectural walkthrough next to the service code, so reasoning still depends on reading large files.

## Scaling Limits

**Single-node SQLite authority is the practical ceiling today:**
- Files: `crates/vel-storage/src/infra.rs`, `docs/MASTER_PLAN.md`
- Current capacity: One SQLite connection and one local authority process.
- Limit: Higher write concurrency and broader multi-client sync pressure will queue behind the single database connection and authority runtime.
- Symptoms at limit: Slower sync responsiveness, longer writeback or run-event persistence latency, and more pressure on route handlers that serialize through the same store.
- Scaling path: Keep the current local-first authority model for 0.4.x, but treat any multi-node or hosted expansion as storage-architecture work, not a small tuning task.

**WebSocket broadcast buffering is fixed-size:**
- Files: `crates/veld/src/app.rs`, `crates/veld/src/state.rs`
- Current capacity: Broadcast channels are created with fixed buffers of 64 messages.
- Limit: Bursty event streams can overrun slow subscribers.
- Symptoms at limit: Clients miss realtime updates and have to recover via query invalidation or reload paths.
- Scaling path: Add observable drop metrics and backpressure strategy before increasing event volume or client count.

## Dependencies at Risk

**Todoist API contract drift affects both sync and writeback paths:**
- Files: `crates/veld/src/services/integrations_todoist.rs`
- Risk: The code hardcodes both `https://api.todoist.com/api/v1` and `https://api.todoist.com/rest/v2`, so upstream API drift can break sync and writeback in different ways.
- Impact: Task ingestion, conflict handling, and Todoist-backed writeback proposals can fail without any local schema change.
- Migration plan: Keep provider calls isolated in `integrations_todoist.rs`, add explicit contract tests around response decoding, and be ready to collapse onto a single supported API family if Todoist retires one surface.

## Missing Critical Features

**GitHub writeback is still synthetic, not real external writeback:**
- Files: `crates/veld/src/services/integrations_github.rs`, `README.md`
- Problem: GitHub issue and comment flows create locally applied writeback records with synthetic issue numbers instead of performing a mediated outbound API call.
- Current workaround: Treat GitHub writeback as provenance-safe local scaffolding only.
- Blocks: Real repo writeback, trustworthy execution evidence, and end-to-end supervised automation against GitHub.
- Implementation complexity: Medium to high because it needs a real capability boundary, credential mediation, and execution-backed verification.

**Execution-backed Apple parity is still missing in this environment:**
- Files: `.planning/STATE.md`, `clients/apple/README.md`, `clients/apple/`
- Problem: The active project state explicitly says Apple/client parity is only source-level aligned here, and `clients/apple/` currently has no test files in-tree.
- Current workaround: Rely on shared Rust contracts plus web reference behavior.
- Blocks: Confident cross-surface milestone closeout and safe refactors that assume Apple behavior stays aligned.
- Implementation complexity: Medium because it needs runnable Apple-surface verification, not just source inspection.

## Test Coverage Gaps

**Settings regressions are under-tested relative to component scope:**
- Files: `clients/web/src/views/settings/SettingsPage.tsx`, `clients/web/src/views/settings/SettingsPage.test.tsx`
- What's not tested: Error states, dead-code cleanup safety, section-specific mutations, and concurrent loading or retry paths.
- Risk: A small settings change can break operator-critical controls without failing the current test suite.
- Priority: High
- Difficulty to test: The component currently owns too many independent data sources and mutation seams.

**Apple client coverage is effectively absent in-repo:**
- Files: `clients/apple/`, `.planning/STATE.md`
- What's not tested: Runtime parity and user-visible Apple shell behavior in this workspace.
- Risk: Source-level alignment can drift from real behavior without any local signal.
- Priority: High
- Difficulty to test: The required execution environment is not available through the current repository test setup.

**Malformed settings and secret-record corruption paths are not defended well enough:**
- Files: `crates/vel-storage/src/repositories/settings_repo.rs`, `crates/veld/src/services/integrations.rs`, `crates/veld/src/services/integrations_todoist.rs`
- What's not tested: Recovery behavior when persisted settings JSON is invalid or secret-bearing records are partially missing.
- Risk: Silent `Null` coercion or partial settings loss can degrade integrations in ways that are hard to diagnose.
- Priority: Medium
- Difficulty to test: The generic settings table makes corruption cases easy to create but currently too easy to ignore.

---

*Concerns audit: 2026-03-22*
*Update as issues are fixed or new ones discovered*
