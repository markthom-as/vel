---
id: SK-001
title: Build knowledge ingestion and indexing pipeline
status: proposed
priority: P0
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Implement the repo scanning and parsing pipeline that ingests code, docs, tests, migrations, and config into structured storage.

# Why

Without a durable ingestion layer, every later capability is just ad hoc grep wearing a fake mustache.

# Scope

Initial sources:

- `src/`
- `docs/`
- `tests/`
- `migrations/`
- root README / config files

Initial formats:

- Rust
- TypeScript
- Python
- Markdown
- JSON
- YAML
- SQL

# Tasks

1. Implement repo walker with file classification.
2. Add content hashing for incremental reindex support.
3. Parse code files using tree-sitter where practical.
4. Extract symbols, imports, exports, and top-level declarations.
5. Parse Markdown and frontmatter metadata.
6. Persist normalized file and entity records into SQLite.
7. Add index run bookkeeping and per-file parse diagnostics.

# Data outputs

Must populate at minimum:

- `files`
- `entities`
- `artifact_metadata`
- `index_runs`

# Acceptance Criteria

- `vel index rebuild` scans the repo and completes successfully on a clean checkout.
- Re-running with no file changes skips unchanged files.
- Symbol extraction works for representative Rust, TS, and Python samples.
- Markdown docs store title/status/frontmatter when present.
- Parse failures are captured as diagnostics rather than crashing the full run.

# Notes

Prefer explicit artifact typing over fuzzy inference. File extensions lie less often than prose.

