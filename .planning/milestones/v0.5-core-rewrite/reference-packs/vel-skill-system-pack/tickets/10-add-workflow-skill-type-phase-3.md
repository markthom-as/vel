# Ticket: Add workflow skill type

## Objective

Support multi-step skill composition after single-skill runtime is stable.

## Deliverables

- workflow manifest section
- sequential step execution
- input wiring between steps
- parent/child run trace linkage
- retry and degraded execution behavior

## Acceptance criteria

- a morning orchestrator workflow can call 3 subskills successfully
- child runs inherit bounded grants from parent run
