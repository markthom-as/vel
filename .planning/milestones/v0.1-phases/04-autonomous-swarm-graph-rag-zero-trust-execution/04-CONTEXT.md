---
phase: 04-autonomous-swarm-graph-rag-zero-trust-execution
created: 2026-03-18
status: active
source_tickets:
  - docs/tickets/phase-4/009-semantic-memory-rag.md
  - docs/tickets/phase-4/010-wasm-agent-sandboxing.md
  - docs/tickets/phase-4/014-swarm-execution-sdk.md
---

# Phase 4 Context

Phase 4 starts with no on-disk planning artifacts. The governing board for this phase is `docs/tickets/phase-4/parallel-execution-board.md`, which explicitly sequences work as:

1. contract and safety foundations
2. core runtime implementations
3. SDK delivery and end-to-end closure

## Key Constraints

- Contracts, schemas, templates, and examples must land before broad sandbox/protocol/index runtime work.
- Sandbox and SDK work must not bypass the capability broker boundary.
- External protocol and sandbox boundaries must remain trace-linked and deny-by-default.
- Semantic memory work must preserve provenance and local-first operation.

## Initial Plan Shape

- `04-01`: contract and schema foundation for semantic memory, sandbox ABI, and swarm protocol
- `04-02`: semantic index backend seam and retrieval lifecycle
- `04-03`: WASM sandbox runtime and operator-visible denial/outcome surfaces
- `04-04`: protocol crate and fixture-backed serialization/validation
- `04-05`: reference SDK and end-to-end scoped action flow
