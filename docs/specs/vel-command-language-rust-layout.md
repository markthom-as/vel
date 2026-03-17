# Vel Command Language Rust Layout Proposal

Status: Planned  
Audience: implementers working in `crates/vel-core`, `crates/vel-cli`, `crates/veld`, and `crates/vel-api-types`  
Purpose: propose a concrete Rust module/file layout for the command language so it can be added incrementally to the existing workspace without breaking current boundaries

## 1. Summary

Given the current workspace structure:

- `crates/vel-core` already owns domain semantics and shared types
- `crates/vel-cli` currently owns the `clap` tree and per-command command files
- `crates/veld` already has a clean services layer

The best first implementation is:

- keep the parser and completion system in `vel-cli`
- add shared typed command contracts in `vel-core`
- add execution adapters in `veld`
- avoid adding a new crate in phase 1 unless reuse pressure becomes real

This keeps the implementation close to the current codebase while still giving the DSL a clean architecture.

## 2. Proposed Module Layout

### 2.1 `crates/vel-core`

Add a new top-level module:

```text
crates/vel-core/src/command/
  mod.rs
  kinds.rs
  target.rs
  operation.rs
  resolution.rs
  planning.rs
```

And export it from:

```text
crates/vel-core/src/lib.rs
```

Recommended responsibilities:

- `kinds.rs`
  `DomainKind`, command-visible kind catalog
- `target.rs`
  `TypedTarget`, selectors, relation endpoints
- `operation.rs`
  `DomainOperation`, relation operations, lifecycle operations
- `resolution.rs`
  `ResolvedCommand`, `IntentResolution`, `ResolutionMeta`, confidence/assumption types
- `planning.rs`
  planning-specific kinds like `SpecDraft`, `ExecutionPlan`, `DelegationPlan` if those become shared concepts

Recommended exported types:

```text
vel_core::command::DomainKind
vel_core::command::DomainOperation
vel_core::command::TypedTarget
vel_core::command::ResolvedCommand
vel_core::command::IntentResolution
```

This should stay domain-facing and shell-agnostic.

### 2.2 `crates/vel-cli`

Add a new top-level module:

```text
crates/vel-cli/src/command_lang/
  mod.rs
  ast.rs
  parse.rs
  tokenize.rs
  infer.rs
  explain.rs
  preview.rs
  completion.rs
  registry.rs
  adapters/
    mod.rs
    captures.rs
    commitments.rs
    runs.rs
    artifacts.rs
    review.rs
    planning.rs
```

Recommended responsibilities:

- `tokenize.rs`
  CLI-local lexical splitting for sentence commands
- `ast.rs`
  `ParsedCommand`, `PhraseFamily`, `Verb`, syntax-level nodes
- `parse.rs`
  deterministic parser from tokens to AST
- `infer.rs`
  grammar defaults, low-risk local inference, alias expansion
- `registry.rs`
  command-facing type registry assembly
- `completion.rs`
  next-token and typed-structure completion
- `preview.rs`
  dry-run rendering from `ResolvedCommand`
- `explain.rs`
  human-readable explanation of parsing and inference
- `adapters/*`
  per-kind CLI adapters providing parse/resolve/complete/preview helpers

This layout matches the current `vel-cli` role without forcing `main.rs` to hold all command-DSL logic.

### 2.3 `crates/veld`

Add a new services module:

```text
crates/veld/src/services/command_lang.rs
```

And optionally a small helper module:

```text
crates/veld/src/services/command_adapters/
  mod.rs
  captures.rs
  commitments.rs
  runs.rs
  artifacts.rs
  planning.rs
```

Recommended responsibilities:

- validate service-level command execution
- map `ResolvedCommand` to existing service calls
- reject unsupported operations cleanly
- centralize command execution policy for typed commands

This should reuse current services rather than duplicate logic.

### 2.4 `crates/vel-api-types`

Only add types here if command execution or explanation needs an HTTP surface later.

Possible future files:

```text
crates/vel-api-types/src/command.rs
```

Possible DTOs:

- `CommandExecuteRequest`
- `CommandExplainRequest`
- `CommandExplainResponse`

Do not put the core DSL model here first. Keep API DTOs as transport-only.

## 3. Suggested `mod.rs` Shapes

### 3.1 `vel-core`

```rust
pub mod command;
```

`crates/vel-core/src/command/mod.rs`

```rust
pub mod kinds;
pub mod operation;
pub mod planning;
pub mod resolution;
pub mod target;

pub use kinds::DomainKind;
pub use operation::DomainOperation;
pub use planning::{DelegationPlanKind, ExecutionPlanKind, SpecDraftKind};
pub use resolution::{IntentResolution, ResolutionMeta, ResolvedCommand};
pub use target::{TargetSelector, TypedTarget};
```

### 3.2 `vel-cli`

`crates/vel-cli/src/command_lang/mod.rs`

```rust
pub mod adapters;
pub mod ast;
pub mod completion;
pub mod explain;
pub mod infer;
pub mod parse;
pub mod preview;
pub mod registry;
pub mod tokenize;
```

## 4. Integration with Current `vel-cli`

Right now `crates/vel-cli/src/main.rs` contains the `clap` tree and the dispatch logic for command handlers under `commands/`.

The lowest-risk integration path is:

### Phase 1

Keep the current command tree intact and add one new top-level subcommand:

```text
vel command <sentence...>
```

or:

```text
vel should ...
```

Implementation options:

- conservative: add `Command::Command { input: Vec<String>, dry_run: bool, json: bool }`
- more ambitious: intercept `vel should ...` before normal `clap` dispatch

The conservative option is cleaner for initial rollout because it does not fight `clap`.

### Phase 2

Add command-language shortcuts that compile to the same path:

- `vel should ...`
- `vel show ...`
- `vel inspect ...`

At that point, `main.rs` should delegate command-language handling to:

```text
crates/vel-cli/src/command_lang/
```

instead of growing more parsing logic inline.

## 5. Suggested CLI File Changes

### 5.1 New files

Add:

```text
crates/vel-cli/src/command_lang/mod.rs
crates/vel-cli/src/command_lang/ast.rs
crates/vel-cli/src/command_lang/parse.rs
crates/vel-cli/src/command_lang/tokenize.rs
crates/vel-cli/src/command_lang/infer.rs
crates/vel-cli/src/command_lang/explain.rs
crates/vel-cli/src/command_lang/preview.rs
crates/vel-cli/src/command_lang/completion.rs
crates/vel-cli/src/command_lang/registry.rs
crates/vel-cli/src/command_lang/adapters/mod.rs
crates/vel-cli/src/command_lang/adapters/captures.rs
crates/vel-cli/src/command_lang/adapters/commitments.rs
crates/vel-cli/src/command_lang/adapters/runs.rs
crates/vel-cli/src/command_lang/adapters/artifacts.rs
crates/vel-cli/src/command_lang/adapters/review.rs
crates/vel-cli/src/command_lang/adapters/planning.rs
```

### 5.2 Existing files to touch

- `crates/vel-cli/src/main.rs`
  add the new entrypoint and dispatch
- `crates/vel-cli/src/commands/mod.rs`
  only if you choose to expose a conventional `command` subcommand handler there
- `crates/vel-cli/src/client.rs`
  only if command explain/execute later needs a unified HTTP endpoint

## 6. Suggested Service File Changes

Add:

```text
crates/veld/src/services/command_lang.rs
```

Potential internal shape:

```rust
pub async fn execute(
    state: &AppState,
    command: vel_core::command::ResolvedCommand,
) -> anyhow::Result<CommandExecutionResult>
```

This service should call existing modules like:

- context/review services
- capture creation service path
- commitment service path
- synthesis/planning artifact creation path

Do not create a second logic path that bypasses the existing service layer.

## 7. Suggested Core File Changes

Add:

```text
crates/vel-core/src/command/mod.rs
crates/vel-core/src/command/kinds.rs
crates/vel-core/src/command/target.rs
crates/vel-core/src/command/operation.rs
crates/vel-core/src/command/resolution.rs
crates/vel-core/src/command/planning.rs
```

Touch:

- `crates/vel-core/src/lib.rs`
  export the new module

This is the minimum shared-type footprint needed to avoid burying the command contract inside the CLI binary.

## 8. Suggested First Rust Types

These are the first concrete types worth adding.

### 8.1 In `vel-core`

```rust
pub enum DomainKind {
    Capture,
    Commitment,
    Artifact,
    Run,
    Signal,
    Nudge,
    Suggestion,
    Thread,
    Context,
    SpecDraft,
    ExecutionPlan,
    DelegationPlan,
}
```

```rust
pub enum DomainOperation {
    Create,
    Inspect,
    List,
    Update,
    Link,
    Explain,
    Execute,
}
```

```rust
pub struct TypedTarget {
    pub kind: DomainKind,
    pub id: Option<String>,
    pub selector: Option<String>,
    pub attributes: serde_json::Value,
}
```

```rust
pub struct ResolvedCommand {
    pub operation: DomainOperation,
    pub targets: Vec<TypedTarget>,
    pub inferred: serde_json::Value,
    pub assumptions: Vec<String>,
    pub resolution: ResolutionMeta,
}
```

### 8.2 In `vel-cli`

```rust
pub enum PhraseFamily {
    Should,
    Show,
    Inspect,
    Review,
    Sync,
}
```

```rust
pub enum Verb {
    Capture,
    Feature,
    Commit,
    Remind,
    Review,
    Synthesize,
    Spec,
    Plan,
    Delegate,
}
```

```rust
pub struct ParsedCommand {
    pub family: PhraseFamily,
    pub verb: Verb,
    pub target_tokens: Vec<String>,
    pub options: Vec<ParsedOption>,
    pub source_text: String,
}
```

These syntax-level types should stay in `vel-cli`; they do not need to be domain-shared.

## 9. Why Not a New Crate Yet

A separate crate like `vel-command` could be justified later, but it is probably premature now.

Reasons not to add it in phase 1:

- the parser/completion UX is CLI-specific
- the workspace is still manageable
- the shared contract surface is small and belongs naturally in `vel-core`
- adding a new crate early increases churn without much immediate payoff

Reconsider a dedicated crate only if:

- chat/voice/API all start reusing the parser directly
- command-language code in `vel-cli` becomes too large
- there is real pressure to share parsing/completion logic outside the CLI

## 10. Recommended Next Step

The best concrete first slice is:

1. add `vel_core::command::{DomainKind, DomainOperation, TypedTarget, ResolvedCommand}`
2. add `crates/vel-cli/src/command_lang/{ast,parse,infer,preview,registry}.rs`
3. expose one conservative entrypoint such as `vel command ...`
4. support only `should capture`, `should feature`, and `should review` first
5. map execution onto the existing API/client/service paths

That is enough to prove the architecture without committing the whole CLI to the new syntax all at once.
