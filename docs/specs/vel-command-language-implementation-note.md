# Vel Command Language Implementation Note

Status: Planned  
Audience: implementers of `vel-cli`, `veld`, `vel-core`, and storage/service boundaries  
Purpose: define where the command language type registry, parser, inference, and execution adapters should live so the DSL integrates uniformly with Vel without violating existing crate boundaries

## 1. Summary

The command language should be implemented as a layered system:

1. `vel-core` owns stable domain kinds and domain-facing operation types
2. `vel-cli` owns parsing, shell UX, completion, and dry-run rendering
3. `veld` services own execution of typed commands against the existing application layer
4. storage remains behind existing service boundaries

The main rule is:

> the CLI may parse and infer commands, but domain meaning and side effects must still pass through typed service contracts.

This keeps the command language uniform across Vel types without turning the CLI into a second application runtime.

## 2. Placement by Layer

### 2.1 `vel-core`

`vel-core` should own the durable, implementation-neutral contracts that describe what the command layer can target.

Recommended responsibilities:

- canonical domain object kind enum
- typed operation enum for command execution
- typed selector/value objects where they represent domain meaning
- relation operation kinds
- explainability structs for resolved commands

Examples:

```text
DomainKind
  Capture
  Commitment
  Artifact
  Run
  Signal
  Nudge
  Suggestion
  Thread
  Context
  SpecDraft
  ExecutionPlan
  DelegationPlan
```

```text
DomainOperation
  Create
  Inspect
  List
  Update
  Link
  Explain
  Execute
```

`vel-core` should not own:

- shell tokenization
- shell completion logic
- path-specific CLI formatting
- model-backed completion providers

### 2.2 `vel-cli`

`vel-cli` should own the user-facing command language surface.

Recommended responsibilities:

- tokenization and parsing
- grammar validation
- shell completion generation
- local dry-run rendering
- command explanation rendering
- low-risk inference using local context
- command-facing type registry assembly

The CLI should compile input into a typed resolved command, but it should not directly implement storage or business logic.

### 2.3 `veld` services

`veld` should own execution of resolved commands.

Recommended responsibilities:

- accept typed command requests from CLI/API
- perform domain validation
- invoke existing services for captures, commitments, runs, artifacts, and future planning flows
- persist artifacts and linked objects
- emit run/event/provenance records where applicable

This follows the existing repo rule that route handlers stay thin and services hold application logic.

### 2.4 Storage

Storage should remain unaware of shell grammar and user phrasing.

Storage responsibilities remain:

- persistence
- retrieval
- typed mapping to core/domain objects

The command language should never require storage crates to understand sentence structure.

## 3. Type Registry Placement

The command language needs a uniform type registry, but not all parts of that registry belong at the same layer.

Recommended split:

### 3.1 Core-owned registry metadata

`vel-core` should define the stable kind catalog and any domain-neutral selector/value contracts.

This is the shared source of truth for:

- what kinds exist
- what operations are valid in principle
- what relation shapes are supported in principle

### 3.2 CLI-owned registry adapters

`vel-cli` should assemble command-facing adapters for each kind.

Each adapter can provide:

- aliases
- parsing helpers
- completion sources
- preview renderers
- local resolution helpers

This keeps shell UX concerns out of `vel-core`.

### 3.3 Service-owned execution adapters

`veld` should map the resolved typed command to the appropriate service call.

This is where:

- `Create(Capture)` maps to capture service
- `Create(Commitment)` maps to commitment service
- `Execute(ReviewToday)` maps to review/orientation flow
- `Create(SpecDraft)` maps to planning artifact creation

## 4. Recommended Data Shapes

### 4.1 Parsed command

CLI-local and syntax-oriented.

```text
ParsedCommand {
  phrase_family,
  verb,
  raw_target_tokens,
  options,
  source_text
}
```

### 4.2 Resolved command

Shared typed contract passed toward execution.

```text
ResolvedCommand {
  operation: DomainOperation,
  targets: [TypedTarget],
  inferred: { ... },
  assumptions: [ ... ],
  resolution_meta: {
    parser,
    model_assisted,
    confirmation_required
  }
}
```

### 4.3 Typed target

Stable cross-domain target shape.

```text
TypedTarget {
  kind: DomainKind,
  id: optional,
  selector: optional,
  attributes: { ... }
}
```

## 5. Inference Boundaries

Inference should be split by risk and by knowledge source.

### 5.1 CLI-local inference

Safe to do in `vel-cli`:

- grammar defaults
- option defaults
- known aliases
- recent local state lookups
- likely spec path suggestions
- dry-run previews

### 5.2 Service-validated inference

Must be checked or finalized in services:

- whether a target object actually exists
- whether a mutation is allowed
- whether a relation is valid
- whether a planning artifact type is supported
- whether a run-backed operation should be created

### 5.3 Optional model assist

If model-backed ranking/repair exists, it should sit at the CLI or sidecar boundary and produce candidate structured interpretations only.

Those candidates must still be:

1. re-parsed or validated
2. resolved against the type registry
3. checked by services before execution

## 6. Uniformity Rule

To keep the command language uniform across types, every supported type should provide the same minimum adapter surface:

- `parse_target`
- `complete`
- `resolve`
- `preview`
- `explain`
- `execute_adapter`

If a new type cannot satisfy this contract, it should not yet be treated as a first-class DSL type.

## 7. Crate Boundary Notes

This design should respect current repo boundaries:

- `vel-core` owns domain semantics and domain types
- `vel-storage` must not depend on `vel-api-types`
- route handlers remain thin
- services hold application logic

Implications:

- the DSL AST itself can live in `vel-cli`
- the domain-facing resolved command types should live in `vel-core` if shared across CLI/API/voice/chat
- service execution adapters should live near `veld` services, not in storage
- API DTOs for command execution, if added later, should live in `vel-api-types` and map from core types at the boundary

## 8. Suggested Implementation Sequence

### Phase 1

- add core enums for `DomainKind`, `DomainOperation`, and `TypedTarget`
- add CLI parser and dry-run output
- add CLI registry adapters for captures, commitments, runs, and artifacts
- map resolved commands to existing service calls

### Phase 2

- add planning artifact kinds such as `SpecDraft`, `ExecutionPlan`, `DelegationPlan`
- add service adapters for planning flows
- add explain command output for inferred settings and assumptions

### Phase 3

- expose the same resolved command contract to API/chat/voice
- add signals, nudges, suggestions, threads, and provenance-aware cross-type operations
- add optional model-backed repair and ranking

## 9. Recommendation

The cleanest split is:

- `vel-core`: shared command target and operation types
- `vel-cli`: parser, completion, inference UX, dry-run rendering
- `veld`: execution adapters and validation

That gives Vel one command architecture that can expand across many entity types and surfaces without violating the existing repo rules.
