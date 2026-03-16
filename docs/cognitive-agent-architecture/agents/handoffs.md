# Agent Handoffs

Agent handoffs must be structured.

## Handoff Envelope

```json
{
  "task_id": "uuid",
  "from_agent": "planner",
  "to_agent": "risk_evaluator",
  "objective": "Evaluate risk for upcoming commitments in next 2 hours",
  "inputs": {},
  "constraints": [],
  "deadline": "ISO8601",
  "expected_output_schema": {}
}
```

## Rules

- every handoff has a declared objective
- every handoff defines output schema
- every handoff is logged
- no hidden scratch assumptions crossing boundaries
