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

These verbs should map to explicit domain operations.

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

## 9. Syntax Shape

The syntax should be simple enough for shell completion and deterministic parsing.

Recommended grammar sketch:

```text
command          := "vel" phrase
phrase           := should_phrase | show_phrase | inspect_phrase | review_phrase | sync_phrase
should_phrase    := "should" should_verb should_object options*
should_verb      := "capture" | "feature" | "commit" | "remind" | "review" | "synthesize" | "import" | "explain"
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
=> capture  feature  commit  remind  review  synthesize

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

## 11. Resolution Pipeline

The execution path should be:

1. Tokenize input.
2. Parse against the DSL grammar.
3. If parse succeeds, resolve identifiers against local state.
4. If resolution yields a single valid action, build a typed internal command.
5. Show preview when side effects are non-trivial or the user requested dry-run.
6. Execute via existing CLI service boundary.

If parse fails:

1. attempt grammar-aware repair
2. optionally use LLM assist to propose a bounded rewrite
3. require the rewritten form to parse cleanly
4. show a correction prompt rather than silently guessing

## 12. Internal Command Contract

The DSL should compile to a typed internal representation such as:

```text
IntentCommand {
  family: Should,
  verb: Capture | Feature | Commit | Remind | Review | Synthesize | Import | Explain,
  target: ...,
  options: ...,
  source_text: "...",
  resolution: {
    parser: deterministic | repaired,
    model_assisted: true | false
  }
}
```

This matters for:

- auditing
- explainability
- testability
- future voice/chat reuse

Voice and chat surfaces should eventually reuse the same internal command contract rather than inventing separate action schemas.

## 13. Safety and Trust Rules

- The DSL must prefer explicit structured resolution over silent inference.
- Side-effecting commands should support `--dry-run`.
- Commands repaired with model help should show the final resolved form before execution.
- Commands with multiple plausible targets should require disambiguation.
- Dangerous or destructive operations should stay in the traditional explicit CLI until there is a strong reason to add sentence syntax.

Examples of commands that should remain explicit at first:

- deleting artifacts
- bulk mutation
- schema/admin commands
- credential configuration

## 14. Explainability

Vel should be able to answer:

- what command family was recognized
- what internal action it compiled to
- whether any repair or model assistance occurred
- what objects were resolved from local state

Recommended operator affordances:

```text
vel should capture "note" --dry-run
vel explain command 'vel should feature "show run provenance"'
```

## 15. Storage and Data Model Impact

This spec should reuse current durable objects.

### Existing object mappings

- `capture` commands -> captures
- `commit` commands -> commitments
- `review` / `synthesize` commands -> runs + artifacts

### Needed near-term addition

If `feature` becomes a first-class workflow, Vel should add one of:

- a dedicated `feature_request` capture type with metadata conventions
- a dedicated improvement queue entity later

The first step should be the typed capture approach because it preserves the current minimum viable slice.

## 16. Output Modes

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

## 17. Testing Strategy

The DSL needs more than snapshot tests.

Required coverage:

- grammar parse tests
- completion tests for each phrase head and verb
- repair tests for near-miss inputs
- resolution tests against fixture state
- dry-run output tests
- end-to-end tests proving the DSL compiles to the same service calls as the existing CLI

Important invariant:

- sentence syntax must not create behavior that bypasses the existing domain/service boundaries

## 18. Incremental Rollout

### Phase 1

- add parser and dry-run mode
- support `should capture`, `should feature`, `should review`
- grammar-backed completion only

### Phase 2

- add state-backed completion
- add `should commit` and `should remind`
- add explain/trace output for command resolution

### Phase 3

- add bounded LLM repair and ranked completions
- share internal command contract with chat and voice entrypoints

## 19. Open Questions

- Should `vel should feature` write only a capture at first, or also create a triageable commitment?
- Should `vel should commit` and `vel commitment add` remain fully equivalent, or should the DSL add smarter defaults like `--due today` inference?
- How much shell quoting burden is acceptable before a lightweight REPL or TUI becomes preferable?
- Should completion learn from personal repeated phrases locally, without requiring an LLM?
- Should there be an alias like `vel should do ...`, or is that too vague compared with `commit`?

## 20. Recommendation

Implement this as a **strict command language with rich completion**, not as natural-language execution.

The right hierarchy is:

1. deterministic grammar
2. local state resolution
3. optional LLM repair/ranking

That keeps Vel aligned with its product principles:

- repeated personal use over broad generality
- capture/review ergonomics over agent complexity
- trust and explainability over speculative magic
