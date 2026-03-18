---
phase: 03-deterministic-verification-continuous-alignment
plan: 05
subsystem: evals, llm, docs, ci
tags: [phase-3, evals, llm-judge, verification, ci]

requires:
  - phase: 03-deterministic-verification-continuous-alignment
    plan: 04
    provides: deterministic replay harness and fixed-time replay seams

provides:
  - New `veld-evals` CLI crate with fixture loading, structured reports, and exit-code policy
  - Versioned eval fixture/report schema and checked-in sample fixture
  - Optional router-backed judge mode using explicit `judge` model routing or per-fixture profile override
  - CI/documentation hooks for reproducible eval execution

affects:
  - Phase 4 planning can now treat deterministic replay + eval gating as existing verification infrastructure

requirements-completed:
  - EVAL-01
  - EVAL-02

duration: 23min
completed: 2026-03-18
---

# Phase 3 Plan 05: Eval Runner and Quality Gates Summary

Completed the remaining Phase 3 eval slice by adding a standalone `veld-evals` runner that consumes versioned fixtures, reuses `vel-sim` as the deterministic hard gate, and optionally applies a router-backed LLM judge rubric with explicit threshold policy.

## Accomplishments

- Added the new eval crate in [`crates/veld-evals/src/lib.rs`](/home/jove/code/vel/crates/veld-evals/src/lib.rs) and [`crates/veld-evals/src/main.rs`](/home/jove/code/vel/crates/veld-evals/src/main.rs)
- Added a checked-in sample fixture at [`crates/veld-evals/fixtures/sample-day-context.json`](/home/jove/code/vel/crates/veld-evals/fixtures/sample-day-context.json)
- Extended model routing docs/templates with an explicit `judge` task class in [`configs/models/routing.toml`](/home/jove/code/vel/configs/models/routing.toml), [`configs/models/templates/routing.template.toml`](/home/jove/code/vel/configs/models/templates/routing.template.toml), and [`configs/models/README.md`](/home/jove/code/vel/configs/models/README.md)
- Updated deterministic fixture ergonomics in [`crates/vel-sim/src/lib.rs`](/home/jove/code/vel/crates/vel-sim/src/lib.rs) so eval fixtures can store human-readable RFC3339 timestamps
- Added operator guidance in [`docs/user/evals.md`](/home/jove/code/vel/docs/user/evals.md) and linked it from [`docs/user/README.md`](/home/jove/code/vel/docs/user/README.md)
- Added a CI eval smoke step in [`.github/workflows/ci.yml`](/home/jove/code/vel/.github/workflows/ci.yml)

## Verification

- `cargo test -p vel-sim -- --nocapture`
- `cargo test -p veld-evals -- --nocapture`
- `cargo run -p veld-evals -- run --fixtures crates/veld-evals/fixtures/sample-day-context.json --report /tmp/vel-eval-report.json`
- `cargo test -p vel-config load_repo_model_profiles_and_routing -- --nocapture`
- `node scripts/verify-repo-truth.mjs`

## Notes

- Deterministic replay remains the hard correctness gate; judge scoring is additive and can be promoted to a failing CI condition via CLI flag or fixture policy.
- The sample fixture ships with judge mode disabled so repo smoke runs remain local and reproducible without requiring an LLM backend.
