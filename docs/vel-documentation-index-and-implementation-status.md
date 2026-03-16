# Vel documentation index and coverage

Purpose: act as a **navigation and coverage index**. Canonical implementation truth lives in `docs/status.md`.

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
- explain commands — [status](status.md)

## Testing
- canonical day fixture — [spec](specs/vel-canonical-day-fixture-spec.md)
- integration tests — see [status](status.md) for current coverage

## Ticket packs and spec packs
- **Ticket packs:** [docs/tickets/README.md](tickets/README.md) — Chat (001–035), [agentic](tickets/agentic/), [ios-watch-monorepo](tickets/ios-watch-monorepo/), [ios-watch](tickets/ios-watch/), [visual-interface](tickets/visual-interface/), [repo-feedback](tickets/repo-feedback/), [context-reasoning](tickets/context-reasoning/), [agent-runtime](tickets/agent-runtime/), [uncertainty](tickets/uncertainty/), [adaptive-configuration](tickets/adaptive-configuration/), [metadata-enrichment](tickets/metadata-enrichment/), [task-hud](tickets/task-hud/), [self-knowledge](tickets/self-knowledge/), [orchestration](tickets/orchestration/).
- **Product spec pack:** [docs/product-spec-pack/](product-spec-pack/) — architecture, surfaces, interaction, engines, design, flows.
- **Visual interface:** [docs/visual-interface/](visual-interface/), [specs/visual-interface-README.md](specs/visual-interface-README.md), repo [packages/](../packages/) (vel-affect-core, vel-visual-morphology, vel-protocol, vel-render-web, vel-render-watch).
- **Cognitive agent architecture:** [docs/cognitive-agent-architecture/](cognitive-agent-architecture/) — architecture (event-bus, state-graph), cognition (memory, context, introspection), agents, voice, devices, integrations, policies, metrics, implementation.
- **LLM backend plan:** [docs/llm-backend-plan/](llm-backend-plan/) — architecture, local model serving, OpenAI/OAuth backend, implementation tickets, examples (Rust trait, toml configs, run scripts).
- **Context reasoning:** [docs/specs/vel-context-decision-spec.md](specs/vel-context-decision-spec.md), [tickets/context-reasoning/](tickets/context-reasoning/).
- **Self-knowledge:** [docs/specs/vel-self-knowledge-system-spec.md](specs/vel-self-knowledge-system-spec.md), [tickets/self-knowledge/](tickets/self-knowledge/).
- **Task HUD:** [docs/specs/vel-task-hud-spec.md](specs/vel-task-hud-spec.md), [tickets/task-hud/](tickets/task-hud/).
