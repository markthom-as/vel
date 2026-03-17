# Coding Conventions

**Analysis Date:** 2026-03-17

## Naming Patterns

**Files:**
- Rust modules: lowercase with underscores (e.g., `chat_repo.rs`, `journal.rs`)
- TypeScript/TSX components: PascalCase for components (e.g., `MessageComposer.tsx`, `MainPanel.tsx`)
- TypeScript utilities/modules: camelCase (e.g., `client.ts`, `resources.ts`)
- Test files: named after source with `.test.ts` or `.test.tsx` suffix (e.g., `client.test.ts`, `MessageComposer.test.tsx`)

**Functions:**
- Rust: snake_case (e.g., `create_conversation`, `record_mood`, `list_conversations`)
- TypeScript: camelCase for regular functions and hooks (e.g., `apiGet`, `syncSource`, `decodeApiResponse`, `useSpeechRecognition`)
- React components: PascalCase, export as named functions (e.g., `export function MessageComposer()`)

**Variables:**
- Rust: snake_case for all locals and struct fields (e.g., `payload`, `capture_id`, `source_text`)
- TypeScript: camelCase (e.g., `text`, `sending`, `conversationId`)
- Boolean prefixes: `is` or `has` for predicates (e.g., `isSupported`, `isListening`, `voiceSupported`, `hasClientId`)

**Types:**
- Rust: PascalCase for structs and enums (e.g., `ConversationRecord`, `MoodJournalInput`, `WorkerRuntimeSnapshot`)
- TypeScript interfaces/types: PascalCase (e.g., `ApiResponse<T>`, `MessageData`, `CreateMessageResponse`)
- Type imports: explicit `type` keyword (e.g., `type ApiResponse, type CreateMessageResponse`)
- Generic types: single capital letters (e.g., `<T>`)

## Code Style

**Formatting:**
- **Rust:** rustfmt via `cargo fmt`. Max line length enforced by project.
- **TypeScript:** Prettier (configured implicitly in Vite/ESLint setup). Semicolons required, single quotes for strings.
- **TSX/JSX:** Tailwind CSS for styling, no separate CSS files in component directories.

**Linting:**
- **Rust:** clippy with `-D warnings` (warnings treated as errors). Run via `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- **TypeScript:** ESLint with flat config (`eslint.config.js`). Uses:
  - `@eslint/js` recommended rules
  - `typescript-eslint/recommended`
  - `eslint-plugin-react-hooks` for Hook rules
  - `eslint-plugin-react-refresh` for Vite refresh

**Verify code quality:**
```bash
make fmt-check        # Rust formatting check
make clippy-check     # Rust linting
make lint-web         # TypeScript linting
```

## Import Organization

**Rust order:**
1. Standard library imports (`use std::...`)
2. Third-party crate imports (`use tokio::...`, `use serde::...`)
3. Internal workspace crates (`use vel_core::...`, `use vel_storage::...`)
4. Local module imports (`use crate::...`)

**TypeScript/React order:**
1. React imports (`import { useState } from 'react'`)
2. Third-party library imports
3. Local API/types imports (relative, shallow: `../api/`, `../types`)
4. Local hook imports (e.g., `../hooks/useSpeechRecognition`)
5. Type imports (explicit `type` keyword for types)

**Path aliases:**
- TypeScript does NOT use path aliases; all imports are relative paths.
- Rust workspace uses crate names directly, no aliasing (e.g., `use vel_core::ConversationId`).

**Barrel files:**
- TypeScript data modules use barrel exports for query keys (e.g., `clients/web/src/data/resources.ts` exports from `./chat`, `./context`, `./operator`).
- Rust modules use pub re-exports for public APIs (sparse, as needed).

## Error Handling

**Patterns:**

**Rust:**
- Route handlers return `Result<Json<T>, AppError>`. Handlers convert storage/service errors to HTTP errors via the `?` operator.
- Services return `Result<T, AppError>`. Use `Err(AppError::bad_request("message"))` for input validation.
- Example: `crates/veld/src/services/journal.rs` validates score range, returns `AppError::bad_request()` on violation.
- All error propagation uses `?` operator; `.await?` for async results.
- Example from `crates/veld/src/routes/journal.rs`:
  ```rust
  pub async fn create_mood_journal(
      State(state): State<AppState>,
      Json(payload): Json<MoodJournalCreateRequest>,
  ) -> Result<Json<ApiResponse<CaptureCreateResponse>>, AppError> {
      let data = journal::record_mood(&state.storage, ...).await?;
      Ok(Json(ApiResponse::success(...)))
  }
  ```

**TypeScript:**
- API client catches fetch errors and wraps in descriptive Error objects.
- Components use try-catch in async handlers, set error state for display.
- Type guards validate decoded responses; throw descriptive errors on mismatch (e.g., `/conversation\.id/`).
- Example from `clients/web/src/api/client.ts`:
  ```typescript
  async function readApiError(res: Response, path: string): Promise<Error> {
    const fallback = new Error(`API ${res.status}: ${path}`);
    if (!contentType.includes('application/json')) return fallback;
    try {
      const body = await res.json();
      const message = body?.error?.message;
      if (typeof message === 'string' && message.trim()) {
        return new Error(`API ${res.status}: ${message}`);
      }
    } catch {
      return fallback;
    }
    return fallback;
  }
  ```

## Logging

**Framework:**
- **Rust:** `tracing` crate for structured logging. Imported as `use tracing::warn`, `use tracing::info`, etc.
- **TypeScript:** `console` for development; no structured logging framework deployed.

**Patterns:**
- **Rust:** Log warnings for non-fatal issues (e.g., `warn!("signal emission failed");` in `services/journal.rs`).
- **TypeScript:** Log errors/state changes to console in development; limit logging in component renders.

## Comments

**When to Comment:**
- Explain **why**, not what. Code should be self-documenting for "what."
- Document non-obvious behavior or workarounds (e.g., "This mutex pattern prevents task reentry").
- Mark temporary decisions with `FIXME` or `TODO` (rarely used in this codebase).
- Document public API surfaces and invariants.

**JSDoc/TSDoc:**
- Rust: Doc comments on public functions/types using `///`. Not heavily used for internal services.
- TypeScript: Interfaces and exported functions document parameter types via TypeScript syntax; minimal JSDoc comments.

## Function Design

**Size:**
- Rust service functions: 15-50 lines (single responsibility pattern)
- TypeScript React hooks: 20-60 lines; component logic extracted to hooks when reusable
- Routes/handlers: thin, 10-20 lines; parse → auth → call service → map to DTO → handle errors

**Parameters:**
- Rust services: accept references or values sensibly (e.g., `&Storage`, owned `MoodJournalInput`)
- TypeScript: destructure props in function signature; use interface types for props
- Example from `MessageComposer.tsx`:
  ```typescript
  interface MessageComposerProps {
    conversationId: string;
    onOptimisticSend?: (text: string) => string | undefined;
    onSent: (clientMessageId: string | undefined, userMessage: MessageData, ...) => void;
  }
  export function MessageComposer({ conversationId, onOptimisticSend, onSent }: MessageComposerProps)
  ```

**Return Values:**
- Rust: explicit `Result<T, E>` for fallible operations; Option for nullable results
- TypeScript:
  - Functions return typed objects or primitives
  - Decoders return the decoded type or throw
  - Async functions return Promises (`Promise<T>`)
  - Handlers set state directly; no return value (handlers are callbacks)

## Module Design

**Exports:**
- Rust: pub/pub(crate) visibility; internal functions not exported
- TypeScript: explicit named exports for functions/types, no default exports (except where Vite/Next requires)
  - `export function apiGet<T>(...)` not `export default apiGet`
  - `export type ApiResponse<T> = ...` for types

**Barrel Files:**
- Used in `clients/web/src/data/` to aggregate query keys:
  ```typescript
  export * from './chat';
  export * from './context';
  export * from './operator';
  ```
- Minimal in Rust (not idiomatic).

## Layer-Specific Conventions

**Rust Routes (`crates/veld/src/routes/`):**
- Handler signature: `pub async fn(State(state): State<AppState>, Json(payload): Json<RequestType>) -> Result<Json<ApiResponse<ResponseType>>, AppError>`
- Thin: parse request → call service → map response to ApiResponse → return
- Do NOT contain business logic or validation beyond basic route matching

**Rust Services (`crates/veld/src/services/`):**
- Pure application logic. Accept `&Storage` and domain input types.
- Validate inputs (e.g., score range in `record_mood`)
- Return domain structs or `AppError`, not HTTP DTOs
- Emit signals/events via storage

**Rust Repositories (`crates/vel-storage/src/repositories/`):**
- Stateless, per-entity database operations
- Accept `&SqlitePool` and domain types
- Async functions using sqlx with `?` error propagation
- Map SQL rows to typed records using helper functions (e.g., `map_conversation_row`)

**TypeScript API Client (`clients/web/src/api/client.ts`):**
- Generic typed functions: `apiGet<T>()`, `apiPost<T>()`, `apiPatch<T>()`
- Accept optional `Decoder<T>` for response validation
- Return typed `Promise<T>` or throw descriptive errors
- Wrap fetch errors contextually

**TypeScript Types (`clients/web/src/types.ts`):**
- Comprehensive transport DTOs mirroring Rust API types
- Type guards/decoders for runtime validation (e.g., `decodeApiResponse()`, `decodeConversationData()`)
- Nullable and optional fields explicit via `| null` and `?`
- Unix timestamps as `UnixSeconds` type alias

---

*Convention analysis: 2026-03-17*
