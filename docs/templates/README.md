# Vel Documentation Templates

This directory contains canonical starting templates for repo documents that should be easy to index in local docs tooling, wiki search, and plain `rg`.

Use these templates when creating new:

- general documentation pages,
- specs,
- tickets,
- prompts and runbooks.

## Indexing rules

Every template here includes:

- YAML front matter for structured metadata,
- a `keywords` list for exact search terms,
- an `index_terms` list for alternate names and likely lookup phrases,
- a short summary/purpose section near the top,
- explicit links or placeholders for related files.

That combination makes the files easier to find through:

- grep/ripgrep,
- static site or wiki indexing,
- knowledge-graph or repo-index tooling,
- simple full-text search on titles plus likely aliases.

## Template set

- [doc-template.md](doc-template.md) for general documentation pages
- [spec-template.md](spec-template.md) for design and architecture specs
- [schema-template.md](schema-template.md) for machine-readable schema and manifest specs
- [ticket-template.md](ticket-template.md) for execution tickets
- [prompt-template.md](prompt-template.md) for reusable prompts, runbooks, and operator prompt assets
- [mvp-loop-contract-checklist.md](mvp-loop-contract-checklist.md) for reviewing MVP-loop scope, contract, and shell-boundary changes against the locked `v0.2` rules

## Usage notes

- Keep `keywords` concrete and short. Prefer nouns and exact subsystem names.
- Use `index_terms` for alternate spellings, abbreviations, aliases, and likely lookup phrases.
- Do not stuff keywords with every possible word. High-signal terms are better than noisy lists.
- Keep `status` truthful. Do not mark planning docs as implemented.
- For architecture docs and tickets, explicitly account for the cross-cutting system traits: modularity, accessibility, configurability, data logging, rewind/replay, and composability.
- If the document affects shipped behavior, reconcile it with [MASTER_PLAN.md](../MASTER_PLAN.md).
- Use `docs/cognitive-agent-architecture/` for durable concept or architecture docs, `docs/tickets/phase-*/` for execution work, and the `docs/user/` or `docs/api/` trees for operator-facing documentation.
- When adding a new ticket or changing queue shape, update both [tickets/README.md](../tickets/README.md) and [MASTER_PLAN.md](../MASTER_PLAN.md).
