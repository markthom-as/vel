# Sandbox Host ABI

This document defines the deny-by-default ABI boundary for sandboxed WASM agents.

## Current Status

Implemented today:

- brokered capability grants, denials, and executions persist broker audit events
- the sandbox host executor consumes decoded ABI envelopes, enforces `allowed_calls`, and fails closed
- sandbox call outcomes are written into run events as `sandbox_call_evaluated` and `sandbox_run_completed`
- `vel run inspect` now prints payloads for sandbox/search diagnostic events so denial reasons are operator-visible

Still planned:

- direct WASM guest decoding and execution inside a chosen runtime engine
- richer operator surfaces beyond existing run inspection
- broader filesystem/network mediation beyond the current explicit call set

## Purpose

Sandboxed modules should not gain ambient host authority. Every host interaction must move through an explicit ABI envelope and policy decision path.

## Core Contracts

- `SandboxCapabilityPolicy`: default mode, allowed call list, filesystem/network policy, and resource limits.
- `SandboxHostCallEnvelope`: trace-linked sandbox-to-host request envelope.
- `SandboxDecisionRecord`: inspectable approval, denial, or failure record for each host call.

## Hard Rules

- unknown ABI calls fail closed
- network and filesystem access are separate policy surfaces
- capability requests are brokered, not silently upgraded inside the sandbox
- denials must remain operator-readable and trace-linked
- resource limits are explicit contract fields, not hidden runtime defaults

## Published Artifacts

- schema: `config/schemas/sandbox-policy.schema.json`
- schema: `config/schemas/sandbox-host-call.schema.json`
- example: `config/examples/sandbox-host-call.example.json`
- template: `config/templates/sandbox-policy.template.json`
- template: `config/templates/sandbox-host-call.template.json`
