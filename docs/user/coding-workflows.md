# Coding Workflows

Vel’s coding path is supervised and repo-local by design.

The shipped workflow is:

1. save project execution context
2. preview or export the repo-local sidecar pack
3. review the persisted handoff and routing decision
4. launch a supervised runtime through `/v1/connect/instances`
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

The live launch transport is `POST /v1/connect/instances`.

Shipped runtime kinds:

- `local_command`
- `wasm_guest`

Today, Vel does not yet ship a dedicated `vel connect launch` wrapper. The intended launch input is either:

- an authenticated API call to `/v1/connect/instances`, or
- a payload built by `vel-agent-sdk` and sent to that route

The reference SDK now mirrors the live route contract with:

- `AgentSdkClient::manifest_reference(...)`
- `AgentSdkClient::connect_launch_request(...)`

For direct inspection after launch:

```bash
cargo run -p vel-cli -- connect instances
cargo run -p vel-cli -- connect inspect <run_id>
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
