# Ticket: Build prompt-only execution engine

## Objective

Allow prompt-only skills to run through the shared runtime with schema validation.

## Deliverables

- prompt file loading
- prompt rendering with mounted context
- model invocation wrapper
- output parsing and schema validation
- execution record creation

## Acceptance criteria

- prompt-only skills can be run from CLI
- output schema validation passes or fails clearly
- execution logs record model and token usage if available
