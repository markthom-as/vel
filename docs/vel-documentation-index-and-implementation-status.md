# Vel documentation index and coverage

Purpose: act as a **navigation and coverage index**. Canonical implementation truth lives in `docs/status.md`.

Start with [docs/README.md](README.md) if you need the shortest authoritative reading path.

- For repo-wide implementation status, see **`docs/status.md`**.
- This document tracks which subsystems are documented and where to find their specs and tickets.

## Core Engine
- commitments — [spec](specs/vel-migrations-and-schema-spec.md), [status](status.md)
- current context — [spec](specs/vel-current-context-spec.md), [status](status.md) (web operator surface now uses explain-oriented context data)
- risk engine — [spec](specs/vel-risk-engine-spec.md), [status](status.md)
- policy engine — [spec](specs/vel-policy-engine-spec.md), [status](status.md)
- nudges — [spec](specs/vel-policy-engine-spec.md), [status](status.md)
- suggestions — [spec](specs/vel-agent-next-implementation-steps.md), [status](status.md)
- synthesis — [spec](specs/vel-weekly-synthesis-spec.md), [status](status.md)

## Attention & Drift
- drift detection spec — [spec](specs/vel-attention-and-drift-detection-spec.md)
- attention state model — [status](status.md)

## Distributed Architecture
- node roles / replication / offline — [spec](specs/vel-distributed-and-ambient-architecture-spec.md)

## Apple / Client Boundary
- Rust vs Swift boundary — [spec](specs/vel-rust-swift-boundary-spec.md)
- Apple client architecture & bootstrap — [spec](specs/vel-apple-and-voice-client-spec.md), [status](status.md)

## Voice
- voice interaction spec — [spec](specs/vel-voice-interaction-spec.md)

## Chat Interface
- chat implementation brief — [spec](specs/vel-chat-interface-implementation-brief.md)
- chat execution plan — [spec](specs/vel-chat-execution-plan.md)
- ticket pack — [tickets](tickets/README.md) (Chat 001–035)
- subsystem status detail — [chat-interface-status-and-outstanding.md](chat-interface-status-and-outstanding.md) (shipped websocket contract, explain-backed context panel, hydrated message provenance, and follow-up work)

## Integrations
- integration roadmap — [spec](specs/vel-integration-priority-and-adapter-roadmap.md)

## Operator Tooling
- CLI cockpit spec — [spec](specs/vel-operator-cockpit-spec.md)
- command language / structured DSL — [spec](specs/vel-command-language-spec.md), [status](status.md)
- command language implementation note — [spec](specs/vel-command-language-implementation-note.md), [status](status.md)
- command language Rust layout — [spec](specs/vel-command-language-rust-layout.md), [status](status.md)
- terminology, glossary, and DSL/domain vocabulary — [docs](vocabulary.md)
- thread graph / planning metadata — [spec](specs/vel-thread-graph-spec.md), [status](status.md)
- explain commands — [status](status.md)

## User Documentation
- full-fat end-user documentation target — [spec](specs/vel-user-documentation-spec.md)
- shipped implementation truth for user-facing maturity claims — [status](status.md)
- current user-docs entrypoint — [docs/user/README.md](user/README.md)
- planned web/operator UX convergence — [spec](specs/vel-web-operator-surface-spec.md), [tickets](tickets/web-ui-convergence/README.md)

## Projects and Workspaces
- projects surface and shared workspace contract — [spec](specs/vel-projects-page-spec.md), [tickets](tickets/projects/README.md)
- project registry, normalized tags, dependencies, and routines substrate — [spec](specs/vel-project-operations-substrate-spec.md), [tickets](tickets/projects/README.md)

## Testing
- canonical day fixture — [spec](specs/vel-canonical-day-fixture-spec.md)
- integration tests — see [status](status.md) for current coverage

## Ticket packs and spec packs
- **Ticket packs:** [docs/tickets/README.md](tickets/README.md) — Chat (001–037), [repo-feedback](tickets/repo-feedback/), [vel-docs](tickets/vel-docs/), [repo-audit-hardening](tickets/repo-audit-hardening/), [web-ui-convergence](tickets/web-ui-convergence/), [agentic](tickets/agentic/), [agent-runtime](tickets/agent-runtime/), [ios-watch-monorepo](tickets/ios-watch-monorepo/), [visual-interface](tickets/visual-interface/), [uncertainty](tickets/uncertainty/), [adaptive-configuration](tickets/adaptive-configuration/), [metadata-enrichment](tickets/metadata-enrichment/), [projects](tickets/projects/), [self-knowledge](tickets/self-knowledge/), [multi-client-swarm](tickets/multi-client-swarm/), [integration-expansion](tickets/integration-expansion/), [ticket-object](tickets/ticket-object/), [storage-backup-sync](tickets/storage-backup-sync/), [connect-agent-launch](tickets/connect-agent-launch/).
- **Product spec pack:** [docs/product-spec-pack/](product-spec-pack/) — architecture, surfaces, interaction, engines, design, flows, plus imported packets preserved under [docs/product-spec-pack/imported/](product-spec-pack/imported/).
- **Visual interface:** [docs/visual-interface/](visual-interface/), [specs/visual-interface-README.md](specs/visual-interface-README.md), repo [packages/](../packages/) (vel-affect-core, vel-visual-morphology, vel-protocol, vel-render-web, vel-render-watch).
- **Cognitive agent architecture:** [docs/cognitive-agent-architecture/](cognitive-agent-architecture/) — architecture (event-bus, state-graph), cognition (memory, context, introspection), agents, voice, devices, integrations, policies, metrics, implementation.
- **LLM backend plan:** [docs/llm-backend-plan/](llm-backend-plan/) — architecture, local model serving, OpenAI/OAuth backend, implementation tickets, examples (Rust trait, toml configs, run scripts).
- **Context reasoning:** [docs/specs/vel-context-decision-spec.md](specs/vel-context-decision-spec.md), [tickets/context-reasoning/](tickets/context-reasoning/).
- **Self-knowledge:** [docs/specs/vel-self-knowledge-system-spec.md](specs/vel-self-knowledge-system-spec.md), [tickets/self-knowledge/](tickets/self-knowledge/).
- **Task HUD:** [docs/specs/vel-task-hud-spec.md](specs/vel-task-hud-spec.md), [tickets/task-hud/](tickets/task-hud/).
