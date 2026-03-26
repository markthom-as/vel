# Pairing Core Trust Bootstrap Split

## Status

Active planning note. This is not shipped-behavior authority yet.

## Intent

Move Vel pairing semantics into Rust core as a transport-agnostic trust bootstrap subsystem while keeping platform-specific secret custody, approval UX, and discovery adapters outside core.

## Why

The current Apple pairing path depends on a live `veld` endpoint to issue and redeem pairing tokens. That couples:

- trust bootstrap
- node discovery
- current daemon reachability
- platform onboarding choreography

too tightly.

Vel should support a stronger model where a device can pair with a node identity before it can currently reach a daemon transport endpoint.

## Boundary

### Rust core should own

- node identity references used by pairing
- pairing and enrollment artifact schemas
- capability grants and requested scope validation
- pairing state machine and lifecycle states
- trust-anchor and revocation semantics
- candidate endpoint inventory for a trusted node
- endpoint freshness metadata
- signed bootstrap/import-export payload shapes

### Adapters and apps should own

- Keychain / Keystore / Secure Enclave storage
- QR rendering and scanning
- deep links and approval UI
- Bonjour / mDNS / relay-specific discovery
- local-network permission prompts
- push-token plumbing
- biometric unlock flows

## Architectural rule

Never let `paired` imply `reachable`.

Model these separately:

1. trusted identity
2. granted capabilities
3. known endpoints
4. last successful session / observation
5. current reachability

## Recommended crate split

- `vel-core`
  - trust bootstrap domain
  - enrollment artifacts
  - pairing state machine
  - trust records
  - capability grants
- transport/discovery layer
  - authenticated session setup
  - route advertisement and route freshness
- platform clients
  - secure storage adapters
  - pairing UX
  - discovery UX

## First implementation slice

Add portable core types for:

- trusted node endpoint inventory
- endpoint kinds and observations
- transport reachability posture
- trust bootstrap artifacts

without forcing a full migration of current `veld` linking routes in the same patch.

## Follow-on migration path

1. Introduce new core pairing/trust types.
2. Refactor current linking records to use those types internally.
3. Keep current HTTP pairing routes as one adapter over the core state machine.
4. Add platform-specific secret storage and ceremony adapters later.
