# Runtime Architecture

## Runtime responsibilities

The skill runtime should be the single place responsible for:

- discovery
- manifest parsing and validation
- registry resolution
- dependency resolution
- capability requests
- policy evaluation
- context mounting
- hook execution
- model/tool mediation
- output validation
- artifact emission
- logging and telemetry

In other words: the runtime is the governor, not just a file loader.

## Proposed core modules

### `skill-manifest`
Parses and validates manifests.

### `skill-registry`
Discovers skills from filesystem or later remote registries.

### `skill-policy`
Resolves permissions, grants, confirmation requirements, and denials.

### `skill-context`
Builds typed execution context slices.

### `skill-runtime`
Coordinates actual execution lifecycle.

### `skill-cli`
Exposes CLI-facing command surface.

### `skill-telemetry`
Emits logs, traces, metrics, and audit records.

## Discovery and registry order

Recommended resolution order:

1. workspace-local: `./.vel/skills`
2. user-local: `~/.config/vel/skills`
3. installed/global: `/opt/vel/skills` or packaged install path
4. bundled/core: shipped with Vel

This allows local overrides without sacrificing deterministic defaults.

## Execution protocol for hooks

Hooks should use a stable subprocess contract.

### Input
JSON over stdin.

### Output
JSON over stdout.

### Logs
stderr.

### Status
exit code.

This is boring, which is exactly why it is good.

Boring contracts are portable contracts.

## Hook runtimes

Initial recommended supported hook runtimes:

- TypeScript/Node
- Python
- shell for strictly bounded internal use

Longer term:

- WASM hooks for safer portable execution
- native Rust-compiled hooks

## Runtime execution stages

### 1. Resolve skill reference
By namespace/name/version or alias.

### 2. Load manifest and validate
Reject malformed skill packs early.

### 3. Resolve dependencies
Skills, templates, schemas, runtime requirements.

### 4. Evaluate policy
Determine whether requested capabilities are allowed, denied, or require confirmation.

### 5. Mount context
Construct typed execution context based on explicit grants and runtime environment.

### 6. Run prepare hook
Optional deterministic normalization phase.

### 7. Run execute phase
Prompt, script, or hybrid execution.

### 8. Validate output
Against schema if provided.

### 9. Run cleanup hook
Optional persistence/artifact stage.

### 10. Emit telemetry
Logs, metrics, artifacts, audit trace.

## Runtime surfaces

The same skill runtime should be callable from:

- CLI
- chat assistant backend
- UI actions
- automations/scheduled tasks
- background jobs
- event-driven triggers

Do not create one weird runtime per surface. That way lies duplication and eventually theological disputes about which path is “real.”

## Artifact model

A skill execution may produce:

- structured output
- markdown or text artifact
- JSON report
- created tasks/events/messages
- references to persistent entities
- logs and trace metadata

Artifacts should be explicit first-class outputs, not hidden side effects.

## Suggested execution record structure

A run should emit something like:

```json
{
  "runId": "run_123",
  "skill": "core/daily-brief",
  "version": "0.1.0",
  "status": "success",
  "startedAt": "2026-03-21T20:00:00Z",
  "endedAt": "2026-03-21T20:00:06Z",
  "capabilityGrants": ["calendar.read", "tasks.read"],
  "model": "reasoning",
  "toolCalls": 5,
  "artifacts": [
    {"type": "markdown", "path": "artifacts/run_123/brief.md"}
  ],
  "warnings": [],
  "cost": {"tokensIn": 5500, "tokensOut": 900}
}
```

This makes skills operationally legible rather than magical.
