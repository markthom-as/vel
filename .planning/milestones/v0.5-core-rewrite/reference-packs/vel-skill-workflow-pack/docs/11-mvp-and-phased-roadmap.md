# MVP and Phased Roadmap

## Strategic framing

Do not build the whole imagined ecosystem at once.

The right move is to build a narrow but real internal substrate that can already support Vel features in production-ish form, then iterate outward.

## Phase 0 — Design and architecture freeze

### Goals

- define core concepts and boundaries
- ratify manifest shape
- define capability taxonomy
- define registry resolution order
- define CLI naming conventions
- define execution record shape

### Deliverables

- approved spec docs
- starter manifest schema
- initial capability namespace list
- package layout convention
- examples for 2–3 skills

### Exit criteria

- team alignment on skill/tool/agent distinction
- agreement that Vel-native IR is source of truth

## Phase 1 — MVP: internal prompt-first skill runtime

### Scope

This is the real MVP.

### Must-have capabilities

- discover skills from filesystem
- parse and validate `skill.yaml`
- support prompt-only skills
- support input/output schema validation
- expose CLI commands:
  - `list`
  - `inspect`
  - `run`
  - `validate`
- mount minimal typed context buckets
- log execution metadata
- support skill enable/disable config

### Recommended MVP constraints

- no arbitrary third-party remote registry yet
- no code hooks yet, or only behind internal flag
- no workflow engine yet
- no full external compatibility yet
- no write-capability auto-execution

### Good MVP candidate skills

- `core/daily-brief`
- `core/task-triage`
- `core/generate-standup`
- `core/context-pack`

### Why this phase matters

It proves the abstraction without overfitting to future complexity.

### Exit criteria

- Vel can run at least 3 internal skills through a common CLI/runtime path
- skills have working schemas and tests
- basic policy enforcement exists for read-only capability gating

## Phase 2 — Hybrid skill support and stronger policy

### Scope

Add code-backed utility without exploding trust boundaries.

### Capabilities

- `prepare` and `cleanup` hooks
- bounded subprocess hook protocol
- stronger capability grant logic
- confirmation handling for mutating actions
- artifact emission
- richer run logs and traces
- dry-run mode

### Good candidate skills

- `integrations/google-calendar-reconcile`
- `integrations/todoist-enrich`
- `local/repo-standup`

### Exit criteria

- at least 2 hybrid skills in production-ish use
- permission requests and grants visible in logs
- artifacts emitted and inspectable

## Phase 3 — Composition and workflows

### Scope

Support skill chaining and reusable multi-step execution.

### Capabilities

- workflow skill type
- sequential steps
- structured output passing
- conditional branching
- parent/child trace linkage
- retry policies
- degraded execution behavior

### Good candidate workflows

- morning startup pipeline
- engineering standup pipeline
- weekly review pipeline

### Exit criteria

- at least one multi-step workflow running through shared runtime
- nested permission and trace model functioning

## Phase 4 — UI integration and richer routing

### Scope
nExpose skills in the product surfaces cleanly.

### Capabilities

- skill browser/inspector in settings or developer panel
- enable/disable toggles by workspace or surface
- invocation from Now page / chat / voice / automations
- routing hints for planner layer
- surfacing skill status and policy prompts in UI

### Exit criteria

- a user can discover and toggle skills in product UI
- at least one voice or Now-surface invocation runs through the skill runtime

## Phase 5 — External compatibility and packaging

### Scope

Broaden ecosystem interoperability without giving up internal coherence.

### Capabilities

- import adapter for external prompt/skill packs
- export adapter for partial interoperability
- registry syncing from git repos
- signatures or provenance metadata
- compatibility reporting

### Exit criteria

- at least one external pack format partially importable
- imported packs can be inspected and safely sandboxed

## Phase 6 — Advanced runtime and ecosystem growth

### Scope

This is where things get spicy, but only after the substrate is stable.

### Capabilities

- WASM execution support
- remote registries
- richer policy profiles by device/surface
- hot reload in development
- visual workflow authoring
- cost-aware routing and caching
- marketplace or shared internal registry

## Recommended development order of concern

1. concept clarity
2. filesystem packaging
3. runtime validation
4. CLI execution
5. policy mediation
6. context mounting
7. hybrid hooks
8. workflows
9. UI integration
10. compatibility adapters
11. ecosystem expansion

## Biggest risk to avoid

Trying to build phase 4–6 dreams in phase 1 clothing.

That is how good architecture gets strangled by fantasy pre-optimization.
