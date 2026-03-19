# Swarm Protocol Contract

This document defines the versioned message envelope that SDKs and external limbs use to communicate with the authority runtime.

## Current Status

Implemented today:

- `vel-protocol` owns the versioned protocol envelope and validation rules
- `vel-core` re-exports the protocol types for compatibility while ownership shifts outward
- checked-in example and template fixtures are validated in crate tests against the current protocol version
- `vel-agent-sdk` ships a reference Rust client for handshake, heartbeat, capability negotiation, and action-batch submission
- `veld::services::agent_protocol` validates envelopes and mediates handshake, lease renewal, brokered capability negotiation, and scoped action execution against the sandbox host path

Still planned:

- transport/auth exposure of this envelope over external routes
- additional SDK implementations beyond the shipped Rust baseline
- richer version negotiation beyond the current explicit single-version validation

## Purpose

The protocol boundary must be explicit and versioned before SDK or transport implementations widen.

## Core Contracts

- `ProtocolEnvelope`: versioned outer envelope with sender identity and trace linkage.
- `ProtocolPayload`: handshake, heartbeat, capability request, action batch submission, and action result variants.
- `CapabilityRequest`: up-front declared capability request with scope and reason.

## Hard Rules

- protocol versioning is explicit and carried in every envelope
- trace linkage is mandatory for externally visible work
- capability requests declare scope and reason up front
- handshake establishes a connect-run lease and persists the granted capability baseline before action batches execute
- action batches execute only through brokered, deny-by-default sandbox policy enforcement
- protocol envelopes must remain fixture-backed and parseable before runtime integration expands

## Published Artifacts

- schema: `config/schemas/swarm-protocol-envelope.schema.json`
- example: `config/examples/swarm-protocol-envelope.example.json`
- template: `config/templates/swarm-protocol-envelope.template.json`
