# Agent Handoffs

Agent handoffs must be structured.

## Handoff Envelope

```json
{
  "task_id": "uuid",
  "trace_id": "uuid",
  "from_agent": "planner",
  "to_agent": "risk_evaluator",
  "objective": "Evaluate risk for upcoming commitments in next 2 hours",
  "inputs": {},
  "constraints": [],
  "read_scopes": [],
  "write_scopes": [],
  "allowed_tools": [],
  "capability_scope": {},
  "deadline": "ISO8601",
  "expected_output_schema": {}
}
```

## Rules

- every handoff has a declared objective
- every handoff defines output schema
- every handoff declares tool and capability scope
- every handoff declares repository read/write scope when code, docs, or config may be inspected or changed
- every handoff links to a trace or run identifier
- every handoff is logged
- no hidden scratch assumptions crossing boundaries
- no secret material should cross a handoff unless the receiving boundary is explicitly allowed to hold it
