# Vel API Overview

This directory contains canonical API documentation for both:

- the **runtime API** (`/v1`) served by `veld`, and
- the **operator/chat API** (`/api`) plus the WebSocket stream (`/ws`) used by the web surfaces.

## Runtime API (`/v1`)

See [`runtime.md`](runtime.md) for:

- health
- run inspection and trace-linked workflow lineage
- captures
- search
- context
- commitments
- nudges
- risk
- explain endpoints
- synthesis
- daily-loop overdue workflow (`menu -> confirm -> apply -> undo`)

Mounted runtime contract notes:

- [`standup-overdue-workflow-contract.md`](standup-overdue-workflow-contract.md) (`menu -> confirm -> apply -> undo` overdue-task workflow)

## Chat API (`/api`)

See [`chat.md`](chat.md) for:

- conversations
- messages
- inbox
- interventions
- settings
- components
- integrations
- WebSocket events
