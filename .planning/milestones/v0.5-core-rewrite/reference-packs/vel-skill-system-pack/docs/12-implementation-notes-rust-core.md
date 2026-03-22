# Implementation Notes: Rust Core

## Why Rust is a strong fit

The skill runtime has a lot of properties that Rust handles well:

- parsing and validation
- filesystem traversal
- strong typing for manifests and schemas
- deterministic policy enforcement
- robust CLI tooling
- performance without drama
- portable shared core across server/desktop/CLI

## Suggested crate layout

```text
crates/
  skill-manifest/
  skill-schema/
  skill-registry/
  skill-policy/
  skill-context/
  skill-runtime/
  skill-cli/
  skill-telemetry/
```

## Suggested responsibilities

### `skill-manifest`
Serde models, validation, file references, manifest versioning.

### `skill-schema`
Schema resolution and input/output validation helpers.

### `skill-registry`
Discovery, indexing, installation, enable/disable state.

### `skill-policy`
Capabilities, grants, confirmation resolution, scoped policies.

### `skill-context`
Typed context buckets and renderers.

### `skill-runtime`
Execution coordinator and lifecycle state machine.

### `skill-cli`
Subcommands and terminal UX.

### `skill-telemetry`
Run records, structured logs, tracing, metrics.

## Hook bridge approach

Keep hooks decoupled from internal implementation.

Recommended contract:

- runtime invokes subprocess
- stdin carries JSON input envelope
- stdout returns JSON result envelope
- stderr is for logs
- environment variables provide metadata handles if needed

This makes hook languages swappable without destabilizing runtime core.

## Context rendering strategy

Keep a typed internal model, then render model-specific prompt sections late.

That gives you:

- clean source-of-truth data
- compact-mode rendering for smaller models
- richer rendering for reasoning-heavy paths

## Internal persistence

Execution records can start as local JSONL or sqlite-backed storage, depending on Vel’s broader architecture.

For MVP, JSONL or sqlite is fine. Do not invent a distributed telemetry empire on day one.

## Recommendation

Use Rust for the runtime core and CLI. Use Node/TS and Python for hooks initially. Add WASM later if and when portability/sandboxing needs become real enough to justify the complexity tax.
