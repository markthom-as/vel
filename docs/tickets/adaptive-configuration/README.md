# Vel Adaptive Configuration — Full-Fat Pack

This pack expands the earlier adaptive configuration concept into a more implementation-ready design for the current Vel monorepo shape.

## Included

- `vel-adaptive-configuration-spec.md` — comprehensive spec
- `tickets/` — agent-ready implementation tickets with status frontmatter

## Goals

- durable user settings
- dynamic runtime overrides driven by context and policy
- deterministic resolution into effective config
- append-only auditability and replay
- operator-facing explainability
- conservative, testable rollout path

## Suggested implementation order

1. schema + migrations
2. typed config model + resolver
3. context signal ingestion
4. policy engine
5. effective config + explain endpoints
6. audit log + replay
7. operator/debug UI
8. shipped runtime profiles + defaults
9. SDK / client wiring
10. simulation tooling

## Notes

This pack assumes a Rust backend in `crates/veld`, route mounting in `app.rs`, docs in `docs/`, and numbered SQL migrations similar to the existing repo patterns discussed in this conversation.
