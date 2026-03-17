# Vel Command Language Spec

Status: Planned  
Audience: product, CLI, daemon, and UX implementers  
Purpose: define a structured command language for Vel that feels sentence-like, supports deep autocomplete, and compiles to existing Vel domain operations without turning the CLI into an opaque chatbot

## 1. Summary

Vel should support a command language that sits between:

- strict Unix-style flags
- open-ended natural language chat

The goal is a **structured DSL with guided sentence completion**.

Examples:

```text
vel should feature "quick triage for unresolved message threads"
vel should capture "remember to test signal replay dedupe"
vel should capture file ./notes/dimitri.txt
vel should commit "ship run retry inspection"
vel should review today
```

This language should let the user express intent in short, memorable phrases while keeping the execution path deterministic and inspectable.

The design principle is:

> parse with rules first, autocomplete deeply, and use LLM assistance only as a bounded helper rather than as the execution engine.

## 2. Why This Exists

Vel already has a growing noun/verb CLI surface. That is useful, but it risks becoming:

- hard to memorize
- fragmented across domains
- awkward for daily personal use

Vel also should not collapse into pure chat input for routine work because that would:

- reduce predictability
- make scripting harder
- weaken user trust
- make approvals and side effects less legible

The command language should preserve CLI precision while making commands feel closer to how a person actually thinks:

- "Vel should feature ..."
- "Vel should capture ..."
- "Vel should remind me ..."
- "Vel should review ..."

## 3. Position in the Product

This is a **planned operator and daily-use surface**, not a replacement for the existing CLI.

It should layer on top of current Vel concepts from `docs/status.md`:

- captures
- commitments
- suggestions
- runs
- artifacts
- review/orientation flows

It should not invent a separate hidden domain model. The DSL is only a front door. The underlying execution still targets the same services and storage model.

## 4. Design Goals

- Make daily commands easier to remember than deep subcommand trees.
- Keep execution deterministic once a command is accepted.
- Provide rich autocomplete that teaches the grammar while the user types.
- Support direct mapping to domain objects and run-backed workflows.
- Make ambiguity visible and recoverable.
- Preserve machine-readable output and shell ergonomics.

## 5. Non-Goals

- Replace standard subcommands immediately.
- Turn Vel into a general-purpose shell.
- Hide side effects behind speculative LLM interpretation.
- Require network access or a remote model for basic command execution.
- Support arbitrary free-form natural language as a primary contract.

## 6. User Experience Model

The command language should feel like a **guided sentence editor**.

The user starts with a stable phrase head, then autocomplete narrows the valid continuations.

Examples:

```text
vel should …
  capture …
  feature …
  remind …
  review …
  synthesize …

vel should capture …
  "text"
  file <path>
  stdin
  url <url>

vel should feature …
  "text"
  --project <slug>
  --priority <low|normal|high>
```

The shell experience should support:

- tab completion of valid next tokens
- inline hints for expected objects
- preview of the resolved action before execution
- clarification prompts when multiple resolutions are plausible

## 7. Core Model

The language has three layers:

### 7.1 Phrase head

A small stable set of top-level intent phrases.

Initial phrase heads:

- `vel should ...`
- `vel show ...`
- `vel review ...`
- `vel sync ...`
- `vel inspect ...`

`vel should ...` is the most important because it covers the user's intent-oriented examples.

### 7.2 Intent verb

The next token after the phrase head selects a domain action.

Initial intent verbs:

- `capture`
- `feature`
- `commit`
- `remind`
- `review`
- `synthesize`
- `import`
- `explain`
- `spec`
- `plan`
- `delegate`

These verbs should map to explicit domain operations.

Later expansion can add uniform relation and lifecycle verbs such as:

- `link`
- `attach`
- `resolve`
- `show`
- `inspect`

### 7.3 Structured object tail

The remainder of the command fills a typed object shape:

- free text payload
- file path
- capture id
- commitment id
- date/window
- project slug
- source selector
- priority or privacy enum

The user should not need to remember raw JSON or long flag sets for common cases.

### 7.4 Uniform Type Integration

The command language should be designed to integrate with as many Vel object types as possible through one uniform mechanism.

It should not grow as a pile of one-off parsers for each command family.

The DSL should instead target a shared typed object registry covering current and future Vel domains such as:

- captures
- commitments
- runs
- run events
- artifacts
- refs
- signals
- nudges
- suggestions
- threads
- context snapshots
- planning/spec artifacts
- future work units or delegation records

Each domain should plug into the command language through the same concepts:

- type name
- aliases/synonyms
- identifier forms
- completion provider
- resolver
- preview renderer
- execution adapter

This matters because the command language will only remain coherent if new Vel types can be added without inventing a new command architecture each time.

## 8. Initial Semantics

### 8.1 `vel should capture`

Purpose: create a capture from text, file, stream, or URL.

Examples:

```text
vel should capture "follow up with Dimitri about timeline"
vel should capture file ./notes/meeting.md
vel should capture stdin
vel should capture url https://example.com/note
```

Compiles to existing capture/import flows:

- inline text -> `vel capture`
- `file` -> `vel import file`
- `stdin` -> `vel capture --stdin`
- `url` -> `vel capture-url`

### 8.2 `vel should feature`

Purpose: add a product or system improvement idea into a durable queue for later triage.

Examples:

```text
vel should feature "add thread-level waiting-on summaries"
vel should feature "show why this suggestion appeared" --project vel
```

Recommended initial implementation:

- create a typed capture with `capture_type=feature_request` or `capture_type=idea`
- optionally auto-promote to a commitment or suggestion candidate later

This command should not bypass the existing data model. If no dedicated feature queue exists yet, it should land as a structured capture with clear metadata.

### 8.3 `vel should commit`

Purpose: create an actionable commitment directly from intent text.

Examples:

```text
vel should commit "finish CLI DSL first draft"
vel should commit "review message backlog" --due today
```

Compiles to commitment creation.

### 8.4 `vel should remind`

Purpose: create or schedule a reminder-oriented commitment or nudge seed.

Examples:

```text
vel should remind me about meds at 9:00
vel should remind me to leave for Dimitri at 10:20
```

This should remain bounded. It is not a generic calendar assistant in v1.

### 8.5 `vel should review`

Purpose: invoke review/orientation flows in a sentence-like form.

Examples:

```text
vel should review today
vel should review week
vel should review recent captures
```

Compiles to existing review surfaces where available.

### 8.6 `vel should spec`

Purpose: create or draft a planning/spec artifact for a topic, subsystem, or workflow.

Examples:

```text
vel should spec cluster sync for apple offline mode
vel should spec command language autocomplete
```

This is a higher-order planning command. It should create a planned spec artifact or spec draft request, not imply that the described system already exists.

### 8.7 `vel should plan`

Purpose: generate an execution plan, implementation slice, or breakdown for a goal.

Examples:

```text
vel should plan transcript ingestion hardening
vel should plan command language parser rollout
```

This should compile to a typed planning request, optionally producing an artifact, linked commitments, and suggested work units.

### 8.8 `vel should delegate`

Purpose: produce a delegation plan or work-unit breakdown for bounded work.

Examples:

```text
vel should delegate transcript ingestion hardening
vel should delegate cluster sync research
```

This command should first create a delegation artifact or work plan. It should not silently dispatch real workers unless the user explicitly confirms a later execution step.

## 9. Syntax Shape

The syntax should be simple enough for shell completion and deterministic parsing.

Recommended grammar sketch:

```text
command          := "vel" phrase
phrase           := should_phrase | show_phrase | inspect_phrase | review_phrase | sync_phrase
should_phrase    := "should" should_verb should_object options*
should_verb      := "capture" | "feature" | "commit" | "remind" | "review" | "synthesize" | "import" | "explain" | "spec" | "plan" | "delegate"
should_object    := quoted_text | typed_object | keyword_object
typed_object     := "file" path
                  | "stdin"
                  | "url" url
                  | "today"
                  | "week"
                  | "recent" noun
keyword_object   := token+
options          := flag | key_value
```

Important constraint:

- every accepted command must resolve to a typed internal action before execution

The parser should never directly execute from raw token similarity alone.

## 10. Deep Autocomplete

Deep autocomplete is the defining UX feature.

It should not only complete words. It should complete **valid next structures**.

Examples:

```text
vel sh<TAB>
=> vel should

vel should <TAB>
=> capture  feature  commit  remind  review  synthesize  spec  plan  delegate

vel should capture <TAB>
=> "…"  file  stdin  url

vel should review <TAB>
=> today  week  recent captures
```

Autocomplete should draw from three sources.

### 10.1 Grammar-backed completion

Always available offline.

Used for:

- phrase heads
- verbs
- reserved keywords
- enum values
- expected object forms

### 10.2 State-backed completion

Uses local Vel state to complete meaningful identifiers.

Used for:

- project slugs
- recent captures
- commitment IDs or aliases
- known sources
- recent files or artifact types

### 10.3 LLM-backed assist

Optional and bounded.

Used for:

- repair suggestions when the input is nearly valid
- expansion of shorthand into a valid typed form
- ranked completions for text-heavy commands
- explaining why a command is invalid and how to fix it

Constraints:

- LLM assist must never be required for execution
- LLM output must be re-parsed by the deterministic grammar
- if the model proposes a rewrite, Vel should show the resolved command before running it

## 11. Inference Layer

The command language should not stop at parsing explicit tokens. It should also infer omitted settings where the system can do so safely.

The model is:

```text
input command
  -> deterministic parse
  -> typed intent
  -> inferred settings fill
  -> previewable resolved command
  -> execute or persist
```

This is especially important for higher-order commands like:

- `vel should spec ...`
- `vel should plan ...`
- `vel should delegate ...`

These commands often imply many settings that the user does not want to spell out.

### 11.1 Purpose

Inference should reduce friction by filling in:

- output kind
- likely subsystem
- likely doc or artifact location
- template choice
- related entities
- default constraints
- suggested follow-up actions

It should do this without turning execution into hidden magic.

### 11.2 Inferred Settings Contract

Each command resolution should produce:

```text
IntentResolution {
  explicit: { ... },
  inferred: { ... },
  assumptions: [ ... ],
  confidence: { field -> score },
  requires_confirmation: true | false
}
```

Recommended fields:

- `explicit`
  fields stated directly by the user
- `inferred`
  fields filled by grammar, local context, policy, or optional model assist
- `assumptions`
  human-readable statements of what Vel inferred
- `confidence`
  per-field confidence scores or bands
- `requires_confirmation`
  whether any inferred field is high-impact enough to block immediate execution

### 11.3 Sources of Inference

Inference should come from these layers, in order:

#### Grammar defaults

Examples:

- `vel should spec ...` -> default artifact kind is `spec`
- `vel should review today` -> default window is `today`

#### Domain policy

Examples:

- planning/spec commands default to `planned`, not `implemented`
- delegation commands default to proposal mode, not live dispatch

#### Local repo/runtime context

Examples:

- infer likely spec path under `docs/specs/`
- infer related subsystem from existing docs or known domains
- infer project slug from nearby references or active context

#### Optional model assist

Examples:

- classify a topic into a subsystem
- rank likely templates or related domains
- propose a clearer title or spec slug

Model assist must remain bounded and revalidated by deterministic rules.

### 11.4 Safe vs Unsafe Inference

Safe to infer automatically:

- doc template
- artifact kind
- output path suggestion
- related subsystem
- metadata labels
- likely linked docs or ticket packs
- planning status such as `planned`
- default output mode

Unsafe to infer silently:

- destructive actions
- real worker dispatch
- production rollout behavior
- external side effects
- approvals or credential use
- priority changes that materially reorder existing work

Unsafe inference should trigger preview and confirmation.

### 11.5 Preview Contract

Any command with meaningful inferred settings should support `--dry-run` and a readable preview.

Example:

```text
vel should spec cluster sync for apple offline mode --dry-run
```

Could resolve to:

```text
Intent: spec
Topic: cluster sync
Qualifier: apple offline mode

Inferred:
- artifact_kind: spec
- planning_status: planned
- subsystem: distributed_sync
- related_client_surface: apple_clients
- suggested_path: docs/specs/vel-cluster-sync-apple-offline-spec.md
- template: repo_spec

Assumptions:
- this request is for a planning document, not shipped behavior
- apple offline mode is related to cluster/degraded sync behavior

Execution:
- create or update a spec draft artifact
- suggest linked follow-up planning items
```

### 11.6 Higher-Order Commands

Some commands are not direct object mutations. They are requests to produce structured planning outputs.

These commands should compile into typed higher-order intents.

Recommended initial higher-order intents:

- `SpecIntent`
- `PlanIntent`
- `DelegationIntent`
- `BreakdownIntent`

These can then produce:

- spec artifacts
- execution plan artifacts
- linked commitments
- proposed work units
- dependency graphs
- suggested review checkpoints

### 11.7 Spec Command Inference

Example:

```text
vel should spec cluster sync for apple offline mode
```

Potential inferred settings:

- artifact kind: `spec`
- scope: `cluster_sync`
- related subsystem: `distributed_sync`
- related client domain: `apple_clients`
- output path: `docs/specs/...`
- status label: `planned`
- template: architecture/spec template

This should not directly claim that cluster sync or full Apple offline support is shipped. The inference layer must preserve the planned-vs-implemented boundary.

### 11.8 Delegation Command Inference

Example:

```text
vel should delegate transcript ingestion hardening
```

Potential inferred settings:

- intent kind: `delegation_plan`
- domain: `signals/transcripts`
- deliverables: `implementation plan`, `tests`, `doc updates`
- suggested work-unit split: `adapter`, `storage`, `api/cli`, `tests`
- dispatch mode: `proposal_only`

If actual worker dispatch exists later, that should be a separate confirmation or follow-up command.

### 11.9 Personal Learning

Over time, Vel may learn repeated defaults locally, such as:

- preferred doc locations
- preferred planning templates
- common subsystem names
- common delegation patterns

This learning should remain local-first and inspectable.

Vel should be able to show:

- what default was learned
- where it came from
- how to override or reset it

## 12. Uniform Type System Contract

The DSL should compile into a uniform typed layer that can address many Vel entity kinds in a consistent way.

Recommended shape:

```text
TypedTarget {
  kind: Capture | Commitment | Artifact | Run | Signal | Nudge | Suggestion | Thread | Context | SpecDraft | ExecutionPlan | DelegationPlan | ...,
  id: optional,
  selector: optional,
  attributes: { ... }
}
```

Every command should ultimately resolve against one or more `TypedTarget` values plus an operation.

This gives Vel a stable way to support commands like:

```text
vel show run <id>
vel inspect artifact latest context_brief
vel should link capture <id> to thread <id>
vel should resolve suggestion <id>
vel should spec cluster sync for apple offline mode
```

without each command family inventing new ad hoc object plumbing.

### 12.1 Type Registry

Vel should maintain a command-facing type registry that describes how each domain type participates in the DSL.

Recommended fields per registered type:

- canonical kind name
- accepted aliases
- valid selector forms
- ID parser
- autocomplete provider
- local resolver
- dry-run preview formatter
- execution adapter mapping to service calls

Example:

```text
TypeRegistryEntry {
  kind: "commitment",
  aliases: ["commitment", "todo", "task"],
  selectors: ["id", "latest", "open", "due-today"],
  completion_source: "commitments",
  resolver: CommitmentResolver,
  preview: CommitmentPreview,
  adapter: CommitmentCommandAdapter
}
```

### 12.2 Uniform Adapters

Each type should expose the same adapter surface to the DSL:

- parse target
- resolve target
- complete target
- explain target resolution
- execute supported operations

This prevents command behavior from fragmenting by subsystem.

### 12.3 Cross-Type Commands

The type system should support commands involving multiple object kinds in one sentence.

Examples:

```text
vel should link capture <capture_id> to commitment <commitment_id>
vel should attach artifact <artifact_id> to run <run_id>
vel should explain suggestion <suggestion_id> from signals
```

That implies the command layer should treat relations as first-class typed operations, not just unary object actions.

### 12.4 Planned Type Coverage

The command language should be planned as a repo-wide interface layer, not only a capture shortcut.

Near-term coverage should include:

- captures
- commitments
- runs
- artifacts
- context/review flows
- planning/spec artifacts

Medium-term coverage should include:

- signals
- nudges
- suggestions
- threads
- refs/provenance
- uncertainty records

Longer-term coverage may include:

- delegation/work units
- cluster/client nodes
- sync state
- voice/chat command intents as alternate frontends to the same typed layer

### 12.5 Status and Boundary Rules

Uniform integration across types does not mean every type gets full mutation support immediately.

For each type, the registry should distinguish:

- inspectable
- creatable
- updatable
- linkable
- explainable
- executable

This keeps the DSL honest about what is already implemented versus only planned.

## 13. Resolution Pipeline

The execution path should be:

1. Tokenize input.
2. Parse against the DSL grammar.
3. Build a typed intent from explicit fields.
4. Resolve identifiers and infer omitted settings from local state and policy.
5. If resolution yields a single valid action, build a typed internal command.
6. Show preview when side effects are non-trivial, inference is meaningful, or the user requested dry-run.
7. Execute via existing CLI service boundary.

If parse fails:

1. attempt grammar-aware repair
2. optionally use LLM assist to propose a bounded rewrite
3. require the rewritten form to parse cleanly
4. show a correction prompt rather than silently guessing

## 14. Internal Command Contract

The DSL should compile to a typed internal representation such as:

```text
IntentCommand {
  family: Should,
  verb: Capture | Feature | Commit | Remind | Review | Synthesize | Import | Explain | Spec | Plan | Delegate,
  target: ...,
  options: ...,
  source_text: "...",
  inferred: { ... },
  assumptions: [ ... ],
  resolution: {
    parser: deterministic | repaired,
    model_assisted: true | false,
    confirmation_required: true | false
  }
}
```

This matters for:

- auditing
- explainability
- testability
- future voice/chat reuse

Voice and chat surfaces should eventually reuse the same internal command contract rather than inventing separate action schemas.

## 15. Safety and Trust Rules

- The DSL must prefer explicit structured resolution over silent inference.
- Inference must populate typed fields, not opaque hidden behavior.
- Side-effecting commands should support `--dry-run`.
- Commands repaired with model help should show the final resolved form before execution.
- Commands with substantial inferred settings should show assumptions in dry-run or explain mode.
- Commands with multiple plausible targets should require disambiguation.
- Dangerous or destructive operations should stay in the traditional explicit CLI until there is a strong reason to add sentence syntax.

Examples of commands that should remain explicit at first:

- deleting artifacts
- bulk mutation
- schema/admin commands
- credential configuration

## 16. Explainability

Vel should be able to answer:

- what command family was recognized
- what internal action it compiled to
- what settings were inferred
- what assumptions were made
- whether any repair or model assistance occurred
- what objects were resolved from local state

Recommended operator affordances:

```text
vel should capture "note" --dry-run
vel explain command 'vel should feature "show run provenance"'
vel explain command 'vel should spec cluster sync for apple offline mode'
```

## 17. Storage and Data Model Impact

This spec should reuse current durable objects.

### Existing object mappings

- `capture` commands -> captures
- `commit` commands -> commitments
- `review` / `synthesize` commands -> runs + artifacts
- `spec` / `plan` / `delegate` commands -> planned artifacts first, with optional linked commitments

### Needed near-term addition

If `feature` becomes a first-class workflow, Vel should add one of:

- a dedicated `feature_request` capture type with metadata conventions
- a dedicated improvement queue entity later

The first step should be the typed capture approach because it preserves the current minimum viable slice.

For higher-order commands, a lightweight planning artifact model may be useful even before a full delegation runtime exists.

Examples:

- `artifact_type=spec_draft`
- `artifact_type=execution_plan`
- `artifact_type=delegation_plan`

These can later connect to more formal work-unit or swarm/domain models if those ship.

## 18. Output Modes

The command language should preserve normal CLI output modes:

- human-readable default
- `--json`
- `--dry-run`

Autocomplete and repair metadata should be available in JSON for testing.

Example:

```json
{
  "input": "vel should capture file ./notes/x.md",
  "parsed": true,
  "resolved_command": {
    "verb": "capture",
    "kind": "file_import",
    "path": "./notes/x.md"
  },
  "model_assisted": false
}
```

## 19. Testing Strategy

The DSL needs more than snapshot tests.

Required coverage:

- grammar parse tests
- type registry and adapter tests
- completion tests for each phrase head and verb
- repair tests for near-miss inputs
- resolution tests against fixture state
- inference tests for omitted settings and confidence bands
- cross-type command tests
- dry-run output tests
- end-to-end tests proving the DSL compiles to the same service calls as the existing CLI

Important invariant:

- sentence syntax must not create behavior that bypasses the existing domain/service boundaries

## 20. Incremental Rollout

### Phase 1

- add parser and dry-run mode
- support `should capture`, `should feature`, `should review`
- add inference contract with grammar and policy defaults
- add initial type registry for captures, commitments, runs, and artifacts
- grammar-backed completion only

### Phase 2

- add state-backed completion
- add `should commit` and `should remind`
- add `should spec` and `should plan`
- extend uniform adapters to signals, nudges, suggestions, and threads
- add explain/trace output for command resolution

### Phase 3

- add bounded LLM repair and ranked completions
- add `should delegate` as proposal mode
- add learned local defaults for repeated command patterns
- expose the same typed contract to chat and voice surfaces
- share internal command contract with chat and voice entrypoints

## 21. Open Questions

- Should `vel should feature` write only a capture at first, or also create a triageable commitment?
- Should `vel should commit` and `vel commitment add` remain fully equivalent, or should the DSL add smarter defaults like `--due today` inference?
- How much shell quoting burden is acceptable before a lightweight REPL or TUI becomes preferable?
- Should completion learn from personal repeated phrases locally, without requiring an LLM?
- Should there be an alias like `vel should do ...`, or is that too vague compared with `commit`?
- Should planning artifacts for `spec` / `plan` / `delegate` be represented as artifact types only, or should Vel add a dedicated planning/work-unit model?
- What confidence threshold should allow silent low-risk inference versus mandatory preview?
- Should the type registry live in `vel-core` as a shared domain contract, or at the CLI/service boundary as a command-facing projection of domain types?

## 22. Recommendation

Implement this as a **strict command language with rich completion and explicit inference**, not as natural-language execution.

The right hierarchy is:

1. deterministic grammar
2. local state and policy inference
3. uniform typed-object integration across Vel domains
4. optional LLM repair/ranking

The key rule is:

> infer as much as possible into typed settings, but do not hide assumptions or side effects.

That keeps Vel aligned with its product principles:

- repeated personal use over broad generality
- capture/review ergonomics over agent complexity
- trust and explainability over speculative magic
