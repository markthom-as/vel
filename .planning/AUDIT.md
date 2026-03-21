# Vel Codebase Audit

**Date:** 2026-03-20
**Scope:** Architecture, AI/LLM subsystem, test coverage, code quality, user action flows

---

## 1. Overall Architecture

### Crate Dependency Graph

```
vel-protocol          ← wire protocol types (Rust + TypeScript)
vel-core              ← domain types, invariants, NO transport, NO storage
vel-storage           ← SQLite repositories via sqlx (depends only on vel-core)
vel-api-types         ← HTTP transport DTOs (7,527 lines, single file)
vel-config            ← TOML config loading, model profile loading
vel-llm               ← provider-agnostic LLM abstraction
vel-agent-sdk         ← SDK client for external agent processes
vel-sim               ← test scenario replay harness (depends on veld)
veld-evals            ← eval fixture runner (uses vel-sim + vel-llm live)
veld                  ← daemon: axum routes → services → storage
vel-cli               ← CLI binary (vel)
```

The layering rule (vel-storage must NOT depend on vel-api-types) is respected in `Cargo.toml`. `veld` is the only crate that may depend on `vel-api-types`.

### What Each Crate Actually Contains

**`vel-core`** (~6,700 LOC): Domain types and invariants. Notably includes `operator_queue.rs` (922 LOC) and `run.rs` (843 LOC), which contain scoring constants and ranking logic. This is business logic leaking into the domain type crate — it should live in `veld/src/services/`.

**`vel-storage`** (~11,400 LOC across repositories): 37 repository files. All are pure SQL over sqlx. No business logic. The `semantic_memory_repo.rs` at 1,519 LOC is the most complex, implementing BM25-style lexical scoring inline. Layering is clean here.

**`vel-api-types`** (7,527 LOC, single file `lib.rs`): Monolithic DTO file. Every HTTP response shape lives here. It imports selectively from `vel-core` for ID types and enums, which is correct. However, a single 7,527-line file is unmaintainable at scale.

**`veld`** (largest crate):
- `src/app.rs`: 12,155 LOC — about 530 LOC of real routing logic plus 11,600 LOC of tests. The test suite is embedded in the production module rather than `tests/`.
- `src/services/`: 58 service files, ~33,800 LOC total
- `src/routes/`: 41 route files
- `src/adapters/`: 10 integration adapters

**`vel-llm`**: Clean, small (~700 LOC). Two provider implementations: `LlamaCppProvider` (OpenAI-compat) and `OpenAiOauthProvider` (wraps llama_cpp with localhost guard). `ProviderRegistry` and `Router` are straightforward.

**`vel-agent-sdk`**: Single file SDK for building agents that communicate via `vel-protocol`. Mostly envelope construction.

**`vel-sim` / `veld-evals`**: Scenario replay and fixture-based evals. These work by spinning up in-memory storage, seeding fixture data, and running context generation. The eval crate drives them with a JSON fixture format and optional LLM judge.

### Architectural Drift

The `crates/veld/src/app.rs` at 12,155 lines is the most egregious single-file issue, but 11,600+ of those lines are tests that belong in `crates/veld/tests/` or separate test modules. The production routing logic (lines 1–530) is well-structured.

There is a `packages/` directory at the root (`packages/vel-affect-core/`, `packages/vel-render-watch/`, `packages/vel-render-web/`, `packages/vel-visual-morphology/`, `packages/vel-protocol/`) that contains TypeScript source files but has NO `package.json` files and is NOT referenced by `clients/web`. These appear to be speculative UI packages that have been abandoned mid-development. They are orphaned dead code.

---

## 2. AI/LLM Subsystem

### `vel-llm` — The Provider Abstraction

**Location:** `crates/vel-llm/src/`

The design is correct and clean:
- `LlmProvider` trait: `generate()`, `health()`, `models()`
- `LlamaCppProvider`: calls local llama-server on OpenAI-compat `/v1/chat/completions`
- `OpenAiOauthProvider`: wraps `LlamaCppProvider` with a localhost-only guard (policy: never call remote OpenAI directly)
- `ProviderRegistry`: `HashMap<profile_id, Arc<dyn LlmProvider>>`
- `Router`: selects provider by `req.model_profile`, dispatches `generate()`

**Configuration path:** Model profiles live in `configs/models/*.toml` (read by `vel-config`). Routing (which profile ID maps to which task: `chat`, `codegen`, `summarize`, etc.) lives in `configs/models/routing.toml`. The `veld` main initializes the `ProviderRegistry` and `Router` from these configs, passing `Option<Arc<Router>>` into `AppState`.

**Key types:**
- `LlmRequest` has `model_profile: String` (not a model name — a profile alias). This is the right abstraction.
- `ResponseFormat::Text` vs `ResponseFormat::JsonObject` — capability checks are done at the provider level.
- Tool call round-tripping: the `LlmResponse` carries typed `Vec<ToolCall>`, arguments parsed from JSON string.

**Known issue in `LlamaCppProvider`:** Tool call argument parsing at line 291 of `llama_cpp.rs` silently falls back to `serde_json::Value::Null` on parse failure: `serde_json::from_str(&args_str).unwrap_or(serde_json::Value::Null)`. This will cause silent failures if a model emits malformed JSON arguments.

### Chat Assistant Service

**Location:** `crates/veld/src/services/chat/`

The chat subsystem is the only live LLM consumer. The flow:

1. User sends message → `POST /v1/chat/messages` or `POST /v1/chat/entry`
2. `services/chat/messages.rs` creates user message, routes to `generate_assistant_reply()`
3. `services/chat/assistant.rs` loads conversation history (last 50 messages), builds system prompt via `build_chat_grounding_prompt()`, calls `Router.generate()`
4. Tool-call loop: up to `MAX_CHAT_TOOL_ROUNDS = 3` rounds. If model calls tools, executes them, appends results as user message, re-calls.
5. On text response: persists assistant message, emits WebSocket event.

**Fallback behavior:** If primary profile fails with `Transport`, `Protocol`, `Backend`, or `Capability` error, falls back to `chat_fallback_profile_id`. Auth and Config errors are NOT retriable.

**Available chat tools** (`services/chat/tools.rs`):
- `vel_get_now` — reads current NowData
- `vel_search_memory` — semantic search
- `vel_get_recall_context` — builds bounded recall pack
- `vel_list_projects`
- `vel_list_people`
- `vel_list_open_commitments`
- `vel_get_daily_loop_status`
- `vel_list_threads`
- `vel_get_context_brief`

**Quality concern:** Tool results are rendered to a text string and appended as a `"user"` role message. This is a workaround for models that don't support proper tool result roles. It means multi-turn tool conversations look syntactically wrong for models that do support it. No configuration to switch between modes.

### Inference Engine

**Location:** `crates/veld/src/services/inference/`

**This is entirely rule-based, not LLM.** It reads signals and commitments, applies a reducer pipeline, and writes a typed `CurrentContextV1` to storage. The reducers:
- `ReducerRegistry` applies reducers in order
- Built-in reducers: `CalendarReducer`, `GitReducer`, `HealthReducer`, `MessagesReducer`
- The main `run_at()` function (~200 lines of orchestration) in `mod.rs` (1,788 LOC total) computes derived state before the reducer pipeline

The inference module has `#[cfg(test)]` with 20+ unit tests at the bottom of `mod.rs`, covering temporal window logic, commitment priority scoring, meds status, etc. This is the most thoroughly tested service in the codebase.

**Evaluate boundary is enforced:** Routes correctly only call `services::evaluate::run()` from `POST /v1/evaluate`. Explain and read routes do NOT call inference directly. This boundary is documented in `services/mod.rs` and `services/evaluate.rs`.

### Context Generation

**Location:** `crates/veld/src/services/context_generation.rs`

Generates `TodayContextData`, `MorningContextData`, `EndOfDayContextData` from an `OrientationSnapshot`. These are pure functions (`build_today_at`, `build_morning_at`, etc.). Not LLM — keyword extraction only (`extract_focus_candidates`, `extract_commitments`). Has unit tests.

### Synthesis

**Location:** `crates/veld/src/services/synthesis.rs`

Weekly and project synthesis. Creates a run, gathers data, writes a text artifact. **Not LLM** — assembles structured text from storage data. The artifacts are written to disk under `var/artifacts/synthesis/`.

### Agent Protocol / Connect Runtime

**Location:** `crates/veld/src/services/agent_protocol.rs`, `services/connect_runtime.rs`, `services/wasm_guest_runtime.rs`

These services handle external agent processes connecting via `vel-protocol`. `BrokerService` mediates capability grants. These paths are code-complete but have **no tests at all** and represent a significant regression risk area.

---

## 3. Test Coverage

### Rust Test Counts by Crate

| Crate | Test count |
|-------|-----------|
| `veld` | 515 |
| `vel-cli` | 88 |
| `vel-storage` | 57 |
| `vel-core` | 46 |
| `vel-api-types` | 23 |
| `vel-config` | 22 |
| `vel-protocol` | 4 |
| `vel-agent-sdk` | 4 |
| `veld-evals` | 3 |
| `vel-llm` | 2 |
| `vel-sim` | 1 |

**Total: ~765 test functions across 154 source files with tests.**

### Integration Tests in `crates/veld/tests/`

17 integration test files that spin up the full axum app against in-memory SQLite:
- `agent_grounding.rs` — `/v1/agent/inspect` endpoint
- `agent_sdk.rs` — SDK envelope construction
- `apple_behavior_summary.rs` — behavior summary API
- `apple_voice_loop.rs` — voice turn flow
- `backup_flow.rs` — backup create/inspect/verify
- `chat_assistant_entry.rs` — full chat assistant with mock LLM provider
- `chat_grounding.rs` — grounding prompt construction
- `commitment_scheduling_api.rs` — commitment scheduling proposals
- `connect_runtime.rs` — agent connect/launch
- `daily_loop_morning.rs` — morning overview session (3 tests)
- `daily_loop_standup.rs` — standup session
- `execution_routing.rs` — handoff CRUD + approve/reject (3 tests)
- `planning_profile_api.rs` — planning profile CRUD
- `runtime_loops.rs` — loop management
- `semantic_memory.rs` — semantic search provenance (2 tests)
- `suggestion_engine.rs` — suggestion evaluation
- `wasm_guest_runtime.rs` — WASM guest launch

The integration tests use real storage (`:memory:`), real axum routing, and mock LLM providers. The pattern is consistent and high quality. `chat_assistant_entry.rs` is particularly thorough — it verifies LLM requests are correctly built and sent.

### Coverage Gaps — Critical

**Services with zero tests:**
- `services/agent_grounding.rs` — builds `AgentInspectData` (used by `/v1/agent/inspect`)
- `services/agent_protocol.rs` — protocol handling for connected agents
- `services/apple_voice.rs` (1,153 LOC) — the entire iOS voice turn flow
- `services/backup.rs` (626 LOC) — backup state machine
- `services/commitment_scheduling.rs` — proposal generation
- `services/connect_runtime.rs` — agent connect lifecycle
- `services/context_runs.rs` (526 LOC) — run-backed context generation
- `services/daily_loop.rs` — session state machine (covered only via integration tests)
- `services/daily_loop_inputs.rs` — morning snapshot inputs
- `services/execution_routing.rs` (737 LOC) — handoff routing decisions
- `services/explain.rs` — explain endpoints
- `services/integration_runtime.rs` — integration sync orchestration
- `services/integrations_google.rs` — Google Calendar sync
- `services/journal.rs` — mood/pain journals
- `services/suggestions.rs` — suggestion evaluation
- `services/uncertainty.rs` — uncertainty record management
- `services/wasm_guest_runtime.rs` — WASM runtime

**Routes with zero tests (most route files have no tests):**
- `routes/captures.rs` — the primary capture creation path
- `routes/commitments.rs` — commitment CRUD
- `routes/daily_loop.rs` — daily loop session API
- `routes/execution.rs` — execution handoff API
- Almost all other route files

Most route testing is done through integration tests in `crates/veld/tests/`, not per-file. This is reasonable but means route-level validations (query param parsing, auth enforcement) are only tested when a matching integration test exists.

**Repositories with zero tests:**
- `captures_repo.rs` — the primary data store for the capture flow
- `chat_repo.rs` — conversation and message storage
- `threads_repo.rs`
- `suggestions_repo.rs`
- `runs_repo.rs`
- `settings_repo.rs`

### Frontend Tests (Web)

22 test files. Pattern: `vi.mock('../api/client')` for API calls, component rendering with `@testing-library/react`. Quality is adequate — tests verify correct API URLs and basic render behavior. Coverage is shallow on component business logic.

---

## 4. Code Quality Issues

### 4.1 `app.rs` at 12,155 Lines

`crates/veld/src/app.rs` has ~530 LOC of routing and ~11,600 LOC of tests. The tests should be extracted into `crates/veld/tests/app_routing.rs` or split by domain. The current state makes the file effectively unsearchable and adds compile time.

### 4.2 `vel-api-types/src/lib.rs` at 7,527 Lines

A monolithic DTO file. Every HTTP response shape, request shape, and their `From<>` impls live here. It imports `vel_core` types but does not add logic. This should be split by domain (e.g., `chat.rs`, `now.rs`, `execution.rs`). Low regression risk to refactor since it's pure data structs.

### 4.3 Double-Layer DTO Translation

For `GET /v1/now`, the data goes:
1. `services/now.rs` computes `NowOutput` (with sub-structs like `NowLabelOutput`, `NowEventOutput`, etc.)
2. `routes/now.rs` contains 17 `From<>` impls mapping `NowOutput` → `NowData` and all sub-types

Both type families are structurally identical. This pattern repeats for most endpoints. The motivation appears to be keeping API types decoupled from service internals, which is correct in principle, but in practice it doubles the struct count with no meaningful divergence. The route layer is the only place the `From<>` impls live, so they're hard to discover.

### 4.4 Orphaned `packages/` Directory

`packages/vel-affect-core/`, `packages/vel-render-watch/`, `packages/vel-render-web/`, `packages/vel-visual-morphology/`, `packages/vel-protocol/` are TypeScript files with no `package.json`, not referenced by `clients/web`, not in the web build. Dead code. The `packages/vel-protocol/src/types.ts` describes a `VelSyncPacket` that does not match the Rust `vel-protocol` wire format.

### 4.5 Business Logic in `vel-core`

`vel-core/src/operator_queue.rs` (922 LOC) and `vel-core/src/run.rs` (843 LOC) contain scoring constants, priority bucket logic, and state machine helpers. These are application-level concerns, not domain types. They should live in `veld/src/services/`. The comment in `vel-core/src/lib.rs` says it's "domain types, semantics, invariants (no transport, no storage)" but these files violate that.

### 4.6 Silent Tool Argument Parse Failure

In `crates/vel-llm/src/providers/llama_cpp.rs` line 291:
```rust
let arguments = serde_json::from_str(&args_str).unwrap_or(serde_json::Value::Null);
```
If a model emits malformed JSON in a tool call's arguments field, this silently produces `Null` and the tool execution proceeds with no arguments. The tool executor (`services/chat/tools.rs`) will receive `Null` arguments and likely return a confusing error. Should propagate parse failure.

### 4.7 `inference/mod.rs` at 1,788 Lines

The inference engine is a monolith. The `run_at()` entry point delegates to ~12 private functions spread across the file, with helper structs (`InferenceInputs`, `DerivedContextState`, `SignalInputs`, `AttentionState`, etc.) defined inline. The reducer trait was added (`SignalReducer`), and 4 reducers exist in `reducers/`, but the main orchestration still has large inline functions like `derive_context_state()`. Incrementally harder to modify.

### 4.8 `services/client_sync.rs` at 2,770 Lines

The cluster sync service is the largest service file. It handles work assignment, pull/push sync, bootstrap, and Apple behavior reporting. No `#[cfg(test)]` block, but behavior is tested via integration tests. High complexity and no in-file tests is a risk.

### 4.9 Auth Not Enforced in Tests by Default

Integration tests in `crates/veld/tests/` use the raw `build_app()` with no exposure policy (so auth is unenforced). The `daily_loop_morning.rs` test hardcodes `x-vel-operator-token: operator-secret` but the app built with `build_app()` doesn't require it. Only `test_app_with_policy()` in `app.rs` tests actually enforce auth. This means route auth behavior is not covered by integration tests.

### 4.10 Suggestions Service Has No Tests

`services/suggestions.rs` (no in-file tests) evaluates nudge-triggered suggestions. The `suggestion_engine.rs` integration test exists but only exercises a narrow path. Suggestion logic includes scoring, feedback, and acceptance — none of these are unit tested.

### 4.11 Inconsistent Error Handling in Capture Flow

`routes/captures.rs` (create_capture) uses `if let Err(e)` with `warn!()` for signal emission and commitment auto-creation failures — these are silently swallowed. For `todo` captures, a failure to create the commitment is logged but the route still returns 200. This is an intentional resilience trade-off but undocumented.

---

## 5. Real User Action Flows

### 5.1 Capture Flow

```
User input → POST /v1/captures
  └─ routes/captures.rs :: create_capture
       ├─ storage.insert_capture()           [vel-storage: captures_repo]
       ├─ storage.emit_event(CAPTURE_CREATED)
       ├─ storage.insert_signal(capture_created)
       └─ if capture_type == "todo":
            storage.insert_commitment()      [auto-promotion]
  └─ 200 OK with capture_id
```

**Missing:** No semantic indexing happens at capture time. The semantic search index (`semantic_memory_repo`) is populated by the context run service (`services/context_runs.rs`) and integration adapters. Fresh captures are not searchable until the next context run. This is a latency gap for capture→recall.

### 5.2 Evaluate / Inference Flow

```
POST /v1/evaluate
  └─ routes/evaluate.rs :: run_evaluate
       └─ services::evaluate::run_and_broadcast()
            ├─ services::risk::run()          [commitment risk scoring]
            ├─ services::inference::run_at()  [signal → CurrentContextV1]
            │    ├─ collect_inputs()          [open commitments, signals, nudges, risk]
            │    ├─ derive_context_state()    [attention, mode, meds, etc.]
            │    ├─ build_base_context()      [assemble CurrentContextV1]
            │    ├─ ReducerRegistry.reduce()  [calendar, git, health, messages reducers]
            │    └─ storage.upsert_current_context()
            ├─ services::nudge_engine::evaluate()
            └─ services::suggestions::evaluate_after_nudges()
  └─ broadcast WS "context:updated"
```

**Key boundary:** `run()` is only called here. Read routes (`GET /v1/context/current`, `GET /v1/explain/context`) read what's already stored — they never recompute.

### 5.3 Daily Loop (Morning Overview)

```
POST /v1/daily-loop/sessions
  └─ routes/daily_loop.rs :: start_session
       └─ services::daily_loop::start_morning_overview()
            ├─ daily_loop_inputs::load_daily_loop_inputs()
            │    ├─ storage.list_commitments(Open, today+overdue)   [max 3 todoist]
            │    ├─ storage.list_signals(calendar_event, next 12h)  [max 3]
            │    └─ operator_queue::build_action_items()            [friction callouts]
            └─ DailyLoopSession { state: MorningOverview }
  └─ 200 OK with session

POST /v1/daily-loop/sessions/:id/turn
  └─ routes/daily_loop.rs :: submit_turn
       └─ services::daily_loop::advance_turn()
            ├─ validate action (Skip requires note_text)
            ├─ append check_in_history
            └─ advance prompt ordinal or mark completed
  └─ updated session
```

**Morning overview is pure data, no LLM.** Commitments do NOT get created during morning (tested explicitly). The prompts are generated from a bounded input snapshot.

### 5.4 Chat / Assistant Flow

```
POST /v1/chat/entry  (or /v1/apple/voice/turn → internally)
  └─ services::chat::messages::create_assistant_entry()
       ├─ detect daily loop intent (keyword matching on transcript)
       │    └─ if morning/standup: start/resume session instead
       ├─ detect planning profile voice command
       ├─ create_user_message()
       ├─ generate_assistant_reply()     [in assistant.rs]
       │    ├─ load conversation history (last 50 msgs)
       │    ├─ build_chat_grounding_prompt()   [from now + projects + agent inspect]
       │    ├─ chat_tool_specs()               [9 tools]
       │    └─ Router.generate() [loop up to MAX_CHAT_TOOL_ROUNDS=3]
       │         ├─ if tool_calls: execute_chat_tool() for each
       │         └─ if text: persist assistant message
       └─ detect proposal in assistant response
            └─ if proposal: create intervention for operator approval
```

**LLM is optional:** If `state.llm_router.is_none()`, the route returns user message only, assistant message is None. Clients must handle the `None` case.

### 5.5 Context Recall Flow

```
GET /v1/search?q=...
  └─ routes/search.rs
       └─ storage.semantic_query()         [BM25 lexical scoring in semantic_memory_repo]
  └─ Vec<SemanticHit> with provenance

(internal) context_runs::generate_today_at()
  └─ context_generation::semantic_query_for_snapshot()
       ├─ extract focus candidates from snapshot
       └─ build SemanticQuery { strategy: Hybrid, top_k: 8 }
  └─ retrieval::semantic_query()
       └─ storage.semantic_query()
  └─ hits appended to run events (SearchExecuted)
```

**Semantic memory is lexical only.** No vector embeddings. The `semantic_memory_repo.rs` implements BM25-style TF-IDF scoring via SQLite FTS5. Queries are text-matched. "Hybrid" retrieval is lexical+metadata filtering, not semantic+lexical fusion. The naming is misleading — there is no neural embedding anywhere in the system.

### 5.6 Execution Handoff / Supervised Execution Flow

```
POST /v1/execution/handoffs
  └─ routes/execution.rs :: create_execution_handoff
       └─ execution_routing::create_execution_handoff()
            ├─ validate: scopes not empty, expected_output_schema not empty
            ├─ compute routing decision (review_gate based on write_scope)
            │    write_scope → OperatorApproval
            │    no write_scope → OperatorPreview
            └─ storage.insert_execution_handoff()
  └─ 200 with handoff + routing decision + review_state: "pending_review"

GET /v1/execution/handoffs/:id/launch-preview
  └─ compute blockers (pending review blocks launch)

POST /v1/execution/handoffs/:id/approve
  └─ execution_routing::approve_execution_handoff()
       └─ storage.update_execution_handoff_review_state("approved")
  └─ 200 with updated handoff

POST /v1/execution/handoffs/:id/reject
  └─ similar to approve, sets "rejected"
```

**Execution does not actually launch anything.** The approve/reject sets a state flag. Actual agent invocation (launching `codex`, claude-code, etc.) is in `services/connect_runtime.rs` and `services/wasm_guest_runtime.rs`, which are called via `POST /v1/connect/launch`. The handoff review and the agent launch are not yet wired together in a single flow.

### 5.7 Integration Sync Flow

```
POST /v1/sync/calendar  (or todoist, activity, health, git, etc.)
  └─ routes/sync.rs
       └─ services::integrations::sync_*()
            └─ adapters::{calendar, todoist, ...}::sync()
                 ├─ read snapshot file from disk (e.g. var/integrations/calendar/local.ics)
                 ├─ parse into signals
                 └─ storage.insert_signal() for each
  └─ 200 OK
```

Todoist sync additionally updates commitments (upsert by `source_id`). Google Calendar can also sync via OAuth token in storage. Integrations are pull-based (operator triggers) not push/webhook.

---

## 6. Regression Risks

### High

1. **`inference/mod.rs` 1,788 LOC monolith.** Any change to `derive_context_state()` or `build_base_context()` can silently break context shape. Unit tests exist but don't cover all field interactions.

2. **`services/apple_voice.rs` (1,153 LOC, zero tests).** The voice turn pipeline routes between daily loop, planning profile edits, chat, and schedule responses. Any change here risks breaking iOS continuity.

3. **`services/connect_runtime.rs` + `services/wasm_guest_runtime.rs` (zero tests).** Agent launch is untested at the service level.

4. **Tool argument silent null fallback** in `vel-llm/src/providers/llama_cpp.rs:291`. Any model emitting slightly malformed JSON will silently pass `Null` to tool executors.

5. **Semantic memory not populated at capture time.** A capture created via `POST /v1/captures` is NOT indexed until the next context run. This is a functional gap, not just a test gap.

### Medium

6. **`vel-api-types/src/lib.rs` at 7,527 LOC.** Not a runtime risk, but editing it is error-prone due to naming collisions between similarly-named types.

7. **Business logic in `vel-core`** (`operator_queue.rs`, `run.rs`). Changes to ranking constants there affect displayed action items but the constants are not near the UI code, so the connection is non-obvious.

8. **Auth not enforced in integration tests.** Route-level auth failures will only surface in production or in the specific auth matrix tests in `app.rs`. Easy to add a route without testing its auth class.

9. **`vel-protocol` exists in both `crates/vel-protocol/` (Rust) and `packages/vel-protocol/` (TypeScript).** The TypeScript version defines `VelSyncPacket` with fields like `valence`, `arousal`, `faciality` — an affect model that has no Rust counterpart. These are diverged protocols.

### Low

10. **Orphaned `packages/` TypeScript code.** No runtime risk but misleading to contributors.

11. **Double DTO structs** (service Output types → API Data types). Not a bug, but adds confusion when tracing a response field.

---

## 7. Summary Table

| Area | Status | Key Risk |
|------|--------|----------|
| Crate layering | Good — vel-storage doesn't leak | None |
| LLM abstraction | Clean | Tool arg parse silently nulls |
| Inference engine | Tested, documented boundary | Monolith size |
| Chat assistant | Tested via integration tests | Tool result formatting |
| Daily loop | Well-tested | Minor: no auth in integration tests |
| Execution handoff | Tested | Handoff → launch not wired |
| Apple voice | 1,153 LOC, zero tests | High regression risk |
| Semantic memory | Lexical only (no embeddings) | Not indexed at capture time |
| `vel-api-types` | 7,527 LOC, single file | Editing difficulty |
| `app.rs` | 12k LOC (11.6k are tests) | Should be split out |
| `packages/` | Orphaned TypeScript | Dead code confusion |
| Frontend tests | 22 test files, shallow | Inadequate for UI logic |
