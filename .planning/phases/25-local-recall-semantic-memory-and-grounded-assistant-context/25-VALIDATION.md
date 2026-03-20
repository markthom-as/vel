# Phase 25 Validation

## Goal

Improve local recall quality and assistant grounding without creating a parallel memory system, weakening explainability, or widening into unrelated provider work.

## Required Truths

- recall and grounding reuse the canonical semantic-memory and agent-grounding contracts already published in the repo
- local retrieval quality improves across captures, notes, projects, people, threads, and transcripts
- assistant context assembly becomes more backend-owned and less dependent on repeated raw tool calls
- retrieved context remains inspectable from ranking inputs, provenance, and durable source references

## Plan Shape

Phase 25 should be executed in four slices:

1. recall/grounding contract tightening
2. semantic retrieval quality and provenance improvements
3. backend-owned assistant context assembly over the improved recall seam
4. shell/docs verification closure

## Block Conditions

Block if any slice:

- invents an assistant-only memory representation outside the canonical retrieval/grounding seams
- claims semantic or recall behavior that cannot be explained from persisted sources, scores, or provenance
- widens into unrelated provider expansion, hosted memory infrastructure, or a new architecture phase
- pushes recall assembly into shell-local heuristics instead of backend-owned contracts

## Exit Condition

Phase 25 is complete when the product can honestly say:

- Vel retrieves better local context for recall questions over the entities it already owns
- the assistant gets a stronger bounded recall/context pack from backend services
- operators can inspect why a memory hit or recall answer was produced
