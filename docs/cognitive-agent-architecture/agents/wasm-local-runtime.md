---
title: WASM Local Runtime
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - wasm
  - guest-runtime
  - connect
  - sandbox
summary: Phase 08 owner doc for the direct guest-runtime seam, writable-root rules, and its shared brokered policy boundary.
---

# Purpose

Document the direct guest-runtime seam introduced in Phase 08 and the policy boundaries it shares with the existing sandbox host executor.

# Current Behavior

- `runtime_kind: "wasm_guest"` launches through the same `/v1/connect/instances` transport as local command runtimes.
- Guest execution stays deny-by-default and broker-mediated through `services::sandbox`.
- Writable roots are declared up front on the launch request and guest modules may not widen them after launch.
- Any requested network host is currently rejected. Guest network scope does not widen silently.
- The current direct guest seam consumes a checked-in guest module spec that expands to sandbox host-call envelopes. This keeps the policy, trace, and review model explicit while leaving room for a future engine-backed binary loader.

# Hard Rules

- guest runtimes may not widen filesystem roots beyond the declared writable roots
- guest runtimes may not widen network scope after launch
- capability requests still flow through broker mediation and allowlists
- sandbox denials and terminal states remain trace-linked and inspectable
- the compatibility host-executor path must remain explicit, never a hidden bypass around guest-runtime policy

# Writable Roots

- launch requests must declare bounded `writable_roots`
- guest module requested writable roots must stay within both the working directory and the declared writable roots
- out-of-scope writable roots fail closed with `403`

# Downstream Usage

- connect-runtime owns lifecycle entry, persistence, and inspectability
- sandbox owns host-call mediation, broker checks, and terminal decision recording
- runtime docs should describe guest runtime as a supervised variant of the same connect boundary, not a privileged escape hatch
