# Phase 3: Deterministic Verification & Continuous Alignment - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 3 closes the trust gap between "the runtime works" and "the operator can prove it worked correctly." The phase delivers three connected capabilities: (1) trace-linked inspection for runs and delegated workflows, (2) deterministic replay/simulation for regression verification, and (3) paired eval infrastructure that combines hard deterministic checks with judge-model scoring. It also closes the remaining operator-facing documentation architecture gaps so shipped behavior stays reviewable and supportable.

Authoritative specifications are in `docs/tickets/phase-3/` — tickets 007, 008, 017, 013. The parallel execution board (`docs/tickets/phase-3/parallel-execution-board.md`) defines the required sequencing: SP1 trace/doc closure → SP2 deterministic simulation → SP3 eval pipeline.

</domain>

<decisions>
## Implementation Decisions

### Execution Strategy
- Start with ticket 017 and the Phase 3 SP1 merge gate before touching simulation or judge-eval work
- Split SP1 into separate executable slices for trace contracts, inspect surfaces, and documentation parity so each patch has a clean write scope
- Treat deterministic replay as a hard-gate foundation for evals; ticket 008 must build on ticket 007 instead of bypassing it
- Preserve route/service/storage layering while adding trace and replay seams; no HTTP DTO logic in services and no transport concerns in storage

### Trace And Reviewability
- Introduce explicit trace linkage in shared types and operator DTOs first, even if initial storage persistence is metadata-backed rather than a full new table
- Reuse existing run IDs, run events, and operator run inspection surfaces as the starting point; do not invent a parallel observability system
- Handoff metadata must stay structured and explainable from persisted inputs, event payloads, or documented fallback rules

### Deterministic Verification
- Replay-sensitive code must move behind injectable time/order seams rather than relying on wall-clock calls in the simulation path
- The day-simulation harness should reuse runtime service seams where possible instead of duplicating orchestration logic
- Deterministic checks are hard failures; model-judge output is additive quality scoring only

### Documentation And Support
- `docs/user/` and `docs/api/` remain the canonical operator-facing documentation surfaces; do not create a parallel "wiki" tree
- User docs must continue to distinguish shipped behavior from planned behavior, using `docs/MASTER_PLAN.md` as the top-level truth anchor

### Claude's Discretion
- Exact plan granularity inside SP1/SP2/SP3 as long as ticket scope and dependency order remain intact
- Specific DTO field names and fallback behavior for trace metadata where existing persisted records do not yet contain explicit trace IDs
- Test helper structure and fixture layout for simulation/eval crates, provided they remain deterministic and reviewable

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/vel-core/src/run.rs` already defines run IDs, run events, agent spawn inputs, and retry policy semantics
- `crates/vel-storage/src/repositories/runs_repo.rs` and `run_refs_repo.rs` provide the current run/run-event persistence seam
- `crates/veld/src/routes/runs.rs` already maps run detail/list responses and has test coverage for run inspection and retry metadata
- `crates/vel-cli/src/commands/runs.rs` already exposes `vel runs` and `vel run inspect`
- `docs/api/runtime.md` and `docs/cognitive-agent-architecture/agents/handoffs.md` already document the runtime/run-inspection and handoff concepts, but they are not yet aligned to an explicit trace contract
- Existing user docs already cover setup, daily use, troubleshooting, and recovery flows; Phase 3 docs are closure work, not greenfield docs

### Established Patterns
- Thin route handlers map storage/service results to DTOs and propagate `AppError`
- Run metadata that is not yet first-class schema sometimes lives in structured JSON payloads and is surfaced through operator DTOs
- Focused API and runtime behavior tests live in `crates/veld/src/app.rs` and targeted integration tests under `crates/veld/tests/`
- Docs are updated in the same slice when operator-visible contracts or terminology change

### Integration Points
- Trace linkage begins in `vel-core` domain types, then flows through `vel-api-types`, `veld` run routes, CLI inspection, and web/dashboard surfaces
- Deterministic replay will need seams in `veld` services and likely a new `crates/vel-sim/` crate or equivalent module
- Judge-eval work will need a new `crates/veld-evals/` crate and configuration hooks in `vel-llm`

</code_context>

<specifics>
## Specific Ideas

- Use the active Phase 3 board literally: SP1 trace/doc closure first, then deterministic simulation, then eval pipeline
- Keep the first implementation slice small: shared trace contract + API/detail exposure + contract docs + tests
- Preserve compatibility for existing runs by deriving trace metadata from persisted input/output/event payloads until deeper storage work lands

</specifics>

<deferred>
## Deferred Ideas

- Full web trace explorer UI beyond the first operator-inspection entrypoint
- CI threshold enforcement and judge-model automation until deterministic replay is in place
- Rich graph-style multi-agent trace visualization; Phase 3 only needs reviewable operator inspection, not a complex tracing console

</deferred>
