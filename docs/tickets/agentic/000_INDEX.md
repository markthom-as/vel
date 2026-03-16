# vel — agentic engineering tickets (current-state regeneration)

These tickets are regenerated from the **actual repository snapshot** in `vel-main (13).zip`, not from a hypothetical architecture.

## What is already present in the repo

- Rust workspace with `vel-core`, `vel-storage`, `vel-config`, `vel-api-types`, `veld`, `vel-cli`
- Apple bootstrap under `clients/apple/`
- Implemented routes/services for captures, commitments, risk, suggestions, runs, context, explain, sync, synthesis
- Inline tests in `crates/veld/src/app.rs`, storage tests in `crates/vel-storage/src/db.rs`, CLI/config tests
- Canonical day fixture coverage already living inside `crates/veld/src/app.rs`
- Status/spec docs already extensive under `docs/` and `docs/specs/`

## Why these tickets exist

The codebase now has real architectural shape. The next job is to fold **agentic engineering patterns** into the repo and the dev loop without re-breaking subsystem boundaries.

These tickets are intentionally biased toward:

1. reuse before reinvention
2. tests before mutation
3. small, reviewable slices
4. explainable behavior
5. turning solved work into reusable capability

## Ticket order

1. `001_agentic_bootstrap_first_run_the_tests.md`
2. `002_knowledge_hoard_library.md`
3. `003_linear_walkthroughs_and_architecture_snapshot.md`
4. `004_promote_canonical_examples_from_existing_tests.md`
5. `005_prompt_library_and_agent_runbooks.md`
6. `006_agentic_ci_guardrails.md`
7. `007_refactor_inference_into_context_reducer_helpers.md`
8. `008_make_next_event_and_next_commitment_selection_explicit.md`
9. `009_harden_suggestion_loop_with_evidence_and_policy.md`
10. `010_record_example_and_skill_extraction_cli.md`

## Shared repo facts these tickets assume

- `AGENTS.md` already defines repo rules and reading order.
- `docs/status.md` is the implementation truth source.
- `crates/veld/src/app.rs` already contains many behavioral tests and the canonical day fixture.
- `services/inference.rs`, `services/risk.rs`, `services/nudge_engine.rs`, and `services/suggestions.rs` are the main “thinking loop” files.
- There is no obvious `scripts/`, `justfile`, `Makefile`, or `.github/workflows/` yet.

## Definition of done for this ticket set

The repo should emerge with:

- a standard “first run the tests” entrypoint
- a reusable knowledge/examples library
- codebase walkthrough artifacts for agents
- prompt/runbook assets under version control
- CI-quality guardrails for agent changes
- less monolithic inference logic
- explicit urgency ordering instead of accidental ordering
- a suggestion loop with stronger evidence contracts
- a way to store solved tasks as reusable examples/skills
