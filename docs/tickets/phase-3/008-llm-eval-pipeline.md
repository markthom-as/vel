---
title: LLM-as-a-Judge Evaluation Pipeline (Evals)
status: planned
owner: staff-eng
type: verification
priority: medium
created: 2026-03-17
labels:
  - veld
  - llm-eval
  - reasoning
---

Implement a "crates/veld-evals" tool that uses high-tier LLM models as "Judges" to verify the reasoning accuracy of local synthesis and inference outputs.

## Technical Details
- **Eval Library**: Build a CLI tool in `crates/veld-evals`.
- **Golden Dataset**: Define a set of input/output scenarios (Signal input -> Expected Synthesis).
- **Comparison Logic**: Use a strong remote model (e.g., Claude 3.5 Sonnet) to grade the accuracy of a local model's output based on a predefined "Vel Constitution" of reasoning principles.
- **Execution Pairing**: Pair LLM grading with deterministic fixtures, replay checks, and execution-backed assertions so the judge is not the only gate.
- **Reporting**: Output a structured JSON report with metrics like `ReasoningAccuracy`, `HallucinationRate`, `NudgeConsistency`, and trace completeness.

## Acceptance Criteria
- Nightly eval runs can measure the reasoning quality of the latest code.
- A "Reasoning Score" is reported after every major change to LLM prompts.
- CI/CD can fail if reasoning accuracy drops below a configurable threshold.
- Eval reports clearly distinguish model judgment from deterministic or execution-backed failures.
