# vel_documentation_index_and_implementation_status.md

Purpose: Ensure all discussed subsystems are documented and tracked.

## Core Engine
- commitments ✔
- current context ✔
- risk engine ✔
- policy engine ✔
- nudges ✔
- suggestions ✔
- synthesis ✔

## Attention & Drift
- drift detection spec ✔
- attention state model ✔

## Distributed Architecture
- node roles ✔
- replication strategy ✔
- offline behavior ✔

## Apple / Client Boundary
- Rust vs Swift boundary ✔
- Apple client architecture ✔

## Voice
- voice interaction spec ✔

## Chat Interface
- chat interface implementation brief ✔ (agent console, structured cards, provenance, phases 1–4)
- chat execution plan ✔ (domain model, API, WebSocket, React client, acceptance criteria)

## Integrations
- integration roadmap ✔

## Operator Tooling
- CLI cockpit spec ✔
- explain commands ✔

## Testing
- canonical day fixture ✔
- integration tests required ✔

## Remaining Implementation Work
1. ~~finish risk computation wiring~~ ✔
2. ~~drift detection heuristics~~ ✔ (morning_drift, prep_drift in context)
3. ~~suggestion trigger loop~~ ✔ (evaluate_after_nudges)
4. ~~project synthesis artifact generation~~ ✔
5. ~~CLI cockpit polish~~ ✔ (runs --kind/--today, explain --json, run status)
6. ~~first Apple client slice~~ ✔ (iOS/Watch/Mac bootstrap)
7. desktop voice push‑to‑talk (deferred)
8. ~~Canonical day fixture: full replay test suite~~ ✔ (fixture helper, §6.1–6.6 assertions, Variants A/B/C)
9. Chat interface (in progress) — Ticket pack: [docs/tickets/](tickets/) (35 tickets). Done through 033: core IDs, repos, /api/ (conversations, messages, inbox, interventions, message interventions, provenance, settings), WebSocket /ws + broadcast, React client (shell, conversation list, thread view, message composer, card renderer, inline actions, inbox, provenance drawer, settings page), migration 0024 settings, seed script. Remaining: 034 backend tests, 035 frontend tests. See [vel-chat-interface-implementation-brief.md](specs/vel-chat-interface-implementation-brief.md), [vel-chat-execution-plan.md](specs/vel-chat-execution-plan.md)

## Ticket packs and spec packs (incorporated)
- **Ticket packs:** [docs/tickets/README.md](tickets/README.md) — Chat (001–035), [agentic](tickets/agentic/), [ios-watch-monorepo](tickets/ios-watch-monorepo/), [ios-watch](tickets/ios-watch/), [visual-interface](tickets/visual-interface/), [repo-feedback](tickets/repo-feedback/) (001–009).
- **Product spec pack:** [docs/product-spec-pack/](product-spec-pack/) — architecture, surfaces, interaction, engines, design, flows.
- **Visual interface:** [docs/visual-interface/](visual-interface/), [specs/visual-interface-README.md](specs/visual-interface-README.md), repo [packages/](../packages/) (vel-affect-core, vel-visual-morphology, vel-protocol, vel-render-web, vel-render-watch).
- **Cognitive agent architecture:** [docs/cognitive-agent-architecture/](cognitive-agent-architecture/) — architecture (event-bus, state-graph), cognition (memory, context, introspection), agents, voice, devices, integrations, policies, metrics, implementation.
