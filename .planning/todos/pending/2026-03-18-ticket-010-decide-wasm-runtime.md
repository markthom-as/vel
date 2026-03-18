---
created: 2026-03-18T07:25:40.260Z
title: Ticket 010 - decide WASM runtime before Phase 4 SP1
area: docs
files:
  - docs/tickets/phase-4/010-wasm-agent-sandboxing.md
  - docs/tickets/phase-4/parallel-execution-board.md
---

## Problem

Ticket 010 (Zero-Trust WASM Agent Sandboxing) says "embed WASM runtime" without specifying which one. This decision must be made before Phase 4 Sub-Phase 1 contract work begins because:
- The Host ABI contract design depends on whether you use Component Model WIT or a custom ABI
- The SDK (ticket 014) guest-side interface is shaped by the host runtime's ABI model
- You can't design a typed host ABI without knowing which module format guests target

Leaving this implicit will cause the SP1 contract work to over-abstract and the SP2 implementation to re-design.

## Solution

**Recommended: wasmtime + Component Model.**
- Component Model (WIT) gives typed imports/exports — capability boundaries are compile-time verified, not runtime string checks
- Deny-by-default is the Component Model's default: modules only see explicitly exported host functions
- First-class async support (critical for non-blocking veld integration)
- Production-grade security record (Fastly, Cloudflare Workers)
- **Extism** is worth evaluating as a thin SDK-friendly wrapper over wasmtime if author DX is a priority

Add this decision to ticket 010's "Key Decisions" section and include the chosen module format in the Phase 4 SP1 host ABI contract scope.
