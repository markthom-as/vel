# Vel Skill System Spec Pack

This pack proposes a **toggleable, pluggable, CLI-compatible skill system** for Vel, designed to be Vel-native first and only later adapted outward for Claude/Codex-style compatibility.

## Included

- `docs/00-overview.md` — executive summary and architectural stance
- `docs/01-product-goals-and-principles.md` — goals, non-goals, design principles
- `docs/02-core-concepts.md` — skills vs tools vs agents vs workflows
- `docs/03-skill-package-spec.md` — package layout, manifest model, lifecycle
- `docs/04-runtime-architecture.md` — runtime, registry, execution, hooks, artifacts
- `docs/05-permissions-and-policy.md` — capabilities, grants, confirmation, least privilege
- `docs/06-context-and-data-mounting.md` — typed context model and prompt hygiene
- `docs/07-cli-design.md` — command surface and UX model for CLI integration
- `docs/08-workflows-composition-and-orchestration.md` — chaining and composition
- `docs/09-compatibility-strategy.md` — external compatibility shims and adapters
- `docs/10-testing-observability-and-versioning.md` — tests, logs, telemetry, rollout
- `docs/11-mvp-and-phased-roadmap.md` — phased development plan from MVP onward
- `docs/12-implementation-notes-rust-core.md` — recommended internal implementation shape
- `docs/13-authoring-guide.md` — how internal skill authors should build skills
- `schemas/skill.schema.json` — starter manifest schema
- `schemas/input.schema.json` — sample input schema
- `schemas/output.schema.json` — sample output schema
- `examples/skills/...` — concrete example skill packages
- `tickets/` — Codex-ready implementation tickets

## Recommended first read order

1. `docs/00-overview.md`
2. `docs/02-core-concepts.md`
3. `docs/03-skill-package-spec.md`
4. `docs/04-runtime-architecture.md`
5. `docs/05-permissions-and-policy.md`
6. `docs/07-cli-design.md`
7. `docs/11-mvp-and-phased-roadmap.md`
8. `tickets/`

## Core recommendation in one sentence

Build **Vel-native, typed, permissioned, composable skill packages** with a stable runtime and CLI first, then layer external compatibility adapters on top.
