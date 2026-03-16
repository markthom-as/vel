---
title: Executor Integration
status: open
---

# Goal

Integrate the executor: capability token issuance, executor RPC, and tool call logging.

# Tasks

1. Capability token issuance — issue short-lived, least-privilege tokens (subject, tool, scope, expires_at, side_effects); log issuance.
2. Executor RPC — implement `POST /execute` (run_id, tool, arguments, capability_token); enforce timeouts and return structured result or error.
3. Tool call logging — persist tool_calls (call_id, run_id, tool_name, arguments_hash, started_at, ended_at, status, side_effect_class) for replay and audit.

# Acceptance Criteria

- Agents request tool execution via runtime; executor validates token and runs tool.
- Tool calls are logged with required fields.
- Executor returns typed success/error; no untyped failures.

# Spec reference

[docs/specs/vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md) — Tool Permission Model, Executor RPC Interface, Observability Schema (tool_calls).
