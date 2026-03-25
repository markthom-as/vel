# Coding Workflows

Vel’s coding path is supervised and repo-local by design.

The shipped workflow is:

1. save project execution context
2. preview or export the repo-local sidecar pack
3. review the persisted handoff and routing decision
4. launch a supervised runtime from the approved handoff
5. inspect the resulting connect run and backing run

## 1. Persist execution context

Start from a typed project record that already has a primary repo and notes root.

```bash
cargo run -p vel-cli -- exec save <project_id> \
  --objective "ship the next safe slice" \
  --constraint "sidecar only" \
  --expected-output .planning/vel/gsd-handoff.md
```

Use these supporting commands as needed:

```bash
cargo run -p vel-cli -- exec show <project_id>
cargo run -p vel-cli -- exec preview <project_id>
```

## 2. Export the repo-local sidecar pack

Export writes only inside the project’s declared primary repo root.

```bash
cargo run -p vel-cli -- exec export <project_id>
```

The default pack lives under `.planning/vel/` and is meant to stay readable by GSD and other supervised repo-local tooling.

## 3. Review the handoff explicitly

Handoffs are persisted before launch so the operator can inspect objective, scopes, routing reasons, and review gate.

```bash
cargo run -p vel-cli -- exec review --state pending_review
cargo run -p vel-cli -- exec launch-preview <handoff_id>
cargo run -p vel-cli -- exec approve <handoff_id> --reason "scope and output contract look right"
```

Reject when the write scope, tools, or expected output contract are not explicit enough:

```bash
cargo run -p vel-cli -- exec reject <handoff_id> --reason "write scope is broader than intended"
```

The web General settings surface exposes the same pending execution review queue beside sync and SAFE MODE status.

## 4. Launch a supervised runtime

The handoff-aware launch transport is `POST /v1/execution/handoffs/:id/launch`.

Shipped runtime kinds:

- `local_command`
- `wasm_guest`

CLI launch wrapper:

```bash
cargo run -p vel-cli -- exec launch <handoff_id> \
  --runtime-kind local_command \
  --working-dir /home/jove/code/vel \
  --writable-root /home/jove/code/vel \
  -- /bin/sh -lc "sleep 30"
```

Direct connect launch transport still exists at `POST /v1/connect/instances` when you need raw runtime control.

The reference SDK now mirrors the live route contract with:

- `AgentSdkClient::manifest_reference(...)`
- `AgentSdkClient::connect_launch_request(...)`

For direct inspection after launch:

```bash
cargo run -p vel-cli -- connect instances
cargo run -p vel-cli -- connect inspect <run_id>
cargo run -p vel-cli -- connect attach <run_id>
cargo run -p vel-cli -- connect stdin <run_id> "status"
cargo run -p vel-cli -- connect events <run_id> --limit 200
cargo run -p vel-cli -- connect tail <run_id> --poll-ms 500
cargo run -p vel-cli -- connect stream <run_id> --poll-ms 500
cargo run -p vel-cli -- run inspect <run_id>
```

## 5. Guest-runtime limits

`wasm_guest` stays inside the same supervised sandbox boundary as other delegated work.

- guest modules cannot widen network scope
- writable roots must stay inside the declared working directory
- undeclared writable-root requests are denied before launch
- terminal reasons stay persisted on the connect instance and backing run

## Practical rule

Use Vel to prepare context, persist reviewable handoffs, and supervise launch. Do not treat it as ambient repo authority.

## Using this pattern for integrations and internals

This is the recommended direction.
Use the same supervised handoff pattern for integration work and internal runtime changes, not just app-feature coding.

Apply these guardrails:

- keep write scope explicit and narrow (integration adapter, route, schema, or policy file set).
- keep read scope broader than write scope when diagnosis needs it.
- require review and approval before any mutation-capable launch.
- keep capability access scoped to the target provider/action instead of broad ambient access.
- preserve run, event, and artifact lineage so every significant decision is inspectable later.

Examples of good fit:

- adding or refining a connector contract.
- implementing a policy-gated writeback action.
- adjusting run orchestration or trace surfaces.
- migrating internal module boundaries while preserving behavior and evidence.

Avoid using this pattern as an excuse for:

- broad unscoped repo mutation.
- hidden secret exposure in prompts or logs.
- direct mutation paths that bypass confirmation discipline.

## Reusable workflow definition template

Use this template when you design a new integration or internal workflow:

1. objective: one sentence outcome and success condition.
2. read scope: what may be inspected to make decisions.
3. write scope: exact files/tables/routes/surfaces allowed to change.
4. capabilities: exact provider/tool actions required (no ambient wildcard).
5. decision steps: ordered checkpoints where Vel asks for confirmation.
6. mutation steps: explicit operations with preconditions and rollback notes.
7. evidence outputs: required run events, artifacts, and operator-readable summary.
8. deny paths: what must fail closed and how that reason is surfaced.
9. completion proof: tests/manual checks required before closure.

Suggested handoff fields:

- `objective`
- `constraint`
- `expected_output`
- `read_scope`
- `write_scope`
- `capability_scope`
- `approval_required_for`
- `evidence_required`

Example (overdue-task action workflow sketch):

- objective: resolve overdue commitments without silent mutation.
- read scope: commitments, due dates, relevant calendar window.
- write scope: commitment status/due-date fields only.
- capability scope: local commitment mutation actions only.
- approval required for: close, reschedule, unschedule, tombstone.
- evidence required: before/after task state and run-event timeline.
