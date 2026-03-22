# Phase 57 Dependency And Invariants Sheet

## Purpose

This is the anti-drift sheet for Phase 57.

It states:

- which chunk may depend on which earlier decisions
- which invariants no chunk is allowed to violate
- which seams are reserved but deferred
- which scope expansions are forbidden during contract lock

Phase 57 should use this sheet as binding planning law, not as optional commentary.

## Cross-Cutting Rule

`57-05` is a cross-cutting constraint packet, not a late cleanup phase.

Its backend constraints are binding across:

- `57-01`
- `57-02`
- `57-03`
- `57-04`

That means earlier chunks must already satisfy:

- storage agnosticism
- typed Rust ID newtype posture
- deterministic serde and versioning rules
- feature-gating discipline
- error taxonomy awareness
- optimistic concurrency posture
- deterministic bootstrap/seeding rules
- trait-based scheduler/secret/query/runtime seams
- projection/read-model separation
- testability requirements

## Dependency Map

### `57-01` Object Model And Linkage

May depend on:

- milestone `0.5` roadmap/context
- imported reference packs
- repo-wide layering and schema rules

Must not assume:

- finalized adapter semantics beyond what is needed to name canonical linkage seams
- storage-engine-specific implementation
- platform-specific secret or runtime APIs

### `57-02` Membrane, Ownership, And WriteIntent

May depend on:

- finalized object classes, IDs, envelope, relations, and SyncLink role from `57-01`

Must not:

- redefine canonical object taxonomy
- collapse runtime records into canonical content objects
- assume provider-specific execution details in the membrane contract

### `57-03` Registry And Workflows

May depend on:

- finalized object/linkage rules from `57-01`
- membrane and policy semantics from `57-02`

Must not:

- redefine action semantics
- bypass membrane or policy boundaries
- introduce raw-tool access for skills
- collapse registry entities into ordinary content-object semantics

### `57-05` Rust Backend Constraints

Constrains:

- all other chunks

May depend on:

- previously locked ontology and membrane decisions

Must not:

- reopen product semantics without identifying a real backend contradiction
- mutate provider-scope decisions except where backend portability makes a current contract impossible

### `57-04` Adapter Boundaries, Migration, And Proving Flows

May depend on:

- object/linkage decisions from `57-01`
- membrane/ownership/write-intent decisions from `57-02`
- registry/workflow/runtime boundaries from `57-03`
- backend constraints from `57-05`

Must not:

- redefine canonical object classes
- redefine action/policy semantics
- redefine registry identity rules
- smuggle platform/storage assumptions into adapter contracts

## Phase 57 Invariants

No Phase 57 chunk may violate these:

- canonical `content`, `registry`, `read_model`, and `runtime` classes remain distinct
- canonical linkage truth remains external through `SyncLink` plus typed relations
- `source_summary` is derived only
- workflows live in canonical storage
- `Module`, `Skill`, and `Tool` are canonical registry objects
- `Availability` remains a read model
- `WriteIntent` remains a provider-agnostic runtime record
- provider-owned truth is not flattened into canonical ownership ambiguity
- backend domain remains storage-agnostic and platform-neutral
- projections remain rebuildable and non-authoritative
- secrets remain outside canonical account objects
- seeded workflow bootstrap remains deterministic and idempotent

## Reserved But Deferred

These seams may be named or reserved, but not fully widened in Phase 57:

- executable hooks
- first-class `Place`
- recurring edit scope `this and following`
- generic plugin marketplace
- arbitrary schema-extension system
- direct raw-tool invocation by skills
- all-history Google Calendar import by default
- generalized cross-provider merge engine
- visual workflow authoring

## Forbidden Scope Creep

Do not widen Phase 57 into:

- UI/client embodiment work
- speculative provider/platform expansion
- marketplace/remote-registry design
- full generalized plugin execution substrate
- storage-engine-specific architecture drift
- platform-native secret/storage/runtime coupling in core contracts

## Acceptance Use

Every Phase 57 chunk should:

- reference this sheet
- preserve these invariants
- call out reserved/deferred seams explicitly
- fail review if it reopens settled boundaries without naming the contradiction
