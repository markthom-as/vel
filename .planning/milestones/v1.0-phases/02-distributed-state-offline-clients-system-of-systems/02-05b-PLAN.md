---
phase: 02-distributed-state-offline-clients-system-of-systems
plan: 05b
type: execute
wave: 4
depends_on:
  - 02-02
  - 02-03
  - 02-04
  - 02-05
files_modified:
  - crates/veld/src/routes/node_pairing.rs
  - crates/veld/src/routes/mod.rs
  - crates/veld/src/services/node_pairing.rs
  - crates/veld/src/services/mod.rs
  - crates/vel-storage/src/repositories/pairing_tokens_repo.rs
  - crates/vel-storage/src/repositories/mod.rs
  - crates/vel-storage/src/lib.rs
  - migrations/0014_pairing_tokens.sql
  - crates/veld/src/app.rs
autonomous: true
requirements:
  - CONN-03

must_haves:
  truths:
    - "An operator can call POST /api/node/pair/issue and receive a short-lived scoped pairing token"
    - "A pairing token expires after its TTL and is rejected on redeem attempts after expiry"
    - "Tokens are scoped to 'node:link' and cannot be reused for other operations"
    - "The pairing token flow integrates with the CLI vel node link command (implemented in 02-05)"
  artifacts:
    - path: "migrations/0014_pairing_tokens.sql"
      provides: "pairing_tokens table with token, scope, expires_at, used_at, issued_at"
      contains: "CREATE TABLE pairing_tokens"
    - path: "crates/vel-storage/src/repositories/pairing_tokens_repo.rs"
      provides: "repository: insert_pairing_token, get_pairing_token, mark_token_used, expire_stale_tokens"
      contains: "pub(crate) async fn insert_pairing_token"
    - path: "crates/veld/src/services/node_pairing.rs"
      provides: "issue_pairing_token service returning PairingTokenRecord (domain type)"
      contains: "pub async fn issue_pairing_token"
    - path: "crates/veld/src/routes/node_pairing.rs"
      provides: "POST /api/node/pair/issue route handler mapping PairingTokenRecord to DTO"
      contains: "pub async fn issue_pairing_token"
  key_links:
    - from: "crates/veld/src/routes/node_pairing.rs"
      to: "crates/veld/src/services/node_pairing.rs"
      via: "service call in route handler"
      pattern: "node_pairing::issue_pairing_token"
    - from: "crates/veld/src/services/node_pairing.rs"
      to: "crates/vel-storage/src/repositories/pairing_tokens_repo.rs"
      via: "storage call"
      pattern: "storage.insert_pairing_token"
    - from: "crates/veld/src/app.rs"
      to: "crates/veld/src/routes/node_pairing.rs"
      via: "route registration"
      pattern: "routes::node_pairing"
---

<objective>
SP3 Lane A (veld backend) — Pairing token issue/redeem backend for `POST /api/node/pair/issue` (ticket 012, CONN-03). This plan provides the veld-side implementation that the `vel node link` CLI flow (implemented in plan 02-05) calls.

Purpose: CONN-03 requires short-lived scoped pairing tokens that fail closed on expiry. This is the backend half of the pairing flow — without it, the CLI and web pairing UI from 02-05 have no endpoint to call.

Output: `pairing_tokens` storage table, PairingTokenRepository, node_pairing service (returns domain type), POST /api/node/pair/issue route handler (maps domain type to DTO), route registered in app.rs.
</objective>

<execution_context>
@/home/jove/.claude/get-shit-done/workflows/execute-plan.md
@/home/jove/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/phases/02-distributed-state-offline-clients-system-of-systems/02-CONTEXT.md
@docs/tickets/phase-2/012-tester-readiness-onboarding.md
@.planning/phases/02-distributed-state-offline-clients-system-of-systems/02-01-SUMMARY.md
@.planning/phases/02-distributed-state-offline-clients-system-of-systems/02-03-SUMMARY.md
@.planning/phases/02-distributed-state-offline-clients-system-of-systems/02-05-SUMMARY.md

<interfaces>
<!-- Key types and patterns the executor needs. Extracted from codebase. -->

From crates/veld/src/services/client_sync.rs (heartbeat TTL pattern — replicate for pairing tokens):
```rust
// Worker heartbeat: expire_cluster_workers sets status to expired after TTL
// Pairing token TTL: 300 seconds (5 minutes), single-use, scoped to "node:link"
// Same pattern: issued_at + ttl_seconds = expires_at; reject if expires_at < now
```

From crates/veld/src/services/connect_lifecycle.rs (service layer pattern — returns domain type):
```rust
// Services return storage records (ConnectRunRecord), NEVER HTTP DTOs
// Route handlers do the record → DTO mapping
// Same pattern applies here: node_pairing service returns PairingTokenRecord
```

From crates/veld/src/routes/connect.rs (thin route handler pattern — from 02-03):
```rust
pub async fn launch_agent(
    State(state): State<AppState>,
    Json(payload): Json<ConnectLaunchRequest>,
) -> Result<Json<ApiResponse<ConnectLaunchResponse>>, AppError> {
    let record = connect_lifecycle::launch(&state.storage, &payload).await?;
    // DTO mapping here in route handler:
    let response = ConnectLaunchResponse { run_id: record.id.clone(), ... };
    Ok(Json(ApiResponse::success(response, Uuid::new_v4().to_string())))
}
```

From crates/veld/src/app.rs (route registration — operator_authenticated_routes):
```rust
fn operator_authenticated_routes() -> Router<AppState> {
    Router::new()
        // existing routes...
        // Add: .route("/api/node/pair/issue", post(routes::node_pairing::issue_pairing_token))
}
```

From crates/vel-api-types/src/lib.rs (response DTO pattern):
```rust
pub struct ApiResponse<T> { pub ok: bool, pub data: Option<T>, ... }
// Add new DTO: NodePairIssueResponse { token: String, expires_at: i64, display_url: String }
// This matches the TypeScript NodePairIssueResponse in clients/web/src/types.ts (added in 02-05)
```

From migrations/0013_sync_ordering.sql (numbering — next is 0014):
```sql
-- Next migration must be 0014_pairing_tokens.sql
```
</interfaces>
</context>

<tasks>

<task type="auto" tdd="true">
  <name>Task 1: Pairing tokens storage — migration and repository</name>
  <files>migrations/0014_pairing_tokens.sql, crates/vel-storage/src/repositories/pairing_tokens_repo.rs, crates/vel-storage/src/repositories/mod.rs, crates/vel-storage/src/lib.rs</files>

  <read_first>
    - migrations/0013_sync_ordering.sql (see migration structure — next is 0014)
    - crates/vel-storage/src/repositories/connect_runs_repo.rs (repository pattern written in 02-03)
    - crates/vel-storage/src/repositories/mod.rs (how repos are registered)
    - crates/vel-storage/src/lib.rs (Storage struct — where to expose new repo methods)
    - docs/tickets/phase-2/012-tester-readiness-onboarding.md (authoritative spec)
  </read_first>

  <behavior>
    Migration creates `pairing_tokens` table with columns:
    - `token TEXT PRIMARY KEY NOT NULL` — cryptographically random token string
    - `scope TEXT NOT NULL` — capability scope (e.g., "node:link")
    - `issued_at INTEGER NOT NULL` — unix timestamp of issuance
    - `expires_at INTEGER NOT NULL` — unix timestamp of expiry
    - `used_at INTEGER` — unix timestamp when redeemed (NULL if not yet used)
    - `display_url TEXT NOT NULL` — human-readable URL for out-of-band display

    PairingTokenRecord struct:
    - Fields mirror all table columns

    Repository functions:
    - `insert_pairing_token(token, scope, issued_at, expires_at, display_url)` — inserts new token
    - `get_pairing_token(token)` — returns Option<PairingTokenRecord>
    - `mark_token_used(token, used_at)` — sets used_at; errors if already used or expired
    - `expire_stale_tokens(now_ts)` — deletes or marks expired tokens; returns count

    Tests (TDD):
    - Test: insert_pairing_token → get_pairing_token returns same data with used_at=None
    - Test: mark_token_used sets used_at on a fresh token
    - Test: mark_token_used on an already-used token returns error (single-use enforcement)
    - Test: get_pairing_token on non-existent token returns None
    - Test: expire_stale_tokens removes tokens where expires_at < now_ts, leaves fresh tokens
  </behavior>

  <action>
    **1. Create `migrations/0014_pairing_tokens.sql`:**
    ```sql
    CREATE TABLE IF NOT EXISTS pairing_tokens (
        token TEXT PRIMARY KEY NOT NULL,
        scope TEXT NOT NULL,
        issued_at INTEGER NOT NULL,
        expires_at INTEGER NOT NULL,
        used_at INTEGER,
        display_url TEXT NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_pairing_tokens_expires ON pairing_tokens (expires_at);
    ```

    **2. Create `crates/vel-storage/src/repositories/pairing_tokens_repo.rs`:**
    - Define `PairingTokenRecord { token, scope, issued_at, expires_at, used_at, display_url }` struct with `Debug, Clone`
    - Implement all 4 functions using `sqlx::query!` with compile-time verification
    - Write `#[cfg(test)]` block with 5 tests listed in behavior section using `SqlitePool::connect(":memory:")` and migration application

    **3. Register in `crates/vel-storage/src/repositories/mod.rs`:**
    - Add `pub(crate) mod pairing_tokens_repo;`

    **4. Expose on `Storage` in `crates/vel-storage/src/lib.rs`:**
    - Add pub methods: `pub async fn insert_pairing_token(...)`, `pub async fn get_pairing_token(...)`, `pub async fn mark_pairing_token_used(...)`, `pub async fn expire_stale_pairing_tokens(...)` — each delegates to the repo function with `&self.pool`
  </action>

  <verify>
    <automated>cargo test -p vel-storage 2>&1 | grep -E "pairing_token|FAILED" | head -20</automated>
  </verify>

  <done>pairing_tokens migration created. PairingTokenRecord struct and 4 repository functions implemented. 5 tests passing. Storage pub API updated.</done>
</task>

<task type="auto" tdd="true">
  <name>Task 2: Pairing token service, DTO, route handler, and app.rs registration</name>
  <files>crates/veld/src/services/node_pairing.rs, crates/veld/src/services/mod.rs, crates/vel-api-types/src/lib.rs, crates/veld/src/routes/node_pairing.rs, crates/veld/src/routes/mod.rs, crates/veld/src/app.rs</files>

  <read_first>
    - crates/veld/src/services/connect_lifecycle.rs (service returns domain type pattern — replicate exactly)
    - crates/veld/src/routes/connect.rs (thin route handler with DTO mapping in handler, not service)
    - crates/vel-api-types/src/lib.rs (existing DTO patterns)
    - crates/veld/src/app.rs (operator_authenticated_routes for route registration)
    - crates/veld/src/services/mod.rs (register new service module)
    - crates/veld/src/routes/mod.rs (register new route module)
    - docs/tickets/phase-2/012-tester-readiness-onboarding.md (authoritative spec)
  </read_first>

  <behavior>
    NodePairIssueResponse DTO (add to vel-api-types):
    - `token: String` — the short-lived pairing token
    - `expires_at: i64` — unix timestamp of expiry
    - `display_url: String` — human-readable URL for the pairing device to enter manually

    NodePairIssueRequest DTO (add to vel-api-types):
    - `scope: String` — must be "node:link" (validate; reject other scopes)
    - `ttl_seconds: Option<u32>` — defaults to 300 (5 minutes)

    Service behavior tests:
    - Test: issue_pairing_token returns PairingTokenRecord with scope="node:link", expires_at = now + 300
    - Test: issue_pairing_token with scope != "node:link" returns AppError::bad_request
    - Test: issued token is retrievable via storage.get_pairing_token(token)
    - Test: token expires_at is within 1 second of now + ttl_seconds
  </behavior>

  <action>
    **LAYERING RULE (CLAUDE.md):** Service functions return `PairingTokenRecord` (vel-storage domain type). The route handler does `PairingTokenRecord → NodePairIssueResponse` mapping. Services MUST NOT return HTTP DTOs.

    **1. Add DTOs to `crates/vel-api-types/src/lib.rs`:**
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NodePairIssueRequest {
        pub scope: String,
        pub ttl_seconds: Option<u32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NodePairIssueResponse {
        pub token: String,
        pub expires_at: i64,
        pub display_url: String,
    }
    ```

    **2. Create `crates/veld/src/services/node_pairing.rs`:**

    ```rust
    const ALLOWED_SCOPES: &[&str] = &["node:link"];
    const DEFAULT_TTL_SECONDS: u32 = 300;  // 5 minutes

    pub async fn issue_pairing_token(
        storage: &Storage,
        req: &NodePairIssueRequest,
        base_url: &str,
    ) -> Result<PairingTokenRecord, AppError>
    ```
    - Validate `req.scope` is in ALLOWED_SCOPES; reject with `AppError::bad_request("scope not allowed: {scope}")` for unknown scopes
    - Generate a cryptographically random token using `uuid::Uuid::new_v4().to_string()` (no dashes: `.simple().to_string()` for compactness)
    - `now = OffsetDateTime::now_utc().unix_timestamp()`
    - `ttl = req.ttl_seconds.unwrap_or(DEFAULT_TTL_SECONDS) as i64`
    - `expires_at = now + ttl`
    - `display_url = format!("{}/pair?token={}", base_url, token)` (base_url from AppState config sync_base_url)
    - Call `storage.insert_pairing_token(&token, &req.scope, now, expires_at, &display_url)`
    - Return `storage.get_pairing_token(&token).await?.ok_or(AppError::internal("token insert failed"))`

    Write `#[cfg(test)]` block with 4 tests listed in behavior section.

    **3. Register in `crates/veld/src/services/mod.rs`:**
    - Add `pub mod node_pairing;`

    **4. Create `crates/veld/src/routes/node_pairing.rs`** with thin route handler:

    ```rust
    use vel_api_types::{ApiResponse, NodePairIssueRequest, NodePairIssueResponse};
    use crate::{services::node_pairing, state::AppState, errors::AppError};

    // POST /api/node/pair/issue — OperatorAuthenticated
    pub async fn issue_pairing_token(
        State(state): State<AppState>,
        Json(payload): Json<NodePairIssueRequest>,
    ) -> Result<Json<ApiResponse<NodePairIssueResponse>>, AppError> {
        let base_url = &state.config.sync_base_url;  // read sync_base_url from AppState config
        let record = node_pairing::issue_pairing_token(&state.storage, &payload, base_url).await?;
        // DTO mapping is the route handler's responsibility (CLAUDE.md):
        let response = NodePairIssueResponse {
            token: record.token,
            expires_at: record.expires_at,
            display_url: record.display_url,
        };
        Ok(Json(ApiResponse::success(response, uuid::Uuid::new_v4().to_string())))
    }
    ```

    Read `crates/veld/src/state.rs` to find the correct field path for sync_base_url in AppState (it may be `state.config.cluster.sync_base_url` or similar — use whatever exists).

    **5. Register in `crates/veld/src/routes/mod.rs`:**
    - Add `pub mod node_pairing;`

    **6. Register route in `crates/veld/src/app.rs`:**
    - Add to `operator_authenticated_routes()`:
      ```rust
      .route("/api/node/pair/issue", post(routes::node_pairing::issue_pairing_token))
      ```
  </action>

  <verify>
    <automated>cargo test -p veld 2>&1 | grep -E "node_pairing|pairing|FAILED" | head -20</automated>
  </verify>

  <acceptance_criteria>
    - `grep "NodePairIssueRequest\|NodePairIssueResponse" /home/jove/code/vel/crates/vel-api-types/src/lib.rs` returns 2 matches
    - `grep "pub async fn issue_pairing_token" /home/jove/code/vel/crates/veld/src/services/node_pairing.rs` returns match
    - `grep "ALLOWED_SCOPES\|node:link" /home/jove/code/vel/crates/veld/src/services/node_pairing.rs` returns match
    - `grep "PairingTokenRecord" /home/jove/code/vel/crates/veld/src/services/node_pairing.rs` returns match (service returns domain type)
    - `grep "NodePairIssueResponse" /home/jove/code/vel/crates/veld/src/services/node_pairing.rs` returns 0 matches (DTO NOT in service)
    - `grep "NodePairIssueResponse" /home/jove/code/vel/crates/veld/src/routes/node_pairing.rs` returns match (DTO mapping in route handler)
    - `grep "api/node/pair/issue" /home/jove/code/vel/crates/veld/src/app.rs` returns match
    - `cargo test -p veld 2>&1 | grep "node_pairing" | grep -c "FAILED"` returns 0
    - `cargo test -p veld 2>&1 | grep "node_pairing" | grep -c "ok"` returns at least 4
    - `cargo test -p vel-storage 2>&1 | grep "pairing_token" | grep -c "FAILED"` returns 0
    - `cargo build -p veld 2>&1 | grep -c "^error"` returns 0
  </acceptance_criteria>

  <done>POST /api/node/pair/issue endpoint active and OperatorAuthenticated. node_pairing service issues short-lived scoped tokens (TTL 300s default, scope enforced to "node:link"). Service returns PairingTokenRecord (domain type); route handler maps to NodePairIssueResponse DTO. CONN-03 fully implemented. Full flow testable: vel node link CLI (02-05) calls this endpoint.</done>
</task>

</tasks>

<verification>
SP3 Lane A (veld backend) gate criteria:
1. `cargo test -p vel-storage 2>&1 | grep "pairing_token" | grep -c "ok"` returns at least 5
2. `cargo test -p veld 2>&1 | grep "node_pairing" | grep -c "ok"` returns at least 4
3. `cargo test -p veld 2>&1 | grep -c "FAILED"` returns 0
4. `grep "NodePairIssueResponse" crates/veld/src/services/node_pairing.rs` returns 0 (layering rule enforced)
5. `curl -X POST http://localhost:4130/api/node/pair/issue -H "Authorization: Bearer $TOKEN" -d '{"scope":"node:link"}' | jq .ok` returns true (manual smoke test with veld running)
</verification>

<success_criteria>
- pairing_tokens migration 0014 creates table with token, scope, issued_at, expires_at, used_at, display_url
- 4 repository functions: insert, get, mark_used, expire_stale
- node_pairing service: scope validation, token generation, TTL enforcement, returns PairingTokenRecord (NOT HTTP DTO)
- POST /api/node/pair/issue route: thin handler mapping PairingTokenRecord to NodePairIssueResponse, OperatorAuthenticated
- CONN-03 requirement closed: pairing tokens are short-lived, scoped, single-use, fail-closed on expiry
- `cargo test -p vel-storage && cargo test -p veld` all pass
</success_criteria>

<output>
After completion, create `.planning/phases/02-distributed-state-offline-clients-system-of-systems/02-05b-SUMMARY.md`
</output>
