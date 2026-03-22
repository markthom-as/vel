# Coding Conventions

**Analysis Date:** 2026-03-22

## Naming Patterns

**Files:**
- Rust source uses `snake_case.rs` module files across `crates/`, for example `crates/vel-core/src/context.rs`, `crates/vel-storage/src/infra.rs`, and `crates/veld/src/services/operator_settings.rs`.
- Web React components and feature modules use `PascalCase.tsx` for component files and `camelCase.ts` for data/helpers, for example `clients/web/src/views/settings/SettingsPage.tsx`, `clients/web/src/shell/MainPanel/MainPanel.tsx`, `clients/web/src/data/operator.ts`, and `clients/web/src/data/query.ts`.
- Web feature folders are `PascalCase` only when they map to a component package such as `clients/web/src/core/Button/` or `clients/web/src/views/threads/ConversationList/`; broader feature folders stay lowercase such as `clients/web/src/views/now/` and `clients/web/src/views/settings/`.
- Barrel files use `index.ts` in web packages. Follow the pattern in `clients/web/src/data/resources.ts` and the `clients/web/src/README.md` guidance instead of importing deep internals when a local barrel exists.
- Test files are either colocated `*.test.ts` / `*.test.tsx` in `clients/web/src/**` or module-scoped / integration files in Rust under `crates/*/src/**` with `#[cfg(test)]` and `crates/veld/tests/*.rs`.

**Functions:**
- TypeScript functions use `camelCase`, including hooks and decoders: `buildBackupTrustProjection` in `clients/web/src/data/operator.ts`, `decodeSyncResultData` in `clients/web/src/data/operator.ts`, and `useQuery` in `clients/web/src/data/query.ts`.
- Event handlers in React use `onX` for props and `handleX` less consistently inside components. Preserve the surrounding file style rather than forcing a rename.
- Rust functions use `snake_case`, including helpers such as `sqlite_connect_options` in `crates/vel-storage/src/infra.rs`, `docs_catalog` in `crates/vel-cli/src/commands/docs.rs`, and `build_app_with_state` references in `crates/veld/tests/*.rs`.
- Async Rust tests and integration helpers also use `snake_case` with behavior-heavy names, for example `commitment_scheduling_apply_route_updates_commitment_and_thread_continuity` in `crates/veld/tests/commitment_scheduling_api.rs`.

**Variables:**
- TypeScript local variables and object keys use `camelCase` for code-owned names such as `peopleById`, `peopleNeedingReview`, and `outputRoot` in `clients/web/src/data/operator.ts`.
- Transport-shaped payloads keep backend wire keys in `snake_case` when mirroring API contracts, especially in fixtures and decoder outputs. Preserve that in tests like `clients/web/src/views/settings/SettingsPage.test.tsx` and `clients/web/src/views/now/NowView.test.tsx`.
- Rust locals and fields use `snake_case`; constants use `UPPER_SNAKE_CASE`, for example `DOCS_CATALOG_JSON` in `crates/vel-cli/src/commands/docs.rs` and `MIGRATOR` in `crates/vel-storage/src/db.rs`.
- Do not add underscore-prefixed “private” names in TypeScript; the repo does not use that convention.

**Types:**
- TypeScript interfaces, type aliases, and React props use `PascalCase`, for example `SettingsPageProps`, `RetryDraft`, and `OperatorReviewStatusData` in `clients/web/src/views/settings/SettingsPage.tsx` and `clients/web/src/data/operator.ts`.
- String-union types are preferred over enums in the web client, for example `SettingsTab`, `SettingsSectionKey`, and `IntegrationActionKey` in `clients/web/src/views/settings/SettingsPage.tsx`.
- Rust structs, enums, and traits use `PascalCase`, for example `CurrentContextV1` in `crates/vel-core/src/context.rs`, `StorageError` in `crates/vel-storage/src/db.rs`, and `LlmProvider` implementations in `crates/veld/tests/chat_grounding.rs`.
- Rust enum variants are `PascalCase`; serialized wire values are usually normalized with serde attributes such as `#[serde(rename_all = "snake_case")]` in `crates/vel-core/src/context.rs`.

## Code Style

**Formatting:**
- Rust formatting is effectively `rustfmt` default style. The repo enforces this via `cargo fmt --all -- --check` in `Makefile`.
- Rust uses 4-space indentation, trailing commas in multiline literals, and grouped imports as shown in `crates/veld/tests/commitment_scheduling_api.rs` and `crates/vel-core/src/context.rs`.
- The web client has no Prettier or Biome config in the repo root or `clients/web/`. Do not assume an autoformatter beyond editor defaults.
- TypeScript style is currently mixed. Many newer files use semicolons and wider import blocks, for example `clients/web/src/views/settings/SettingsPage.tsx` and `clients/web/src/data/operator.ts`; many tests and smaller modules omit semicolons, for example `clients/web/src/data/query.test.tsx`, `clients/web/src/api/client.test.ts`, and `clients/web/src/shell/MainPanel/MainPanel.test.tsx`.
- Because frontend formatting is not fully enforced, preserve the dominant style of the file you touch instead of reformatting unrelated lines.

**Linting:**
- Frontend linting is `ESLint 9` with flat config in `clients/web/eslint.config.js`.
- The active frontend rule set is `@eslint/js` recommended, `typescript-eslint` recommended, `eslint-plugin-react-hooks` recommended, and `eslint-plugin-react-refresh` Vite config.
- ESLint currently ignores only `dist/` via `globalIgnores(['dist'])` in `clients/web/eslint.config.js`.
- There is no repo-wide TypeScript style rule for quotes, semicolons, or import ordering. If you want consistency, match the existing file and stay lint-clean.
- Rust linting is `cargo clippy --workspace --all-targets --all-features -- -D warnings` from `Makefile`, so new warnings are build blockers.

## Import Organization

**Order:**
1. External packages first.
2. Same-package internal modules next.
3. Relative imports last.
4. Type imports are usually mixed into the nearest import rather than isolated into separate `import type` blocks unless the file already does that.

**Grouping:**
- Web files commonly separate groups with a blank line when moving from packages to local modules, as in `clients/web/src/views/settings/SettingsPage.tsx`.
- Shorter web files often keep a compact import block with no enforced sorting, as in `clients/web/src/api/client.test.ts` and `clients/web/src/data/query.test.tsx`.
- Rust imports are grouped by crate, often with nested braces and one blank line between external crates and `crate::...` imports, as in `crates/veld/src/app.rs` and `crates/vel-core/src/context.rs`.

**Path Aliases:**
- No TypeScript path aliases are configured in the inspected web surface. Use relative imports under `clients/web/src/` and follow the depth guidance in `clients/web/src/README.md`.
- For web code, import through local barrels when the folder exposes one, for example `../../views/now` and `../../data/resources` in `clients/web/src/shell/MainPanel/MainPanel.test.tsx`.

## Error Handling

**Patterns:**
- Rust code prefers typed errors with `thiserror` or `anyhow` at process edges. Use typed domain/storage errors inside crates, for example `StorageError` in `crates/vel-storage/src/db.rs`.
- Rust conversion helpers usually map lower-level errors into boundary-specific messages immediately, as in `docs_catalog` and `contracts_manifest` in `crates/vel-cli/src/commands/docs.rs`.
- TypeScript decoder functions throw `Error` on malformed payloads instead of returning sentinel values. Follow patterns like `decodeSyncResultData` and `decodeEvaluateResultData` in `clients/web/src/data/operator.ts`.
- Tests assert explicit error messages rather than generic failure, for example the `rejects.toThrow` assertions in `clients/web/src/api/client.test.ts`.
- Route and service layering should keep transport mapping at the boundary; the codebase authority for that is reinforced by `docs/MASTER_PLAN.md` and visible in route registration from `crates/veld/src/app.rs`.

**Error Types:**
- Throw or return errors for invariant violations, malformed transport payloads, and missing storage entities. Do not silently coerce invalid data.
- In Rust, prefer `expect("reason")` only in tests and short setup helpers, as seen throughout `crates/veld/tests/*.rs` and `crates/vel-storage/src/infra.rs`.
- In TypeScript tests, use `throw new Error(\`Unexpected GET ${path}\`)` or similar guard failures for unmocked branches, as in `clients/web/src/views/settings/SettingsPage.test.tsx`.

## Logging

**Framework:**
- Runtime logging is Rust-side tracing infrastructure from workspace dependencies `tracing` and `tracing-subscriber` declared in `Cargo.toml`.
- Frontend production logging conventions are not prominently established in the inspected web files; avoid introducing `console.log` unless the surrounding code already uses it for a deliberate local debug path.

**Patterns:**
- High-value runtime boundaries are expected to be traceable by the repository rules in `docs/MASTER_PLAN.md`; keep logging decisions near service, route, execution, and external-call boundaries rather than deep utility functions.
- The inspected web tests do not assert on logs. Prefer visible state and structured return values over log-driven behavior.

## Comments

**When to Comment:**
- Rust uses doc comments and targeted inline comments to explain domain intent or migration edge cases, not obvious mechanics. Examples: the module header and versioning notes in `crates/vel-core/src/context.rs`, and migration-repair comments in `crates/vel-storage/src/infra.rs`.
- TypeScript comments are sparse and usually reserved for architecture or phase-specific context, for example the note in `clients/web/src/data/context.ts` referenced by search results.
- Add comments when the code is encoding a product rule, migration exception, or contract boundary. Skip narration comments for straightforward JSX or data plumbing.

**JSDoc/TSDoc:**
- TSDoc is not widely used in the inspected web surface. Favor clear names and focused types over block comments.
- Rust doc comments are used selectively for public domain types and contract surfaces, as in `crates/vel-core/src/context.rs`.

**TODO Comments:**
- No durable TODO convention is enforced by tooling in the inspected surface. If you must add one, include enough context to be actionable and prefer a ticket reference.

## Function Design

**Size:**
- Small focused helpers are common in Rust domain and storage modules, for example `backend_pool` in `crates/vel-storage/src/db.rs` and `backupStatusLabel` in `clients/web/src/data/operator.ts`.
- Large React screens and orchestration modules do exist, especially `clients/web/src/views/settings/SettingsPage.tsx`. Do not treat that size as a target for new code; prefer extracting focused helpers or subcomponents when adding behavior.
- Large Rust integration surfaces exist in `crates/veld/src/app.rs`, but repository guidance explicitly says not to add new large scenario tests there. Follow that rule for new tests and new logic placement.

**Parameters:**
- Web helpers commonly take structured objects or typed payloads rather than long positional lists once complexity rises, for example `buildBackupTrustProjection(backup)` and query-key builders in `clients/web/src/data/operator.ts`.
- Rust constructors and service calls may still take several positional arguments in setup-heavy areas, such as `AppState::new(...)` in tests. When adding new APIs, prefer clearer typed inputs if the call shape is expanding.

**Return Values:**
- TypeScript decoders and selectors return fully typed values or throw. Avoid `null`/`undefined` unless absence is part of the contract.
- Rust functions return `Result<_, _>` at fallible boundaries and use early returns for branch exits, as seen across `crates/vel-storage/src/infra.rs` and `crates/vel-cli/src/commands/docs.rs`.

## Module Design

**Exports:**
- Web shared packages frequently use named exports and a local `index.ts` barrel, as shown by `clients/web/src/data/resources.ts`.
- React component files often export named components instead of default exports, for example `SettingsPage` in `clients/web/src/views/settings/SettingsPage.tsx`.
- Rust crates expose modules through `mod` declarations and public types/functions from crate roots; keep module seams explicit across `vel-core`, `vel-storage`, `vel-api-types`, and `veld`.

**Barrel Files:**
- Use web barrel files only for a folder’s intended public API. `clients/web/src/README.md` treats `index.ts` as the package boundary for `core/`, `shell/`, and `views/` subpackages.
- Do not introduce broad catch-all barrels across unrelated features; keep re-exports local to the feature or component package.

---

*Convention analysis: 2026-03-22*
