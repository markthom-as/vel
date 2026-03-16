---
title: Memory Contracts
status: open
---

# Goal

Implement typed memory read/write APIs with provenance metadata and topic-pad access controls.

# Tasks

1. Typed read API — query by run_id, surface (constitution, topic_pad, event_store, fact_store), selector; return scoped result; log memory_reads.
2. Typed write API — write with run_id, write_type, target, payload, provenance, confidence, retention_hint; log memory_writes.
3. Provenance metadata — require provenance on all writes; no subagent writes to constitutional memory.
4. Topic pad access controls — enforce memory_scope from agent spec (topic_pads allowlist, event_query scope).

# Acceptance Criteria

- Agents access memory only through typed read/write; no raw DB access.
- All writes include provenance; constitutional memory is protected from subagents.
- Topic pad and event scope respect agent spec.

# Spec reference

[docs/specs/vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md) — Memory Query Contract, Memory Write Contract, Observability Schema (memory_reads, memory_writes).
