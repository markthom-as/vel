# Eval Runner

`veld-evals` is the shipped fixture-driven verification runner for deterministic replay plus optional LLM-judge scoring.

Use it when you want to verify that a known scenario still produces the expected replay output, and optionally score the resulting context with a configured judge model.

`vel evaluate` remains the live recompute-and-persist command against a running `veld` API. It now points operators toward this fixture replay path, but it does not embed `veld-evals`, depend on it, or change runtime behavior.

## What it does

- replays versioned eval fixtures against the deterministic `vel-sim` harness,
- hard-fails on deterministic mismatches such as missing boundary events or unexpected output shape,
- optionally asks a judge model to score output quality using an explicit rubric and threshold,
- writes a machine-readable JSON report that separates deterministic failures from judge outcomes.

## Basic run

```bash
cargo run -p veld-evals -- run \
  --fixtures crates/veld-evals/fixtures/sample-day-context.json \
  --report /tmp/vel-eval-report.json
```

The sample fixture uses deterministic checks only. The command prints the report JSON and also writes it to the `--report` path.

Use this command after a code change when you need reproducible proof that the checked-in fixture still replays through `vel-sim` the same way. Use `vel evaluate` when you want the daemon to recompute current live operator state.

## Judge mode

Judge mode is configured per fixture.

- Set `"mode": "router"` in the fixture `judge` block.
- Either set `judge.model_profile` directly, or configure `judge = "profile-id"` in [`configs/models/routing.toml`](/home/jove/code/vel/configs/models/routing.toml).
- Keep the rubric explicit in the fixture so score changes are reviewable.

To make judge regressions fail CI or shell scripts, either:

- pass `--fail-on-judge-regression`, or
- set `"fail_on_regression": true` in the fixture judge block.

## Reading the report

- `summary.deterministic_failure_count` tells you whether the hard replay gate failed.
- `summary.judge_failure_count` tracks scenarios whose judge score fell below threshold or could not run.
- Each scenario report includes `deterministic.failures` and a separate `judge` section so you can tell execution regressions from quality-score regressions.

## Operator rules

- Treat deterministic failures as hard correctness regressions first.
- Treat judge scores as additive quality signals, not replacements for deterministic checks.
- Update the fixture and rubric in the same change when the intended behavior changes.
