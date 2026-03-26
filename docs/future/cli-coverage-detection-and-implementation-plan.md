---
title: CLI Coverage Detection And Implementation Plan
doc_type: doc
status: draft
owner: contributors
created: 2026-03-25
updated: 2026-03-25
audience:
  - contributors
  - coding-agents
keywords:
  - vel-cli
  - coverage
  - test-automation
  - ci
  - verification
index_terms:
  - cli coverage plan
  - vel cli test coverage
  - automated coverage detection
  - command coverage rollout
related_files:
  - docs/MASTER_PLAN.md
  - docs/README.md
  - crates/vel-cli/Cargo.toml
  - crates/vel-cli/src/main.rs
  - crates/vel-cli/src/client.rs
  - scripts/ci-smoke.sh
  - .github/workflows/ci.yml
summary: This document records the current `vel-cli` coverage baseline and defines a concrete plan to add automated coverage detection plus phased implementation work for the largest CLI test gaps.
---

# CLI Coverage Detection And Implementation Plan

## Purpose

This document answers two contributor questions:

1. what `vel-cli` coverage exists today
2. how the repo should automate detection and close the highest-value CLI gaps without destabilizing CI

## Scope

This document covers:

- the current `vel-cli` test baseline
- the missing coverage detection automation
- a phased implementation plan for command, client, and smoke-path coverage
- suggested CI thresholds and ownership rules

This document does not change shipped behavior and does not itself add test automation code.

## Current Reality

`vel-cli` has meaningful test presence, but current coverage is strongest in parser and formatter paths rather than operator-visible execution behavior.

Observed baseline on 2026-03-25:

- `cargo test -p vel-cli` passes with `97` tests
- tests are all unit tests from the binary crate; there is no dedicated `crates/vel-cli/tests/` integration-test tree
- there is no repo-wired line or branch coverage report for `vel-cli`
- `crates/vel-cli/Cargo.toml` does not define coverage tooling or CLI integration-test helpers such as `assert_cmd` or `trycmd`
- the top-level automation paths in `Makefile` and `.github/workflows/ci.yml` run tests, but do not publish or enforce CLI coverage metrics

Current source inventory:

- `crates/vel-cli/src` contains `52` Rust files
- `23` files contain local tests
- `29` files contain no local tests
- under `crates/vel-cli/src/commands`, `16` files contain tests and `25` do not

Current strengths:

- argument parsing coverage in `crates/vel-cli/src/main.rs`
- command language coverage in `crates/vel-cli/src/command_lang/parse.rs`, `infer.rs`, `completion.rs`, `preview.rs`, and `registry.rs`
- formatter and rendering coverage in selected command modules such as `exec.rs`, `runs.rs`, `review.rs`, `docs.rs`, `connect.rs`, and `backup.rs`

Current weak points:

- `crates/vel-cli/src/client.rs` is large and untested
- many operator-facing commands have no tests
- the repo smoke path validates only a narrow set of CLI commands
- there is no automated detection that flags new untested command surfaces or falling coverage trends

Representative untested CLI command files as of this baseline:

- `commands/capture.rs`
- `commands/commitments.rs`
- `commands/doctor.rs`
- `commands/evaluate.rs`
- `commands/export_.rs`
- `commands/health.rs`
- `commands/integrations.rs`
- `commands/journal.rs`
- `commands/loops.rs`
- `commands/nudges.rs`
- `commands/people.rs`
- `commands/recent.rs`
- `commands/risk.rs`
- `commands/signals.rs`
- `commands/suggestions.rs`
- `commands/sync.rs`
- `commands/synthesize.rs`
- `commands/threads.rs`
- `commands/uncertainty.rs`

## Goal

The target state is:

- CI publishes a deterministic `vel-cli` coverage report
- CI fails when coverage meaningfully regresses below the accepted floor
- command modules with networked or user-visible behavior have integration coverage, not only formatter tests
- `vel-cli` coverage status is easy to inspect locally and in CI
- new CLI commands cannot land without at least baseline parser plus execution-path verification

## Coverage Model

`vel-cli` needs three complementary coverage layers.

### 1. Parser And Shape Coverage

Purpose:

- verify `clap` command trees, flags, and argument constraints

Current state:

- already reasonably strong in `crates/vel-cli/src/main.rs`

Success condition:

- every top-level command and high-risk subcommand has parser coverage

### 2. Behavior And Rendering Coverage

Purpose:

- verify command handlers, output rendering, error text, normalization, and serialization

Current state:

- partial and uneven

Success condition:

- each command module has focused unit tests for formatting logic and validation helpers

### 3. End-To-End CLI Coverage

Purpose:

- verify the compiled `vel` binary against realistic API responses and local runtime behavior

Current state:

- narrow smoke coverage only via `scripts/ci-smoke.sh`

Success condition:

- integration tests exercise representative happy-path and failure-path command execution
- smoke tests remain thin and fast, while richer command behavior moves into dedicated integration tests

## Proposed Automated Detection

The repo should add two forms of automated detection.

### A. Coverage Metric Detection

Add a dedicated `vel-cli` coverage command based on `cargo llvm-cov`.

Recommended local command:

```bash
cargo llvm-cov -p vel-cli --summary-only --json
```

Recommended CI artifact outputs:

- text summary for logs
- JSON summary for threshold checks
- LCOV report for future upload or local inspection

Why `cargo llvm-cov`:

- it is the standard Rust coverage path built on LLVM instrumentation
- it supports crate-scoped reporting
- it gives both human-readable and machine-readable output
- it avoids inventing a home-grown line coverage counter

If Nix or CI image setup makes `cargo llvm-cov` expensive initially, phase it in behind a separate make target first, then promote it into required CI once stable.

### B. Structural Gap Detection

Add a lightweight repo script that inspects `crates/vel-cli/src/commands/*.rs` and reports:

- command files with no local tests
- command files missing matching integration-test references
- newly added command modules since the baseline

This script should not pretend to be line coverage. Its purpose is to fail fast when new CLI surfaces are added with zero verification scaffolding.

Recommended checks:

- every file in `crates/vel-cli/src/commands/*.rs` must satisfy at least one of:
  - contains a `#[cfg(test)]` module
  - has a matching integration test in `crates/vel-cli/tests/`
  - is explicitly listed in an allowlisted debt file with an owner and exit date
- `crates/vel-cli/src/client.rs` must have at least one dedicated test module or integration suite coverage marker

## Proposed Repo Changes

The implementation should add the following surfaces.

### Makefile Targets

Add:

- `test-cli`: run `cargo test -p vel-cli`
- `coverage-cli`: run the crate-scoped coverage pipeline
- `coverage-cli-check`: enforce minimum thresholds and structural rules

Suggested progression:

- keep `make verify` unchanged until coverage runtime is stable
- add `coverage-cli-check` to `make ci` only after the baseline is reliable in CI

### CI Workflow

Extend `.github/workflows/ci.yml` to:

- run `cargo test -p vel-cli`
- run the new coverage target
- upload the coverage summary as an artifact
- fail on threshold regression once the threshold is ratified

### CLI Integration Test Harness

Add `crates/vel-cli/tests/` with:

- `assert_cmd` for invoking the binary
- fixture-backed HTTP stubs for deterministic API responses
- snapshot-light assertions focused on stable, high-signal fields

Recommended harness shape:

- spawn a tiny mock HTTP server per test
- point `vel` at the mock server through `--base-url` or `VEL_BASE_URL`
- assert stdout, stderr, exit status, and JSON-mode output

### Coverage Debt Manifest

Add a checked-in machine-readable file for temporary exceptions, for example:

`config/coverage/vel-cli-coverage-debt.json`

Each entry should include:

- file path
- gap type
- owner
- rationale
- created date
- target removal date

This keeps deliberate test debt explicit and reviewable instead of hidden in tribal knowledge.

## Implementation Plan

Work should land in narrow slices.

### Phase 1. Establish Automated Detection

Deliverables:

- add `cargo llvm-cov` support for `vel-cli`
- add `make coverage-cli`
- add a script that reports untested CLI command modules
- publish a baseline JSON or text artifact in CI without failing builds yet

Verification:

- local run of `make coverage-cli`
- CI artifact contains crate summary and structural gap report

Exit criteria:

- contributors can answer "what is `vel-cli` coverage today?" from CI output alone

### Phase 2. Add Integration Harness

Deliverables:

- add `crates/vel-cli/tests/`
- add minimal dependencies such as `assert_cmd`, `predicates`, and a mock HTTP server crate
- cover a representative set of read commands and one mutation command

Initial target commands:

- `vel health`
- `vel config show`
- `vel recent --json`
- `vel search --json`
- `vel capture`

Verification:

- `cargo test -p vel-cli`
- coverage summary shows execution beyond parser-only paths

Exit criteria:

- at least one command from each major pattern is covered:
  - plain read
  - JSON read
  - mutation/write
  - failure propagation

### Phase 3. Cover Client Boundary And High-Value Commands

Deliverables:

- add tests around `crates/vel-cli/src/client.rs`
- cover request construction, query parameter handling, error mapping, and JSON decoding
- add command tests for high-frequency operator surfaces

Priority order:

1. `health.rs`
2. `capture.rs`
3. `recent.rs`
4. `journal.rs`
5. `commitments.rs`
6. `nudges.rs`
7. `sync.rs`
8. `doctor.rs`

Verification:

- targeted `cargo test -p vel-cli <filter>` runs for each slice
- coverage summary improves across both `client.rs` and command modules

Exit criteria:

- no high-frequency daily-use command remains completely untested

### Phase 4. Raise The Floor And Enforce Regression Gates

Deliverables:

- define minimum line coverage threshold for `vel-cli`
- define minimum changed-lines coverage threshold if supported by CI tooling
- fail CI on threshold regressions
- require new command modules to ship with parser plus behavior coverage

Recommended initial enforcement policy:

- no regression below the ratified baseline once Phase 3 lands
- no new `commands/*.rs` file without local or integration coverage
- no new networked command without an end-to-end invocation test

Verification:

- intentionally lower coverage in a trial branch and confirm CI fails

Exit criteria:

- coverage detection is policy, not just a report

## Command Coverage Backlog

The CLI command backlog should be prioritized by operator frequency and risk rather than alphabetically.

### Tier 1: Daily Operator Loop

- `capture`
- `recent`
- `search`
- `today`
- `morning`
- `journal`
- `commitments`
- `nudges`
- `health`
- `doctor`

### Tier 2: Runtime And Recovery Trust

- `runs`
- `run inspect`
- `inspect`
- `artifact`
- `backup`
- `config`
- `policy`
- `uncertainty`

### Tier 3: Sync And Integration Boundaries

- `sync`
- `integrations`
- `signals`
- `evaluate`
- `people`

### Tier 4: Supervised Execution And Planning Surfaces

- `exec`
- `connect`
- `review`
- `synthesize`
- `suggestions`
- `threads`
- `loops`
- `export_`

## Threshold Proposal

The repo should avoid arbitrary large thresholds before execution coverage exists. Use staged thresholds.

Suggested rollout:

- Stage A: report only, no enforced percentage
- Stage B: enforce "no regression below recorded baseline"
- Stage C: enforce crate line coverage floor
- Stage D: enforce per-path expectations for `client.rs` and `commands/*.rs`

Suggested first ratified thresholds after Phase 3:

- overall `vel-cli` line coverage: ratchet from measured baseline upward
- changed-lines coverage for `crates/vel-cli/src/**`: `>= 80%`
- structural rule: `0` newly added untested command files

The exact line percentage should be measured from the first stable `llvm-cov` run instead of guessed in advance.

## Cross-Cutting Notes

### Modularity

Coverage should follow the actual CLI seams:

- parser
- command handler
- client transport
- smoke path

Do not hide weak transport coverage behind parser-only test counts.

### Accessibility

For CLI work, accessibility mainly means readable, stable, operator-usable output.

Tests should assert:

- JSON mode stays machine-readable
- human output stays label-driven and understandable
- failure messages explain next action when appropriate

### Configurability

Coverage must include:

- `--base-url`
- environment-based configuration
- JSON vs human output modes
- failure behavior when daemon endpoints are unavailable

### Logging And Inspection

Coverage reports should be saved as CI artifacts and easy to inspect after failures.

### Replay And Recovery

Tests should prefer deterministic fixtures and mock servers so failures are reproducible.

### Composability

The harness should be reusable across command families instead of hand-rolling a new mock setup per command.

## Recommended First Execution Slice

The smallest high-value first slice is:

1. add `coverage-cli` reporting with `cargo llvm-cov`
2. add a structural untested-command detector
3. add integration tests for `health`, `recent --json`, and `capture`

That slice creates:

- measurable baseline visibility
- one reusable invocation harness
- one read command
- one JSON command
- one mutation command

## Related Terms

- canonical name: CLI coverage detection
- aliases: vel-cli coverage, CLI test gap plan, command coverage rollout
- nearby subsystems: CLI parser, API client, smoke tests, CI verification

## Related Documents

- [MASTER_PLAN.md](../MASTER_PLAN.md)
- [README.md](../README.md)
- [agent-implementation-protocol.md](../templates/agent-implementation-protocol.md)

## Search Terms

- vel-cli coverage
- cli coverage plan
- automated coverage detection
- cargo llvm-cov vel-cli
- CLI integration test rollout
- command coverage debt
