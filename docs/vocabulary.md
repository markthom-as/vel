# Vel Vocabulary

This page is the first-class glossary, vocabulary index, and appendix for Vel terminology.

- Use it when you need the canonical meaning of a Vel term.
- Use `crates/vel-core/src/vocabulary.rs` when you need the same vocabulary programmatically.
- Treat [status.md](status.md) as the source of truth for shipped behavior; this page defines terms, not rollout truth.

## Why this exists

Vel uses the same words across docs, CLI, API, storage, and the structured command language. This glossary makes those words explicit so docs and code do not drift independently.

Programmatic source:

- `vel_core::glossary_entries()`
- `vel_core::glossary_entry_for_kind(...)`
- `vel_core::dsl_registry_entries()`

Those APIs currently feed the command-language registry in `vel-cli`.

## Core Terms

### capture

A durable user or system input such as a note, transcript-derived item, or imported document fragment.

- Aliases: `note`, `inbox item`
- Related: `artifact`, `commitment`, `signal`
- DSL selectors: `id`, `latest`, `recent`
- DSL operations: `create`, `inspect`, `list`, `link`, `explain`

### commitment

An actionable, reviewable item that matters over time and remains distinct from a raw capture.

- Aliases: `todo`, `task`
- Related: `capture`, `thread`, `nudge`
- DSL selectors: `id`, `open`, `due_today`, `latest`
- DSL operations: `create`, `inspect`, `list`, `update`, `link`, `explain`

### run

A tracked execution record for a run-backed workflow, including lifecycle state and emitted events.

- Aliases: `job`
- Related: `artifact`, `run event`, `context`
- DSL selectors: `id`, `latest`, `today`
- DSL operations: `inspect`, `list`, `update`, `explain`

### artifact

A durable output or external reference produced by Vel, such as a context brief or synthesis file.

- Aliases: `output`
- Related: `capture`, `run`, `ref`
- DSL selectors: `id`, `latest`, `type`
- DSL operations: `create`, `inspect`, `list`, `link`, `explain`

### thread

A first-class continuity object for an active line of work, conversation, or theme.

- Related: `commitment`, `context`, `artifact`
- DSL selectors: `id`, `open`, `latest`
- DSL operations: `create`, `inspect`, `list`, `update`, `link`, `explain`

### context

The current orientation picture Vel computes from captures, commitments, integrations, and recent runs.

- Aliases: `current context`, `brief`
- Related: `run`, `artifact`, `review`
- DSL selectors: `today`, `morning`, `end_of_day`
- DSL operations: `inspect`, `execute`, `explain`

### spec draft

A planning artifact for a proposed design or documentation slice that is not implementation truth by itself.

- Aliases: `spec`, `design doc`
- Related: `execution plan`, `artifact`
- DSL selectors: `topic`, `latest`
- DSL operations: `create`, `inspect`, `list`, `explain`

### execution plan

A planning artifact that breaks implementation work into an ordered slice, distinct from shipped behavior.

- Aliases: `plan`, `implementation plan`
- Related: `spec draft`, `artifact`
- DSL selectors: `topic`, `latest`
- DSL operations: `create`, `inspect`, `list`, `explain`

## Command Language Terms

The current structured command language uses the `should` phrase family.

### should

The explicit command-language family for intent-oriented commands.

Example:

```bash
vel command should capture remember the buffer before the meeting
```

### capture

Creates a capture from inline text.

```bash
vel command should capture remember to inspect the latest run
```

### feature

Creates a feature-request capture.

```bash
vel command should feature add glossary search
```

### commit

Creates an open commitment from inline text.

```bash
vel command should commit write the glossary appendix
```

### review

Runs a read-oriented review flow against supported scopes such as `today` and `week`.

```bash
vel command should review today
```

### spec

Creates a planned spec-draft artifact intent.

```bash
vel command should spec command language glossary
```

### plan

Creates a planned execution-plan artifact intent.

```bash
vel command should plan repo glossary rollout
```

### delegate

Reserved for delegation-oriented flows. It is recognized in the DSL vocabulary but not yet executable.

## Index Hints

Common lookup phrases:

- glossary
- vocabulary
- appendix
- terminology
- command language terms
- domain terms
- DSL nouns
- DSL verbs
