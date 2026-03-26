# AGENTS.md

This document defines durable repository rules for AI coding agents working in Vel.

Vel is mid-migration. Some current code still carries debt that violates the target architecture. Do not treat existing drift as precedent for new work.

## Authority

- **Implementation truth** lives in `docs/MASTER_PLAN.md`.
- **Workflow template** lives in `docs/templates/agent-implementation-protocol.md`.
- **Repo entrypoint** lives in `README.md`.
- **Web UI layout** (`core/`, `shell/`, `views/`) lives in `clients/web/src/README.md`.
- **User-facing behavior docs** live under `docs/user/`.
- `docs/README.md` is a navigation aid, not shipped-behavior authority. If it points to missing or stale files, defer to `docs/MASTER_PLAN.md` and the relevant phase ticket.
- Tickets and specs describe the intended change target. If they conflict with current shipped behavior, treat `docs/MASTER_PLAN.md` as the source of truth for what is implemented and the active ticket as the source of truth for what should change next.

## Mission

Vel is a local-first cognition runtime for capture, recall, daily orientation, and supervised execution.

Product principles:
- optimize for repeated personal use before broad generality
- prefer daily loops over speculative automation
- prefer capture/review ergonomics over agent complexity
- prefer trust, provenance, and export over opaque convenience

## Durable Repository Rules

### 1. Domain and Layering

- `vel-core` owns domain semantics, domain types, and invariants. Keep it free of transport and storage concerns.
- `vel-storage` must not depend on `vel-api-types`.
- `vel-api-types` is for transport DTOs only. Keep transport mapping at the boundary instead of pushing API DTO concerns deeper into services or storage.
- Route handlers should stay thin: parse/auth/request context, call a service, map the result to transport DTOs, map errors.
- Services hold application logic. Do not add new service APIs that return HTTP DTOs or embedded JSON strings.
- Let errors propagate to the boundary that can handle them correctly. Avoid defensive catch-and-continue logic in services unless there is a documented recovery path.
- Client surfaces (`clients/apple`, `clients/web`) are UI/operator shells. Policy, inference, and durable state rules belong in Rust backend layers unless a boundary doc explicitly says otherwise.

### 2. State, Consistency, and Data Shape

- Do not deepen `current_context` as an untyped JSON blob. Prefer versioned typed structs and keep serialization at the storage edge.
- Prefer structured payloads such as `serde_json::Value` over raw JSON strings in domain, service, and API layers.
- Run-backed operations must emit run lifecycle events and persist terminal state.
- Use prefixed identifiers and the ticketed sync/conflict rules consistently when touching distributed state.
- Every user-visible synthesis, suggestion, or action should remain explainable from persisted inputs, rules, or run events.
- Log and trace high-value execution boundaries: external calls, tool invocations, run transitions, and key routing decisions.

### 3. Secrets and Capability Boundaries

- Do not hand raw third-party credentials to agents if a mediated capability boundary can perform the action on their behalf.
- Prefer credential indirection: scoped tokens, brokered requests, or server-side injection at the execution boundary over prompt-visible secrets.
- Decrypt secrets only at the narrowest point of use, and never persist decrypted values in logs, prompts, traces, fixtures, or test snapshots.
- Scope external access as tightly as possible by host, path, tool, action, or resource. Avoid broad provider-wide access when a narrower capability will work.
- Unknown or unmatched external-access requests should fail closed by default.

### 4. Modularization and DRYness

- If a focused repository module already exists in `crates/vel-storage/src/`, extend it instead of adding unrelated behavior back into `db.rs`.
- If the same orchestration pattern appears in more than one route or service, extract it into an application service or shared helper instead of copying it again.
- Do not add new large scenario tests to `crates/veld/src/app.rs`. Prefer focused module tests or integration tests under `crates/veld/tests/`.
- Keep subsystem seams explicit between capture, memory, context, execution, sync, and interface layers.

### 5. Documentation and Contracts

- Docs must clearly distinguish implemented behavior from planned behavior.
- Place docs by role:
  - `.planning/` for active milestone work, execution packets, validation, and queued follow-on work
  - `docs/tickets/` for bounded implementation targets
  - `docs/future/` for future-only specs that are explicitly not shipped-behavior authority
  - `docs/notes/` for working notes, interview logs, and exploratory material that are not authority by themselves
  - `docs/cognitive-agent-architecture/` for accepted or active design-contract material; `status: draft` there is not proof of shipped behavior
- When changing a wire contract, update the Rust DTOs and the affected client boundary code in the same change.
- New schema-bearing or config-bearing surfaces should ship with four things together once the boundary is stable: owner documentation, a checked-in template or example, a machine-readable schema or manifest, and verification that the checked-in artifact still parses.
- Integration and connector work should use the canonical family, provider, source-mode, and capability vocabulary instead of inventing per-provider terminology ad hoc.
- Repo-aware or self-aware behavior must keep read scope separate from write scope. Observation and diagnosis may be broad; applied edits must stay task-bounded, traceable, and review-gated.
- When adding a new module, contract, endpoint, table, security boundary, or message-flow seam, document it close to the code and in the relevant ticket or doc entrypoint.
- If you encounter stale authority pointers while already touching the affected area, repair them instead of adding another shadow document.
- Record reusable lessons. If a task reveals a better prompt pattern, verification trick, or architectural guardrail, capture it in repo docs instead of leaving it in chat history only.
- Before creating a new doc, decide whether it is authority, active planning, future spec, or working note, and place it accordingly.

### 5.1 Language-Specific Agent Guidance

- For Rust work:
  - keep domain semantics in `vel-core`, storage logic in `vel-storage`, DTOs in `vel-api-types`, and application logic in `veld` services
  - keep route handlers thin and avoid returning transport DTOs from services
  - prefer typed structs/enums over new JSON blobs and keep serialization at boundaries
  - add focused crate, repository, service, or integration tests for changed behavior
- For JavaScript and TypeScript work:
  - keep client surfaces as shells over Rust-owned truth rather than inventing UI-only authority
  - preserve the existing `core/`, `shell/`, and `views/` boundaries and reuse shared primitives or view-model seams when logic repeats
  - verify interaction-heavy changes directly in the browser rather than relying on reasoning only
- For WASM and sandboxed runtime work:
  - keep manifests, allowlists, writable roots, and host-call boundaries explicit
  - never allow guests to widen filesystem or network scope after launch
  - treat WASM execution as a supervised runtime boundary, not a privileged bypass around backend policy

See `docs/cognitive-agent-architecture/agents/language-specific-agentic-coding-guidance.md` for the full Rust, JS/TS, and WASM-specific guidance.

### 6. Agentic Runtime Guidance

- Prefer a single orchestrator by default. Add multi-agent or multi-worker flows only when responsibilities are clearly separated, independently verifiable, and explicitly supervised.
- Keep tool boundaries explicit and traceable. New agentic flows should have stable run IDs, event trails, and provenance for major decisions.
- Default to local-first execution. Remote services should be optional, replaceable, and never required for core capture or recall loops.
- Subagents and delegated workers must use explicit allowlists for tools and capabilities. Do not give subordinate agents ambient access to everything by default.
- Agents must not be able to widen their own permissions, tool access, or sandbox boundaries through normal runtime actions.
- Code execution, plugin execution, or filesystem-heavy tool access must stay in dedicated sandboxes or isolated runtimes, not in the main authority process.
- Reuse known-good examples from this repository when possible. Prefer recombining existing tests, fixtures, handlers, and small utilities over inventing brand-new patterns from scratch.
- Pay down cognitive debt in complex areas. If a subsystem is hard to reason about, produce or update a concise walkthrough before or alongside substantial changes.
- Never treat unexecuted agent output as trustworthy. Code is only credible after it has been run through automated checks or direct manual execution.

## Agent Workflow

Before substantial implementation work, read:

1. `docs/MASTER_PLAN.md`
2. the relevant phase ticket in `docs/tickets/phase-*/`
3. `README.md`
4. the closest subtree guide or README for the surface you are touching
5. `config/README.md` if the task touches config, schemas, manifests, integrations, or policy surfaces
6. `docs/templates/agent-implementation-protocol.md` if the task needs process guidance

Then:

1. run the narrowest relevant existing tests first so the current baseline is known
2. implement the minimum viable slice first
3. preserve the target architecture even if nearby code has drift
4. for logic changes, prefer red/green TDD: add or extend a failing test first, then make it pass
5. add or update focused tests for the touched behavior
6. manually exercise the changed behavior as well as running automated checks
7. update docs when the change alters a module boundary, API contract, workflow, or authority pointer
8. if the change introduces or reshapes a durable contract, update the checked-in template/example and machine-readable schema/manifest in the same slice

The workflow protocol is normative about sequencing and verification intent, not literal tool names. Use the equivalent search, edit, and test tools available in your agent runtime.

## Review and Change Hygiene

- Do not hand off unreviewed agent-generated code as if review were someone else's job.
- Keep changes small enough to review efficiently. Prefer a sequence of narrow patches over one large mixed-purpose diff.
- Include concrete verification evidence in your summary: tests run, manual checks performed, and any important limits or gaps.
- For web and API work, prefer direct execution-based checks such as `curl`, targeted CLI invocations, or browser automation over purely speculative reasoning.
- When generating explanations or walkthroughs, prefer command-backed snippets and outputs over manually retyped excerpts.
- Do not rely on undocumented or internal behavior of third-party libraries unless that dependency is called out explicitly and justified.

## Security Defaults

- New HTTP endpoints should require authentication by default. Public routes should be rare, explicit, and documented.
- When there is doubt about exposure, prefer the safer default and require auth until a concrete public use case exists.
- Reject undefined or unsupported routes, tools, and external request patterns by default instead of silently allowing them through.

## Platform Ticket Preference

When choosing tickets or specs, prefer the pack closest to the active surface:

- prefer Apple tickets when working in `clients/apple` or Apple bridge code
- prefer runtime/storage/sync tickets when working in Rust daemon, CLI, or storage layers
- prefer web/operator tickets when working in `clients/web` or browser-facing packages
- for shared-core work, choose the ticket that governs the boundary you are changing most directly

If multiple tickets are relevant, start with the nearest architectural boundary and widen only when the change genuinely crosses layers.

## Priority Order

1. capture system
2. memory graph
3. context recall
4. daily alignment engine
5. execution automation

## Early Non-Goals

- complex distributed systems before the local-first core is solid
- unnecessary cloud dependencies
- premature optimization
- speculative productization features
- excessive UI complexity ahead of state correctness and trustworthiness
