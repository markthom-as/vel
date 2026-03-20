# Phase 8: Coding-centric supervised execution with GSD and local agents - Research

**Researched:** 2026-03-19
**Domain:** project-linked coding execution context, repo-local GSD handoff artifacts, supervised local runtime launch, connect transport closure, explicit handoff review, direct WASM guest follow-on
**Confidence:** HIGH

## Summary

Phase 8 should not invent a second planning system beside GSD, and it should not make Vel an ambient code-writing authority. The right move is to reuse the Phase 5 project substrate, Phase 3 trace/handoff contracts, the existing `vel-protocol` and `vel-agent-sdk` baseline, and the Phase 2/4 re-scoped runtime gaps. Vel should become the supervised execution coordinator around coding work: it persists repo-aware execution context, emits explicit handoffs, generates repo-local artifacts that GSD can consume, activates authenticated connect transport, and keeps local or WASM-backed runtimes inside explicit review, trace, capability, and write-scope boundaries.

The live codebase already has several Phase 8 foundations:

- projects carry typed primary repo and notes-root references in `vel-core`
- trace-linked `HandoffEnvelope` types already exist in `vel-core`
- `vel-protocol` already models handshake, heartbeat, capability negotiation, and action batches
- `vel-agent-sdk` and `veld/tests/agent_sdk.rs` already prove a basic protocol loop against `services::agent_protocol`
- connect-run persistence already exists in storage
- sandbox/broker mediation already exists, but the direct `/v1/connect` transport and direct guest-runtime closure are still missing

The correct slice order is:

1. publish typed execution-context, handoff, routing, and local-manifest contracts
2. persist project execution context and generate bounded repo-local GSD artifacts
3. activate authenticated connect transport and supervised local runtime lifecycle
4. surface explicit routing and handoff review across operator surfaces
5. add direct WASM guest execution behind the same mediated policy boundary
6. close the loop with GSD/operator docs, local-agent samples, and end-to-end verification

**Primary recommendation:** keep Phase 8 backend-first and contract-first. Let Apple continue independently. Use Phase 8 to make repo-aware coding execution supervised, inspectable, and task-bounded rather than broadening autonomy.

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| EXEC-01 | Projects carry repo/GSD execution context | Build on Phase 5 `ProjectRecord` roots and add typed execution context instead of ad hoc repo metadata |
| EXEC-02 | Vel can launch and supervise coding runtimes for bounded work | Activate `/v1/connect` and local-runtime lifecycle over explicit manifests, leases, and traces |
| GSD-01 | Vel generates repo-local artifacts that GSD can consume | Prefer sidecar repo-local docs/manifests over inventing a separate planner protocol |
| GSD-02 | GSD handoffs are explicit and reviewable | Reuse trace-linked handoff envelopes plus operator review surfaces |
| HANDOFF-01 | Human-to-agent handoff is explicit | Persist task objective, repo scope, write scope, and expected outputs before launch |
| HANDOFF-02 | Agent-to-agent handoff is explicit and inspectable | Extend the existing handoff contract into stored execution handoffs and review output |
| LOCAL-01 | Supervised local coding-agent support exists | Add manifest-backed local runtime launch with explicit writable roots/capability allowlists |
| POLICY-01 | Routing and execution remain safe and explainable | Encode token budget, agent profile, task type, write scope, and review gate as typed policy inputs |

## User Constraints

- Phase 08 should be planned so it can overlap with Phase 07 execution where possible.
- Phase 08 should begin from repo-local docs/context that GSD already knows how to consume.
- Do not widen Apple write scopes or create a hidden dependency on Apple quick-loop work.
- Vel remains a supervised execution coordinator, not an ambient code-modification authority.

## Parallelization Recommendation

Phase 8 still depends on Phase 7 at the roadmap level, but the first three Phase 8 slices can be executed in parallel with Phase 7 implementation because they stay in disjoint write scopes and depend only on already-shipped project, trace, protocol, and storage baselines:

- `08-01` contract publication: parallel-safe with all Phase 7 work
- `08-02` project execution context and repo-local artifact generation: parallel-safe with `07-02` through `07-04`
- `08-03` connect transport and supervised local runtime lifecycle: parallel-safe with `07-02` through `07-04`

The later Phase 8 slices should wait on those foundations, not on Apple-specific work.

## Standard Stack

### Core

| Library / Surface | Purpose | Why Standard |
|---------|---------|-------------|
| `vel-core` project + run contracts | typed execution context, handoff, policy vocabulary | keeps domain semantics out of transport/storage |
| `vel-protocol` | transport envelopes for runtime handshake/heartbeat/action submission | already present and tested |
| `veld::services::agent_protocol` | current protocol handling baseline | direct seam for `/v1/connect` activation |
| `veld::services::sandbox` + broker | capability mediation and trace-linked decisions | already deny-by-default and inspectable |
| `vel-agent-sdk` | reference limb/runtime client | existing baseline for Phase 8 closure |

### Supporting

| Library / Surface | Purpose | When to Use |
|---------|---------|-------------|
| `ProjectRecord.primary_repo` / notes roots | repo-aware execution anchor | execution context and artifact export |
| connect-run storage | lifecycle persistence | launch/list/terminate/expiry and operator inspection |
| CLI `connect` commands | operator shell for runtime lifecycle | wire stubs into active backend routes |
| existing `Now` / review surfaces | operator-visible execution status and pending review | handoff/routing visibility instead of a separate hidden queue |

## Recommended Plan Slices

### Slice 1: Publish execution contracts first

- Add typed project execution context, routing profile, budget class, execution handoff, and local-agent manifest contracts.
- Publish matching transport DTOs, schemas/examples, and owner docs.
- Keep write-scope and review-gate fields explicit in the contract itself.

### Slice 2: Persist project execution context and repo-local GSD artifacts

- Persist per-project execution context linked to Phase 5 projects.
- Add bounded export/generation of repo-local sidecar artifacts that GSD can read.
- Write only inside the project's primary repo root, never arbitrary filesystem paths.

### Slice 3: Activate connect transport and local runtime launch

- Replace the current `/v1/connect` deny-all reservation with authenticated launch/list/heartbeat/terminate routes.
- Support manifest-backed local runtimes with explicit capability allowlists, writable roots, and leases.
- Keep the first launch surface local-first and supervised, not remote-first.

### Slice 4: Make routing and handoffs operator-visible

- Persist execution handoffs and routing decisions.
- Surface them in CLI and existing operator review/`Now` surfaces.
- Use token budget, agent profile, task type, and writable scope as typed routing inputs, not hidden heuristics.

### Slice 5: Add direct WASM guest runtime follow-on

- Reuse the same protocol, policy, and broker boundary for direct guest execution.
- Keep host-executor fallback only as compatibility behavior.
- Ensure the guest runtime cannot widen permissions or bypass broker mediation.

### Slice 6: Close the loop with SDK, docs, and end-to-end samples

- Expand the reference SDK/sample path to use the shipped connect transport.
- Document repo-local GSD artifact flow, handoff review, local runtime launch, and guest-runtime limits.
- End with execution-backed smoke checks instead of doc-only closure.

## Existing Seams To Reuse

### Project substrate

- `crates/vel-core/src/project.rs`
- `crates/vel-storage/src/repositories/projects_repo.rs`
- Phase 5 project roots and family vocabulary

### Handoff and trace baseline

- `crates/vel-core/src/run.rs`
- `docs/cognitive-agent-architecture/agents/handoffs.md`
- Phase 3 trace-linked run inspection work

### Protocol and runtime baseline

- `crates/vel-protocol/src/lib.rs`
- `crates/vel-agent-sdk/src/`
- `crates/veld/src/services/agent_protocol.rs`
- `crates/veld/src/services/sandbox.rs`
- `crates/vel-storage/src/repositories/connect_runs_repo.rs`

### Operator shell seams

- `crates/vel-cli/src/commands/connect.rs`
- `crates/vel-cli/src/client.rs`
- existing review/`Now` and web operator surfaces from Phases 5 and 6

## Key Planning Rules

### Rule 1: GSD integration starts with files, not a new agent bus

The first shipped GSD integration should be repo-local artifacts and explicit handoff docs that existing GSD workflows can read. Do not design a brand-new planner transport before the repo-local path exists.

### Rule 2: Read scope can be broad, write scope must stay narrow

Phase 8 must preserve the self-awareness contract: runtimes may inspect more than they can modify. Every launched runtime needs explicit writable roots and reviewable write boundaries.

### Rule 3: Routing policy must be explainable

Token budget, profile, task type, and writable scope decisions must be inspectable from stored policy inputs and traces. No opaque "best model" routing.

### Rule 4: Direct `/v1/connect` exposure must remain auth-by-default

Phase 8 is the re-scoped transport closure from Phase 2/4, but public or ambient connect exposure would violate repo rules. Keep the new routes operator-authenticated and fail closed on undefined actions.

### Rule 5: WASM guest follow-on must reuse the existing broker/sandbox boundary

Do not create a second capability system for guest runtimes. The guest path should compose with the same brokered capability and trace rules already established.

## Common Risks

### Risk 1: Accidental ambient repo authority

If Phase 8 writes anywhere outside project-declared repo roots or skips review gates, it defeats the point of supervised execution.

### Risk 2: Rebuilding GSD inside Vel

Vel should prepare context, route work, and supervise execution. It should not fork GSD into a duplicate planning engine.

### Risk 3: Connect closure without operator reviewability

Direct `/v1/connect` transport without list/inspect/terminate and persisted routing/handoff context would leave launches opaque.

### Risk 4: Budget/profile policy encoded as vague heuristics

Routing decisions need typed classes and persisted reasons. Otherwise Phase 8 will be impossible to trust or debug.

### Risk 5: Guest runtime bypass

If the WASM guest path bypasses broker mediation or trace linkage, it will regress the zero-trust baseline rather than completing it.

## Verification Suggestions

### Automated

| Area | Command | Notes |
|------|---------|-------|
| Protocol invariants | `cargo test -p vel-protocol -- --nocapture` | Existing baseline should stay green |
| SDK/protocol flow | `cargo test -p veld agent_sdk -- --nocapture` | Existing reference flow to extend |
| Connect transport | `cargo test -p veld connect -- --nocapture` | Add in Phase 8 |
| CLI connect shell | `cargo test -p vel-cli connect -- --nocapture` | Wire stubs into active behavior |
| Sandbox/guest boundary | `cargo test -p veld sandbox -- --nocapture` | Extend for guest-runtime path |

### Manual

- Export repo-local execution artifacts for a project and inspect that writes stay inside the primary repo root.
- Launch a local coding runtime, observe heartbeat/list/inspect/terminate, and confirm denied write-scope expansion fails closed.
- Review a human-to-agent handoff from operator surfaces and verify the same persisted reasoning appears in CLI/API output.
- Run the reference SDK/sample against the activated connect transport and confirm stable trace/run linkage.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test`, targeted CLI smoke commands, optional targeted web tests for operator review surfaces |
| Quick run command | `cargo test -p vel-protocol -- --nocapture && cargo test -p veld agent_sdk -- --nocapture && cargo test -p vel-cli connect -- --nocapture` |
| Full suite command | `make verify && cargo test -p veld connect -- --nocapture && cargo test -p veld sandbox -- --nocapture` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| EXEC-01 | project execution context persists and exports bounded repo-local artifacts | integration/CLI | `cargo test -p veld execution_context -- --nocapture` | ❌ Wave 0 |
| EXEC-02 | launch/list/heartbeat/terminate flow works with supervised leases | integration | `cargo test -p veld connect -- --nocapture` | ❌ Wave 0 |
| GSD-01 | repo-local GSD artifact pack renders from project execution context | integration/CLI | `cargo test -p veld gsd_artifacts -- --nocapture` | ❌ Wave 0 |
| GSD-02 | handoff/routing review is explicit and inspectable | integration | `cargo test -p veld execution_routing -- --nocapture` | ❌ Wave 0 |
| HANDOFF-01 | human-to-agent handoff stores objective/scope/review state | integration | `cargo test -p veld handoff_review -- --nocapture` | ❌ Wave 0 |
| HANDOFF-02 | agent-to-agent handoffs remain trace-linked and inspectable | unit/integration | `cargo test -p vel-core handoff -- --nocapture && cargo test -p veld handoff_review -- --nocapture` | ✅ extend + ❌ Wave 0 |
| LOCAL-01 | supervised local coding-agent runtime launches with explicit manifests and writable roots | integration | `cargo test -p veld local_runtime -- --nocapture` | ❌ Wave 0 |
| POLICY-01 | routing, budget, and policy denials remain explainable and fail closed | integration | `cargo test -p veld execution_policy -- --nocapture` | ❌ Wave 0 |

### Wave 0 Gaps

- Add targeted execution-context storage/service tests
- Add connect transport HTTP integration tests
- Add routing/handoff review tests
- Add local-runtime manifest and writable-scope denial tests
- Add direct guest-runtime tests that prove no capability bypass

## Sources

### Primary

- `docs/MASTER_PLAN.md`
- `.planning/ROADMAP.md`
- `.planning/PROJECT.md`
- `.planning/STATE.md`
- `docs/templates/agent-implementation-protocol.md`
- `docs/tickets/phase-2/006-connect-launch-protocol.md`
- `docs/tickets/phase-4/010-wasm-agent-sandboxing.md`
- `docs/tickets/phase-4/014-swarm-execution-sdk.md`
- `docs/cognitive-agent-architecture/agents/handoffs.md`
- `docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md`
- `crates/vel-core/src/project.rs`
- `crates/vel-core/src/run.rs`
- `crates/vel-protocol/src/lib.rs`
- `crates/veld/src/services/agent_protocol.rs`
- `crates/veld/src/services/sandbox.rs`
- `crates/vel-storage/src/repositories/connect_runs_repo.rs`
- `crates/vel-cli/src/commands/connect.rs`
- `crates/veld/tests/agent_sdk.rs`

### Nearby Planning Context

- `.planning/phases/07-apple-action-loops-and-behavioral-signal-ingestion/07-RESEARCH.md`
- `.planning/phases/05-now-inbox-core-and-project-substrate/05-CONTEXT.md`
- `.planning/phases/06-high-value-write-back-integrations-and-lightweight-people-graph/06-RESEARCH.md`
