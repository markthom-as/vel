---
title: Observability and Debuggability Upgrade
status: proposed
priority: medium
owner: codex
---

# Goal

Improve operator and developer visibility into why Vel made a decision.

# Concrete code changes

## A. Add evaluation summary artifact or event
After `POST /v1/evaluate`, persist a concise structured summary:
- risk_count
- inferred_state_written
- nudge_count_created
- suggestion_count_created
- current_context_hash

This can be a run output payload or a dedicated event payload. Keep it simple.

## B. Add `vel debug evaluate`
Update CLI:
- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/client.rs`

This command should print:
- latest current context summary
- latest top risks
- active nudges
- pending suggestions

## C. Add a compact explain surface for suggestions
Future work depends on inspectable suggestions. Start with:
- `GET /v1/suggestions/:id/evidence` or include evidence in `GET /v1/suggestions/:id`

If storage work from Ticket 005 lands, use it here.

# Acceptance criteria

- operator can inspect the full evaluation outcome without spelunking raw tables
- suggestion provenance is easier to inspect
- debug output stays deterministic and non-LLM
