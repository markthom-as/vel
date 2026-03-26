# Connect API

The live operator-authenticated connect runtime surface is documented in [runtime.md](runtime.md#connect-runtime-lifecycle).

Current mounted routes:

- `GET /v1/connect/instances`
- `POST /v1/connect/instances`
- `GET /v1/connect/instances/:id`
- `GET /v1/connect/instances/:id/attach`
- `GET /v1/connect/instances/:id/events`
- `GET /v1/connect/instances/:id/events/stream`
- `POST /v1/connect/instances/:id/heartbeat`
- `POST /v1/connect/instances/:id/stdin`
- `POST /v1/connect/instances/:id/terminate`

Current role:

- supervised local coding/runtime transport for `local_command` and `wasm_guest`
- persisted event stream over `stdin` / `stdout` / `stderr` / `system`
- attachable from CLI and linkable from execution-review threads after a handoff-backed launch

Related CLI surfaces:

- `vel connect ...` for direct runtime inspection and control
- `vel thread follow <thread_id>` and `vel thread reply <thread_id> ...` when a launched runtime is already attached to a continuity thread
