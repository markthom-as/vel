# Swarm Protocol Contract

This document defines the versioned message envelope that future SDKs and external limbs will use to communicate with the authority runtime.

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
- protocol envelopes must remain fixture-backed and parseable before runtime integration expands

## Published Artifacts

- schema: `config/schemas/swarm-protocol-envelope.schema.json`
- example: `config/examples/swarm-protocol-envelope.example.json`
- template: `config/templates/swarm-protocol-envelope.template.json`
