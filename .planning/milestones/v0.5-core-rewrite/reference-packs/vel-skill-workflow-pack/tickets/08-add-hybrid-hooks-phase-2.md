# Ticket: Add hybrid hook execution support

## Objective

Extend the runtime to support `prepare` and `cleanup` hooks using a stable subprocess JSON protocol.

## Deliverables

- subprocess contract definition
- Node/TS and Python hook runner support
- timeout handling
- stderr log capture
- structured error reporting

## Acceptance criteria

- a hybrid skill can normalize inputs in prepare and emit artifact metadata in cleanup
- failed hooks do not crash the runtime silently
