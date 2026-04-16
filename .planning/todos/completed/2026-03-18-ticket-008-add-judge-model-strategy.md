---
created: 2026-03-18T07:25:40.260Z
title: Ticket 008 - add judge model strategy decision
area: docs
files:
  - docs/tickets/phase-3/008-llm-eval-pipeline.md
  - crates/vel-llm/src/
---

## Problem

Ticket 008 (LLM-as-a-Judge Eval Pipeline) is abstract about the judge model. The "provider-configurable judge" framing leaves open whether CI uses local or remote inference. This matters for:
- Eval determinism (local models are more stable across runs; remote models have API versioning)
- CI feasibility (remote judge adds latency, cost, and network dependencies)
- The judge rubric format (local llama.cpp inference vs. OpenAI-compatible chat completions differ in prompt format)

## Solution

Add a decision record to ticket 008: **Primary judge mode is local via `vel-llm` router** (consistent with local-first philosophy, no cloud dependencies in CI). Remote provider is a configuration override. The eval runner should abstract the judge call through `vel-llm`'s existing `Router` interface so both modes work transparently.
