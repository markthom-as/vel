# Phase 27 Validation

## Goal

Make scheduler and commitment rule semantics canonical, SQL-backed, and reusable across Vel without inventing a second planner or keeping provider labels as hidden product truth.

## Required Truths

- canonical scheduler facets exist as Vel-owned semantics rather than raw provider labels
- the preserved rule system intentionally covers `block:*`, `cal:free`, duration, time windows, urgent/defer, and anchored due datetime behavior
- normalized scheduler semantics are persisted and inspectable through backend/domain seams
- reflow, recall, and assistant grounding consume the normalized semantics instead of reparsing raw labels as their primary source of truth
- shells stay presentation surfaces and do not become owners of scheduler interpretation

## Plan Shape

Phase 27 should be executed in four slices:

1. canonical scheduler facet schema and ingest mapping
2. durable persistence and backend exposure of normalized facets
3. consumption by reflow, recall, and grounded assistant context
4. docs/examples/verification closure

## Block Conditions

Block if any slice:

- treats raw upstream/provider tags as the durable product model
- invents a separate agent-only or shell-only scheduler semantics layer
- claims broad autonomous or multi-day planning behavior that is not implemented
- stores only opaque JSON blobs with no durable typed scheduler seam
- weakens explainability compared to the current `codex-workspace` rule system

## Exit Condition

Phase 27 is complete when the product can honestly say:

- Vel persists canonical scheduler semantics for commitments
- agent and recall lanes can query those semantics directly
- reflow no longer depends primarily on ad hoc raw-label parsing
- docs and examples teach the canonical rule model and its current limits clearly
