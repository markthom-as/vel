# Phase 3 Parallel Execution Board

This board defines Phase 3 parallel execution ownership with non-overlapping primary write scopes and a three-sub-phase rollout.

This board is execution guidance, not shipped-behavior authority.
Shipped behavior remains anchored in `docs/MASTER_PLAN.md`.

## Sub-Phase 1: Tracing + Documentation Closure Baseline

Goal: convert existing partial run inspectability and user docs into explicit, aligned closure work.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Trace Contract | `017` | `crates/vel-core/src/`, `crates/vel-storage/src/`, `docs/cognitive-agent-architecture/agents/` | active | Standardize trace and handoff envelope contracts. |
| B: Inspect Surfaces | `017` | `crates/veld/src/routes/`, `crates/vel-cli/src/commands/`, `clients/web/src/` | active | Expose trace-linked inspection paths across operator surfaces. |
| C: User Doc Architecture | `013` | `docs/user/`, `docs/api/` | active | Close support architecture/version parity and recovery guidance gaps. |

Sub-phase 1 merge gate:

- trace/handoff contract language is explicit and consistent across code/docs
- operator surfaces can inspect run-linked workflow evidence beyond raw logs
- user docs match shipped operator terminology and recovery paths

## Sub-Phase 2: Deterministic Simulation Harness

Goal: ship reproducible day-simulation and replay assertions as a hard verification layer.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Clock & Ordering Seams | `007` | `crates/vel-core/src/`, `crates/veld/src/services/` | queued | Add deterministic time/order seams for replay-sensitive logic. |
| B: Simulation Runner | `007` | `crates/vel-sim/` (new) or equivalent module | queued | Build scenario runner and fixture loader. |
| C: Replay Assertions | `007`, `017` | `crates/veld/tests/`, `crates/vel-storage/src/repositories/` | queued | Assert terminal state + boundary event completeness. |

Sub-phase 2 merge gate:

- day-simulation scenarios execute deterministically
- replay checks validate both state and event boundaries
- simulation runtime is practical for regular automation use

## Sub-Phase 3: Eval Pipeline & Quality Gates

Goal: add LLM-judge evals as a paired layer on top of deterministic verification.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Eval Runner | `008` | `crates/veld-evals/` (new) | queued | Add fixture-driven eval CLI and report schema. |
| B: Judge Integration | `008` | `crates/vel-llm/`, eval config surfaces | queued | Provider-configurable judge rubric and scoring. |
| C: CI/Reporting | `008`, `013` | `.github/` workflows, `docs/` eval operation docs | queued | Enforce thresholds and publish readable outcomes. |

Sub-phase 3 merge gate:

- eval reports separate deterministic failures from model-judge outcomes
- threshold regressions can fail automated gates
- eval operation guidance is documented and reproducible

## Dependency Order

1. Sub-phase 1 closes trace/doc alignment gaps and stabilizes inspection semantics.
2. Sub-phase 2 builds deterministic replay on top of that baseline.
3. Sub-phase 3 layers judge-based quality checks on deterministic foundations.

## Coordination Rules

- Do not overlap primary write scopes in the same patch unless planned as a cross-lane integration slice.
- Deterministic checks are hard-gates; model-judge checks are additive quality gates.
- If trace schema changes, update storage, API types, and operator docs in the same slice.
- Every lane reports command-backed verification evidence.

## Suggested Verification Commands

- `cargo test -p veld`
- `cargo test -p vel-storage`
- `cargo test -p vel-cli`
- `node scripts/verify-repo-truth.mjs`

## First PR Batches

### Lane A: Trace Contract (`017`)

1. Add `trace_id` and parent linkage fields to core trace/handoff types with serialization tests.
2. Add storage schema/repository support for trace-linked handoff records.
3. Publish trace/handoff contract updates in runtime and architecture docs.

### Lane B: Inspect Surfaces (`017`)

1. Extend `/v1/runs/:id` detail response with trace linkage fields.
2. Add `vel run inspect --trace` output path and compatibility tests.
3. Add web trace panel entrypoint for run-linked workflows.

### Lane C: User Doc Architecture (`013`)

1. Add support ownership/update model in `docs/user/README.md` and `docs/user/reality-and-maturity.md`.
2. Add troubleshooting decision trees for stale context, sync, and auth failures.
3. Add doc parity checklist for CLI/web/Apple terminology and recovery guidance.
