# CLI Design

## Why CLI-first matters

Vel already wants a serious CLI story. Skills should therefore be first-class CLI entities, not hidden implementation details of the assistant.

A strong CLI gives you:

- explicit execution
- scriptability
- inspectability
- testability
- composability
- better debugging than chat-only flows

## Proposed command surface

### Discovery and inspection

```bash
vel skill list
vel skill inspect core/daily-brief
vel skill search brief
vel skill tree
```

### Execution

```bash
vel skill run core/daily-brief
vel skill run core/daily-brief --input ./input.json
vel skill run core/daily-brief --json
vel skill run core/daily-brief --output ./brief.md
vel skill run core/daily-brief --dry-run
```

### State and enablement

```bash
vel skill enable core/daily-brief
vel skill disable integrations/spotify-now
vel skill toggle local/repo-standup
```

### Validation and testing

```bash
vel skill validate core/daily-brief
vel skill test core/daily-brief
vel skill lint
```

### Logs and telemetry

```bash
vel skill logs core/daily-brief --last 20
vel skill runs core/daily-brief
vel skill trace run_123
```

### Registry management

```bash
vel skill install ./skills/my-pack
vel skill uninstall local/some-skill
vel skill sync
```

## Alias UX

Frequently used skills should optionally register short aliases.

Example:

```bash
vel daily-brief
vel standup --today
vel task-triage --project vel
```

Under the hood this still resolves to skill references.

## Output modes

Skills should support multiple output formats where possible:

- human-readable text
- JSON
- YAML
- markdown artifact
- machine-readable execution metadata

Example:

```bash
vel skill run core/daily-brief --json
```

This is important for piping skills into other tools or workflows.

## Exit semantics

A skill run should have useful exit codes.

Examples:

- `0` success
- `2` validation failure
- `3` permission denied
- `4` dependency missing
- `5` runtime execution failure

## CLI and policy

CLI should make it obvious when a run is:

- denied by policy
- waiting for confirmation
- partially degraded due to missing capabilities
- operating in dry-run mode

## Recommendation

Expose skills as a primary CLI substrate from the start. That gives Vel a durable execution interface even before fancy UI routing and richer agent planning are complete.
