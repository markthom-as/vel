# Commitments — Implementation Spec

First-class **commitment** object: something the user needs to remember, do, prepare for, or respond to. Distinct from a capture (raw input); commitments are actionable, reviewable, and statusful.

## Schema

```sql
CREATE TABLE commitments (
  id TEXT PRIMARY KEY,
  text TEXT NOT NULL,
  source_type TEXT NOT NULL,
  source_id TEXT,
  status TEXT NOT NULL,
  due_at INTEGER,
  project TEXT,
  commitment_kind TEXT,
  created_at INTEGER NOT NULL,
  resolved_at INTEGER,
  metadata_json TEXT NOT NULL
);

CREATE INDEX idx_commitments_status ON commitments(status);
CREATE INDEX idx_commitments_due_at ON commitments(due_at);
CREATE INDEX idx_commitments_project ON commitments(project);
CREATE INDEX idx_commitments_source ON commitments(source_type, source_id);
CREATE INDEX idx_commitments_created_at ON commitments(created_at);
```

Timestamps (`created_at`, `resolved_at`, `due_at`) are stored as Unix seconds (INTEGER).

## Allowed statuses (v1)

- `open`
- `done`
- `cancelled`

Snoozing belongs to nudges, not commitments; do not add `snoozed` as a commitment status.

## API

- **Create** — `POST /v1/commitments`  
  Body: `text`, `source_type` (default `manual`), `source_id`, `due_at`, `project`, `commitment_kind`, `metadata`.

- **List** — `GET /v1/commitments`  
  Query: `status`, `project`, `kind`, `limit` (default 50). At minimum, `?status=open` is supported.

- **Inspect** — `GET /v1/commitments/:id`

- **Update** — `PATCH /v1/commitments/:id`  
  Allowed in v1: `status`, `due_at`, `project`, `commitment_kind`, `metadata`. Setting `status=done` or `status=cancelled` sets `resolved_at` automatically.

## CLI

- `vel commitments` — list commitments (default: open), optional `--status`, `--project`, `--limit`, `--json`.
- `vel commitment add "text"` — `--kind`, `--project`.
- `vel commitment done <id>`
- `vel commitment cancel <id>`
- `vel commitment inspect <id>`

## Creation rules (v1)

A commitment is created in one of three ways:

1. **Explicit** — `vel commitment add ...`
2. **Capture promotion** — any capture with `capture_type == "todo"` auto-creates an open commitment (source_type `capture`, source_id = capture_id, commitment_kind `todo`).
3. **External source** — (Phase B) Todoist/task import creates commitments.

Do **not** implement fuzzy LLM-based extraction from arbitrary captures in this phase; keep the rule explicit and inspectable.

## Source integration role

Commitments are the normalized “what matters” layer. Calendar and task sources (Phase B) will create or update commitments; nudge and morning context (Phase C) will consume open commitments and due/prep windows.
