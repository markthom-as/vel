---
title: Zero-Trust WASM Agent Sandboxing
status: planned
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
labels:
  - veld
  - agentic
  - wasm
---

Implement a Zero-Trust WebAssembly (WASM) agent sandboxing environment within `veld` to securely execute 3rd-party community agents and skills.

## Technical Details
- **WASM Runtime**: Incorporate `wasmtime` or `extism` as a host environment inside `veld`.
- **Capability Host API**: Develop a `VelHostABI` that provides a strict set of data and action requests to the sandbox.
- **Permission Interception**: Implement a system that prompts the user for approval via the Web UI when an agent requests access to external resources (e.g., the filesystem).
- **Skill SDK**: Provide a basic Rust/TS SDK for community developers to compile their logic to WASM.

## Acceptance Criteria
- External agents can be executed in a secure, isolated sandbox.
- Sandbox modules have zero direct access to the host's filesystem or network.
- All requests for Vel data/actions must pass through the Host ABI and Policy Engine.
