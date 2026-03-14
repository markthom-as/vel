# Vel — API Specification

## Purpose

This document defines the initial API boundary for Vel.

The API exists to let multiple clients talk to the same core system (`veld`) without duplicating logic.

Primary clients include:

- `vel-cli`
- iPhone app
- Apple Watch surfaces
- future desktop/dashboard clients
- optional local voice interfaces
- internal background jobs or automation hooks

The v0 API should optimize for:

- simplicity
- typed request/response boundaries
- local-first usage
- easy debugging
- easy evolution

The v0 API should not optimize for public third-party developer ergonomics yet.

## Design Principles

### 1. veld is the canonical service

Clients should not implement business logic independently when it can live in `veld`.

### 2. HTTP + JSON first

Use HTTP, JSON, explicit versioning, and typed schemas.

### 3. API should be local-first

The API must work on localhost first, then LAN and remote hosts later.

### 4. Typed schema, flexible payloads

Core request and response shapes should be explicit, with metadata dictionaries where needed.

### 5. Degraded mode matters

Responses should make degraded state explicit instead of hiding it.

## Versioning

All bootstrap work uses `/v1`.

## Authentication Model

For v0:

- local-only unauthenticated mode on localhost
- bearer token mode when networked

## Common Response Shape

Prefer a consistent envelope:

```json
{
  "ok": true,
  "data": {},
  "warnings": [],
  "meta": {
    "request_id": "req_123",
    "degraded": false
  }
}
```

## Endpoint Groups

The v0 API surface should grow around:

- health
- captures
- search
- context
- projects
- goals
- suggestions
- behavior
- jobs
- artifacts
- timeline

## Bootstrap Endpoints

### `GET /v1/health`

Returns:

- daemon status
- DB status
- version
- degraded flag if relevant

### `POST /v1/captures`

Accepts:

- simple text capture payload

Behavior:

- validate request
- write capture row
- return capture ID
- optionally create a stub processing job

