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
  "project_id": "proj_velruntime",
  "task_kind": "implementation",
  "agent_profile": "balanced",
  "token_budget": "large",
  "review_gate": "operator_preview",
  "repo_root": {
    "path": "/home/jove/code/vel",
    "label": "vel",
    "branch": "main",
    "head_rev": "abc1234"
  },
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
- coding handoffs also declare project context, task kind, profile, token budget, review gate, and repo root when repo-local execution is in scope
- every handoff links to a trace or run identifier
- every handoff is logged
- no hidden scratch assumptions crossing boundaries
- no secret material should cross a handoff unless the receiving boundary is explicitly allowed to hold it

## Phase 08 Repo-Aware Handoff Guidance

- project-linked coding work should carry both the outer execution context and the inner trace-linked handoff envelope
- repo-local sidecars such as `.planning/vel/*` are inputs to supervised execution, not blanket authorization to edit a repository
- writable roots should remain narrower than readable roots and should be reviewable without reconstructing planner intent from chat history

## Runtime Inspection Contract

- operator run inspection surfaces expose a stable `trace_id` for every run
- delegated runs should also expose `parent_run_id` when that relationship is known from persisted input/output/event metadata
- older runs that predate explicit trace persistence may use `run_id` as the fallback `trace_id`; this is compatibility behavior, not the long-term storage target
