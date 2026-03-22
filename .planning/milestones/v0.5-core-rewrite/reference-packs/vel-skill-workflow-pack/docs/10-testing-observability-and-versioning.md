# Testing, Observability, and Versioning

## Testing requirements

Skills should be testable like real software, not merely admired like artisanal prompts.

## Recommended test types

### Manifest validation
Ensure schema correctness and required fields.

### Input validation
Check malformed or missing input handling.

### Smoke tests
Basic run against fixture inputs.

### Golden output tests
For stable deterministic or semi-structured results.

### Permission tests
Validate denied and confirmation-required paths.

### Dry-run tests
Ensure non-mutating preview behavior works.

## Example smoke test file

```yaml
name: basic morning brief
input: ./fixtures/input-basic.json
expected:
  status: success
  schemaValid: true
```

## Observability

Every run should emit:

- run ID
- skill version
- parent run if nested
- start/end time
- requested capabilities
- granted capabilities
- tool call count
- model used
- warnings
- validation results
- artifacts produced
- side effects

## Logs vs traces

### Logs
Human-readable execution notes.

### Traces
Structured event stream or spans for lifecycle stages.

Examples:

- manifest loaded
- policy evaluated
- context mounted
- prepare started
- execute completed
- output validation failed

## Versioning

Version both:

- **API version** — the skill manifest/runtime contract
- **Skill version** — the package itself

Example:

- `apiVersion: vel/v1`
- `version: 0.3.2`

## Upgrade strategy

When the runtime evolves:

- prefer adapters/migrations over breaking everything
- lint for deprecated fields
- provide manifest migration utilities later

## Recommendation

Treat testing and observability as first-class from MVP. Otherwise every future debugging session becomes divination by smoke patterns.
