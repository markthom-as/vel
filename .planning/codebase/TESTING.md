# Testing Patterns

**Analysis Date:** 2026-03-22

## Test Framework

**Runner:**
- Rust uses `cargo test` across the workspace declared in `Cargo.toml`. The top-level `Makefile` runs `cargo test --workspace --all-features` through `make test-api`.
- Web tests use `Vitest 2.1.8` with config in `clients/web/vitest.config.ts`.
- CI executes both stacks through `make ci` in `.github/workflows/ci.yml`, then runs `make smoke`, then an eval-fixture smoke command for `crates/veld-evals`.

**Assertion Library:**
- Rust uses built-in `assert!`, `assert_eq!`, and `expect(...)` patterns in `crates/veld/tests/*.rs`, `crates/vel-core/src/*`, and `crates/vel-storage/src/infra.rs`.
- Web tests use Vitest built-in `expect` plus `@testing-library/jest-dom` matchers wired in `clients/web/src/test/setup.ts`.

**Run Commands:**
```bash
make test                                      # Run Rust and web tests
make test-api                                  # Run cargo test --workspace --all-features
make test-web                                  # Run clients/web Vitest suite
cargo test --workspace --all-features          # Direct Rust suite
cargo test -p veld --test commitment_scheduling_api
cd clients/web && npm run test                 # Run all Vitest tests
cd clients/web && npm run test -- src/views/settings/SettingsPage.test.tsx
cd clients/web && npm run test:watch           # Interactive Vitest watch mode
make smoke                                     # Daemon/API/CLI smoke path
```

## Test File Organization

**Location:**
- Rust unit tests are usually inline under `#[cfg(test)] mod tests` in the source file, for example `crates/vel-core/src/context.rs`, `crates/vel-storage/src/infra.rs`, and `crates/vel-cli/src/commands/docs.rs`.
- Rust integration and route-flow tests live under `crates/veld/tests/`, for example `crates/veld/tests/commitment_scheduling_api.rs`, `crates/veld/tests/chat_grounding.rs`, and `crates/veld/tests/runtime_loops.rs`.
- Web tests are colocated under `clients/web/src/**` as `*.test.ts` or `*.test.tsx`, for example `clients/web/src/api/client.test.ts`, `clients/web/src/data/query.test.tsx`, and `clients/web/src/views/now/NowView.test.tsx`.

**Naming:**
- Rust integration tests use descriptive behavior files named after the surface under test, such as `planning_profile_api.rs`, `apple_voice_loop.rs`, and `backup_flow.rs` in `crates/veld/tests/`.
- Rust inline tests use behavior-first function names, often long and explicit, such as `context_migrator_parses_known_context_shape` in `crates/vel-core/src/context.rs`.
- Web tests use the source filename plus `.test.ts[x]`, with `describe('ComponentName' | 'feature name', ...)` headings matching the unit under test.

**Structure:**
```text
crates/
  vel-core/
    src/
      context.rs              # inline unit tests with #[cfg(test)]
  vel-storage/
    src/
      infra.rs                # inline async storage tests
  veld/
    tests/
      commitment_scheduling_api.rs
      chat_grounding.rs
      runtime_loops.rs

clients/web/
  src/
    api/
      client.test.ts
    data/
      query.test.tsx
      operator.test.ts
    shell/
      MainPanel/MainPanel.test.tsx
    views/
      now/NowView.test.tsx
      settings/SettingsPage.test.tsx
```

## Test Structure

**Suite Organization:**
```typescript
describe('SettingsPage', () => {
  beforeEach(() => {
    clearQueryCache()
    resetWsQuerySyncForTests()
    vi.mocked(client.apiGet).mockReset()
    vi.mocked(client.apiPatch).mockReset()
    vi.mocked(client.apiPost).mockReset()
  })

  it('renders a compact settings shell with a left tab rail', async () => {
    render(<SettingsPage onBack={() => {}} />)

    await waitFor(() => {
      expect(screen.getByRole('button', { name: /Profile/i })).toBeInTheDocument()
    })
  })
})
```

```rust
#[tokio::test]
async fn commitment_scheduling_apply_route_updates_commitment_and_thread_continuity() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());

    let apply_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/commitment-scheduling/proposals/thr_day_plan_apply_1/apply",
            None,
        ))
        .await
        .expect("apply response");

    assert_eq!(apply_response.status(), StatusCode::OK);
}
```

**Patterns:**
- Web tests use `describe` + `it`, with `beforeEach` or `afterEach` for cache reset and mock cleanup. See `clients/web/src/views/settings/SettingsPage.test.tsx`, `clients/web/src/views/now/NowView.test.tsx`, and `clients/web/src/api/client.test.ts`.
- Web component tests follow Testing Library patterns: `render`, `screen`, `fireEvent`, and `waitFor`.
- Hook tests use `renderHook`, as in `clients/web/src/data/query.test.tsx`.
- Rust async behavior tests use `#[tokio::test]` extensively for storage, route, and service flows.
- Rust tests usually construct a minimal in-memory app state or storage, perform one real operation, then assert status codes plus persisted state.

## Mocking

**Framework:**
- Web mocking uses `vi.mock(...)`, `vi.mocked(...)`, and `vi.spyOn(...)` from Vitest.
- Rust does not use a general mocking framework in the sampled tests. Instead, it builds small fake structs that implement traits, or it uses real in-memory storage and app state.

**Patterns:**
```typescript
vi.mock('../../api/client', () => ({
  apiGet: vi.fn(),
  apiPatch: vi.fn(),
  apiPost: vi.fn(),
}))

beforeEach(() => {
  vi.mocked(client.apiGet).mockReset()
  vi.mocked(client.apiGet).mockImplementation(async (path: string) => {
    if (path === '/api/settings') {
      return { ok: true, data: { timezone: 'America/Denver' }, meta: { request_id: 'req_settings' } } as never
    }
    throw new Error(`Unexpected GET ${path}`)
  })
})
```

```rust
struct MockChatProvider {
    requests: Arc<Mutex<Vec<LlmRequest>>>,
}

#[async_trait]
impl LlmProvider for MockChatProvider {
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
        // capture request and return deterministic tool call / completion
    }
}
```

**What to Mock:**
- Web tests mock API clients, websocket subscriptions, and view modules when the test targets composition rather than transport. Examples: `clients/web/src/views/settings/SettingsPage.test.tsx` and `clients/web/src/shell/MainPanel/MainPanel.test.tsx`.
- Web decoder tests stub `globalThis.fetch` rather than the higher-level client when verifying low-level HTTP behavior, as in `clients/web/src/api/client.test.ts`.
- Rust tests fake LLM providers and other trait-based collaborators when deterministic dialog behavior matters, as in `crates/veld/tests/chat_grounding.rs`.

**What NOT to Mock:**
- Do not mock typed decoders, query-cache logic, or route handlers when the test is supposed to validate the contract itself. `clients/web/src/types.test.ts` and `clients/web/src/data/query.test.tsx` exercise the real logic directly.
- Prefer real in-memory SQLite and real `axum` routers in Rust API tests instead of mocking storage or HTTP plumbing. `crates/veld/tests/commitment_scheduling_api.rs` is the preferred pattern.

## Fixtures and Factories

**Test Data:**
```typescript
function buildNowData(overrides: Record<string, unknown> = {}) {
  return {
    computed_at: 1710000000,
    timezone: 'America/Denver',
    // full canonical payload...
    ...overrides,
  }
}
```

```rust
async fn test_state() -> AppState {
    let storage = vel_storage::Storage::connect(":memory:")
        .await
        .expect("storage");
    storage.migrate().await.expect("migrations");
    let (broadcast_tx, _) = broadcast::channel(16);
    AppState::new(
        storage,
        AppConfig::default(),
        PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}
```

**Location:**
- Web tests usually keep fixture builders inside the test file, especially for route-shaped payloads. Examples: `buildClusterBootstrapFixture`, `buildClusterWorkersFixture`, and `buildNowData` in `clients/web/src/views/now/NowView.test.tsx`.
- Rust integration tests usually define local helper constructors like `test_state`, `request`, and `decode_json` in the same test file, as in `crates/veld/tests/commitment_scheduling_api.rs`.
- Shared non-test fixture assets exist for evals under `crates/veld-evals/fixtures/`, and CI exercises them with `cargo run -p veld-evals -- run --fixtures ...` from `.github/workflows/ci.yml`.

## Coverage

**Requirements:**
- No explicit line or branch coverage threshold is configured in the inspected Rust or web tooling.
- Quality enforcement is based on passing tests, `cargo fmt`, `cargo clippy -D warnings`, frontend ESLint, and smoke execution via `make ci`.

**Configuration:**
- Vitest is configured only with `jsdom`, `setupFiles`, and `include` in `clients/web/vitest.config.ts`; coverage reporting is not configured there.
- Rust coverage tooling such as `cargo-llvm-cov` is not wired into `Makefile` or `.github/workflows/ci.yml`.

**View Coverage:**
```bash
Not configured in-repo
```

## Test Types

**Unit Tests:**
- Rust unit tests live inline in library modules and validate parsing, serialization, migration helpers, and small domain rules, for example `crates/vel-core/src/context.rs`, `crates/vel-protocol/src/lib.rs`, and `crates/vel-config/src/lib.rs`.
- Web unit tests cover decoder logic, query state, and isolated components, for example `clients/web/src/types.test.ts`, `clients/web/src/data/query.test.tsx`, and `clients/web/src/core/Button/Button.test.tsx`.

**Integration Tests:**
- Rust integration tests are a major pattern in `crates/veld/tests/`. They build real app state, issue HTTP requests through `tower::ServiceExt::oneshot`, and assert persisted outcomes and transport payloads.
- Web integration-style tests render a live component with mocked network edges and assert user-visible behavior, for example `clients/web/src/views/settings/SettingsPage.test.tsx`, `clients/web/src/views/inbox/InboxView.test.tsx`, and `clients/web/src/views/context/ContextPanel.test.tsx`.

**E2E Tests:**
- Browser E2E tooling such as Playwright is not configured in the inspected repo.
- The nearest repo-level end-to-end checks are CLI/API smoke scripts invoked by `make smoke` and the eval fixture smoke step in `.github/workflows/ci.yml`.

## Common Patterns

**Async Testing:**
```typescript
it('shares one in-flight fetch across subscribers for the same key', async () => {
  const first = renderHook(() => useQuery(key, fetcher))
  const second = renderHook(() => useQuery(key, fetcher))

  await waitFor(() => {
    expect(first.result.current.data).toEqual(['value'])
    expect(second.result.current.data).toEqual(['value'])
  })
})
```

```rust
#[tokio::test]
async fn wal_mode_enabled_for_file_db() {
    let pool = connect_pool(&db_path).await.expect("pool opens");
    let row: (String,) = sqlx::query_as("PRAGMA journal_mode")
        .fetch_one(&pool)
        .await
        .expect("pragma query");
    assert_eq!(row.0, "wal");
}
```

**Error Testing:**
```typescript
await expect(
  apiPost('/v1/runs/run_1', {})
).rejects.toThrow('API 409: run cannot be retried automatically')
```

```rust
let apply_response = app
    .clone()
    .oneshot(request(
        "POST",
        "/v1/commitment-scheduling/proposals/thr_reflow_apply_missing/apply",
        None,
    ))
    .await
    .expect("apply response");
assert_eq!(apply_response.status(), StatusCode::NOT_FOUND);
```

**Snapshot Testing:**
- Snapshot matcher usage was not detected in `clients/web/src/**` or `crates/**`.
- Do not introduce snapshot-heavy tests by default; the current repo favors explicit contract and UI assertions.

---

*Testing analysis: 2026-03-22*
